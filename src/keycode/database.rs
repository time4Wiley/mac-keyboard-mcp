use once_cell::sync::Lazy;
use std::collections::HashMap;
use anyhow::Result;
use tracing::{debug, info};

use super::types::{KeyCode, KeyCategory};

/// Static key code database loaded at startup
pub static KEY_DATABASE: Lazy<KeyDatabase> = Lazy::new(|| {
    KeyDatabase::load().expect("Failed to load key database")
});

/// Database containing all AppleScript key codes
pub struct KeyDatabase {
    /// Lookup by key name (case-insensitive)
    by_name: HashMap<String, KeyCode>,
    /// Lookup by key code
    by_code: HashMap<u16, KeyCode>,
    /// Keys grouped by category
    by_category: HashMap<KeyCategory, Vec<KeyCode>>,
    /// Alias to canonical name mapping
    aliases: HashMap<String, String>,
}

impl KeyDatabase {
    /// Load the key database from embedded data
    pub fn load() -> Result<Self> {
        info!("Loading key code database...");
        
        let mut by_name = HashMap::new();
        let mut by_code = HashMap::new();
        let mut by_category = HashMap::new();
        let mut aliases = HashMap::new();
        
        // Load all keys
        let all_keys = Self::create_all_keys();
        
        for key in all_keys {
            // Add to name lookup (lowercase for case-insensitive)
            by_name.insert(key.name.to_lowercase(), key.clone());
            
            // Add to code lookup
            by_code.insert(key.code, key.clone());
            
            // Add to category
            by_category
                .entry(key.category)
                .or_insert_with(Vec::new)
                .push(key.clone());
            
            // Process aliases
            for alias in &key.aliases {
                aliases.insert(alias.to_lowercase(), key.name.clone());
            }
        }
        
        // Sort categories for consistent ordering
        for keys in by_category.values_mut() {
            keys.sort_by(|a, b| a.name.cmp(&b.name));
        }
        
        let total_keys = by_name.len();
        let total_aliases = aliases.len();
        info!("Loaded {} keys with {} aliases", total_keys, total_aliases);
        
        Ok(Self {
            by_name,
            by_code,
            by_category,
            aliases,
        })
    }
    
