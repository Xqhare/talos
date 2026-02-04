use crate::{codex::{Codex, pages::SPACE_GLYPH}, layout::Rect, render::{CCell, Glyph, Style}, widgets::traits::Widget};

pub struct List<'a> {
    items: Vec<&'a mut dyn Widget>,
    state: Option<&'a mut ListState>,
    selected_style: Style,
    selected_symbol: Option<Glyph>
}

pub struct ListState {
    pub selected: Option<usize>,
    pub scroll_offset: usize,
}

impl<'a> List<'a> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            state: None,
            selected_style: Style::default(),
            selected_symbol: None
        }
    }

    pub fn add_item(mut self, item: &'a mut dyn Widget) -> Self {
        self.items.push(item);
        self
    }

    pub fn with_items<I, W>(mut self, items: I) -> Self where I: IntoIterator<Item = &'a mut W>, W: Widget + 'a {
        self.items = items.into_iter().map(|i| i as &'a mut dyn Widget).collect();
        self
    }

    pub fn with_state(mut self, state: &'a mut ListState) -> Self {
        self.state = Some(state);
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

impl Widget for List<'_> {
    fn style(&mut self, style: Style) {
        self.selected_style = style;
    }
    fn render(&mut self, canvas: &mut crate::render::Canvas, area: crate::layout::Rect, codex: &crate::codex::Codex) {

        if self.items.is_empty() { return; }

        let x_offset = if self.selected_symbol.is_some() { 3 } else { 0 };
        
        let selected_idx = self.state.as_ref().and_then(|s| s.selected);

        for (i, item) in self.items.iter_mut().enumerate() {
            let y = area.y.saturating_add(i as u16);
            if y >= area.bottom() { break; }

            let is_selected = Some(i) == selected_idx;

            if is_selected {
                // Apply the style to the generic widget using your new trait method
                // We use re-assignment here as trait methods often return Self
                // Since 'item' is a &mut dyn Widget, we apply styling directly
                item.style(self.selected_style);

                if let Some(symbol) = self.selected_symbol {
                    canvas.set_ccell(area.x, y, CCell { char: symbol, style: self.selected_style });
                    canvas.set_ccell(area.x + 1, y, CCell { char: SPACE_GLYPH, style: self.selected_style });
                }
            }

            let item_area = Rect::new(
                area.x + x_offset, 
                y, 
                area.width.saturating_sub(x_offset), 
                1
            );
            
            item.render(canvas, item_area, codex);
        }
    }
}
