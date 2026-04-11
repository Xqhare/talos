use crate::render::Colour;

use super::Style;

/// A builder for the `Style` struct
///
/// # Example
/// ```rust
/// use talos::render::{Colour, Normal, Style};
///
/// let style = Style::builder()
///     .set_fg(Colour::Normal(Normal::Red))
///     .set_bg(Colour::Normal(Normal::Blue))
///     .set_bold(true)
///     .build();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct StyleBuilder {
    fg: Option<Colour>,
    bg: Option<Colour>,
    bit_flag: u8,
}

impl StyleBuilder {
    /// Creates a new StyleBuilder
    ///
    /// Consider using the [`Style::builder()`](struct.Style.html#method.builder)
    /// method instead to construct new styles.
    ///
    /// Consider using the [`Style::edit()`](struct.Style.html#method.edit) method
    /// to edit existing styles.
    #[inline]
    #[must_use]
    pub fn new(fg: Option<Colour>, bg: Option<Colour>, bit_flag: u8) -> Self {
        Self { fg, bg, bit_flag }
    }
    /// Sets the foreground color
    ///
    /// Supports setting the foreground color to `None`. This will be rendered as the default
    /// foreground color of the Terminal Session
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{Colour, Normal, Style};
    ///
    /// let style = Style::builder()
    ///     .set_fg_option(None)
    ///     .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn set_fg_option(mut self, fg: Option<Colour>) -> Self {
        self.fg = fg;
        self
    }
    /// Sets the background color
    ///
    /// Supports setting the background color to `None`. This will be rendered as the default
    /// background color of the Terminal Session
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{Colour, Normal, Style};
    ///
    /// let style = Style::builder()
    ///     .set_bg_option(None)
    ///     .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn set_bg_option(mut self, bg: Option<Colour>) -> Self {
        self.bg = bg;
        self
    }
    /// Sets the foreground color
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{Colour, Normal, Style};
    ///
    /// let style = Style::builder()
    ///     .set_fg(Colour::Normal(Normal::Red))
    ///     .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn set_fg(mut self, fg: Colour) -> Self {
        self.fg = Some(fg);
        self
    }

    /// Sets the background color
    ///
    /// # Example
    /// ```rust
    /// use talos::render::{Colour, Normal, Style};
    ///
    /// let style = Style::builder()
    ///     .set_bg(Colour::Normal(Normal::Blue))
    ///     .build();
    /// ```
    #[inline]
    #[must_use]
    pub fn set_bg(mut self, bg: Colour) -> Self {
        self.bg = Some(bg);
        self
    }

    /// Sets the bold attribute
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Style;
    ///
    /// let style = Style::builder().set_bold(true).build();
    /// ```
    #[inline]
    #[must_use]
    pub fn set_bold(mut self, bold: bool) -> Self {
        if bold {
            self.bit_flag |= 0b1000_0000;
        } else {
            self.bit_flag &= !0b1000_0000;
        }
        self
    }

    /// Sets the dim attribute
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Style;
    ///
    /// let style = Style::builder().set_dim(true).build();
    /// ```
    #[inline]
    #[must_use]
    pub fn set_dim(mut self, dim: bool) -> Self {
        if dim {
            self.bit_flag |= 0b0100_0000;
        } else {
            self.bit_flag &= !0b0100_0000;
        }
        self
    }

    /// Sets the italic attribute
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Style;
    ///
    /// let style = Style::builder().set_italic(true).build();
    /// ```
    #[inline]
    #[must_use]
    pub fn set_italic(mut self, italic: bool) -> Self {
        if italic {
            self.bit_flag |= 0b0010_0000;
        } else {
            self.bit_flag &= !0b0010_0000;
        }
        self
    }

    /// Sets the underline attribute
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Style;
    ///
    /// let style = Style::builder().set_underline(true).build();
    /// ```
    #[inline]
    #[must_use]
    pub fn set_underline(mut self, underline: bool) -> Self {
        if underline {
            self.bit_flag |= 0b0001_0000;
        } else {
            self.bit_flag &= !0b0001_0000;
        }
        self
    }

    /// Sets the blink attribute
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Style;
    ///
    /// let style = Style::builder().set_blink(true).build();
    /// ```
    #[inline]
    #[must_use]
    pub fn set_blink(mut self, blink: bool) -> Self {
        if blink {
            self.bit_flag |= 0b0000_1000;
        } else {
            self.bit_flag &= !0b0000_1000;
        }
        self
    }

    /// Sets the reverse attribute
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Style;
    ///
    /// let style = Style::builder().set_reverse(true).build();
    /// ```
    #[inline]
    #[must_use]
    pub fn set_reverse(mut self, reverse: bool) -> Self {
        if reverse {
            self.bit_flag |= 0b0000_0100;
        } else {
            self.bit_flag &= !0b0000_0100;
        }
        self
    }

    /// Sets the hidden attribute
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Style;
    ///
    /// let style = Style::builder().set_hidden(true).build();
    /// ```
    #[inline]
    #[must_use]
    pub fn set_hidden(mut self, hidden: bool) -> Self {
        if hidden {
            self.bit_flag |= 0b0000_0010;
        } else {
            self.bit_flag &= !0b0000_0010;
        }
        self
    }

    /// Sets the strikethrough attribute
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Style;
    ///
    /// let style = Style::builder().set_strikethrough(true).build();
    /// ```
    #[inline]
    #[must_use]
    pub fn set_strikethrough(mut self, strikethrough: bool) -> Self {
        if strikethrough {
            self.bit_flag |= 0b0000_0001;
        } else {
            self.bit_flag &= !0b0000_0001;
        }
        self
    }

    /// Builds the `Style`
    ///
    /// # Example
    /// ```rust
    /// use talos::render::Style;
    ///
    /// let style = Style::builder().build();
    /// ```
    #[inline]
    #[must_use]
    pub fn build(self) -> Style {
        Style {
            fg: self.fg,
            bg: self.bg,
            bit_flag: self.bit_flag,
        }
    }
}
