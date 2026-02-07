use crate::{
    codex::{Codex, pages::SPACE_GLYPH},
    layout::Rect,
    render::{CCell, Canvas, Glyph, Style},
    widgets::traits::Widget,
};

/// A simple signal box
///
/// Takes up one cell, changes symbol based on state
#[must_use]
pub struct SignalBox<'a> {
    state: Option<&'a mut SignalBoxState>,
    style: Style,
    signal_on_symbol: Glyph,
    signal_off_symbol: Glyph,
}

pub struct SignalBoxState {
    pub signal: bool,
}

impl Default for SignalBox<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> SignalBox<'a> {
    pub fn new() -> Self {
        Self {
            state: None,
            style: Style::default(),
            // UTF Geometric Shapes is the 3rd page
            signal_on_symbol: 0x0328,  // 3 - 40
            signal_off_symbol: 0x0327, // 3 - 39
        }
    }

    pub fn with_state(mut self, state: &'a mut SignalBoxState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn with_signal_on_symbol(mut self, char: char, codex: &Codex) -> Self {
        self.signal_on_symbol = codex.lookup(char);
        self
    }

    pub fn with_signal_off_symbol(mut self, char: char, codex: &Codex) -> Self {
        self.signal_off_symbol = codex.lookup(char);
        self
    }
}

impl Widget for SignalBox<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, _codex: &Codex) {
        if let Some(state) = &self.state {
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
        } else {
            canvas.set_ccell(
                area.x,
                area.y,
                CCell {
                    char: SPACE_GLYPH,
                    style: self.style,
                },
            );
        }
    }
}
