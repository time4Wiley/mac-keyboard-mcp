use anyhow::Result;
use serde_json::{json, Value};
use tracing::debug;

use crate::keycode::{KEY_DATABASE, KeyCategory};

/// Resource definition for all keycodes
pub fn all_keycodes_resource() -> Value {
    json!({
        "uri": "keycode://all",
        "name": "All Key Codes",
        "description": "Complete database of AppleScript key codes",
        "mimeType": "application/json"
    })
}

/// Resource definition for aliases
pub fn aliases_resource() -> Value {
    json!({
        "uri": "keycode://aliases",
        "name": "Key Aliases",
        "description": "Common aliases and alternative names for keys",
        "mimeType": "application/json"
    })
}

/// Resource definitions for each category
pub fn category_resources() -> Vec<Value> {
    vec![
        json!({
            "uri": "keycode://category/letters",
            "name": "Letter Keys",
            "description": "Letter keys A-Z",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "keycode://category/numbers",
            "name": "Number Keys",
            "description": "Number keys 0-9",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "keycode://category/function_keys",
            "name": "Function Keys",
            "description": "Function keys F1-F20",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "keycode://category/modifier_keys",
            "name": "Modifier Keys",
            "description": "Modifier keys (Command, Shift, Option, Control)",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "keycode://category/navigation_keys",
            "name": "Navigation Keys",
            "description": "Navigation keys (arrows, page up/down, home, end)",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "keycode://category/special_keys",
            "name": "Special Keys",
            "description": "Special keys (space, tab, return, delete, escape)",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "keycode://category/numpad_keys",
            "name": "Numpad Keys",
            "description": "Numeric keypad keys",
            "mimeType": "application/json"
        }),
        json!({
            "uri": "keycode://category/punctuation",
            "name": "Punctuation Keys",
            "description": "Punctuation and symbol keys",
            "mimeType": "application/json"
        }),
    ]
}

/// Read resource content by URI
pub async fn read_resource_content(uri: &str) -> Result<Value> {
    debug!("Reading resource: {}", uri);
    
    match uri {
        "keycode://all" => {
            // Return all keys organized by category
            let mut all_keys = json!({});
            
            for (category, _) in KEY_DATABASE.categories() {
                let keys = KEY_DATABASE.by_category(category);
                let category_data: Vec<Value> = keys.into_iter().map(|key| {
                    json!({
                        "name": key.name,
                        "code": key.code,
                        "aliases": key.aliases,
                    })
                }).collect();
                
                all_keys[category.to_string()] = json!(category_data);
            }
            
            Ok(json!({
                "totalKeys": KEY_DATABASE.all_keys().len(),
                "categories": all_keys,
            }))
        }
        
        "keycode://aliases" => {
            // Build alias map
            let mut aliases = json!({});
            
            for key in KEY_DATABASE.all_keys() {
                for alias in &key.aliases {
                    aliases[alias] = json!({
                        "canonical": key.name,
                        "code": key.code,
                    });
                }
            }
            
            Ok(json!({
                "aliases": aliases,
                "count": aliases.as_object().unwrap().len(),
            }))
        }
        
        uri if uri.starts_with("keycode://category/") => {
            let category_name = uri.strip_prefix("keycode://category/")
                .ok_or_else(|| anyhow::anyhow!("Invalid category URI"))?;
            
            let category = category_name.parse::<KeyCategory>()
                .map_err(|_| anyhow::anyhow!("Unknown category: {}", category_name))?;
            
            let keys = KEY_DATABASE.by_category(category);
            let key_data: Vec<Value> = keys.into_iter().map(|key| {
                json!({
                    "name": key.name,
                    "code": key.code,
                    "aliases": key.aliases,
                })
            }).collect();
            
            Ok(json!({
                "category": category.to_string(),
                "count": key_data.len(),
                "keys": key_data,
            }))
        }
        
        _ => Err(anyhow::anyhow!("Unknown resource URI: {}", uri)),
    }
}