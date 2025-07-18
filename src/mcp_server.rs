use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::keycode::{KEY_DATABASE, KeyCategory, parse_shortcut};
use crate::search::FuzzySearcher;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct LookupKeyCodeRequest {
    /// Name of the key (e.g., 'A', 'Space', 'Command', 'F1')
    pub key_name: String,
    /// Enable fuzzy matching for typos
    #[serde(default)]
    pub fuzzy: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SearchKeysRequest {
    /// Search term (partial match supported)
    #[serde(default)]
    pub query: Option<String>,
    /// Filter by category (letters, numbers, function_keys, etc.)
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetKeyCombinationRequest {
    /// Shortcut string (e.g., 'Cmd+A', 'Ctrl+Shift+F5', '⌘⇧A')
    pub shortcut: String,
}

#[derive(Clone)]
pub struct MacKeyboardServer {
    tool_router: ToolRouter<MacKeyboardServer>,
}

#[tool_router]
impl MacKeyboardServer {
    pub fn new() -> Self {
        // Initialize the key database on first access
        let _ = &*KEY_DATABASE;
        
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Find AppleScript key code for a specific key")]
    fn lookup_keycode(
        &self,
        Parameters(LookupKeyCodeRequest { key_name, fuzzy }): Parameters<LookupKeyCodeRequest>,
    ) -> Result<CallToolResult, McpError> {
        tracing::debug!("Looking up key: {}", key_name);
        
        if let Some(keycode) = KEY_DATABASE.lookup(&key_name) {
            let result = json!({
                "found": true,
                "key": {
                    "name": keycode.name,
                    "code": keycode.code,
                    "category": keycode.category.to_string(),
                    "aliases": keycode.aliases,
                }
            });
            
            Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&result).unwrap()
            )]))
        } else if fuzzy {
            // Try fuzzy search
            let searcher = FuzzySearcher::new();
            let results = searcher.search_keys(&key_name, 5);
            
            let suggestions: Vec<Value> = results.into_iter().map(|(key, score)| {
                json!({
                    "name": key.name,
                    "code": key.code,
                    "score": score,
                })
            }).collect();
            
            let result = json!({
                "found": false,
                "suggestions": suggestions,
            });
            
            Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&result).unwrap()
            )]))
        } else {
            let result = json!({
                "found": false,
                "message": format!("Key '{}' not found. Try enabling fuzzy search.", key_name),
            });
            
            Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&result).unwrap()
            )]))
        }
    }

    #[tool(description = "Search for keys by pattern or category")]
    fn search_keys(
        &self,
        Parameters(SearchKeysRequest { query, category }): Parameters<SearchKeysRequest>,
    ) -> Result<CallToolResult, McpError> {
        tracing::debug!("Searching keys: query={:?}, category={:?}", query, category);
        
        let mut results = Vec::new();
        
        if let Some(category_str) = category {
            // Search by category
            if let Ok(cat) = category_str.parse::<KeyCategory>() {
                let keys = KEY_DATABASE.by_category(cat);
                for key in keys {
                    if let Some(q) = &query {
                        // Filter by query within category
                        if key.name.to_lowercase().contains(&q.to_lowercase()) {
                            results.push(key);
                        }
                    } else {
                        // All keys in category
                        results.push(key);
                    }
                }
            } else {
                let error = json!({
                    "error": format!("Invalid category: {}", category_str),
                    "valid_categories": ["letters", "numbers", "function_keys", "modifier_keys", 
                                       "navigation_keys", "special_keys", "numpad_keys", "punctuation"]
                });
                
                return Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&error).unwrap()
                )]));
            }
        } else if let Some(q) = query {
            // Search all keys by query
            results = KEY_DATABASE.search(&q, true);
        } else {
            // No filters - return all keys
            results = KEY_DATABASE.all_keys();
        }
        
        let response = json!({
            "count": results.len(),
            "keys": results.into_iter().map(|key| {
                json!({
                    "name": key.name,
                    "code": key.code,
                    "category": key.category.to_string(),
                })
            }).collect::<Vec<_>>()
        });
        
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap()
        )]))
    }

    #[tool(description = "Generate key code sequences for shortcuts")]
    fn get_key_combinations(
        &self,
        Parameters(GetKeyCombinationRequest { shortcut }): Parameters<GetKeyCombinationRequest>,
    ) -> Result<CallToolResult, McpError> {
        tracing::debug!("Parsing shortcut: {}", shortcut);
        
        match parse_shortcut(&shortcut) {
            Ok(combination) => {
                let response = json!({
                    "success": true,
                    "shortcut": combination.to_string(),
                    "keycodes": combination.to_keycodes(),
                    "modifiers": combination.modifiers.iter().map(|m| {
                        json!({
                            "name": m.name,
                            "code": m.code,
                        })
                    }).collect::<Vec<_>>(),
                    "key": {
                        "name": combination.key.name,
                        "code": combination.key.code,
                    }
                });
                
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap()
                )]))
            }
            Err(e) => {
                let response = json!({
                    "success": false,
                    "error": e.to_string(),
                    "hint": "Use format like 'Cmd+A', 'Ctrl+Shift+F5', or '⌘⇧A'"
                });
                
                Ok(CallToolResult::success(vec![Content::text(
                    serde_json::to_string_pretty(&response).unwrap()
                )]))
            }
        }
    }

    #[tool(description = "List all available key categories")]
    fn list_categories(&self) -> Result<CallToolResult, McpError> {
        let categories = KEY_DATABASE.categories();
        
        let response = json!({
            "categories": categories.into_iter().map(|(cat, count)| {
                json!({
                    "name": cat.to_string(),
                    "count": count,
                    "description": match cat {
                        KeyCategory::Letters => "Letter keys A-Z",
                        KeyCategory::Numbers => "Number keys 0-9",
                        KeyCategory::FunctionKeys => "Function keys F1-F20",
                        KeyCategory::ModifierKeys => "Modifier keys (Command, Shift, Option, Control)",
                        KeyCategory::NavigationKeys => "Navigation keys (arrows, page up/down, home, end)",
                        KeyCategory::SpecialKeys => "Special keys (space, tab, return, delete, escape)",
                        KeyCategory::NumpadKeys => "Numeric keypad keys",
                        KeyCategory::Punctuation => "Punctuation and symbol keys",
                    }
                })
            }).collect::<Vec<_>>()
        });
        
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap()
        )]))
    }
    
    fn _create_resource(&self, uri: &str, name: &str, _description: &str) -> Resource {
        // TODO: Add description when supported by SDK
        RawResource::new(uri, name.to_string())
            .no_annotation()
    }
}

