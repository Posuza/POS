/// Custom error types for POS application
use std::fmt;

/// Result type alias for POS operations
pub type Result<T> = std::result::Result<T, PosError>;

/// POS system errors
#[derive(Debug, Clone)]
pub enum PosError {
    /// Database operation failed
    DatabaseError(String),
    
    /// Image operation failed
    ImageError(String),
    
    /// Authentication failed
    AuthenticationError(String),
    
    /// Validation failed
    ValidationError(String),
    
    /// File operation failed
    FileError(String),
    
    /// Configuration error
    ConfigError(String),
    
    /// Not found error
    NotFound(String),
    
    /// General error
    Other(String),
}

impl fmt::Display for PosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PosError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            PosError::ImageError(msg) => write!(f, "Image error: {}", msg),
            PosError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            PosError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            PosError::FileError(msg) => write!(f, "File error: {}", msg),
            PosError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            PosError::NotFound(msg) => write!(f, "Not found: {}", msg),
            PosError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for PosError {}

// Conversion implementations
impl From<String> for PosError {
    fn from(err: String) -> Self {
        PosError::Other(err)
    }
}

impl From<&str> for PosError {
    fn from(err: &str) -> Self {
        PosError::Other(err.to_string())
    }
}

impl From<sqlx::Error> for PosError {
    fn from(err: sqlx::Error) -> Self {
        PosError::DatabaseError(err.to_string())
    }
}
