use crate::{
    error::TalosResult,
    input::{
        Event,
        event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind},
    },
};

use super::InputParser;

/// States for the Xterm state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParserState {
    Normal,
    Esc,
    Csi,
    Ss3,
}

/// The Xterm input parser
pub struct XtermParser {
    state: ParserState,
    params: Vec<u16>,
    current_param: u16,
    has_param_digit: bool,
    pending_buffer: Vec<u8>,
    is_sgr_mouse: bool,
}

impl InputParser for XtermParser {
    fn new() -> Self {
        Self {
            state: ParserState::Normal,
            params: Vec::with_capacity(4),
            current_param: 0,
            has_param_digit: false,
            pending_buffer: Vec::with_capacity(32),
            is_sgr_mouse: false,
        }
    }

    fn parse(&mut self, new_bytes: &[u8], output: &mut Vec<Event>) -> TalosResult<()> {
        for &byte in new_bytes {
            match self.state {
                ParserState::Normal => self.handle_normal(byte, output),
                ParserState::Esc => self.handle_esc(byte, output),
                ParserState::Csi => self.handle_csi(byte, output),
                ParserState::Ss3 => self.handle_ss3(byte, output),
            }
        }
        Ok(())
    }

    fn flush(&mut self, output: &mut Vec<Event>) {
        if self.state == ParserState::Esc {
            output.push(Event::KeyEvent(KeyEvent::new(
                KeyCode::Esc,
                KeyModifiers::default(),
            )));
            self.reset_state();
        }
    }
}

impl XtermParser {
    /// Resets the parser state
    fn reset_state(&mut self) {
        self.state = ParserState::Normal;
        self.params.clear();
        self.current_param = 0;
        self.has_param_digit = false;
        self.pending_buffer.clear();
        self.is_sgr_mouse = false;
    }

    /// Handles a byte in the normal state
    fn handle_normal(&mut self, byte: u8, output: &mut Vec<Event>) {
        if byte == 0x1B {
            self.state = ParserState::Esc;
            return;
        }

        // Handle Control Codes and UTF-8
        if byte < 32 || byte == 127 {
            if let Some(event) = parse_control_byte(byte) {
                output.push(event);
            }
        } else {
            // Basic UTF-8 accumulation
            self.pending_buffer.push(byte);
            if let Some((ch, len)) = try_parse_utf8(&self.pending_buffer) {
                output.push(Event::KeyEvent(KeyEvent::new(
                    KeyCode::Char(ch),
                    KeyModifiers::default(),
                )));
                self.pending_buffer.drain(..len);
            }
        }
    }

    /// Handles a byte in the ESC state
    fn handle_esc(&mut self, byte: u8, output: &mut Vec<Event>) {
        match byte {
            b'[' => self.state = ParserState::Csi,
            b'O' => self.state = ParserState::Ss3,
            _ => {
                // Not a known sequence starter, treat previous ESC as key and re-process byte
                output.push(Event::KeyEvent(KeyEvent::new(
                    KeyCode::Esc,
                    KeyModifiers::default(),
                )));
                self.state = ParserState::Normal;
                self.handle_normal(byte, output);
            }
        }
    }

    /// Handles a byte in the CSI state
    fn handle_csi(&mut self, byte: u8, output: &mut Vec<Event>) {
        match byte {
            b'<' => self.is_sgr_mouse = true,
            b'0'..=b'9' => {
                self.current_param = self
                    .current_param
                    .saturating_mul(10)
                    .saturating_add(u16::from(byte - b'0'));
                self.has_param_digit = true;
            }
            b';' => {
                self.params.push(self.current_param);
                self.current_param = 0;
                self.has_param_digit = false;
            }
            0x40..=0x7E => {
                // Final Byte
                if self.has_param_digit {
                    self.params.push(self.current_param);
                }

                let event = if self.is_sgr_mouse {
                    self.parse_sgr_mouse(byte)
                } else {
                    self.finalize_csi(byte)
                };

                if let Some(ev) = event {
                    output.push(ev);
                }
                self.reset_state();
            }
            _ => {} // Intermediate bytes (0x20..0x2F) or parameter bytes (0x30..0x3F) ignored for now
        }
    }

    /// Handles a byte in the SS3 state
    fn handle_ss3(&mut self, byte: u8, output: &mut Vec<Event>) {
        let code = match byte {
            b'P' => Some(KeyCode::F(1)),
            b'Q' => Some(KeyCode::F(2)),
            b'R' => Some(KeyCode::F(3)),
            b'S' => Some(KeyCode::F(4)),
            _ => None,
        };

        if let Some(k) = code {
            output.push(Event::KeyEvent(KeyEvent::new(k, KeyModifiers::default())));
        } else {
            output.push(Event::Unknown(vec![0x1B, b'O', byte]));
        }
        self.reset_state();
    }

