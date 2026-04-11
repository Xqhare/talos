use super::Colour;

/// Style builder module.
mod style_builder;
pub use style_builder::StyleBuilder;

/// Style utility functions.
mod utils;
pub(crate) use utils::handle_colour;

/// The style of a cell on the canvas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Style {
    /// Foreground color.
    fg: Option<Colour>,
    /// Background color.
    bg: Option<Colour>,
    /// Bit flags for style attributes.
    bit_flag: u8,
}

impl Style {
    /// Create a new style builder
    #[inline]
    #[must_use]
    pub fn builder() -> StyleBuilder {
        StyleBuilder::default()
    }
    /// Create a new style builder from the current style
    #[inline]
    #[must_use]
    pub fn new_from_self(&self) -> StyleBuilder {
        StyleBuilder::new(self.fg, self.bg, self.bit_flag)
    }
    /// Set the foreground color
    #[inline]
    pub fn set_fg(&mut self, fg: Option<Colour>) {
        self.fg = fg;
    }
    /// Set the background color
    #[inline]
    pub fn set_bg(&mut self, bg: Option<Colour>) {
        self.bg = bg;
    }
    /// Get the foreground color
    #[inline]
    #[must_use]
    pub fn get_fg(&self) -> Option<Colour> {
        self.fg
    }
    /// Get the background color
    #[inline]
    #[must_use]
    pub fn get_bg(&self) -> Option<Colour> {
        self.bg
    }

    /// Whether the bold attribute is set.
    #[inline]
    #[must_use]
    fn is_bold(self) -> bool {
        self.bit_flag & 0b1000_0000 != 0
    }
    /// Whether the dim attribute is set.
    #[inline]
    #[must_use]
    fn is_dim(self) -> bool {
        self.bit_flag & 0b0100_0000 != 0
    }
    /// Whether the italic attribute is set.
    #[inline]
    #[must_use]
    fn is_italic(self) -> bool {
        self.bit_flag & 0b0010_0000 != 0
    }
    /// Whether the underline attribute is set.
    #[inline]
    #[must_use]
    fn is_underline(self) -> bool {
        self.bit_flag & 0b0001_0000 != 0
    }
    /// Whether the blink attribute is set.
    #[inline]
    #[must_use]
    fn is_blink(self) -> bool {
        self.bit_flag & 0b0000_1000 != 0
    }
    /// Whether the reverse attribute is set.
    #[inline]
    #[must_use]
    fn is_reverse(self) -> bool {
        self.bit_flag & 0b0000_0100 != 0
    }
    /// Whether the hidden attribute is set.
    #[inline]
    #[must_use]
    fn is_hidden(self) -> bool {
        self.bit_flag & 0b0000_0010 != 0
    }
    /// Whether the strikethrough attribute is set.
    #[inline]
    #[must_use]
    fn is_strikethrough(self) -> bool {
        self.bit_flag & 0b0000_0001 != 0
    }

    /// Generate the ANSI escape sequence for the style
    pub fn generate(&self, buffer: &mut Vec<u8>) {
        let _ = buffer.write_all(b"\x1b[0m");
        if let Some(fg) = self.fg {
            handle_colour(fg, true, buffer);
        }
        if let Some(bg) = self.bg {
            handle_colour(bg, false, buffer);
        }
        if self.is_bold() {
            let _ = buffer.write_all(b"\x1b[1m");
        }
        if self.is_dim() {
            let _ = buffer.write_all(b"\x1b[2m");
        }
        if self.is_italic() {
            let _ = buffer.write_all(b"\x1b[3m");
        }
        if self.is_underline() {
            let _ = buffer.write_all(b"\x1b[4m");
        }
        if self.is_blink() {
            let _ = buffer.write_all(b"\x1b[5m");
        }
        if self.is_reverse() {
            let _ = buffer.write_all(b"\x1b[7m");
        }
        if self.is_hidden() {
            let _ = buffer.write_all(b"\x1b[8m");
        }
        if self.is_strikethrough() {
            let _ = buffer.write_all(b"\x1b[9m");
        }
    }

    /// Generate the ANSI escape sequence for the difference between two styles
    pub fn generate_diff(&self, previous: Style, buffer: &mut Vec<u8>) {
        if self == &previous {
            return;
        }

        // If any attribute changed from true to false, we must reset
        // Check if any bits set in previous are NOT set in self
        if (previous.bit_flag & !self.bit_flag) != 0 {
            self.generate(buffer);
            return;
        }

        // Otherwise, only apply the changes
        if self.fg != previous.fg {
            if let Some(fg) = self.fg {
                handle_colour(fg, true, buffer);
            } else {
                let _ = buffer.write_all(b"\x1b[39m");
            }
        }
        if self.bg != previous.bg {
            if let Some(bg) = self.bg {
                handle_colour(bg, false, buffer);
            } else {
                let _ = buffer.write_all(b"\x1b[49m");
            }
        }
        if self.is_bold() && !previous.is_bold() {
            let _ = buffer.write_all(b"\x1b[1m");
        }
        if self.is_dim() && !previous.is_dim() {
            let _ = buffer.write_all(b"\x1b[2m");
        }
        if self.is_italic() && !previous.is_italic() {
            let _ = buffer.write_all(b"\x1b[3m");
        }
        if self.is_underline() && !previous.is_underline() {
            let _ = buffer.write_all(b"\x1b[4m");
        }
        if self.is_blink() && !previous.is_blink() {
            let _ = buffer.write_all(b"\x1b[5m");
        }
        if self.is_reverse() && !previous.is_reverse() {
            let _ = buffer.write_all(b"\x1b[7m");
        }
        if self.is_hidden() && !previous.is_hidden() {
            let _ = buffer.write_all(b"\x1b[8m");
        }
        if self.is_strikethrough() && !previous.is_strikethrough() {
            let _ = buffer.write_all(b"\x1b[9m");
        }
    }
}

use std::io::Write;
