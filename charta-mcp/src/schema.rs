use serde::{Deserialize, Serialize};
use crate::error::{Result, MCPError};

/// MCP Tool Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPToolSchema {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<serde_json::Value>,
}

/// Charta Block Signature inferred from MCP tool
#[derive(Debug, Clone)]
pub struct BlockSignature {
    pub name: String,
    pub inputs: Vec<PortSignature>,
    pub outputs: Vec<PortSignature>,
    pub effect: String, // e.g., "MCP[tool_name]"
}

#[derive(Debug, Clone)]
pub struct PortSignature {
    pub name: String,
    pub type_: String,
}

/// Infer Charta block signature from MCP tool schema
pub fn infer_block_signature(tool: &MCPToolSchema) -> Result<BlockSignature> {
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    
    // Extract inputs from tool schema
    if let Some(ref input_schema) = tool.input_schema {
        if let Some(properties) = input_schema.get("properties").and_then(|p| p.as_object()) {
            for (name, prop) in properties {
                let type_ = infer_type_from_json_schema(prop)
                    .unwrap_or_else(|| "Any".to_string());
                inputs.push(PortSignature {
                    name: name.clone(),
                    type_,
                });
            }
        }
    }
    
    // Default output (MCP tools typically return JSON)
    outputs.push(PortSignature {
        name: "result".to_string(),
        type_: "Any".to_string(),
    });
    
    Ok(BlockSignature {
        name: format!("MCP_{}", tool.name),
        inputs,
        outputs,
        effect: format!("MCP[\"{}\"]", tool.name),
    })
}

fn infer_type_from_json_schema(schema: &serde_json::Value) -> Option<String> {
    if let Some(type_str) = schema.get("type").and_then(|t| t.as_str()) {
        match type_str {
            "string" => Some("Text".to_string()),
            "number" => Some("Float".to_string()),
            "integer" => Some("Int".to_string()),
            "boolean" => Some("Bool".to_string()),
            "array" => {
                // Try to infer element type
                if let Some(items) = schema.get("items") {
                    if let Some(elem_type) = infer_type_from_json_schema(items) {
                        return Some(format!("list[{}]", elem_type));
                    }
                }
                Some("list[Any]".to_string())
            }
            "object" => Some("Any".to_string()),
            _ => Some("Any".to_string()),
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_block_signature() {
        let tool = MCPToolSchema {
            name: "search".to_string(),
            description: Some("Search tool".to_string()),
            input_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string"
                    },
                    "limit": {
                        "type": "integer"
                    }
                }
            })),
        };
        
        let signature = infer_block_signature(&tool).unwrap();
        assert_eq!(signature.name, "MCP_search");
        assert_eq!(signature.inputs.len(), 2);
        assert_eq!(signature.effect, "MCP[\"search\"]");
    }
}
