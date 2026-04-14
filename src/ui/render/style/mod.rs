use crate::utils::constants::ansi::CONTROL_SEQUENCE_INTRO;

use super::Colour;

mod style_builder;
pub use style_builder::StyleBuilder;
use utils::{handle_bg, handle_fg};

mod utils;

/// Represents a style
///
/// The style is generated from a [`StyleBuilder`](struct.StyleBuilder.html)
///
/// # Example
/// ```rust
/// use talos::render::{Colour, Normal, Style};
///
/// let style = Style::builder()
///     .set_fg(Colour::Normal(Normal::Red))
///     .set_bg(Colour::Normal(Normal::Blue))
///     .set_bold(true)
///     .build();
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Style {
    fg: Option<Colour>,
    bg: Option<Colour>,
    /// Contains the bit flags for the style.
    /// From MSB to LSB: bold, dim, italic, underline, `blink_slow`, `reverse_colours`, hidden, strikethrough
    bit_flag: u8,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: None,
            bg: None,
            bit_flag: 0,
        }
    }
}

impl Style {
    /// Returns a new `StyleBuilder`
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Style;
    ///
    /// let builder = Style::builder();
    /// ```
    #[must_use]
    pub fn builder() -> StyleBuilder {
        StyleBuilder::default()
    }

    /// Returns a new `StyleBuilder` with the current style as the default values
    ///
    /// Does not modify the current style
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{Colour, Normal, Style};
    ///
    /// let style = Style::builder()
    ///     .set_fg(Colour::Normal(Normal::Red))
    ///     .set_bg(Colour::Normal(Normal::Black))
    ///     .build();
    ///
    /// let style2 = style.new_from_self()
    ///     .set_fg(Colour::Normal(Normal::Blue))
    ///     .build();
    ///
    /// assert_eq!(style.get_fg(), Some(Colour::Normal(Normal::Red)));
    /// assert_eq!(style.get_bg(), Some(Colour::Normal(Normal::Black)));
    ///
    /// assert_eq!(style2.get_fg(), Some(Colour::Normal(Normal::Blue)));
    /// assert_eq!(style2.get_bg(), Some(Colour::Normal(Normal::Black)));
    /// ```
    #[must_use]
    pub fn new_from_self(self) -> StyleBuilder {
        StyleBuilder::new(self.fg, self.bg, self.bit_flag)
    }
    /// Sets the foreground colour
    ///
    /// The foreground colour can be set to `None`. This will be rendered as the default
    /// foreground color of the Terminal Session
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{Colour, Normal, Style};
    ///
    /// let style = Style::builder()
    ///     .set_fg(Colour::Normal(Normal::Red))
    ///     .build();
    /// ```
    pub fn set_fg(mut self, fg: Option<Colour>) -> Self {
        self.fg = fg;
        self
    }

    /// Sets the background colour
    ///
    /// The background colour can be set to `None`. This will be rendered as the default
    /// background color of the Terminal Session
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{Colour, Normal, Style};
    ///
    /// let style = Style::builder()
    ///     .set_bg(Colour::Normal(Normal::Blue))
    ///     .build();
    /// ```
    pub fn set_bg(mut self, bg: Option<Colour>) -> Self {
        self.bg = bg;
        self
    }

    /// Returns the foreground colour
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{Colour, Normal, Style};
    ///
    /// let style = Style::builder()
    ///     .set_fg(Colour::Normal(Normal::Red))
    ///     .build();
    /// assert_eq!(style.get_fg(), Some(Colour::Normal(Normal::Red)));
    /// ```
    #[must_use]
    pub fn get_fg(&self) -> Option<Colour> {
        self.fg
    }

    /// Returns the background colour
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{Colour, Normal, Style};
    ///
    /// let style = Style::builder()
    ///     .set_bg(Colour::Normal(Normal::Blue))
    ///     .build();
    /// assert_eq!(style.get_bg(), Some(Colour::Normal(Normal::Blue)));
    /// ```
    #[must_use]
    pub fn get_bg(&self) -> Option<Colour> {
        self.bg
    }

    /// Returns whether the bold attribute is set
    #[must_use]
    pub fn get_bold(&self) -> Option<bool> {
        Some(self.bit_flag & 0b1000_0000 != 0)
    }

    /// Returns whether the dim attribute is set
    #[must_use]
    pub fn get_dim(&self) -> Option<bool> {
        Some(self.bit_flag & 0b0100_0000 != 0)
    }

    /// Returns whether the italic attribute is set
    #[must_use]
    pub fn get_italic(&self) -> Option<bool> {
        Some(self.bit_flag & 0b0010_0000 != 0)
    }

    /// Returns whether the underline attribute is set
    #[must_use]
    pub fn get_underline(&self) -> Option<bool> {
        Some(self.bit_flag & 0b0001_0000 != 0)
    }

