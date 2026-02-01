
use crate::layout::Rect;
use crate::render::Canvas;

pub trait Widget {
    /// Renders the widget into the specified area
    ///
    /// The area is relative to the top left corner of the canvas (1,1)
    ///
    /// # Arguments
    /// * `canvas` - The canvas to render into
    /// * `area` - The area to render into. Only this area is available for the widget to draw
    /// into.
    fn render(&self, canvas: &mut Canvas, area: Rect);
}
