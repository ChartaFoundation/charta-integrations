# Charta MCP Integration

Model Context Protocol (MCP) integration layer for Charta.

## Components

- **Server Discovery**: Discover and list MCP servers from configuration
- **Schema Inference**: Infer Charta block signatures from MCP tool schemas
- **Adapter Framework**: Framework for MCP tool invocation
- **Client**: MCP client (placeholder for future implementation)

## Implementation Status

### Completed (Phase 1)

- MCP server discovery from JSON configuration
- Tool schema inference (MCP tool schema → Charta block signature)
- Basic adapter framework with trait definition
- Tool registry and management

## Usage

```rust
use charta_mcp::{discover_servers, BasicMCPAdapter, MCPAdapter};

// Discover MCP servers
let config_path = PathBuf::from("mcp_config.json");
let servers = discover_servers(&config_path)?;

// Create adapter
let mut adapter = BasicMCPAdapter::new();

// Infer block signature from tool
let signature = adapter.infer_block_signature("tool_name")?;
```

## MCP Configuration

Example `mcp_config.json`:

```json
{
  "servers": {
    "hansard": {
      "name": "hansard",
      "command": "mcp-server-hansard",
      "args": []
    }
  }
}
```

## Future Enhancements

- Full MCP client implementation (stdio/HTTP)
- Tool invocation
- Error handling and timeouts
- Tool caching