#[tool_handler]
impl ServerHandler for MacKeyboardServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "mac-keyboard-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some(
                "This server provides AppleScript key codes for macOS automation. \
                Use 'lookup_keycode' to find specific keys, 'search_keys' to browse categories, \
                'get_key_combinations' to parse shortcuts like 'Cmd+A', and 'list_categories' \
                to see all available key categories.".to_string()
            ),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                self._create_resource(
                    "keycode://all",
                    "All Key Codes",
                    "Complete database of AppleScript key codes"
                ),
                self._create_resource(
                    "keycode://aliases", 
                    "Key Aliases",
                    "Common aliases and alternative names for keys"
                ),
            ],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        match uri.as_str() {
            "keycode://all" => {
                let all_keys = KEY_DATABASE.all_keys();
                let mut by_category = std::collections::HashMap::new();
                
                for key in all_keys {
                    by_category
                        .entry(key.category.to_string())
                        .or_insert_with(Vec::new)
                        .push(json!({
                            "name": key.name,
                            "code": key.code,
                            "aliases": key.aliases,
                        }));
                }
                
                let content = serde_json::to_string_pretty(&json!({
                    "total": KEY_DATABASE.all_keys().len(),
                    "categories": by_category,
                })).unwrap();
                
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(content, uri)],
                })
            }
            "keycode://aliases" => {
                let aliases_map = KEY_DATABASE.all_keys()
                    .iter()
                    .flat_map(|key| {
                        key.aliases.iter().map(move |alias| {
                            (alias.clone(), json!({
                                "canonical": key.name,
                                "code": key.code,
                            }))
                        })
                    })
                    .collect::<std::collections::HashMap<_, _>>();
                
                let content = serde_json::to_string_pretty(&json!({
                    "count": aliases_map.len(),
                    "aliases": aliases_map,
                })).unwrap();
                
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(content, uri)],
                })
            }
            _ => Err(McpError::resource_not_found(
                "Resource not found",
                Some(json!({
                    "uri": uri,
                    "available": ["keycode://all", "keycode://aliases"]
                })),
            )),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        tracing::info!("Client initialized");
        Ok(self.get_info())
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        // No prompts for this server
        Ok(ListPromptsResult {
            next_cursor: None,
            prompts: vec![],
        })
    }

    async fn get_prompt(
        &self,
        GetPromptRequestParam { name, .. }: GetPromptRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        Err(McpError::invalid_params(
            "Prompt not found",
            Some(json!({ "prompt": name }))
        ))
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        // No resource templates for this server
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: Vec::new(),
        })
    }
}