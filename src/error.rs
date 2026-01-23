
pub type TalosResult<T> = std::result::Result<T, TalosError>;

pub enum TalosError {
    IOError(std::io::Error),
    InvalidArgument,
    InvalidState,
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
            TalosError::InvalidArgument => write!(f, "InvalidArgument"),
            TalosError::InvalidState => write!(f, "InvalidState"),
        }
    }
}
