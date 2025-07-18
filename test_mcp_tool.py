#!/usr/bin/env python3
"""Test the MCP server tool response format."""

import json
import subprocess
import sys

def send_request(request):
    """Send a JSON-RPC request to the MCP server."""
    proc = subprocess.Popen(
        ["/Users/wei/Projects/mac-keyboard-mcp/target/release/mac-keyboard-mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    stdout, stderr = proc.communicate(json.dumps(request) + "\n")
    
    if stderr:
        print(f"Server logs:\n{stderr}", file=sys.stderr)
    
    return json.loads(stdout.strip())

def test_tool_response():
    """Test that tool responses have the correct format."""
    # Initialize
    init_request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {}
    }
    
    response = send_request(init_request)
    print("Initialize response:", json.dumps(response, indent=2))
    
    # Test lookup_keycode tool
    lookup_request = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "lookup_keycode",
            "arguments": {
                "key_name": "F17"
            }
        }
    }
    
    proc = subprocess.Popen(
        ["/Users/wei/Projects/mac-keyboard-mcp/target/release/mac-keyboard-mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # Send both requests
    proc.stdin.write(json.dumps(init_request) + "\n")
    proc.stdin.write(json.dumps(lookup_request) + "\n")
    proc.stdin.flush()
    
    # Read responses
    init_response = json.loads(proc.stdout.readline().strip())
    lookup_response = json.loads(proc.stdout.readline().strip())
    
    print("\nLookup response:", json.dumps(lookup_response, indent=2))
    
    # Verify response structure
    if "result" in lookup_response and "content" in lookup_response["result"]:
        content = lookup_response["result"]["content"]
        if isinstance(content, list) and len(content) > 0:
            first_item = content[0]
            if "type" in first_item and first_item["type"] == "text":
                print("\n✅ Response format is correct!")
                print("Tool output:", first_item["text"])
                
                # Parse the actual tool output
                tool_result = json.loads(first_item["text"])
                if tool_result.get("found") and tool_result["key"]["code"] == 160:
                    print("✅ F17 key code is correct: 160")
                else:
                    print("❌ Unexpected tool result")
            else:
                print("❌ Missing 'type': 'text' in content")
        else:
            print("❌ Content is not a list or is empty")
    else:
        print("❌ Missing result.content in response")
    
    proc.terminate()

if __name__ == "__main__":
    test_tool_response()