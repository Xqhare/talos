use crate::layout::Rect;
use crate::render::{CCell, Canvas, Codex, Style, traits::Widget};

pub struct Text {
    content: String,
    style: Style,
    align_center: bool,
    align_vertically: bool
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
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
    fn render(&self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
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
        
        for (i, line) in lines.iter().enumerate() {
            if i as u16 >= area.height {
                break;
            }

            let y = top + i as u16;
            let mut x = area.left();

            // Simple Center Alignment
            if self.align_center {
                let text_width = line.chars().count() as u16;
                if text_width < area.width {
                    x += (area.width - text_width) / 2;
                }
            }

            // Draw characters
            let mut drawn_width = 0;
            for ch in line.chars() {
                if drawn_width >= area.width {
                    break;
                }
                
                let glyph = codex.lookup(ch);
                canvas.set_ccell(x + drawn_width, y, CCell {
                    char: glyph,
                    style: self.style,
                });
                
                drawn_width += 1;
            }
        }
    }
}
