use std::io::Write;

use builder::TalosBuilder;
use constants::ansi::TO_TOP_LEFT;
use error::TalosResult;
use render::Canvas;
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
    // Terminal Size
    /// Width, Height
    size: (Width, Height),
}

impl Talos {
    pub fn builder() -> TalosBuilder {
        TalosBuilder::default()
    }

    pub fn begin_frame(&mut self) {
        self.canvas.clear();
    }

    pub fn present(&mut self) -> TalosResult<()> {
        write!(self.terminal, "{}", TO_TOP_LEFT)?;
        for row in self.canvas.buffer.iter() {
            for cell in row.iter() {
                write!(self.terminal, "{}", cell.char)?;
            }
            write!(self.terminal, "\r\n")?;
        }
        self.terminal.stdout().flush()?;
        Ok(())
    }
}
