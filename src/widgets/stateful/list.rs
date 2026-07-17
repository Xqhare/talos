use crate::{
    layout::Rect,
    render::{CCell, Grapheme, Style},
    widgets::{Block, traits::Widget},
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
///     let (canvas, thoth) = talos.render_ctx();
///
///     let mut list_state = ListState::default();
///     list_state.selected = Some(0);
///
///     let items: Vec<Box<dyn Widget>> = vec![
///         Box::new(Text::new("Item 1", thoth)) as Box<dyn Widget>,
///         Box::new(Text::new("Item 2", thoth)) as Box<dyn Widget>,
///         Box::new(Text::new("Item 3", thoth)) as Box<dyn Widget>,
///     ];
///
///     let mut list = List::new(&mut list_state, items);
///
///     let rect = Rect::new(0, 0, 20, 10);
///     list.render(canvas, rect, thoth);
///
///     talos.present()?;
///
///     Ok(())
/// }
/// ```

#[must_use]
pub struct List<'a> {
    items: Vec<Box<dyn Widget + 'a>>,
    state: &'a mut ListState,
    style: Style,
    selected_style: Style,
    selected_symbol: Option<Grapheme>,
    horizontal: bool,
    item_height: u16,
    as_buttons: bool,
    fat_border: bool,
}

/// The state of a list
#[derive(Default, Debug, Clone, Copy)]
pub struct ListState {
    /// The index of the currently selected item
    pub selected: Option<usize>,
    /// The offset of the list - used for scrolling
    pub scroll_offset: usize,
}

