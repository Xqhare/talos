use crate::{
    LayoutBuilder,
    codex::Codex,
    layout::{Constraint, Direction, Rect},
    render::{Canvas, Style},
    widgets::{
        Block,
        stateful::{Button, SignalBox, SignalBoxState},
        traits::{State, Widget},
    },
};

/// The state or contents of a `CheckBox`
pub struct CheckBoxState<'a> {
    /// The state of the `SignalBox`
    /// Set the desired Text and clicked state of the `CheckBox` here
    ///
    /// The styling of the `Button` will not be overwritten
    pub button: Button<'a>,
}

impl State for CheckBoxState<'_> {}

/// A widget for a checkbox
///
/// Consists of three widgets - A `SignalBox` and a `Button` wrapped inside a `Block`.
/// Only the state of the `Button` is used, the state of the `SignalBox` is derived from that state
///
/// # Example
/// ```rust,no_run
/// use talos::{
///     Talos,
///     layout::Rect,
///     render::{Colour, Normal, Style},
///     widgets::{stateful::{CheckBox, CheckBoxState, Button, ButtonState}, traits::Widget},
/// };
///
/// fn main() -> Result<(), talos::TalosError> {
///     let mut talos = Talos::builder().build()?;
///     talos.begin_frame();
///     let (mut canvas, codex) = talos.render_ctx();
///
///     let mut button_state = ButtonState { clicked: true };
///     let mut state = CheckBoxState { button: Button::new("Hello, world!", &mut button_state, &codex) };
///     let mut checkbox = CheckBox::new(&mut state);
///     checkbox.render(&mut canvas, Rect::new(0, 0, 10, 1), &codex);
///     # assert!(true);
///     Ok(())
/// }
/// ```
pub struct CheckBox<'a> {
    state: &'a mut CheckBoxState<'a>,
    style: Style,
    fat_border: bool,
}

impl<'a> CheckBox<'a> {
    /// Creates a new checkbox
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, layout::Rect, widgets::{stateful::{CheckBox, CheckBoxState, Button, ButtonState}, traits::Widget}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (mut canvas, codex) = talos.render_ctx();
    /// let mut button_state = ButtonState { clicked: true };
    /// let mut state = CheckBoxState { button: Button::new("Hello, world!", &mut button_state, &codex) };
    /// let mut checkbox = CheckBox::new(&mut state);
    /// checkbox.render(&mut canvas, Rect::new(0, 0, 10, 1), &codex);
    /// # assert!(true);
    /// ```
    pub fn new(state: &'a mut CheckBoxState<'a>) -> Self {
        Self {
            state,
            style: Style::default(),
            fat_border: false,
        }
    }

    /// Gets the state of the checkbox
    pub fn get_state(&self) -> &CheckBoxState<'a> {
        &self.state
    }

    /// Sets the border of the checkbox to be fat or double lined
    pub fn with_fat_border(mut self) -> Self {
        self.fat_border = true;
        self
    }

    /// Sets the border of the checkbox to be fat or double lined
    ///
    /// Same as `with_fat_border` but takes a boolean and does not return self
    pub fn set_fat_border(&mut self, fat: bool) {
        self.fat_border = fat;
    }
}

impl Widget for CheckBox<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        let mut outer_block = Block::new().with_bg_fill();
        outer_block.style(self.style);
        outer_block.set_fat_border(self.fat_border);
        outer_block.render(canvas, area, codex);

        let inner_rect = outer_block.inner(area);
        let layout = LayoutBuilder::new()
            .direction(Direction::Horizontal)
            .add_constraint(Constraint::Max(1))
            .add_constraint(Constraint::Min(1))
            .build()
            .split(inner_rect);

        let mut signal_state = SignalBoxState {
            signal: self.state.button.get_state().clicked,
        };
        let mut signal_box = SignalBox::new(&mut signal_state).use_classical_symbols();
        signal_box.style(self.style);
        signal_box.render(canvas, layout[0], codex);

        self.state.button.render(canvas, layout[1], codex);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::stateful::ButtonState;

    #[test]
    fn test_checkbox_render() {
        let codex = Codex::new();
        let mut canvas = Canvas::new(20, 5);
        let mut button_state = ButtonState { clicked: true };
        let mut state = CheckBoxState {
            button: Button::new("Test", &mut button_state, &codex),
        };
        let mut checkbox = CheckBox::new(&mut state);
        let area = Rect::new(0, 0, 20, 5);

        checkbox.render(&mut canvas, area, &codex);

        // SignalBox should show '[X]' or similar.
        // In classical symbols, it should be 0x035E if clicked.
        // It is rendered in layout[0], which is at (1,1) if area is (0,0,20,3)
        assert_eq!(canvas.get_ccell(1, 1).char, 0x035E);

        // Button text should be in layout[1].
        // layout[1] starts at x=2.
        // Button uses Block internally, so it adds more borders.
        // Inner area of Button starts at x=2+1=3.
        // Button centers text: (15 - 4) / 2 + 1 = 6.
        // Start x = 3 + 6 = 9.
        // y = 1 (inner area start) + 1 (vertical center) = 2.
        assert_eq!(canvas.get_ccell(9, 2).char, codex.lookup('T'));
    }
}
