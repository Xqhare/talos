use crate::{
    layout::Rect,
    render::Style,
    widgets::{
        stateful::Button,
        traits::Widget,
    },
};

/// The state or contents of a `MenuButton`
pub struct MenuButtonState<'a> {
    /// The main button, always visible
    pub main_button: Button<'a>,
}

/// A menu button.
///
/// If the `main_button` in the `MenuButtonState` is active, the children will be displayed.
/// If the style is set, it will be used for all the elements of the menu.
/// To use different styles for each button, do not set the style on the `MenuButton`, set it
/// directly on the children instead.
///
/// The layout may be vertical or horizontal. To set use `with_vertical_layout` or `with_horizontal_layout`.
/// Setting the layout on the `MenuButton` will affect only the direct children of the `MenuButton`.
/// If set to `vertical`, the children will be placed below the main button.
/// If set to `horizontal`, the children will be placed to the right of the main button.
///
/// The size of the direct children of the `MenuButton` can be set using `with_child_width` and `with_child_height`.
/// If they are not set, the size of the main button will be used.
///
/// This Widget should be expected to overlap with widgets placed directly below or to the right of it.
/// Take special care with the ordering of the render calls of your widgets.
///
pub struct MenuButton<'a> {
    state: MenuButtonState<'a>,
    menu: Vec<Box<dyn Widget + 'a>>,
    style: Option<Style>,
    child_height: Option<u16>,
    child_width: Option<u16>,
    // Determines in which direction the children are placed
    // If `true`, the children are placed below the main button
    // If `false`, the children are placed to the right of the main button
    vertical: bool,
}

impl<'a> MenuButton<'a> {
    /// Creates a new `MenuButton`
    ///
    /// # Arguments
    /// * `main_button` - The main button
    pub fn new(main_button: Button<'a>) -> Self {
        Self {
            state: MenuButtonState {
                main_button,
            },
            menu: Vec::new(),
            style: None,
            child_height: None,
            child_width: None,
            vertical: true,
        }
    }

    /// Adds an item to the menu
    pub fn add<W: Widget + 'a>(mut self, item: W) -> Self {
        self.menu.push(Box::new(item));
        self
    }

    /// Returns the state of the `MenuButton`
    pub fn get_state(&mut self) -> &mut MenuButtonState<'a> {
        &mut self.state
    }
    /// Sets the layout to be vertical
    ///
    /// Default layout
    /// This will place the children below the main button
    ///
    /// Use `with_horizontal_layout` to place the children to the right of the main button
    pub fn with_vertical_layout(mut self) -> Self {
        self.vertical = true;
        self
    }
    /// Sets the layout to be horizontal
    ///
    /// This will place the children to the right of the main button
    ///
    /// Use `with_vertical_layout` to place the children below the main button
    pub fn with_horizontal_layout(mut self) -> Self {
        self.vertical = false;
        self
    }
    /// Sets the width of each button in the menu.
    /// If not set, the width of the main button is used.
    pub fn with_child_width(mut self, width: u16) -> Self {
        self.child_width = Some(width);
        self
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
    fn render(&mut self, ctx: &mut crate::render::RenderContext, area: Rect) {
        let main_button = &mut self.state.main_button;
        if let Some(style) = self.style {
            main_button.style(style);
        }
        main_button.render(ctx, area);

        let main_state = main_button.get_state();
        if main_state.clicked {
            let child_width = self.child_width.unwrap_or(area.width);
            let child_height = self.child_height.unwrap_or(area.height);

            for (num, child) in self.menu.iter_mut().enumerate() {
                if let Some(style) = self.style {
                    child.style(style);
                }
                let (x, y) = if self.vertical {
                    let offset = (num as u16).saturating_mul(child_height);
                    (area.x, area.bottom().saturating_add(offset))
                } else {
                    let offset = (num as u16).saturating_mul(child_width);
                    (area.right().saturating_add(offset), area.y)
                };

                // Calculate the offset for each child button
                let child_area = Rect {
                    x,
                    y,
                    width: child_width,
                    height: child_height,
                };
                child.render(ctx, child_area);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::stateful::ButtonState;
    use crate::render::RenderContext;

    #[test]
    fn test_menu_button_render_no_overlap() {
        let codex = Codex::new();
        let mut canvas = Canvas::new(20, 20);
        let mut main_state = ButtonState { clicked: true };
        let mut menu_state = ButtonState { clicked: true };
        let main_button = Button::new("Main", &mut main_state, &codex);
        let menu_item = Button::new("Item 1", &mut menu_state, &codex);

        let mut menu_button = MenuButton::new(main_button).add(menu_item);
        let area = Rect::new(0, 0, 10, 1);

        let mut ctx = RenderContext::new(&mut canvas, &codex);
        menu_button.render(&mut ctx, area);

        // The main button is at (0,0) with height 1, so its bottom is 1.
        // The first menu item should start at y=1.
        let cell = canvas.get_ccell(0, 1);
        assert_ne!(cell.char, 0); 
    }
}
