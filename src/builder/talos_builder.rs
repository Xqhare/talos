use crate::{error::TalosResult, render::Canvas, terminal::term_io::TerminalIO, Talos};

pub struct TalosBuilder {
    hide_cursor: bool,
    alternate_screen: bool,
}

impl Default for TalosBuilder {
    fn default() -> Self {
        Self { 
            hide_cursor: true, 
            alternate_screen: true 
        }
    }
}

impl TalosBuilder {
    pub fn with_cursor(mut self) -> Self {
        self.hide_cursor = false;
        self
    }

    pub fn build(self) -> TalosResult<Talos> {
        // Initialize TerminalIO based on these settings
        let terminal = TerminalIO::new(self.hide_cursor, self.alternate_screen)?;
        
        Ok(Talos {
            terminal,
            canvas: Canvas::new(),
            size: terminal.size()?,
        })
    }
}
