pub const FG_PRE_DIGIT_NORMAL: u8 = 3;
pub const FG_PRE_DIGIT_BRIGHT: u8 = 9;

pub const BG_PRE_DIGIT_NORMAL: u8 = 4;
pub const BG_PRE_DIGIT_BRIGHT: u8 = 10;

/// Colour is a representation of a terminal colour
///
/// # Example
/// ```rust
/// use talos::render::{Colour, Normal};
///
/// let color = Colour::Normal(Normal::Red);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colour {
    /// Normal 8 colours
    Normal(Normal),
    /// Bright 8 colours
    Bright(Bright),
    /// More colours
    Extended(Extended),
}

/// Normal 8 colours
///
/// # Example
/// ```rust
/// use talos::render::Normal;
///
/// let color = Normal::Red;
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Normal {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Normal {
    /// Returns the index of the colour for the terminal
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Normal;
    ///
    /// let color = Normal::Red;
    /// assert_eq!(color.decode(), 1);
    /// ```
    #[must_use]
    pub fn decode(self) -> u8 {
        match self {
            Normal::Black => 0,
            Normal::Red => 1,
            Normal::Green => 2,
            Normal::Yellow => 3,
            Normal::Blue => 4,
            Normal::Magenta => 5,
            Normal::Cyan => 6,
            Normal::White => 7,
        }
    }
}

/// Bright 8 colours
///
/// # Example
/// ```rust
/// use talos::render::Bright;
///
/// let color = Bright::Red;
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum Bright {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Bright {
    /// Returns the index of the colour for the terminal
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Bright;
    ///
    /// let color = Bright::Red;
    /// assert_eq!(color.decode(), 1);
    /// ```
    #[must_use]
    pub fn decode(self) -> u8 {
        match self {
            Bright::Black => 0,
            Bright::Red => 1,
            Bright::Green => 2,
            Bright::Yellow => 3,
            Bright::Blue => 4,
            Bright::Magenta => 5,
            Bright::Cyan => 6,
            Bright::White => 7,
        }
    }
}

/// More colours - All RGB colours
///
/// # Example
/// ```rust
/// use talos::render::{Extended, ColourMode, TrueColour};
///
/// let color = Extended::ColourMode(ColourMode::RgbBit(123));
/// let true_color = Extended::TrueColour(TrueColour::RGB(10, 20, 30));
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Extended {
    /// Simple one byte RGB
    ColourMode(ColourMode),
    /// Large three byte RGB (no alpha)
    TrueColour(TrueColour),
}

pub const COLOURMODE_SIGNAL_BIT: u8 = 5;
pub const EXTENDED_FG_BIT: u8 = 38;
pub const EXTENDED_BG_BIT: u8 = 48;

/// One byte RGB
///
/// # Example
/// ```rust
/// use talos::render::ColourMode;
///
/// let color = ColourMode::RgbBit(123);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColourMode {
    /// One byte RGB
    RgbBit(u8),
}

impl ColourMode {
    /// Returns the index of the colour for the terminal
    ///
    /// # Example
    /// ```rust
    /// use talos::render::ColourMode;
    ///
    /// let color = ColourMode::RgbBit(123);
    /// assert_eq!(color.decode(), 123);
    /// ```
    #[must_use]
    pub fn decode(self) -> u8 {
        match self {
            ColourMode::RgbBit(n) => n,
        }
    }
}

pub const TRUE_COLOURMODE_SIGNAL_BIT: u8 = 2;

/// Three byte RGB
///
/// # Example
/// ```rust
/// use talos::render::TrueColour;
///
/// let color = TrueColour::RGB(10, 20, 30);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrueColour {
    /// Three byte RGB
    RGB(u8, u8, u8),
}

impl TrueColour {
    /// Returns the index of the colour for the terminal
    ///
    /// # Example
    /// ```rust
    /// use talos::render::TrueColour;
    ///
    /// let color = TrueColour::RGB(10, 20, 30);
    /// assert_eq!(color.decode(), (10, 20, 30));
    /// ```
    #[must_use]
    pub fn decode(self) -> (u8, u8, u8) {
        match self {
            TrueColour::RGB(r, g, b) => (r, g, b),
        }
    }
}
