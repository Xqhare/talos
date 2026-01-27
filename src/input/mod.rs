use std::io::{Read, Stdin};

mod event;
pub use event::Event;
mod parser;
use parser::parse_byte_stream;

use crate::error::TalosResult;

pub fn poll_input_into_events(
    std_in: &mut Stdin,
    max_poll_input_buffer: u16,
) -> TalosResult<Option<Vec<Event>>> {
    poll_input_bytes(std_in, max_poll_input_buffer)?
        .map(parse_byte_stream)
        .transpose()
}

fn poll_input_bytes(
    std_in: &mut Stdin,
    max_poll_input_buffer: u16,
) -> TalosResult<Option<Vec<u8>>> {
    let mut buffer = [0u8; 32];

    let read_bytes = match std_in.read(&mut buffer) {
        Ok(0) => return Ok(None),
        Ok(n) => n,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::WouldBlock {
                // Should never actually happen - TerminalIO is non-blocking
                return Ok(None);
            } else {
                return Err(e.into());
            }
        }
    };

    if read_bytes < buffer.len() {
        return Ok(Some(buffer[0..read_bytes].to_vec()));
    } else {
        let mut large_input = Vec::with_capacity(256);
        large_input.extend_from_slice(&buffer);

        let mut large_buffer = [0u8; 128];

        loop {
            match std_in.read(&mut large_buffer) {
                Ok(0) => return Ok(Some(large_input)),
                Ok(n) => large_input.extend_from_slice(&large_buffer[0..n]),
                Err(e) => return Err(e.into()),
            }
            if large_input.len() > max_poll_input_buffer as usize {
                return Ok(Some(large_input));
            }
        }
    }
}
