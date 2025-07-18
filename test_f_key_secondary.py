#!/usr/bin/env python3
"""Test F-key secondary function queries."""

import json
import subprocess
import sys

def send_requests(requests):
    """Send multiple JSON-RPC requests to the MCP server."""
    proc = subprocess.Popen(
        ["/Users/wei/Projects/mac-keyboard-mcp/target/release/mac-keyboard-mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # Send all requests
    for req in requests:
        proc.stdin.write(json.dumps(req) + "\n")
    proc.stdin.flush()
    
    # Read responses
    responses = []
    for _ in requests:
        line = proc.stdout.readline().strip()
        if line:
            responses.append(json.loads(line))
    
    proc.terminate()
    return responses

def test_f_key_secondary():
    """Test F-key secondary function lookups."""
    # Initialize
    init_request = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {}
    }
    
    # Test various F-key secondary function queries
    test_queries = [
        ("F1+", "Brightness Down"),
        ("F2+", "Brightness Up"),
        ("F3+", "Mission Control"),
        ("F4+", "Launchpad"),
        ("F10+", "Mute"),
        ("F11+", "Volume Down"),
        ("F12+", "Volume Up"),
    ]
    
    requests = [init_request]
    
    for i, (query, expected_name) in enumerate(test_queries, start=2):
        requests.append({
            "jsonrpc": "2.0",
            "id": i,
            "method": "tools/call",
            "params": {
                "name": "lookup_keycode",
                "arguments": {
                    "key_name": query
                }
            }
        })
    
    responses = send_requests(requests)
    
    print("F-Key Secondary Function Test Results")
    print("=" * 50)
    
    # Skip init response
    for i, (query, expected_name) in enumerate(test_queries, start=1):
        response = responses[i]
        
        if "result" in response and "content" in response["result"]:
            content = response["result"]["content"][0]["text"]
            result = json.loads(content)
            
            if result.get("found") and result.get("query_type") == "secondary_function":
                f_key_info = result["f_key"]
                primary = f_key_info["primary_function"]
                secondary = f_key_info["secondary_function"]
                
                print(f"\n{query} lookup:")
                print(f"  Primary: {primary['name']} (code {primary['code']})")
                print(f"  Secondary: {secondary['name']} (code {secondary['code']})")
                print(f"  Description: {secondary['description']}")
                
                if secondary['name'] == expected_name:
                    print(f"  ✅ Correct secondary function!")
                else:
                    print(f"  ❌ Expected '{expected_name}', got '{secondary['name']}'")
            else:
                print(f"\n❌ {query}: Unexpected response format")
        else:
            print(f"\n❌ {query}: Error in response")
    
    # Also test regular F-key lookup (without +)
    print("\n\nRegular F-key lookup (F3 without +):")
    regular_request = {
        "jsonrpc": "2.0",
        "id": 100,
        "method": "tools/call",
        "params": {
            "name": "lookup_keycode",
            "arguments": {
                "key_name": "F3"
            }
        }
    }
    
    responses = send_requests([init_request, regular_request])
    response = responses[1]
    
    if "result" in response and "content" in response["result"]:
        content = response["result"]["content"][0]["text"]
        result = json.loads(content)
        
        if result.get("found"):
            key = result["key"]
            print(f"  Name: {key['name']}")
            print(f"  Code: {key['code']}")
            print(f"  Category: {key['category']}")
            print(f"  ✅ Regular F-key lookup works correctly")

if __name__ == "__main__":
    test_f_key_secondary()