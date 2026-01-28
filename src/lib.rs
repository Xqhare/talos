use std::io::Write;

use builder::TalosBuilder;
use constants::ansi::{CLEAR_ALL, TO_TOP_LEFT};
use error::TalosResult;
use input::Event;
use input::poll_input_bytes;
use input::Parser;
use render::{CCell, Canvas, Codex};
use sys::{check_resize, check_terminate};
use terminal::term_io::TerminalIO;
use utils::write_all_bytes;

mod builder;
mod constants;
mod error;
mod input;
mod render;
mod sys;
mod terminal;
mod utils;

pub use render::Colour;
pub use render::Style;

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

    // Input - TODO: Move into separate struct
    parser: Parser,
    event_buffer: Vec<Event>,
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

    // TODO: return a value that indicates if the terminal was resized
    // probably a bool - false if resize, true if not (to say "present exited the way you wanted"
    // or "Hey, I resized! present did not do what you expected")
    //
    // The new size is stored in `self.size` and would thus not need to be returned
    pub fn present(&mut self) -> TalosResult<()> {
        let _resized = self.handle_signals()?;

        self.output_buffer.clear();

        write_all_bytes(&mut self.output_buffer, TO_TOP_LEFT.as_bytes())?;

        let mut prev_x_cell: u16 = 0;

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let buffer_index = (x + y * self.size.0) as usize;

                if self.canvas.buffer[buffer_index] != self.previous_buffer[buffer_index] {
                    let ccell = self.canvas.get_ccell(x, y);

                    // Cursor handling
                    if x - prev_x_cell != 1 {
                        let bytes = [
                            0x1b,
                            b'[',
                            (x as u8).saturating_add(1),
                            b';',
                            (y as u8).saturating_add(1),
                            b'H',
                        ];
                        write_all_bytes(&mut self.output_buffer, &bytes)?;
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
            return Ok(());
        }

        self.terminal.stdout().write_all(&self.output_buffer)?;
        self.terminal.stdout().flush()?;

        // Pointer swapping of the buffers
        std::mem::swap(&mut self.previous_buffer, &mut self.canvas.buffer);

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
    pub fn poll_input(&mut self) -> TalosResult<Option<&[Event]>> {
        let _ = self.handle_signals()?;

        self.event_buffer.clear();

        if let Some(bytes) = poll_input_bytes(
            &mut self.terminal.stdin(),
            &mut self.poll_input_buffer,
            self.max_poll_input_buffer,
            self.buffer_linear_growth_step,
        )? {
            self.parser.parse(bytes, &mut self.event_buffer)?;
        }

        Ok(Some(self.event_buffer.as_slice()))
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