impl ListState {
    /// Creates a new, empty list state
    ///
    /// # Fields
    /// * `selected` - The index of the currently selected item; Initalised to `None`
    /// * `scroll_offset` - The offset of the list - used for scrolling; Initalised to `0`
    pub fn new() -> Self {
        Self {
            selected: None,
            scroll_offset: 0,
        }
    }
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
    /// let (_, thoth) = talos.render_ctx();
    /// let mut list_state = ListState::default();
    /// let items: Vec<Box<dyn Widget>> = Vec::new();
    /// let list = List::new(&mut list_state, items);
    /// # assert!(true);
    /// ```
    pub fn new<I>(state: &'a mut ListState, items: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn Widget + 'a>>,
    {
        Self {
            items: items.into_iter().collect(),
            state,
            style: Style::default(),
            selected_style: Style::default(),
            selected_symbol: None,
            horizontal: false,
            item_height: 1,
            as_buttons: false,
            fat_border: false,
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

    /// Sets the height of each item in the list
    pub fn with_item_height(mut self, height: u16) -> Self {
        self.item_height = height;
        self
    }

    /// Sets the list to render each item as a button
    pub fn with_as_buttons(mut self) -> Self {
        self.as_buttons = true;
        self
    }

    /// Sets the border of the buttons to be fat or double lined
    pub fn with_fat_border(mut self) -> Self {
        self.fat_border = true;
        self
    }

    /// Sets the style of the selected item
    pub fn with_selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    /// Sets the symbol of the selected item - this is rendered in front (to the left) of the
    /// selected item
    pub fn with_selected_symbol(mut self, char: char, _thoth: &thoth::Thoth) -> Self {
        self.selected_symbol = Some(crate::render::Grapheme::new(char.encode_utf8(&mut [0; 4])));
        self
    }
}

impl Widget for List<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn inner(&self, area: Rect) -> Vec<Rect> {
        let mut regions = vec![area];
        if self.horizontal {
            // Horizontal list item layouts are render-dependent (widths based on child render states),
            // so we default to returning just the container boundary.
            return regions;
        }

        let offset = self.state.scroll_offset;
        let x_offset = if self.selected_symbol.is_some() { 3 } else { 0 };

        for i in offset..self.items.len() {
            let line_index = i - offset;
            let y = area.y.saturating_add((line_index as u16).saturating_mul(self.item_height));

            if y >= area.bottom() {
                break;
            }

            let item_area = Rect::new(
                area.x.saturating_add(x_offset),
                y,
                area.width.saturating_sub(x_offset),
                self.item_height,
            );
            regions.push(item_area);
        }
        regions
    }
    #[allow(clippy::too_many_lines)]
    fn render(
        &mut self,
        canvas: &mut crate::render::Canvas,
        area: crate::layout::Rect,
        thoth: &thoth::Thoth,
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
                                char: crate::render::Grapheme::default(),
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

                let mut item_area = Rect::new(
                    x_sum,
                    area.y,
                    area.right().saturating_sub(x_sum),
                    area.height,
                );

                if self.as_buttons {
                    let mut block = Block::new().with_bg_fill().with_style(self.style);
                    if self.fat_border {
                        block = block.with_fat_border();
                    }
                    if is_selected {
                        block.style(self.selected_style);
                    }
                    block.render(canvas, item_area, thoth);
                    item_area = block.inner(item_area);
                }

                item.render(canvas, item_area, thoth);

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
                            char: crate::render::Grapheme::default(),
                            style: self.style,
                        },
                    );
                }
            }
        } else {
            // Ensure the selected item is visible before we start rendering.
            if let (state, Some(selected)) = (self.state.as_mut(), selected_idx) {
                let visible_items = (area.height / self.item_height.max(1)) as usize;

                if selected < state.scroll_offset {
                    state.scroll_offset = selected;
                } else if selected >= state.scroll_offset + visible_items {
                    state.scroll_offset = selected.saturating_sub(visible_items).saturating_add(1);
                }
            }

            let offset = self.state.as_ref().scroll_offset;

            for (i, item) in self.items.iter_mut().enumerate().skip(offset) {
                let line_index = i - offset;
                #[allow(clippy::cast_possible_truncation)]
                let y = area
                    .y
                    .saturating_add((line_index as u16).saturating_mul(self.item_height));

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
                                char: crate::render::Grapheme::default(),
                                style: self.selected_style,
                            },
                        );
                    }
                }

                let mut item_area = Rect::new(
                    area.x.saturating_add(x_offset),
                    y,
                    area.width.saturating_sub(x_offset),
                    self.item_height,
                );

                if self.as_buttons {
                    let mut block = Block::new().with_bg_fill().with_style(self.style);
                    if self.fat_border {
                        block = block.with_fat_border();
                    }
                    if is_selected {
                        block.style(self.selected_style);
                    }
                    block.render(canvas, item_area, thoth);
                    item_area = block.inner(item_area);
                }

                item.render(canvas, item_area, thoth);
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
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = crate::render::Canvas::new(10, 5);
        let mut state = ListState::default();
        let items: Vec<Box<dyn Widget>> = vec![
            Box::new(Text::new("Item 1", &thoth)) as Box<dyn Widget>,
            Box::new(Text::new("Item 2", &thoth)) as Box<dyn Widget>,
            Box::new(Text::new("Item 3", &thoth)) as Box<dyn Widget>,
        ];
        let mut list = List::new(&mut state, items);
        let area = Rect::new(0, 0, 10, 5);

        list.render(&mut canvas, area, &thoth);

        // Check if items are rendered line by line
        assert_eq!(canvas.get_ccell(0, 0).char, crate::render::Grapheme::new("I"));
        assert_eq!(canvas.get_ccell(0, 1).char, crate::render::Grapheme::new("I"));
        assert_eq!(canvas.get_ccell(0, 2).char, crate::render::Grapheme::new("I"));
    }

    #[test]
    fn test_list_selected_symbol() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = crate::render::Canvas::new(10, 5);
        let mut state = ListState::default();
        state.selected = Some(1);
        let items: Vec<Box<dyn Widget>> = vec![
            Box::new(Text::new("Item 1", &thoth)) as Box<dyn Widget>,
            Box::new(Text::new("Item 2", &thoth)) as Box<dyn Widget>,
        ];
        let mut list = List::new(&mut state, items).with_selected_symbol('>', &thoth);
        let area = Rect::new(0, 0, 10, 5);

        list.render(&mut canvas, area, &thoth);

        // Item 2 is at y=1. Symbol should be at x=1.
        assert_eq!(canvas.get_ccell(1, 1).char, crate::render::Grapheme::new(">"));
        // Text should be offset by 3.
        assert_eq!(canvas.get_ccell(3, 1).char, crate::render::Grapheme::new("I"));
    }

    #[test]
    fn test_list_scrolling() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = crate::render::Canvas::new(10, 2); // Only 2 lines high
        let mut state = ListState::default();
        state.selected = Some(2); // Third item selected
        let items: Vec<Box<dyn Widget>> = vec![
            Box::new(Text::new("Item 1", &thoth)) as Box<dyn Widget>,
            Box::new(Text::new("Item 2", &thoth)) as Box<dyn Widget>,
            Box::new(Text::new("Item 3", &thoth)) as Box<dyn Widget>,
        ];
        let mut list = List::new(&mut state, items);
        let area = Rect::new(0, 0, 10, 2);

        list.render(&mut canvas, area, &thoth);
        drop(list);

        // Should have scrolled to show Item 3 at some position.
        // If height is 2, and selected is 2 (index 2), scroll_offset becomes 1.
        // So Item 2 and Item 3 are shown.
        assert_eq!(state.scroll_offset, 1);
        assert_eq!(canvas.get_ccell(0, 0).char, crate::render::Grapheme::new("I")); // This is Item 2
        assert_eq!(canvas.get_ccell(5, 0).char, crate::render::Grapheme::new("2"));
        assert_eq!(canvas.get_ccell(5, 1).char, crate::render::Grapheme::new("3"));
    }

    #[test]
    fn test_list_widget_inner() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut state = ListState::default();
        let item1 = Box::new(Text::new("Item 1", &thoth)) as Box<dyn Widget>;
        let item2 = Box::new(Text::new("Item 2", &thoth)) as Box<dyn Widget>;
        
        let list = List::new(&mut state, vec![item1, item2]).with_item_height(2);
        let area = Rect::new(0, 0, 10, 10);
        
        let widget_ref: &dyn Widget = &list;
        let regions = widget_ref.inner(area);
        
        // Index 0: List container bounds, Index 1: Item 1, Index 2: Item 2
        assert_eq!(regions.len(), 3);
        assert_eq!(regions[0], area);
        assert_eq!(regions[1], Rect::new(0, 0, 10, 2));
        assert_eq!(regions[2], Rect::new(0, 2, 10, 2));
    }
}
