use crate::{error::Result as TalosResult, input::event::Event};

/// Xterm input parser module.
pub mod xterm;
pub use xterm::XtermParser;

/// Input parser trait
pub trait InputParser: Send + Sync {
    /// Parse the input buffer into events
    ///
    /// # Arguments
    /// * `buffer` - The buffer to parse
    /// * `event_buffer` - The buffer to store the parsed events in
    ///
    /// # Errors
    /// Returns an error if the input could not be parsed
    fn parse(&mut self, buffer: &[u8], event_buffer: &mut Vec<Event>) -> TalosResult<()>;
    /// Flush the input parser.
    /// Used when no more bytes are available but there may be partial input sequences
    /// remaining.
    ///
    /// The default implementation does nothing.
    ///
    /// # Arguments
    /// * `event_buffer` - The buffer to store the parsed events in
    fn flush(&mut self, _event_buffer: &mut Vec<Event>) {}
}
