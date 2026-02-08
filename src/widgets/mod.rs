mod block;
pub use block::Block;

mod text;
pub use text::Text;

mod number;
pub use number::Number;

pub mod stateful;

/// Widget traits for creating custom widgets
pub mod traits {
    use crate::{
        codex::Codex,
        layout::Rect,
        render::{Canvas, Style},
    };

    /// Widget trait
    ///
    /// A widget is a building block for your UI
    ///
    /// If you want to create a custom widget, you need to implement this trait.
    /// To learn more, take a look at the widgets included in this crate and how they implement
    /// this trait.
    pub trait Widget {
        /// Renders the widget into the specified area
        ///
        /// The area is relative to the top left corner of the canvas (1,1)
        ///
        /// # Arguments
        /// * `canvas` - The canvas to render into
        /// * `area` - The area to render into. Only this area is available for the widget to draw
        ///   into.
        fn render(&mut self, canvas: &mut Canvas, area: Rect, codex: &Codex);
        /// Sets the primary style of the widget
        fn style(&mut self, style: Style);
    }
}
