use crate::{
    codex::{Codex, pages::SPACE_GLYPH},
    layout::Rect,
    render::{Canvas, CCell, Glyph, Style},
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
    /// The items in the list.
    items: Vec<&'a mut dyn Widget>,
    /// The state of the list.
    state: &'a mut ListState,
    /// The style of the selected item.
    selected_style: Style,
    /// The symbol of the selected item.
    selected_symbol: Option<Glyph>,
    /// Whether the list is horizontal.
    horizontal: bool,
}

/// The state of a list
#[derive(Default, Debug, Clone, Copy)]
#[non_exhaustive]
pub struct ListState {
    /// The index of the currently selected item
    pub selected: Option<usize>,
    /// The offset of the list - used for scrolling
    pub scroll_offset: usize,
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
    #[inline]
    #[must_use]
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

    /// Sets the list to be horizontal
    #[inline]
    #[must_use]
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }

    /// Sets the style of the selected item
    #[inline]
    #[must_use]
    pub fn with_selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    /// Sets the symbol of the selected item - this is rendered in front (to the left) of the
    /// selected item
    #[inline]
    #[must_use]
    pub fn with_selected_symbol(mut self, char: char, codex: &Codex) -> Self {
        self.selected_symbol = Some(codex.lookup(char));
        self
    }
}

impl Widget for List<'_> {
    #[inline]
    fn style(&mut self, style: Style) {
        self.selected_style = style;
    }
    #[inline]
    #[expect(clippy::too_many_lines, reason = "Render functions are naturally long")]
    fn render(
        &mut self,
        canvas: &mut Canvas,
        area: Rect,
        codex: &Codex,
    ) {
        if self.items.is_empty() {
            return;
        }

        let x_offset = if self.selected_symbol.is_some() { 3 } else { 0 };

        let selected_idx = self.state.selected;

        if self.horizontal {
            let offset = self.state.scroll_offset;
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
                        self.state.scroll_offset += 3;
                    }
                    if i == self.state.scroll_offset
                        && self.state.scroll_offset != 0
                    {
                        self.state.scroll_offset -= 1;
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
            if let Some(selected) = selected_idx {
                let height = usize::from(area.height);

                if selected < self.state.scroll_offset {
                    self.state.scroll_offset = selected;
                } else if selected >= self.state.scroll_offset + height {
                    self.state.scroll_offset = selected.saturating_sub(height).saturating_add(1);
                }
            }

            let offset = self.state.scroll_offset;

            for (i, item) in self.items.iter_mut().enumerate().skip(offset) {
                let line_index = i - offset;
                let y = area.y.saturating_add(u16::try_from(line_index).unwrap_or(u16::MAX));

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
