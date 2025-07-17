# Level 2 Plan: Detailed Implementation

## Project Structure

```
mac-keyboard-mcp/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs              # Entry point and MCP server setup
│   ├── lib.rs               # Library exports
│   ├── keycode/
│   │   ├── mod.rs           # Module declarations
│   │   ├── database.rs      # Key code database and loading
│   │   ├── types.rs         # Data structures and types
│   │   └── parser.rs        # Shortcut parsing logic
│   ├── mcp/
│   │   ├── mod.rs           # MCP module declarations
│   │   ├── server.rs        # MCP server implementation
│   │   ├── tools.rs         # Tool implementations
│   │   └── resources.rs     # Resource implementations
│   ├── search/
│   │   ├── mod.rs           # Search module declarations
│   │   ├── fuzzy.rs         # Fuzzy matching implementation
│   │   └── aliases.rs       # Alias resolution
│   └── utils/
│       ├── mod.rs           # Utility module declarations
│       └── error.rs         # Error types and handling
├── data/
│   └── keycodes.json        # Static key code data
├── tests/
│   ├── integration_tests.rs # Integration tests
│   └── unit_tests.rs        # Unit tests
└── examples/
    └── client_example.rs    # Example MCP client usage
```

## Module Specifications

### 1. Main Module (`src/main.rs`)

```rust
use rmcp::{ServerHandler, ServerOptions, StdioTransport};
use crate::mcp::MacKeyboardServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Create server instance
    let server = MacKeyboardServer::new()?;
    
    // Configure server options
    let options = ServerOptions {
        name: "mac-keyboard-mcp".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        ..Default::default()
    };
    
    // Start server with stdio transport
    let transport = StdioTransport::new();
    ServerHandler::new(server, options)
        .run(transport)
        .await?;
    
    Ok(())
}
```

### 2. Key Code Database (`src/keycode/database.rs`)

```rust
use once_cell::sync::Lazy;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCode {
    pub name: String,
    pub code: u16,
    pub category: KeyCategory,
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum KeyCategory {
    Letters,
    Numbers,
    FunctionKeys,
    ModifierKeys,
    NavigationKeys,
    SpecialKeys,
    NumpadKeys,
    Punctuation,
}

pub static KEY_DATABASE: Lazy<KeyDatabase> = Lazy::new(|| {
    KeyDatabase::load().expect("Failed to load key database")
});

pub struct KeyDatabase {
    by_name: HashMap<String, KeyCode>,
    by_code: HashMap<u16, KeyCode>,
    by_category: HashMap<KeyCategory, Vec<KeyCode>>,
    aliases: HashMap<String, String>,
}

impl KeyDatabase {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Load from embedded JSON file
        let json_data = include_str!("../../data/keycodes.json");
        let keycodes: HashMap<String, HashMap<String, u16>> = 
            serde_json::from_str(json_data)?;
        
        // Build database structures
        // ...implementation...
    }
    
    pub fn lookup(&self, name: &str) -> Option<&KeyCode> {
        // Case-insensitive lookup with alias resolution
    }
    
    pub fn search(&self, query: &str, fuzzy: bool) -> Vec<&KeyCode> {
        // Search implementation
    }
}
```

### 3. MCP Server Implementation (`src/mcp/server.rs`)

```rust
use rmcp::{Tool, Resource, ServerCapabilities};
use async_trait::async_trait;

pub struct MacKeyboardServer {
    tools: Vec<Tool>,
    resources: Vec<Resource>,
}

impl MacKeyboardServer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let tools = vec![
            create_lookup_tool(),
            create_search_tool(),
            create_combinations_tool(),
            create_categories_tool(),
        ];
        
        let resources = vec![
            create_all_keycodes_resource(),
            create_category_resources(),
            create_aliases_resource(),
        ];
        
        Ok(Self { tools, resources })
    }
}

#[async_trait]
impl rmcp::Server for MacKeyboardServer {
    async fn list_tools(&self) -> Result<Vec<Tool>, rmcp::Error> {
        Ok(self.tools.clone())
    }
    
    async fn call_tool(
        &self,
        name: &str,
        arguments: serde_json::Value,
    ) -> Result<serde_json::Value, rmcp::Error> {
        match name {
            "lookup_keycode" => handle_lookup(arguments).await,
            "search_keys" => handle_search(arguments).await,
            "get_key_combinations" => handle_combinations(arguments).await,
            "list_categories" => handle_categories(arguments).await,
            _ => Err(rmcp::Error::ToolNotFound),
        }
    }
    
    async fn list_resources(&self) -> Result<Vec<Resource>, rmcp::Error> {
        Ok(self.resources.clone())
    }
    
    async fn read_resource(
        &self,
        uri: &str,
    ) -> Result<serde_json::Value, rmcp::Error> {
        // Resource reading implementation
    }
}
```

### 4. Tool Implementations (`src/mcp/tools.rs`)

