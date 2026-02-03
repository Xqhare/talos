use crate::{codex::Codex, render::Glyph};


/// Represents the contents of a title
///
/// Has 6 positions, any or all of which may be `None`.
///
/// - Top
///     - Left
///     - Center
///     - Right
/// - Bottom
///     - Left
///     - Center
///     - Right
pub struct TitleContents {
    top_left: Option<String>,
    top_left_buffer: Option<Vec<Glyph>>,
    top_center: Option<String>,
    top_center_buffer: Option<Vec<Glyph>>,
    top_right: Option<String>,
    top_right_buffer: Option<Vec<Glyph>>,

    bottom_left: Option<String>,
    bottom_left_buffer: Option<Vec<Glyph>>,
    bottom_center: Option<String>,
    bottom_center_buffer: Option<Vec<Glyph>>,
    bottom_right: Option<String>,
    bottom_right_buffer: Option<Vec<Glyph>>,
}

pub enum TitlePosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Default for TitleContents {
    fn default() -> Self {
        Self {
            top_left: None,
            top_left_buffer: None,
            top_center: None,
            top_center_buffer: None,
            top_right: None,
            top_right_buffer: None,
            bottom_left: None,
            bottom_left_buffer: None,
            bottom_center: None,
            bottom_center_buffer: None,
            bottom_right: None,
            bottom_right_buffer: None,
        }
    }
}

impl TitleContents {
    pub fn set_position(&mut self, position: TitlePosition, string: impl Into<String>, codex: &Codex) {
        match position {
            TitlePosition::TopLeft => self.set_top_left(string, codex),
            TitlePosition::TopCenter => self.set_top_center(string, codex),
            TitlePosition::TopRight => self.set_top_right(string, codex),
            TitlePosition::BottomLeft => self.set_bottom_left(string, codex),
            TitlePosition::BottomCenter => self.set_bottom_center(string, codex),
            TitlePosition::BottomRight => self.set_bottom_right(string, codex),
        }
    }

    pub fn get_position(&self, position: TitlePosition) -> Option<&[Glyph]> {
        match position {
            TitlePosition::TopLeft => self.top_left_buffer.as_deref(),
            TitlePosition::TopCenter => self.top_center_buffer.as_deref(),
            TitlePosition::TopRight => self.top_right_buffer.as_deref(),

            TitlePosition::BottomLeft => self.bottom_left_buffer.as_deref(),
            TitlePosition::BottomCenter => self.bottom_center_buffer.as_deref(),
            TitlePosition::BottomRight => self.bottom_right_buffer.as_deref(),
        }
    }

    fn set_top_left(&mut self, string: impl Into<String>, codex: &Codex) {
        self.top_left = Some(string.into());
        self.top_left_buffer = Some(decode_string(&self.top_left.as_ref().unwrap(), codex));
    }

    fn set_top_center(&mut self, string: impl Into<String>, codex: &Codex) {
        self.top_center = Some(string.into());
        self.top_center_buffer = Some(decode_string(&self.top_center.as_ref().unwrap(), codex));
    }

    fn set_top_right(&mut self, string: impl Into<String>, codex: &Codex) {
        self.top_right = Some(string.into());
        self.top_right_buffer = Some(decode_string(&self.top_right.as_ref().unwrap(), codex));
    }

    fn set_bottom_left(&mut self, string: impl Into<String>, codex: &Codex) {
        self.bottom_left = Some(string.into());
        self.bottom_left_buffer = Some(decode_string(&self.bottom_left.as_ref().unwrap(), codex));
    }

    fn set_bottom_center(&mut self, string: impl Into<String>, codex: &Codex) {
        self.bottom_center = Some(string.into());
        self.bottom_center_buffer = Some(decode_string(&self.bottom_center.as_ref().unwrap(), codex));
    }

    fn set_bottom_right(&mut self, string: impl Into<String>, codex: &Codex) {
        self.bottom_right = Some(string.into());
        self.bottom_right_buffer = Some(decode_string(&self.bottom_right.as_ref().unwrap(), codex));
    }
}

fn decode_string(string: &str, codex: &Codex) -> Vec<Glyph> {
    string.chars().map(|c| codex.lookup(c)).collect()
}
