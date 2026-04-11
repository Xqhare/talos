use crate::codex::Codex;
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
///     let (canvas, codex) = talos.render_ctx();
///
///     let rect = Rect::new(0, 0, 20, 10);
///     let mut text = Text::new("Hello, world!", codex)
///         .align_center()
///         .align_vertically();
///
///     let style = Style::builder()
///         .set_fg(Colour::Normal(Normal::White))
///         .set_bg(Colour::Normal(Normal::Black))
///         .build();
///
///     text.style(style);
///     text.render(canvas, rect, codex);
///
///     talos.present()?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
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
    /// let (_, codex) = talos.render_ctx();
    /// let text = Text::new("Hello, world!", &codex);
    /// # assert!(true);
    /// ```
    pub fn new(content: impl Into<String>, codex: &Codex) -> Self {
        Self {
            content: InternalText::new(content, codex),
        }
    }

    /// Update the text content.
    pub fn set_content(&mut self, content: impl Into<String>, codex: &Codex) {
        self.content.set_content(content, codex);
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
    /// let (_, codex) = talos.render_ctx();
    /// let text = Text::new("Hello, world!", &codex).align_center();
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
    /// let (_, codex) = talos.render_ctx();
    /// let text = Text::new("Hello, world!", &codex).align_vertically();
    /// # assert!(true);
    /// ```
    pub fn align_vertically(mut self) -> Self {
        self.content = self.content.align_vertically();
        self
    }
}

impl Widget for Text {
    fn style(&mut self, style: Style) {
        self.content.style(style);
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        self.content.render(canvas, area, codex);
    }
}
