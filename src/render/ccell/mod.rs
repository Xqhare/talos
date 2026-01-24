
use super::Style;


#[derive(Clone, Copy, Debug)]
pub struct CCell {
    pub char: Glyph,
    pub style: Style,
}

impl Default for CCell {
    fn default() -> Self {
        Self { char: ' ', style: Style::default() }
    }
}
