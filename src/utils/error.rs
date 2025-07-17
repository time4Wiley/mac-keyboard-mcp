use thiserror::Error;

/// Custom error types for the Mac Keyboard MCP server
#[derive(Error, Debug)]
pub enum MacKeyboardError {
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    
    #[error("Invalid category: {0}")]
    InvalidCategory(String),
    
    #[error("Parse error: {0}")]
    ParseError(#[from] crate::keycode::parser::ParseError),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("MCP error: {0}")]
    McpError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Result type alias for Mac Keyboard operations
pub type Result<T> = std::result::Result<T, MacKeyboardError>;