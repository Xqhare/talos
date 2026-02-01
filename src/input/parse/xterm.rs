use crate::{error::TalosResult, input::{event::{KeyCode, KeyEvent, KeyModifiers}, Event}};

use super::InputParser;

// TODO: Refactor Parser
//
// Current Limitations:
//    - The internal logic of `XtermParser` is still using "lookahead slicing".
//    - It technically violates strict ANSI specs (ignores intermediates like '!' or '?').
//    - The current parser will choke on complex responses like `\x1b[?1;2c`.
//    - Decision: Keep as-is for now. It works for 95% of standard keyboard inputs.
//
// Refactor into State Machine:
//    - When implementing Mouse Support or Terminal Queries (asking the terminal for cursor pos),
//      rewrite `XtermParser` into a proper State Machine (ref: Paul Williams DEC Parser).

/// A stateful parser that turns a stream of bytes into Input Events.
///
/// It maintains an internal buffer to handle cases where an escape sequence
/// or multi-byte character is split across multiple `poll` calls.
pub struct XtermParser {
    pending_buffer: Vec<u8>,
}

impl InputParser for XtermParser {
    fn new() -> Self {
        Self {
            // Just an arbitrary size - 32 bytes should be enough for most input
            pending_buffer: Vec::with_capacity(32),
        }
    }
    fn parse(&mut self, new_bytes: &[u8], output: &mut Vec<Event>) -> TalosResult<()> {
        self.pending_buffer.extend_from_slice(new_bytes);

        let mut bytes_consumed = 0;
        let mut i = 0;

        while i < self.pending_buffer.len() {
            let slice = &self.pending_buffer[i..];

            if let Some((event, len)) = self.try_parse_one(slice) {
                if let Some(ev) = event {
                    output.push(ev);
                }
                i += len;
                bytes_consumed = i;
            } else {
                if self.is_potential_incomplete_sequence(slice) {
                    break;
                }

                // 3. Unknown/Invalid byte. Consume to prevent infinite loops.
                // We wrap it in Unknown so the user can see what's happening if needed.
                output.push(Event::Unknown(vec![slice[0]]));
                i += 1;
                bytes_consumed = i;
            }
        }

        if bytes_consumed > 0 {
            // TODO: Deque with its O(1) `pop_front` is probably the best option here - But it breaks the slicing logic above.
            self.pending_buffer.drain(0..bytes_consumed);
        }

        Ok(())
    }
    fn flush(&mut self, output: &mut Vec<Event>) {
        if self.pending_buffer.len() == 1 && self.pending_buffer[0] == 0x1B {
            output.push(Event::KeyEvent(KeyEvent::new(KeyCode::Esc, KeyModifiers::default())));
            self.pending_buffer.clear();
        }
    }

}

impl XtermParser {

    /// Attempts to parse exactly one event from the front of the slice.
    fn try_parse_one(&self, slice: &[u8]) -> Option<(Option<Event>, usize)> {
        if slice.is_empty() {
            return None;
        }

        let byte = slice[0];

        if byte == 0x1B {
            if let Some(res) = self.parse_csi(slice) {
                return Some(res);
            }
            if let Some(res) = self.parse_ss3(slice) {
                return Some(res);
            }

            // Fallback: Just \x1b alone
            if slice.len() == 1 {
                return None; // Wait for potential following chars
            }

            // If we are here, it's \x1b followed by something that isn't [ or O.
            // Treat strictly as Esc key.
            return Some((
                Some(Event::KeyEvent(KeyEvent::new(KeyCode::Esc, KeyModifiers::default()))),
                1
            ));
        }

        if byte < 32 || byte == 127 {
            let (code, modifiers) = match byte {
                13 | 10 => (KeyCode::Enter, KeyModifiers::default()),
                9 => (KeyCode::Tab, KeyModifiers::default()),
                127 | 8 => (KeyCode::Backspace, KeyModifiers::default()),
                // Map Ctrl+A (1) through Ctrl+Z (26)
                1..=26 => {
                    let ch = (byte + 96u8) as char; // 1 -> 'a', 3 -> 'c'
                    (KeyCode::Char(ch), KeyModifiers { ctrl: true, ..KeyModifiers::default() })
                },
                // ESC (27) is handled above. 
                // 0 and 28-31 are less standard, ignore or map to Unknown?
                // For now, map to Unknown via the None return.
                _ => return None, 
            };
            
            let final_mods = sanitize_modifiers(modifiers);
            return Some((Some(Event::KeyEvent(KeyEvent::new(code, final_mods))), 1));
        }

        if let Some((ch, len)) = self.parse_utf8_char(slice) {
            return Some((
                Some(Event::KeyEvent(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::default()))),
                len
            ));
        }

