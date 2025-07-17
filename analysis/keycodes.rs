// Auto-generated AppleScript key code mappings
use std::collections::HashMap;

pub fn create_keycode_mappings() -> HashMap<String, u16> {
    let mut map = HashMap::new();
    
    // Letters
    map.insert("A".to_string(), 1);
    map.insert("E".to_string(), 1);

    map
}

pub fn get_keycode_by_name(name: &str) -> Option<u16> {
    let mappings = create_keycode_mappings();
    mappings.get(name).copied()
}

pub fn get_keycode_by_alias(alias: &str) -> Option<u16> {
    let normalized = alias.to_lowercase().replace(" ", "").replace("-", "");
    
    // Try common aliases
    match normalized.as_str() {
        "cmd" | "command" | "⌘" => Some(55),
        "opt" | "option" | "alt" | "⌥" => Some(58), // left option as default
        "ctrl" | "control" | "⌃" => Some(59), // left control as default
        "shift" | "⇧" => Some(56), // left shift as default
        "esc" | "escape" => Some(53),
        "del" | "delete" | "backspace" => Some(51),
        "enter" | "return" | "↵" => Some(36),
        "space" | "spacebar" => Some(49),
        _ => None
    }
}
