use std::collections::HashMap;

use crate::{
    constants::pages::{
        Page, REG_CP437, REG_WIN_1252, UNKNOWN_CHAR, UNKNOWN_CHAR_GLYPH, validate_page,
    },
    error::{TalosError, TalosResult},
};

use super::Glyph;

pub struct Codex {
    pages: Vec<Option<&'static Page>>,
    reverse_map: HashMap<char, Glyph>,
}

impl Codex {
    pub fn new() -> Codex {
        let mut codex = Codex {
            // Currently only `windows-1252` and `cp437` are planned but init
            // all pages with `None` will not expand the memory footprint but save from
            // new allocations
            pages: vec![None; 256],
            reverse_map: HashMap::new(),
        };

        // ONLY these pages should be registered at startup - all others will be on the
        // users request
        //
        // TODO: Reconsider the `expect` calls - HOWEVER: The pages are build inside this function
        // anyway, and the page IDs are thus guaranteed to be valid
        // So its more a case of shutting up the compiler
        // ON THE OTHER HAND: The logic is only linked implicitly (by this comment to be exact) -
        // nothing is preventing me from moving the init or the registering into their own
        // functions and making this implicit link even harder to see
        codex
            .register_page(REG_WIN_1252.0, REG_WIN_1252.1)
            .expect("Page ID must be free during init");
        codex
            .register_page(REG_CP437.0, REG_CP437.1)
            .expect("Page ID must be free during init");

        codex
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

    pub fn resolve(&self, glyph: Glyph) -> &str {
        let page_id = (glyph >> 8) as usize;
        let char_id = (glyph & 0xFF) as usize;

        match self.pages.get(page_id) {
            Some(Some(page)) => page[char_id],
            _ => return UNKNOWN_CHAR,
        }
    }

    pub fn lookup(&self, ch: char) -> Glyph {
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