    /// Returns whether the blink attribute is set
    #[must_use]
    pub fn get_blink(&self) -> Option<bool> {
        Some(self.bit_flag & 0b0000_1000 != 0)
    }

    /// Returns whether the reverse attribute is set
    #[must_use]
    pub fn get_reverse(&self) -> Option<bool> {
        Some(self.bit_flag & 0b0000_0100 != 0)
    }

    /// Returns whether the hidden attribute is set
    #[must_use]
    pub fn get_hidden(&self) -> Option<bool> {
        Some(self.bit_flag & 0b0000_0010 != 0)
    }

    /// Returns whether the strikethrough attribute is set
    #[must_use]
    pub fn get_strikethrough(&self) -> Option<bool> {
        Some(self.bit_flag & 0b0000_0001 != 0)
    }

    /// Generates an ANSI control sequence that transforms the terminal style from `from` to `self`
    pub(crate) fn generate_diff(self, from: Style, output_buffer: &mut Vec<u8>) {
        if self == from {
            return;
        }

        // If something was removed (e.g. bold was on, now it's off), we need a full reset
        // This is simpler than sending individual "off" codes for now.
        let bit_flag_removed = (from.bit_flag & !self.bit_flag) != 0;
        let fg_removed = from.fg.is_some() && self.fg.is_none();
        let bg_removed = from.bg.is_some() && self.bg.is_none();

        if bit_flag_removed || fg_removed || bg_removed {
            self.generate(output_buffer);
            return;
        }

        // Only add things that changed
        output_buffer.extend_from_slice(CONTROL_SEQUENCE_INTRO.as_bytes());
        let mut first = true;

        if self.fg != from.fg
            && let Some(fg) = self.fg
        {
            handle_fg(fg, output_buffer);
            first = false;
        }

        if self.bg != from.bg
            && let Some(bg) = self.bg
        {
            if !first {
                output_buffer.push(b';');
            }
            handle_bg(bg, output_buffer);
            first = false;
        }

        let new_bits = self.bit_flag & !from.bit_flag;
        if new_bits != 0 {
            let bits = [
                (0b1000_0000, "1"),
                (0b0100_0000, "2"),
                (0b0010_0000, "3"),
                (0b0001_0000, "4"),
                (0b0000_1000, "5"),
                (0b0000_0100, "7"),
                (0b0000_0010, "9"),
                (0b0000_0001, "6"),
            ];
            for (mask, code) in bits {
                if new_bits & mask != 0 {
                    if !first {
                        output_buffer.push(b';');
                    }
                    output_buffer.extend_from_slice(code.as_bytes());
                    first = false;
                }
            }
        }

        if first {
            // If we didn't actually add any sequences, remove the CSI
            output_buffer.truncate(output_buffer.len() - CONTROL_SEQUENCE_INTRO.len());
        } else {
            output_buffer.push(b'm');
        }
    }

    /// Generates an ANSI control sequence from the style
    ///
    /// If a default Style is used, it will generate `\x1b[m` - Which will reset any previous style used
    pub(crate) fn generate(self, output_buffer: &mut Vec<u8>) {
        output_buffer.extend_from_slice(CONTROL_SEQUENCE_INTRO.as_bytes());

        // TODO: Optimise: We don't need to push a 0 if the style is the exact same as the previous
        // Keep in mind: This was added to remove 'ghost characters' in the terminal, left after
        // resizing
        //
        // 1. try to optimise failed with no content being rendered if using a layouter
        output_buffer.push(b'0');

        if let Some(fg) = self.fg {
            output_buffer.extend_from_slice(b";");
            handle_fg(fg, output_buffer);
        }
        if let Some(bg) = self.bg {
            output_buffer.extend_from_slice(b";");
            handle_bg(bg, output_buffer);
        }
        if self.bit_flag != 0 {
            if self.bit_flag & 0b1000_0000 != 0 {
                output_buffer.extend_from_slice(b";1");
            }
            if self.bit_flag & 0b0100_0000 != 0 {
                output_buffer.extend_from_slice(b";2");
            }
            if self.bit_flag & 0b0010_0000 != 0 {
                output_buffer.extend_from_slice(b";3");
            }
            if self.bit_flag & 0b0001_0000 != 0 {
                output_buffer.extend_from_slice(b";4");
            }
            if self.bit_flag & 0b0000_1000 != 0 {
                output_buffer.extend_from_slice(b";5");
            }
            if self.bit_flag & 0b0000_0100 != 0 {
                output_buffer.extend_from_slice(b";7");
            }
            if self.bit_flag & 0b0000_0010 != 0 {
                output_buffer.extend_from_slice(b";9");
            }
            if self.bit_flag & 0b0000_0001 != 0 {
                output_buffer.extend_from_slice(b";6");
            }
        }
        output_buffer.extend_from_slice(b"m");
    }
}
