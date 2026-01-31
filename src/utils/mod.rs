use std::io::Write;

use crate::error::TalosResult;

mod terminal;
pub use terminal::TerminalIO;

mod sys;

mod constants;

pub fn write_all_bytes<T: Write>(writer: &mut T, bytes: &[u8]) -> TalosResult<()> {
    writer.write_all(bytes).map_err(Into::into)
}

pub fn push_u16_as_ascii(buffer: &mut Vec<u8>, mut n: u16) {
    if n == 0 {
        buffer.push(b'0');
        return;
    }
    
    // We parse digits in reverse order (123 -> 3, 2, 1)
    let start_index = buffer.len();
    while n > 0 {
        let digit = (n % 10) as u8;
        buffer.push(b'0' + digit); // '0' is 0x30
        n /= 10;
    }
    
    // Reverse the digits back to correct order
    buffer[start_index..].reverse();
}
