use crate::utils::constants::ansi::CONTROL_SEQUENCE_INTRO;

use super::Colour;

mod style_builder;
pub use style_builder::StyleBuilder;
use utils::{handle_bg, handle_fg};

mod utils;

/// Represents a style
///
/// The style is generated from a [`StyleBuilder`](struct.StyleBuilder.html)
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Style {
    fg: Option<Colour>,
    bg: Option<Colour>,
    /// Contains the bit flags for the style.
    /// From MSB to LSB: bold, dim, italic, underline, `blink_slow`, `reverse_colours`, hidden, strikethrough
    bit_flag: u8,
}

impl Style {
    /// Returns a new `StyleBuilder`
    #[must_use]
    pub fn builder() -> StyleBuilder {
        StyleBuilder::default()
    }

    /// Returns the foreground colour
    #[must_use]
    pub fn get_fg(&self) -> Option<Colour> {
        self.fg
    }

    /// Returns the background colour
    #[must_use]
    pub fn get_bg(&self) -> Option<Colour> {
        self.bg
    }

    /// Generates an ANSI control sequence from the style
    ///
    /// If a default Style is used, it will generate `\x1b[m` - Which will reset any previous style used
    pub fn generate(self, output_buffer: &mut Vec<u8>) {
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
