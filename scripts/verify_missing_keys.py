#!/usr/bin/env python3
"""
Verify missing keys by comparing with the keyboard image
"""

# Keys visible in the image with their codes
image_keys = {
    # Function row
    53: "esc",
    122: "F1", 120: "F2", 99: "F3", 118: "F4", 96: "F5", 97: "F6",
    98: "F7", 100: "F8", 101: "F9", 109: "F10", 103: "F11", 111: "F12",
    
    # Number row
    50: "~", 18: "1", 19: "2", 20: "3", 21: "4", 23: "5", 22: "6",
    26: "7", 28: "8", 25: "9", 29: "0", 27: "-", 24: "=", 51: "delete",
    
    # QWERTY row
    48: "tab", 12: "Q", 13: "W", 14: "E", 15: "R", 17: "T", 16: "Y",
    32: "U", 34: "I", 31: "O", 35: "P", 33: "[", 30: "]", 42: "\\",
    
    # ASDF row
    57: "caps lock", 0: "A", 1: "S", 2: "D", 3: "F", 5: "G", 4: "H",
    38: "J", 40: "K", 37: "L", 41: ";", 39: "'", 36: "enter/return",
    
    # ZXCV row
    56: "shift", 6: "Z", 7: "X", 8: "C", 9: "V", 11: "B", 45: "N",
    46: "M", 43: ",", 47: ".", 44: "/", 60: "shift",
    
    # Bottom row
    63: "fn", 59: "control", 58: "option", 55: "command", 49: "space",
    55: "command", 61: "option",
    
    # Arrow keys
    123: "←", 125: "↓", 124: "→", 126: "↑",
    
    # Special keys from image
    107: "F13", 113: "F14", 160: "F15", 131: "F16", 160: "F17", 
    76: "enter (numpad)"
}

# Load our current database
import json
with open('../analysis/keycodes_manual.json', 'r') as f:
    our_db = json.load(f)

# Flatten our database
our_keys = {}
for category, keys in our_db.items():
    for name, code in keys.items():
        our_keys[code] = name

# Find missing keys
missing_keys = []
for code, name in image_keys.items():
    if code not in our_keys:
        missing_keys.append((code, name))

print("Missing keys from our database:")
print("==============================")
for code, name in sorted(missing_keys):
    print(f"Code {code}: {name}")

# Check for code 160 specifically
print("\nChecking code 160:")
if 160 in our_keys:
    print(f"Code 160 is in our DB as: {our_keys[160]}")
else:
    print("Code 160 is MISSING from our database")

# Also check if we have different codes for the same key
print("\nPotential conflicts:")
for code, name in image_keys.items():
    if code in our_keys and our_keys[code].lower() != name.lower():
        print(f"Code {code}: Image shows '{name}', we have '{our_keys[code]}'")