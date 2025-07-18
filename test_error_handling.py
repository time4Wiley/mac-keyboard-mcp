#!/usr/bin/env python3
"""Test MCP server error handling for malformed requests."""

import json
import subprocess
import sys

def send_raw_requests(raw_requests):
    """Send raw JSON strings to the MCP server."""
    proc = subprocess.Popen(
        ["/Users/wei/Projects/mac-keyboard-mcp/target/release/mac-keyboard-mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    responses = []
    
    for req in raw_requests:
        proc.stdin.write(req + "\n")
        proc.stdin.flush()
        
        # Read response
        line = proc.stdout.readline().strip()
        if line:
            try:
                responses.append(json.loads(line))
            except json.JSONDecodeError:
                responses.append({"error": "Failed to parse response", "raw": line})
    
    proc.terminate()
    _, stderr = proc.communicate()
    
    return responses, stderr

def test_error_handling():
    """Test various malformed request scenarios."""
    print("Testing MCP Server Error Handling")
    print("=" * 50)
    
    # Test cases that match the Zod errors from the screenshot
    test_cases = [
        # Valid initialization
        ('{"jsonrpc": "2.0", "id": 1, "method": "initialize"}', "Valid initialization"),
        
        # Missing method (Zod error: "Required")
        ('{"jsonrpc": "2.0", "id": 2}', "Missing method field"),
        
        # Null ID (Zod error: "Expected string, received null")
        ('{"jsonrpc": "2.0", "id": null, "method": "test"}', "Null ID"),
        
        # Missing ID (notification style)
        ('{"jsonrpc": "2.0", "method": "initialized"}', "Notification (no ID)"),
        
        # Completely malformed JSON
        ('{"invalid json}', "Malformed JSON"),
        
        # Empty object
        ('{}', "Empty object"),
        
        # Wrong types
        ('{"jsonrpc": 2.0, "id": "test", "method": 123}', "Wrong field types"),
        
        # Extra fields (Zod: "Unrecognized key(s)")
        ('{"jsonrpc": "2.0", "id": 3, "method": "test", "extra": "field"}', "Extra fields"),
    ]
    
    raw_requests = [case[0] for case in test_cases]
    descriptions = [case[1] for case in test_cases]
    
    responses, stderr = send_raw_requests(raw_requests)
    
    # Print stderr for debugging
    if stderr:
        print("\nServer logs:")
        print(stderr)
        print("=" * 50)
    
    print("\nTest Results:")
    for i, (desc, response) in enumerate(zip(descriptions, responses)):
        print(f"\n{i+1}. {desc}")
        print(f"   Request: {raw_requests[i]}")
        
        if "error" in response and response["error"] == "Failed to parse response":
            print(f"   ❌ Server sent invalid response: {response['raw']}")
        elif "error" in response:
            error = response["error"]
            print(f"   ✅ Error handled: {error['code']} - {error['message']}")
        elif "result" in response:
            print(f"   ✅ Success response received")
        else:
            print(f"   ⚠️  Unexpected response: {json.dumps(response, indent=2)}")

if __name__ == "__main__":
    test_error_handling()