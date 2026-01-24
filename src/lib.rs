use std::io::{Write, Read};

use builder::TalosBuilder;
use constants::ansi::TO_TOP_LEFT;
use error::TalosResult;
use render::{CCell, Canvas, Codex};
use terminal::term_io::TerminalIO;

mod error;
mod terminal;
mod builder;
mod render;
mod constants;
mod sys;

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
        // TODO: Intercept `SIGWINCH` and update terminal size
        
        self.output_buffer.clear();

        write!(self.output_buffer, "{}", TO_TOP_LEFT)?;

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let buffer_index = (x + y * self.size.0) as usize;

                if self.canvas.buffer[buffer_index] != self.previous_buffer[buffer_index] {
                    // update cursor to current position
                    write!(self.output_buffer, "\x1b[{};{}H", y + 1, x + 1)?;
                    let ccell = self.canvas.get_ccell(x, y);
                    let styled_char = {
                        // TODO: Add Style
                        self.codex.resolve(ccell.char)
                    };
                    write!(self.output_buffer, "{}", styled_char)?;
                }
            }
        }

        self.terminal.stdout().write_all(&self.output_buffer)?;
        self.terminal.stdout().flush()?;

        self.previous_buffer = self.canvas.buffer.clone();

        Ok(())
    }

    pub fn poll_input(&mut self) -> TalosResult<Option<Vec<u8>>> {
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

        if read_bytes <= buffer.len() {
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
}
