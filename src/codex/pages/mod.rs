mod cp_437;
use cp_437::CP437;
use win_1252::WIN_1252;

use crate::{
    error::{TalosError, TalosResult},
    render::Glyph,
};
mod win_1252;

// A page is a 256 char array
// each char is a unicode character that takes up one cell if rendered.
// each page is a different encoding.
//
// By default, `windows-1252` and `cp437` are provided.
// Providing the `ISO 8859` family, other windows pages, or even MacRoman is easily done.

pub type Page = [&'static str; 256];

pub const UNKNOWN_CHAR: &str = "?";

// resolves to first page (id 0), entry 63 - question mark as long as first page is `win-1252`
pub const UNKNOWN_CHAR_GLYPH: Glyph = 0x003F;
pub const SPACE_GLYPH: Glyph = 0x0020;

// Page ID's given here are assumed to be always valid, there are hardcoded Glyph references that require these ID values (e.g. `UNKNOWN_CHAR_GLYPH`)
pub const REG_WIN_1252: (u8, &'static Page) = (0, &WIN_1252);
pub const REG_CP437: (u8, &'static Page) = (1, &CP437);

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
                "Page entry '{}' must be a single unicode character, got '{}'",
                id, glyph
            )));
        }
    }
    Ok(())
}

pub fn pre_computed_char(g: Glyph) -> Option<&'static str> {
    match g {
        0..=127 => {
            // Windows 1252 is page 0 -> Upper byte = 0
            Some(WIN_1252[g as usize])
        },
        _ => return None
    }
}

/// Checks if the character is part of 0-127 ASCII
pub fn pre_computed_glyph(c: char) -> Option<Glyph> {
    match c as u8 {
        0..=127 => {
            // Windows 1252 is page 0 -> Upper byte = 0
            Some(c as u16)
        },
        _ => return None
    }
}

