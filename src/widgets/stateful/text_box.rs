use crate::{
    codex::Codex,
    layout::Rect,
    render::{Canvas, Style},
    widgets::{Text, traits::Widget},
};

/// The state of a `TextBox`.
#[non_exhaustive]
pub struct TextBoxState {
    /// Whether the text box is currently focused/active.
    pub active: bool,
    /// The current position of the cursor within the text.
    pub cursor: Option<usize>,
    /// The text content of the text box.
    pub text: Text,
}

/// A widget for text input.
pub struct TextBox<'a> {
    /// The state of the text box.
    state: Option<&'a mut TextBoxState>,
    /// The style of the text box.
    style: Style,
    /// The style of the highlighted part of the text box.
    highlight_style: Option<Style>,
}

impl<'a> TextBox<'a> {
    /// Create a new `TextBox` with the given state.
    #[inline]
    #[must_use]
    pub fn new(state: &'a mut TextBoxState) -> Self {
        Self {
            state: Some(state),
            style: Style::default(),
            highlight_style: None,
        }
    }

    /// Set the style for the highlighted part of the text box (e.g. the cursor).
    #[inline]
    #[must_use]
    pub fn with_highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = Some(style);
        self
    }
}

impl Widget for TextBox<'_> {
    #[inline]
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    #[inline]
    fn render(
        &mut self,
        canvas: &mut Canvas,
        area: Rect,
        codex: &Codex,
    ) {
        if let Some(state) = &mut self.state {
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
}
