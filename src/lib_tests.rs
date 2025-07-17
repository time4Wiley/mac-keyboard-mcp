#[cfg(test)]
mod tests {
    use super::*;
    use crate::keycode::{KEY_DATABASE, parse_shortcut};
    use crate::search::FuzzySearcher;
    
    #[test]
    fn test_key_database_loading() {
        // Force database initialization
        let db = &*KEY_DATABASE;
        
        // Check some basic keys
        assert!(db.lookup("A").is_some());
        assert!(db.lookup("Space").is_some());
        assert!(db.lookup("F17").is_some()); // Our previously missing key
        assert_eq!(db.lookup("F17").unwrap().code, 160);
    }
    
    #[test]
    fn test_case_insensitive_lookup() {
        let db = &*KEY_DATABASE;
        
        let key1 = db.lookup("command");
        let key2 = db.lookup("Command");
        let key3 = db.lookup("COMMAND");
        
        assert!(key1.is_some());
        assert_eq!(key1.unwrap().code, key2.unwrap().code);
        assert_eq!(key2.unwrap().code, key3.unwrap().code);
    }
    
    #[test]
    fn test_alias_lookup() {
        let db = &*KEY_DATABASE;
        
        // Test Command aliases
        assert_eq!(db.lookup("Cmd").unwrap().name, "Command");
        assert_eq!(db.lookup("⌘").unwrap().name, "Command");
        
        // Test other aliases
        assert_eq!(db.lookup("Enter").unwrap().name, "Return");
        assert_eq!(db.lookup("Backspace").unwrap().name, "Delete");
    }
    
    #[test]
    fn test_function_key_codes() {
        let db = &*KEY_DATABASE;
        
        // Test corrected function keys
        assert_eq!(db.lookup("F7").unwrap().code, 98);
        assert_eq!(db.lookup("F8").unwrap().code, 100);
        assert_eq!(db.lookup("F10").unwrap().code, 109);
        assert_eq!(db.lookup("F17").unwrap().code, 160);
        assert_eq!(db.lookup("F18").unwrap().code, 131);
    }
    
    #[test]
    fn test_shortcut_parsing() {
        // Simple shortcuts
        let combo1 = parse_shortcut("Cmd+A").unwrap();
        assert_eq!(combo1.modifiers.len(), 1);
        assert_eq!(combo1.key.name, "A");
        
        // Multiple modifiers
        let combo2 = parse_shortcut("Cmd+Shift+S").unwrap();
        assert_eq!(combo2.modifiers.len(), 2);
        assert_eq!(combo2.key.name, "S");
        
        // Symbol shortcuts
        let combo3 = parse_shortcut("⌘⇧A").unwrap();
        assert_eq!(combo3.modifiers.len(), 2);
        assert_eq!(combo3.key.name, "A");
    }
    
    #[test]
    fn test_fuzzy_search() {
        let searcher = FuzzySearcher::new();
        
        // Test typo tolerance
        let results = searcher.search_keys("comand", 3);
        assert!(!results.is_empty());
        assert_eq!(results[0].0.name, "Command");
        
        // Test partial matching
        let results = searcher.search_keys("arr", 10);
        assert!(results.iter().any(|(k, _)| k.name.contains("Arrow")));
    }
    
    #[test]
    fn test_categories() {
        let db = &*KEY_DATABASE;
        
        let letters = db.by_category(crate::keycode::KeyCategory::Letters);
        assert_eq!(letters.len(), 26);
        
        let numbers = db.by_category(crate::keycode::KeyCategory::Numbers);
        assert_eq!(numbers.len(), 10);
        
        let functions = db.by_category(crate::keycode::KeyCategory::FunctionKeys);
        assert!(functions.len() >= 20); // F1-F20
    }
}