    /// Finalizes a CSI sequence
    fn finalize_csi(&self, final_byte: u8) -> Option<Event> {
        let modifier_param = if self.params.len() > 1 {
            self.params[1]
        } else {
            1
        };
        let modifiers = parse_modifier_param(modifier_param);

        let key_code = match final_byte {
            b'A' => Some(KeyCode::Up),
            b'B' => Some(KeyCode::Down),
            b'C' => Some(KeyCode::Right),
            b'D' => Some(KeyCode::Left),
            b'H' => Some(KeyCode::Home),
            b'F' => Some(KeyCode::End),
            b'Z' => {
                return Some(Event::KeyEvent(KeyEvent::new(
                    KeyCode::Tab,
                    KeyModifiers {
                        shift: true,
                        none: false,
                        ctrl: false,
                        alt: false,
                    },
                )));
            }
            b'~' => match self.params.first().copied().unwrap_or(0) {
                1 => Some(KeyCode::Home),
                2 => Some(KeyCode::Insert),
                3 => Some(KeyCode::Delete),
                4 => Some(KeyCode::End),
                5 => Some(KeyCode::PageUp),
                6 => Some(KeyCode::PageDown),
                11..=15 | 17..=21 | 23..=24 => {
                    let f_map = [0, 1, 2, 3, 4, 5, 0, 6, 7, 8, 9, 10, 0, 11, 12];
                    let idx = (self.params[0] - 11) as usize;
                    if idx < f_map.len() && f_map[idx] != 0 {
                        Some(KeyCode::F(f_map[idx]))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        };

        key_code.map(|code| Event::KeyEvent(KeyEvent::new(code, modifiers)))
    }

    /// Finalizes a SGR mouse sequence
    fn parse_sgr_mouse(&self, final_byte: u8) -> Option<Event> {
        // SGR format: ESC [ < button ; column ; row (m or M)
        if self.params.len() < 3 {
            return None;
        }

        let b_code = self.params[0];
        let col = self.params[1].saturating_sub(1);
        let row = self.params[2].saturating_sub(1);

        // Bit 2: Shift, Bit 3: Alt, Bit 4: Ctrl
        let modifiers = KeyModifiers {
            shift: (b_code & 4) != 0,
            alt: (b_code & 8) != 0,
            ctrl: (b_code & 16) != 0,
            none: (b_code & 28) == 0,
        };

        // b_code bits 0-1 and 6-7 determine the button/event type
        // final_byte 'M' is press/drag, 'm' is release
        let kind = match (final_byte, b_code & 0x63) {
            (b'M', 0) => MouseEventKind::Down(MouseButton::Left),
            (b'M', 1) => MouseEventKind::Down(MouseButton::Middle),
            (b'M', 2) => MouseEventKind::Down(MouseButton::Right),
            (b'm', 0) => MouseEventKind::Up(MouseButton::Left),
            (b'm', 1) => MouseEventKind::Up(MouseButton::Middle),
            (b'm', 2) => MouseEventKind::Up(MouseButton::Right),
            (b'M', 32) => MouseEventKind::Drag(MouseButton::Left),
            (b'M', 33) => MouseEventKind::Drag(MouseButton::Middle),
            (b'M', 34) => MouseEventKind::Drag(MouseButton::Right),
            (b'M', 35) => MouseEventKind::Moved,
            (b'M', 64) => MouseEventKind::ScrollUp,
            (b'M', 65) => MouseEventKind::ScrollDown,
            _ => return None,
        };

        Some(Event::MouseEvent(MouseEvent {
            kind,
            column: col,
            row,
            modifiers,
        }))
    }
}

fn parse_modifier_param(param: u16) -> KeyModifiers {
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

fn parse_control_byte(byte: u8) -> Option<Event> {
        let (code, modifiers) = match byte {
            13 | 10 => (KeyCode::Enter, KeyModifiers::default()),
            9 => (KeyCode::Tab, KeyModifiers::default()),
            127 | 8 => (KeyCode::Backspace, KeyModifiers::default()),
            1..=26 => {
                let ch = (byte + 96u8) as char;
                (
                    KeyCode::Char(ch),
                    KeyModifiers {
                        ctrl: true,
                        none: false,
                        ..KeyModifiers::default()
                    },
                )
            }
            _ => return None,
        };
        Some(Event::KeyEvent(KeyEvent::new(code, modifiers)))
    }

    fn try_parse_utf8(buffer: &[u8]) -> Option<(char, usize)> {
        if buffer.is_empty() {
            return None;
        }
        let first = buffer[0];
        let len = match first {
            0x00..=0x7F => 1,
            0xC0..=0xDF => 2,
            0xE0..=0xEF => 3,
            0xF0..=0xF7 => 4,
            _ => return None,
        };

        if buffer.len() >= len {
            std::str::from_utf8(&buffer[..len])
                .ok()
                .and_then(|s| s.chars().next().map(|ch| (ch, len)))
        } else {
            None
        }
    }
