use crate::{
    layout::Rect,
    render::Style,
    widgets::{Text, internal_text::InternalText, traits::Widget},
};

/// The state of a `TextBox`.
#[derive(Debug, Default, Clone)]
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
    state: &'a mut TextBoxState,
    style: Style,
    highlight_style: Option<Style>,
    hint_text: Option<InternalText>,
    id: Option<String>,
}

impl<'a> TextBox<'a> {
    /// Create a new `TextBox` with the given state.
    pub fn new(state: &'a mut TextBoxState) -> Self {
        Self {
            state: state,
            style: Style::default(),
            highlight_style: None,
            hint_text: None,
            id: None,
        }
    }

    /// Sets the ID of the text box for automated interaction mapping
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the hint text for the `TextBox`.
    pub fn with_hint_text(mut self, hint_text: Text) -> Self {
        self.hint_text = Some(hint_text.get_content_internal().clone());
        self
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
        ctx: &mut crate::render::RenderContext,
        area: Rect,
    ) {
        if let Some(id) = &self.id {
            ctx.interactions.register(id, area);
        }
        let state = &mut self.state;
        let cursor = if state.active { state.cursor } else { None };

        // Apply blink to the highlight style
        let highlight_style = if let Some(h_style) = self.highlight_style {
            Some(h_style.new_from_self().set_blink(true).build())
        } else {
            Some(self.style.new_from_self().set_blink(true).build())
        };

        if !state.active
            && state.text.get_content().is_empty()
            && let Some(hint_text) = self.hint_text.as_mut()
        {
            hint_text.with_highlight(cursor, highlight_style);
            hint_text.style(self.style.new_from_self().set_dim(true).build());
            hint_text.render(ctx, area);
            return;
        }

        state
            .text
            .get_mut_content()
            .with_highlight(cursor, highlight_style);
        state.text.style(self.style);
        state.text.render(ctx, area);
    }
}
