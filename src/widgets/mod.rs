
mod block;
pub use block::Block;

mod text;
pub use text::Text;

mod list;
pub use list::List;

pub mod traits {
    use crate::{codex::Codex, layout::Rect, render::Canvas};
    pub trait Widget {
        /// Renders the widget into the specified area
        ///
        /// The area is relative to the top left corner of the canvas (1,1)
        ///
        /// # Arguments
        /// * `canvas` - The canvas to render into
        /// * `area` - The area to render into. Only this area is available for the widget to draw
        /// into.
        fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex);
    }
}
