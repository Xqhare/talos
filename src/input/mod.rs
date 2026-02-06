use std::{cmp::min, io::Read};

mod event;
pub use event::{Event, KeyCode, KeyEvent, KeyModifiers};
mod parse;
pub use parse::{InputParser, XtermParser};

use crate::TalosResult;

pub struct Parser {
    pub parser: Box<dyn InputParser>,
    pub event_buffer: Vec<Event>,
    pub poll_input_buffer: Vec<u8>,
    pub buffer_linear_growth_step: usize,
    pub max_poll_input_buffer: usize,
}

pub fn poll_input_bytes<'a, R: Read>(
    std_in: &mut R,
    poll_input_buffer: &'a mut Vec<u8>,
    max_poll_input_buffer: usize,
    buffer_linear_growth_step: usize,
) -> TalosResult<Option<&'a [u8]>> {
    let mut total_read = 0;

    loop {
        let available_space = &mut poll_input_buffer[total_read..];

        if available_space.is_empty() {
            let current_len = poll_input_buffer.len();

            if current_len >= max_poll_input_buffer {
                break;
            }

            // Slightly overly complicated logic to grow the buffer
            // Could change, but a lot of though about how the memory should be managed went into it - so keep it
            let new_len = if current_len < buffer_linear_growth_step {
                if current_len == 0 {
                    32
                } else {
                    current_len.saturating_mul(2)
                }
            } else {
                current_len.saturating_add(buffer_linear_growth_step)
            };

            let target_len = new_len;
            let capped_len = min(target_len, max_poll_input_buffer);

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
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_poll_input_parsing_branches() -> TalosResult<()> {
        // A stream simulating:
        // - 'a'
        // - Backspace
        // - Up Arrow
        // - F1
        // - '€' (UTF-8 Multi-byte)
        // - CTRL+a
        let input_bytes: Vec<u8> = vec![
            b'a', 0x7f, 0x1b, b'[', b'A', 0x1b, b'O', b'P', 0xe2, 0x82, 0xac, 0x01,
        ];

        // Cursor implements Read
        let mut reader = Cursor::new(input_bytes);

        // Setup Parser and Buffers
        let mut poll_buffer = vec![0u8; 32];
        let mut event_buffer = Vec::new();
        let mut parser = XtermParser::new();

        // 1. Poll Bytes
        let bytes_opt = poll_input_bytes(&mut reader, &mut poll_buffer, 1024, 1024)?;

        let bytes = bytes_opt.ok_or("Polling should return Some")?;

        // 2. Parse Bytes into Events
        assert!(
            parser.parse(bytes, &mut event_buffer).is_ok(),
            "Parsing should succeed"
        );

        // 3. Verify
        assert_eq!(event_buffer.len(), 6, "Should parse exactly 6 events");

        assert_eq!(
            event_buffer[0],
            Event::KeyEvent(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::default()))
        );
        assert_eq!(
            event_buffer[1],
            Event::KeyEvent(KeyEvent::new(KeyCode::Backspace, KeyModifiers::default()))
        );
        assert_eq!(
            event_buffer[2],
            Event::KeyEvent(KeyEvent::new(KeyCode::Up, KeyModifiers::default()))
        );
        assert_eq!(
            event_buffer[3],
            Event::KeyEvent(KeyEvent::new(KeyCode::F(1), KeyModifiers::default()))
        );
        assert_eq!(
            event_buffer[4],
            Event::KeyEvent(KeyEvent::new(KeyCode::Char('€'), KeyModifiers::default()))
        );

        // Ctrl+C parsing check
        let mut ctrl_c_mods = KeyModifiers::default();
        ctrl_c_mods.ctrl = true;
        ctrl_c_mods.none = false;
        assert_eq!(
            event_buffer[5],
            Event::KeyEvent(KeyEvent::new(KeyCode::Char('a'), ctrl_c_mods))
        );

        Ok(())
    }

    #[test]
    fn test_empty_input() -> TalosResult<()> {
        let mut reader = Cursor::new(vec![]);
        let mut buffer = vec![0u8; 32];

        let result = poll_input_bytes(&mut reader, &mut buffer, 1024, 1024)?;

        assert!(result.is_none(), "Empty input should return None");

        Ok(())
    }
}
