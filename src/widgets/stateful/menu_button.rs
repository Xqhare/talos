use crate::{
    codex::Codex,
    layout::Rect,
    render::{Canvas, Style},
    widgets::{stateful::Button, traits::Widget},
};

/// The state or contents of a `MenuButton`
pub struct MenuButtonState<'a> {
    /// The main button, always visible
    pub main_button: Button<'a>,
    /// The buttons in the menu, shown when the main button is clicked
    pub menu: Vec<Button<'a>>,
}

/// A button with a menu.
///
/// If the button is clicked, the menu will be displayed.
///
/// If the style is set, it will be used for all the buttons.
/// To use different styles for each button, do not set the style on the `MenuButton` and set it on
/// the `Button` instead.
///
/// The menu will be displayed below the main button
///     - same width as the main button
///     - if `child_heigt` == None, same height as the main button else `child_height`
/// This leads to some overlap with widgets placed directly below the `MenuButton`.
pub struct MenuButton<'a> {
    state: MenuButtonState<'a>,
    style: Option<Style>,
    child_height: Option<u16>,
}

impl<'a> MenuButton<'a> {
    pub fn new(main_button: Button<'a>, menu: Vec<Button<'a>>) -> Self {
        Self {
            state: MenuButtonState { main_button, menu },
            style: None,
            child_height: None,
        }
    }
    /// Sets the height of each button in the menu.
    /// If not set, the height of the main button is used.
    pub fn with_child_height(mut self, height: u16) -> Self {
        self.child_height = Some(height);
        self
    }
}

impl Widget for MenuButton<'_> {
    fn style(&mut self, style: Style) {
        self.style = Some(style);
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        let main_button = &mut self.state.main_button;
        if let Some(style) = self.style {
            main_button.style(style);
        }
        main_button.render(canvas, area, codex);

        if let Some(main_state) = main_button.get_state()
            && main_state.clicked
        {
            let child_width = area.width;
            let child_height = self.child_height.unwrap_or(area.height);

            for (num, child) in self.state.menu.iter_mut().enumerate() {
                if let Some(style) = self.style {
                    child.style(style);
                }

                // Calculate the offset for each child button
                let offset = (num as u16).saturating_mul(child_height);
                let child_area = Rect {
                    x: area.x,
                    y: area.bottom().saturating_add(offset),
                    width: child_width,
                    height: child_height,
                };
                child.render(canvas, child_area, codex);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::stateful::ButtonState;

    #[test]
    fn test_menu_button_render_no_overlap() {
        let codex = Codex::default();
        let mut canvas = Canvas::new(20, 20);
        let mut main_state = ButtonState { clicked: true };
        let main_button = Button::new("Main", &codex).with_state(&mut main_state);
        let menu_item = Button::new("Item 1", &codex);

        let mut menu_button = MenuButton::new(main_button, vec![menu_item]);
        let area = Rect::new(0, 0, 10, 1);

        menu_button.render(&mut canvas, area, &codex);

        // The main button is at (0,0) with height 1, so its bottom is 1.
        // The first menu item should start at y=1.
        // We verify this by checking if the canvas has content at y=1.
        // Button uses Block with bg fill, so it should write something.
        let cell = canvas.get_ccell(0, 1);
        assert_ne!(cell.char, 0); // Assuming 0 is the default empty char
    }
}
