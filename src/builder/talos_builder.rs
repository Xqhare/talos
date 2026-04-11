use crate::{
    Talos,
    backend::{TerminalIO, sys::register_signal_handlers},
    codex::Codex,
    error::Result as TalosResult,
    input::Parser,
    render::{CCell, Canvas},
};

use super::ParserBuilder;

/// A builder for the `Talos` struct
///
/// # Example
/// ```rust,no_run
/// use talos::Talos;
///
/// let talos = Talos::builder().build();
/// assert!(talos.is_ok());
/// ```
pub struct TalosBuilder {
    /// Whether to hide the terminal cursor.
    hide_cursor: bool,
    /// Whether to use the alternate screen.
    alternate_screen: bool,
    /// Whether to set up a panic handler.
    set_up_panic_handler: bool,
    /// The input parser to use.
    input_parser: Parser,
}

impl Default for TalosBuilder {
    #[inline]
    fn default() -> Self {
        let input_parser = ParserBuilder::default().build();
        Self {
            hide_cursor: true,
            alternate_screen: true,
            set_up_panic_handler: true,
            input_parser,
        }
    }
}

impl TalosBuilder {
    /// Sets the input parser to a custom one
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{ParserBuilder, Talos};
    ///
    /// let input_parser = ParserBuilder::default().build();
    /// let talos = Talos::builder().with_input_parser(input_parser).build();
    /// ```
    #[inline]
    #[must_use]
    pub fn with_input_parser(mut self, input_parser: Parser) -> Self {
        self.input_parser = input_parser;
        self
    }

    /// Enables the Terminal cursor
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let talos = Talos::builder().with_cursor().build();
    /// ```
    #[inline]
    #[must_use]
    pub fn with_cursor(mut self) -> Self {
        self.hide_cursor = false;
        self
    }

    /// Uses the alternate screen
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let talos = Talos::builder().with_alternate_screen().build();
    /// ```
    #[inline]
    #[must_use]
    pub fn with_alternate_screen(mut self) -> Self {
        self.alternate_screen = true;
        self
    }

    /// Disables the panic handler hook
    ///
    /// This can lead to unrecoverable panics, and returning the Terminal in a partially configured
    /// and not reset (broken) state.
    ///
    /// If disableing the panic handler, `Talos` expects you to set up a custom panic handler, called
    /// before building `Talos`.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let talos = Talos::builder().without_panic_handler().build();
    /// ```
    #[inline]
    #[must_use]
    pub fn without_panic_handler(mut self) -> Self {
        self.set_up_panic_handler = false;
        self
    }

    /// Build the `Talos` instance
    ///
    /// # Errors
    /// Returns an error if the terminal could not be initialized
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let talos = Talos::builder().build();
    /// assert!(talos.is_ok());
    /// ```
    #[inline]
    pub fn build(self) -> TalosResult<Talos> {
        // Set up panic handler as the very first thing
        if self.set_up_panic_handler {
            register_signal_handlers()?;
        }

        let terminal = TerminalIO::new(self.hide_cursor, self.alternate_screen)?;
        let (rows, cols) = TerminalIO::size()?;

        let codex = Codex::new();

        let len = usize::from(cols).saturating_mul(usize::from(rows));
        let previous_buffer = vec![CCell::default(); len];
        let mut output_buffer = Vec::with_capacity(len.saturating_mul(10));
        output_buffer.clear();

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
