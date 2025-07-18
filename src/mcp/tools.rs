use anyhow::Result;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::debug;

use crate::keycode::{KEY_DATABASE, KeyCategory, parse_shortcut, is_secondary_function_query, get_secondary_function};
use crate::search::FuzzySearcher;

/// Tool definition for lookup_keycode
pub fn lookup_keycode_tool() -> Value {
    json!({
        "name": "lookup_keycode",
        "description": "Find AppleScript key code for a specific key. Use 'F{n}+' format (e.g., 'F3+') to get secondary function info for F-keys",
        "inputSchema": {
            "type": "object",
            "properties": {
                "key_name": {
                    "type": "string",
                    "description": "Name of the key (e.g., 'A', 'Space', 'F1', 'F3+' for F3's secondary function)"
                },
                "fuzzy": {
                    "type": "boolean",
                    "description": "Enable fuzzy matching for typos",
                    "default": false
                }
            },
            "required": ["key_name"]
        }
    })
}

/// Tool definition for search_keys
pub fn search_keys_tool() -> Value {
    json!({
        "name": "search_keys",
        "description": "Search for keys by pattern or category",
        "inputSchema": {
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search term (partial match supported)"
                },
                "category": {
                    "type": "string",
                    "description": "Filter by category (letters, numbers, function_keys, etc.)",
                    "enum": ["letters", "numbers", "function_keys", "modifier_keys", 
                             "navigation_keys", "special_keys", "numpad_keys", "punctuation"]
                }
            }
        }
    })
}

/// Tool definition for get_key_combinations
pub fn get_key_combinations_tool() -> Value {
    json!({
        "name": "get_key_combinations",
        "description": "Generate key code sequences for shortcuts",
        "inputSchema": {
            "type": "object",
            "properties": {
                "shortcut": {
                    "type": "string",
                    "description": "Shortcut string (e.g., 'Cmd+A', 'Ctrl+Shift+F5', '⌘⇧A')"
                }
            },
            "required": ["shortcut"]
        }
    })
}

/// Tool definition for list_categories
pub fn list_categories_tool() -> Value {
    json!({
        "name": "list_categories",
        "description": "List all available key categories",
        "inputSchema": {
            "type": "object",
            "properties": {}
        }
    })
}

#[derive(Deserialize)]
struct LookupArgs {
    key_name: String,
    #[serde(default)]
    fuzzy: bool,
}

/// Handle lookup_keycode tool call
pub async fn handle_lookup(args: Value) -> Result<Value> {
    let args: LookupArgs = serde_json::from_value(args)?;
    debug!("Looking up key: {}", args.key_name);
    
    // Check if this is a secondary function query (e.g., "F3+")
    if let Some(f_key) = is_secondary_function_query(&args.key_name) {
        if let Some(secondary) = get_secondary_function(f_key) {
            // Also get the primary F-key info
            let primary = KEY_DATABASE.lookup(f_key).unwrap();
            
            return Ok(json!({
                "found": true,
                "query_type": "secondary_function",
                "f_key": {
                    "name": f_key,
                    "primary_function": {
                        "name": primary.name,
                        "code": primary.code,
                        "description": format!("Traditional {} function key", f_key),
                    },
                    "secondary_function": {
                        "name": secondary.name,
                        "code": secondary.keycode,
                        "description": secondary.description,
                    },
                    "note": format!(
                        "On modern Mac keyboards, pressing {} triggers '{}' by default. \
                        Use Fn+{} to get the traditional {} function.",
                        f_key, secondary.name, f_key, f_key
                    ),
                }
            }));
        }
    }
    
    // Regular lookup
    if let Some(keycode) = KEY_DATABASE.lookup(&args.key_name) {
        Ok(json!({
            "found": true,
            "key": {
                "name": keycode.name,
                "code": keycode.code,
                "category": keycode.category.to_string(),
                "aliases": keycode.aliases,
            }
        }))
    } else if args.fuzzy {
        // Try fuzzy search
        let searcher = FuzzySearcher::new();
        let results = searcher.search_keys(&args.key_name, 5);
        
        Ok(json!({
            "found": false,
            "suggestions": results.into_iter().map(|(key, score)| {
                json!({
                    "name": key.name,
                    "code": key.code,
                    "score": score,
                })
            }).collect::<Vec<_>>()
        }))
    } else {
        Ok(json!({
            "found": false,
            "message": format!("Key '{}' not found. Try enabling fuzzy search.", args.key_name),
        }))
    }
}

#[derive(Deserialize)]
struct SearchArgs {
    query: Option<String>,
    category: Option<String>,
}

/// Handle search_keys tool call
pub async fn handle_search(args: Value) -> Result<Value> {
    let args: SearchArgs = serde_json::from_value(args)?;
    debug!("Searching keys: query={:?}, category={:?}", args.query, args.category);
    
    let mut results = Vec::new();
    
    if let Some(category_str) = args.category {
        // Search by category
        if let Ok(category) = category_str.parse::<KeyCategory>() {
            let keys = KEY_DATABASE.by_category(category);
            for key in keys {
                if let Some(query) = &args.query {
                    // Filter by query within category
                    if key.name.to_lowercase().contains(&query.to_lowercase()) {
                        results.push(key);
                    }
                } else {
                    // All keys in category
                    results.push(key);
                }
            }
        } else {
            return Ok(json!({
                "error": format!("Invalid category: {}", category_str),
                "valid_categories": ["letters", "numbers", "function_keys", "modifier_keys", 
                                   "navigation_keys", "special_keys", "numpad_keys", "punctuation"]
            }));
        }
    } else if let Some(query) = args.query {
        // Search all keys by query
        results = KEY_DATABASE.search(&query, true);
    } else {
        // No filters - return all keys
        results = KEY_DATABASE.all_keys();
    }
    
    Ok(json!({
        "count": results.len(),
        "keys": results.into_iter().map(|key| {
            json!({
                "name": key.name,
                "code": key.code,
                "category": key.category.to_string(),
            })
        }).collect::<Vec<_>>()
    }))
}

#[derive(Deserialize)]
struct CombinationArgs {
    shortcut: String,
}

/// Handle get_key_combinations tool call
pub async fn handle_combinations(args: Value) -> Result<Value> {
    let args: CombinationArgs = serde_json::from_value(args)?;
    debug!("Parsing shortcut: {}", args.shortcut);
    
    match parse_shortcut(&args.shortcut) {
        Ok(combination) => {
            Ok(json!({
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
            }))
        }
        Err(e) => {
            Ok(json!({
                "success": false,
                "error": e.to_string(),
                "hint": "Use format like 'Cmd+A', 'Ctrl+Shift+F5', or '⌘⇧A'"
            }))
        }
    }
}

/// Handle list_categories tool call
pub async fn handle_categories(_args: Value) -> Result<Value> {
    let categories = KEY_DATABASE.categories();
    
    Ok(json!({
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
    }))
}