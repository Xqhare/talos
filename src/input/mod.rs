use std::{io::{Read, Stdin}, cmp::min};

mod event;
pub use event::Event;
mod parser;
use parser::parse_byte_stream;

use crate::error::TalosResult;

pub fn poll_input_into_events(
    std_in: &mut Stdin,
    poll_input_buffer: &mut Vec<u8>,
    max_poll_input_buffer: usize,
    buffer_linear_growth_step: usize
) -> TalosResult<Option<Vec<Event>>> {
    poll_input_bytes(std_in, poll_input_buffer, max_poll_input_buffer, buffer_linear_growth_step)?
        .as_deref()
        .map(parse_byte_stream)
        .transpose()
}

fn poll_input_bytes<'a>(
    std_in: &mut Stdin,
    poll_input_buffer: &'a mut Vec<u8>,
    max_poll_input_buffer: usize,
    buffer_linear_growth_step: usize,
) -> TalosResult<Option<&'a [u8]>> {
    let mut total_read = 0;

    loop {
        let available_space = &mut poll_input_buffer[total_read..];

        if available_space.is_empty() {
            let current_len = poll_input_buffer.len();

            if current_len >= max_poll_input_buffer as usize {
                break;
            }

            let growth = if current_len < buffer_linear_growth_step {
                if current_len == 0 { 32 } else { current_len.saturating_mul(2) }
            } else {
                buffer_linear_growth_step
            };

            let target_len = current_len.saturating_add(growth);
            let capped_len = min(target_len, max_poll_input_buffer as usize);

            if capped_len == current_len {
                break;
            }

            poll_input_buffer.resize(capped_len, 0);
            continue;
        }

        match std_in.read(available_space) {
            Ok(0) => break,
            Ok(n) => {
                total_read += n;
                if n < available_space.len() {
                    break;
                }
            },
            // ErrorKind::WouldBlock should never happen - but we gracefully exit and
            // return read bytes
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(e) => return Err(e.into()),
        }

    }
    
    if total_read == 0 {
        Ok(None)
    } else {
        Ok(Some(&poll_input_buffer[..total_read]))
    }
}
