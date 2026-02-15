use crate::input::{InputParser, Parser, XtermParser};

/// A builder for the `Parser` struct
///
/// # Example
/// ```rust
/// use talos::ParserBuilder;
///
/// let parser = ParserBuilder::default().build();
/// ```
#[must_use]
pub struct ParserBuilder {
    parser: Box<dyn InputParser>,
    buffer_linear_growth_step: usize,
    max_poll_input_buffer: usize,
    initial_poll_input_buffer_size: usize,
}

impl Default for ParserBuilder {
    fn default() -> Self {
        // Around 1MB of max input - PER FRAME
        let max_poll_input_buffer = 1024 * 1024;
        let buffer_linear_growth_step = 2048;
        let initial_poll_input_buffer_size = 512;
        ParserBuilder {
            parser: Box::new(XtermParser::new()),
            buffer_linear_growth_step,
            max_poll_input_buffer,
            initial_poll_input_buffer_size,
        }
    }
}

impl ParserBuilder {
    /// Sets the initial size of the input buffer
    ///
    /// The default is 512 bytes
    ///
    /// # Example
    /// ```rust
    /// use talos::ParserBuilder;
    ///
    /// let parser = ParserBuilder::default()
    ///     .with_initial_poll_input_buffer_size(1024)
    ///     .build();
    /// ```
    pub fn with_initial_poll_input_buffer_size(mut self, poll_input_buffer_size: usize) -> Self {
        self.initial_poll_input_buffer_size = poll_input_buffer_size;
        self
    }
    /// Sets the maximum size of the input buffer
    ///
    /// The default supports a maximum of around 1MB of input per frame
    ///
    /// # Example
    /// ```rust
    /// use talos::ParserBuilder;
    ///
    /// let parser = ParserBuilder::default()
    ///     .with_max_poll_input_buffer(2 * 1024 * 1024)
    ///     .build();
    /// ```
    pub fn with_max_poll_input_buffer(mut self, max_poll_input_buffer: usize) -> Self {
        self.max_poll_input_buffer = max_poll_input_buffer;
        self
    }
    /// Sets the amount to grow the input buffer by when it is full
    ///
    /// The default is 2048
    ///
    /// # Example
    /// ```rust
    /// use talos::ParserBuilder;
    ///
    /// let parser = ParserBuilder::default()
    ///     .with_buffer_linear_growth_step(4096)
    ///     .build();
    /// ```
    pub fn with_buffer_linear_growth_step(mut self, buffer_linear_growth_step: usize) -> Self {
        self.buffer_linear_growth_step = buffer_linear_growth_step;
        self
    }
    /// Sets the input parser to a custom one
    ///
    /// # Example
    /// ```rust
    /// use talos::{input::XtermParser, ParserBuilder};
    ///
    /// let parser = ParserBuilder::default()
    ///     .with_input_parser(Box::new(XtermParser::new()))
    ///     .build();
    /// ```
    pub fn with_input_parser(mut self, input_parser: Box<dyn InputParser>) -> Self {
        self.parser = input_parser;
        self
    }
    /// Builds the parser with the current settings
    ///
    /// # Example
    /// ```rust
    /// use talos::ParserBuilder;
    ///
    /// let parser = ParserBuilder::default().build();
    /// ```
    #[must_use]
    pub fn build(self) -> Parser {
        let poll_input_buffer = vec![0u8; self.initial_poll_input_buffer_size];

        let event_buffer = Vec::with_capacity(self.initial_poll_input_buffer_size);
        Parser {
            parser: self.parser,
            event_buffer,
            poll_input_buffer,
            buffer_linear_growth_step: self.buffer_linear_growth_step,
            max_poll_input_buffer: self.max_poll_input_buffer,
        }
    }
}
