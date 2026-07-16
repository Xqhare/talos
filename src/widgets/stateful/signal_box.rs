use crate::{
    layout::Rect,
    render::{CCell, Canvas, Style},
    widgets::traits::Widget,
};

/// A stateful widget that displays a boolean state.
///
/// The `SignalBox` widget can be used to display a boolean state. The state of the signal box is
/// managed by a `SignalBoxState` struct, which must be passed to the `with_state` method.
///
/// # Example
///
/// ```rust,no_run
/// use talos::{
///     Talos,
///     input::{Event, KeyCode, KeyEvent},
///     layout::Rect,
///     widgets::{
///         stateful::{SignalBox, SignalBoxState},
///         traits::Widget,
///     },
/// };
///
/// fn main() -> Result<(), talos::TalosError> {
///     let mut talos = Talos::builder().build()?;
///     let (canvas, thoth) = talos.render_ctx();
///
///     let mut signal_box_state = SignalBoxState { signal: true };
///
///     let mut signal_box = SignalBox::new(&mut signal_box_state);
///
///     let rect = Rect::new(0, 0, 1, 1);
///     signal_box.render(canvas, rect, thoth);
///
///     talos.present()?;
///
///     Ok(())
/// }
/// ```
#[must_use]
pub struct SignalBox<'a> {
    state: &'a mut SignalBoxState,
    style: Style,
    signal_on_symbol: crate::render::Grapheme,
    signal_off_symbol: crate::render::Grapheme,
}

/// The state of a signal box
#[derive(Default, Debug, Clone, Copy)]
pub struct SignalBoxState {
    /// Whether the signal is on or off
    pub signal: bool,
}

impl SignalBoxState {
    /// Creates a new signal box state
    ///
    /// # Fields
    /// * `signal` - Whether the signal is on or off; Initalised to `false`
    pub fn new() -> Self {
        Self { signal: false }
    }
    /// Sets the state of the signal box
    pub fn set_signal(&mut self, signal: bool) {
        self.signal = signal;
    }
    /// Toggles the state of the signal box
    pub fn toggle_signal(&mut self) {
        self.signal = !self.signal;
    }
}

impl<'a> SignalBox<'a> {
    /// Creates a new, empty signal box
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::stateful::{SignalBox, SignalBoxState}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, thoth) = talos.render_ctx();
    /// let mut state = SignalBoxState { signal: true };
    /// let signal_box = SignalBox::new(&mut state);
    /// # assert!(true);
    /// ```
    pub fn new(state: &'a mut SignalBoxState) -> Self {
        Self {
            state,
            style: Style::default(),
            // The default symbols are from the "UTF Geometric Shapes" page (page 3).
            // The glyph is constructed by combining the page ID and the character ID.
            // For example, `0x0328` is `(3 << 8) | 40`.
            signal_on_symbol: crate::render::Grapheme::new("◈"),
            signal_off_symbol: crate::render::Grapheme::new("◇"),
        }
    }

    /// Gets the state of the signal box
    pub fn get_state(&mut self) -> &mut SignalBoxState {
        &mut self.state
    }

    /// Sets the on symbol of the signal box
    pub fn with_signal_on_symbol(mut self, char: char, _thoth: &thoth::Thoth) -> Self {
        self.signal_on_symbol = crate::render::Grapheme::new(char.encode_utf8(&mut [0; 4]));
        self
    }

    /// Sets the off symbol of the signal box
    pub fn with_signal_off_symbol(mut self, char: char, _thoth: &thoth::Thoth) -> Self {
        self.signal_off_symbol = crate::render::Grapheme::new(char.encode_utf8(&mut [0; 4]));
        self
    }

    /// Uses `☐` and `☑` as the on and off symbols instead of the default diamond
    pub fn use_classical_symbols(mut self) -> Self {
        self.signal_on_symbol = crate::render::Grapheme::new("☑");
        self.signal_off_symbol = crate::render::Grapheme::new("☐");
        self
    }
}

impl Widget for SignalBox<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, _thoth: &thoth::Thoth) {
        let state = &self.state;
        let symbol = if state.signal {
            self.signal_on_symbol
        } else {
            self.signal_off_symbol
        };
        canvas.set_ccell(
            area.x,
            area.y,
            CCell {
                char: symbol,
                style: self.style,
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_box_render_on() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(1, 1);
        let mut state = SignalBoxState { signal: true };
        let mut signal_box = SignalBox::new(&mut state);
        let area = Rect::new(0, 0, 1, 1);

        signal_box.render(&mut canvas, area, &thoth);

        assert_eq!(canvas.get_ccell(0, 0).char, crate::render::Grapheme::new("◈"));
    }

    #[test]
    fn test_signal_box_render_off() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(1, 1);
        let mut state = SignalBoxState { signal: false };
        let mut signal_box = SignalBox::new(&mut state);
        let area = Rect::new(0, 0, 1, 1);

        signal_box.render(&mut canvas, area, &thoth);

        assert_eq!(canvas.get_ccell(0, 0).char, crate::render::Grapheme::new("◇"));
    }

    #[test]
    fn test_signal_box_classical_symbols() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(1, 1);
        let mut state = SignalBoxState { signal: true };
        let mut signal_box = SignalBox::new(&mut state).use_classical_symbols();
        let area = Rect::new(0, 0, 1, 1);

        signal_box.render(&mut canvas, area, &thoth);

        assert_eq!(canvas.get_ccell(0, 0).char, crate::render::Grapheme::new("☑"));
    }
}
