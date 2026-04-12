use crate::{
    codex::Codex,
    layout::Rect,
    render::{CCell, Canvas, Glyph, Style},
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
///     let (canvas, codex) = talos.render_ctx();
///
///     let mut signal_box_state = SignalBoxState { signal: true };
///
///     let mut signal_box = SignalBox::new(&mut signal_box_state);
///
///     let rect = Rect::new(0, 0, 1, 1);
///     signal_box.render(canvas, rect, codex);
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
    signal_on_symbol: Glyph,
    signal_off_symbol: Glyph,
}

/// The state of a signal box
pub struct SignalBoxState {
    /// Whether the signal is on or off
    pub signal: bool,
}

impl<'a> SignalBox<'a> {
    /// Creates a new, empty signal box
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::stateful::{SignalBox, SignalBoxState}};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, codex) = talos.render_ctx();
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
            signal_on_symbol: 0x0328,
            signal_off_symbol: 0x0327,
        }
    }

    /// Gets the state of the signal box
    pub fn get_state(&mut self) -> &mut SignalBoxState {
        &mut self.state
    }

    /// Sets the on symbol of the signal box
    pub fn with_signal_on_symbol(mut self, char: char, codex: &Codex) -> Self {
        self.signal_on_symbol = codex.lookup(char);
        self
    }

    /// Sets the off symbol of the signal box
    pub fn with_signal_off_symbol(mut self, char: char, codex: &Codex) -> Self {
        self.signal_off_symbol = codex.lookup(char);
        self
    }

    /// Uses `☐` and `☑` as the on and off symbols instead of the default diamond
    pub fn use_classical_symbols(mut self) -> Self {
        self.signal_on_symbol = 0x035E;
        self.signal_off_symbol = 0x035D;
        self
    }
}

impl Widget for SignalBox<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, _codex: &Codex) {
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
