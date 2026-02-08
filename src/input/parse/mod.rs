use crate::error::TalosResult;

use super::Event;

mod xterm;
pub use xterm::XtermParser;

pub trait InputParser {
    /// Creates a new parser
    fn new() -> Self
    where
        Self: Sized;
    /// Parses new bytes into events
    ///
    /// # Arguments
    /// * `new_bytes` - The new bytes to parse
    /// * `output` - The output buffer to write events to
    ///
    /// # Errors
    /// Returns an error if the parser failed to parse the bytes
    fn parse(&mut self, new_bytes: &[u8], output: &mut Vec<Event>) -> TalosResult<()>;
    /// Flushes the parser
    fn flush(&mut self, output: &mut Vec<Event>);
}
