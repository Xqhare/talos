use std::{
    fmt::Display,
    ops::{Add, Mul},
};

use crate::{
    content::text::TextContent,
    layout::Rect,
    render::{CCell, Canvas, Style},
    widgets::traits::Widget,
};

/// A widget that displays a number
///
/// Basic building block for your UI
///
/// In contrast to [Text](struct.Text.html), this widget does not support alignment
/// and does not support wrapping
///
/// # Example
/// ```rust,no_run
/// use talos::{Talos, widgets::Number};
///
/// let mut talos = Talos::builder().build().unwrap();
/// let (_, thoth) = talos.render_ctx();
/// let u_number = Number::new(&42, &thoth);
/// let i_number = Number::new(&-42, &thoth);
/// let f_number = Number::new(&3.14, &thoth);
/// # assert!(true);
/// ```
#[derive(Debug, Clone)]
pub struct Number {
    content: TextContent,
    style: Style,
}

impl Number {
    /// Create a new number widget
    ///
    /// # Arguments
    /// * `content` - The number to display
    /// * `thoth` - The thoth to use for glyph lookup
    ///
    /// `content` may be any float, signed or unsigned integer of any size
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::Number};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, thoth) = talos.render_ctx();
    /// let u_number = Number::new(&42, &thoth);
    /// let i_number = Number::new(&-42, &thoth);
    /// let f_number = Number::new(&3.14, &thoth);
    /// # assert!(true);
    /// ```
    pub fn new<N>(content: &N, thoth: &thoth::Thoth) -> Self
    where
        N: Add<Output = N> + Mul<Output = N> + Display,
    {
        let content = TextContent::new(format!("{content}"), thoth, None);
        Self {
            content,
            style: Style::default(),
        }
    }
}

impl Widget for Number {
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    fn render(&mut self, canvas: &mut Canvas, area: Rect, thoth: &thoth::Thoth) {
        self.content.set_wrap_limit(area.width, thoth);
        for (i, seq) in self.content.get_sequences().iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let x = area.x + i as u16;
            if x >= area.right() {
                break;
            }
            for (i, glyph) in seq.glyphs().iter().enumerate() {
                #[allow(clippy::cast_possible_truncation)]
                canvas.set_ccell(
                    x + i as u16,
                    area.y,
                    CCell {
                        char: *glyph,
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

    #[test]
    fn test_number_render() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(10, 1);
        let mut number = Number::new(&123, &thoth);
        let area = Rect::new(0, 0, 10, 1);

        number.render(&mut canvas, area, &thoth);

        assert_eq!(canvas.get_ccell(0, 0).char, crate::render::Grapheme::new("1"));
        assert_eq!(canvas.get_ccell(1, 0).char, crate::render::Grapheme::new("2"));
        assert_eq!(canvas.get_ccell(2, 0).char, crate::render::Grapheme::new("3"));
    }

    #[test]
    fn test_number_render_negative() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(10, 1);
        let mut number = Number::new(&-42, &thoth);
        let area = Rect::new(0, 0, 10, 1);

        number.render(&mut canvas, area, &thoth);

        assert_eq!(canvas.get_ccell(0, 0).char, crate::render::Grapheme::new("-"));
        assert_eq!(canvas.get_ccell(1, 0).char, crate::render::Grapheme::new("4"));
        assert_eq!(canvas.get_ccell(2, 0).char, crate::render::Grapheme::new("2"));
    }

    #[test]
    fn test_number_render_float() {
        let thoth = thoth::Thoth::new().unwrap();
        let mut canvas = Canvas::new(10, 1);
        let mut number = Number::new(&3.14, &thoth);
        let area = Rect::new(0, 0, 10, 1);

        number.render(&mut canvas, area, &thoth);

        assert_eq!(canvas.get_ccell(0, 0).char, crate::render::Grapheme::new("3"));
        assert_eq!(canvas.get_ccell(1, 0).char, crate::render::Grapheme::new("."));
        assert_eq!(canvas.get_ccell(2, 0).char, crate::render::Grapheme::new("1"));
        assert_eq!(canvas.get_ccell(3, 0).char, crate::render::Grapheme::new("4"));
    }
}
