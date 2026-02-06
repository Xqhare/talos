use crate::render::Colour;

use super::Style;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StyleBuilder {
    fg: Option<Colour>,
    bg: Option<Colour>,
    bit_flag: u8,
}

impl StyleBuilder {
    pub fn set_fg(mut self, fg: Colour) -> Self {
        self.fg = Some(fg);
        self
    }

    pub fn set_bg(mut self, bg: Colour) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn set_bold(mut self, bold: bool) -> Self {
        if bold {
            self.bit_flag |= 0b10000000;
        } else {
            self.bit_flag &= !0b10000000;
        }
        self
    }

    pub fn set_dim(mut self, dim: bool) -> Self {
        if dim {
            self.bit_flag |= 0b01000000;
        } else {
            self.bit_flag &= !0b01000000;
        }
        self
    }

    pub fn set_italic(mut self, italic: bool) -> Self {
        if italic {
            self.bit_flag |= 0b00100000;
        } else {
            self.bit_flag &= !0b00100000;
        }
        self
    }

    pub fn set_underline(mut self, underline: bool) -> Self {
        if underline {
            self.bit_flag |= 0b00010000;
        } else {
            self.bit_flag &= !0b00010000;
        }
        self
    }

    pub fn set_blink(mut self, blink: bool) -> Self {
        if blink {
            self.bit_flag |= 0b00001000;
        } else {
            self.bit_flag &= !0b00001000;
        }
        self
    }

    pub fn set_reverse(mut self, reverse: bool) -> Self {
        if reverse {
            self.bit_flag |= 0b00000100;
        } else {
            self.bit_flag &= !0b00000100;
        }
        self
    }

    pub fn set_hidden(mut self, hidden: bool) -> Self {
        if hidden {
            self.bit_flag |= 0b00000010;
        } else {
            self.bit_flag &= !0b00000010;
        }
        self
    }

    pub fn set_strikethrough(mut self, strikethrough: bool) -> Self {
        if strikethrough {
            self.bit_flag |= 0b00000001;
        } else {
            self.bit_flag &= !0b00000001;
        }
        self
    }

    pub fn build(self) -> Style {
        Style {
            fg: self.fg,
            bg: self.bg,
            bit_flag: self.bit_flag,
        }
    }
}

impl Default for StyleBuilder {
    fn default() -> Self {
        Self {
            fg: None,
            bg: None,
            bit_flag: 0,
        }
    }
}
