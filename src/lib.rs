#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::all)]
#![warn(clippy::restriction)]
#![expect(
    clippy::mod_module_files,
    clippy::pub_use,
    clippy::single_char_lifetime_names,
    clippy::unseparated_literal_suffix,
    clippy::arithmetic_side_effects,
    clippy::non_ascii_literal,
    clippy::double_must_use,
    clippy::implicit_return,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    clippy::pub_with_shorthand,
    clippy::exhaustive_structs,
    clippy::float_arithmetic,
    clippy::integer_division,
    clippy::cognitive_complexity,
    clippy::integer_division_remainder_used,
    clippy::std_instead_of_core,
    clippy::missing_inline_in_public_items,
    clippy::unused_trait_names,
    clippy::else_if_without_else,
    clippy::shadow_unrelated,
    clippy::single_call_fn,
    clippy::str_to_string,
    clippy::question_mark_used,
    clippy::indexing_slicing,
    clippy::pattern_type_mismatch,
    clippy::arbitrary_source_item_ordering,
    clippy::doc_paragraphs_missing_punctuation,
    clippy::exhaustive_enums,
    clippy::min_ident_chars,
    clippy::missing_trait_methods,
    clippy::impl_trait_in_params,
    clippy::as_conversions,
    clippy::shadow_reuse,
    clippy::blanket_clippy_restriction_lints,
    clippy::doc_include_without_cfg,
    clippy::doc_markdown,
    clippy::absolute_paths,
    clippy::std_instead_of_alloc,
    clippy::cloned_instead_of_copied,
    clippy::missing_panics_doc,
    clippy::panic,
    clippy::uninlined_format_args,
    clippy::unwrap_or_default,
    clippy::let_underscore_untyped,
    clippy::let_underscore_must_use,
    clippy::if_then_some_else_none,
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block,
    clippy::module_name_repetitions,
    clippy::default_numeric_fallback,
    clippy::fn_to_numeric_cast_any,
    clippy::collapsible_if,
    clippy::similar_names,
    clippy::field_scoped_visibility_modifiers,
    dead_code,
    reason = "Ignored warnings"
)]
use std::io::Write;
use std::process::exit;
use core::mem::swap;

use codex::Codex;
use input::Parser;
use input::poll_input_bytes;
use ui::render::{CCell, Style};
use utils::constants::ansi::CLEAR_ALL;
use utils::constants::ansi::TO_TOP_LEFT;
use utils::constants::ansi::{BEGIN_SYNC_UPDATE, END_SYNC_UPDATE};
use utils::write_all_bytes;

mod builder;
pub use builder::{LayoutBuilder, ParserBuilder, TalosBuilder};
pub mod error;
pub use error::{Error as TalosError, Result as TalosResult};

/// Style and Layout Atlases
pub mod atlases;

use crate::backend::TerminalIO;
use crate::backend::sys::check_resize;
use crate::backend::sys::check_terminate;
use crate::input::Event;
use crate::ui::render::Canvas;
use crate::utils::move_render_cursor;

/// Backend abstraction.
mod backend;
/// Codex
pub mod codex;
/// Content abstraction.
mod content;
/// UI abstraction.
mod ui;

/// Input
pub mod input;
/// Layout abstraction.
pub use ui::layout;
/// Render abstraction.
pub use ui::render;
/// Utility functions.
mod utils;
/// Widgets
pub mod widgets;

/// Width of the terminal.
type Width = u16;
/// Height of the terminal.
type Height = u16;

/// The main struct of the library
///
/// # Example
/// ```rust,no_run
/// use talos::Talos;
///
/// let talos = Talos::builder().build();
/// assert!(talos.is_ok());
/// ```
///
/// For more information on building the struct, see [`TalosBuilder`](struct.TalosBuilder.html).
pub struct Talos {
    /// Terminal abstraction.
    terminal: TerminalIO,
    /// Drawing canvas.
    canvas: Canvas,
    /// Character mapping codex.
    codex: Codex,
    // Terminal Size
    /// Width, Height
    size: (Width, Height),
    /// Previous frame's buffer for optimized rendering.
    previous_buffer: Vec<CCell>,
    /// Buffer for storing the output bytes.
    output_buffer: Vec<u8>,
    // Input-Parser
    /// Input parser.
    parser: Parser,
}