    /// Create all key definitions
    fn create_all_keys() -> Vec<KeyCode> {
        let mut keys = Vec::new();
        
        // Letters A-Z
        keys.extend(vec![
            KeyCode::new("A", 0, KeyCategory::Letters),
            KeyCode::new("B", 11, KeyCategory::Letters),
            KeyCode::new("C", 8, KeyCategory::Letters),
            KeyCode::new("D", 2, KeyCategory::Letters),
            KeyCode::new("E", 14, KeyCategory::Letters),
            KeyCode::new("F", 3, KeyCategory::Letters),
            KeyCode::new("G", 5, KeyCategory::Letters),
            KeyCode::new("H", 4, KeyCategory::Letters),
            KeyCode::new("I", 34, KeyCategory::Letters),
            KeyCode::new("J", 38, KeyCategory::Letters),
            KeyCode::new("K", 40, KeyCategory::Letters),
            KeyCode::new("L", 37, KeyCategory::Letters),
            KeyCode::new("M", 46, KeyCategory::Letters),
            KeyCode::new("N", 45, KeyCategory::Letters),
            KeyCode::new("O", 31, KeyCategory::Letters),
            KeyCode::new("P", 35, KeyCategory::Letters),
            KeyCode::new("Q", 12, KeyCategory::Letters),
            KeyCode::new("R", 15, KeyCategory::Letters),
            KeyCode::new("S", 1, KeyCategory::Letters),
            KeyCode::new("T", 17, KeyCategory::Letters),
            KeyCode::new("U", 32, KeyCategory::Letters),
            KeyCode::new("V", 9, KeyCategory::Letters),
            KeyCode::new("W", 13, KeyCategory::Letters),
            KeyCode::new("X", 7, KeyCategory::Letters),
            KeyCode::new("Y", 16, KeyCategory::Letters),
            KeyCode::new("Z", 6, KeyCategory::Letters),
        ]);
        
        // Numbers 0-9
        keys.extend(vec![
            KeyCode::new("0", 29, KeyCategory::Numbers),
            KeyCode::new("1", 18, KeyCategory::Numbers),
            KeyCode::new("2", 19, KeyCategory::Numbers),
            KeyCode::new("3", 20, KeyCategory::Numbers),
            KeyCode::new("4", 21, KeyCategory::Numbers),
            KeyCode::new("5", 23, KeyCategory::Numbers),
            KeyCode::new("6", 22, KeyCategory::Numbers),
            KeyCode::new("7", 26, KeyCategory::Numbers),
            KeyCode::new("8", 28, KeyCategory::Numbers),
            KeyCode::new("9", 25, KeyCategory::Numbers),
        ]);
        
        // Function keys (corrected based on keyboard image)
        keys.extend(vec![
            KeyCode::new("F1", 122, KeyCategory::FunctionKeys),
            KeyCode::new("F2", 120, KeyCategory::FunctionKeys),
            KeyCode::new("F3", 99, KeyCategory::FunctionKeys),
            KeyCode::new("F4", 118, KeyCategory::FunctionKeys),
            KeyCode::new("F5", 96, KeyCategory::FunctionKeys),
            KeyCode::new("F6", 97, KeyCategory::FunctionKeys),
            KeyCode::new("F7", 98, KeyCategory::FunctionKeys),
            KeyCode::new("F8", 100, KeyCategory::FunctionKeys),
            KeyCode::new("F9", 101, KeyCategory::FunctionKeys),
            KeyCode::new("F10", 109, KeyCategory::FunctionKeys),
            KeyCode::new("F11", 103, KeyCategory::FunctionKeys),
            KeyCode::new("F12", 111, KeyCategory::FunctionKeys),
            KeyCode::new("F13", 105, KeyCategory::FunctionKeys)
                .with_alias("Keyboard Backlight Down"),  // On some Macs
            KeyCode::new("F14", 107, KeyCategory::FunctionKeys)
                .with_alias("Brightness Down"),  // Fn+F1 on compact keyboards
            KeyCode::new("F15", 113, KeyCategory::FunctionKeys)
                .with_alias("Brightness Up"),    // Fn+F2 on compact keyboards
            KeyCode::new("F16", 106, KeyCategory::FunctionKeys)
                .with_alias("Keyboard Backlight Up"),  // On some Macs
            KeyCode::new("F17", 160, KeyCategory::FunctionKeys)  // This was missing!
                .with_alias("Mission Control"),  // Fn+F3 on some keyboards
            KeyCode::new("F18", 131, KeyCategory::FunctionKeys)  // Corrected from 79
                .with_alias("Launchpad"),        // Fn+F4 on some keyboards
            KeyCode::new("F19", 80, KeyCategory::FunctionKeys),
            KeyCode::new("F20", 90, KeyCategory::FunctionKeys),
        ]);
        
        // Modifier keys
        keys.extend(vec![
            KeyCode::new("Command", 55, KeyCategory::ModifierKeys)
                .with_aliases(vec!["Cmd".to_string(), "⌘".to_string()]),
            KeyCode::new("Shift", 56, KeyCategory::ModifierKeys)
                .with_alias("⇧"),
            KeyCode::new("ShiftLeft", 56, KeyCategory::ModifierKeys)
                .with_alias("Left Shift"),
            KeyCode::new("ShiftRight", 60, KeyCategory::ModifierKeys)
                .with_alias("Right Shift"),
            KeyCode::new("Option", 58, KeyCategory::ModifierKeys)
                .with_aliases(vec!["Opt".to_string(), "Alt".to_string(), "⌥".to_string()]),
            KeyCode::new("OptionLeft", 58, KeyCategory::ModifierKeys)
                .with_alias("Left Option"),
            KeyCode::new("OptionRight", 61, KeyCategory::ModifierKeys)
                .with_alias("Right Option"),
            KeyCode::new("Control", 59, KeyCategory::ModifierKeys)
                .with_aliases(vec!["Ctrl".to_string(), "⌃".to_string()]),
            KeyCode::new("ControlLeft", 59, KeyCategory::ModifierKeys)
                .with_alias("Left Control"),
            KeyCode::new("ControlRight", 62, KeyCategory::ModifierKeys)
                .with_alias("Right Control"),
            KeyCode::new("CapsLock", 57, KeyCategory::ModifierKeys)
                .with_alias("Caps Lock"),
            KeyCode::new("Fn", 63, KeyCategory::ModifierKeys)
                .with_alias("Function"),
        ]);
        
        // Navigation keys
        keys.extend(vec![
            KeyCode::new("LeftArrow", 123, KeyCategory::NavigationKeys)
                .with_aliases(vec!["Left".to_string(), "←".to_string()]),
            KeyCode::new("RightArrow", 124, KeyCategory::NavigationKeys)
                .with_aliases(vec!["Right".to_string(), "→".to_string()]),
            KeyCode::new("UpArrow", 126, KeyCategory::NavigationKeys)
                .with_aliases(vec!["Up".to_string(), "↑".to_string()]),
            KeyCode::new("DownArrow", 125, KeyCategory::NavigationKeys)
                .with_aliases(vec!["Down".to_string(), "↓".to_string()]),
            KeyCode::new("PageUp", 116, KeyCategory::NavigationKeys)
                .with_alias("Page Up"),
            KeyCode::new("PageDown", 121, KeyCategory::NavigationKeys)
                .with_alias("Page Down"),
            KeyCode::new("Home", 115, KeyCategory::NavigationKeys),
            KeyCode::new("End", 119, KeyCategory::NavigationKeys),
        ]);
        
        // Special keys
        keys.extend(vec![
            KeyCode::new("Space", 49, KeyCategory::SpecialKeys)
                .with_aliases(vec!["Spacebar".to_string(), "Space Bar".to_string()]),
            KeyCode::new("Return", 36, KeyCategory::SpecialKeys)
                .with_aliases(vec!["Enter".to_string(), "↵".to_string()]),
            KeyCode::new("Tab", 48, KeyCategory::SpecialKeys)
                .with_alias("⇥"),
            KeyCode::new("Delete", 51, KeyCategory::SpecialKeys)
                .with_aliases(vec!["Backspace".to_string(), "Del".to_string()]),
            KeyCode::new("ForwardDelete", 117, KeyCategory::SpecialKeys)
                .with_alias("Forward Delete"),
            KeyCode::new("Escape", 53, KeyCategory::SpecialKeys)
                .with_alias("Esc"),
            KeyCode::new("Clear", 71, KeyCategory::SpecialKeys),
            KeyCode::new("Help", 114, KeyCategory::SpecialKeys),
            KeyCode::new("Mute", 74, KeyCategory::SpecialKeys),
            KeyCode::new("VolumeUp", 72, KeyCategory::SpecialKeys)
                .with_alias("Volume Up"),
            KeyCode::new("VolumeDown", 73, KeyCategory::SpecialKeys)
                .with_alias("Volume Down"),
        ]);
        
        // Numpad keys
        keys.extend(vec![
            KeyCode::new("Numpad0", 82, KeyCategory::NumpadKeys),
            KeyCode::new("Numpad1", 83, KeyCategory::NumpadKeys),
            KeyCode::new("Numpad2", 84, KeyCategory::NumpadKeys),
            KeyCode::new("Numpad3", 85, KeyCategory::NumpadKeys),
            KeyCode::new("Numpad4", 86, KeyCategory::NumpadKeys),
            KeyCode::new("Numpad5", 87, KeyCategory::NumpadKeys),
            KeyCode::new("Numpad6", 88, KeyCategory::NumpadKeys),
            KeyCode::new("Numpad7", 89, KeyCategory::NumpadKeys),
            KeyCode::new("Numpad8", 91, KeyCategory::NumpadKeys),
            KeyCode::new("Numpad9", 92, KeyCategory::NumpadKeys),
            KeyCode::new("NumpadClear", 71, KeyCategory::NumpadKeys),
            KeyCode::new("NumpadDecimal", 65, KeyCategory::NumpadKeys),
            KeyCode::new("NumpadDivide", 75, KeyCategory::NumpadKeys),
            KeyCode::new("NumpadEnter", 76, KeyCategory::NumpadKeys),
            KeyCode::new("NumpadEquals", 81, KeyCategory::NumpadKeys),
            KeyCode::new("NumpadMinus", 78, KeyCategory::NumpadKeys),
            KeyCode::new("NumpadMultiply", 67, KeyCategory::NumpadKeys),
            KeyCode::new("NumpadPlus", 69, KeyCategory::NumpadKeys),
        ]);
        
        // Punctuation
        keys.extend(vec![
            KeyCode::new("Grave", 50, KeyCategory::Punctuation)
                .with_aliases(vec!["`".to_string(), "~".to_string()]),
            KeyCode::new("Minus", 27, KeyCategory::Punctuation)
                .with_aliases(vec!["-".to_string(), "_".to_string()]),
            KeyCode::new("Equal", 24, KeyCategory::Punctuation)
                .with_aliases(vec!["=".to_string(), "+".to_string()]),
            KeyCode::new("LeftBracket", 33, KeyCategory::Punctuation)
                .with_aliases(vec!["[".to_string(), "{".to_string()]),
            KeyCode::new("RightBracket", 30, KeyCategory::Punctuation)
                .with_aliases(vec!["]".to_string(), "}".to_string()]),
            KeyCode::new("Backslash", 42, KeyCategory::Punctuation)
                .with_aliases(vec!["\\".to_string(), "|".to_string()]),
            KeyCode::new("Semicolon", 41, KeyCategory::Punctuation)
                .with_aliases(vec![";".to_string(), ":".to_string()]),
            KeyCode::new("Quote", 39, KeyCategory::Punctuation)
                .with_aliases(vec!["'".to_string(), "\"".to_string()]),
            KeyCode::new("Comma", 43, KeyCategory::Punctuation)
                .with_aliases(vec![",".to_string(), "<".to_string()]),
            KeyCode::new("Period", 47, KeyCategory::Punctuation)
                .with_aliases(vec![".".to_string(), ">".to_string()]),
            KeyCode::new("Slash", 44, KeyCategory::Punctuation)
                .with_aliases(vec!["/".to_string(), "?".to_string()]),
        ]);
        
        keys
    }
    
