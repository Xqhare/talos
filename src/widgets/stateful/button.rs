use crate::{
    codex::Codex,
    layout::Rect,
    render::Style,
    widgets::{Block, Text, traits::Widget},
};

/// A button widget
///
/// The `Button` widget is a basic building block for your UI. It is a block that can be used to
/// display text.
pub struct Button<'a> {
    text: Text,
    style: Style,
    state: &'a mut ButtonState,
    clicked_style: Style,
    clicked_text: Option<Text>,
    fat_border: bool,
    id: Option<String>,
}

/// The state of a button
#[derive(Default, Debug, Clone, Copy)]
pub struct ButtonState {
    /// Whether the button is currently clicked
    pub clicked: bool,
}

impl<'a> Button<'a> {
    /// Creates a new button
    pub fn new(text: impl Into<String>, state: &'a mut ButtonState, codex: &Codex) -> Self {
        let mut text = Text::new(text, codex);
        text = text.align_vertically().align_center();
        Self {
            text,
            style: Style::default(),
            state,
            clicked_style: Style::default(),
            clicked_text: None,
            fat_border: false,
            id: None,
        }
    }

    /// Sets the ID of the button for automated interaction mapping
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Gets the state of the button
    pub fn get_state(&self) -> &ButtonState {
        &self.state
    }

    /// Sets the style of the button if clicked
    pub fn with_clicked_style(mut self, style: Style) -> Self {
        self.clicked_style = style;
        self
    }
    /// This text is used when the button is clicked
    pub fn with_clicked_text(mut self, text: impl Into<String>, codex: &Codex) -> Self {
        let mut text = Text::new(text, codex);
        text = text.align_vertically().align_center();
        self.clicked_text = Some(text);
        self
    }
    /// Sets the border of the button to be fat or double lined
    pub fn with_fat_border(mut self) -> Self {
        self.fat_border = true;
        self
    }
}

impl Widget for Button<'_> {
    fn style(&mut self, style: Style) {
        self.style = style;
    }

    fn inner(&self, area: Rect) -> Rect {
        let mut block = Block::new();
        if self.fat_border {
            block = block.with_fat_border();
        }
        block.inner(area)
    }

    fn render(&mut self, ctx: &mut crate::render::RenderContext, area: Rect) {
        if let Some(id) = &self.id {
            ctx.interactions.register(id, area);
        }
        let state = &self.state;
        let bg_style = {
            if state.clicked {
                self.clicked_style
            } else {
                self.style
            }
        };

        let mut outer_block = Block::new().with_bg_fill();
        if self.fat_border {
            outer_block = outer_block.with_fat_border();
        }
        outer_block.style(bg_style);
        outer_block.render(ctx, area);

        let inner_rect = outer_block.inner(area);
        if state.clicked {
            if let Some(text) = &mut self.clicked_text {
                text.style(bg_style);
                text.render(ctx, inner_rect);
                return;
            }
        }
        self.text.style(bg_style);
        self.text.render(ctx, inner_rect);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::{Colour, Normal, RenderContext};
    use crate::widgets::traits::Widget;

    #[test]
    fn test_button_render_normal() {
        let codex = Codex::new();
        let mut canvas = crate::render::Canvas::new(10, 3);
        let mut state = ButtonState { clicked: false };
        let mut button = Button::new("OK", &mut state, &codex);
        let area = Rect::new(0, 0, 10, 3);

        let mut interactions = crate::render::InteractionMap::new();
        let mut ctx = RenderContext::new(&mut canvas, &codex, &mut interactions);
        button.render(&mut ctx, area);

        assert_eq!(canvas.get_ccell(4, 1).char, codex.lookup('O'));
        assert_eq!(canvas.get_ccell(5, 1).char, codex.lookup('K'));
    }

    #[test]
    fn test_button_interaction_registration() {
        let codex = Codex::new();
        let mut canvas = crate::render::Canvas::new(10, 3);
        let mut state = ButtonState { clicked: false };
        let mut button = Button::new("OK", &mut state, &codex).with_id("test-btn");
        let area = Rect::new(5, 5, 10, 3);

        let mut interactions = crate::render::InteractionMap::new();
        let mut ctx = RenderContext::new(&mut canvas, &codex, &mut interactions);
        button.render(&mut ctx, area);

        assert_eq!(interactions.get_at(6, 6), Some(&"test-btn".to_string()));
    }
}
