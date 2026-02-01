use crate::layout::Rect;
use crate::render::traits::Widget;
use crate::render::{CCell, Canvas, Codex, Style};

pub struct Block {
    title: Option<String>,
    style: Style,
    fill_bg: bool,
}

impl Block {
    pub fn new() -> Self {
        Self {
            title: None,
            style: Style::default(),
            fill_bg: false,
        }
    }

    /// Sets the title of the block
    ///
    /// By default, the block has no title
    /// If `""` is passed, no title is rendered
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn with_bg_fill(mut self) -> Self {
        self.fill_bg = true;
        self
    }

    /// Returns the inner area inside the block's borders.
    /// Useful for rendering child widgets inside this block.
    pub fn inner(&self, area: Rect) -> Rect {
        if area.width < 2 || area.height < 2 {
            return Rect::default();
        }
        Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        }
    }
}

impl Widget for Block {
    fn render(&self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        if area.width < 2 || area.height < 2 { return; }

        let left = area.left();
        let right = area.right() - 1;
        let top = area.top();
        let bottom = area.bottom() - 1;

        let h_bar = codex.lookup('─');
        let v_bar = codex.lookup('│');
        let tl = codex.lookup('┌');
        let tr = codex.lookup('┐');
        let bl = codex.lookup('└');
        let br = codex.lookup('┘');

        // Draw Corners
        canvas.set_ccell(left, top, CCell { char: tl, style: self.style });
        canvas.set_ccell(right, top, CCell { char: tr, style: self.style });
        canvas.set_ccell(left, bottom, CCell { char: bl, style: self.style });
        canvas.set_ccell(right, bottom, CCell { char: br, style: self.style });

        // Draw Edges (Using the cached glyphs)
        for x in (left + 1)..right {
            canvas.set_ccell(x, top, CCell { char: h_bar, style: self.style });
            canvas.set_ccell(x, bottom, CCell { char: h_bar, style: self.style });
        }

        for y in (top + 1)..bottom {
            canvas.set_ccell(left, y, CCell { char: v_bar, style: self.style });
            canvas.set_ccell(right, y, CCell { char: v_bar, style: self.style });
        }

        // Fill Background (Optional)
        if self.fill_bg {
            for y in (top + 1)..bottom {
                for x in (left + 1)..right {
                    canvas.set_ccell(x, y, CCell { char: 0x0020, style: self.style });
                }
            }
        }

        // Draw Title
        if let Some(title) = &self.title {
            let max_len = (area.width as usize).saturating_sub(2); // Minus borders
            if max_len > 0 {
                let render_title = if title.len() > max_len {
                    &title[..max_len]
                } else {
                    &title
                };

                let start_x = left + 1;
                for (i, ch) in render_title.chars().enumerate() {
                    let glyph = codex.lookup(ch); 
                    canvas.set_ccell(start_x + i as u16, top, CCell { char: glyph, style: self.style });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::{Canvas, Codex};
    use crate::utils::constants::pages::SPACE_GLYPH;

    #[test]
    fn test_block_render_borders() {
        // 1. Setup Headless Environment
        let mut canvas = Canvas::new(10, 10); // 10x10 virtual grid
        let codex = Codex::new(); // Standard glyph lookups (CP437/Windows1252)
        
        // 2. Define Area and Widget
        let area = Rect::new(0, 0, 5, 5); // 5x5 box at top-left
        let block = Block::new(); // Default block (simple borders)

        // 3. Render (No terminal needed!)
        // Note: We need to import the Widget trait in the test module or parent to call .render()
        use crate::render::traits::Widget; 
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
        assert_eq!(canvas.get_ccell(0, 0).char, tl_glyph, "Top Left corner mismatch");
        assert_eq!(canvas.get_ccell(4, 0).char, tr_glyph, "Top Right corner mismatch");
        assert_eq!(canvas.get_ccell(0, 4).char, bl_glyph, "Bottom Left corner mismatch");
        assert_eq!(canvas.get_ccell(4, 4).char, br_glyph, "Bottom Right corner mismatch");

        // Check Edges (Sample points)
        assert_eq!(canvas.get_ccell(1, 0).char, h_glyph, "Top edge mismatch");
        assert_eq!(canvas.get_ccell(0, 1).char, v_glyph, "Left edge mismatch");
        
        // Check Interior (Should be empty/transparent for default block)
        // (1,1) is inside the block
        assert_eq!(canvas.get_ccell(1, 1).char, SPACE_GLYPH, "Inside should be empty (space)");
    }

    #[test]
    fn test_block_clipping() {
        // Test that block doesn't draw outside its Rect
        let mut canvas = Canvas::new(10, 10);
        let codex = Codex::new();
        use crate::render::traits::Widget;
        
        // Draw a block starting at (2,2) with size 3x3
        // It covers (2,2) to (4,4)
        let area = Rect::new(2, 2, 3, 3); 
        let block = Block::new();
        block.render(&mut canvas, area, &codex);

        // Check a point OUTSIDE the rect (e.g., 0,0)
        assert_eq!(canvas.get_ccell(0, 0).char, SPACE_GLYPH, "Block drew outside bounds at (0,0)!");
        
        // Check a point INSIDE the rect (e.g., 2,2 - Top Left Corner of block)
        let tl_glyph = codex.lookup('┌');
        assert_eq!(canvas.get_ccell(2, 2).char, tl_glyph, "Block failed to draw at correct offset (2,2)");
    }
    
    #[test]
    fn test_block_title() {
        let mut canvas = Canvas::new(20, 5);
        let codex = Codex::new();
        use crate::render::traits::Widget;

        let area = Rect::new(0, 0, 20, 5);
        let block = Block::new().title("Test");
        
        block.render(&mut canvas, area, &codex);

        // Title starts at x+1 (index 1)
        // "Test" -> 'T' at 1, 'e' at 2, 's' at 3, 't' at 4
        let t_glyph = codex.lookup('T');
        let e_glyph = codex.lookup('e');

        assert_eq!(canvas.get_ccell(1, 0).char, t_glyph, "Title 'T' missing");
        assert_eq!(canvas.get_ccell(2, 0).char, e_glyph, "Title 'e' missing");
    }
}
