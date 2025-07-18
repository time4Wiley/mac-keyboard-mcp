#!/usr/bin/env python3
"""
Export key codes from Rust source to JSON for verification
"""

import json
import re
from pathlib import Path

def parse_rust_keycodes():
    """Parse key codes from Rust source"""
    # Read the Rust database file
    rust_file = Path("../src/keycode/database.rs")
    with open(rust_file, 'r') as f:
        content = f.read()
    
    # Parse KeyCode::new entries from the entire file
    pattern = r'KeyCode::new\("([^"]+)",\s*(\d+),\s*KeyCategory::(\w+)\)'
    matches = re.findall(pattern, content)
    
    # Organize by category
    categories = {
        "Letters": {},
        "Numbers": {},
        "FunctionKeys": {},
        "ModifierKeys": {},
        "NavigationKeys": {},
        "SpecialKeys": {},
        "NumpadKeys": {},
        "Punctuation": {}
    }
    
    for name, code, category in matches:
        if category in categories:
            categories[category][name] = int(code)
    
    # Convert to lowercase keys for JSON
    result = {}
    for cat, keys in categories.items():
        # Convert category names to snake_case
        snake_case = re.sub(r'(?<!^)(?=[A-Z])', '_', cat).lower()
        result[snake_case] = keys
    
    return result

def main():
    keycodes = parse_rust_keycodes()
    
    # Save to JSON
    output_file = Path("keycodes.json")
    with open(output_file, 'w') as f:
        json.dump(keycodes, f, indent=2)
    
    print(f"✅ Exported {sum(len(v) for v in keycodes.values())} key codes to {output_file}")
    
    # Print summary
    for category, keys in keycodes.items():
        print(f"  {category}: {len(keys)} keys")

if __name__ == "__main__":
    main()