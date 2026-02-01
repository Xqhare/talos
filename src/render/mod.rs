mod colour;
pub use colour::Colour;

mod codex;
pub use codex::Codex;

mod canvas;
pub use canvas::Canvas;

mod ccell;
pub use ccell::CCell;

mod style;
pub use style::Style;

pub type Glyph = u16;

use crate::layout::Rect;

pub trait Widget {
    /// Renders the widget into the specified area
    ///
    /// The area is relative to the top left corner of the canvas (1,1)
    ///
    /// # Arguments
    /// * `canvas` - The canvas to render into
    /// * `area` - The area to render into. Only this area is available for the widget to draw
    /// into.
    fn render(&self, canvas: &mut Canvas, area: Rect, codex: &Codex);
}
