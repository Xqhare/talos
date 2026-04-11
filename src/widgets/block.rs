#[cfg(test)]
mod tests {
    use super::*;
    use crate::codex::Codex;
    use crate::codex::pages::SPACE_GLYPH;
    use crate::render::Canvas;
    use crate::widgets::traits::Widget;

    #[test]
    fn block_render_borders() {
        // 1. Setup Headless Environment
        let mut canvas = Canvas::new(10, 10); // 10x10 virtual grid
        let codex = Codex::new(); // Standard glyph lookups (CP437/Windows1252)

        // 2. Define Area and Widget
        let area = Rect::new(0, 0, 5, 5); // 5x5 box at top-left
        let mut block = Block::new(); // Default block (simple borders)

        // 3. Render (No terminal needed!)
        block.render(&mut canvas, area, &codex);

        // 4. Verification
        // We expect the top-left corner (0,0) to be '┌'
        let tl_glyph = codex.lookup('┌');
        let tr_glyph = codex.lookup('┐');
        let bl_glyph = codex.lookup('└');
        let br_glyph = codex.lookup('┘');
        let h_glyph = codex.lookup('─');
        let v_glyph = codex.lookup('│');

        // Check Corners
        assert_eq!(
            canvas.get_ccell(0, 0).char,
            tl_glyph,
            "Top Left corner mismatch"
        );
        assert_eq!(
            canvas.get_ccell(4, 0).char,
            tr_glyph,
            "Top Right corner mismatch"
        );
        assert_eq!(
            canvas.get_ccell(0, 4).char,
            bl_glyph,
            "Bottom Left corner mismatch"
        );
        assert_eq!(
            canvas.get_ccell(4, 4).char,
            br_glyph,
            "Bottom Right corner mismatch"
        );

        // Check Edges (Sample points)
        assert_eq!(canvas.get_ccell(1, 0).char, h_glyph, "Top edge mismatch");
        assert_eq!(canvas.get_ccell(0, 1).char, v_glyph, "Left edge mismatch");

        // Check Interior (Should be empty/transparent for default block)
        // (1,1) is inside the block
        assert_eq!(
            canvas.get_ccell(1, 1).char,
            SPACE_GLYPH,
            "Inside should be empty (space)"
        );
    }

    #[test]
    fn block_clipping() {
        // Test that block doesn't draw outside its Rect
        let mut canvas = Canvas::new(10, 10);
        let codex = Codex::new();

        // Draw a block starting at (2,2) with size 3x3
        // It covers (2,2) to (4,4)
        let area = Rect::new(2, 2, 3, 3);
        let mut block = Block::new();
        block.render(&mut canvas, area, &codex);

        // Check a point OUTSIDE the rect (e.g., 0,0)
        assert_eq!(
            canvas.get_ccell(0, 0).char,
            SPACE_GLYPH,
            "Block drew outside bounds at (0,0)!"
        );

        // Check a point INSIDE the rect (e.g., 2,2 - Top Left Corner of block)
        let tl_glyph = codex.lookup('┌');
        assert_eq!(
            canvas.get_ccell(2, 2).char,
            tl_glyph,
            "Block failed to draw at correct offset (2,2)"
        );
    }

    #[test]
    fn block_title() {
        let mut canvas = Canvas::new(20, 5);
        let codex = Codex::new();

        let area = Rect::new(0, 0, 20, 5);
        let mut block = Block::new().title("Test", &codex, false);

        block.render(&mut canvas, area, &codex);

        // Title starts at x+1 (index 1)
        // "Test" -> 'T' at 1, 'e' at 2, 's' at 3, 't' at 4
        let t_glyph = codex.lookup('T');
        let e_glyph = codex.lookup('e');

        assert_eq!(canvas.get_ccell(1, 0).char, t_glyph, "Title 'T' missing");
        assert_eq!(canvas.get_ccell(2, 0).char, e_glyph, "Title 'e' missing");
    }
}
