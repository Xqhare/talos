use crate::constants::ansi::CONTROL_SEQUENCE_INTRO;

use super::Colour;

mod style_builder;
pub use style_builder::StyleBuilder;
use utils::{handle_bg, handle_fg};

mod utils;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Style {
    fg: Option<Colour>,
    bg: Option<Colour>,
    /// Contains the bit flags for the style.
    /// From MSB to LSB: bold, dim, italic, underline, blink_slow, reverse_colours, hidden, strikethrough
    bit_flag: u8,
}

impl Style {
    pub fn builder() -> StyleBuilder {
        StyleBuilder::default()
    }

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

    /// Generates an ANSI control sequence from the style
    ///
    /// If a default Style is used, it will generate `\x1b[m` - Which will reset any previous style used
    pub fn generate(self, output_buffer: &mut Vec<u8>) {
        output_buffer.extend_from_slice(CONTROL_SEQUENCE_INTRO.as_bytes());
        if let Some(fg) = self.fg {
            handle_fg(fg, output_buffer);
            output_buffer.extend_from_slice(b";");
        }
        if let Some(bg) = self.bg {
            handle_bg(bg, output_buffer);
            output_buffer.extend_from_slice(b";");
        }
        if self.bit_flag != 0 {
            if self.bit_flag & 0b10000000 != 0 { output_buffer.extend_from_slice(b"1;"); }
            if self.bit_flag & 0b01000000 != 0 { output_buffer.extend_from_slice(b"2;"); }
            if self.bit_flag & 0b00100000 != 0 { output_buffer.extend_from_slice(b"3;"); }
            if self.bit_flag & 0b00010000 != 0 { output_buffer.extend_from_slice(b"4;"); }
            if self.bit_flag & 0b00001000 != 0 { output_buffer.extend_from_slice(b"5;"); }
            if self.bit_flag & 0b00000100 != 0 { output_buffer.extend_from_slice(b"7;"); }
            if self.bit_flag & 0b00000010 != 0 { output_buffer.extend_from_slice(b"9;"); }
            if self.bit_flag & 0b00000001 != 0 { output_buffer.extend_from_slice(b"6;"); }
        }
        output_buffer.extend_from_slice(b"m");
    }
}

impl Default for Style {
    fn default() -> Self {
        Style {
            fg: None,
            bg: None,
            bit_flag: 0,
        }
    }
}
