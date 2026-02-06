mod colour;
pub use colour::{Bright, Colour, ColourMode, Extended, Normal, TrueColour};

mod canvas;
pub use canvas::Canvas;

mod ccell;
pub use ccell::CCell;

mod style;
pub use style::Style;

pub type Glyph = u16;
