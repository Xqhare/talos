use crate::{codex::{Codex, pages::SPACE_GLYPH}, layout::Rect, render::{CCell, Glyph, Style}, widgets::{Text, traits::Widget}};

pub struct List {
    items: Vec<Text>,
    selected: Option<usize>,
    selected_style: Style,
    selected_symbol: Option<Glyph>
}

impl List {
    pub fn new(items: Vec<Text>) -> Self {
        Self {
            items,
            selected: None,
            selected_style: Style::default(),
            selected_symbol: None
        }
    }

    pub fn with_selected(mut self, selected: usize) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn with_selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    pub fn with_selected_symbol(mut self, char: char, codex: &Codex) -> Self {
        self.selected_symbol = Some(codex.lookup(char));
        self
    }
}

impl Widget for List {
    fn render(&mut self, canvas: &mut crate::render::Canvas, area: crate::layout::Rect, codex: &crate::codex::Codex) {

        let x = if let Some(_) = self.selected_symbol {
            area.x.saturating_add(3)
        } else {
            area.x
        };

        if let Some(selected) = self.selected {
            self.items[selected] = self.items[selected].clone().style(self.selected_style);
            if let Some(symbol) = self.selected_symbol {
                canvas.set_ccell(x.saturating_sub(2), area.y, CCell { char: symbol, style: self.selected_style });
                canvas.set_ccell(x.saturating_sub(1), area.y, CCell { char: SPACE_GLYPH, style: self.selected_style });
            }
            self.items[selected].render(canvas, Rect::new(x, area.y, area.width, area.height), codex);

            let mut rendered_rows: u16 = 1;
            for i in (selected + 1)..self.items.len() {
                if rendered_rows.saturating_add(area.y) >= area.bottom() {
                    break;
                }
                self.items[i].render(canvas, Rect::new(x, area.y.saturating_add(rendered_rows), area.width, area.height), codex);
                rendered_rows += 1;
            }
        } else {
            for i in 0..self.items.len() {
                if i.saturating_add(area.y as usize) >= area.bottom() as usize {
                    break;
                }
                self.items[i].render(canvas, Rect::new(x, area.y.saturating_add(i as u16), area.width, area.height), codex);
            }
        }
    }
}
