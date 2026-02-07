pub type TalosResult<T> = std::result::Result<T, TalosError>;

#[derive(Debug)]
pub enum TalosError {
    IOError(std::io::Error),
    InvalidArgument(String),
    InvalidState,
    DefaultPageId(u8),
    PageIdInUse(u8),
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
