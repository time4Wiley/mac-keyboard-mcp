#!/usr/bin/env python3
"""
Test the exact Claude Code MCP protocol sequence
"""

import json
import subprocess
import time
import threading
from queue import Queue

class ClaudeProtocolTester:
    def __init__(self, server_path):
        self.server_path = server_path
        self.process = None
        self.stderr_queue = Queue()
        self.stdout_queue = Queue()
        
    def stderr_reader(self):
        """Read stderr in a separate thread"""
        for line in iter(self.process.stderr.readline, ''):
            if line:
                print(f"[STDERR] {line.strip()}")
                self.stderr_queue.put(line)
    
    def stdout_reader(self):
        """Read stdout in a separate thread"""
        for line in iter(self.process.stdout.readline, ''):
            if line:
                self.stdout_queue.put(line)
    
    def start_server(self):
        """Start the server process"""
        print(f"Starting server: {self.server_path}")
        self.process = subprocess.Popen(
            [self.server_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=0
        )
        
        # Start reader threads
        stderr_thread = threading.Thread(target=self.stderr_reader)
        stderr_thread.daemon = True
        stderr_thread.start()
        
        stdout_thread = threading.Thread(target=self.stdout_reader)
        stdout_thread.daemon = True
        stdout_thread.start()
        
        time.sleep(0.5)  # Give server time to start
    
    def send_request(self, request):
        """Send a request and wait for response"""
        request_str = json.dumps(request)
        print(f"\n→ Sending: {request_str}")
        self.process.stdin.write(request_str + "\n")
        self.process.stdin.flush()
        
        # Wait for response
        start_time = time.time()
        while time.time() - start_time < 5:  # 5 second timeout
            try:
                line = self.stdout_queue.get(timeout=0.1)
                print(f"← Received: {line.strip()}")
                return json.loads(line)
            except:
                continue
        
        print("⚠️  No response received within 5 seconds")
        return None
    
    def test_claude_sequence(self):
        """Test the exact sequence Claude Code uses"""
        print("\n=== Testing Claude Code Protocol Sequence ===")
        
        # Step 1: Initialize
        print("\n1. Sending initialize request...")
        init_response = self.send_request({
            "jsonrpc": "2.0",
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "roots": {
                        "listChanged": True
                    },
                    "sampling": {}
                },
                "clientInfo": {
                    "name": "Claude Desktop",
                    "version": "0.7.27"
                }
            },
            "id": 0
        })
        
        if init_response:
            print("✅ Initialize response received")
            print(f"   Protocol version: {init_response.get('result', {}).get('protocolVersion')}")
        else:
            print("❌ No initialize response")
            return
        
        # Step 2: Send initialized notification
        print("\n2. Sending initialized notification...")
        self.send_request({
            "jsonrpc": "2.0",
            "method": "initialized",
            "params": {}
        })
        print("✅ Initialized notification sent")
        
        # Step 3: List tools
        print("\n3. Listing tools...")
        tools_response = self.send_request({
            "jsonrpc": "2.0",
            "method": "tools/list",
            "params": {},
            "id": 1
        })
        
        if tools_response and "result" in tools_response:
            tools = tools_response["result"].get("tools", [])
            print(f"✅ Found {len(tools)} tools")
        
        # Step 4: Keep connection alive
        print("\n4. Keeping connection alive...")
        print("   Server should stay running, not exit")
        
        # Wait a bit to see if server stays alive
        time.sleep(2)
        
        if self.process.poll() is None:
            print("✅ Server is still running")
            
            # Test a tool call
            print("\n5. Testing tool call...")
            tool_response = self.send_request({
                "jsonrpc": "2.0",
                "method": "tools/call",
                "params": {
                    "name": "lookup_keycode",
                    "arguments": {
                        "key_name": "A"
                    }
                },
                "id": 2
            })
            
            if tool_response:
                print("✅ Tool call successful")
        else:
            print("❌ Server exited prematurely")
            print(f"   Exit code: {self.process.returncode}")
    
    def run(self):
        """Run the test"""
        try:
            self.start_server()
            self.test_claude_sequence()
            
            print("\n=== Test Complete ===")
            print("Waiting 5 seconds to observe server behavior...")
            time.sleep(5)
            
            if self.process.poll() is None:
                print("✅ Server is still running (good!)")
                print("Terminating server...")
                self.process.terminate()
            else:
                print(f"❌ Server exited with code: {self.process.returncode}")
            
        except Exception as e:
            print(f"❌ Test failed: {e}")
            import traceback
            traceback.print_exc()
        finally:
            if self.process and self.process.poll() is None:
                self.process.terminate()

def main():
    server_path = "../target/release/mac-keyboard-mcp"
    tester = ClaudeProtocolTester(server_path)
    tester.run()

if __name__ == "__main__":
    main()