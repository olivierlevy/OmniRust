/// A common result type for OmniRust operations.
pub type OmniResult<T> = Result<T, OmniError>;

/// Custom error type.
#[derive(Debug)]
pub enum OmniError {
    Io(std::io::Error),
    Parse(String),
    Custom(String),
}

impl std::fmt::Display for OmniError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OmniError::Io(e) => write!(f, "IO Error: {}", e),
            OmniError::Parse(msg) => write!(f, "Parse Error: {}", msg),
            OmniError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for OmniError {}

impl From<std::io::Error> for OmniError {
    fn from(e: std::io::Error) -> Self {
        OmniError::Io(e)
    }
}
