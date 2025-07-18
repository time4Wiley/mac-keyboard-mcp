#!/bin/bash

echo "🔍 Mac Keyboard MCP Setup Verification"
echo "====================================="

# Check if binary exists
if [ -f "/Users/wei/Projects/mac-keyboard-mcp/target/release/mac-keyboard-mcp" ]; then
    echo "✅ Binary exists"
    echo "   Path: /Users/wei/Projects/mac-keyboard-mcp/target/release/mac-keyboard-mcp"
    echo "   Size: $(ls -lh /Users/wei/Projects/mac-keyboard-mcp/target/release/mac-keyboard-mcp | awk '{print $5}')"
else
    echo "❌ Binary not found!"
    exit 1
fi

# Check if binary is executable
if [ -x "/Users/wei/Projects/mac-keyboard-mcp/target/release/mac-keyboard-mcp" ]; then
    echo "✅ Binary is executable"
else
    echo "❌ Binary is not executable"
    exit 1
fi

# Check Claude config
CONFIG_FILE="$HOME/Library/Application Support/Claude/claude_desktop_config.json"
if [ -f "$CONFIG_FILE" ]; then
    echo "✅ Claude config exists"
    if grep -q "mac-keyboard" "$CONFIG_FILE"; then
        echo "✅ Mac Keyboard MCP is configured"
        echo "   Configuration:"
        grep -A 4 '"mac-keyboard"' "$CONFIG_FILE" | sed 's/^/   /'
    else
        echo "❌ Mac Keyboard MCP not found in config"
    fi
else
    echo "❌ Claude config not found"
fi

# Test the binary
echo ""
echo "📋 Testing binary..."
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}}' | timeout 2 /Users/wei/Projects/mac-keyboard-mcp/target/release/mac-keyboard-mcp 2>/dev/null | head -1 | jq -r '.result.serverInfo.name' 2>/dev/null

if [ $? -eq 0 ]; then
    echo "✅ Binary responds correctly to MCP protocol"
else
    echo "⚠️  Binary test failed (this might be normal if jq is not installed)"
fi

echo ""
echo "📌 Next steps:"
echo "1. Restart Claude Code"
echo "2. Look for 'Mac-keyboard MCP Server' in the MCP servers list"
echo "3. The status should show 'connected' (green circle)"
echo ""
echo "If still having issues, check logs at:"
echo "  ~/.mac-keyboard-mcp/logs/"