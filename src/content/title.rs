use crate::render::Grapheme;

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
#[derive(Default)]
pub struct TitleContents {
    top_left: Option<String>,
    top_left_buffer: Option<Vec<Grapheme>>,
    top_center: Option<String>,
    top_center_buffer: Option<Vec<Grapheme>>,
    top_right: Option<String>,
    top_right_buffer: Option<Vec<Grapheme>>,

    bottom_left: Option<String>,
    bottom_left_buffer: Option<Vec<Grapheme>>,
    bottom_center: Option<String>,
    bottom_center_buffer: Option<Vec<Grapheme>>,
    bottom_right: Option<String>,
    bottom_right_buffer: Option<Vec<Grapheme>>,
}

pub enum TitlePosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl TitleContents {
    pub fn set_position(
        &mut self,
        position: &TitlePosition,
        string: impl Into<String>,
        thoth: &thoth::Thoth,
    ) {
        match position {
            TitlePosition::TopLeft => self.set_top_left(string, thoth),
            TitlePosition::TopCenter => self.set_top_center(string, thoth),
            TitlePosition::TopRight => self.set_top_right(string, thoth),
            TitlePosition::BottomLeft => self.set_bottom_left(string, thoth),
            TitlePosition::BottomCenter => self.set_bottom_center(string, thoth),
            TitlePosition::BottomRight => self.set_bottom_right(string, thoth),
        }
    }

    pub fn get_position(&self, position: &TitlePosition) -> Option<&[Grapheme]> {
        match position {
            TitlePosition::TopLeft => self.top_left_buffer.as_deref(),
            TitlePosition::TopCenter => self.top_center_buffer.as_deref(),
            TitlePosition::TopRight => self.top_right_buffer.as_deref(),

            TitlePosition::BottomLeft => self.bottom_left_buffer.as_deref(),
            TitlePosition::BottomCenter => self.bottom_center_buffer.as_deref(),
            TitlePosition::BottomRight => self.bottom_right_buffer.as_deref(),
        }
    }

    fn set_top_left(&mut self, string: impl Into<String>, thoth: &thoth::Thoth) {
        self.top_left = Some(string.into());
        self.top_left_buffer = Some(decode_string(self.top_left.as_ref().unwrap(), thoth));
    }

    fn set_top_center(&mut self, string: impl Into<String>, thoth: &thoth::Thoth) {
        self.top_center = Some(string.into());
        self.top_center_buffer = Some(decode_string(self.top_center.as_ref().unwrap(), thoth));
    }

    fn set_top_right(&mut self, string: impl Into<String>, thoth: &thoth::Thoth) {
        self.top_right = Some(string.into());
        self.top_right_buffer = Some(decode_string(self.top_right.as_ref().unwrap(), thoth));
    }

    fn set_bottom_left(&mut self, string: impl Into<String>, thoth: &thoth::Thoth) {
        self.bottom_left = Some(string.into());
        self.bottom_left_buffer = Some(decode_string(self.bottom_left.as_ref().unwrap(), thoth));
    }

    fn set_bottom_center(&mut self, string: impl Into<String>, thoth: &thoth::Thoth) {
        self.bottom_center = Some(string.into());
        self.bottom_center_buffer =
            Some(decode_string(self.bottom_center.as_ref().unwrap(), thoth));
    }

    fn set_bottom_right(&mut self, string: impl Into<String>, thoth: &thoth::Thoth) {
        self.bottom_right = Some(string.into());
        self.bottom_right_buffer = Some(decode_string(self.bottom_right.as_ref().unwrap(), thoth));
    }
}

fn decode_string(string: &str, thoth: &thoth::Thoth) -> Vec<Grapheme> {
    let graphemes = thoth.segment(string).unwrap_or_else(|_| {
        string.chars().map(|ch| ch.to_string()).collect()
    });
    graphemes.iter().map(|g| Grapheme::new(g)).collect()
}
