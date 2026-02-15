

use crate::codex::Codex;
use crate::content::text::{Sequence, TextContent};
use crate::layout::Rect;
use crate::render::{CCell, Canvas, Style};
use crate::widgets::traits::Widget;

/// A text widget
///
/// The `Text` widget is used to display text. It supports text wrapping, horizontal centering, and
/// vertical alignment.
///
/// # Example
///
/// ```rust
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
    content: TextContent,
    style: Style,
    align_center: bool,
    align_vertically: bool,
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
        let content = TextContent::new(content, codex, None);
        Self {
            content,
            style: Style::default(),
            align_center: false,
            align_vertically: false,
        }
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
        self.align_center = true;
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
        self.align_vertically = true;
        self
    }
}

impl Widget for Text {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        // Update wrap limit
        if let Some(wrap_limit) = self.content.get_wrap_limit() {
            if wrap_limit > area.width {
                self.content.set_wrap_limit(area.width, codex);
            }
        } else {
            // Assume first run - so set up wrap limit
            self.content.set_wrap_limit(area.width, codex);
        }

        // After setup, each Sequence should be guaranteed to be at most
        // `area.width` wide
        let sequences: &[Sequence] = self.content.get_sequences();
        //let mut out = Vec::with_capacity(sequences.len());

        let top = if self.align_vertically {
            #[allow(clippy::cast_possible_truncation)]
            if (sequences.len() as u16) < area.height {
                let rest = area.height - sequences.len() as u16;
                if rest.is_multiple_of(2) {
                    (rest / 2) + area.top()
                } else {
                    (rest / 2 + 1) + area.top()
                }
            } else {
                area.top()
            }
        } else {
            area.top()
        };

        for (i, seq) in sequences.iter().enumerate() {
            if seq.width() > area.width {
                // Hard clip
                break;
            }

            let left_margin = if self.align_center {
                let rest_width = area.width - seq.width();
                if rest_width.is_multiple_of(2) {
                    rest_width / 2
                } else {
                    rest_width / 2 + 1
                }
            } else {
                0
            };

            #[allow(clippy::cast_possible_truncation)]
            let y = top + i as u16;
            let mut x = area.left() + left_margin;

            #[allow(clippy::cast_possible_truncation)]
            if x + seq.glyphs().len() as u16 > area.right() {
                x = area.left();
            }
            if y >= area.bottom() {
                break;
            }
            for glyph in seq.glyphs() {
                if x >= area.right() {
                    // Hard clip the edge of the provided area - even if we drop parts of a line
                    break;
                }

                canvas.set_ccell(
                    x,
                    y,
                    CCell {
                        char: *glyph,
                        style: self.style,
                    },
                );

                x += 1;
            }
        }
    }
}
