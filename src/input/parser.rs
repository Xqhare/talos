use crate::error::TalosResult;

use super::Event;

// TODO: Add input parser that converts the bytes to:
// `Event::Key(Key::UP)`, `Event::Char('a')`,
pub fn parse_byte_stream(bytes: Vec<u8>) -> TalosResult<Vec<Event>> {
    todo!()
}
