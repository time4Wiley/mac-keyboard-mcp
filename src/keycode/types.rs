use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Represents a single keyboard key with its AppleScript code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyCode {
    /// Display name of the key
    pub name: String,
    /// AppleScript key code
    pub code: u16,
    /// Category this key belongs to
    pub category: KeyCategory,
    /// Alternative names/aliases for this key
    pub aliases: Vec<String>,
}

/// Categories for organizing keyboard keys
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum KeyCategory {
    /// Letter keys A-Z
    Letters,
    /// Number keys 0-9
    Numbers,
    /// Function keys F1-F20
    FunctionKeys,
    /// Modifier keys (Cmd, Shift, Option, Control)
    ModifierKeys,
    /// Navigation keys (arrows, page up/down, home, end)
    NavigationKeys,
    /// Special keys (space, tab, return, delete, escape)
    SpecialKeys,
    /// Numeric keypad keys
    NumpadKeys,
    /// Punctuation and symbol keys
    Punctuation,
}

impl KeyCode {
    /// Create a new KeyCode
    pub fn new(name: impl Into<String>, code: u16, category: KeyCategory) -> Self {
        Self {
            name: name.into(),
            code,
            category,
            aliases: Vec::new(),
        }
    }
    
    /// Add an alias to this key
    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }
    
    /// Add multiple aliases
    pub fn with_aliases(mut self, aliases: Vec<String>) -> Self {
        self.aliases.extend(aliases);
        self
    }
    
    /// Check if this key matches a given name (case-insensitive)
    pub fn matches(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        
        // Check main name
        if self.name.to_lowercase() == query_lower {
            return true;
        }
        
        // Check aliases
        self.aliases.iter()
            .any(|alias| alias.to_lowercase() == query_lower)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keycode_creation() {
        let key = KeyCode::new("A", 0, KeyCategory::Letters);
        assert_eq!(key.name, "A");
        assert_eq!(key.code, 0);
        assert_eq!(key.category, KeyCategory::Letters);
        assert!(key.aliases.is_empty());
    }
    
    #[test]
    fn test_keycode_with_aliases() {
        let key = KeyCode::new("Command", 55, KeyCategory::ModifierKeys)
            .with_alias("Cmd")
            .with_alias("⌘");
        
        assert_eq!(key.aliases.len(), 2);
        assert!(key.matches("Command"));
        assert!(key.matches("cmd"));
        assert!(key.matches("⌘"));
    }
    
    #[test]
    fn test_category_display() {
        assert_eq!(KeyCategory::Letters.to_string(), "letters");
        assert_eq!(KeyCategory::FunctionKeys.to_string(), "function_keys");
    }
}