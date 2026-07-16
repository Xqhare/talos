use crate::render::Grapheme;

/// The content is parsed into glyph sequences no wider than `max_width`
///
/// A glyph sequence is a vector of glyphs bounded by included, trailing, whitespace
#[derive(Debug, Default, Clone)]
pub struct TextContent {
    raw: String,
    buffer: Vec<Sequence>,
    // If not set, Sequences will be bounded by included, trailing, newlines.
    max_width: Option<u16>,
}

impl TextContent {
    pub fn new(content: impl Into<String>, thoth: &thoth::Thoth, max_width: Option<u16>) -> Self {
        let raw = content.into();
        let buffer = Self::parse_content_to_glyphs(&raw, thoth, max_width);
        Self {
            raw,
            buffer,
            max_width,
        }
    }

    pub fn set_wrap_limit(&mut self, max_width: u16, thoth: &thoth::Thoth) {
        if self.max_width == Some(max_width) {
            return;
        }
        self.max_width = Some(max_width);
        self.buffer = Self::parse_content_to_glyphs(&self.raw, thoth, Some(max_width));
    }

    pub fn set_content(&mut self, content: impl Into<String>, thoth: &thoth::Thoth) {
        self.raw = content.into();
        self.buffer = Self::parse_content_to_glyphs(&self.raw, thoth, self.max_width);
    }

    pub fn get_wrap_limit(&self) -> Option<u16> {
        self.max_width
    }

    pub fn get_sequences(&self) -> &[Sequence] {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        self.raw.chars().count()
    }

    pub fn get_rendered_width(&self) -> u16 {
        self.buffer.iter().map(|s| s.width()).max().unwrap_or(0)
    }

    pub fn get_content(&self) -> &str {
        &self.raw
    }

    // TODO: Cleanup of nested ifs
    fn parse_content_to_glyphs(
        content: &str,
        thoth: &thoth::Thoth,
        max_width: Option<u16>,
    ) -> Vec<Sequence> {
        // Overallocates a fair bit
        let mut out = Vec::with_capacity(content.len());

        if let Some(max_width) = max_width {
            let mut current_line = Vec::new();
            let mut current_width = 0;

            if max_width == 0 {
                return Vec::new();
            }

            // Split by words but keep whitespace
            for word in content.split_inclusive(char::is_whitespace) {
                if word.ends_with('\n') && word != "\n" {
                    // Remove the trailing newline, push the word and start a new line
                    let word = &word[..word.len() - 1];
                    let word_graphemes = thoth.segment(word).unwrap_or_else(|_| {
                        word.chars().map(|ch| ch.to_string()).collect()
                    });
                    let word_glyphs: Vec<Grapheme> =
                        word_graphemes.iter().map(|g| Grapheme::new(g)).collect();
                    #[allow(clippy::cast_possible_truncation)]
                    let word_len = word_glyphs.len() as u16;
                    current_line.extend(word_glyphs);
                    out.push(Sequence::new(
                        std::mem::take(&mut current_line),
                        current_width + word_len,
                    ));
                    current_width = 0;
                    continue;
                }
                if word == "\n" {
                    out.push(Sequence::new(
                        std::mem::take(&mut current_line),
                        current_width,
                    ));
                    current_width = 0;
                    continue;
                }

                let word_graphemes = thoth.segment(word).unwrap_or_else(|_| {
                    word.chars().map(|ch| ch.to_string()).collect()
                });
                let word_glyphs: Vec<Grapheme> =
                    word_graphemes.iter().map(|g| Grapheme::new(g)).collect();
                #[allow(clippy::cast_possible_truncation)]
                let word_len = word_glyphs.len() as u16;

                #[allow(clippy::comparison_chain)]
                if current_width + word_len < max_width {
                    current_line.extend(word_glyphs);
                    current_width += word_len;
                } else if current_width + word_len == max_width {
                    current_line.extend(word_glyphs);
                    out.push(Sequence::new(
                        std::mem::take(&mut current_line),
                        current_width + word_len,
                    ));
                    current_width = 0;
                } else {
                    // If line isn't empty, push it and start new line
                    if !current_line.is_empty() {
                        out.push(Sequence::new(
                            std::mem::take(&mut current_line),
                            current_width,
                        ));
                    }

                    // Handle words longer than max_width by slicing
                    let mut remaining_glyphs = word_glyphs;
                    #[allow(clippy::cast_possible_truncation)]
                    while remaining_glyphs.len() as u16 > max_width {
                        let tail = remaining_glyphs.split_off(max_width as usize);
                        out.push(Sequence::new(remaining_glyphs, max_width));
                        remaining_glyphs = tail;
                    }

                    // Put the remaining part of the word on the current line
                    #[allow(clippy::cast_possible_truncation)]
                    let new_width = remaining_glyphs.len() as u16;
                    current_width = new_width;
                    current_line = remaining_glyphs;
                }
            }

            if !current_line.is_empty() {
                out.push(Sequence::new(current_line, current_width));
            }
        } else {
            let split_input: Vec<&str> = content.split_inclusive('\n').collect();
            for line in split_input {
                let line_graphemes = thoth.segment(line).unwrap_or_else(|_| {
                    line.chars().map(|ch| ch.to_string()).collect()
                });
                let mut buffer = Vec::with_capacity(line_graphemes.len());
                for g in &line_graphemes {
                    buffer.push(Grapheme::new(g));
                }
                #[allow(clippy::cast_possible_truncation)]
                out.push(Sequence::new(buffer, line_graphemes.len() as u16));
            }
        }

        out
    }
}

#[derive(Debug, Clone)]
pub struct Sequence {
    buffer: Vec<Grapheme>,
    width: u16,
}

impl Sequence {
    pub fn new(buffer: Vec<Grapheme>, width: u16) -> Self {
        Self { buffer, width }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn glyphs(&self) -> &[Grapheme] {
        &self.buffer
    }
}
