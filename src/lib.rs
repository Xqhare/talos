use std::io::Write;

use builder::TalosBuilder;
use error::TalosResult;
use input::Event;
use input::poll_input_bytes;
use input::Parser;
use render::{CCell, Codex};
use utils::constants::ansi::CLEAR_ALL;
use utils::constants::ansi::TO_TOP_LEFT;
use utils::push_u16_as_ascii;
use utils::sys::check_resize;
use utils::sys::check_terminate;
use utils::terminal::TerminalIO;
use utils::write_all_bytes;

mod builder;
mod error;
mod input;
mod render;
pub use render::{Colour, Style, Widget, Canvas};
mod utils;
pub mod layout;
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

impl Talos {
    pub fn builder() -> TalosBuilder {
        TalosBuilder::default()
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    pub fn begin_frame(&mut self) {
        self.canvas.clear();
    }

    /// Present the canvas to the terminal
    ///
    /// Returns whether the terminal was resized.
    /// Returns `false` if the terminal was resized.
    /// Returns `true` if the terminal was not resized.
    ///
    /// While the logic of the returned boolean seems flipped, it describes if `present`
    /// finished what it was supposed to do. If a resize event happened, `present` exited
    /// without drawing to the terminal.
    ///
    /// The new size is stored in `self.size`.
    pub fn present(&mut self) -> TalosResult<bool> {
        let resized = self.handle_signals()?;
        if resized {
            return Ok(false);
        }

        self.output_buffer.clear();

        write_all_bytes(&mut self.output_buffer, TO_TOP_LEFT.as_bytes())?;

        let mut prev_x_cell: u16 = 0;

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let buffer_index = (x + y * self.size.0) as usize;

                if self.canvas.buffer[buffer_index] != self.previous_buffer[buffer_index] {
                    let ccell = self.canvas.get_ccell(x, y);

                    // Cursor handling
                    // TODO: refactor into its own function
                    if x - prev_x_cell != 1 {
                        let bytes = [
                            0x1b,
                            b'[',
                            
                        ];
                        write_all_bytes(&mut self.output_buffer, &bytes)?;
                        push_u16_as_ascii(&mut self.output_buffer, y.saturating_add(1));
                        write_all_bytes(&mut self.output_buffer, &[b';'])?;
                        push_u16_as_ascii(&mut self.output_buffer, x.saturating_add(1));
                        write_all_bytes(&mut self.output_buffer, &[b'H'])?;
                    }

                    // Write styled char
                    ccell.style.generate(&mut self.output_buffer);
                    write_all_bytes(&mut self.output_buffer, self.codex.resolve(ccell.char).as_bytes())?;
                }
                prev_x_cell = x;
            }
        }

        if self.handle_signals()? {
            // Resized! - Just show one blank frame - should be imperceivable anyways
            self.output_buffer.clear();
            write_all_bytes(&mut self.terminal.stdout(), CLEAR_ALL.as_bytes())?;
            self.terminal.stdout().flush()?;
            return Ok(false);
        }

        self.terminal.stdout().write_all(&self.output_buffer)?;
        self.terminal.stdout().flush()?;

        // Pointer swapping of the buffers
        std::mem::swap(&mut self.previous_buffer, &mut self.canvas.buffer);

        Ok(true)
    }

    pub fn codex(&mut self) -> &mut Codex {
        &mut self.codex
    }

    /// Returns all input events since the last call.
    /// If there is no input, returns None.
    ///
    /// Eagerly evaluates all bytes read, and returns an `Event::Unknown` if
    /// the bytes cannot be parsed.
    pub fn poll_input(&mut self) -> TalosResult<Option<&[Event]>> {
        let _ = self.handle_signals()?;

        self.parser.event_buffer.clear();

        if let Some(bytes) = poll_input_bytes(
            &mut self.terminal.stdin(),
            &mut self.parser.poll_input_buffer,
            self.parser.max_poll_input_buffer,
            self.parser.buffer_linear_growth_step,
        )? {
            self.parser.parser.parse(bytes, &mut self.parser.event_buffer)?;
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
            let new_size = self.terminal.size()?;
            self.size = new_size;

            self.canvas = Canvas::new(self.size.0, self.size.1);
            let len = (self.size.0 as usize) * (self.size.1 as usize);
            self.previous_buffer = vec![CCell::default(); len];
            self.output_buffer.clear();
            self.output_buffer.reserve(len * 10);
            return Ok(true);
        }

        Ok(false)
    }
}
