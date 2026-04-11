use crate::{
    codex::Codex,
    layout::Rect,
    render::{Canvas, Style},
    widgets::{
        stateful::Button,
        traits::{Widget, make_dyn_iter},
    },
};

/// The state or contents of a `MenuButton`
pub struct MenuButtonState<'a> {
    /// The main button, always visible
    pub main_button: Button<'a>,
    /// The Widgets in the menu, shown when the main button is clicked.
    ///
    /// To create, consider using `talos::widgets::traits::make_dyn_iter`
    pub menu: Vec<&'a mut dyn Widget>,
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
/// # Example
///
/// ```rust,no_run
/// use talos::{Talos, widgets::{traits::Widget, stateful::{Button, MenuButton}}};
/// let mut menus: Vec<&mut dyn Widget> = vec![];
/// let mut talos = Talos::builder().build().unwrap();
/// let (mut canvas, codex) = talos.render_ctx();
/// let main_button = MenuButton::new(Button::new("Main", &codex), menus.iter_mut());
/// ```
pub struct MenuButton<'a> {
    state: MenuButtonState<'a>,
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
    /// * `menu` - The Widgets in the menu
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::{traits::Widget, stateful::{Button, MenuButton}}};
    /// let mut menus: Vec<&mut dyn Widget> = vec![];
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (mut canvas, codex) = talos.render_ctx();
    /// let main_button = MenuButton::new(Button::new("Main", &codex), menus.iter_mut());
    ///
    /// ```
    ///
    pub fn new<I, W>(main_button: Button<'a>, menu: I) -> Self
    where
        I: Iterator<Item = &'a mut W>,
        W: Widget + 'a,
    {
        Self {
            state: MenuButtonState {
                main_button,
                menu: make_dyn_iter(menu),
            },
            style: None,
            child_height: None,
            child_width: None,
            vertical: true,
        }
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
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        let main_button = &mut self.state.main_button;
        if let Some(style) = self.style {
            main_button.style(style);
        }
        main_button.render(canvas, area, codex);

        if let Some(main_state) = main_button.get_state()
            && main_state.clicked
        {
            let child_width = self.child_width.unwrap_or(area.width);
            let child_height = self.child_height.unwrap_or(area.height);

            for (num, child) in self.state.menu.iter_mut().enumerate() {
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

        let mut inner = vec![menu_item];
        let mut menu_button = MenuButton::new(main_button, inner.iter_mut());
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
