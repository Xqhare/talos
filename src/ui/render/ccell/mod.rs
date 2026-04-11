use super::{Glyph, Style};

/// A cell on the canvas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CCell {
    /// The glyph of the cell
    pub char: Glyph,
    /// The style of the cell
    pub style: Style,
}

impl CCell {
    /// Create a new cell
    #[inline]
    #[must_use]
    pub fn new(char: Glyph, style: Style) -> Self {
        Self { char, style }
    }
}
