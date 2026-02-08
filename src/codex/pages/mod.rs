mod cp_437;
use cp_437::CP437;
mod win_1252;
use win_1252::WIN_1252;
mod utf_misc_technical;
use utf_misc_technical::MISC_TECHNICAL;
mod utf_geometric_shapes;
use utf_geometric_shapes::GEOMETRIC_SHAPES;

use crate::{
    error::{TalosError, TalosResult},
    render::Glyph,
};

// A page is a 256 char array
// each char is a unicode character that takes up one cell if rendered.
// each page is a different encoding.
//
// By default, `windows-1252` and `cp437` are provided.
// Providing the `ISO 8859` family, other windows pages, or even MacRoman is easily done.

pub type Page = [&'static str; 256];

pub const UNKNOWN_CHAR: &str = "⚠";

// resolves to first page (id 0), entry 0 - Warning sign (Not part of Win-1252, put there by me)
pub const UNKNOWN_CHAR_GLYPH: Glyph = 0x0000;
pub const SPACE_GLYPH: Glyph = 0x0020;

// Page ID's given here are assumed to be always valid
// THERE ARE HARDCODED GLYPH REFERENCES THAT REQUIRE AND EXPECT THESE ID VALUES! (see above)
pub const REG_WIN_1252: (u8, &Page) = (0, &WIN_1252);
pub const REG_CP437: (u8, &Page) = (1, &CP437);
pub const REG_UTF_MISC_TECHNICAL: (u8, &Page) = (2, &MISC_TECHNICAL);
pub const REG_UTF_GEOMETRIC_SHAPES: (u8, &Page) = (3, &GEOMETRIC_SHAPES);

/// Checks if a page is valid
///
/// # Arguments
/// * `page` - The page to check
///
/// # Errors
/// Returns an error if the page is invalid (does not have 256 entries or if any entry is not a single unicode character)
pub fn validate_page(page: &Page) -> TalosResult<()> {
    if page.len() != 256 {
        return Err(TalosError::InvalidArgument(format!(
            "Page must have 256 entries, got {}",
            page.len()
        )));
    }

    for (id, &glyph) in page.iter().enumerate() {
        if glyph.chars().count() != 1 {
            return Err(TalosError::InvalidArgument(format!(
                "Page entry '{id}' must be a single unicode character, got '{glyph}'"
            )));
        }
    }
    Ok(())
}

#[must_use]
pub fn pre_computed_char(g: Glyph) -> Option<&'static str> {
    match g {
        0..=127 => {
            // Windows 1252 is page 0 -> Upper byte = 0
            if REG_WIN_1252.0 == 0 {
                Some(WIN_1252[g as usize])
            } else {
                None
            }
        }
        _ => None,
    }
}
