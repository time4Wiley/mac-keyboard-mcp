# Level 1 Plan: macOS AppleScript KeyCode MCP Server

## Project Overview

A Model Context Protocol (MCP) server that provides AppleScript key code lookups for macOS. The server will be implemented in Rust and designed to help AI assistants and developers quickly find the correct key codes for AppleScript automation.

## Architecture Overview

### Core Components

1. **MCP Server Framework**
   - Built using `rmcp` (official Rust SDK for Model Context Protocol)
   - Implements JSON-RPC 2.0 protocol over stdio transport
   - Provides both tools and resources for key code access

2. **Key Code Database**
   - Static key code mappings loaded from JSON
   - In-memory HashMap for O(1) lookup performance
   - Support for multiple naming conventions and aliases

3. **API Layer**
   - MCP Tools for interactive queries
   - MCP Resources for bulk data access
   - Fuzzy search and intelligent matching

## API Design

### MCP Tools (Interactive Operations)

1. **lookup_keycode**
   - Description: Find AppleScript key code for a specific key
   - Parameters:
     - `key_name`: String (e.g., "A", "Space", "Command", "F1")
     - `fuzzy`: Boolean (optional, enable fuzzy matching)
   - Returns: Key code number or null if not found

2. **search_keys**
   - Description: Search for keys by pattern or category
   - Parameters:
     - `query`: String (search term)
     - `category`: String (optional: letters, numbers, function_keys, etc.)
   - Returns: Array of matching keys with their codes

3. **get_key_combinations**
   - Description: Generate key code sequences for shortcuts
   - Parameters:
     - `shortcut`: String (e.g., "Cmd+Shift+A", "Ctrl+Space")
   - Returns: Array of key codes in correct order

4. **list_categories**
   - Description: List all available key categories
   - Returns: Array of category names with key counts

### MCP Resources (Static Data Access)

1. **keycode://all**
   - Complete key code database in JSON format
   
2. **keycode://category/{name}**
   - Key codes for specific category (letters, modifiers, etc.)
   
3. **keycode://aliases**
   - Common aliases and alternative names for keys

## Key Features

### 1. Intelligent Key Recognition
- Support multiple naming conventions:
  - Full names: "Command", "Control", "Option"
  - Short names: "Cmd", "Ctrl", "Opt"
  - Symbols: "⌘", "⌃", "⌥"
  - Common aliases: "Alt" → "Option", "Enter" → "Return"

### 2. Fuzzy Search
- Levenshtein distance for typo tolerance
- Partial matching for substring searches
- Case-insensitive matching

### 3. Combination Parser
- Parse complex shortcuts like "Cmd+Shift+A"
- Handle multiple modifier keys
- Support both "+" and "-" as separators

### 4. Category Organization
- Letters (A-Z)
- Numbers (0-9)
- Function Keys (F1-F20)
- Modifier Keys (Command, Shift, Option, Control)
- Navigation Keys (Arrow keys, Page Up/Down, Home, End)
- Special Keys (Space, Tab, Return, Delete, Escape)
- Numpad Keys
- Punctuation and Symbols

## Technical Stack

### Dependencies
- `rmcp` (v0.2.0) - MCP protocol implementation
- `tokio` - Async runtime
- `serde` / `serde_json` - Serialization
- `fuzzy-matcher` - Fuzzy string matching
- `once_cell` - Lazy static initialization
- `tracing` - Logging and diagnostics

### Platform Requirements
- macOS only (uses AppleScript key codes)
- Rust 1.75+ (for stable async traits)
- No special permissions required (read-only service)

## Error Handling

1. **Invalid Key Names**
   - Return helpful suggestions for similar keys
   - Provide list of valid alternatives

2. **Category Errors**
   - List available categories on invalid input
   - Suggest correct category for misplaced keys

3. **Parsing Errors**
   - Clear error messages for malformed shortcuts
   - Examples of correct format

## Performance Considerations

1. **Static Data Loading**
   - Load key codes once at startup
   - Use lazy_static or once_cell for initialization

2. **Efficient Lookups**
   - HashMap for O(1) key lookups
   - Pre-computed aliases and variations

3. **Minimal Memory Footprint**
   - ~200 key entries total
   - Shared static data structures

## Security Considerations

1. **Read-Only Service**
   - No file system access
   - No network requests
   - No key event generation

2. **Input Validation**
   - Sanitize all input strings
   - Limit query lengths
   - Prevent resource exhaustion

## Future Enhancements

1. **Extended Key Support**
   - Media keys (Play, Pause, Volume)
   - Touch Bar keys
   - International keyboard layouts

2. **AppleScript Generation**
   - Generate complete AppleScript snippets
   - Support for key down/up events
   - Timing and delay options

3. **Integration Features**
   - Export to various formats (CSV, JSON, Markdown)
   - Generate documentation
   - Keyboard layout visualization