#!/usr/bin/env python3
"""
Research Fn key combinations and their key codes
"""

# From the keyboard image, we can see:
# - F5 key shows both "F5" (96) and what appears to be F17 (160)
# - This suggests Fn+F5 = F17

# Let's map the pattern
fn_key_mappings = {
    # Base key -> Fn+key mapping (based on the image)
    "F1": {"base": 122, "with_fn": 107},  # F1 -> F14
    "F2": {"base": 120, "with_fn": 113},  # F2 -> F15
    "F3": {"base": 99, "with_fn": 160},   # F3 -> F17? (or is it F5?)
    "F4": {"base": 118, "with_fn": 131},  # F4 -> F18
    "F5": {"base": 96, "with_fn": 160},   # F5 -> F17 (visible in image)
    "F6": {"base": 97, "with_fn": None},
    "F7": {"base": 98, "with_fn": None},
    "F8": {"base": 100, "with_fn": None},
    "F9": {"base": 101, "with_fn": None},
    "F10": {"base": 109, "with_fn": None},
    "F11": {"base": 103, "with_fn": None},
    "F12": {"base": 111, "with_fn": None},
}

# Modern Mac keyboards often have media/system functions as primary
# and F-keys as secondary (requiring Fn key)
media_functions = {
    "F1": "Brightness Down",
    "F2": "Brightness Up", 
    "F3": "Mission Control",
    "F4": "Launchpad",
    "F5": "Keyboard Backlight Down",
    "F6": "Keyboard Backlight Up",
    "F7": "Previous Track",
    "F8": "Play/Pause",
    "F9": "Next Track",
    "F10": "Mute",
    "F11": "Volume Down",
    "F12": "Volume Up",
}

print("Mac Keyboard Function Key Analysis")
print("==================================\n")

print("Standard F-keys (without Fn):")
for key, info in fn_key_mappings.items():
    media = media_functions.get(key, "Unknown")
    print(f"  {key} (code {info['base']}): {media}")

print("\nWith Fn modifier (traditional F-keys):")
for key, info in fn_key_mappings.items():
    if info['with_fn']:
        print(f"  Fn+{key}: F{info['with_fn']} (code {info['with_fn']})?")

print("\nPossible naming conventions:")
print("1. F1-F12: Standard function keys")
print("2. F13-F24: Extended function keys (Fn+F1 through Fn+F12)")
print("3. Media keys: Brightness, Volume, etc. (default without Fn)")
print("4. F1+ notation: Could indicate Fn+F1")

print("\nRecommendation:")
print("- Keep F1-F20 as standard names")
print("- Add aliases for media functions")
print("- Document Fn key behavior clearly")