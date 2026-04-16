use crate::{
    codex::Codex,
    render::Style,
    widgets::{
        Text,
        traits::{State, Widget},
    },
};

/// The state of a `TextBox`.
pub struct TextBoxState {
    /// Whether the text box is currently focused/active.
    pub active: bool,
    /// The current position of the cursor within the text.
    pub cursor: Option<usize>,
    /// The text content of the text box.
    pub text: Text,
}

impl State for TextBoxState {}

/// A widget for text input.
pub struct TextBox<'a> {
    state: &'a mut TextBoxState,
    style: Style,
    highlight_style: Option<Style>,
}

impl<'a> TextBox<'a> {
    /// Create a new `TextBox` with the given state.
    pub fn new(state: &'a mut TextBoxState) -> Self {
        Self {
            state: state,
            style: Style::default(),
            highlight_style: None,
        }
    }

    /// Get the state of the `TextBox`.
    pub fn get_state(&self) -> &TextBoxState {
        &self.state
    }

    /// Set the style for the highlighted part of the text box (e.g. the cursor).
    pub fn with_highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = Some(style);
        self
    }
}

impl Widget for TextBox<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(
        &mut self,
        canvas: &mut crate::render::Canvas,
        area: crate::layout::Rect,
        codex: &Codex,
    ) {
        let state = &mut self.state;
        let cursor = if state.active { state.cursor } else { None };

        // Apply blink to the highlight style
        let highlight_style = self
            .highlight_style
            .map(|s| s.new_from_self().set_blink(true).build());

        state
            .text
            .get_mut_content()
            .with_highlight(cursor, highlight_style);
        state.text.style(self.style);
        state.text.render(canvas, area, codex);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Canvas;

    #[test]
    fn test_text_box_render_active() {
        let codex = Codex::new();
        let mut canvas = Canvas::new(10, 1);
        let mut state = TextBoxState {
            active: true,
            cursor: Some(1),
            text: Text::new("AB", &codex),
        };
        let mut text_box = TextBox::new(&mut state);
        let area = Rect::new(0, 0, 10, 1);

        text_box.render(&mut canvas, area, &codex);

        // 'A' at 0, 'B' at 1. Cursor is at 1.
        // Cell at 1 should have blink set (if highlight_style is None)
        assert_eq!(canvas.get_ccell(1, 0).char, codex.lookup('B'));
        // We can't easily check blink bit from Style here unless we know its structure,
        // but we can check if it's different from default style.
        assert!(canvas.get_ccell(1, 0).style.get_blink().unwrap_or(false));
    }

    #[test]
    fn test_text_box_render_inactive() {
        let codex = Codex::new();
        let mut canvas = Canvas::new(10, 1);
        let mut state = TextBoxState {
            active: false,
            cursor: Some(1),
            text: Text::new("AB", &codex),
        };
        let mut text_box = TextBox::new(&mut state);
        let area = Rect::new(0, 0, 10, 1);

        text_box.render(&mut canvas, area, &codex);

        // Cell at 1 should NOT have blink set
        assert!(!canvas.get_ccell(1, 0).style.get_blink().unwrap_or(false));
    }
}
