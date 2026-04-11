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
///     widgets::{stateful::Button, traits::Widget},
/// };
///
/// fn main() -> Result<(), talos::TalosError> {
///     let mut talos = Talos::builder().build()?;
///
///     talos.begin_frame();
///     let (canvas, codex) = talos.render_ctx();
///
///     let rect = Rect::new(0, 0, 20, 10);
///     let mut button = Button::new("Hello, world!", &codex);
///     button.render(canvas, rect, codex);
///
///     talos.present()?;
///
///     Ok(())
/// }
/// ```
pub struct Button<'a> {
    /// The text of the button.
    text: Text,
    /// The style of the button.
    style: Style,
    /// The state of the button.
    state: Option<&'a mut ButtonState>,
    /// The style when the button is clicked.
    clicked_style: Style,
    /// The text to display when the button is clicked.
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
    /// use talos::{Talos, widgets::stateful::Button};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, codex) = talos.render_ctx();
    /// let button = Button::new("Hello, world!", &codex);
    /// # assert!(true);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(text: impl Into<String>, codex: &Codex) -> Self {
        let mut text = Text::new(text, codex);
        text = text.align_vertically().align_center();
        Self {
            text,
            style: Style::default(),
            clicked_style: Style::default(),
            clicked_text: None,
            state: None,
        }
    }
    /// Sets the state of the button
    ///
    /// The state must be externally managed.
    ///
    /// # Arguments
    /// * `state` - The state of the button
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::stateful::{Button, ButtonState}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, codex) = talos.render_ctx();
    /// let mut button_state = ButtonState { clicked: true };
    /// let button = Button::new("Hello, world!", &codex).with_state(&mut button_state);
    /// # assert!(true);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_state(mut self, state: &'a mut ButtonState) -> Self {
        self.state = Some(state);
        self
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
    /// let button = Button::new("Hello, world!", &codex).with_state(&mut button_state);
    /// let state = button.get_state().unwrap();
    /// assert!(state.clicked);
    /// # assert!(true);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_state(&self) -> Option<&ButtonState> {
        self.state.as_deref()
    }
    /// This style is used when the button is clicked.
    /// Not used for the `Text` widget itself.
    #[inline]
    #[must_use]
    pub fn with_clicked_style(mut self, style: Style) -> Self {
        self.clicked_style = style;
        self
    }
    /// This text is used when the button is clicked
    ///
    /// If left blank or unused, the original text is used
    #[inline]
    #[must_use]
    pub fn with_clicked_text(mut self, text: impl Into<String>, codex: &Codex) -> Self {
        let mut text = Text::new(text, codex);
        text = text.align_vertically().align_center();
        self.clicked_text = Some(text);
        self
    }
}

impl Widget for Button<'_> {
    #[inline]
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    #[inline]
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        let bg_style = if let Some(state) = &self.state {
            if state.clicked {
                self.clicked_style
            } else {
                self.style
            }
        } else {
            self.style
        };

        let mut outer_block = Block::new().with_bg_fill();
        outer_block.style(bg_style);
        outer_block.render(canvas, area, codex);

        let inner_rect = outer_block.inner(area);
        if let Some(state) = &mut self.state
            && state.clicked
            && let Some(text) = &mut self.clicked_text
        {
            text.style(bg_style);
            text.render(canvas, inner_rect, codex);
            return;
        }
        self.text.style(bg_style);
        self.text.render(canvas, inner_rect, codex);
    }
}
