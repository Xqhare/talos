use crate::error::TalosResult;

use super::Event;

mod xterm;
pub use xterm::XtermParser;

pub trait InputParser {
    fn new() -> Self;
    fn parse(&mut self, new_bytes: &[u8], output: &mut Vec<Event>) -> TalosResult<()>;
}