```rust
use rmcp::{Tool, ToolParameter};
use serde_json::json;
use crate::keycode::KEY_DATABASE;

pub fn create_lookup_tool() -> Tool {
    Tool {
        name: "lookup_keycode".to_string(),
        description: "Find AppleScript key code for a specific key".to_string(),
        parameters: vec![
            ToolParameter {
                name: "key_name".to_string(),
                type_: "string".to_string(),
                description: "Name of the key (e.g., 'A', 'Space', 'Command')".to_string(),
                required: true,
            },
            ToolParameter {
                name: "fuzzy".to_string(),
                type_: "boolean".to_string(),
                description: "Enable fuzzy matching".to_string(),
                required: false,
            },
        ],
    }
}

pub async fn handle_lookup(args: serde_json::Value) -> Result<serde_json::Value, rmcp::Error> {
    let key_name = args["key_name"]
        .as_str()
        .ok_or_else(|| rmcp::Error::InvalidArguments("key_name required".to_string()))?;
    
    let fuzzy = args["fuzzy"].as_bool().unwrap_or(false);
    
    if let Some(keycode) = KEY_DATABASE.lookup(key_name) {
        Ok(json!({
            "found": true,
            "key": {
                "name": keycode.name,
                "code": keycode.code,
                "category": keycode.category,
                "aliases": keycode.aliases,
            }
        }))
    } else if fuzzy {
        let suggestions = KEY_DATABASE.search(key_name, true);
        Ok(json!({
            "found": false,
            "suggestions": suggestions.iter()
                .take(5)
                .map(|k| json!({
                    "name": k.name,
                    "code": k.code,
                    "similarity": calculate_similarity(key_name, &k.name),
                }))
                .collect::<Vec<_>>()
        }))
    } else {
        Ok(json!({
            "found": false,
            "message": format!("Key '{}' not found", key_name),
        }))
    }
}
```

### 5. Shortcut Parser (`src/keycode/parser.rs`)

```rust
use crate::keycode::{KeyCode, KEY_DATABASE};

#[derive(Debug)]
pub struct KeyCombination {
    pub modifiers: Vec<KeyCode>,
    pub key: KeyCode,
}

pub fn parse_shortcut(shortcut: &str) -> Result<KeyCombination, ParseError> {
    let parts: Vec<&str> = shortcut
        .split(|c| c == '+' || c == '-')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    if parts.is_empty() {
        return Err(ParseError::EmptyShortcut);
    }
    
    let mut modifiers = Vec::new();
    let mut main_key = None;
    
    for part in parts {
        if let Some(keycode) = KEY_DATABASE.lookup(part) {
            match keycode.category {
                KeyCategory::ModifierKeys => modifiers.push(keycode.clone()),
                _ => {
                    if main_key.is_some() {
                        return Err(ParseError::MultipleMainKeys);
                    }
                    main_key = Some(keycode.clone());
                }
            }
        } else {
            return Err(ParseError::UnknownKey(part.to_string()));
        }
    }
    
    let key = main_key.ok_or(ParseError::NoMainKey)?;
    
    Ok(KeyCombination { modifiers, key })
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Empty shortcut")]
    EmptyShortcut,
    #[error("Unknown key: {0}")]
    UnknownKey(String),
    #[error("Multiple non-modifier keys in shortcut")]
    MultipleMainKeys,
    #[error("No main key in shortcut")]
    NoMainKey,
}
```

### 6. Fuzzy Search (`src/search/fuzzy.rs`)

```rust
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

pub struct FuzzySearcher {
    matcher: SkimMatcherV2,
}

impl FuzzySearcher {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
        }
    }
    
    pub fn search<'a>(
        &self,
        query: &str,
        candidates: &'a [String],
    ) -> Vec<(&'a str, i64)> {
        let mut results: Vec<_> = candidates
            .iter()
            .filter_map(|candidate| {
                self.matcher
                    .fuzzy_match(candidate, query)
                    .map(|score| (candidate.as_str(), score))
            })
            .collect();
        
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results
    }
}

pub fn calculate_similarity(a: &str, b: &str) -> f64 {
    let matcher = SkimMatcherV2::default();
    let max_len = std::cmp::max(a.len(), b.len()) as f64;
    
    if let Some(score) = matcher.fuzzy_match(b, a) {
        (score as f64) / (max_len * 100.0)
    } else {
        0.0
    }
}
```

### 7. Resource Implementations (`src/mcp/resources.rs`)

