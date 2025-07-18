#!/usr/bin/env python3
"""
Quick automated test of key codes
"""

import json
import subprocess
import time
from datetime import datetime

def test_key(name, code):
    """Test a single key code"""
    script = f'tell application "System Events" to key code {code}'
    
    try:
        result = subprocess.run(
            ['osascript', '-e', script],
            capture_output=True,
            text=True,
            timeout=2
        )
        return result.returncode == 0, result.stderr if result.returncode != 0 else "Success"
    except Exception as e:
        return False, str(e)

def main():
    print("🎹 Quick Key Code Test")
    print("=" * 40)
    print(f"Time: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    
    # Test a few key codes
    test_cases = [
        ("A", 0),
        ("Space", 49),
        ("Return", 36),
        ("Command", 55),
        ("F1", 122),
        ("F17", 160),  # The one we fixed!
        ("LeftArrow", 123),
        ("Escape", 53),
    ]
    
    results = []
    passed = 0
    
    print("\n🧪 Testing key codes...")
    for name, code in test_cases:
        print(f"  {name:12} (code {code:3})... ", end='', flush=True)
        
        success, message = test_key(name, code)
        
        if success:
            print("✅")
            passed += 1
        else:
            print(f"❌ {message}")
        
        results.append({
            "key": name,
            "code": code,
            "passed": success,
            "message": message
        })
        
        time.sleep(0.1)  # Small delay between tests
    
    # Summary
    total = len(test_cases)
    print(f"\n📊 Summary: {passed}/{total} passed ({passed/total*100:.0f}%)")
    
    # Test a shortcut
    print("\n🔗 Testing shortcut (Cmd+A)...")
    script = 'tell application "System Events" to key code 0 using command down'
    result = subprocess.run(['osascript', '-e', script], capture_output=True, text=True)
    
    if result.returncode == 0:
        print("✅ Shortcut test passed")
    else:
        print(f"❌ Shortcut test failed: {result.stderr}")
    
    # Save results
    with open("test_results.json", "w") as f:
        json.dump({
            "timestamp": datetime.now().isoformat(),
            "results": results,
            "summary": {
                "total": total,
                "passed": passed,
                "failed": total - passed
            }
        }, f, indent=2)
    
    print("\n💾 Results saved to test_results.json")

if __name__ == "__main__":
    main()