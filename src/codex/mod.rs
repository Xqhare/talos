use std::collections::HashMap;

use crate::{codex::pages::{Page, REG_CP437, REG_WIN_1252, UNKNOWN_CHAR, UNKNOWN_CHAR_GLYPH, pre_computed_char, validate_page}, error::{TalosError, TalosResult}, render::Glyph};

pub mod pages;

pub struct Codex {
    pages: Vec<Option<&'static Page>>,
    reverse_map: HashMap<char, Glyph>,
}

impl Codex {
    pub fn new() -> TalosResult<Self> {
        let mut codex = Codex {
            // Currently only `windows-1252` and `cp437` are planned but init
            // all pages with `None` will not expand the memory footprint but save from
            // new allocations
            pages: vec![None; 256],
            reverse_map: HashMap::new(),
        };

        codex.register_page(REG_WIN_1252.0, REG_WIN_1252.1)?;
        codex.register_page(REG_CP437.0, REG_CP437.1)?;

        Ok(codex)
    }

    pub fn register_page(&mut self, id: u8, page: &'static Page) -> TalosResult<()> {
        if self.pages[id as usize].is_some() {
            return Err(TalosError::PageIdInUse(id));
        }
        validate_page(&page)?;

        self.pages[id as usize] = Some(page);

        self.update_cache(id, page);

        Ok(())
    }

    // TODO: Optimise by hardcoding ASCII 0-127
    pub fn resolve(&self, glyph: Glyph) -> &str {
        if let Some(char) = pre_computed_char(glyph) {
            return char;
        }

        let page_id = (glyph >> 8) as usize;
        let char_id = (glyph & 0xFF) as usize;

        match self.pages.get(page_id) {
            Some(Some(page)) => page[char_id],
            _ => return UNKNOWN_CHAR,
        }
    }

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
                let glyph_id = ((id as u16) << 8) | (index as u16);
                self.reverse_map.entry(ch).or_insert(glyph_id);
            }
        }
    }
}
