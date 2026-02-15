//! The codex is responsible for mapping characters to glyphs.
//!
//! A glyph is a `u16` value that represents a character. The first 8 bits of the glyph are the
//! page ID, and the last 8 bits are the character ID.
//!
//! The codex is pre-loaded with a few default pages, such as `CP437` and `windows-1252`. You can
//! also register your own pages.
//!
//! # Custom Code Pages
//!
//! There are a total of 256 possible code pages. The first two (Index 0 and 1) are reserved for
//! windows-1252 and cp437 respectively.
//!
//! Each code page has 256 entries and each entry represents a character.
//! Every entry must have a displayed width of 1 and must be stored in valid utf-8.
//!
//! Talos builds a cache of the code pages and checks if a char is in a code page before
//! displaying it. Should a char not be in a code page, it will be displayed as a question mark.
//!
//! It is recommended that any custom code pages use an ID of `16` or higher.
//! The range of `0` to `15` is softly reserved for the default code pages.
//!
//! # Example
//!
//! ```rust
//! use talos::codex::Codex;
//!
//! let codex = Codex::new();
//! let glyph = codex.lookup('a');
//! let char = codex.resolve(glyph);
//! assert_eq!(char, "a");
//! ```

use std::collections::HashMap;

use crate::{
    codex::pages::{
        Page, REG_CP437, REG_UTF_GEOMETRIC_SHAPES, REG_UTF_MISC_TECHNICAL, REG_WIN_1252,
        UNKNOWN_CHAR, UNKNOWN_CHAR_GLYPH, pre_computed_char, validate_page,
    },
    error::{TalosError, TalosResult},
    render::Glyph,
};

/// Codex pages
pub mod pages;

/// The codex
///
/// A codex is a collection of pages
#[derive(Debug, Clone, Default)]
#[must_use]
pub struct Codex {
    pages: Vec<Option<&'static Page>>,
    reverse_map: HashMap<char, Glyph>,
}

impl Codex {
    /// Creates a new codex
    ///
    /// # Example
    /// ```rust
    /// use talos::codex::Codex;
    ///
    /// let codex = Codex::new();
    /// # assert!(true);
    /// ```
    pub fn new() -> Self {
        let mut codex = Codex {
            // Initialize all 256 pages with `None`. This does not expand the memory footprint
            // but saves from new allocations.
            pages: vec![None; 256],
            reverse_map: HashMap::new(),
        };

        codex.register_startup_page(REG_WIN_1252.0, REG_WIN_1252.1);
        codex.register_startup_page(REG_CP437.0, REG_CP437.1);
        codex.register_startup_page(REG_UTF_MISC_TECHNICAL.0, REG_UTF_MISC_TECHNICAL.1);
        codex.register_startup_page(REG_UTF_GEOMETRIC_SHAPES.0, REG_UTF_GEOMETRIC_SHAPES.1);

        codex
    }

    /// Internal function to register a new page.
    /// ONLY USE FOR ADDING DEFAULT PAGES
    ///
    /// Does not check if ID is in default range.
    ///
    /// # Arguments
    /// * `id` - The page id
    /// * `page` - The page
    ///
    /// # Panics
    /// Panics if the page is invalid or if the page id is already in use.
    /// This is fine as I guarantee that the default pages and their ID's are valid.
    /// I want to panic to check my work during development.
    fn register_startup_page(&mut self, id: u8, page: &'static Page) {
        assert!(self.pages[id as usize].is_none(), "{1}: {:?}", TalosError::PageIdInUse(id), "Default Page ID is guaranteed to be valid");
        if let Err(err) = validate_page(page) {
            panic!("{1}: {:?}", err, "Default Page contents are guaranteed to be valid")
        }

        self.pages[id as usize] = Some(page);

        self.update_cache(id, page);
    }

    /// Register a new page
    ///
    /// # Arguments
    /// * `id` - The page id
    /// * `page` - The page
    ///
    /// # Returns
    /// Returns `Ok(())` if the page was registered successfully.
    ///
    /// # Errors
    /// Returns an error if the page id is already in use or the page is invalid.
    /// Returns an error if the page id is inside the default pages range of 0-15.
    ///
    /// # Example
    /// ```rust
    /// use talos::codex::Codex;
    /// use talos::codex::pages::REG_UTF_GEOMETRIC_SHAPES;
    ///
    /// let mut codex = Codex::new();
    /// assert!(codex.register_page(16, REG_UTF_GEOMETRIC_SHAPES.1).is_ok());
    /// ```
    pub fn register_page(&mut self, id: u8, page: &'static Page) -> TalosResult<()> {
        if id < 16 {
            return Err(TalosError::DefaultPageId(id));
        }

        if self.pages[id as usize].is_some() {
            return Err(TalosError::PageIdInUse(id));
        }
        validate_page(page)?;

        self.pages[id as usize] = Some(page);

        self.update_cache(id, page);

        Ok(())
    }

    /// Resolve a glyph to a character
    ///
    /// # Arguments
    /// * `glyph` - The glyph
    ///
    /// # Returns
    /// Returns the character
    ///
    /// # Example
    /// ```rust
    /// use talos::codex::Codex;
    ///
    /// let codex = Codex::new();
    /// let glyph = codex.lookup('a');
    /// let char = codex.resolve(glyph);
    /// assert_eq!(char, "a");
    /// ```
    ///
    #[must_use]
    pub fn resolve(&self, glyph: Glyph) -> &str {
        if let Some(char) = pre_computed_char(glyph) {
            return char;
        }

        let page_id = (glyph >> 8) as usize;
        let char_id = (glyph & 0xFF) as usize;

        match self.pages.get(page_id) {
            Some(Some(page)) => page[char_id],
            _ => UNKNOWN_CHAR,
        }
    }

    /// Resolve a character to a glyph
    ///
    /// # Arguments
    /// * `ch` - The character
    ///
    /// # Returns
    /// Returns the glyph
    ///
    /// # Example
    /// ```rust
    /// use talos::codex::Codex;
    ///
    /// let codex = Codex::new();
    /// let glyph = codex.lookup('a');
    /// let char = codex.resolve(glyph);
    /// assert_eq!(char, "a");
    /// ```
    ///
    #[must_use]
    pub fn lookup(&self, ch: char) -> Glyph {
        if ch.is_ascii() {
            return ch as u16;
        }
        self.reverse_map
            .get(&ch)
            .copied()
            .unwrap_or(UNKNOWN_CHAR_GLYPH)
    }

    fn update_cache(&mut self, id: u8, page: &'static Page) {
        for (index, &symbol) in page.iter().enumerate() {
            if let Some(ch) = symbol.chars().next() {
                #[allow(clippy::cast_possible_truncation)] // Index must be less than 256
                let glyph_id = (u16::from(id) << 8) | (index as u16);
                self.reverse_map.entry(ch).or_insert(glyph_id);
            }
        }
    }
}
