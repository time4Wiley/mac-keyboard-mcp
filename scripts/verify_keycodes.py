#!/usr/bin/env python3
"""
Automated verification assistant for Mac Keyboard MCP
Helps verify key codes work correctly on macOS
"""

import json
import subprocess
import time
import sys
from typing import Dict, List, Tuple, Optional
from datetime import datetime
import argparse

class KeyCodeVerifier:
    def __init__(self, keycode_file: str = "../data/keycodes.json"):
        self.keycode_file = keycode_file
        self.results = []
        self.keycodes = self.load_keycodes()
        
    def load_keycodes(self) -> Dict:
        """Load key codes from JSON file"""
        try:
            with open(self.keycode_file, 'r') as f:
                return json.load(f)
        except FileNotFoundError:
            # Use embedded data if file not found
            return {
                "letters": {"A": 0, "B": 11, "C": 8, "D": 2, "E": 14},
                "numbers": {"0": 29, "1": 18, "2": 19, "3": 20, "4": 21},
                "special_keys": {"Space": 49, "Return": 36, "Escape": 53, "Tab": 48},
                "modifier_keys": {"Command": 55, "Shift": 56, "Option": 58, "Control": 59}
            }
    
    def test_single_keycode(self, name: str, code: int) -> Tuple[bool, str]:
        """Test a single key code using AppleScript"""
        script = f'tell application "System Events" to key code {code}'
        
        try:
            result = subprocess.run(
                ['osascript', '-e', script],
                capture_output=True,
                text=True,
                timeout=2
            )
            
            if result.returncode == 0:
                return True, "Success"
            else:
                return False, f"Error: {result.stderr.strip()}"
                
        except subprocess.TimeoutExpired:
            return False, "Timeout"
        except Exception as e:
            return False, f"Exception: {str(e)}"
    
    def verify_category(self, category: str, limit: Optional[int] = None) -> List[Dict]:
        """Verify all keys in a category"""
        if category not in self.keycodes:
            print(f"❌ Category '{category}' not found")
            return []
        
        keys = self.keycodes[category]
        if limit:
            keys = dict(list(keys.items())[:limit])
        
        results = []
        print(f"\n🔍 Verifying {category} ({len(keys)} keys)...")
        
        for name, code in keys.items():
            print(f"  Testing {name} (code {code})... ", end='', flush=True)
            
            success, message = self.test_single_keycode(name, code)
            
            if success:
                print("✅")
            else:
                print(f"❌ {message}")
            
            results.append({
                "category": category,
                "key": name,
                "code": code,
                "passed": success,
                "message": message,
                "timestamp": datetime.now().isoformat()
            })
            
            # Small delay between tests
            time.sleep(0.1)
        
        return results
    
    def verify_shortcuts(self) -> List[Dict]:
        """Verify common keyboard shortcuts"""
        shortcuts = [
            ("Select All", [55, 0]),  # Cmd+A
            ("Copy", [55, 8]),        # Cmd+C  
            ("Paste", [55, 9]),       # Cmd+V
            ("Undo", [55, 6]),        # Cmd+Z
            ("Save", [55, 1]),        # Cmd+S
        ]
        
        results = []
        print("\n🔍 Verifying shortcuts...")
        
        for name, codes in shortcuts:
            print(f"  Testing {name}... ", end='', flush=True)
            
            # Build AppleScript for key combination
            if len(codes) == 2:
                script = f'''
                tell application "System Events"
                    key code {codes[1]} using command down
                end tell
                '''
            else:
                script = f'tell application "System Events" to key code {codes[0]}'
            
            try:
                result = subprocess.run(
                    ['osascript', '-e', script],
                    capture_output=True,
                    text=True,
                    timeout=2
                )
                
                success = result.returncode == 0
                message = "Success" if success else result.stderr.strip()
                
            except Exception as e:
                success = False
                message = str(e)
            
            if success:
                print("✅")
            else:
                print(f"❌ {message}")
            
            results.append({
                "type": "shortcut",
                "name": name,
                "codes": codes,
                "passed": success,
                "message": message
            })
            
            time.sleep(0.2)
        
        return results
    
    def interactive_verify(self):
        """Interactive verification mode"""
        print("\n🎯 Interactive Verification Mode")
        print("Type a key name to test, or 'quit' to exit")
        
        while True:
            key_name = input("\nKey to test: ").strip()
            
            if key_name.lower() in ['quit', 'exit', 'q']:
                break
            
            # Search for key in all categories
            found = False
            for category, keys in self.keycodes.items():
                if key_name in keys:
                    code = keys[key_name]
                    print(f"Found in {category}: {key_name} = {code}")
                    
                    success, message = self.test_single_keycode(key_name, code)
                    
                    if success:
                        print(f"✅ Test passed")
                    else:
                        print(f"❌ Test failed: {message}")
                    
                    found = True
                    break
            
            if not found:
                print(f"❌ Key '{key_name}' not found in database")
    
    def generate_report(self, results: List[Dict]) -> Dict:
        """Generate verification report"""
        total = len(results)
        passed = sum(1 for r in results if r.get('passed', False))
        failed = total - passed
        
        report = {
            "timestamp": datetime.now().isoformat(),
            "summary": {
                "total_tests": total,
                "passed": passed,
                "failed": failed,
                "pass_rate": f"{(passed/total*100):.1f}%" if total > 0 else "0%"
            },
            "results": results,
            "system_info": {
                "platform": sys.platform,
                "python_version": sys.version.split()[0]
            }
        }
        
        return report
    
    def save_report(self, report: Dict, filename: str = "verification_report.json"):
        """Save report to file"""
        with open(filename, 'w') as f:
            json.dump(report, f, indent=2)
        print(f"\n📄 Report saved to {filename}")
    
    def quick_verify(self):
        """Quick verification of essential keys"""
        essential_keys = [
            ("letters", ["A", "Z"]),
            ("numbers", ["0", "9"]), 
            ("special_keys", ["Space", "Return", "Escape"]),
            ("modifier_keys", ["Command", "Shift"])
        ]
        
        results = []
        print("\n⚡ Quick Verification (Essential Keys)")
        
        for category, keys_to_test in essential_keys:
            if category in self.keycodes:
                category_keys = self.keycodes[category]
                for key in keys_to_test:
                    if key in category_keys:
                        code = category_keys[key]
                        print(f"  {key} ({code})... ", end='', flush=True)
                        
                        success, message = self.test_single_keycode(key, code)
                        
                        if success:
                            print("✅")
                        else:
                            print(f"❌")
                        
                        results.append({
                            "key": key,
                            "code": code,
                            "passed": success
                        })
                        
                        time.sleep(0.1)
        
        # Summary
        passed = sum(1 for r in results if r['passed'])
        total = len(results)
        print(f"\n✨ Quick Check Complete: {passed}/{total} passed")
        
        return results

