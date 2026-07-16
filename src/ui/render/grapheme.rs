//! This module defines the [`Grapheme`] structure, representing a stack-allocated
//! Unicode grapheme cluster.

/// A stack-allocated Unicode grapheme cluster representation.
///
/// Under the hood, this contains an inline buffer of up to 15 bytes to store the
/// UTF-8 representation of the grapheme cluster. This avoids allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Grapheme {
    bytes: [u8; 15],
    len: u8,
}

impl Default for Grapheme {
    /// Creates a default `Grapheme` instance containing a single space (" ").
    fn default() -> Self {
        Self::new(" ")
    }
}

impl Grapheme {
    /// Creates a new `Grapheme` from the given string slice.
    ///
    /// If the string slice has more than 15 bytes, it will be truncated.
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice representing the grapheme.
    #[must_use]
    #[inline]
    pub fn new(s: &str) -> Self {
        let bytes_slice = s.as_bytes();
        let mut bytes = [0u8; 15];
        let len = bytes_slice.len().min(15);
        bytes[..len].copy_from_slice(&bytes_slice[..len]);
        Self {
            bytes,
            len: len as u8,
        }
    }

    /// Returns a string slice referencing the stored grapheme cluster.
    ///
    /// If the internal byte slice does not form a valid UTF-8 sequence,
    /// a default space character string (" ") is returned.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..self.len as usize]).unwrap_or(" ")
    }
}
