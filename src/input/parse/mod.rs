use crate::error::TalosResult;

use super::Event;

mod xterm;
pub use xterm::XtermParser;

/// The input parser interface trait
///
/// If you want to create a custom input parser, you need to implement this trait - take a look at
/// the [`Event`](struct.Event.html) struct for more information.
///
/// # Example
/// ```rust
/// use talos::{
///     error::TalosResult,
///     input::{Event, InputParser},
/// };
///
/// struct MyParser;
///
/// impl InputParser for MyParser {
///     fn new() -> Self {
///         MyParser
///     }
///
///     fn parse(&mut self, new_bytes: &[u8], output: &mut Vec<Event>) -> TalosResult<()> {
///         // Parse bytes and push events to output
///         Ok(())
///     }
///
///     fn flush(&mut self, output: &mut Vec<Event>) {
///         // Flush any remaining bytes and push events to output
///     }
/// }
/// ```
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
    ///
    /// # Arguments
    /// * `output` - The output buffer to write events to
    fn flush(&mut self, output: &mut Vec<Event>);
}
