#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    KeyEvent(KeyEvent),
    MouseEvent(MouseEvent),
    Unknown(Vec<u8>), // Useful for debugging weird sequences
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub column: u16,
    pub row: u16,
    pub modifiers: KeyModifiers,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseEventKind {
    Down(MouseButton),
    Up(MouseButton),
    Drag(MouseButton),
    Moved,
    ScrollUp,
    ScrollDown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyEvent {
    #[must_use] 
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