def main():
    parser = argparse.ArgumentParser(description="Verify Mac keyboard codes")
    parser.add_argument('--mode', choices=['quick', 'full', 'interactive', 'category'],
                        default='quick', help='Verification mode')
    parser.add_argument('--category', help='Specific category to verify')
    parser.add_argument('--limit', type=int, help='Limit number of keys to test')
    parser.add_argument('--report', action='store_true', help='Generate report file')
    
    args = parser.parse_args()
    
    verifier = KeyCodeVerifier()
    
    print("🎹 Mac Keyboard Code Verifier")
    print("=" * 40)
    
    all_results = []
    
    if args.mode == 'quick':
        results = verifier.quick_verify()
        all_results.extend(results)
        
    elif args.mode == 'full':
        for category in verifier.keycodes:
            results = verifier.verify_category(category, args.limit)
            all_results.extend(results)
        
        # Also test shortcuts
        shortcut_results = verifier.verify_shortcuts()
        all_results.extend(shortcut_results)
        
    elif args.mode == 'category':
        if args.category:
            results = verifier.verify_category(args.category, args.limit)
            all_results.extend(results)
        else:
            print("❌ Please specify --category for category mode")
            return
            
    elif args.mode == 'interactive':
        verifier.interactive_verify()
        return
    
    # Generate and optionally save report
    if all_results and args.report:
        report = verifier.generate_report(all_results)
        verifier.save_report(report)
        
        # Print summary
        print(f"\n📊 Summary:")
        print(f"  Total: {report['summary']['total_tests']}")
        print(f"  Passed: {report['summary']['passed']}")
        print(f"  Failed: {report['summary']['failed']}")
        print(f"  Pass Rate: {report['summary']['pass_rate']}")

if __name__ == "__main__":
    main()