# Mac Keyboard MCP

A Model Context Protocol (MCP) server for macOS AppleScript key codes. This server helps AI assistants and developers quickly find the correct key codes for AppleScript automation on macOS.

## Features

- 🔍 **Smart Key Lookup**: Find key codes by name with fuzzy search support
- 🎹 **100+ Key Codes**: Complete database including F1-F20, media keys, and special characters  
- ⌨️ **Shortcut Parsing**: Convert shortcuts like "Cmd+Shift+A" to key code sequences
- 🏷️ **Alias Support**: Multiple names per key (e.g., "Cmd", "Command", "⌘")
- 📚 **Category Organization**: Browse keys by type (letters, numbers, modifiers, etc.)
- 🚀 **Fast & Efficient**: In-memory database with O(1) lookups

## Key Code Database

The server includes accurate key codes for:
- **Letters**: A-Z
- **Numbers**: 0-9  
- **Function Keys**: F1-F20 (including F17 at code 160)
- **Modifiers**: Command, Shift, Option, Control, Fn, Caps Lock
- **Navigation**: Arrow keys, Page Up/Down, Home, End
- **Special**: Space, Tab, Return, Delete, Escape
- **Numpad**: All numeric keypad keys
- **Punctuation**: All symbols and punctuation marks

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/username/mac-keyboard-mcp.git
cd mac-keyboard-mcp

# Build the project
cargo build --release

# The binary will be at target/release/mac-keyboard-mcp
```

### MCP Client Configuration

Add to your MCP client configuration:

```json
{
  "mcpServers": {
    "mac-keyboard": {
      "command": "/path/to/mac-keyboard-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

## MCP Tools

### 1. `lookup_keycode`
Find the AppleScript key code for a specific key.

**Parameters:**
- `key_name` (string, required): Name of the key (e.g., "A", "Space", "Command", "F3+")
- `fuzzy` (boolean, optional): Enable fuzzy matching for typos

**Example:**
```json
{
  "tool": "lookup_keycode",
  "arguments": {
    "key_name": "Command",
    "fuzzy": false
  }
}
```

**Response:**
```json
{
  "found": true,
  "key": {
    "name": "Command",
    "code": 55,
    "category": "modifier_keys",
    "aliases": ["Cmd", "⌘"]
  }
}
```

**F-Key Secondary Functions:**
Use the `F{n}+` notation (e.g., "F3+") to query F-key secondary functions:

```json
{
  "tool": "lookup_keycode",
  "arguments": {
    "key_name": "F3+"
  }
}
```

**Response:**
```json
{
  "found": true,
  "query_type": "secondary_function",
  "f_key": {
    "name": "F3",
    "primary_function": {
      "name": "F3",
      "code": 99,
      "description": "Traditional F3 function key"
    },
    "secondary_function": {
      "name": "Mission Control",
      "code": 160,
      "description": "Show all open windows"
    },
    "note": "On modern Mac keyboards, pressing F3 triggers 'Mission Control' by default. Use Fn+F3 to get the traditional F3 function."
  }
}
```

### 2. `search_keys`
Search for keys by pattern or category.

**Parameters:**
- `query` (string, optional): Search term (partial match supported)
- `category` (string, optional): Filter by category

**Example:**
```json
{
  "tool": "search_keys",
  "arguments": {
    "query": "arrow",
    "category": "navigation_keys"
  }
}
```

### 3. `get_key_combinations`
Parse keyboard shortcuts into key code sequences.

**Parameters:**
- `shortcut` (string, required): Shortcut string (e.g., "Cmd+A", "⌘⇧S")

**Example:**
```json
{
  "tool": "get_key_combinations",
  "arguments": {
    "shortcut": "Cmd+Shift+A"
  }
}
```

**Response:**
```json
{
  "success": true,
  "shortcut": "Command+Shift+A",
  "keycodes": [55, 56, 0],
  "modifiers": [
    {"name": "Command", "code": 55},
    {"name": "Shift", "code": 56}
  ],
  "key": {"name": "A", "code": 0}
}
```

### 4. `list_categories`
List all available key categories with counts.

## MCP Resources

### `keycode://all`
Complete database of all key codes organized by category.

### `keycode://category/{name}`
Key codes for a specific category:
- `keycode://category/letters`
- `keycode://category/numbers`
- `keycode://category/function_keys`
- `keycode://category/modifier_keys`
- `keycode://category/navigation_keys`
- `keycode://category/special_keys`
- `keycode://category/numpad_keys`
- `keycode://category/punctuation`

### `keycode://aliases`
All key aliases and their canonical names.

## AppleScript Usage

Once you have the key codes, use them in AppleScript:

```applescript
-- Single key press
tell application "System Events" to key code 0 -- 'A'

-- With modifiers
tell application "System Events" to key code 0 using command down -- Cmd+A

-- Multiple modifiers
tell application "System Events" to key code 1 using {command down, shift down} -- Cmd+Shift+S
```

## Development

### Building from Source

```bash
# Development build
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Verification

The project includes comprehensive verification tools:

```bash
# Quick verification
./scripts/continuous_verify.sh

# Interactive key testing
cargo run --bin verify --features verification

# Python-based verification
cd scripts
python3 verify_keycodes.py --mode interactive
```

## Modern Mac Keyboards

This server handles the dual nature of modern Mac function keys:
- **Default**: Media functions (brightness, volume, etc.)
- **With Fn**: Traditional F1-F12
- **F13-F19**: Often accessed via Fn+F1 through Fn+F12

The server provides aliases for media functions where applicable (e.g., "Brightness Down" maps to F14/code 107).

### F-Key Secondary Function Mappings

| F-Key | Primary (Fn+F#) | Secondary (Default) | Secondary Code |
|-------|-----------------|---------------------|----------------|
| F1    | F1 (122)       | Brightness Down     | 107 (F14)      |
| F2    | F2 (120)       | Brightness Up       | 113 (F15)      |
| F3    | F3 (99)        | Mission Control     | 160 (F17)      |
| F4    | F4 (118)       | Launchpad          | 131 (F18)      |
| F5    | F5 (96)        | Keyboard Illumination Down | 105 (F13) |
| F6    | F6 (97)        | Keyboard Illumination Up | 106 (F16) |
| F7    | F7 (98)        | Previous Track      | 98             |
| F8    | F8 (100)       | Play/Pause         | 100            |
| F9    | F9 (101)       | Next Track         | 101            |
| F10   | F10 (109)      | Mute               | 74             |
| F11   | F11 (103)      | Volume Down        | 73             |
| F12   | F12 (111)      | Volume Up          | 72             |

Use the `F{n}+` notation with the `lookup_keycode` tool to get both mappings.

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## Acknowledgments

- Key code data verified against macOS system behavior
- Built with the Model Context Protocol specification
- Fuzzy search powered by `fuzzy-matcher` crate