use crate::{
    layout::Rect,
    render::{Canvas, Style},
    widgets::{
        stateful::{Button, ButtonState},
        traits::Widget,
    },
};

/// A menu button.
///
/// If the main button (`state`) is active, the children will be displayed.
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
/// use talos::{Talos, widgets::{traits::Widget, stateful::{ButtonState, MenuButton}}};
/// let mut talos = Talos::builder().build().unwrap();
/// let mut button_state = ButtonState { clicked: true };
/// let menu: Vec<Box<dyn Widget>> = vec![];
/// let menu_button = MenuButton::new("Main", &mut button_state, menu);
/// ```
pub struct MenuButton<'a> {
    state: &'a mut ButtonState,
    text: String,
    menu: Vec<Box<dyn Widget + 'a>>,
    style: Option<Style>,
    child_height: Option<u16>,
    child_width: Option<u16>,
    vertical: bool,
}

impl<'a> MenuButton<'a> {
    /// Creates a new `MenuButton`
    ///
    /// # Arguments
    /// * `text` - The label text of the button
    /// * `state` - The state of the button
    /// * `menu` - The Widgets in the menu
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::{traits::Widget, stateful::{ButtonState, MenuButton}}};
    /// let mut talos = Talos::builder().build().unwrap();
    /// let mut button_state = ButtonState { clicked: true };
    /// let menu: Vec<Box<dyn Widget>> = vec![];
    /// let menu_button = MenuButton::new("Main", &mut button_state, menu);
    /// ```
    pub fn new<I>(text: impl Into<String>, state: &'a mut ButtonState, menu: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn Widget + 'a>>,
    {
        Self {
            state,
            text: text.into(),
            menu: menu.into_iter().collect(),
            style: None,
            child_height: None,
            child_width: None,
            vertical: true,
        }
    }

    /// Returns the state of the `MenuButton`
    pub fn get_state(&mut self) -> &mut ButtonState {
        self.state
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

    fn render(&mut self, canvas: &mut Canvas, area: Rect, thoth: &thoth::Thoth) {
        let mut main_button = Button::new(self.text.as_str(), self.state, thoth);
        if let Some(style) = self.style {
            main_button.style(style);
        }
        main_button.render(canvas, area, thoth);

        if self.state.clicked {
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
                child.render(canvas, child_area, thoth);
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
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(20, 20);
        let mut main_state = ButtonState { clicked: true };
        let mut menu_state = ButtonState { clicked: true };
        let menu_item = Button::new("Item 1", &mut menu_state, &thoth);

        let menu: Vec<Box<dyn Widget + '_>> = vec![Box::new(menu_item)];
        let mut menu_button = MenuButton::new("Main", &mut main_state, menu);
        let area = Rect::new(0, 0, 10, 2);

        menu_button.render(&mut canvas, area, &thoth);

        // The main button is at (0,0) with height 2, so its bottom is 2.
        // The first menu item should start at y=2.
        // We verify this by checking if the canvas has content at y=2.
        // Button uses Block with bg fill, so it should write something.
        let cell = canvas.get_ccell(0, 2);
        assert_ne!(cell.char, crate::render::Grapheme::default()); // Assuming Grapheme::default() is the default empty char
    }

    #[test]
    fn test_menu_button_horizontal_layout() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(20, 2);
        let mut main_state = ButtonState { clicked: true };
        let mut menu_state = ButtonState { clicked: false };
        let item = Button::new("Item", &mut menu_state, &thoth);
        let items: Vec<Box<dyn Widget + '_>> = vec![Box::new(item)];

        let mut menu_button =
            MenuButton::new("Main", &mut main_state, items).with_horizontal_layout();
        let area = Rect::new(0, 0, 5, 3);

        menu_button.render(&mut canvas, area, &thoth);

        // Main button is at (0,0) with width 5.
        // Item should be at (5,0).
        assert_eq!(canvas.get_ccell(5, 0).char, crate::render::Grapheme::new("┌"));
    }

    #[test]
    fn test_menu_button_not_clicked() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(20, 2);
        let mut main_state = ButtonState { clicked: false };
        let mut menu_state = ButtonState { clicked: false };
        let item = Button::new("Item", &mut menu_state, &thoth);
        let items: Vec<Box<dyn Widget + '_>> = vec![Box::new(item)];

        let mut menu_button = MenuButton::new("Main", &mut main_state, items);
        let area = Rect::new(0, 0, 5, 1);

        menu_button.render(&mut canvas, area, &thoth);

        // Main button is at (0,0). Item at (0,1) should NOT be rendered.
        // Button with borders: (0,1) would be '┌' if it were rendered.
        // It should be crate::render::Grapheme::default() (0) if not rendered and canvas was empty.
        assert_eq!(canvas.get_ccell(0, 1).char, crate::render::Grapheme::default());
    }
}