    /// Look up a key by name (case-insensitive)
    pub fn lookup(&self, name: &str) -> Option<&KeyCode> {
        let name_lower = name.to_lowercase();
        
        // Try direct lookup
        if let Some(key) = self.by_name.get(&name_lower) {
            debug!("Found key '{}' by direct lookup", name);
            return Some(key);
        }
        
        // Try alias lookup
        if let Some(canonical_name) = self.aliases.get(&name_lower) {
            debug!("Found key '{}' via alias '{}'", canonical_name, name);
            return self.by_name.get(&canonical_name.to_lowercase());
        }
        
        debug!("Key '{}' not found", name);
        None
    }
    
    /// Look up a key by code
    pub fn lookup_by_code(&self, code: u16) -> Option<&KeyCode> {
        self.by_code.get(&code)
    }
    
    /// Get all keys in a category
    pub fn by_category(&self, category: KeyCategory) -> Vec<&KeyCode> {
        self.by_category
            .get(&category)
            .map(|keys| keys.iter().collect())
            .unwrap_or_default()
    }
    
    /// Get all keys
    pub fn all_keys(&self) -> Vec<&KeyCode> {
        self.by_name.values().collect()
    }
    
    /// Get all categories with their key counts
    pub fn categories(&self) -> Vec<(KeyCategory, usize)> {
        let mut categories: Vec<_> = self.by_category
            .iter()
            .map(|(cat, keys)| (*cat, keys.len()))
            .collect();
        
        categories.sort_by_key(|(cat, _)| format!("{:?}", cat));
        categories
    }
    
