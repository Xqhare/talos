use std::collections::BTreeMap;

use crate::render::{Colour, Normal, Style};

/// A collection of styles
///
/// Always includes the following styles:
///
/// * `default` - The default style, as passed in the constructor or the default Terminal Session style.
/// * `ok` - Uses `Green` as the foreground color and the default background.
/// * `warning` - Uses `Yellow` as the foreground color and the default background.
/// * `error` - Uses `Red` as the foreground color and the default background.
///
/// # Example
/// ```rust
/// use talos::{render::Style, atlases::StyleAtlas};
///
/// let mut atlas = StyleAtlas::new(None);
///
/// atlas.insert("custom".to_string(), Style::default());
/// atlas.update_ok(Style::default());
/// let custom = atlas.get_style_exists("custom");
/// let ok = atlas.get_ok();
/// let warn = atlas.get_warning();
/// # assert!(true);
/// ```
#[derive(Debug)]
pub struct StyleAtlas {
    store: BTreeMap<String, Style>,
}

impl From<StyleAtlas> for BTreeMap<String, Style> {
    #[inline]
    fn from(atlas: StyleAtlas) -> Self {
        atlas.store
    }
}

impl StyleAtlas {
    /// Creates a new Style atlas
    ///
    /// The atlas contains the following styles:
    ///
    /// * `default` - The default style, as passed in here or the default Terminal Session style if set to `None`.
    /// * `ok` - Uses `Green` as the foreground color and the default background.
    /// * `warning` - Uses `Yellow` as the foreground color and the default background.
    /// * `error` - Uses `Red` as the foreground color and the default background.
    ///
    /// # Arguments
    /// * `default` - The default style
    ///
    /// # Example
    /// ```rust,no_run
    /// use talos::atlases::StyleAtlas;
    ///
    /// let atlas = StyleAtlas::new(None);
    /// # assert!(true);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(default: Option<Style>) -> Self {
        let default = default.unwrap_or_else(Style::default);
        let ok = Style::builder()
            .set_fg(Colour::Normal(Normal::Green))
            .set_bg_option(default.get_bg())
            .build();
        let warning = Style::builder()
            .set_fg(Colour::Normal(Normal::Yellow))
            .set_bg_option(default.get_bg())
            .build();
        let error = Style::builder()
            .set_fg(Colour::Normal(Normal::Red))
            .set_bg_option(default.get_bg())
            .build();

        let mut store = BTreeMap::new();
        let _ = store.insert("default".to_string(), default);
        let _ = store.insert("ok".to_string(), ok);
        let _ = store.insert("warning".to_string(), warning);
        let _ = store.insert("error".to_string(), error);

        Self { store }
    }
    /// Get the default `Style`
    ///
    /// This is the default style, as passed in the constructor or the default Terminal Session style
    /// To change use `update_default` or `update_default_only` as needed.
    #[inline]
    #[must_use]
    pub fn get_default(&self) -> Style {
        self.get_style_exists("default")
    }
    /// Get the `ok` `Style`
    ///
    /// Uses `Green` as the foreground color and the default background.
    /// To change use `update_ok`
    #[inline]
    #[must_use]
    pub fn get_ok(&self) -> Style {
        self.get_style_exists("ok")
    }
    /// Get the `warning` `Style`
    ///
    /// Uses `Yellow` as the foreground color and the default background.
    /// To change use `update_warning`
    #[inline]
    #[must_use]
    pub fn get_warning(&self) -> Style {
        self.get_style_exists("warning")
    }
    /// Get the `error` `Style`
    ///
    /// Uses `Red` as the foreground color and the default background.
    /// To change use `update_error`
    #[inline]
    #[must_use]
    pub fn get_error(&self) -> Style {
        self.get_style_exists("error")
    }
    /// Updates the default `Style`
    ///
    /// This also updates the `ok`, `warning` and `error` styles background colours to be the
    /// same as the new default
    #[inline]
    pub fn update_default(&mut self, style: Style) {
        let new_bg = style.get_bg();
        if let Some(s) = self.store.get_mut("default") {
            s.set_bg(new_bg);
        }
        if let Some(s) = self.store.get_mut("ok") {
            s.set_bg(new_bg);
        }
        if let Some(s) = self.store.get_mut("warning") {
            s.set_bg(new_bg);
        }
        if let Some(s) = self.store.get_mut("error") {
            s.set_bg(new_bg);
        }
    }
    /// Updates the `default` `Style`
    ///
    /// This only updates the `default` `Style`, and gives full control over the style
    #[inline]
    pub fn update_default_only(&mut self, style: Style) {
        if let Some(s) = self.store.get_mut("default") {
            *s = style;
        }
    }
    /// Updates the `ok` `Style`
    ///
    /// This only updates the `ok` `Style`, and gives full control over the style
    #[inline]
    pub fn update_ok(&mut self, style: Style) {
        if let Some(s) = self.store.get_mut("ok") {
            *s = style;
        }
    }
    /// Updates the `warning` `Style`
    ///
    /// This only updates the `warning` `Style`, and gives full control over the style
    #[inline]
    pub fn update_warning(&mut self, style: Style) {
        if let Some(s) = self.store.get_mut("warning") {
            *s = style;
        }
    }
    /// Updates the `error` `Style`
    ///
    /// This only updates the `error` `Style`, and gives full control over the style
    #[inline]
    pub fn update_error(&mut self, style: Style) {
        if let Some(s) = self.store.get_mut("error") {
            *s = style;
        }
    }
    /// Get a `Style` by key
    ///
    /// Returns `None` if the key doesn't exist
    ///
    /// Consider using `get_Style_exists` if you're sure the key exists to write less
    /// boilerplate
    #[inline]
    #[must_use]
    pub fn get_style(&self, key: &str) -> Option<Style> {
        self.store.get(key).cloned()
    }
    /// Gets a `Style`, but panics if it doesn't exist
    ///
    /// Only use this if you're sure the `Style` exists.
    ///
    /// Convenience function to replace `get_Style("default").expect("Known key must exist")`
    #[inline]
    #[must_use]
    pub fn get_style_exists(&self, key: &str) -> Style {
        match self.store.get(key) {
            Some(style) => *style,
            None => panic!("No such key: {}", key),
        }
    }
    /// Insert a `Style` into the atlas. Will overwrite if the key already exists
    #[inline]
    pub fn insert(&mut self, key: String, style: Style) {
        let _ = self.store.insert(key, style);
    }
}
