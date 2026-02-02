use crate::codex::Codex;
use crate::content::text::{Sequence, TextContent};
use crate::layout::Rect;
use crate::render::{CCell, Canvas, Style};
use crate::widgets::traits::Widget;

pub struct Text {
    content: TextContent,
    style: Style,
    align_center: bool,
    align_vertically: bool
}

impl Text {
    pub fn new(content: impl Into<String>, codex: &Codex) -> Self {
        let content = TextContent::new(content, codex, None);
        Self {
            content: content,
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
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {

        // Update wrap limit
        if let Some(wrap_limit) = self.content.get_wrap_limit() {
            if wrap_limit > area.width {
                self.content.set_wrap_limit(area.width, &codex);
            }
        } else {
            // Assume first run - so set up wrap limit
            self.content.set_wrap_limit(area.width, &codex);
        }

        // After setup, each Sequence should be guaranteed to be at most
        // `area.width` wide
        let sequences: &[Sequence] = self.content.get_sequences();
        //let mut out = Vec::with_capacity(sequences.len());
        
        let top = if self.align_vertically {
            if (sequences.len() as u16) < area.height {
                let rest = area.height - sequences.len() as u16;
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

        for (i, seq) in sequences.iter().enumerate() {
            if seq.width() > area.width {
                // TODO: Gracefully handle this - for debugging I want panic
                unreachable!("Rendering Text Widget: Single Sequence must be at most `area.width` wide");
            }

            let left_margin = if self.align_center {
                let rest_width = area.width - seq.width();
                if rest_width % 2 != 0 {
                    rest_width / 2 + 1
                } else {
                    rest_width / 2
                }
            } else {
                0
            };

            let y = top + i as u16;
            let mut x = area.left() + left_margin;

            for glyph in seq.glyphs() {
                
                canvas.set_ccell(x, y, CCell {
                    char: *glyph,
                    style: self.style,
                });
                
                x += 1;
            }

        }

    }
}
