use crate::{codex::Codex, render::Glyph};

/// A collection of titles at different positions.
///
/// Titles can be placed at any of the six positions defined in [`TitlePosition`].
///
/// # Example
///
/// ```rust,no_run
/// use talos::codex::Codex;
/// use talos::title::{TitleContents, TitlePosition};
///
/// let codex = Codex::new();
/// let mut titles = TitleContents::default();
/// titles.set_position(&TitlePosition::TopLeft, "My Title", &codex);
/// ```
#[derive(Default)]
pub struct TitleContents {
    /// Top left title string.
    top_left: Option<String>,
    /// Top left title glyph buffer.
    top_left_buffer: Option<Vec<Glyph>>,
    /// Top center title string.
    top_center: Option<String>,
    /// Top center title glyph buffer.
    top_center_buffer: Option<Vec<Glyph>>,
    /// Top right title string.
    top_right: Option<String>,
    /// Top right title glyph buffer.
    top_right_buffer: Option<Vec<Glyph>>,
    /// Bottom left title string.
    bottom_left: Option<String>,
    /// Bottom left title glyph buffer.
    bottom_left_buffer: Option<Vec<Glyph>>,
    /// Bottom center title string.
    bottom_center: Option<String>,
    /// Bottom center title glyph buffer.
    bottom_center_buffer: Option<Vec<Glyph>>,
    /// Bottom right title string.
    bottom_right: Option<String>,
    /// Bottom right title glyph buffer.
    bottom_right_buffer: Option<Vec<Glyph>>,
}

/// The possible positions for a title.
pub enum TitlePosition {
    /// Top left corner.
    TopLeft,
    /// Top center.
    TopCenter,
    /// Top right corner.
    TopRight,
    /// Bottom left corner.
    BottomLeft,
    /// Bottom center.
    BottomCenter,
    /// Bottom right corner.
    BottomRight,
}

impl TitleContents {
    /// Sets the title at the specified position.
    pub fn set_position(
        &mut self,
        position: &TitlePosition,
        string: impl Into<String>,
        codex: &Codex,
    ) {
        match position {
            TitlePosition::TopLeft => self.set_top_left(string, codex),
            TitlePosition::TopCenter => self.set_top_center(string, codex),
            TitlePosition::TopRight => self.set_top_right(string, codex),
            TitlePosition::BottomLeft => self.set_bottom_left(string, codex),
            TitlePosition::BottomCenter => self.set_bottom_center(string, codex),
            TitlePosition::BottomRight => self.set_bottom_right(string, codex),
        }
    }

    /// Gets the glyph buffer for the title at the specified position.
    #[must_use]
    pub fn get_position(&self, position: &TitlePosition) -> Option<&[Glyph]> {
        match position {
            TitlePosition::TopLeft => self.top_left_buffer.as_deref(),
            TitlePosition::TopCenter => self.top_center_buffer.as_deref(),
            TitlePosition::TopRight => self.top_right_buffer.as_deref(),
            TitlePosition::BottomLeft => self.bottom_left_buffer.as_deref(),
            TitlePosition::BottomCenter => self.bottom_center_buffer.as_deref(),
            TitlePosition::BottomRight => self.bottom_right_buffer.as_deref(),
        }
    }

    /// Internal method to set the top left title.
    fn set_top_left(&mut self, string: impl Into<String>, codex: &Codex) {
        let s = string.into();
        self.top_left_buffer = Some(decode_string(&s, codex));
        self.top_left = Some(s);
    }
    /// Internal method to set the top center title.
    fn set_top_center(&mut self, string: impl Into<String>, codex: &Codex) {
        let s = string.into();
        self.top_center_buffer = Some(decode_string(&s, codex));
        self.top_center = Some(s);
    }
    /// Internal method to set the top right title.
    fn set_top_right(&mut self, string: impl Into<String>, codex: &Codex) {
        let s = string.into();
        self.top_right_buffer = Some(decode_string(&s, codex));
        self.top_right = Some(s);
    }
    /// Internal method to set the bottom left title.
    fn set_bottom_left(&mut self, string: impl Into<String>, codex: &Codex) {
        let s = string.into();
        self.bottom_left_buffer = Some(decode_string(&s, codex));
        self.bottom_left = Some(s);
    }
    /// Internal method to set the bottom center title.
    fn set_bottom_center(&mut self, string: impl Into<String>, codex: &Codex) {
        let s = string.into();
        self.bottom_center_buffer = Some(decode_string(&s, codex));
        self.bottom_center = Some(s);
    }
    /// Internal method to set the bottom right title.
    fn set_bottom_right(&mut self, string: impl Into<String>, codex: &Codex) {
        let s = string.into();
        self.bottom_right_buffer = Some(decode_string(&s, codex));
        self.bottom_right = Some(s);
    }
}

/// Decodes a string into a glyph buffer using the provided codex.
fn decode_string(string: &str, codex: &Codex) -> Vec<Glyph> {
    string.chars().map(|ch| codex.lookup(ch)).collect()
}
