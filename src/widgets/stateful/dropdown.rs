use crate::{
    codex::Codex,
    layout::Rect,
    render::{Canvas, Style},
    widgets::{
        stateful::{Button, ButtonState, List, ListState},
        traits::{Widget, make_dyn_iter},
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
    items: Vec<&'a mut dyn Widget>,
    state: &'a mut DropdownState,
    style: Style,
    placeholder: String,
    label: Option<String>,
    list_height: Option<u16>,
}

impl<'a> Dropdown<'a> {
    /// Creates a new `Dropdown`
    ///
    /// # Arguments
    /// * `state` - The state of the dropdown
    /// * `items` - The items in the dropdown
    pub fn new<I, W>(state: &'a mut DropdownState, items: I) -> Self
    where
        I: Iterator<Item = &'a mut W>,
        W: Widget + 'a,
    {
        Self {
            items: make_dyn_iter(items),
            state,
            style: Style::default(),
            placeholder: "Select...".to_string(),
            label: None,
            list_height: None,
        }
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

    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
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
        let mut button = Button::new(display_text, &mut button_state, codex).with_style(self.style);
        button.render(canvas, area, codex);

        // Render the list if expanded
        if self.state.expanded {
            let list_height = self
                .list_height
                .unwrap_or_else(|| (self.items.len() as u16).min(10));
            let list_area = Rect::new(area.x, area.bottom(), area.width, list_height);

            let mut list = List::new(&mut self.state.list_state, self.items.iter_mut())
                .with_selected_style(self.style); // Use same style for selection for now

            list.render(canvas, list_area, codex);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::Text;

    #[test]
    fn test_dropdown_render_collapsed() {
        let codex = Codex::new();
        let mut canvas = Canvas::new(20, 10);
        let mut state = DropdownState::default();
        let mut item1 = Text::new("Item 1", &codex);
        let items = vec![&mut item1];

        let mut dropdown = Dropdown::new(&mut state, items.into_iter());
        let area = Rect::new(0, 0, 10, 3);

        dropdown.render(&mut canvas, area, &codex);

        // Should show placeholder "Select..."
        // Let's just check if 'S' is somewhere.
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
        let mut item1 = Text::new("Option 1", &codex);
        let items = vec![&mut item1];

        let mut dropdown = Dropdown::new(&mut state, items.into_iter());
        let area = Rect::new(0, 0, 10, 1); // 1 height button

        dropdown.render(&mut canvas, area, &codex);

        // List should be rendered at y=1.
        // Option 1 starts with 'O'.
        assert_eq!(canvas.get_ccell(0, 1).char, codex.lookup('O'));
    }
}
