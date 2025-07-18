#!/usr/bin/env python3
"""
Test MCP server connectivity and protocol implementation
"""

import json
import subprocess
import sys
from typing import Dict, Any, Optional

class MCPServerTester:
    def __init__(self, server_path: str):
        self.server_path = server_path
        self.process = None
        self.request_id = 0
    
    def start_server(self):
        """Start the MCP server process"""
        print(f"Starting server: {self.server_path}")
        self.process = subprocess.Popen(
            [self.server_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=0
        )
        print("Server started, PID:", self.process.pid)
    
    def send_request(self, method: str, params: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Send a JSON-RPC request and get response"""
        self.request_id += 1
        request = {
            "jsonrpc": "2.0",
            "id": self.request_id,
            "method": method
        }
        if params is not None:
            request["params"] = params
        
        request_str = json.dumps(request)
        print(f"\n→ Sending: {request_str}")
        
        # Send request
        self.process.stdin.write(request_str + "\n")
        self.process.stdin.flush()
        
        # Read response
        response_line = self.process.stdout.readline()
        print(f"← Received: {response_line.strip()}")
        
        if response_line:
            return json.loads(response_line)
        else:
            # Check stderr for errors
            stderr_output = self.process.stderr.read()
            if stderr_output:
                print(f"Server stderr: {stderr_output}")
            return None
    
    def test_initialize(self):
        """Test the initialize method"""
        print("\n=== Testing initialize ===")
        response = self.send_request("initialize", {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "test-client",
                "version": "1.0.0"
            }
        })
        
        if response and "result" in response:
            print("✅ Initialize successful")
            print(f"   Protocol version: {response['result'].get('protocolVersion')}")
            print(f"   Server: {response['result'].get('serverInfo', {}).get('name')}")
            return True
        else:
            print("❌ Initialize failed")
            return False
    
    def test_initialized(self):
        """Test the initialized notification"""
        print("\n=== Testing initialized ===")
        response = self.send_request("initialized")
        
        if response and "result" in response:
            print("✅ Initialized notification successful")
            return True
        else:
            print("❌ Initialized notification failed")
            return False
    
    def test_tools_list(self):
        """Test listing tools"""
        print("\n=== Testing tools/list ===")
        response = self.send_request("tools/list")
        
        if response and "result" in response:
            result = response["result"]
            # Handle both formats (list or object with tools property)
            if isinstance(result, list):
                tools = result
            else:
                tools = result.get("tools", [])
            
            print(f"✅ Found {len(tools)} tools:")
            for tool in tools:
                print(f"   - {tool.get('name')}: {tool.get('description')}")
            return True
        else:
            print("❌ Tools list failed")
            return False
    
    def test_tool_call(self):
        """Test calling a tool"""
        print("\n=== Testing tools/call ===")
        response = self.send_request("tools/call", {
            "name": "lookup_keycode",
            "arguments": {
                "key_name": "Space",
                "fuzzy": False
            }
        })
        
        if response and "result" in response:
            print("✅ Tool call successful")
            content = response["result"].get("content", [])
            if content:
                print(f"   Result: {json.dumps(content[0], indent=2)}")
            return True
        else:
            print("❌ Tool call failed")
            if response and "error" in response:
                print(f"   Error: {response['error']}")
            return False
    
    def test_resources_list(self):
        """Test listing resources"""
        print("\n=== Testing resources/list ===")
        response = self.send_request("resources/list")
        
        if response and "result" in response:
            result = response["result"]
            # Handle both formats
            if isinstance(result, list):
                resources = result
            else:
                resources = result.get("resources", [])
                
            print(f"✅ Found {len(resources)} resources:")
            for res in resources[:5]:  # Show first 5
                print(f"   - {res.get('uri')}: {res.get('name')}")
            if len(resources) > 5:
                print(f"   ... and {len(resources) - 5} more")
            return True
        else:
            print("❌ Resources list failed")
            return False
    
    def test_resource_read(self):
        """Test reading a resource"""
        print("\n=== Testing resources/read ===")
        response = self.send_request("resources/read", {
            "uri": "keycode://aliases"
        })
        
        if response and "result" in response:
            print("✅ Resource read successful")
            contents = response["result"].get("contents", [])
            if contents:
                # Show a sample of the content
                content_str = json.dumps(contents[0], indent=2)
                lines = content_str.split('\n')
                for line in lines[:10]:  # Show first 10 lines
                    print(f"   {line}")
                if len(lines) > 10:
                    print("   ...")
            return True
        else:
            print("❌ Resource read failed")
            return False
    
    def run_all_tests(self):
        """Run all tests"""
        print("🧪 MCP Server Test Suite")
        print("=" * 50)
        
        try:
            self.start_server()
            
            # Give server a moment to start
            import time
            time.sleep(0.5)
            
            # Run tests in order
            tests = [
                self.test_initialize,
                self.test_initialized,
                self.test_tools_list,
                self.test_tool_call,
                self.test_resources_list,
                self.test_resource_read
            ]
            
            passed = 0
            for test in tests:
                if test():
                    passed += 1
                time.sleep(0.1)  # Small delay between tests
            
            print(f"\n📊 Results: {passed}/{len(tests)} tests passed")
            
            # Test error handling
            print("\n=== Testing error handling ===")
            response = self.send_request("unknown/method")
            if response and "error" in response:
                print("✅ Error handling works correctly")
                print(f"   Error: {response['error'].get('message')}")
            
        except Exception as e:
            print(f"\n❌ Test failed with exception: {e}")
            import traceback
            traceback.print_exc()
        
        finally:
            # Clean up
            if self.process:
                print("\nTerminating server...")
                self.process.terminate()
                self.process.wait()
                print("Server terminated")

def main():
    if len(sys.argv) > 1:
        server_path = sys.argv[1]
    else:
        server_path = "../target/release/mac-keyboard-mcp"
    
    tester = MCPServerTester(server_path)
    tester.run_all_tests()

if __name__ == "__main__":
    main()