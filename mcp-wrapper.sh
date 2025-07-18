#!/bin/bash
# MCP Server Wrapper with Logging

# Create log directory
LOG_DIR="$HOME/.mac-keyboard-mcp/logs"
mkdir -p "$LOG_DIR"

# Generate timestamp for log files
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="$LOG_DIR/mcp_${TIMESTAMP}.log"

# Log startup
echo "[$(date)] MCP Server Wrapper Started" >> "$LOG_FILE"
echo "[$(date)] Command: $0 $@" >> "$LOG_FILE"
echo "[$(date)] Working Directory: $(pwd)" >> "$LOG_FILE"
echo "[$(date)] Environment:" >> "$LOG_FILE"
env | grep -E "(PATH|HOME|USER|MCP)" >> "$LOG_FILE"

# Run the actual MCP server with logging
echo "[$(date)] Starting MCP server..." >> "$LOG_FILE"

# Use tee to capture both stdin and stdout
exec 3>&1 4>&2
tee -a "$LOG_FILE" | /Users/wei/Projects/mac-keyboard-mcp/target/release/mac-keyboard-mcp 2>>"$LOG_FILE" | tee -a "$LOG_FILE" >&3 2>&4

echo "[$(date)] MCP Server Wrapper Ended" >> "$LOG_FILE"