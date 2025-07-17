#!/usr/bin/env python3
import requests
from bs4 import BeautifulSoup
import json
import re
from typing import Dict, List, Tuple
from collections import OrderedDict

def fetch_keycodes_page():
    """Fetch the AppleScript key codes page"""
    url = "https://eastmanreference.com/complete-list-of-applescript-key-codes"
    headers = {
        'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36'
    }
    response = requests.get(url, headers=headers)
    return response.text

def parse_keycodes(html: str) -> Dict[str, Dict[str, int]]:
    """Parse key codes from HTML content"""
    soup = BeautifulSoup(html, 'html.parser')
    
    keycodes = {
        "letters": {},
        "numbers": {},
        "function_keys": {},
        "modifier_keys": {},
        "navigation_keys": {},
        "special_keys": {},
        "numpad_keys": {},
        "punctuation": {}
    }
    
    # Find all text containing key code patterns
    text = soup.get_text()
    
    # Parse letter keys (a-z)
    letter_pattern = r'([a-zA-Z])\s*[:–-]\s*(\d+)'
    for match in re.finditer(letter_pattern, text):
        letter, code = match.groups()
        if len(letter) == 1 and letter.isalpha():
            keycodes["letters"][letter.upper()] = int(code)
    
    # Parse number keys (0-9)
    number_pattern = r'(?<!\d)([0-9])\s*[:–-]\s*(\d{2,3})(?!\d)'
    for match in re.finditer(number_pattern, text):
        number, code = match.groups()
        keycodes["numbers"][number] = int(code)
    
    # Parse function keys
    func_pattern = r'F(\d+)\s*[:–-]\s*(\d+)'
    for match in re.finditer(func_pattern, text):
        f_num, code = match.groups()
        keycodes["function_keys"][f"F{f_num}"] = int(code)
    
    # Parse special keys with various names
    special_mappings = {
        r'(?:escape|esc)\s*[:–-]\s*(\d+)': "Escape",
        r'(?:space\s*bar?|spacebar)\s*[:–-]\s*(\d+)': "Space",
        r'tab\s*[:–-]\s*(\d+)': "Tab",
        r'(?:delete|del)\s*[:–-]\s*(\d+)': "Delete",
        r'(?:return|enter)\s*[:–-]\s*(\d+)': "Return",
        r'(?:forward\s*delete)\s*[:–-]\s*(\d+)': "ForwardDelete",
        r'clear\s*[:–-]\s*(\d+)': "Clear",
        r'help\s*[:–-]\s*(\d+)': "Help"
    }
    
    for pattern, key_name in special_mappings.items():
        match = re.search(pattern, text, re.IGNORECASE)
        if match:
            keycodes["special_keys"][key_name] = int(match.group(1))
    
    # Parse navigation keys
    nav_mappings = {
        r'(?:left\s*arrow?|←)\s*[:–-]\s*(\d+)': "LeftArrow",
        r'(?:right\s*arrow?|→)\s*[:–-]\s*(\d+)': "RightArrow",
        r'(?:up\s*arrow?|↑)\s*[:–-]\s*(\d+)': "UpArrow",
        r'(?:down\s*arrow?|↓)\s*[:–-]\s*(\d+)': "DownArrow",
        r'(?:page\s*up)\s*[:–-]\s*(\d+)': "PageUp",
        r'(?:page\s*down)\s*[:–-]\s*(\d+)': "PageDown",
        r'home\s*[:–-]\s*(\d+)': "Home",
        r'end\s*[:–-]\s*(\d+)': "End"
    }
    
    for pattern, key_name in nav_mappings.items():
        match = re.search(pattern, text, re.IGNORECASE)
        if match:
            keycodes["navigation_keys"][key_name] = int(match.group(1))
    
    # Parse modifier keys
    mod_mappings = {
        r'(?:control|ctrl)(?:\s*\(?\s*right\s*\)?)\s*[:–-]\s*(\d+)': "ControlRight",
        r'(?:control|ctrl)(?:\s*\(?\s*left\s*\)?)\s*[:–-]\s*(\d+)': "ControlLeft",
        r'(?:option|alt)(?:\s*\(?\s*right\s*\)?)\s*[:–-]\s*(\d+)': "OptionRight",
        r'(?:option|alt)(?:\s*\(?\s*left\s*\)?)\s*[:–-]\s*(\d+)': "OptionLeft",
        r'shift(?:\s*\(?\s*right\s*\)?)\s*[:–-]\s*(\d+)': "ShiftRight",
        r'shift(?:\s*\(?\s*left\s*\)?)\s*[:–-]\s*(\d+)': "ShiftLeft",
        r'(?:command|cmd|⌘)\s*[:–-]\s*(\d+)': "Command",
        r'(?:caps\s*lock)\s*[:–-]\s*(\d+)': "CapsLock"
    }
    
    for pattern, key_name in mod_mappings.items():
        match = re.search(pattern, text, re.IGNORECASE)
        if match:
            keycodes["modifier_keys"][key_name] = int(match.group(1))
    
    # Parse punctuation and symbols
    punct_mappings = {
        r'[`~]\s*[:–-]\s*(\d+)': "Grave",
        r'[-_]\s*[:–-]\s*(\d+)': "Minus",
        r'[=+]\s*[:–-]\s*(\d+)': "Equal",
        r'[\[\{]\s*[:–-]\s*(\d+)': "LeftBracket",
        r'[\]\}]\s*[:–-]\s*(\d+)': "RightBracket",
        r'[\\|]\s*[:–-]\s*(\d+)': "Backslash",
        r'[;:]\s*[:–-]\s*(\d+)': "Semicolon",
        r"['\"]\s*[:–-]\s*(\d+)": "Quote",
        r'[,<]\s*[:–-]\s*(\d+)': "Comma",
        r'[.>]\s*[:–-]\s*(\d+)': "Period",
        r'[/?]\s*[:–-]\s*(\d+)': "Slash"
    }
    
    for pattern, key_name in punct_mappings.items():
        match = re.search(pattern, text)
        if match:
            keycodes["punctuation"][key_name] = int(match.group(1))
    
    return keycodes

def generate_rust_data(keycodes: Dict[str, Dict[str, int]]) -> str:
    """Generate Rust code for the key code mappings"""
    rust_code = """// Auto-generated AppleScript key code mappings
use std::collections::HashMap;

pub fn create_keycode_mappings() -> HashMap<String, u16> {
    let mut map = HashMap::new();
    
"""
    
    for category, keys in keycodes.items():
        if keys:
            rust_code += f"    // {category.replace('_', ' ').title()}\n"
            for key, code in sorted(keys.items()):
                rust_code += f'    map.insert("{key}".to_string(), {code});\n'
            rust_code += "\n"
    
    rust_code += """    map
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
"""
    
    return rust_code

def main():
    print("Fetching AppleScript key codes...")
    html = fetch_keycodes_page()
    
    print("Parsing key codes...")
    keycodes = parse_keycodes(html)
    
    # Save parsed data as JSON
    with open('keycodes.json', 'w') as f:
        json.dump(keycodes, f, indent=2)
    print(f"Saved key codes to keycodes.json")
    
    # Generate Rust code
    rust_code = generate_rust_data(keycodes)
    with open('keycodes.rs', 'w') as f:
        f.write(rust_code)
    print(f"Generated Rust code in keycodes.rs")
    
    # Print summary
    total = sum(len(category) for category in keycodes.values())
    print(f"\nSummary:")
    print(f"Total key codes found: {total}")
    for category, keys in keycodes.items():
        if keys:
            print(f"  {category}: {len(keys)} keys")

if __name__ == "__main__":
    main()