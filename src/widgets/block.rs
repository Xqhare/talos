use crate::codex::Codex;
use crate::content::title::{TitleContents, TitlePosition};
use crate::layout::Rect;
use crate::render::{CCell, Canvas, Style};
use crate::widgets::traits::Widget;

#[must_use]
pub struct Block {
    title: TitleContents,
    style: Style,
    fill_bg: bool,
    beautfy_border_breaks: bool,
    fat_border: bool,
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl Block {
    pub fn new() -> Self {
        Self {
            title: TitleContents::default(),
            style: Style::default(),
            fill_bg: false,
            beautfy_border_breaks: false,
            fat_border: false,
        }
    }

    pub fn with_fat_border(mut self) -> Self {
        self.fat_border = true;
        self
    }

    /// Sets the title of the block
    ///
    /// By default, the block has no title
    ///
    /// # Arguments
    /// * `title` - The title of the block
    /// * `centered` - Whether the title should be centered
    ///
    /// if `centered` is false the title will be on the top left corner
    pub fn title(mut self, title: impl Into<String>, codex: &Codex, centered: bool) -> Self {
        if centered {
            self.title
                .set_position(TitlePosition::TopCenter, title, codex);
        } else {
            self.title
                .set_position(TitlePosition::TopLeft, title, codex);
        }
        self
    }

    pub fn top_subtitle(mut self, subtitle: impl Into<String>, codex: &Codex) -> Self {
        self.title
            .set_position(TitlePosition::TopRight, subtitle, codex);
        self
    }

    pub fn bottom_right_subtitle(mut self, subtitle: impl Into<String>, codex: &Codex) -> Self {
        self.title
            .set_position(TitlePosition::BottomRight, subtitle, codex);
        self
    }

    pub fn bottom_center_subtitle(mut self, subtitle: impl Into<String>, codex: &Codex) -> Self {
        self.title
            .set_position(TitlePosition::BottomCenter, subtitle, codex);
        self
    }

    pub fn bottom_left_subtitle(mut self, subtitle: impl Into<String>, codex: &Codex) -> Self {
        self.title
            .set_position(TitlePosition::BottomLeft, subtitle, codex);
        self
    }

    pub fn with_bg_fill(mut self) -> Self {
        self.fill_bg = true;
        self
    }

    pub fn with_beautify_border_breaks(mut self) -> Self {
        self.beautfy_border_breaks = true;
        self
    }

    /// Returns the inner area inside the block's borders.
    /// Useful for rendering child widgets inside this block.
    #[must_use]
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
    fn style(&mut self, style: Style) {
        self.style = style;
    }
    #[allow(clippy::too_many_lines)]
    fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex) {
        if area.width < 2 || area.height < 2 {
            return;
        }

        let left = area.left();
        let right = area.right() - 1;
        let top = area.top();
        let bottom = area.bottom() - 1;

        let h_bar = if self.fat_border {
            codex.lookup('═')
        } else {
            codex.lookup('─')
        };
        let v_bar = if self.fat_border {
            codex.lookup('║')
        } else {
            codex.lookup('│')
        };
        let tl = if self.fat_border {
            codex.lookup('╔')
        } else {
            codex.lookup('┌')
        };
        let tr = if self.fat_border {
            codex.lookup('╗')
        } else {
            codex.lookup('┐')
        };
        let bl = if self.fat_border {
            codex.lookup('╚')
        } else {
            codex.lookup('└')
        };
        let br = if self.fat_border {
            codex.lookup('╝')
        } else {
            codex.lookup('┘')
        };

        // Draw Corners
        canvas.set_ccell(
            left,
            top,
            CCell {
                char: tl,
                style: self.style,
            },
        );
        canvas.set_ccell(
            right,
            top,
            CCell {
                char: tr,
                style: self.style,
            },
        );
        canvas.set_ccell(
            left,
            bottom,
            CCell {
                char: bl,
                style: self.style,
            },
        );
        canvas.set_ccell(
            right,
            bottom,
            CCell {
                char: br,
                style: self.style,
            },
        );

        // Draw Edges (Using the cached glyphs)
        for x in (left + 1)..right {
            canvas.set_ccell(
                x,
                top,
                CCell {
                    char: h_bar,
                    style: self.style,
                },
            );
            canvas.set_ccell(
                x,
                bottom,
                CCell {
                    char: h_bar,
                    style: self.style,
                },
            );
        }

        for y in (top + 1)..bottom {
            canvas.set_ccell(
                left,
                y,
                CCell {
                    char: v_bar,
                    style: self.style,
                },
            );
            canvas.set_ccell(
                right,
                y,
                CCell {
                    char: v_bar,
                    style: self.style,
                },
            );
        }

        // Fill Background (Optional)
        if self.fill_bg {
            for y in (top + 1)..bottom {
                for x in (left + 1)..right {
                    canvas.set_ccell(
                        x,
                        y,
                        CCell {
                            char: 0x0020,
                            style: self.style,
                        },
                    );
                }
            }
        }

        // Draw Title

        // DIFFERENT BOTTOM STYLES
        let left_break = if self.beautfy_border_breaks {
            if self.fat_border {
                codex.lookup('╗')
            } else {
                codex.lookup('┐')
            }
        } else {
            0
        };
        let right_break = if self.beautfy_border_breaks {
            if self.fat_border {
                codex.lookup('╔')
            } else {
                codex.lookup('┌')
            }
        } else {
            0
        };

        if let Some(title) = &self.title.get_position(TitlePosition::TopLeft) {
            let mut start_x = left + 1;
            if self.beautfy_border_breaks {
                canvas.set_ccell(
                    start_x,
                    top,
                    CCell {
                        char: left_break,
                        style: self.style,
                    },
                );
                start_x += 1;
            }
            #[allow(clippy::cast_possible_truncation)]
            for (i, glyph) in title.iter().enumerate() {
                canvas.set_ccell(
                    start_x + i as u16,
                    top,
                    CCell {
                        char: *glyph,
                        style: self.style,
                    },
                );
            }
            #[allow(clippy::cast_possible_truncation)]
            if self.beautfy_border_breaks {
                canvas.set_ccell(
                    start_x + title.len() as u16,
                    top,
                    CCell {
                        char: right_break,
                        style: self.style,
                    },
                );
            }
        } else if let Some(title) = &self.title.get_position(TitlePosition::TopCenter) {
            let start_x = (area.width as usize / 2).saturating_sub(title.len() / 2);
            #[allow(clippy::cast_possible_truncation)]
            let mut start_x = left + start_x as u16 + 1;
            if self.beautfy_border_breaks {
                canvas.set_ccell(
                    start_x,
                    top,
                    CCell {
                        char: left_break,
                        style: self.style,
                    },
                );
                start_x += 1;
            }
            #[allow(clippy::cast_possible_truncation)]
            for (i, glyph) in title.iter().enumerate() {
                canvas.set_ccell(
                    start_x + i as u16,
                    top,
                    CCell {
                        char: *glyph,
                        style: self.style,
                    },
                );
            }
            #[allow(clippy::cast_possible_truncation)]
            if self.beautfy_border_breaks {
                canvas.set_ccell(
                    start_x + title.len() as u16,
                    top,
                    CCell {
                        char: right_break,
                        style: self.style,
                    },
                );
            }
        }

        // Draw subtitles if set
        if let Some(top_subtitle) = &self.title.get_position(TitlePosition::TopRight) {
            #[allow(clippy::cast_possible_truncation)]
            let mut start_x = right - top_subtitle.len() as u16 - 2;
            if self.beautfy_border_breaks {
                canvas.set_ccell(
                    start_x,
                    top,
                    CCell {
                        char: left_break,
                        style: self.style,
                    },
                );
                start_x += 1;
            }
            #[allow(clippy::cast_possible_truncation)]
            for (i, glyph) in top_subtitle.iter().enumerate() {
                canvas.set_ccell(
                    start_x + i as u16,
                    top,
                    CCell {
                        char: *glyph,
                        style: self.style,
                    },
                );
            }
            #[allow(clippy::cast_possible_truncation)]
            if self.beautfy_border_breaks {
                canvas.set_ccell(
                    start_x + top_subtitle.len() as u16,
                    top,
                    CCell {
                        char: right_break,
                        style: self.style,
                    },
                );
            }
        }

        let left_break = if self.beautfy_border_breaks {
            if self.fat_border {
                codex.lookup('╝')
            } else {
                codex.lookup('┘')
            }
        } else {
            0
        };
        let right_break = if self.beautfy_border_breaks {
            if self.fat_border {
                codex.lookup('╚')
            } else {
                codex.lookup('└')
            }
        } else {
            0
        };

        if let Some(bottom_left_subtitle) = &self.title.get_position(TitlePosition::BottomLeft) {
            let mut start_x = left + 1;
            if self.beautfy_border_breaks {
                canvas.set_ccell(
                    start_x,
                    bottom,
                    CCell {
                        char: left_break,
                        style: self.style,
                    },
                );
                start_x += 1;
            }
            #[allow(clippy::cast_possible_truncation)]
            for (i, glyph) in bottom_left_subtitle.iter().enumerate() {
                canvas.set_ccell(
                    start_x + i as u16,
                    bottom,
                    CCell {
                        char: *glyph,
                        style: self.style,
                    },
                );
            }
            #[allow(clippy::cast_possible_truncation)]
            if self.beautfy_border_breaks {
                canvas.set_ccell(
                    start_x + bottom_left_subtitle.len() as u16,
                    bottom,
                    CCell {
                        char: right_break,
                        style: self.style,
                    },
                );
            }
        }

        if let Some(bottom_center_subtitle) = &self.title.get_position(TitlePosition::BottomCenter)
        {
            let start_x =
                (area.width as usize / 2).saturating_sub(bottom_center_subtitle.len() / 2);
            #[allow(clippy::cast_possible_truncation)]
            let mut start_x = left + start_x as u16 + 1;
            if self.beautfy_border_breaks {
                canvas.set_ccell(
                    start_x,
                    bottom,
                    CCell {
                        char: left_break,
                        style: self.style,
                    },
                );
                start_x += 1;
            }
            #[allow(clippy::cast_possible_truncation)]
            for (i, glyph) in bottom_center_subtitle.iter().enumerate() {
                canvas.set_ccell(
                    start_x + i as u16,
                    bottom,
                    CCell {
                        char: *glyph,
                        style: self.style,
                    },
                );
            }
            #[allow(clippy::cast_possible_truncation)]
            if self.beautfy_border_breaks {
                canvas.set_ccell(
                    start_x + bottom_center_subtitle.len() as u16,
                    bottom,
                    CCell {
                        char: right_break,
                        style: self.style,
                    },
                );
            }
        }

        if let Some(bottom_right_subtitle) = &self.title.get_position(TitlePosition::BottomRight) {
            #[allow(clippy::cast_possible_truncation)]
            let mut start_x = right - bottom_right_subtitle.len() as u16 - 2;
            if self.beautfy_border_breaks {
                canvas.set_ccell(
                    start_x,
                    bottom,
                    CCell {
                        char: left_break,
                        style: self.style,
                    },
                );
                start_x += 1;
            }
            #[allow(clippy::cast_possible_truncation)]
            for (i, glyph) in bottom_right_subtitle.iter().enumerate() {
                canvas.set_ccell(
                    start_x + i as u16,
                    bottom,
                    CCell {
                        char: *glyph,
                        style: self.style,
                    },
                );
            }
            #[allow(clippy::cast_possible_truncation)]
            if self.beautfy_border_breaks {
                canvas.set_ccell(
                    start_x + bottom_right_subtitle.len() as u16,
                    bottom,
                    CCell {
                        char: right_break,
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
    use crate::TalosResult;
    use crate::codex::Codex;
    use crate::codex::pages::SPACE_GLYPH;
    use crate::render::Canvas;

    #[test]
    fn test_block_render_borders() -> TalosResult<()> {
        // 1. Setup Headless Environment
        let mut canvas = Canvas::new(10, 10); // 10x10 virtual grid
        let codex = Codex::new(); // Standard glyph lookups (CP437/Windows1252)

        // 2. Define Area and Widget
        let area = Rect::new(0, 0, 5, 5); // 5x5 box at top-left
        let mut block = Block::new(); // Default block (simple borders)

        // 3. Render (No terminal needed!)
        // Note: We need to import the Widget trait in the test module or parent to call .render()
        use crate::widgets::traits::Widget;
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

        Ok(())
    }

    #[test]
    fn test_block_clipping() -> TalosResult<()> {
        // Test that block doesn't draw outside its Rect
        let mut canvas = Canvas::new(10, 10);
        let codex = Codex::new();
        use crate::widgets::traits::Widget;

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

        Ok(())
    }

    #[test]
    fn test_block_title() -> TalosResult<()> {
        let mut canvas = Canvas::new(20, 5);
        let codex = Codex::new();
        use crate::widgets::traits::Widget;

        let area = Rect::new(0, 0, 20, 5);
        let mut block = Block::new().title("Test", &codex, false);

        block.render(&mut canvas, area, &codex);

        // Title starts at x+1 (index 1)
        // "Test" -> 'T' at 1, 'e' at 2, 's' at 3, 't' at 4
        let t_glyph = codex.lookup('T');
        let e_glyph = codex.lookup('e');

        assert_eq!(canvas.get_ccell(1, 0).char, t_glyph, "Title 'T' missing");
        assert_eq!(canvas.get_ccell(2, 0).char, e_glyph, "Title 'e' missing");

        Ok(())
    }
}
