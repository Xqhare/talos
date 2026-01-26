use std::io::{Read, Write};

use builder::TalosBuilder;
use constants::ansi::{CLEAR_ALL, TO_TOP_LEFT};
use error::TalosResult;
use render::{CCell, Canvas, Codex};
use sys::{check_resize, check_terminate};
use terminal::term_io::TerminalIO;

mod error;
mod terminal;
mod builder;
mod render;
mod constants;
mod sys;

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
    max_poll_input_buffer: u16
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

        let mut prev_cell: u16 = 0;

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let buffer_index = (x + y * self.size.0) as usize;

                if self.canvas.buffer[buffer_index] != self.previous_buffer[buffer_index] {
                    let ccell = self.canvas.get_ccell(x, y);
                    let styled_char = {
                        // TODO: Add Style
                        self.codex.resolve(ccell.char)
                    };
                    // update cursor to current position - check if prev cell
                    // is to the left, if yes just write the char
                    if x - prev_cell == 1 {
                        write!(self.output_buffer, "{}", styled_char)?;
                    } else {
                        write!(self.output_buffer, "\x1b[{};{}H", y + 1, x + 1)?;
                    }
                }
                prev_cell = x;
            }
        }

        if self.handle_signals()? {
            // Resized! - Just show one blank frame - should be imperceivable anyways
            self.output_buffer.clear();
            write!(self.terminal.stdout(), "{}", CLEAR_ALL)?;
            self.terminal.stdout().flush()?;
            return Ok(())
        }

        self.terminal.stdout().write_all(&self.output_buffer)?;
        self.terminal.stdout().flush()?;

        self.previous_buffer = self.canvas.buffer.clone();

        Ok(())
    }

    // TODO: Add input parser that converts the bytes to:
    // `Event::Key(Key::UP)`, `Event::Char('a')`,
    pub fn poll_input(&mut self) -> TalosResult<Option<Vec<u8>>> {
        let _ = self.handle_signals()?;
        let mut buffer = [0u8; 32];

        let read_bytes = match self.terminal.stdin().read(&mut buffer) {
            Ok(0) => return Ok(None),
            Ok(n) => n,
            Err(e) => if e.kind() == std::io::ErrorKind::WouldBlock {
                return Ok(None)
            } else {
                return Err(e.into())
            }
        };

        if read_bytes < buffer.len() {
            return Ok(Some(buffer[0..read_bytes].to_vec()))
        } else {
            let mut large_input = Vec::with_capacity(256);
            large_input.extend_from_slice(&buffer);

            let mut large_buffer = [0u8; 128];

            loop {
                match self.terminal.stdin().read(&mut large_buffer) {
                    Ok(0) => return Ok(Some(large_input)),
                    Ok(n) => large_input.extend_from_slice(&large_buffer[0..n]),
                    Err(e) => return Err(e.into())
                }
                if large_input.len() > self.max_poll_input_buffer as usize {
                    return Ok(Some(large_input))
                }
            }
        }
    }

    pub fn codex(&mut self) -> &mut Codex {
        &mut self.codex
    }

    fn handle_signals(&mut self) -> TalosResult<bool> {
        if check_terminate() {
            // We need to shut down now - No state will be saved, just restore the terminal
            self.terminal.restore()?;
            return Ok(true)
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
