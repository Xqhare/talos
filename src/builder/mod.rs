//! This module contains the builders for the main components of the Talos library.
//!
//! The builders are used to configure and create instances of the main components, such as `Talos`,
//! `Parser`, and `Layout`.

mod talos_builder;
pub use talos_builder::TalosBuilder;

mod parser_builder;
pub use parser_builder::ParserBuilder;

mod layout_builder;
pub use layout_builder::LayoutBuilder;
