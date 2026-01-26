use std::u16;

use crate::{error::TalosResult, render::{CCell, Canvas, Codex}, sys::register_signal_handlers, terminal::term_io::TerminalIO, Talos};

pub struct TalosBuilder {
    hide_cursor: bool,
    alternate_screen: bool,
    max_poll_input_buffer: u16,
    set_up_panic_handler: bool
}

impl Default for TalosBuilder {
    fn default() -> Self {
        Self { 
            hide_cursor: true, 
            alternate_screen: true,
            max_poll_input_buffer: 4096,
            set_up_panic_handler: true,
        }
    }
}

impl TalosBuilder {
    pub fn with_cursor(mut self) -> Self {
        self.hide_cursor = false;
        self
    }

    pub fn with_alternate_screen(mut self) -> Self {
        self.alternate_screen = true;
        self
    }

    /// The default supports 4kb of input per frame
    pub fn with_max_poll_input_buffer(mut self, max_poll_input_buffer: u16) -> Self {
        self.max_poll_input_buffer = max_poll_input_buffer;
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
        let size = terminal.size()?;
        let codex = Codex::new();

        let buffer_size = (size.0 as usize) * (size.1 as usize);
        let previous_buffer = vec![CCell::default(); buffer_size];
        // 10 bytes per cell may seem overkill, with a lot of styling bytes this may not
        //    even be enough!
        let output_buffer = Vec::with_capacity(buffer_size * 10);
        
        Ok(Talos {
            terminal,
            canvas: Canvas::new(size.1, size.0),
            size,
            codex,
            previous_buffer,
            output_buffer,
            max_poll_input_buffer: self.max_poll_input_buffer
        })
    }
}
