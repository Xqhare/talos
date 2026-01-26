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
    pub fn generate(self) -> String {
        let beginning = CONTROL_SEQUENCE_INTRO;

        let fg = if let Some(fg) = self.fg {
            Some(handle_fg(fg))
        } else {
            None
        };
        let bg = if let Some(bg) = self.bg {
            Some(handle_bg(bg))  
        } else {
            None
        };

        let mut attrs = Vec::new();
        if self.bold { attrs.push(1) }
        if self.dim { attrs.push(2) }
        if self.italic { attrs.push(3) }
        if self.underline { attrs.push(4) }
        if self.blink_slow { attrs.push(5) }
        if self.reverse_colours { attrs.push(7) }
        if self.hidden { attrs.push(8) }
        if self.strikethrough { attrs.push(9) }

        // Could this be done better? - Again a heap allocation of a string - this time im
        // not sure if this is a problem or could be solved better
        let mut result = String::from(beginning);
        if let Some(fg) = fg {
            result.push_str(&fg);
        }
        if let Some(bg) = bg {
            result.push_str(&bg);
        }
        if !attrs.is_empty() {
            result.push_str(&format!(";{}", attrs.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(";")));
        }
        result.push_str("m");

        result
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
