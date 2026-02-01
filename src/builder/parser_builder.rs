
use crate::input::{InputParser, Parser, XtermParser};


pub struct ParserBuilder {
    parser: Box<dyn InputParser>,
    buffer_linear_growth_step: usize,
    max_poll_input_buffer: usize,
    initial_poll_input_buffer_size: usize,
}

impl Default for ParserBuilder {
    fn default() -> Self {
        let max_poll_input_buffer = 1024 * 1024;
        let buffer_linear_growth_step = 4096;
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
    pub fn with_initial_poll_input_buffer_size(mut self, poll_input_buffer_size: usize) -> Self {
        self.initial_poll_input_buffer_size = poll_input_buffer_size;
        self
    }
    /// The default supports 4kb of input per frame
    pub fn with_max_poll_input_buffer(mut self, max_poll_input_buffer: usize) -> Self {
        self.max_poll_input_buffer = max_poll_input_buffer;
        self
    }
    pub fn with_buffer_linear_growth_step(mut self, buffer_linear_growth_step: usize) -> Self {
        self.buffer_linear_growth_step = buffer_linear_growth_step;
        self
    }
    pub fn with_input_parser(mut self, input_parser: Box<dyn InputParser>) -> Self {
        self.parser = input_parser;
        self
    }
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
