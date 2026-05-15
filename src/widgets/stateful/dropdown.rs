use crate::{
    layout::Rect,
    render::Style,
    widgets::{
        stateful::{Button, ButtonState, List, ListState},
        traits::Widget,
    },
};

/// The state of a `Dropdown`
#[derive(Default, Debug, Clone, Copy)]
pub struct DropdownState {
    /// Whether the dropdown is expanded
    pub expanded: bool,
    /// The state of the internal list
    pub list_state: ListState,
}

/// A dropdown selection widget
///
/// The `Dropdown` widget allows selecting an item from a list.
/// It consists of a button that, when clicked, shows a list of items.
///
/// Clicks and state changes must be handled by the user.
pub struct Dropdown<'a> {
    items: Vec<Box<dyn Widget + 'a>>,
    state: &'a mut DropdownState,
    style: Style,
    active_style: Style,
    selected_style: Style,
    placeholder: String,
    label: Option<String>,
    list_height: Option<u16>,
    fat_border: bool,
}

impl<'a> Dropdown<'a> {
    /// Creates a new `Dropdown`
    ///
    /// # Arguments
    /// * `state` - The state of the dropdown
    pub fn new(state: &'a mut DropdownState) -> Self {
        Self {
            items: Vec::new(),
            state,
            style: Style::default(),
            active_style: Style::default(),
            selected_style: Style::default(),
            placeholder: "Select...".to_string(),
            label: None,
            list_height: None,
            fat_border: false,
        }
    }

    /// Adds an item to the dropdown
    pub fn add<W: Widget + 'a>(mut self, item: W) -> Self {
        self.items.push(Box::new(item));
        self
    }

    /// Sets the items of the dropdown
    pub fn with_items<I>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn Widget + 'a>>,
    {
        self.items = items.into_iter().collect();
        self
    }

    /// Sets the style of the dropdown main button if active
    pub fn with_active_style(mut self, style: Style) -> Self {
        self.active_style = style;
        self
    }

    /// Sets the style of the dropdown used for the selected item
    pub fn with_selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    /// Sets the border of the dropdown to be fat or double lined
    pub fn with_fat_border(mut self) -> Self {
        self.fat_border = true;
        self
    }

    /// Sets the placeholder text shown when no item is selected
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Sets the label shown on the button.
    /// If not set, the placeholder or a default "Item #" will be used.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the height of the expanded list.
    /// If not set, it defaults to the number of items, capped at 10.
    pub fn with_list_height(mut self, height: u16) -> Self {
        self.list_height = Some(height);
        self
    }
}

impl Widget for Dropdown<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }

    fn render(&mut self, ctx: &mut crate::render::RenderContext, area: Rect) {
        let codex = ctx.codex;

        let display_text = if let Some(label) = &self.label {
            label.clone()
        } else if let Some(selected) = self.state.list_state.selected {
            format!("Item {}", selected)
        } else {
            self.placeholder.clone()
        };

        // Render the main button
        let mut button_state = ButtonState {
            clicked: self.state.expanded,
        };
        let mut button = Button::new(display_text, &mut button_state, codex)
            .with_style(self.style)
            .with_clicked_style(self.active_style);
        if self.fat_border {
            button = button.with_fat_border();
        }
        button.render(ctx, area);

        // Render the list if expanded
        if self.state.expanded {
            let item_height = area.height;
            let list_height = self.list_height.unwrap_or_else(|| {
                (self.items.len() as u16)
                    .saturating_mul(item_height)
                    .min(10u16.saturating_mul(item_height))
            });
            let list_area = Rect::new(area.x, area.bottom(), area.width, list_height);

            let mut list = List::new(&mut self.state.list_state)
                .with_style(self.style)
                .with_selected_style(self.selected_style)
                .with_as_buttons()
                .with_item_height(item_height);

            for item in self.items.iter_mut() {
                list = list.add(item as &mut dyn Widget);
            }

            if self.fat_border {
                list = list.with_fat_border();
            }

            list.render(ctx, list_area);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::RenderContext;
    use crate::widgets::Text;

    #[test]
    fn test_dropdown_render_collapsed() {
        let codex = Codex::new();
        let mut canvas = Canvas::new(20, 10);
        let mut state = DropdownState::default();
        let item1 = Text::new("Item 1", &codex);

        let mut dropdown = Dropdown::new(&mut state).add(item1);
        let area = Rect::new(0, 0, 10, 3);

        let mut ctx = RenderContext::new(&mut canvas, &codex);
        dropdown.render(&mut ctx, area);

        // Should show placeholder "Select..."
        let mut found = false;
        for x in 0..10 {
            if canvas.get_ccell(x, 1).char == codex.lookup('S') {
                found = true;
                break;
            }
        }
        assert!(found);
    }

    #[test]
    fn test_dropdown_render_expanded() {
        let codex = Codex::new();
        let mut canvas = Canvas::new(20, 10);
        let mut state = DropdownState {
            expanded: true,
            list_state: ListState::default(),
        };
        let item1 = Text::new("Option 1", &codex);

        let mut dropdown = Dropdown::new(&mut state).add(item1);
        let area = Rect::new(0, 0, 10, 3); // 3 height button

        let mut ctx = RenderContext::new(&mut canvas, &codex);
        dropdown.render(&mut ctx, area);

        // List should be rendered starting at y=3.
        assert_eq!(canvas.get_ccell(1, 4).char, codex.lookup('O'));
    }
}
