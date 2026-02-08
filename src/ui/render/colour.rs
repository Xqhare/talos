pub const FG_PRE_DIGIT_NORMAL: u8 = 3;
pub const FG_PRE_DIGIT_BRIGHT: u8 = 9;

pub const BG_PRE_DIGIT_NORMAL: u8 = 4;
pub const BG_PRE_DIGIT_BRIGHT: u8 = 10;

/// Colour is a representation of a terminal colour
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColourMode {
    /// One byte RGB
    RgbBit(u8),
}

impl ColourMode {
    /// Returns the index of the colour for the terminal
    #[must_use]
    pub fn decode(self) -> u8 {
        match self {
            ColourMode::RgbBit(n) => n,
        }
    }
}

pub const TRUE_COLOURMODE_SIGNAL_BIT: u8 = 2;

/// Three byte RGB
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrueColour {
    /// Three byte RGB
    RGB(u8, u8, u8),
}

impl TrueColour {
    /// Returns the index of the colour for the terminal
    #[must_use]
    pub fn decode(self) -> (u8, u8, u8) {
        match self {
            TrueColour::RGB(r, g, b) => (r, g, b),
        }
    }
}
