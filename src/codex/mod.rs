use std::collections::HashMap;

use crate::{
    codex::pages::{
        Page, REG_CP437, REG_UTF_GEOMETRIC_SHAPES, REG_UTF_MISC_TECHNICAL, REG_WIN_1252,
        UNKNOWN_CHAR, UNKNOWN_CHAR_GLYPH, pre_computed_char, validate_page,
    },
    error::{TalosError, TalosResult},
    render::Glyph,
};

pub mod pages;

pub struct Codex {
    pages: Vec<Option<&'static Page>>,
    reverse_map: HashMap<char, Glyph>,
}

impl Codex {
    pub fn new() -> Self {
        let mut codex = Codex {
            // Currently only `windows-1252` and `cp437` are planned but init
            // all pages with `None` will not expand the memory footprint but save from
            // new allocations
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
        if self.pages[id as usize].is_some() {
            let _: TalosResult<()> =
                Err(TalosError::PageIdInUse(id)).expect("Default Page is guaranteed to be valid");
        }
        validate_page(page).expect("Default Page is guaranteed to be valid");

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
    /// let mut codex = Codex::new().unwrap();
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
                let glyph_id = (u16::from(id) << 8) | (index as u16);
                self.reverse_map.entry(ch).or_insert(glyph_id);
            }
        }
    }
}
