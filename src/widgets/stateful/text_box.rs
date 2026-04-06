use crate::{
    codex::Codex,
    render::Style,
    widgets::{Text, traits::Widget},
};

pub struct TextBoxState {
    pub active: bool,
    pub cursor: Option<usize>,
    pub text: Text,
}

pub struct TextBox<'a> {
    state: Option<&'a mut TextBoxState>,
    style: Style,
    highlight_style: Option<Style>,
}

impl<'a> TextBox<'a> {
    pub fn new(state: &'a mut TextBoxState) -> Self {
        Self {
            state: Some(state),
            style: Style::default(),
            highlight_style: None,
        }
    }
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
        if let Some(state) = &mut self.state {
            let cursor = if state.active { state.cursor } else { None };

            state
                .text
                .get_mut_content()
                .with_highlight(cursor, self.highlight_style);
            state.text.render(canvas, area, codex);
        }
    }
}
