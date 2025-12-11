use serde::{Deserialize, Serialize};
use crate::error::{Result, MCPError};
use std::collections::HashMap;
use std::path::PathBuf;

/// MCP Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPServer {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

/// MCP Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPConfig {
    pub servers: HashMap<String, MCPServer>,
}

/// Discover MCP servers from configuration file
pub fn discover_servers(config_path: &PathBuf) -> Result<Vec<MCPServer>> {
    if !config_path.exists() {
        return Ok(Vec::new()); // No config file, return empty list
    }
    
    let config_content = std::fs::read_to_string(config_path)?;
    let config: MCPConfig = serde_json::from_str(&config_content)?;
    
    Ok(config.servers.into_values().collect())
}

/// List available MCP servers
pub fn list_servers(config_path: &PathBuf) -> Result<Vec<String>> {
    let servers = discover_servers(config_path)?;
    Ok(servers.into_iter().map(|s| s.name).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_discover_servers() {
        let config = MCPConfig {
            servers: {
                let mut map = HashMap::new();
                map.insert("server1".to_string(), MCPServer {
                    name: "server1".to_string(),
                    command: Some("mcp-server".to_string()),
                    args: None,
                    env: None,
                });
                map
            },
        };
        
        let json = serde_json::to_string(&config).unwrap();
        let temp_file = std::env::temp_dir().join("mcp_config.json");
        std::fs::write(&temp_file, json).unwrap();
        
        let servers = discover_servers(&temp_file).unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].name, "server1");
    }
}
