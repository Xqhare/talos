use crate::layout::Rect;
use crate::render::{Canvas, Style};
use crate::widgets::internal_text::InternalText;
use crate::widgets::traits::Widget;

/// A text widget
///
/// The `Text` widget is used to display text. It supports text wrapping, horizontal centering, and
/// vertical alignment.
///
/// # Example
///
/// ```rust,no_run
/// use talos::{
///     Talos,
///     layout::Rect,
///     render::{Colour, Normal, Style},
///     widgets::{Text, traits::Widget},
/// };
///
/// fn main() -> Result<(), talos::TalosError> {
///     let mut talos = Talos::builder().build()?;
///
///     talos.begin_frame();
///     let (canvas, thoth) = talos.render_ctx();
///
///     let rect = Rect::new(0, 0, 20, 10);
///     let mut text = Text::new("Hello, world!", thoth)
///         .align_center()
///         .align_vertically();
///
///     let style = Style::builder()
///         .set_fg(Colour::Normal(Normal::White))
///         .set_bg(Colour::Normal(Normal::Black))
///         .build();
///
///     text.style(style);
///     text.render(canvas, rect, thoth);
///
///     talos.present()?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Default, Clone)]
#[must_use]
pub struct Text {
    content: InternalText,
}

impl From<InternalText> for Text {
    fn from(content: InternalText) -> Self {
        Self { content }
    }
}

impl Into<InternalText> for Text {
    fn into(self) -> InternalText {
        self.content
    }
}

impl Text {
    /// Create a new text widget from a string
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::Text};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, thoth) = talos.render_ctx();
    /// let text = Text::new("Hello, world!", &thoth);
    /// # assert!(true);
    /// ```
    pub fn new(content: impl Into<String>, thoth: &thoth::Thoth) -> Self {
        Self {
            content: InternalText::new(content, thoth),
        }
    }

    /// Update the text content.
    pub fn set_content(&mut self, content: impl Into<String>, thoth: &thoth::Thoth) {
        self.content.set_content(content, thoth);
    }

    /// Get the text content
    pub fn get_content(&self) -> &str {
        &self.content.get_content()
    }

    pub(super) fn get_content_internal(&self) -> &InternalText {
        &self.content
    }

    pub(super) fn get_mut_content(&mut self) -> &mut InternalText {
        &mut self.content
    }

    /// Align the text to the center of the rendered area
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::Text};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, thoth) = talos.render_ctx();
    /// let text = Text::new("Hello, world!", &thoth).align_center();
    /// # assert!(true);
    /// ```
    pub fn align_center(mut self) -> Self {
        self.content = self.content.align_center();
        self
    }

    /// Align the text vertically
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::Text};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, thoth) = talos.render_ctx();
    /// let text = Text::new("Hello, world!", &thoth).align_vertically();
    /// # assert!(true);
    /// ```
    pub fn align_vertically(mut self) -> Self {
        self.content = self.content.align_vertically();
        self
    }

    /// Get the length of the text
    ///
    /// This is the length of the text in rendered Glyphs.
    ///
    /// To determine the maximum rendered width, use [`Text::get_rendered_width`]
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Get the rendered width of the text
    ///
    /// This is the width of the text in Glyphs per line.
    ///
    /// To determine the length of the text, use [`Text::len`]
    pub fn get_rendered_width(&self) -> u16 {
        self.content.get_rendered_width()
    }
}

impl Widget for Text {
    fn style(&mut self, style: Style) {
        self.content.style(style);
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, thoth: &thoth::Thoth) {
        self.content.render(canvas, area, thoth);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::{Colour, Normal};

    #[test]
    fn test_text_render_basic() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(20, 5);
        let mut text = Text::new("Hello", &thoth);
        let area = Rect::new(0, 0, 20, 5);

        text.render(&mut canvas, area, &thoth);

        // Check first 5 characters
        let h = crate::render::Grapheme::new("H");
        let e = crate::render::Grapheme::new("e");
        let l = crate::render::Grapheme::new("l");
        let o = crate::render::Grapheme::new("o");

        assert_eq!(canvas.get_ccell(0, 0).char, h);
        assert_eq!(canvas.get_ccell(1, 0).char, e);
        assert_eq!(canvas.get_ccell(2, 0).char, l);
        assert_eq!(canvas.get_ccell(3, 0).char, l);
        assert_eq!(canvas.get_ccell(4, 0).char, o);
    }

    #[test]
    fn test_text_render_align_center() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(10, 1);
        // "ABC" is 3 chars. In width 10, (10-3)/2 = 3.5 -> 4 margin.
        // Wait, InternalText says:
        // if rest_width.is_multiple_of(2) { rest_width / 2 } else { rest_width / 2 + 1 }
        // (10-3) = 7. 7/2 + 1 = 4.
        // So it starts at x=4.
        let mut text = Text::new("ABC", &thoth).align_center();
        let area = Rect::new(0, 0, 10, 1);

        text.render(&mut canvas, area, &thoth);

        assert_eq!(canvas.get_ccell(4, 0).char, crate::render::Grapheme::new("A"));
        assert_eq!(canvas.get_ccell(5, 0).char, crate::render::Grapheme::new("B"));
        assert_eq!(canvas.get_ccell(6, 0).char, crate::render::Grapheme::new("C"));
    }

    #[test]
    fn test_text_render_align_vertically() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(1, 5);
        // "A" is 1 line. In height 5, (5-1)/2 = 2.
        // InternalText says:
        // if rest.is_multiple_of(2) { (rest / 2) + area.top() } else { (rest / 2 + 1) + area.top() }
        // (5-1) = 4. 4/2 + 0 = 2.
        let mut text = Text::new("A", &thoth).align_vertically();
        let area = Rect::new(0, 0, 1, 5);

        text.render(&mut canvas, area, &thoth);

        assert_eq!(canvas.get_ccell(0, 2).char, crate::render::Grapheme::new("A"));
    }

    #[test]
    fn test_text_style() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(5, 1);
        let style = Style::builder().set_fg(Colour::Normal(Normal::Red)).build();
        let mut text = Text::new("A", &thoth);
        text.style(style);
        let area = Rect::new(0, 0, 5, 1);

        text.render(&mut canvas, area, &thoth);

        assert_eq!(canvas.get_ccell(0, 0).style, style);
    }
}
