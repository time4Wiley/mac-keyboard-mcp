use anyhow::Result;
use serde_json::{json, Value};
use tracing::info;

use crate::mcp::tools;
use crate::mcp::resources;

/// MCP server for macOS AppleScript key codes
pub struct MacKeyboardServer {
    // Future: Add any server state here
}

impl MacKeyboardServer {
    /// Create a new MCP server instance
    pub fn new() -> Result<Self> {
        info!("Initializing Mac Keyboard MCP server");
        
        // Initialize key database (lazy static will load on first access)
        let _ = &crate::keycode::KEY_DATABASE;
        
        Ok(Self {})
    }
    
    /// Get server capabilities
    pub fn capabilities(&self) -> Value {
        json!({
            "name": "mac-keyboard-mcp",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "MCP server for macOS AppleScript key codes",
            "tools": self.list_tools(),
            "resources": self.list_resources(),
        })
    }
    
    /// List available tools
    pub fn list_tools(&self) -> Value {
        json!([
            tools::lookup_keycode_tool(),
            tools::search_keys_tool(),
            tools::get_key_combinations_tool(),
            tools::list_categories_tool(),
        ])
    }
    
    /// List available resources
    pub fn list_resources(&self) -> Value {
        json!([
            resources::all_keycodes_resource(),
            resources::aliases_resource(),
        ])
    }
    
    /// Handle tool calls
    pub async fn call_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        match name {
            "lookup_keycode" => tools::handle_lookup(arguments).await,
            "search_keys" => tools::handle_search(arguments).await,
            "get_key_combinations" => tools::handle_combinations(arguments).await,
            "list_categories" => tools::handle_categories(arguments).await,
            _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
        }
    }
    
    /// Handle resource reads
    pub async fn read_resource(&self, uri: &str) -> Result<Value> {
        resources::read_resource_content(uri).await
    }
}

// TODO: When rmcp/mcp-sdk is available, implement the actual MCP traits
// #[async_trait]
// impl mcp_sdk::Server for MacKeyboardServer {
//     async fn list_tools(&self) -> Result<Vec<Tool>, Error> {
//         // Implementation
//     }
//     // ... other trait methods
// }