use super::{Grapheme, Style};

/// A single cell in a [Canvas](struct.Canvas.html)
///
/// Contains a [Grapheme](struct.Grapheme.html) and a [Style](struct.Style.html)
///
/// # Example
/// ```rust
/// use talos::render::{CCell, Style, Grapheme};
///
/// let cell = CCell {
///    char: Grapheme::new("a"),
///    style: Style::default(),
/// };
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CCell {
    /// The grapheme
    pub char: Grapheme,
    /// The style
    pub style: Style,
}

impl Default for CCell {
    fn default() -> Self {
        Self {
            char: Grapheme::default(),
            style: Style::default(),
        }
    }
}