/// Return type of `Talos::present`
pub enum Present {
    /// The terminal was resized, and because of that nothing was drawn
    Resized,
    /// The terminal was not resized - the canvas was drawn
    Presented,
}

impl Talos {
    /// Get a `TalosBuilder`
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let talos = Talos::builder()
    /// .with_cursor() // Show the Terminal cursor
    /// .with_alternate_screen() // Use the alternate screen
    /// .without_panic_handler() // Disable the panic handler
    /// .build();
    /// assert!(talos.is_ok());
    /// ```
    #[inline]
    #[must_use]
    pub fn builder() -> TalosBuilder {
        TalosBuilder::default()
    }

    /// Returns a mutable reference to the canvas
    ///
    /// Consider using `Talos::render_ctx` instead
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let canvas = talos.canvas_mut();
    /// # assert!(true);
    /// ```
    #[inline]
    #[must_use]
    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    /// Returns a tuple containing the canvas and the codex in the form `(canvas, codex)`.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let (canvas, codex) = talos.render_ctx();
    /// # assert!(true);
    /// ```
    #[inline]
    #[must_use]
    pub fn render_ctx(&mut self) -> (&mut Canvas, &Codex) {
        (&mut self.canvas, &self.codex)
    }

    /// Clear the canvas.
    /// Call at the beginning of every frame.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// talos.begin_frame();
    /// # assert!(true);
    /// ```
    #[inline]
    pub fn begin_frame(&mut self) {
        self.canvas.clear();
    }

