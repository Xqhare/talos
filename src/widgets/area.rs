use crate::codex::Codex;
use crate::codex::pages::SPACE_GLYPH;
use crate::layout::Rect;
use crate::render::{CCell, Canvas, Style};
use crate::widgets::traits::Widget;

/// An area widget
///
/// The `Area` widget is a simple building block for your UI. It fills the entire assigned area
/// with the specified style. It does not draw any borders or titles.
///
/// # Example
///
/// ```rust,no_run
/// use talos::{
///     Talos,
///     layout::Rect,
///     render::{Colour, Normal, Style},
///     widgets::{Area, traits::Widget},
/// };
///
/// fn main() -> Result<(), talos::TalosError> {
///     let mut talos = Talos::builder().build()?;
///
///     talos.begin_frame();
///     let (canvas, codex) = talos.render_ctx();
///
///     let rect = Rect::new(0, 0, 20, 10);
///     let mut area = Area::new();
///     area.style(Style::builder()
///         .set_fg(Colour::Normal(Normal::White))
///         .set_bg(Colour::Normal(Normal::Black))
///         .build());
///
///     area.render(canvas, rect, codex);
///
///     talos.present()?;
///
///     Ok(())
/// }
/// ```
#[must_use]
pub struct Area {
    style: Style,
}

impl Default for Area {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Area {
    /// Creates a new, empty area
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            style: Style::default(),
        }
    }
}

impl Widget for Area {
    #[inline]
    fn style(&mut self, style: Style) {
        self.style = style;
    }

    #[inline]
    fn render(&mut self, canvas: &mut Canvas, area: Rect, _codex: &Codex) {
        let left = area.left();
        let right = area.right();
        let top = area.top();
        let bottom = area.bottom();

        for y in top..bottom {
            for x in left..right {
                canvas.set_ccell(
                    x,
                    y,
                    CCell {
                        char: SPACE_GLYPH,
                        style: self.style,
                    },
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result as TalosResult;
    use crate::codex::Codex;
    use crate::render::Canvas;

    #[test]
    fn area_builder() {
        use crate::render::{Colour, Normal};
        let style = Style::builder()
            .set_fg(Colour::Normal(Normal::Blue))
            .build();
        let area = Area::new().with_style(style);

        assert_eq!(area.style, style);
    }

    #[test]
    fn area_render_temporary() -> TalosResult<()> {
        use crate::render::{Colour, Normal};
        let mut canvas = Canvas::new(10, 10);
        let codex = Codex::new();
        let area_rect = Rect::new(2, 2, 3, 3);
        let style = Style::builder().set_fg(Colour::Normal(Normal::Red)).build();

        Area::new().with_style(style).render(&mut canvas, area_rect, &codex);

        // Check inside the area
        for y in 2..5 {
            for x in 2..5 {
                let cell = canvas.get_ccell(x, y);
                assert_eq!(cell.char, SPACE_GLYPH);
                assert_eq!(cell.style, style);
            }
        }

        Ok(())
    }
}
