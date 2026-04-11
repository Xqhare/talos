use super::{Glyph, Style};

/// A cell on the canvas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CCell {
    /// The glyph of the cell
    pub char: Glyph,
    /// The style of the cell
    pub style: Style,
}

impl Default for CCell {
    #[inline]
    fn default() -> Self {
        Self {
            char: 0x0020, // Space glyph
            style: Style::default(),
        }
    }
}

impl CCell {
    /// Create a new cell
    #[inline]
    #[must_use]
    pub fn new(char: Glyph, style: Style) -> Self {
        Self { char, style }
    }
}
