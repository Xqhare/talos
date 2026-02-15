use crate::codex::pages::SPACE_GLYPH;

use super::{Glyph, Style};

/// A single cell in a [Canvas](struct.Canvas.html)
///
/// Contains a [Glyph](type.Glyph.html) and a [Style](struct.Style.html)
///
/// # Example
/// ```rust
/// use talos::render::{CCell, Style};
///
/// let cell = CCell {
///    char: 'a' as u16,
///    style: Style::default(),
/// };
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CCell {
    /// The glyph
    pub char: Glyph,
    /// The style
    pub style: Style,
}

impl Default for CCell {
    fn default() -> Self {
        Self {
            char: SPACE_GLYPH,
            style: Style::default(),
        }
    }
}