    /// Search for keys matching a query
    pub fn search(&self, query: &str, fuzzy: bool) -> Vec<&KeyCode> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        // First, try exact matches
        for key in self.by_name.values() {
            if key.matches(query) {
                results.push(key);
            }
        }
        
        // If no exact matches and fuzzy is enabled, do fuzzy search
        if results.is_empty() && fuzzy {
            // This will be implemented with the fuzzy search module
            // For now, do simple substring matching
            for key in self.by_name.values() {
                if key.name.to_lowercase().contains(&query_lower) {
                    results.push(key);
                }
            }
        }
        
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_loading() {
        let db = KeyDatabase::load().unwrap();
        
        // Check some basic keys exist
        assert!(db.lookup("A").is_some());
        assert!(db.lookup("Space").is_some());
        assert!(db.lookup("Command").is_some());
    }
    
    #[test]
    fn test_case_insensitive_lookup() {
        let db = &KEY_DATABASE;
        
        assert_eq!(db.lookup("a").unwrap().code, 0);
        assert_eq!(db.lookup("A").unwrap().code, 0);
        assert_eq!(db.lookup("SPACE").unwrap().code, 49);
    }
    
    #[test]
    fn test_alias_lookup() {
        let db = &KEY_DATABASE;
        
        assert_eq!(db.lookup("Cmd").unwrap().name, "Command");
        assert_eq!(db.lookup("⌘").unwrap().name, "Command");
        assert_eq!(db.lookup("Enter").unwrap().name, "Return");
    }
    
    #[test]
    fn test_category_lookup() {
        let db = &KEY_DATABASE;
        
        let letters = db.by_category(KeyCategory::Letters);
        assert_eq!(letters.len(), 26);
        
        let numbers = db.by_category(KeyCategory::Numbers);
        assert_eq!(numbers.len(), 10);
    }
}