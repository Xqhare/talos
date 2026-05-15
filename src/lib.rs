#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::all)]
use std::io::Write;

use codex::Codex;
use input::Parser;
use input::poll_input_bytes;
use ui::render::{CCell, Style, InteractionMap};
use utils::constants::ansi::CLEAR_ALL;
use utils::constants::ansi::TO_TOP_LEFT;
use utils::constants::ansi::{BEGIN_SYNC_UPDATE, END_SYNC_UPDATE};
use utils::write_all_bytes;

mod builder;
pub use builder::{LayoutBuilder, ParserBuilder, TalosBuilder};
pub mod error;
pub use error::{TalosError, TalosResult};

/// Style and Layout Atlases
pub mod atlases;

use crate::backend::TerminalIO;
use crate::backend::sys::check_resize;
use crate::backend::sys::check_terminate;
use crate::input::Event;
use crate::ui::render::Canvas;
use crate::utils::move_render_cursor;

mod backend;
/// Codex
pub mod codex;
mod content;
mod ui;

/// Input
pub mod input;
pub use ui::layout;
pub use ui::render;
mod utils;
/// Widgets
pub mod widgets;

type Width = u16;
type Height = u16;

/// The main struct of the library
pub struct Talos {
    terminal: TerminalIO,
    canvas: Canvas,
    codex: Codex,
    interactions: InteractionMap,
    // Terminal Size
    /// Width, Height
    size: (Width, Height),
    previous_buffer: Vec<CCell>,
    output_buffer: Vec<u8>,
    // Input-Parser
    parser: Parser,
}

/// Return type of `Talos::present`
pub enum Present {
    /// The terminal was resized, and because of that nothing was drawn
    Resized,
    /// The terminal was not resized - the canvas was drawn
    Presented,
}

impl Talos {
    /// Returns a new `TalosBuilder`
    pub fn builder() -> TalosBuilder {
        TalosBuilder::default()
    }

    /// Returns a mutable reference to the canvas
    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    /// Returns a RenderContext containing the canvas and the codex.
    pub fn render_ctx(&mut self) -> ui::render::RenderContext<'_> {
        ui::render::RenderContext::new(&mut self.canvas, &self.codex, &mut self.interactions)
    }

    /// Returns the interaction map.
    pub fn interactions(&self) -> &InteractionMap {
        &self.interactions
    }

    /// Returns the interaction map mutably.
    pub fn interactions_mut(&mut self) -> &mut InteractionMap {
        &mut self.interactions
    }

    /// Clear the canvas and interaction map.
    /// Call at the beginning of every frame.
    pub fn begin_frame(&mut self) {
        self.canvas.clear();
        self.interactions.clear();
    }

    /// Present the canvas to the terminal
    pub fn present(&mut self) -> TalosResult<Present> {
        let resized = self.handle_signals()?;
        if resized {
            return Ok(Present::Resized);
        }

        self.output_buffer.clear();
        write_all_bytes(&mut self.output_buffer, TO_TOP_LEFT.as_bytes())?;
        Style::default().generate(&mut self.output_buffer);

        let mut prev_x_cell: u16 = u16::MAX;
        let mut current_terminal_style = Style::default();

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let buffer_index = (x + y * self.size.0) as usize;

                if self.canvas.buffer[buffer_index] != self.previous_buffer[buffer_index] {
                    let ccell = self.canvas.get_ccell(x, y);

                    if x.wrapping_sub(prev_x_cell) != 1 {
                        move_render_cursor(&mut self.output_buffer, x, y)?;
                    }

                    // Only generate style if it differs from the current terminal style
                    if ccell.style != current_terminal_style {
                        ccell
                            .style
                            .generate_diff(current_terminal_style, &mut self.output_buffer);
                        current_terminal_style = ccell.style;
                    }

                    write_all_bytes(
                        &mut self.output_buffer,
                        self.codex.resolve(ccell.char).as_bytes(),
                    )?;
                    prev_x_cell = x;
                }
            }
        }

        if self.handle_signals()? {
            self.output_buffer.clear();
            return Ok(Present::Presented);
        }

        self.terminal
            .stdout()
            .write_all(BEGIN_SYNC_UPDATE.as_bytes())?;
        self.terminal.stdout().write_all(&self.output_buffer)?;
        self.terminal
            .stdout()
            .write_all(END_SYNC_UPDATE.as_bytes())?;
        self.terminal.stdout().flush()?;

        std::mem::swap(&mut self.previous_buffer, &mut self.canvas.buffer);

        Ok(Present::Presented)
    }

    /// Returns a mutable reference to the codex
    pub fn codex_mut(&mut self) -> &mut Codex {
        &mut self.codex
    }

    /// Returns the codex
    pub fn codex(&self) -> &Codex {
        &self.codex
    }

    /// Returns all input events since the last call.
    pub fn poll_input(&mut self) -> TalosResult<Option<&[Event]>> {
        let _ = self.handle_signals()?;
        self.parser.event_buffer.clear();

        if let Some(bytes) = poll_input_bytes(
            &mut self.terminal.stdin(),
            &mut self.parser.poll_input_buffer,
            self.parser.max_poll_input_buffer,
            self.parser.buffer_linear_growth_step,
        )? {
            self.parser
                .parser
                .parse(bytes, &mut self.parser.event_buffer)?;
        } else {
            self.parser.parser.flush(&mut self.parser.event_buffer);
        }

        Ok(Some(self.parser.event_buffer.as_slice()))
    }

    /// Handles signals from the OS
    fn handle_signals(&mut self) -> TalosResult<bool> {
        if check_terminate() {
            self.terminal.restore()?;
            std::process::exit(0);
        }

        if check_resize() {
            let (rows, cols) = self.terminal.size()?;
            self.size = (cols, rows);

            self.canvas = Canvas::new(self.size.0, self.size.1);
            let len = (self.size.0 as usize) * (self.size.1 as usize);
            self.previous_buffer = vec![CCell::default(); len];
            self.output_buffer.clear();
            self.output_buffer.reserve(len * 10);
            write_all_bytes(&mut self.terminal.stdout(), CLEAR_ALL.as_bytes())?;
            self.terminal.stdout().flush()?;
            return Ok(true);
        }

        Ok(false)
    }
}
