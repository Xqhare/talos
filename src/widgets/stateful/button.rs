use crate::{
    codex::Codex,
    layout::Rect,
    render::{Canvas, Style},
    widgets::{Block, Text, traits::Widget},
};

/// A button widget
///
/// The `Button` widget is a basic building block for your UI. It is a block that can be used to
/// display text.
///
/// Clicking itself must be implemented by the user.
/// Use the emitted Mouse event (from your input handler), wrapping the coordinates and event type
/// to match against the passed in `Rect`.
///
/// # Example
///
/// ```rust,no_run
/// use talos::{
///     Talos,
///     layout::Rect,
///     render::{Colour, Normal, Style},
///     widgets::{stateful::{Button, ButtonState}, traits::Widget},
/// };
///
/// fn main() -> Result<(), talos::TalosError> {
///     let mut talos = Talos::builder().build()?;
///
///     talos.begin_frame();
///     let (canvas, codex) = talos.render_ctx();
///
///     let rect = Rect::new(0, 0, 20, 10);
///     let mut state = ButtonState { clicked: false };
///     let mut button = Button::new("Hello, world!", &mut state, &codex);
///     button.render(canvas, rect, codex);
///
///     talos.present()?;
///
///     Ok(())
/// }
/// ```
pub struct Button<'a> {
    text: Text,
    style: Style,
    state: &'a mut ButtonState,
    clicked_style: Style,
    clicked_text: Option<Text>,
}

/// The state of the button
pub struct ButtonState {
    /// Whether the button is currently in a clicked/active state.
    pub clicked: bool,
}

impl<'a> Button<'a> {
    /// Create a new button
    ///
    /// # Arguments
    /// * `text` - The text of the button
    /// * `codex` - The codex to use
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::stateful::{Button, ButtonState}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, codex) = talos.render_ctx();
    /// let mut state = ButtonState { clicked: true };
    /// let button = Button::new("Hello, world!", &mut state, &codex);
    /// # assert!(true);
    /// ```
    pub fn new(text: impl Into<String>, state: &'a mut ButtonState, codex: &Codex) -> Self {
        let mut text = Text::new(text, codex);
        text = text.align_vertically().align_center();
        Self {
            text: text,
            style: Style::default(),
            clicked_style: Style::default(),
            clicked_text: None,
            state,
        }
    }
    /// Get the state of the button
    ///
    /// Returns a reference to the state of the button.
    /// To mutate the state, use the `with_state` method.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::stateful::{Button, ButtonState}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, codex) = talos.render_ctx();
    /// let mut button_state = ButtonState { clicked: true };
    /// let button = Button::new("Hello, world!", &mut button_state, &codex);
    /// let state = button.get_state();
    /// assert!(state.clicked);
    /// ```
    pub fn get_state(&self) -> &ButtonState {
        &self.state
    }
    /// This style is used when the button is clicked.
    /// Not used for the `Text` widget itself.
    pub fn with_clicked_style(mut self, style: Style) -> Self {
        self.clicked_style = style;
        self
    }
    /// This text is used when the button is clicked
    ///
    /// If left blank or unused, the original text is used
    pub fn with_clicked_text(mut self, text: impl Into<String>, codex: &Codex) -> Self {
        let mut text = Text::new(text, codex);
        text = text.align_vertically().align_center();
        self.clicked_text = Some(text);
        self
    }
}

impl Widget for Button<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        let state = &self.state;
        let bg_style = {
            if state.clicked {
                self.clicked_style
            } else {
                self.style
            }
        };

        let mut outer_block = Block::new().with_bg_fill();
        outer_block.style(bg_style);
        outer_block.render(canvas, area, codex);

        let inner_rect = outer_block.inner(area);
        if state.clicked {
            if let Some(text) = &mut self.clicked_text {
                text.style(bg_style);
                text.render(canvas, inner_rect, codex);
                return;
            }
        }
        self.text.style(bg_style);
        self.text.render(canvas, inner_rect, codex);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::{Colour, Normal};
    use crate::widgets::traits::Widget;

    #[test]
    fn test_button_render_normal() {
        let codex = Codex::new();
        let mut canvas = Canvas::new(10, 3);
        let mut state = ButtonState { clicked: false };
        let mut button = Button::new("OK", &mut state, &codex);
        let area = Rect::new(0, 0, 10, 3);

        button.render(&mut canvas, area, &codex);

        // Should have "OK" centered.
        // inner area of 10x3 block is 8x1 (if default block has borders).
        // (8-2)/2 = 3. so it should start at x=1+3=4.
        assert_eq!(canvas.get_ccell(4, 1).char, codex.lookup('O'));
        assert_eq!(canvas.get_ccell(5, 1).char, codex.lookup('K'));
    }

    #[test]
    fn test_button_render_clicked() {
        let codex = Codex::new();
        let mut canvas = Canvas::new(10, 3);
        let mut state = ButtonState { clicked: true };
        let style = Style::builder().set_fg(Colour::Normal(Normal::Red)).build();
        let mut button = Button::new("OK", &mut state, &codex)
            .with_clicked_style(style);
        let area = Rect::new(0, 0, 10, 3);

        button.render(&mut canvas, area, &codex);

        // Style should be applied to the block (background).
        // Since it's with_bg_fill, we check a cell style.
        assert_eq!(canvas.get_ccell(0, 0).style, style);
    }

    #[test]
    fn test_button_with_clicked_text() {
        let codex = Codex::new();
        let mut canvas = Canvas::new(10, 3);
        let mut state = ButtonState { clicked: true };
        let mut button = Button::new("OK", &mut state, &codex)
            .with_clicked_text("YES", &codex);
        let area = Rect::new(0, 0, 10, 3);

        button.render(&mut canvas, area, &codex);

        // (8-3)/2 + 1 = 2+1 = 3 + 1 (border) = 4.
        // Wait, (10-3)/2 = 3.5 -> 4.
        // Let's just check if 'Y' is there.
        assert_eq!(canvas.get_ccell(4, 1).char, codex.lookup('Y'));
        assert_eq!(canvas.get_ccell(5, 1).char, codex.lookup('E'));
        assert_eq!(canvas.get_ccell(6, 1).char, codex.lookup('S'));
    }
}
