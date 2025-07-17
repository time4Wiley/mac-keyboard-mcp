use thiserror::Error;
use crate::keycode::{KeyCode, KeyCategory, KEY_DATABASE};

/// Represents a keyboard shortcut combination
#[derive(Debug, Clone, PartialEq)]
pub struct KeyCombination {
    /// Modifier keys in the combination (Command, Shift, Option, Control)
    pub modifiers: Vec<KeyCode>,
    /// The main key (non-modifier)
    pub key: KeyCode,
}

/// Errors that can occur during shortcut parsing
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Empty shortcut string")]
    EmptyShortcut,
    
    #[error("Unknown key: {0}")]
    UnknownKey(String),
    
    #[error("Multiple non-modifier keys in shortcut")]
    MultipleMainKeys,
    
    #[error("No main key in shortcut (only modifiers)")]
    NoMainKey,
    
    #[error("Invalid shortcut format")]
    InvalidFormat,
}

impl KeyCombination {
    /// Convert the combination to a list of key codes
    pub fn to_keycodes(&self) -> Vec<u16> {
        let mut codes = Vec::new();
        
        // Add modifier codes first
        for modifier in &self.modifiers {
            codes.push(modifier.code);
        }
        
        // Add main key code
        codes.push(self.key.code);
        
        codes
    }
    
    /// Get a human-readable representation
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        
        // Add modifiers in standard order
        let modifier_order = ["Command", "Control", "Option", "Shift"];
        for name in &modifier_order {
            if self.modifiers.iter().any(|m| m.name == *name) {
                parts.push(name.to_string());
            }
        }
        
        // Add main key
        parts.push(self.key.name.clone());
        
        parts.join("+")
    }
}

/// Parse a keyboard shortcut string into a KeyCombination
/// 
/// Supports various formats:
/// - "Cmd+A", "Command+A", "⌘A"
/// - "Cmd+Shift+S", "⌘⇧S"
/// - "Ctrl+Option+Delete"
/// 
/// Separators can be '+', '-', or no separator for symbols
pub fn parse_shortcut(shortcut: &str) -> Result<KeyCombination, ParseError> {
    if shortcut.is_empty() {
        return Err(ParseError::EmptyShortcut);
    }
    
    // Split by common separators
    let parts = split_shortcut(shortcut);
    
    if parts.is_empty() {
        return Err(ParseError::EmptyShortcut);
    }
    
    let mut modifiers = Vec::new();
    let mut main_key = None;
    
    for part in parts {
        if let Some(keycode) = KEY_DATABASE.lookup(&part) {
            match keycode.category {
                KeyCategory::ModifierKeys => {
                    // Avoid duplicates
                    if !modifiers.iter().any(|m: &KeyCode| m.code == keycode.code) {
                        modifiers.push(keycode.clone());
                    }
                }
                _ => {
                    if main_key.is_some() {
                        return Err(ParseError::MultipleMainKeys);
                    }
                    main_key = Some(keycode.clone());
                }
            }
        } else {
            return Err(ParseError::UnknownKey(part));
        }
    }
    
    let key = main_key.ok_or(ParseError::NoMainKey)?;
    
    Ok(KeyCombination { modifiers, key })
}

/// Split a shortcut string into parts
fn split_shortcut(shortcut: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut chars = shortcut.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            '+' | '-' => {
                // Separator found
                if !current.is_empty() {
                    parts.push(current.trim().to_string());
                    current.clear();
                }
            }
            '⌘' | '⌃' | '⌥' | '⇧' => {
                // Symbol modifier
                if !current.is_empty() {
                    parts.push(current.trim().to_string());
                    current.clear();
                }
                parts.push(ch.to_string());
            }
            _ => {
                current.push(ch);
            }
        }
    }
    
    // Don't forget the last part
    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }
    
    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_shortcut() {
        let combo = parse_shortcut("Cmd+A").unwrap();
        assert_eq!(combo.modifiers.len(), 1);
        assert_eq!(combo.modifiers[0].name, "Command");
        assert_eq!(combo.key.name, "A");
    }
    
    #[test]
    fn test_parse_multiple_modifiers() {
        let combo = parse_shortcut("Cmd+Shift+S").unwrap();
        assert_eq!(combo.modifiers.len(), 2);
        assert!(combo.modifiers.iter().any(|m| m.name == "Command"));
        assert!(combo.modifiers.iter().any(|m| m.name == "Shift"));
        assert_eq!(combo.key.name, "S");
    }
    
    #[test]
    fn test_parse_symbol_shortcut() {
        let combo = parse_shortcut("⌘⇧A").unwrap();
        assert_eq!(combo.modifiers.len(), 2);
        assert!(combo.modifiers.iter().any(|m| m.name == "Command"));
        assert!(combo.modifiers.iter().any(|m| m.name == "Shift"));
        assert_eq!(combo.key.name, "A");
    }
    
    #[test]
    fn test_parse_with_different_separators() {
        let combo1 = parse_shortcut("Cmd+A").unwrap();
        let combo2 = parse_shortcut("Cmd-A").unwrap();
        assert_eq!(combo1.to_keycodes(), combo2.to_keycodes());
    }
    
    #[test]
    fn test_parse_errors() {
        assert!(matches!(parse_shortcut(""), Err(ParseError::EmptyShortcut)));
        assert!(matches!(parse_shortcut("XYZ"), Err(ParseError::UnknownKey(_))));
        assert!(matches!(parse_shortcut("Cmd+Shift"), Err(ParseError::NoMainKey)));
        assert!(matches!(parse_shortcut("A+B"), Err(ParseError::MultipleMainKeys)));
    }
    
    #[test]
    fn test_to_string() {
        let combo = parse_shortcut("Shift+Cmd+A").unwrap();
        // Should normalize order to Command+Shift+A
        assert_eq!(combo.to_string(), "Command+Shift+A");
    }
}