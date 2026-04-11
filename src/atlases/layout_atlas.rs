use std::collections::BTreeMap;

use crate::layout::Rect;

/// A collection of named layout `Rect` objects.
///
/// A `LayoutAtlas` allows for storing and retrieving `Rect` objects by string keys,
/// which is useful for managing complex layouts where different parts of the UI
/// need to share or reference specific areas.
pub struct LayoutAtlas {
    /// The internal store for the `Rect` objects, mapped by their names.
    pub store: BTreeMap<String, Rect>,
}

impl From<BTreeMap<String, Rect>> for LayoutAtlas {
    #[inline]
    fn from(store: BTreeMap<String, Rect>) -> Self {
        Self { store }
    }
}

impl From<LayoutAtlas> for BTreeMap<String, Rect> {
    #[inline]
    fn from(atlas: LayoutAtlas) -> Self {
        atlas.store
    }
}

impl Default for LayoutAtlas {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutAtlas {
    /// Creates a new layout atlas
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            store: BTreeMap::new(),
        }
    }
    /// Get a `Rect` by key
    ///
    /// Returns `None` if the key doesn't exist
    ///
    /// Consider using `get_rect_exists` if you're sure the key exists to write less
    /// boilerplate
    #[inline]
    #[must_use]
    pub fn get_rect(&self, key: &str) -> Option<Rect> {
        self.store.get(key).cloned()
    }
    /// Gets a `Rect`, but panics if it doesn't exist
    ///
    /// Only use this if you're sure the `Rect` exists.
    ///
    /// Convenience function to replace `get_rect("default").expect("Known key must exist")`
    #[inline]
    #[must_use]
    pub fn get_rect_exists(&self, key: &str) -> Rect {
        match self.store.get(key) {
            Some(rect) => *rect,
            None => panic!("No such key: {}", key),
        }
    }
    /// Insert a `Rect` into the atlas. Will overwrite if the key already exists
    #[inline]
    pub fn insert(&mut self, key: String, rect: Rect) {
        self.store.insert(key, rect);
    }
}
