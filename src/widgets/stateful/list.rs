use crate::{
    codex::{Codex, pages::SPACE_GLYPH},
    layout::Rect,
    render::{CCell, Glyph, Style},
    widgets::traits::Widget,
};

// 1. The shown selected item, if going backwards, is always the second from the start, as
//    rendered. This is an artifact of the current implementation moving the offset around and can
//    probably not be fixed.
pub struct List<'a> {
    items: Vec<&'a mut dyn Widget>,
    state: Option<&'a mut ListState>,
    selected_style: Style,
    selected_symbol: Option<Glyph>,
    horizontal: bool,
}

pub struct ListState {
    pub selected: Option<usize>,
    pub scroll_offset: usize,
}

impl Default for List<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> List<'a> {
    #[must_use] 
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            state: None,
            selected_style: Style::default(),
            selected_symbol: None,
            horizontal: false,
        }
    }

    #[must_use] 
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    pub fn add_item(mut self, item: &'a mut dyn Widget) -> Self {
        self.items.push(item);
        self
    }

    pub fn with_items<I, W>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = &'a mut W>,
        W: Widget + 'a,
    {
        self.items = items.into_iter().map(|i| i as &'a mut dyn Widget).collect();
        self
    }

    pub fn with_state(mut self, state: &'a mut ListState) -> Self {
        self.state = Some(state);
        self
    }

    #[must_use] 
    pub fn with_selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    #[must_use] 
    pub fn with_selected_symbol(mut self, char: char, codex: &Codex) -> Self {
        self.selected_symbol = Some(codex.lookup(char));
        self
    }
}

impl Widget for List<'_> {
    fn style(&mut self, style: Style) {
        self.selected_style = style;
    }
    fn render(
        &mut self,
        canvas: &mut crate::render::Canvas,
        area: crate::layout::Rect,
        codex: &crate::codex::Codex,
    ) {
        if self.items.is_empty() {
            return;
        }

        let x_offset = if self.selected_symbol.is_some() { 3 } else { 0 };

        let selected_idx = self.state.as_ref().and_then(|s| s.selected);

        if self.horizontal {
            let offset = self.state.as_ref().map_or(0, |s| s.scroll_offset);
            for (i, item) in self.items.iter_mut().enumerate().skip(offset) {
                let relative_idx = i - offset;

                let current_x = if relative_idx == 0 {
                    area.x
                } else {
                    canvas.last_cell().map_or(area.x, |(lx, _)| lx + 1)
                };

                if current_x >= area.right() {
                    break;
                }

                let is_selected = Some(i) == selected_idx;

                if is_selected {
                    item.style(self.selected_style);

                    if let Some(symbol) = self.selected_symbol {
                        canvas.set_ccell(
                            current_x + 1,
                            area.y,
                            CCell {
                                char: symbol,
                                style: self.selected_style,
                            },
                        );
                        canvas.set_ccell(
                            current_x + 2,
                            area.y,
                            CCell {
                                char: SPACE_GLYPH,
                                style: self.selected_style,
                            },
                        );
                    }
                }

                let x_symbol_padding = if is_selected && self.selected_symbol.is_some() {
                    3
                } else {
                    0
                };

                if current_x + x_symbol_padding >= area.right() - 2 {
                    break;
                }

                let item_area = Rect::new(
                    current_x + x_symbol_padding,
                    area.y,
                    area.right().saturating_sub(current_x + x_symbol_padding),
                    area.height,
                );

                item.render(canvas, item_area, codex);

                // Scrolling the list if needed
                if is_selected {
                    let pos = canvas.last_cell().map_or_else(|| current_x, |(lx, _)| lx);
                    if pos >= area.right() - 5 {
                        self.state.as_mut().map(|s| s.scroll_offset += 3);
                    }
                    if i == self.state.as_ref().map_or(0, |s| s.scroll_offset)
                        && self.state.as_ref().map(|s| s.scroll_offset) != Some(0)
                    {
                        self.state.as_mut().map(|s| s.scroll_offset -= 1);
                    }
                }
                // Add a space between horizontal items
                let space_x = canvas.last_cell().map_or(current_x, |(lx, _)| lx + 1);
                if space_x < area.right() {
                    canvas.set_ccell(
                        space_x,
                        area.y,
                        CCell {
                            char: SPACE_GLYPH,
                            style: Style::default(),
                        },
                    );
                }
            }
        } else {
            // Ensure the selected item is visible before we start rendering.
            if let (Some(state), Some(selected)) = (self.state.as_mut(), selected_idx) {
                let height = area.height as usize;

                if selected < state.scroll_offset {
                    state.scroll_offset = selected;
                } else if selected >= state.scroll_offset + height {
                    state.scroll_offset = selected - height + 1;
                }
            }

            let offset = self.state.as_ref().map_or(0, |s| s.scroll_offset);

            for (i, item) in self.items.iter_mut().enumerate().skip(offset) {
                let line_index = i - offset;
                let y = area.y.saturating_add(line_index as u16);

                if y >= area.bottom() {
                    break;
                }

                let is_selected = Some(i) == selected_idx;

                if is_selected {
                    item.style(self.selected_style);

                    if let Some(symbol) = self.selected_symbol {
                        canvas.set_ccell(
                            area.x + 1,
                            y,
                            CCell {
                                char: symbol,
                                style: self.selected_style,
                            },
                        );
                        canvas.set_ccell(
                            area.x + 2,
                            y,
                            CCell {
                                char: SPACE_GLYPH,
                                style: self.selected_style,
                            },
                        );
                    }
                }

                let item_area =
                    Rect::new(area.x + x_offset, y, area.width.saturating_sub(x_offset), 1);

                item.render(canvas, item_area, codex);
            }
        }
    }
}