```rust
use rmcp::Resource;
use serde_json::json;
use crate::keycode::{KEY_DATABASE, KeyCategory};

pub fn create_all_keycodes_resource() -> Resource {
    Resource {
        uri: "keycode://all".to_string(),
        name: "All Key Codes".to_string(),
        description: "Complete database of AppleScript key codes".to_string(),
        mime_type: "application/json".to_string(),
    }
}

pub fn create_category_resources() -> Vec<Resource> {
    let categories = [
        (KeyCategory::Letters, "letters", "Letter keys A-Z"),
        (KeyCategory::Numbers, "numbers", "Number keys 0-9"),
        (KeyCategory::FunctionKeys, "function_keys", "Function keys F1-F20"),
        (KeyCategory::ModifierKeys, "modifier_keys", "Modifier keys (Cmd, Shift, etc.)"),
        (KeyCategory::NavigationKeys, "navigation_keys", "Navigation keys (arrows, etc.)"),
        (KeyCategory::SpecialKeys, "special_keys", "Special keys (Space, Tab, etc.)"),
        (KeyCategory::NumpadKeys, "numpad_keys", "Numeric keypad keys"),
        (KeyCategory::Punctuation, "punctuation", "Punctuation and symbol keys"),
    ];
    
    categories
        .iter()
        .map(|(category, name, desc)| Resource {
            uri: format!("keycode://category/{}", name),
            name: format!("{} Key Codes", name.replace('_', " ").to_title_case()),
            description: desc.to_string(),
            mime_type: "application/json".to_string(),
        })
        .collect()
}

pub async fn read_resource_content(uri: &str) -> Result<serde_json::Value, rmcp::Error> {
    match uri {
        "keycode://all" => {
            let all_keys = KEY_DATABASE.all_keys();
            Ok(json!(all_keys))
        }
        uri if uri.starts_with("keycode://category/") => {
            let category_name = uri.strip_prefix("keycode://category/")
                .ok_or_else(|| rmcp::Error::ResourceNotFound)?;
            
            let category = parse_category_name(category_name)?;
            let keys = KEY_DATABASE.by_category(category);
            Ok(json!(keys))
        }
        "keycode://aliases" => {
            let aliases = KEY_DATABASE.all_aliases();
            Ok(json!(aliases))
        }
        _ => Err(rmcp::Error::ResourceNotFound),
    }
}
```

## Error Handling Strategy

### Custom Error Types

```rust
use thiserror::Error;

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
    McpError(#[from] rmcp::Error),
}

pub type Result<T> = std::result::Result<T, MacKeyboardError>;
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lookup_basic_keys() {
        let db = &KEY_DATABASE;
        assert_eq!(db.lookup("A").unwrap().code, 0);
        assert_eq!(db.lookup("Space").unwrap().code, 49);
        assert_eq!(db.lookup("Command").unwrap().code, 55);
    }
    
    #[test]
    fn test_alias_resolution() {
        let db = &KEY_DATABASE;
        assert_eq!(db.lookup("Cmd").unwrap().code, 55);
        assert_eq!(db.lookup("⌘").unwrap().code, 55);
        assert_eq!(db.lookup("Enter").unwrap().code, 36);
    }
    
    #[test]
    fn test_shortcut_parsing() {
        let combo = parse_shortcut("Cmd+Shift+A").unwrap();
        assert_eq!(combo.modifiers.len(), 2);
        assert_eq!(combo.key.name, "A");
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_mcp_lookup_tool() {
    let server = MacKeyboardServer::new().unwrap();
    
    let args = json!({
        "key_name": "Space"
    });
    
    let result = server.call_tool("lookup_keycode", args).await.unwrap();
    assert_eq!(result["found"], true);
    assert_eq!(result["key"]["code"], 49);
}
```

## Build Configuration (`Cargo.toml`)

```toml
[package]
name = "mac-keyboard-mcp"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <email@example.com>"]
description = "MCP server for macOS AppleScript key codes"
license = "MIT"
repository = "https://github.com/username/mac-keyboard-mcp"
keywords = ["mcp", "macos", "applescript", "keyboard", "automation"]
categories = ["api-bindings", "os::macos-apis"]

[dependencies]
rmcp = { version = "0.2.0", features = ["server"] }
tokio = { version = "1.38", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
async-trait = "0.1"
once_cell = "1.19"
fuzzy-matcher = "0.3"
thiserror = "2.0"
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
tokio-test = "0.4"
assert_json_diff = "2.0"

[profile.release]
lto = true
codegen-units = 1
opt-level = 3
strip = true
```

## Deployment Instructions

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

### Installation

```bash
# Install locally
cargo install --path .

# Or install from crates.io (after publishing)
cargo install mac-keyboard-mcp
```

### MCP Client Configuration

Example configuration for Claude Desktop or other MCP clients:

```json
{
  "mcpServers": {
    "mac-keyboard": {
      "command": "mac-keyboard-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

## Performance Optimizations

1. **Lazy Static Loading**
   - Key database loaded once at startup
   - Shared across all requests

2. **Efficient Data Structures**
   - HashMap for O(1) lookups
   - Pre-computed lowercase keys for case-insensitive search
   - Cached alias mappings

3. **Async Operations**
   - Non-blocking I/O for MCP communication
   - Efficient tokio runtime usage

4. **Memory Management**
   - Minimal allocations in hot paths
   - String interning for common key names
   - Borrowed data where possible