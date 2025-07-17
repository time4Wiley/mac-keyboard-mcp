# Function Key Behavior on Modern Mac Keyboards

## Overview

Modern Mac keyboards have dual-purpose function keys:
- **Without Fn**: Media/system functions (brightness, volume, etc.)
- **With Fn**: Traditional F1-F12 keys
- **Globe/Fn key**: Toggles this behavior

## Key Code Mapping

The same physical key can generate different key codes depending on:
1. Whether Fn is pressed
2. System preferences (Use F1, F2, etc. keys as standard function keys)

## Examples from the Keyboard Image

Looking at the keyboard image, we can see:

| Physical Key | Default Action | Code | With Fn | Code |
|-------------|----------------|------|---------|------|
| F1 | Brightness Down | ? | F14 | 107 |
| F2 | Brightness Up | ? | F15 | 113 |
| F3 | Mission Control | ? | F17 | 160 |
| F4 | Launchpad | ? | F18 | 131 |
| F5 | Keyboard Backlight Down | 96 | F17 | 160 |

## Important Notes

1. **Key Code 160** appears on the F5 key in the image, suggesting it's F17 (accessed via Fn+F5)
2. **Extended F-keys** (F13-F19) are typically accessed by:
   - Fn + F1-F12 on compact keyboards
   - Dedicated keys on extended keyboards

## Naming Convention

For our MCP server, we use:
- **F1-F20**: Standard function key names
- **Media aliases**: "BrightnessDown", "MissionControl", etc.
- Both resolve to the same key codes where applicable

## Implementation Strategy

Since AppleScript key codes are fixed values, we:
1. Map all known key codes to their primary names
2. Add aliases for alternate functions
3. Document the Fn key behavior
4. Let users query by either name

Example:
- User queries "F14" → returns code 107
- User queries "BrightnessDown" → returns code 107
- Both are correct depending on keyboard mode