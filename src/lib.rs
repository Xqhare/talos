use std::io::Write;

use builder::TalosBuilder;
use constants::ansi::{CLEAR_ALL, TO_TOP_LEFT};
use error::TalosResult;
use input::Event;
use input::poll_input_into_events;
use render::{CCell, Canvas, Codex};
use sys::{check_resize, check_terminate};
use terminal::term_io::TerminalIO;

mod builder;
mod constants;
mod error;
mod input;
mod render;
mod sys;
mod terminal;

pub use render::Colour;
pub use render::Style;

pub type Width = u16;
pub type Height = u16;

pub struct Talos {
    terminal: TerminalIO,
    canvas: Canvas,
    codex: Codex,
    // Terminal Size
    /// Width, Height
    size: (Width, Height),
    previous_buffer: Vec<CCell>,

    output_buffer: Vec<u8>,

    // Input - TODO: Move into separate struct
    poll_input_buffer: Vec<u8>,
    buffer_linear_growth_step: usize,
    max_poll_input_buffer: usize,
}

impl Talos {
    pub fn builder() -> TalosBuilder {
        TalosBuilder::default()
    }

    pub fn begin_frame(&mut self) {
        self.canvas.clear();
    }

    pub fn present(&mut self) -> TalosResult<()> {
        let _resized = self.handle_signals()?;

        self.output_buffer.clear();

        write!(self.output_buffer, "{}", TO_TOP_LEFT)?;

        let mut prev_x_cell: u16 = 0;

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let buffer_index = (x + y * self.size.0) as usize;

                if self.canvas.buffer[buffer_index] != self.previous_buffer[buffer_index] {
                    let ccell = self.canvas.get_ccell(x, y);

                    // Cursor handling
                    if x - prev_x_cell != 1 {
                        write!(self.output_buffer, "\x1b[{};{}H", y + 1, x + 1)?;
                    }

                    // Write styled char
                    ccell.style.generate(&mut self.output_buffer);
                    write!(self.output_buffer, "{}", self.codex.resolve(ccell.char))?;
                }
                prev_x_cell = x;
            }
        }

        if self.handle_signals()? {
            // Resized! - Just show one blank frame - should be imperceivable anyways
            self.output_buffer.clear();
            write!(self.terminal.stdout(), "{}", CLEAR_ALL)?;
            self.terminal.stdout().flush()?;
            return Ok(());
        }

        self.terminal.stdout().write_all(&self.output_buffer)?;
        self.terminal.stdout().flush()?;

        self.previous_buffer = self.canvas.buffer.clone();

        Ok(())
    }

    pub fn codex(&mut self) -> &mut Codex {
        &mut self.codex
    }

    /// Returns all input events since the last call.
    /// If there is no input, returns None.
    ///
    /// Eagerly evaluates all bytes read, and returns an `Event::Unknown` if
    /// the bytes cannot be parsed.
    pub fn poll_input(&mut self) -> TalosResult<Option<Vec<Event>>> {
        let _ = self.handle_signals()?;
        poll_input_into_events(&mut self.terminal.stdin(), &mut self.poll_input_buffer, self.max_poll_input_buffer, self.buffer_linear_growth_step)
    }

    fn handle_signals(&mut self) -> TalosResult<bool> {
        if check_terminate() {
            // We need to shut down now - No state will be saved, just restore the terminal
            self.terminal.restore()?;
            return Ok(true);
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
