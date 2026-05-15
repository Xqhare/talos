use std::{
    fmt::Display,
    ops::{Add, Mul},
};

use crate::{
    codex::Codex,
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
/// let (_, codex) = talos.render_ctx();
/// let u_number = Number::new(&42, &codex);
/// let i_number = Number::new(&-42, &codex);
/// let f_number = Number::new(&3.14, &codex);
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
    /// * `codex` - The codex to use for glyph lookup
    ///
    /// `content` may be any float, signed or unsigned integer of any size
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::{Talos, widgets::Number};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (_, codex) = talos.render_ctx();
    /// let u_number = Number::new(&42, &codex);
    /// let i_number = Number::new(&-42, &codex);
    /// let f_number = Number::new(&3.14, &codex);
    /// # assert!(true);
    /// ```
    pub fn new<N>(content: &N, codex: &Codex) -> Self
    where
        N: Add<Output = N> + Mul<Output = N> + Display,
    {
        let content = TextContent::new(format!("{content}"), codex, None);
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
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        self.content.set_wrap_limit(area.width, codex);
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
        let codex = Codex::new();
        let mut canvas = Canvas::new(10, 1);
        let mut number = Number::new(&123, &codex);
        let area = Rect::new(0, 0, 10, 1);

        number.render(&mut canvas, area, &codex);

        assert_eq!(canvas.get_ccell(0, 0).char, codex.lookup('1'));
        assert_eq!(canvas.get_ccell(1, 0).char, codex.lookup('2'));
        assert_eq!(canvas.get_ccell(2, 0).char, codex.lookup('3'));
    }

    #[test]
    fn test_number_render_negative() {
        let codex = Codex::new();
        let mut canvas = Canvas::new(10, 1);
        let mut number = Number::new(&-42, &codex);
        let area = Rect::new(0, 0, 10, 1);

        number.render(&mut canvas, area, &codex);

        assert_eq!(canvas.get_ccell(0, 0).char, codex.lookup('-'));
        assert_eq!(canvas.get_ccell(1, 0).char, codex.lookup('4'));
        assert_eq!(canvas.get_ccell(2, 0).char, codex.lookup('2'));
    }

    #[test]
    fn test_number_render_float() {
        let codex = Codex::new();
        let mut canvas = Canvas::new(10, 1);
        let mut number = Number::new(&3.14, &codex);
        let area = Rect::new(0, 0, 10, 1);

        number.render(&mut canvas, area, &codex);

        assert_eq!(canvas.get_ccell(0, 0).char, codex.lookup('3'));
        assert_eq!(canvas.get_ccell(1, 0).char, codex.lookup('.'));
        assert_eq!(canvas.get_ccell(2, 0).char, codex.lookup('1'));
        assert_eq!(canvas.get_ccell(3, 0).char, codex.lookup('4'));
    }
}
