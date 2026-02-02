use crate::{codex::Codex, render::Glyph};

/// The content is parsed into glyph sequences no wider than `max_width`
///
/// A glyph sequence is a vector of glyphs bounded by included, trailing, whitespace
pub struct TextContent {
    raw: String,
    buffer: Vec<Sequence>,
    // If not set, Sequences will be bounded by included, trailing, newlines.
    max_width: Option<u16>,
}

impl TextContent {
    pub fn new(content: impl Into<String>, codex: &Codex, max_width: Option<u16>) -> Self {
        let raw = content.into();
        let buffer = Self::parse_content_to_glyphs(&raw, codex, max_width);
        Self {
            raw,
            buffer,
            max_width,
        }
    }

    pub fn set_wrap_limit(&mut self, max_width: u16, codex: &Codex) {
        self.max_width = Some(max_width);
        self.buffer = Self::parse_content_to_glyphs(&self.raw, codex, Some(max_width));
    }

    pub fn get_wrap_limit(&self) -> Option<u16> {
        self.max_width
    }

    pub fn get_sequences(&self) -> &[Sequence] {
        &self.buffer
    }

    // TODO: Cleanup of nested ifs
    fn parse_content_to_glyphs(content: &str, codex: &Codex, max_width: Option<u16>) -> Vec<Sequence> {
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
                    current_line.extend(word.chars().map(|ch| codex.lookup(ch)));
                    out.push(Sequence::new(std::mem::take(&mut current_line), current_width));
                    current_width = 0;
                    continue;
                }
                if word == "\n" {
                    out.push(Sequence::new(std::mem::take(&mut current_line), current_width));
                    current_width = 0;
                    continue;
                }

                let word_glyphs: Vec<Glyph> = word.chars().map(|ch| codex.lookup(ch)).collect();
                let word_len = word_glyphs.len() as u16;

                // If word fits in current line
                if current_width + word_len < max_width {
                    current_line.extend(word_glyphs);
                    current_width += word_len;
                } else if current_width + word_len == max_width {
                    current_line.extend(word_glyphs);
                    out.push(Sequence::new(std::mem::take(&mut current_line), current_width));
                    current_width = 0;
                } else {
                    // If line isn't empty, push it and start new line
                    if !current_line.is_empty() {
                        out.push(Sequence::new(std::mem::take(&mut current_line), current_width));
                    }

                    // Handle words longer than max_width by slicing
                    let mut remaining_glyphs = word_glyphs;
                    while remaining_glyphs.len() as u16 > max_width {
                        let tail = remaining_glyphs.split_off(max_width as usize);
                        out.push(Sequence::new(remaining_glyphs, max_width));
                        remaining_glyphs = tail;
                    }
                    
                    // Put the remaining part of the word on the current line
                    current_width = remaining_glyphs.len() as u16;
                    current_line = remaining_glyphs;
                }
            }

            if !current_line.is_empty() {
                out.push(Sequence::new(current_line, current_width));
            }
            
        } else {
            let split_input: Vec<&str> = content.split_inclusive('\n').collect();
            // if no max width is set, just assume that it'll fit
            for line in split_input {
                let mut buffer = Vec::with_capacity(line.len());
                for ch in line.chars() {
                    buffer.push(codex.lookup(ch));
                }
                out.push(Sequence::new(buffer, line.len() as u16));
            }
            
        }

        out
    }
}

pub struct Sequence {
    buffer: Vec<Glyph>,
    width: u16,
}

impl Sequence {
    pub fn new(buffer: Vec<Glyph>, width: u16) -> Self {
        Self { buffer, width }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn glyphs(&self) -> &[Glyph] {
        &self.buffer
    }
}
