# Usage Examples

## Common Use Cases

### 1. Finding a Single Key Code

**User**: "What's the key code for the space bar?"

**MCP Request**:
```json
{
  "tool": "lookup_keycode",
  "arguments": {
    "key_name": "Space"
  }
}
```

**Response**: Space key has code 49

### 2. Handling Typos

**User**: "What's the code for 'comand' key?" (typo)

**MCP Request**:
```json
{
  "tool": "lookup_keycode", 
  "arguments": {
    "key_name": "comand",
    "fuzzy": true
  }
}
```

**Response**: Did you mean "Command"? It has code 55

### 3. Building a Shortcut

**User**: "I need to simulate Cmd+Shift+4 for screenshot"

**MCP Request**:
```json
{
  "tool": "get_key_combinations",
  "arguments": {
    "shortcut": "Cmd+Shift+4"
  }
}
```

**Response**: Key codes are [55, 56, 21]

**Generated AppleScript**:
```applescript
tell application "System Events"
    key code 21 using {command down, shift down}
end tell
```

### 4. Finding All Arrow Keys

**User**: "Show me all arrow keys"

**MCP Request**:
```json
{
  "tool": "search_keys",
  "arguments": {
    "query": "arrow"
  }
}
```

**Response**:
- LeftArrow: 123
- RightArrow: 124
- UpArrow: 126
- DownArrow: 125

### 5. Listing Function Keys

**User**: "What function keys are available?"

**MCP Request**:
```json
{
  "tool": "search_keys",
  "arguments": {
    "category": "function_keys"
  }
}
```

**Response**: F1-F20 with codes 122, 120, 99... (20 keys total)

### 6. Using Symbols

**User**: "What's the code for ⌘A?"

**MCP Request**:
```json
{
  "tool": "get_key_combinations",
  "arguments": {
    "shortcut": "⌘A"
  }
}
```

**Response**: Command (55) + A (0)

## Advanced Examples

### Automating Text Selection

**Goal**: Select all text in current application

```applescript
-- Generated from "Cmd+A" lookup
tell application "System Events"
    key code 0 using command down
end tell
```

### Navigating with Arrow Keys

**Goal**: Move to beginning of line

```applescript
-- Generated from "Cmd+LeftArrow" lookup  
tell application "System Events"
    key code 123 using command down
end tell
```

### Media Key Control

**Goal**: Decrease screen brightness

```applescript
-- F14 (code 107) is Brightness Down on many Macs
tell application "System Events"
    key code 107
end tell
```

## Tips

1. **Case Insensitive**: "command", "Command", and "COMMAND" all work
2. **Flexible Separators**: "Cmd+A", "Cmd-A", and "Cmd A" are equivalent  
3. **Symbol Support**: Use ⌘, ⇧, ⌥, ⌃ for modifiers
4. **Fuzzy Search**: Enable for typo tolerance
5. **Browse Categories**: Use `list_categories` to explore available keys