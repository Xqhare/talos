
use crate::{
    Talos, backend::{TerminalIO, sys::register_signal_handlers}, codex::Codex, error::TalosResult, input::Parser, render::{CCell, Canvas}
};

use super::ParserBuilder;

pub struct TalosBuilder {
    hide_cursor: bool,
    alternate_screen: bool,
    set_up_panic_handler: bool,
    input_parser: Parser,
}

impl Default for TalosBuilder {
    fn default() -> Self {
        let input_parser = {
            let tmp = ParserBuilder::default();
            tmp.build()
        };
        Self {
            hide_cursor: true,
            alternate_screen: true,
            set_up_panic_handler: true,
            input_parser,
        }
    }
}

impl TalosBuilder {
    pub fn with_input_parser(mut self, input_parser: Parser) -> Self {
        self.input_parser = input_parser;
        self
    }
    pub fn with_cursor(mut self) -> Self {
        self.hide_cursor = false;
        self
    }

    pub fn with_alternate_screen(mut self) -> Self {
        self.alternate_screen = true;
        self
    }

    /// Disables the panic handler hook
    pub fn without_panic_handler(mut self) -> Self {
        self.set_up_panic_handler = false;
        self
    }

    pub fn build(self) -> TalosResult<Talos> {
        if self.set_up_panic_handler {
            register_signal_handlers()?;
        }
        // Initialize TerminalIO based on these settings
        let terminal = TerminalIO::new(self.hide_cursor, self.alternate_screen)?;
        let (rows, cols) = terminal.size()?;
        let codex = Codex::new()?;

        let buffer_size = (cols as usize) * (rows as usize);
        let previous_buffer = vec![CCell::default(); buffer_size];
        // 10 bytes per cell may seem overkill, with a lot of styling bytes this may not
        //    even be enough!
        let output_buffer = Vec::with_capacity(buffer_size * 10);

        Ok(Talos {
            terminal,
            canvas: Canvas::new(cols, rows),
            size: (cols, rows),
            codex,
            previous_buffer,
            output_buffer,
            parser: self.input_parser,
        })
    }
}
