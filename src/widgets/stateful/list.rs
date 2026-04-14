use crate::{
    codex::{Codex, pages::SPACE_GLYPH},
    layout::Rect,
    render::{CCell, Glyph, Style},
    widgets::traits::{Widget, make_dyn_iter},
};

// 1. The shown selected item, if going backwards, is always the second from the start, as
//    rendered. This is an artifact of the current implementation moving the offset around and can
//    probably not be fixed.

/// A stateful widget that displays a scrollable list of items.
///
/// The `List` widget can be used to display a scrollable list of items. The list can be either
/// horizontal or vertical. The state of the list is managed by a `ListState` struct, which must be
/// passed to the `with_state` method.
///
/// # Example
///
/// ```rust,no_run
/// use talos::{
///     Talos,
///     input::{Event, KeyCode, KeyEvent},
///     layout::Rect,
///     widgets::{
///         stateful::{List, ListState},
///         Text,
///         traits::Widget,
///     },
/// };
///
/// fn main() -> Result<(), talos::TalosError> {
///     let mut talos = Talos::builder().build()?;
///     let (canvas, codex) = talos.render_ctx();
///
///     let mut list_state = ListState::default();
///     list_state.selected = Some(0);
///
///     let mut items = vec![
///         Text::new("Item 1", codex),
///         Text::new("Item 2", codex),
///         Text::new("Item 3", codex),
///     ];
///
///     let mut list = List::new(&mut list_state, items.iter_mut());
///
///     let rect = Rect::new(0, 0, 20, 10);
///     list.render(canvas, rect, codex);
///
///     talos.present()?;
///
///     Ok(())
/// }
/// ```

#[must_use]
pub struct List<'a> {
    items: Vec<&'a mut dyn Widget>,
    state: &'a mut ListState,
    selected_style: Style,
    selected_symbol: Option<Glyph>,
    horizontal: bool,
}

/// The state of a list
#[derive(Default, Debug, Clone, Copy)]
pub struct ListState {
    /// The index of the currently selected item
    pub selected: Option<usize>,
    /// The offset of the list - used for scrolling
    pub scroll_offset: usize,
}

impl AsRef<ListState> for ListState {
    fn as_ref(&self) -> &ListState {
        self
    }
}

impl AsMut<ListState> for ListState {
    fn as_mut(&mut self) -> &mut ListState {
        self
    }
}

impl<'a> List<'a> {
    /// Creates a new, empty list
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::{traits::Widget, stateful::{List, ListState}}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, codex) = talos.render_ctx();
    /// let mut list_state = ListState::default();
    /// let mut items: Vec<&mut dyn Widget> = Vec::new();
    /// let list = List::new(&mut list_state, items.iter_mut());
    /// # assert!(true);
    /// ```
    pub fn new<I, W>(state: &'a mut ListState, items: I) -> Self
    where
        I: Iterator<Item = &'a mut W>,
        W: Widget + 'a,
    {
        Self {
            items: make_dyn_iter(items),
            state,
            selected_style: Style::default(),
            selected_symbol: None,
            horizontal: false,
        }
    }

    /// Gets the state of the list
    pub fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }

    /// Sets the list to be horizontal
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    /// Sets the style of the selected item
    pub fn with_selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    /// Sets the symbol of the selected item - this is rendered in front (to the left) of the
    /// selected item
    pub fn with_selected_symbol(mut self, char: char, codex: &Codex) -> Self {
        self.selected_symbol = Some(codex.lookup(char));
        self
    }
}