        None
    }

    fn parse_csi(&self, slice: &[u8]) -> Option<(Option<Event>, usize)> {
        if slice.len() < 2 || slice[0] != 0x1B || slice[1] != b'[' {
            return None;
        }

        let mut idx = 2;
        let mut params = Vec::new();
        let mut current_param = 0u16;
        let mut has_param = false;

        // Parse Parameters
        while idx < slice.len() {
            let b = slice[idx];
            match b {
                b'0'..=b'9' => {
                    current_param = current_param.saturating_mul(10).saturating_add((b - b'0') as u16);
                    has_param = true;
                }
                b';' => {
                    params.push(current_param);
                    current_param = 0;
                    has_param = false; 
                }
                0x30..=0x3F => { /* Intermediate bytes, ignore */ }
                _ => break, // Reached final byte
            }
            idx += 1;
        }

        if idx >= slice.len() {
            return None; // Incomplete
        }

        if has_param {
            params.push(current_param);
        }

        let final_byte = slice[idx];
        let consumed = idx + 1;

        let modifier_param = if params.len() > 1 { params[1] } else { 1 };
        let modifiers = parse_modifier_param(modifier_param);

        let key_code = match final_byte {
            b'A' => Some(KeyCode::Up),
            b'B' => Some(KeyCode::Down),
            b'C' => Some(KeyCode::Right),
            b'D' => Some(KeyCode::Left),
            b'H' => Some(KeyCode::Home),
            b'F' => Some(KeyCode::End),
            b'Z' => {
                 return Some((
                     Some(Event::KeyEvent(KeyEvent::new(KeyCode::Tab, KeyModifiers { shift: true, none: false, ctrl: false, alt: false }))), 
                     consumed
                 ));
            },
            b'~' => {
                let id = params.first().copied().unwrap_or(0);
                match id {
                    1 => Some(KeyCode::Home),
                    2 => Some(KeyCode::Insert),
                    3 => Some(KeyCode::Delete),
                    4 => Some(KeyCode::End),
                    5 => Some(KeyCode::PageUp),
                    6 => Some(KeyCode::PageDown),
                    11 => Some(KeyCode::F(1)), 12 => Some(KeyCode::F(2)),
                    13 => Some(KeyCode::F(3)), 14 => Some(KeyCode::F(4)),
                    15 => Some(KeyCode::F(5)), 17 => Some(KeyCode::F(6)),
                    18 => Some(KeyCode::F(7)), 19 => Some(KeyCode::F(8)),
                    20 => Some(KeyCode::F(9)), 21 => Some(KeyCode::F(10)),
                    23 => Some(KeyCode::F(11)), 24 => Some(KeyCode::F(12)),
                    _ => None,
                }
            }
            _ => None,
        };

        if let Some(code) = key_code {
            Some((Some(Event::KeyEvent(KeyEvent::new(code, modifiers))), consumed))
        } else {
            // Valid CSI but unknown key
            Some((Some(Event::Unknown(slice[0..consumed].to_vec())), consumed))
        }
    }

    fn parse_ss3(&self, slice: &[u8]) -> Option<(Option<Event>, usize)> {
        if slice.len() < 3 || slice[0] != 0x1B || slice[1] != b'O' {
            if slice.len() < 3 && slice.starts_with(&[0x1B, b'O']) {
                return None; 
            }
            return None;
        }

        let key = match slice[2] {
            b'P' => Some(KeyCode::F(1)),
            b'Q' => Some(KeyCode::F(2)),
            b'R' => Some(KeyCode::F(3)),
            b'S' => Some(KeyCode::F(4)),
            _ => None,
        };

        if let Some(k) = key {
            Some((Some(Event::KeyEvent(KeyEvent::new(k, KeyModifiers::default()))), 3))
        } else {
            Some((Some(Event::Unknown(slice[0..3].to_vec())), 3))
        }
    }

    fn parse_utf8_char(&self, slice: &[u8]) -> Option<(char, usize)> {
        if slice.is_empty() { return None; }

        let first = slice[0];
        let len = match first {
            0x00..=0x7F => 1,
            0xC0..=0xDF => 2,
            0xE0..=0xEF => 3,
            0xF0..=0xF7 => 4,
            _ => return None,
        };

        if slice.len() < len {
            return None;
        }

        match str::from_utf8(&slice[..len]) {
            Ok(s) => Some((s.chars().next().unwrap(), len)),
            Err(_) => None, 
        }
    }

    fn is_potential_incomplete_sequence(&self, slice: &[u8]) -> bool {
        if slice.is_empty() { return false; }
        let b = slice[0];

        // Escape Sequence?
        if b == 0x1B { return true; }

        // UTF-8 Start Byte?
        if (b & 0xE0) == 0xC0 || (b & 0xF0) == 0xE0 || (b & 0xF8) == 0xF0 {
            let len = match b {
                0xC0..=0xDF => 2,
                0xE0..=0xEF => 3,
                0xF0..=0xF7 => 4,
                _ => 1,
            };
            if slice.len() < len { return true; }
        }

        false
    }
}

fn sanitize_modifiers(mods: KeyModifiers) -> KeyModifiers {
    let mut m = mods;
    if m.shift || m.ctrl || m.alt {
        m.none = false;
    } else {
        m.none = true;
    }
    m
}

fn parse_modifier_param(param: u16) -> KeyModifiers {
    // Standard Xterm modifiers:
    // 2: Shift, 3: Alt, 4: Shift+Alt, 5: Ctrl, 6: Shift+Ctrl, 7: Alt+Ctrl, 8: S+A+C
    let (shift, alt, ctrl) = match param {
        2 => (true, false, false),
        3 => (false, true, false),
        4 => (true, true, false),
        5 => (false, false, true),
        6 => (true, false, true),
        7 => (false, true, true),
        8 => (true, true, true),
        _ => (false, false, false),
    };

    KeyModifiers {
        none: !shift && !alt && !ctrl,
        shift,
        alt,
        ctrl,
    }
}
