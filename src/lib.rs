use std::io::Write;

use builder::TalosBuilder;
use constants::ansi::{CLEAR_ALL, TO_TOP_LEFT};
use error::TalosResult;
use render::{Canvas, Codex};
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
    current_buffer: Option<Vec<u8>>,
    previous_buffer: Option<Vec<u8>>,
}

impl Talos {
    pub fn builder() -> TalosBuilder {
        TalosBuilder::default()
    }

    pub fn begin_frame(&mut self) {
        if self.current_buffer.is_some() {
            self.previous_buffer = self.current_buffer.take();
        }
        self.canvas.clear();
    }

    pub fn present(&mut self) -> TalosResult<()> {
        // TODO: Intercept `SIGWINCH` and update terminal size
        
        let mut buffer = Vec::new();
        write!(buffer, "{}", CLEAR_ALL)?;
        write!(buffer, "{}", TO_TOP_LEFT)?;

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let ccell = self.canvas.get_ccell(x, y);
                let styled_char = {
                    // TODO: Add Style
                    self.codex.resolve(ccell.char)
                };
                write!(buffer, "{}", styled_char)?;
            }
            write!(buffer, "\n")?;
        }

        // if prev buffer is some, take the diff with current buffer and write only changed
        // cells
        //
        // This is wrong, I can feel it
        if let Some(prev_buffer) = self.previous_buffer.as_ref() {
            let mut diff = Vec::new();
            for i in 0..buffer.len() {
                if prev_buffer[i] != buffer[i] {
                    diff.push(buffer[i]);
                }
            }
            self.terminal.stdout().write(&diff)?;
        }

        self.terminal.stdout().write(&buffer)?;
        Ok(())
    }

    pub fn codex(&mut self) -> &mut Codex {
        &mut self.codex
    }
}
