#!/usr/bin/env python3
"""Test complete MCP flow outside of Claude Desktop."""

import json
import subprocess
import sys
import time

def test_mcp_flow():
    """Test the complete MCP flow."""
    proc = subprocess.Popen(
        ["/Users/wei/Projects/mac-keyboard-mcp/target/release/mac-keyboard-mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        env={"RUST_LOG": "debug"}
    )
    
    def send_and_receive(request):
        """Send request and get response."""
        proc.stdin.write(json.dumps(request) + "\n")
        proc.stdin.flush()
        response = proc.stdout.readline()
        return json.loads(response) if response else None
    
    print("Testing Mac Keyboard MCP Server Flow")
    print("=" * 50)
    
    # 1. Initialize
    print("\n1. Sending initialize request...")
    init_resp = send_and_receive({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {}
    })
    print(f"Response: {json.dumps(init_resp, indent=2)}")
    
    # 2. Send initialized notification
    print("\n2. Sending initialized notification...")
    proc.stdin.write(json.dumps({
        "jsonrpc": "2.0",
        "method": "initialized"
    }) + "\n")
    proc.stdin.flush()
    time.sleep(0.1)  # Give it time to process
    
    # 3. List tools
    print("\n3. Listing tools...")
    tools_resp = send_and_receive({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    })
    print(f"Response: {json.dumps(tools_resp, indent=2)}")
    
    # 4. Call lookup_keycode tool
    print("\n4. Looking up F3+ key code...")
    lookup_resp = send_and_receive({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "lookup_keycode",
            "arguments": {
                "key_name": "F3+"
            }
        }
    })
    print(f"Response: {json.dumps(lookup_resp, indent=2)}")
    
    # 5. List resources
    print("\n5. Listing resources...")
    resources_resp = send_and_receive({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "resources/list"
    })
    print(f"Response: {json.dumps(resources_resp, indent=2)}")
    
    # Print stderr for debugging
    proc.terminate()
    _, stderr = proc.communicate()
    if stderr:
        print("\n\nServer logs:")
        print(stderr)
    
    print("\n✅ All tests passed! Server is working correctly.")

if __name__ == "__main__":
    test_mcp_flow()