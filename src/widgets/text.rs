use crate::codex::Codex;
use crate::layout::Rect;
use crate::render::{CCell, Canvas, Glyph, Style};
use crate::widgets::traits::Widget;

// TODO: No wrapping support whatsoever
// To optimise, we could split not into lines, but non breaking sequences (e.g. slices separated by
// whitespace) - Draw the first sequence, if there is enough space, draw the next sequence, if not
// wrap.
pub struct Text {
    content: String,
    content_glyphs: Vec<Vec<Glyph>>,
    style: Style,
    align_center: bool,
    align_vertically: bool
}

fn parse_content_to_glyphs(content: &str, codex: &Codex) -> Vec<Vec<Glyph>> {
    let mut out = Vec::with_capacity(content.len());
    for line in content.lines() {
        let mut out_line = Vec::with_capacity(line.len());
        for ch in line.chars() {
            out_line.push(codex.lookup(ch));
        }
        out.push(out_line);
    }
    out
}

impl Text {
    pub fn new(content: impl Into<String>, codex: &Codex) -> Self {
        let content = content.into();
        let content_glyphs = parse_content_to_glyphs(&content, codex);
        Self {
            content: content,
            // Just an arbitrary capacity to save on allocs
            content_glyphs,
            style: Style::default(),
            align_center: false,
            align_vertically: false
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn align_center(mut self) -> Self {
        self.align_center = true;
        self
    }

    pub fn align_vertically(mut self) -> Self {
        self.align_vertically = true;
        self
    }
}

impl Widget for Text {
    fn render(&self, canvas: &mut Canvas, area: Rect, _codex: &Codex) {
        let lines: Vec<&str> = self.content.lines().collect();

        let top = if self.align_vertically {
            if (lines.len() as u16) < area.height {
                let rest = area.height - lines.len() as u16;
                if rest % 2 != 0 {
                    (rest / 2 + 1) + area.top()
                } else {
                    (rest / 2) + area.top()
                }
            } else {
                area.top()
            }
        } else {
            area.top()
        };
        
        for (i, line) in self.content_glyphs.iter().enumerate() {
            if i as u16 >= area.height {
                break;
            }

            let y = top + i as u16;
            let mut x = area.left();

            // Simple Center Alignment
            if self.align_center {
                let text_width = line.len() as u16;
                if text_width < area.width {
                    x += (area.width - text_width) / 2;
                }
            }

            // Draw characters
            let mut drawn_width = 0;
            for glyph in line {
                if drawn_width >= area.width {
                    break;
                }
                
                canvas.set_ccell(x + drawn_width, y, CCell {
                    char: *glyph,
                    style: self.style,
                });
                
                drawn_width += 1;
            }
        }
    }
}
