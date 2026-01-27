#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Key(Key),
    Char(char),
    Signal(Signal),
    Unknown(Vec<u8>), // Useful for debugging weird sequences
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Interrupt, // Ctrl+C
    Quit,      // Ctrl+\
    Suspend,   // Ctrl+Z
}
