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
    // Attributes
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    blink_slow: bool,
    reverse_colours: bool,
    hidden: bool,
    strikethrough: bool,
}

impl Style {
    pub fn builder() -> StyleBuilder {
        StyleBuilder::default()
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
        if self.bold {
            output_buffer.extend_from_slice(b"1;")
        }
        if self.dim {
            output_buffer.extend_from_slice(b"2;")
        }
        if self.italic {
            output_buffer.extend_from_slice(b"3;")
        }
        if self.underline {
            output_buffer.extend_from_slice(b"4;")
        }
        if self.blink_slow {
            output_buffer.extend_from_slice(b"5;")
        }
        if self.reverse_colours {
            output_buffer.extend_from_slice(b"7;")
        }
        if self.hidden {
            output_buffer.extend_from_slice(b"8;")
        }
        if self.strikethrough {
            output_buffer.extend_from_slice(b"9;")
        }
        output_buffer.extend_from_slice(b"m");
    }
}

impl Default for Style {
    fn default() -> Self {
        Style {
            fg: None,
            bg: None,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            blink_slow: false,
            reverse_colours: false,
            hidden: false,
            strikethrough: false,
        }
    }
}
