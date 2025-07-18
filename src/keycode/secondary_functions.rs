use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Mapping of F-keys to their secondary (media/system) functions
/// These are the functions accessible when pressing F-keys without Fn on modern Mac keyboards
pub static F_KEY_SECONDARY_FUNCTIONS: Lazy<HashMap<&'static str, SecondaryFunction>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // F1-F12 secondary functions on modern Mac keyboards
    map.insert("F1", SecondaryFunction {
        name: "Brightness Down",
        keycode: 107,  // F14
        description: "Decrease display brightness",
    });
    
    map.insert("F2", SecondaryFunction {
        name: "Brightness Up",
        keycode: 113,  // F15
        description: "Increase display brightness",
    });
    
    map.insert("F3", SecondaryFunction {
        name: "Mission Control",
        keycode: 160,  // F17
        description: "Show all open windows",
    });
    
    map.insert("F4", SecondaryFunction {
        name: "Launchpad",
        keycode: 131,  // F18
        description: "Show all apps",
    });
    
    map.insert("F5", SecondaryFunction {
        name: "Keyboard Illumination Down",
        keycode: 105,  // F13
        description: "Decrease keyboard backlight",
    });
    
    map.insert("F6", SecondaryFunction {
        name: "Keyboard Illumination Up",
        keycode: 106,  // F16
        description: "Increase keyboard backlight",
    });
    
    map.insert("F7", SecondaryFunction {
        name: "Previous Track",
        keycode: 98,  // F7 actually sends its own code for media keys
        description: "Skip to previous track",
    });
    
    map.insert("F8", SecondaryFunction {
        name: "Play/Pause",
        keycode: 100,  // F8
        description: "Play or pause media",
    });
    
    map.insert("F9", SecondaryFunction {
        name: "Next Track",
        keycode: 101,  // F9
        description: "Skip to next track",
    });
    
    map.insert("F10", SecondaryFunction {
        name: "Mute",
        keycode: 74,  // Mute key
        description: "Mute/unmute audio",
    });
    
    map.insert("F11", SecondaryFunction {
        name: "Volume Down",
        keycode: 73,  // Volume Down
        description: "Decrease volume",
    });
    
    map.insert("F12", SecondaryFunction {
        name: "Volume Up",
        keycode: 72,  // Volume Up
        description: "Increase volume",
    });
    
    map
});

#[derive(Debug, Clone)]
pub struct SecondaryFunction {
    pub name: &'static str,
    pub keycode: u8,
    pub description: &'static str,
}

/// Check if a query is asking for a secondary function (e.g., "F3+", "F1+")
pub fn is_secondary_function_query(query: &str) -> Option<&'static str> {
    let trimmed = query.trim();
    
    // Check for F{n}+ pattern
    if trimmed.len() >= 3 && trimmed.ends_with('+') {
        let base = &trimmed[..trimmed.len() - 1];
        
        // Check if it matches F1-F12
        if base.starts_with('F') || base.starts_with('f') {
            if let Ok(num) = base[1..].parse::<u8>() {
                if num >= 1 && num <= 12 {
                    // Return normalized F-key name
                    match num {
                        1 => return Some("F1"),
                        2 => return Some("F2"),
                        3 => return Some("F3"),
                        4 => return Some("F4"),
                        5 => return Some("F5"),
                        6 => return Some("F6"),
                        7 => return Some("F7"),
                        8 => return Some("F8"),
                        9 => return Some("F9"),
                        10 => return Some("F10"),
                        11 => return Some("F11"),
                        12 => return Some("F12"),
                        _ => {}
                    }
                }
            }
        }
    }
    
    None
}

/// Get secondary function information for an F-key
pub fn get_secondary_function(f_key: &str) -> Option<&SecondaryFunction> {
    F_KEY_SECONDARY_FUNCTIONS.get(f_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_secondary_function_query_detection() {
        assert_eq!(is_secondary_function_query("F3+"), Some("F3"));
        assert_eq!(is_secondary_function_query("f3+"), Some("F3"));
        assert_eq!(is_secondary_function_query("F12+"), Some("F12"));
        assert_eq!(is_secondary_function_query("F13+"), None); // Only F1-F12
        assert_eq!(is_secondary_function_query("F3"), None); // No +
        assert_eq!(is_secondary_function_query("G3+"), None); // Not F
    }
    
    #[test]
    fn test_get_secondary_function() {
        let f3_secondary = get_secondary_function("F3").unwrap();
        assert_eq!(f3_secondary.name, "Mission Control");
        assert_eq!(f3_secondary.keycode, 160);
    }
}