impl Widget for List<'_> {
    fn style(&mut self, style: Style) {
        self.selected_style = style;
    }
    #[allow(clippy::too_many_lines)]
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

        let selected_idx = self.state.as_ref().selected;

        if self.horizontal {
            let offset = self.state.as_ref().scroll_offset;
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
                            current_x.saturating_add(1),
                            area.y,
                            CCell {
                                char: symbol,
                                style: self.selected_style,
                            },
                        );
                        canvas.set_ccell(
                            current_x.saturating_add(2),
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

                let x_sum = current_x.saturating_add(x_symbol_padding);
                if x_sum >= area.right().saturating_sub(2) {
                    break;
                }

                let item_area = Rect::new(
                    x_sum,
                    area.y,
                    area.right().saturating_sub(x_sum),
                    area.height,
                );

                item.render(canvas, item_area, codex);

                // Scrolling the list if needed
                if is_selected {
                    let pos = canvas.last_cell().map_or_else(|| current_x, |(lx, _)| lx);
                    if pos >= area.right().saturating_sub(5) {
                        self.state.as_mut().scroll_offset += 3;
                    }
                    if i == self.state.as_ref().scroll_offset
                        && self.state.as_ref().scroll_offset != 0
                    {
                        self.state.as_mut().scroll_offset -= 1;
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
            if let (state, Some(selected)) = (self.state.as_mut(), selected_idx) {
                let height = area.height as usize;

                if selected < state.scroll_offset {
                    state.scroll_offset = selected;
                } else if selected >= state.scroll_offset + height {
                    state.scroll_offset = selected.saturating_sub(height).saturating_add(1);
                }
            }

            let offset = self.state.as_ref().scroll_offset;

            for (i, item) in self.items.iter_mut().enumerate().skip(offset) {
                let line_index = i - offset;
                #[allow(clippy::cast_possible_truncation)]
                let y = area.y.saturating_add(line_index as u16);

                if y >= area.bottom() {
                    break;
                }

                let is_selected = Some(i) == selected_idx;

                if is_selected {
                    item.style(self.selected_style);

                    if let Some(symbol) = self.selected_symbol {
                        canvas.set_ccell(
                            area.x.saturating_add(1),
                            y,
                            CCell {
                                char: symbol,
                                style: self.selected_style,
                            },
                        );
                        canvas.set_ccell(
                            area.x.saturating_add(2),
                            y,
                            CCell {
                                char: SPACE_GLYPH,
                                style: self.selected_style,
                            },
                        );
                    }
                }

                let item_area = Rect::new(
                    area.x.saturating_add(x_offset),
                    y,
                    area.width.saturating_sub(x_offset),
                    1,
                );

                item.render(canvas, item_area, codex);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::Text;

    #[test]
    fn test_list_render_vertical() {
        let codex = Codex::new();
        let mut canvas = crate::render::Canvas::new(10, 5);
        let mut state = ListState::default();
        let mut items = vec![
            Text::new("Item 1", &codex),
            Text::new("Item 2", &codex),
            Text::new("Item 3", &codex),
        ];
        let mut list = List::new(&mut state, items.iter_mut());
        let area = Rect::new(0, 0, 10, 5);

        list.render(&mut canvas, area, &codex);

        // Check if items are rendered line by line
        assert_eq!(canvas.get_ccell(0, 0).char, codex.lookup('I'));
        assert_eq!(canvas.get_ccell(0, 1).char, codex.lookup('I'));
        assert_eq!(canvas.get_ccell(0, 2).char, codex.lookup('I'));
    }

    #[test]
    fn test_list_selected_symbol() {
        let codex = Codex::new();
        let mut canvas = crate::render::Canvas::new(10, 5);
        let mut state = ListState::default();
        state.selected = Some(1);
        let mut items = vec![
            Text::new("Item 1", &codex),
            Text::new("Item 2", &codex),
        ];
        let mut list = List::new(&mut state, items.iter_mut())
            .with_selected_symbol('>', &codex);
        let area = Rect::new(0, 0, 10, 5);

        list.render(&mut canvas, area, &codex);

        // Item 2 is at y=1. Symbol should be at x=1.
        assert_eq!(canvas.get_ccell(1, 1).char, codex.lookup('>'));
        // Text should be offset by 3.
        assert_eq!(canvas.get_ccell(3, 1).char, codex.lookup('I'));
    }

    #[test]
    fn test_list_scrolling() {
        let codex = Codex::new();
        let mut canvas = crate::render::Canvas::new(10, 2); // Only 2 lines high
        let mut state = ListState::default();
        state.selected = Some(2); // Third item selected
        let mut items = vec![
            Text::new("Item 1", &codex),
            Text::new("Item 2", &codex),
            Text::new("Item 3", &codex),
        ];
        let mut list = List::new(&mut state, items.iter_mut());
        let area = Rect::new(0, 0, 10, 2);

        list.render(&mut canvas, area, &codex);

        // Should have scrolled to show Item 3 at some position.
        // If height is 2, and selected is 2 (index 2), scroll_offset becomes 1.
        // So Item 2 and Item 3 are shown.
        assert_eq!(state.scroll_offset, 1);
        assert_eq!(canvas.get_ccell(0, 0).char, codex.lookup('I')); // This is Item 2
        assert_eq!(canvas.get_ccell(5, 0).char, codex.lookup('2'));
        assert_eq!(canvas.get_ccell(5, 1).char, codex.lookup('3'));
    }
}
