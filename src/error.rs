/// The result type for `Talos`
pub type TalosResult<T> = std::result::Result<T, TalosError>;

/// The error type for `Talos`
#[derive(Debug)]
pub enum TalosError {
    /// Standard IO error
    IOError(std::io::Error),
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

impl From<std::io::Error> for TalosError {
    fn from(e: std::io::Error) -> Self {
        TalosError::IOError(e)
    }
}

impl From<String> for TalosError {
    fn from(s: String) -> Self {
        TalosError::GenericError(s)
    }
}

impl From<&str> for TalosError {
    fn from(s: &str) -> Self {
        TalosError::GenericError(s.to_string())
    }
}

impl std::fmt::Display for TalosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TalosError::IOError(e) => write!(f, "IOError: {e}"),
            TalosError::InvalidArgument(s) => write!(f, "InvalidArgument: {s}"),
            TalosError::InvalidState => write!(f, "InvalidState"),
            TalosError::DefaultPageId(id) => {
                write!(f, "Page ID '{id}' is a default page - Page ID unavailable.")
            }
            TalosError::PageIdInUse(id) => write!(f, "Page ID '{id}' already in use"),
            TalosError::GenericError(s) => write!(f, "GenericError: {s}"),
        }
    }
}
