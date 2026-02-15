/// An input event
///
/// # Example
/// ```rust
/// use talos::input::{Event, KeyEvent, KeyCode, KeyModifiers};
///
/// let event = Event::KeyEvent(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::default()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// A key event
    KeyEvent(KeyEvent),
    /// A mouse event
    MouseEvent(MouseEvent),
    /// An unknown event, used as a fallback
    Unknown(Vec<u8>), // Useful for debugging weird sequences
}

/// A mouse event
///
/// # Example
/// ```rust
/// use talos::input::{MouseEvent, MouseEventKind, MouseButton, KeyModifiers};
///
/// let event = MouseEvent {
///    kind: MouseEventKind::Down(MouseButton::Left),
///    column: 10,
///    row: 20,
///    modifiers: KeyModifiers::default(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseEvent {
    /// The kind of mouse event
    pub kind: MouseEventKind,
    /// The column where the mouse event occurred
    pub column: u16,
    /// The row where the mouse event occurred
    pub row: u16,
    /// The modifiers that were active when the mouse event occurred
    pub modifiers: KeyModifiers,
}

/// The kind of mouse event
///
/// # Example
/// ```rust
/// use talos::input::{MouseEventKind, MouseButton};
///
/// let kind = MouseEventKind::Down(MouseButton::Left);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseEventKind {
    /// A mouse button was pressed
    Down(MouseButton),
    /// A mouse button was released
    Up(MouseButton),
    /// A mouse button was dragged
    Drag(MouseButton),
    /// The mouse was moved
    Moved,
    /// The mouse wheel was scrolled upwards
    ScrollUp,
    /// The mouse wheel was scrolled downwards
    ScrollDown,
}

/// A mouse button
///
/// # Example
/// ```rust
/// use talos::input::MouseButton;
///
/// let button = MouseButton::Left;
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseButton {
    /// The left mouse button
    Left,
    /// The middle mouse button
    Middle,
    /// The right mouse button
    Right,
}

/// A key event
///
/// # Example
/// ```rust
/// use talos::input::{KeyEvent, KeyCode, KeyModifiers};
///
/// let event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::default());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    /// The key code
    pub code: KeyCode,
    /// The modifiers that were active when the key event occurred
    pub modifiers: KeyModifiers,
}

impl KeyEvent {
    /// Create a new key event
    ///
    /// # Arguments
    /// * `code` - The key code
    /// * `modifiers` - The modifiers that were active when the key event occurred
    ///
    /// # Example
    /// ```rust
    /// use talos::input::{KeyEvent, KeyCode, KeyModifiers};
    ///
    /// let event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::default());
    /// ```
    #[must_use]
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

/// A key on the keyboard
///
/// # Example
/// ```rust
/// use talos::input::KeyCode;
///
/// let key_code = KeyCode::Char('a');
/// ```
#[allow(missing_docs)]
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

/// The modifiers that were active when the key event occurred
///
/// # Example
/// ```rust
/// use talos::input::KeyModifiers;
///
/// let modifiers = KeyModifiers {
///    none: true,
///    shift: false,
///    ctrl: false,
///    alt: false,
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct KeyModifiers {
    /// No modifiers
    pub none: bool,
    /// The shift key
    pub shift: bool,
    /// The control key
    pub ctrl: bool,
    /// The alt key
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
