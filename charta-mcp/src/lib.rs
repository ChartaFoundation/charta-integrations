pub mod discovery;
pub mod adapter;
pub mod schema;
pub mod client;
pub mod error;

pub use discovery::{discover_servers, MCPServer};
pub use adapter::MCPAdapter;
pub use schema::{infer_block_signature, BlockSignature};
pub use error::{MCPError, Result};
