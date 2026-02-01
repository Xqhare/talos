
use crate::codex::pages::SPACE_GLYPH;

use super::{Glyph, Style};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CCell {
    pub char: Glyph,
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
