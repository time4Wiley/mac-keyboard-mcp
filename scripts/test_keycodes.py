#!/usr/bin/env python3
"""
Test specific key codes on macOS
"""

import json
import subprocess
import time
import sys
from pathlib import Path

def load_keycodes():
    """Load key codes from JSON"""
    with open("keycodes.json", 'r') as f:
        return json.load(f)

def test_keycode(name, code):
    """Test a single key code"""
    print(f"\n🔑 Testing: {name} (code {code})")
    print("⏳ Sending key code in 2 seconds...")
    print("👀 Watch for the key press effect!")
    
    time.sleep(2)
    
    # Open TextEdit to see the effect
    subprocess.run(['open', '-a', 'TextEdit'])
    time.sleep(1)
    
    # Send the key code
    script = f'tell application "System Events" to key code {code}'
    result = subprocess.run(['osascript', '-e', script], capture_output=True, text=True)
    
    if result.returncode == 0:
        print("✅ Key code sent successfully!")
        return True
    else:
        print(f"❌ Error: {result.stderr}")
        return False

def test_modifier_combo(mod_name, mod_code, key_name, key_code):
    """Test a modifier + key combination"""
    print(f"\n🔑 Testing: {mod_name}+{key_name} (codes {mod_code}+{key_code})")
    print("⏳ Sending combination in 2 seconds...")
    
    time.sleep(2)
    
    # Open TextEdit
    subprocess.run(['open', '-a', 'TextEdit'])
    time.sleep(1)
    
    # Send the combination
    modifier_map = {
        55: "command down",
        56: "shift down", 
        58: "option down",
        59: "control down"
    }
    
    modifier = modifier_map.get(mod_code, "command down")
    script = f'tell application "System Events" to key code {key_code} using {modifier}'
    
    result = subprocess.run(['osascript', '-e', script], capture_output=True, text=True)
    
    if result.returncode == 0:
        print("✅ Combination sent successfully!")
        return True
    else:
        print(f"❌ Error: {result.stderr}")
        return False

def main():
    keycodes = load_keycodes()
    
    print("🎹 Mac Keyboard Code Tester")
    print("=" * 40)
    print("\n📋 Test Categories:")
    print("1. Basic Letters (A, B, C)")
    print("2. Numbers (1, 2, 3)")
    print("3. Function Keys (F1, F17, F18)")
    print("4. Special Keys (Space, Return, Tab)")
    print("5. Modifiers + Letters (Cmd+A, Shift+B)")
    print("6. Arrow Keys")
    print("7. Custom Test")
    
    choice = input("\nSelect test (1-7): ").strip()
    
    if choice == "1":
        # Test basic letters
        test_keycode("A", 0)
        test_keycode("B", 11)
        test_keycode("C", 8)
        
    elif choice == "2":
        # Test numbers
        test_keycode("1", 18)
        test_keycode("2", 19)
        test_keycode("3", 20)
        
    elif choice == "3":
        # Test function keys
        test_keycode("F1", 122)
        test_keycode("F17", 160)  # The missing one we fixed!
        test_keycode("F18", 131)
        
    elif choice == "4":
        # Test special keys
        test_keycode("Space", 49)
        test_keycode("Return", 36)
        test_keycode("Tab", 48)
        
    elif choice == "5":
        # Test modifier combinations
        test_modifier_combo("Command", 55, "A", 0)
        test_modifier_combo("Shift", 56, "B", 11)
        test_modifier_combo("Option", 58, "C", 8)
        
    elif choice == "6":
        # Test arrow keys
        test_keycode("LeftArrow", 123)
        test_keycode("RightArrow", 124)
        test_keycode("UpArrow", 126)
        test_keycode("DownArrow", 125)
        
    elif choice == "7":
        # Custom test
        key_name = input("Enter key name: ").strip()
        
        # Search for the key
        found = False
        for category, keys in keycodes.items():
            if key_name in keys:
                code = keys[key_name]
                print(f"Found in {category}: {key_name} = {code}")
                test_keycode(key_name, code)
                found = True
                break
        
        if not found:
            print(f"❌ Key '{key_name}' not found in database")
            print("\nAvailable keys:")
            for category, keys in keycodes.items():
                print(f"\n{category}: {', '.join(list(keys.keys())[:5])}...")
    
    print("\n✨ Test complete!")

if __name__ == "__main__":
    main()