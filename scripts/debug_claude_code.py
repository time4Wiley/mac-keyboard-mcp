#!/usr/bin/env python3
"""
Debug Claude Code MCP connectivity
"""

import json
import sys
import time
from datetime import datetime

def log(message):
    """Log to stderr with timestamp"""
    timestamp = datetime.now().strftime("%H:%M:%S.%f")[:-3]
    print(f"[{timestamp}] {message}", file=sys.stderr)

def main():
    log("Mac Keyboard MCP Debug Server Started")
    log("Waiting for requests on stdin...")
    
    request_count = 0
    
    # Read from stdin line by line
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        
        request_count += 1
        log(f"Request #{request_count}: {line}")
        
        try:
            request = json.loads(line)
            method = request.get("method", "unknown")
            request_id = request.get("id")
            
            # Create a simple response based on method
            if method == "initialize":
                response = {
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {},
                        "serverInfo": {
                            "name": "mac-keyboard-mcp-debug",
                            "version": "0.1.0"
                        }
                    }
                }
            elif method == "initialized":
                response = {
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "result": {}
                }
            elif method == "tools/list":
                response = {
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "result": {
                        "tools": [{
                            "name": "test_tool",
                            "description": "A test tool",
                            "inputSchema": {
                                "type": "object",
                                "properties": {}
                            }
                        }]
                    }
                }
            else:
                response = {
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "error": {
                        "code": -32601,
                        "message": f"Method not found: {method}"
                    }
                }
            
            response_str = json.dumps(response)
            log(f"Sending response: {response_str}")
            print(response_str, flush=True)
            
        except Exception as e:
            log(f"Error processing request: {e}")
            error_response = {
                "jsonrpc": "2.0",
                "id": None,
                "error": {
                    "code": -32700,
                    "message": "Parse error",
                    "data": str(e)
                }
            }
            print(json.dumps(error_response), flush=True)
    
    log("Server shutting down")

if __name__ == "__main__":
    main()