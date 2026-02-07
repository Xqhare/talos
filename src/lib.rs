// Uncomment below when appropriate
#![warn(clippy::pedantic)]
#![warn(clippy::all)]
// Uncomment below when approaching stable
//#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::io::Write;

use codex::Codex;
use input::Parser;
use input::poll_input_bytes;
use ui::render::CCell;
use utils::constants::ansi::CLEAR_ALL;
use utils::constants::ansi::TO_TOP_LEFT;
use utils::write_all_bytes;

mod builder;
pub use builder::{LayoutBuilder, ParserBuilder, TalosBuilder};
mod error;
pub use error::{TalosError, TalosResult};

use crate::backend::TerminalIO;
use crate::backend::sys::check_resize;
use crate::backend::sys::check_terminate;
use crate::input::Event;
use crate::ui::render::Canvas;
use crate::utils::move_render_cursor;

mod backend;
pub mod codex;
mod content;
mod ui;

pub mod input;
pub use ui::layout;
pub use ui::render;
mod utils;
pub mod widgets;

type Width = u16;
type Height = u16;

pub struct Talos {
    terminal: TerminalIO,
    canvas: Canvas,
    codex: Codex,
    // Terminal Size
    /// Width, Height
    size: (Width, Height),
    previous_buffer: Vec<CCell>,

    output_buffer: Vec<u8>,

    // Input-Parser
    parser: Parser,
}

pub enum Present {
    Resized,
    Presented,
}

impl Talos {
    #[must_use]
    pub fn builder() -> TalosBuilder {
        TalosBuilder::default()
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    pub fn render_ctx(&mut self) -> (&mut Canvas, &Codex) {
        (&mut self.canvas, &self.codex)
    }

    pub fn begin_frame(&mut self) {
        self.canvas.clear();
    }

    /// Present the canvas to the terminal
    /// The new size is stored in `self.size`.
    ///
    /// # Returns
    /// Returns whether the terminal was resized.
    /// If the terminal was resized, `present` will not draw anything to the terminal.
    /// Returns `Ok(Present::Resized)` if the terminal was resized.
    /// Returns `Ok(Present::Presented)` if the terminal was not resized.
    ///
    /// # Errors
    /// Returns an error if the terminal was terminated.
    pub fn present(&mut self) -> TalosResult<Present> {
        let resized = self.handle_signals()?;
        if resized {
            return Ok(Present::Resized);
        }

        self.output_buffer.clear();

        write_all_bytes(&mut self.output_buffer, TO_TOP_LEFT.as_bytes())?;

        let mut prev_x_cell: u16 = u16::MAX;

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let buffer_index = (x + y * self.size.0) as usize;

                if self.canvas.buffer[buffer_index] != self.previous_buffer[buffer_index] {
                    let ccell = self.canvas.get_ccell(x, y);

                    if x.wrapping_sub(prev_x_cell) != 1 {
                        move_render_cursor(&mut self.output_buffer, x, y)?;
                    }

                    // Write styled char
                    ccell.style.generate(&mut self.output_buffer);
                    write_all_bytes(
                        &mut self.output_buffer,
                        self.codex.resolve(ccell.char).as_bytes(),
                    )?;
                    prev_x_cell = x;
                }
            }
        }

        if self.handle_signals()? {
            // Resized! - Just show one blank frame - should be imperceivable anyways
            self.output_buffer.clear();
            return Ok(Present::Presented);
        }

        self.terminal.stdout().write_all(&self.output_buffer)?;
        self.terminal.stdout().flush()?;

        // Pointer swapping of the buffers
        std::mem::swap(&mut self.previous_buffer, &mut self.canvas.buffer);

        Ok(Present::Presented)
    }

    pub fn codex(&mut self) -> &mut Codex {
        &mut self.codex
    }

    /// Returns all input events since the last call.
    /// If there is no input, returns None.
    ///
    /// Eagerly evaluates all bytes read, and returns an `Event::Unknown` if
    /// the bytes cannot be parsed.
    ///
    /// # Errors
    /// Returns an error if the terminal was terminated.
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
    ///
    /// Returns `true` whether the terminal was resized OR terminated.
    /// Returns `false` if the terminal was not resized or terminated.
    ///
    /// If the terminal was terminated, the terminal is restored and the process exits.
    fn handle_signals(&mut self) -> TalosResult<bool> {
        if check_terminate() {
            // We need to shut down now - No state will be saved, just restore the terminal
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
