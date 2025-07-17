# Key Code Database - Final Summary

## Corrections Made

1. **Fixed Function Keys**:
   - F7: 98 (was 131)
   - F8: 100 (was 109) 
   - F10: 109 (was 100)
   - F17: 160 (was missing!)
   - F18: 131 (was 79)

2. **Understanding Modern Mac Keyboards**:
   - Physical F-keys have dual purposes
   - Default: Media functions (brightness, volume, etc.)
   - With Fn key: Traditional F1-F12
   - Extended F-keys (F13-F19) often accessed via Fn+F1 through Fn+F12

3. **Key Code Behavior**:
   - AppleScript key codes are fixed values
   - Same code works regardless of Fn key state
   - F13-F19 have their own unique codes
   - Media functions typically handled by macOS, not key codes

## Database Design Decision

We chose to:
- Keep F1-F20 as primary names (most universal)
- Add media function aliases to F13-F19 where applicable
- Document but don't duplicate codes
- Let users search by any common name

## Example Lookups

```
"F17" → 160
"Mission Control" → 160 (via F17 alias)
"F14" → 107  
"Brightness Down" → 107 (via F14 alias)
```

## Total Key Codes

- 100+ unique key codes
- Range: 0-160
- All standard keys covered
- Extended function keys included