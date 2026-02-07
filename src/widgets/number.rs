use std::{
    fmt::Display,
    ops::{Add, Mul},
};

use crate::{
    codex::Codex,
    content::text::TextContent,
    layout::Rect,
    render::{CCell, Canvas, Style},
    widgets::traits::Widget,
};

#[derive(Debug, Clone)]
pub struct Number {
    content: TextContent,
    style: Style,
}

impl Number {
    pub fn new<N>(content: &N, codex: &Codex) -> Self
    where
        N: Add<Output = N> + Mul<Output = N> + Display,
    {
        let content = TextContent::new(format!("{content}"), codex, None);
        Self {
            content,
            style: Style::default(),
        }
    }
}

impl Widget for Number {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        self.content.set_wrap_limit(area.width, codex);
        for (i, seq) in self.content.get_sequences().iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let x = area.x + i as u16;
            if x >= area.right() {
                break;
            }
            for (i, glyph) in seq.glyphs().iter().enumerate() {
                #[allow(clippy::cast_possible_truncation)]
                canvas.set_ccell(
                    x + i as u16,
                    area.y,
                    CCell {
                        char: *glyph,
                        style: self.style,
                    },
                );
            }
        }
    }
}
