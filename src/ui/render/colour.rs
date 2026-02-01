pub const FG_PRE_DIGIT_NORMAL: u8 = 3;
pub const FG_PRE_DIGIT_BRIGHT: u8 = 9;

pub const BG_PRE_DIGIT_NORMAL: u8 = 4;
pub const BG_PRE_DIGIT_BRIGHT: u8 = 10;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Colour {
    Normal(Normal),
    Bright(Bright),
    Extended(Extended),
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Extended {
    ColourMode(ColourMode),
    TrueColour(TrueColour),
}

pub const COLOURMODE_SIGNAL_BIT: u8 = 5;
pub const EXTENDED_FG_BIT: u8 = 38;
pub const EXTENDED_BG_BIT: u8 = 48;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColourMode {
    RgbBit(u8),
}

impl ColourMode {
    pub fn decode(self) -> u8 {
        match self {
            ColourMode::RgbBit(n) => n,
        }
    }
}

pub const TRUE_COLOURMODE_SIGNAL_BIT: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrueColour {
    RGB(u8, u8, u8),
}

impl TrueColour {
    pub fn decode(self) -> (u8, u8, u8) {
        match self {
            TrueColour::RGB(r, g, b) => (r, g, b),
        }
    }
}
