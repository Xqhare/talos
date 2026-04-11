#[cfg(test)]
mod tests {
    use super::*;
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
    fn area_render_temporary() {
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
    }
}
