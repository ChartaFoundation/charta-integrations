use thiserror::Error;

pub type Result<T> = std::result::Result<T, MCPError>;

#[derive(Error, Debug)]
pub enum MCPError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("MCP server error: {0}")]
    Server(String),
    
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    
    #[error("Schema inference error: {0}")]
    SchemaInference(String),
}
