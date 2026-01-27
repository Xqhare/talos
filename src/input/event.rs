#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    KeyEvent(KeyEvent),
    Unknown(Vec<u8>), // Useful for debugging weird sequences
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyEvent {
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Esc,
    Enter,
    Backspace,
    Tab,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    F(u8), // F1 - F12
    Char(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyModifiers {
    pub none: bool,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

impl Default for KeyModifiers {
    fn default() -> Self {
        Self {
            none: true,
            shift: false,
            ctrl: false,
            alt: false,
        }
    }
}
