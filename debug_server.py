#!/usr/bin/env python3
"""Debug server to capture raw MCP communication."""

import sys
import json
import datetime

# Log file
log_file = open("mcp_debug.log", "w")

def log(message):
    timestamp = datetime.datetime.now().isoformat()
    log_file.write(f"[{timestamp}] {message}\n")
    log_file.flush()

log("Debug server started")

try:
    while True:
        # Read from stdin
        line = sys.stdin.readline()
        if not line:
            break
            
        log(f"RECEIVED: {repr(line)}")
        
        # Try to parse as JSON
        try:
            data = json.loads(line.strip())
            log(f"PARSED: {json.dumps(data, indent=2)}")
            
            # Respond based on method
            response = None
            if data.get("method") == "initialize":
                response = {
                    "jsonrpc": "2.0",
                    "id": data.get("id"),
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {
                            "tools": {},
                            "resources": {},
                            "prompts": {},
                            "logging": {}
                        },
                        "serverInfo": {
                            "name": "mac-keyboard-mcp",
                            "version": "0.1.0"
                        }
                    }
                }
            elif data.get("method") == "initialized":
                # This is a notification, might not need response
                if data.get("id") is not None:
                    response = {
                        "jsonrpc": "2.0",
                        "id": data.get("id"),
                        "result": {}
                    }
            elif data.get("method") == "tools/list":
                response = {
                    "jsonrpc": "2.0",
                    "id": data.get("id"),
                    "result": {
                        "tools": [{
                            "name": "lookup_keycode",
                            "description": "Find AppleScript key code",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "key_name": {"type": "string"}
                                },
                                "required": ["key_name"]
                            }
                        }]
                    }
                }
            else:
                # Unknown method
                response = {
                    "jsonrpc": "2.0",
                    "id": data.get("id"),
                    "result": None,
                    "error": {
                        "code": -32601,
                        "message": f"Method not found: {data.get('method')}"
                    }
                }
            
            if response:
                response_str = json.dumps(response)
                log(f"SENDING: {response_str}")
                sys.stdout.write(response_str + "\n")
                sys.stdout.flush()
                
        except json.JSONDecodeError as e:
            log(f"JSON PARSE ERROR: {e}")
            error_response = {
                "jsonrpc": "2.0",
                "id": None,
                "error": {
                    "code": -32700,
                    "message": "Parse error"
                }
            }
            sys.stdout.write(json.dumps(error_response) + "\n")
            sys.stdout.flush()
            
except Exception as e:
    log(f"ERROR: {e}")
    
log("Debug server stopped")
log_file.close()