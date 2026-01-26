use crate::render::Colour;

use super::Style;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StyleBuilder {
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

impl StyleBuilder {
    pub fn with_fg(mut self, fg: Colour) -> Self {
        self.fg = Some(fg);
        self
    }

    pub fn with_bg(mut self, bg: Colour) -> Self {
        self.bg = Some(bg);
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    pub fn dim(mut self, dim: bool) -> Self {
        self.dim = dim;
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    pub fn blink_slow(mut self, blink_slow: bool) -> Self {
        self.blink_slow = blink_slow;
        self
    }

    pub fn reverse_colours(mut self, reverse_colours: bool) -> Self {
        self.reverse_colours = reverse_colours;
        self
    }

    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }

    pub fn build(self) -> Style {
        Style {
            fg: self.fg,
            bg: self.bg,
            bold: self.bold,
            dim: self.dim,
            italic: self.italic,
            underline: self.underline,
            blink_slow: self.blink_slow,
            reverse_colours: self.reverse_colours,
            hidden: self.hidden,
            strikethrough: self.strikethrough,
        }
    }
}

impl Default for StyleBuilder {
    fn default() -> Self {
        Self {
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