    /// Present the canvas to the terminal
    /// The new size is stored in `self.size`.
    ///
    /// # Returns
    /// Returns whether the terminal was resized.
    /// If the terminal was resized, `present` will not draw anything to the terminal.
    /// Returns `Ok(Present::Resized)` if the terminal was resized.
    /// Returns `Ok(Present::Presented)` if the terminal was not resized.
    ///
    /// # Errors
    /// Returns an error if the terminal was terminated.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// assert!(talos.present().is_ok());
    /// ```
    #[inline]
    pub fn present(&mut self) -> TalosResult<Present> {
        let resized = self.handle_signals()?;
        if resized {
            return Ok(Present::Resized);
        }

        self.output_buffer.clear();

        write_all_bytes(&mut self.output_buffer, TO_TOP_LEFT.as_bytes())?;

        // Removing the next line will cause some weird side effects (Bleeding the `selected` style of
        // `List` to previous elements for example);
        // Doing this is not ideal (performance-wise) but it works
        Style::default().generate(&mut self.output_buffer);

        let mut prev_x_cell = u16::MAX;
        let mut current_terminal_style = Style::default();

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let buffer_index = usize::from(x).saturating_add(usize::from(y).saturating_mul(usize::from(self.size.0)));

                if self.canvas.buffer[buffer_index] != self.previous_buffer[buffer_index] {
                    let ccell = self.canvas.get_ccell(x, y);

                    if x.wrapping_sub(prev_x_cell) != 1 {
                        move_render_cursor(&mut self.output_buffer, x, y)?;
                    }

                    // Only generate style if it differs from the current terminal style
                    if ccell.style != current_terminal_style {
                        ccell
                            .style
                            .generate_diff(current_terminal_style, &mut self.output_buffer);
                        current_terminal_style = ccell.style;
                    }

                    write_all_bytes(
                        &mut self.output_buffer,
                        self.codex.resolve(ccell.char).as_bytes(),
                    )?;
                    prev_x_cell = x;
                }
            }
        }

        if self.handle_signals()? {
            // Resized! - Just show one blank frame - should be imperceivable anyways
            self.output_buffer.clear();
            return Ok(Present::Presented);
        }

        self.terminal
            .stdout()
            .write_all(BEGIN_SYNC_UPDATE.as_bytes())?;
        self.terminal.stdout().write_all(&self.output_buffer)?;
        self.terminal
            .stdout()
            .write_all(END_SYNC_UPDATE.as_bytes())?;
        self.terminal.stdout().flush()?;

        // Pointer swapping of the buffers
        swap(&mut self.previous_buffer, &mut self.canvas.buffer);

        Ok(Present::Presented)
    }

    /// Returns a mutable reference to the codex
    ///
    /// Consider using `Talos::render_ctx` instead
    /// A mutable reference is only needed to add more pages to the codex.
    ///
    /// For more on adding pages to the codex, see the documentation of `Codex`.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let codex = talos.codex_mut();
    /// # assert!(true);
    /// ```
    #[inline]
    #[must_use]
    pub fn codex_mut(&mut self) -> &mut Codex {
        &mut self.codex
    }

    /// Returns the codex
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// let codex = talos.codex();
    /// # assert!(true);
    /// ```
    #[inline]
    #[must_use]
    pub fn codex(&self) -> &Codex {
        &self.codex
    }

    /// Returns all input events since the last call.
    /// If there is no input, returns None.
    ///
    /// Eagerly evaluates all bytes read, and returns an `Event::Unknown` if
    /// the bytes cannot be parsed.
    ///
    /// # Errors
    /// Returns an error if the terminal was terminated.
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::Talos;
    /// use talos::input::{Event, KeyEvent, KeyCode};
    ///
    /// let mut talos = Talos::builder().build().unwrap();
    /// # let mut c = 0;
    /// let mut run = true;
    /// while run {
    ///     if let Ok(Some(events)) = talos.poll_input() {
    ///         for event in events {
    ///             match event {
    ///                 Event::KeyEvent(KeyEvent {
    ///                     code: KeyCode::Char('q'),
    ///                     ..
    ///                 }) => {
    ///                     run = false;
    ///                 }
    ///                 _ => {}
    ///             }
    ///         }
    ///     }
    ///     # c += 1;
    ///     # if c > 10 { break; }
    /// }
    /// # assert!(true);
    /// ```
    #[inline]
    pub fn poll_input(&mut self) -> TalosResult<Option<&[Event]>> {
        let _: bool = self.handle_signals()?;

        self.parser.event_buffer.clear();

        if let Some(bytes) = poll_input_bytes(
            &mut self.terminal.stdin(),
            &mut self.parser.poll_input_buffer,
            self.parser.max_poll_input_buffer,
            self.parser.buffer_linear_growth_step,
        )? {
            self.parser
                .parser
                .parse(bytes, &mut self.parser.event_buffer)?;
        } else {
            self.parser.parser.flush(&mut self.parser.event_buffer);
        }

        Ok(Some(self.parser.event_buffer.as_slice()))
    }

    /// Handles signals from the OS
    ///
    /// Returns `true` whether the terminal was resized
    /// Returns `false` if the terminal was not resized
    /// ENDS THE PROCESS if the terminate signal was received
    ///
    /// If the terminal was terminated, the terminal is restored and the process exits.
    ///
    /// # Errors
    /// Returns an error if internal I/O errors occur
    fn handle_signals(&mut self) -> TalosResult<bool> {
        if check_terminate() {
            // We need to shut down now - No state will be saved, just restore the terminal
            self.terminal.restore()?;
            exit(0);
        }

        if check_resize() {
            let (rows, cols) = TerminalIO::size()?;
            self.size = (cols, rows);

            self.canvas = Canvas::new(self.size.0, self.size.1);
            let len = usize::from(self.size.0).saturating_mul(usize::from(self.size.1));
            self.previous_buffer = vec![CCell::default(); len];
            self.output_buffer.clear();
            self.output_buffer.reserve(len.saturating_mul(10));
            write_all_bytes(&mut self.terminal.stdout(), CLEAR_ALL.as_bytes())?;
            self.terminal.stdout().flush()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
