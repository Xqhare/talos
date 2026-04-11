//! This module contains the error types for the Talos library.

use std::{fmt, io, result};

/// The result type for `Talos`
pub type Result<T> = result::Result<T, Error>;

/// The error type for `Talos`
#[derive(Debug)]
pub enum Error {
    /// Standard IO error
    IO(io::Error),
    /// Simple invalid argument
    InvalidArgument(String),
    /// Invalid State of State Machine
    InvalidState,
    /// Codex page ID is already in use as a default (0-15)
    DefaultPageId(u8),
    /// Codex page ID is already in use
    PageIdInUse(u8),
    /// Generic error - Only use for development, always refactor to something more specific
    GenericError(String),
}

impl From<io::Error> for Error {
    #[inline]
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<String> for Error {
    #[inline]
    fn from(s: String) -> Self {
        Error::GenericError(s)
    }
}

impl From<&str> for Error {
    #[inline]
    fn from(s: &str) -> Self {
        Error::GenericError(s.to_string())
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(e) => write!(f, "IO: {e}"),
            Error::InvalidArgument(s) => write!(f, "InvalidArgument: {s}"),
            Error::InvalidState => write!(f, "InvalidState"),
            Error::DefaultPageId(id) => {
                write!(f, "Page ID '{id}' is a default page - Page ID unavailable.")
            }
            Error::PageIdInUse(id) => write!(f, "Page ID '{id}' already in use"),
            Error::GenericError(s) => write!(f, "GenericError: {s}"),
        }
    }
}
