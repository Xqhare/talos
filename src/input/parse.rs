use crate::{error::TalosResult, input::event::Event};

/// Xterm input parser
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
}
