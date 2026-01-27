pub type TalosResult<T> = std::result::Result<T, TalosError>;

#[derive(Debug)]
pub enum TalosError {
    IOError(std::io::Error),
    InvalidArgument(String),
    InvalidState,
    PageIdInUse(u8),
}

impl From<std::io::Error> for TalosError {
    fn from(e: std::io::Error) -> Self {
        TalosError::IOError(e)
    }
}

impl std::fmt::Display for TalosError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TalosError::IOError(e) => write!(f, "IOError: {}", e),
            TalosError::InvalidArgument(s) => write!(f, "InvalidArgument: {}", s),
            TalosError::InvalidState => write!(f, "InvalidState"),
            TalosError::PageIdInUse(id) => write!(f, "Page ID '{}' already in use", id),
        }
    }
}
