#!/usr/bin/env python3
"""
Check all keys from the keyboard image
"""

# All keys visible in the image with their codes
all_image_keys = {
    # Function row
    53: "Escape",
    122: "F1", 120: "F2", 99: "F3", 118: "F4", 96: "F5", 97: "F6",
    98: "F7", 100: "F8", 101: "F9", 109: "F10", 103: "F11", 111: "F12",
    
    # Extended function keys visible
    105: "F13", 107: "F14", 113: "F15", 106: "F16", 160: "F17", 131: "F18",
    
    # Number row
    50: "Grave/Tilde", 18: "1", 19: "2", 20: "3", 21: "4", 23: "5", 22: "6",
    26: "7", 28: "8", 25: "9", 29: "0", 27: "Minus", 24: "Equal", 51: "Delete",
    
    # QWERTY row
    48: "Tab", 12: "Q", 13: "W", 14: "E", 15: "R", 17: "T", 16: "Y",
    32: "U", 34: "I", 31: "O", 35: "P", 33: "LeftBracket", 30: "RightBracket", 42: "Backslash",
    
    # ASDF row
    57: "CapsLock", 0: "A", 1: "S", 2: "D", 3: "F", 5: "G", 4: "H",
    38: "J", 40: "K", 37: "L", 41: "Semicolon", 39: "Quote", 36: "Return",
    
    # ZXCV row
    56: "ShiftLeft", 6: "Z", 7: "X", 8: "C", 9: "V", 11: "B", 45: "N",
    46: "M", 43: "Comma", 47: "Period", 44: "Slash", 60: "ShiftRight",
    
    # Bottom row
    63: "Fn", 59: "ControlLeft", 58: "OptionLeft", 55: "CommandLeft", 
    49: "Space", 
    55: "CommandRight", 61: "OptionRight",
    
    # Arrow keys
    123: "LeftArrow", 125: "DownArrow", 124: "RightArrow", 126: "UpArrow",
    
    # Numpad enter (visible in image)
    76: "NumpadEnter"
}

# Check for duplicates
from collections import Counter
code_counts = Counter(all_image_keys.keys())
duplicates = [(code, count) for code, count in code_counts.items() if count > 1]

if duplicates:
    print("Duplicate key codes found:")
    for code, count in duplicates:
        print(f"  Code {code} appears {count} times")
        
# Check unique codes
unique_codes = sorted(set(all_image_keys.keys()))
print(f"\nTotal unique key codes visible: {len(unique_codes)}")
print(f"Key codes range: {min(unique_codes)} to {max(unique_codes)}")

# Group by ranges
print("\nKey code distribution:")
ranges = [
    (0, 20, "Letters/Numbers"),
    (21, 50, "Numbers/Special"),
    (51, 80, "Special/Modifiers"),
    (81, 100, "Function/Numpad"),
    (101, 130, "Function keys"),
    (131, 160, "Extended Function"),
]

for start, end, name in ranges:
    codes_in_range = [c for c in unique_codes if start <= c <= end]
    if codes_in_range:
        print(f"  {name} ({start}-{end}): {len(codes_in_range)} keys")