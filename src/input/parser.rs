use crate::error::TalosResult;
use super::{Event, Key, Signal};
use std::str;

// TODO: Rework - this is a mess of copied (how old school of me) and vibed code
//
// 1. Probably move to a buffer like for polling the input bytes
// 2. Do I need to add a better text handler? Passed in I get UTF-8 bytes, but internally I use
//    Glyphs - do I convert here or check for validity or naw?
// 3. A lot of if nesting -> State machine?
//      - ANSI seems to follow a structure:
//          - \x1b
//          - [ or O
//          - More specific chars
pub fn parse_byte_stream(bytes: &[u8]) -> TalosResult<Vec<Event>> {
    let mut events = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let byte = bytes[i];

        // Handle Escape Sequences (starting with \x1b)
        if byte == 0x1B {
            if i + 1 < bytes.len() {
                let next = bytes[i + 1];
                
                // CSI Sequences: \x1b [ ...
                if next == b'[' {
                    if let Some((event, consumed)) = parse_csi(&bytes[i..]) {
                        events.push(event);
                        i += consumed;
                        continue;
                    }
                }
                // SS3 Sequences: \x1b O ... (often F1-F4)
                else if next == b'O' {
                     if let Some((event, consumed)) = parse_ss3(&bytes[i..]) {
                        events.push(event);
                        i += consumed;
                        continue;
                    }
                }
            }
            
            // If strictly just \x1b or unknown sequence, treat as Esc key
            events.push(Event::Key(Key::Esc));
            i += 1;
            continue;
        }

        // Handle Control Characters (0x00 - 0x1F)
        if byte < 32 || byte == 127 {
            match byte {
                // Signals
                3 => events.push(Event::Signal(Signal::Interrupt)), // Ctrl+C
                26 => events.push(Event::Signal(Signal::Suspend)),  // Ctrl+Z
                28 => events.push(Event::Signal(Signal::Quit)),     // Ctrl+\
                
                // Standard Control Keys
                13 | 10 => events.push(Event::Key(Key::Enter)),     // CR or LF
                127 | 8 => events.push(Event::Key(Key::Backspace)), // DEL or BS
                9 => events.push(Event::Key(Key::Tab)),             // Tab
                
                _ => {} // Ignore other obscure control codes for now
            }
            i += 1;
            continue;
        }

        // UTF-8 input text
        if let Some((ch, len)) = parse_utf8_char(&bytes[i..]) {
            events.push(Event::Char(ch));
            i += len;
        } else {
            i += 1;
        }
    }

    Ok(events)
}

/// Parse "Control Sequence Introducer" (CSI) - \x1b[...
fn parse_csi(slice: &[u8]) -> Option<(Event, usize)> {
    // Minimal safety check: needs at least ESC + [ + Command
    if slice.len() < 3 || slice[0] != 0x1B || slice[1] != b'[' { 
        return None; 
    }

    match slice[2] {
        b'A' => return Some((Event::Key(Key::Up), 3)),
        b'B' => return Some((Event::Key(Key::Down), 3)),
        b'C' => return Some((Event::Key(Key::Right), 3)),
        b'D' => return Some((Event::Key(Key::Left), 3)),
        b'H' => return Some((Event::Key(Key::Home), 3)),
        b'F' => return Some((Event::Key(Key::End), 3)),
        _ => {}
    }

    // Handle Tilde Sequences: \x1b[<num>~ (Home, End, Del, PgUp...)
    // Example: \x1b[3~ (Delete)
    if let Some(tilde_pos) = slice.iter().position(|&b| b == b'~') {
        let num_str = str::from_utf8(&slice[2..tilde_pos]).ok()?;
        let num: u8 = num_str.parse().ok()?;
        
        let key = match num {
            1 => Key::Home,
            2 => Key::Insert,
            3 => Key::Delete,
            4 => Key::End,
            5 => Key::PageUp,
            6 => Key::PageDown,
            // F-keys 1-12 often map to 11-24 with some gaps
            11 => Key::F(1), 12 => Key::F(2), 13 => Key::F(3), 14 => Key::F(4),
            15 => Key::F(5), 17 => Key::F(6), 18 => Key::F(7), 19 => Key::F(8),
            20 => Key::F(9), 21 => Key::F(10), 23 => Key::F(11), 24 => Key::F(12),
            _ => return None, 
        };
        
        return Some((Event::Key(key), tilde_pos + 1));
    }

    None
}

/// Parse "Single Shift Select 3" (SS3) - \x1bO...
fn parse_ss3(slice: &[u8]) -> Option<(Event, usize)> {
    if slice.len() < 3 || slice[0] != 0x1B || slice[1] != b'O' { 
        return None; 
    }

    let key = match slice[2] {
        b'P' => Key::F(1),
        b'Q' => Key::F(2),
        b'R' => Key::F(3),
        b'S' => Key::F(4),
        _ => return None,
    };

    Some((Event::Key(key), 3))
}

/// Helper to pull one valid char off the front of a byte slice
/// Eagerly determines UTF-8 length using leading bits
/// Not guaranteed to be character - but at least its a valid utf8 byte
fn parse_utf8_char(slice: &[u8]) -> Option<(char, usize)> {
    if slice.is_empty() { return None; }

    let first = slice[0];
    
    // Determine UTF-8 length
    let len = match first.leading_ones() {
        0 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        _ => return None, // Invalid UTF-8 - not a char!
    };

    if slice.len() < len {
        return None; 
    }

    match str::from_utf8(&slice[..len]) {
        Ok(s) => Some((s.chars().next()?, len)),
        Err(_) => None,
    }
}
