use crate::layout::Rect;
use crate::render::traits::Widget;
use crate::render::{CCell, Canvas, Codex, Style};

pub struct Block {
    title: Option<String>,
    style: Style,
    fill_bg: bool,
}

impl Block {
    pub fn new() -> Self {
        Self {
            title: None,
            style: Style::default(),
            fill_bg: false,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_bg_fill(mut self) -> Self {
        self.fill_bg = true;
        self
    }
}

impl Widget for Block {
    fn render(&self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        if area.width < 2 || area.height < 2 { return; }

        let left = area.left();
        let right = area.right() - 1;
        let top = area.top();
        let bottom = area.bottom() - 1;

        let h_bar = codex.lookup('─');
        let v_bar = codex.lookup('│');
        let tl = codex.lookup('┌');
        let tr = codex.lookup('┐');
        let bl = codex.lookup('└');
        let br = codex.lookup('┘');

        // Draw Corners
        canvas.set_ccell(left, top, CCell { char: tl, style: self.style });
        canvas.set_ccell(right, top, CCell { char: tr, style: self.style });
        canvas.set_ccell(left, bottom, CCell { char: bl, style: self.style });
        canvas.set_ccell(right, bottom, CCell { char: br, style: self.style });

        // Draw Edges (Using the cached glyphs)
        for x in (left + 1)..right {
            canvas.set_ccell(x, top, CCell { char: h_bar, style: self.style });
            canvas.set_ccell(x, bottom, CCell { char: h_bar, style: self.style });
        }

        for y in (top + 1)..bottom {
            canvas.set_ccell(left, y, CCell { char: v_bar, style: self.style });
            canvas.set_ccell(right, y, CCell { char: v_bar, style: self.style });
        }

        // Fill Background (Optional)
        if self.fill_bg {
            for y in (top + 1)..bottom {
                for x in (left + 1)..right {
                    canvas.set_ccell(x, y, CCell { char: 0x0020, style: self.style });
                }
            }
        }

        // Draw Title
        if let Some(title) = &self.title {
            let max_len = (area.width as usize).saturating_sub(2); // Minus borders
            if max_len > 0 {
                let render_title = if title.len() > max_len {
                    &title[..max_len]
                } else {
                    &title
                };

                let start_x = left + 1;
                for (i, ch) in render_title.chars().enumerate() {
                    let glyph = codex.lookup(ch); 
                    canvas.set_ccell(start_x + i as u16, top, CCell { char: glyph, style: self.style });
                }
            }
        }
    }
}
