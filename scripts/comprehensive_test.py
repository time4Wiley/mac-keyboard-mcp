#!/usr/bin/env python3
"""
Comprehensive key code verification with detailed reporting
"""

import json
import subprocess
import time
from datetime import datetime
from pathlib import Path

class ComprehensiveVerifier:
    def __init__(self):
        self.keycodes = self.load_keycodes()
        self.results = {
            "metadata": {
                "timestamp": datetime.now().isoformat(),
                "platform": "macOS",
                "test_type": "comprehensive"
            },
            "categories": {},
            "special_tests": {},
            "summary": {}
        }
    
    def load_keycodes(self):
        """Load key codes from JSON"""
        with open("keycodes.json", 'r') as f:
            return json.load(f)
    
    def test_key(self, name, code):
        """Test a single key code"""
        script = f'tell application "System Events" to key code {code}'
        
        try:
            result = subprocess.run(
                ['osascript', '-e', script],
                capture_output=True,
                text=True,
                timeout=1
            )
            return result.returncode == 0
        except:
            return False
    
    def test_category(self, category_name, sample_size=5):
        """Test a sample from each category"""
        if category_name not in self.keycodes:
            return
        
        keys = self.keycodes[category_name]
        # Select keys to test
        if len(keys) <= sample_size:
            test_keys = list(keys.items())
        else:
            # Test first, last, and some middle keys
            all_keys = list(keys.items())
            indices = [0, len(all_keys)//4, len(all_keys)//2, 3*len(all_keys)//4, -1]
            test_keys = [all_keys[i] for i in indices if i < len(all_keys)]
        
        results = []
        for key_name, code in test_keys:
            success = self.test_key(key_name, code)
            results.append({
                "key": key_name,
                "code": code,
                "passed": success
            })
            time.sleep(0.05)  # Small delay
        
        self.results["categories"][category_name] = {
            "total_keys": len(keys),
            "tested": len(test_keys),
            "results": results,
            "passed": sum(1 for r in results if r["passed"])
        }
    
    def test_special_cases(self):
        """Test special cases and known issues"""
        special_tests = {
            "F17_key": {
                "description": "Testing F17 (code 160) - previously missing",
                "tests": [("F17", 160)]
            },
            "corrected_f_keys": {
                "description": "Testing corrected F-keys",
                "tests": [
                    ("F7", 98),   # Was 131
                    ("F8", 100),  # Was 109
                    ("F10", 109), # Was 100
                    ("F18", 131)  # Was 79
                ]
            },
            "modifier_combinations": {
                "description": "Testing modifier combinations",
                "tests": []  # Will be tested differently
            }
        }
        
        for test_name, test_info in special_tests.items():
            if test_name == "modifier_combinations":
                # Test some common shortcuts
                shortcuts = [
                    ("Cmd+A", 'key code 0 using command down'),
                    ("Cmd+C", 'key code 8 using command down'),
                    ("Shift+Tab", 'key code 48 using shift down'),
                    ("Option+Left", 'key code 123 using option down')
                ]
                
                results = []
                for name, script_part in shortcuts:
                    script = f'tell application "System Events" to {script_part}'
                    try:
                        result = subprocess.run(
                            ['osascript', '-e', script],
                            capture_output=True,
                            text=True,
                            timeout=1
                        )
                        success = result.returncode == 0
                    except:
                        success = False
                    
                    results.append({
                        "shortcut": name,
                        "passed": success
                    })
                    time.sleep(0.05)
                
                self.results["special_tests"][test_name] = {
                    "description": test_info["description"],
                    "results": results,
                    "passed": sum(1 for r in results if r["passed"]),
                    "total": len(results)
                }
            else:
                results = []
                for key_name, code in test_info["tests"]:
                    success = self.test_key(key_name, code)
                    results.append({
                        "key": key_name,
                        "code": code,
                        "passed": success
                    })
                    time.sleep(0.05)
                
                self.results["special_tests"][test_name] = {
                    "description": test_info["description"],
                    "results": results,
                    "passed": sum(1 for r in results if r["passed"]),
                    "total": len(results)
                }
    
    def generate_summary(self):
        """Generate test summary"""
        total_tested = 0
        total_passed = 0
        
        # Category tests
        for cat_data in self.results["categories"].values():
            total_tested += cat_data["tested"]
            total_passed += cat_data["passed"]
        
        # Special tests
        for test_data in self.results["special_tests"].values():
            total_tested += test_data["total"]
            total_passed += test_data["passed"]
        
        self.results["summary"] = {
            "total_keys_in_database": sum(len(keys) for keys in self.keycodes.values()),
            "total_tested": total_tested,
            "total_passed": total_passed,
            "total_failed": total_tested - total_passed,
            "pass_rate": f"{(total_passed/total_tested*100):.1f}%" if total_tested > 0 else "0%",
            "categories_tested": len(self.results["categories"]),
            "special_tests_run": len(self.results["special_tests"])
        }
    
    def print_report(self):
        """Print formatted report"""
        print("\n" + "="*60)
        print("🎹 COMPREHENSIVE KEY CODE VERIFICATION REPORT")
        print("="*60)
        print(f"📅 Date: {self.results['metadata']['timestamp']}")
        print(f"💻 Platform: {self.results['metadata']['platform']}")
        
        # Category results
        print("\n📂 CATEGORY TESTS:")
        print("-"*40)
        for category, data in self.results["categories"].items():
            status = "✅" if data["passed"] == data["tested"] else "⚠️"
            print(f"{status} {category:15} - {data['passed']}/{data['tested']} passed (of {data['total_keys']} total)")
            
            # Show any failures
            failures = [r for r in data["results"] if not r["passed"]]
            if failures:
                for f in failures:
                    print(f"   ❌ {f['key']} (code {f['code']})")
        
        # Special tests
        print("\n🔬 SPECIAL TESTS:")
        print("-"*40)
        for test_name, data in self.results["special_tests"].items():
            status = "✅" if data["passed"] == data["total"] else "⚠️"
            print(f"{status} {data['description']}")
            print(f"   Results: {data['passed']}/{data['total']} passed")
            
            # Show details
            if "results" in data and data["results"]:
                for r in data["results"]:
                    if "key" in r:
                        symbol = "✅" if r["passed"] else "❌"
                        print(f"   {symbol} {r['key']} (code {r['code']})")
                    elif "shortcut" in r:
                        symbol = "✅" if r["passed"] else "❌"
                        print(f"   {symbol} {r['shortcut']}")
        
        # Summary
        print("\n📊 SUMMARY:")
        print("-"*40)
        for key, value in self.results["summary"].items():
            print(f"{key.replace('_', ' ').title():25} {value}")
        
        print("\n" + "="*60)
    
    def save_report(self):
        """Save detailed report to file"""
        report_path = Path("verification_report_comprehensive.json")
        with open(report_path, 'w') as f:
            json.dump(self.results, f, indent=2)
        print(f"\n💾 Detailed report saved to: {report_path}")
    
    def run(self):
        """Run comprehensive verification"""
        print("🚀 Starting comprehensive key code verification...")
        print("This will test a sample of keys from each category")
        print("and verify special cases including F17 and corrected F-keys\n")
        
        # Test each category
        for category in self.keycodes.keys():
            print(f"Testing {category}...", end='', flush=True)
            self.test_category(category)
            print(" done")
        
        # Test special cases
        print("Testing special cases...", end='', flush=True)
        self.test_special_cases()
        print(" done")
        
        # Generate summary
        self.generate_summary()
        
        # Print and save report
        self.print_report()
        self.save_report()

def main():
    verifier = ComprehensiveVerifier()
    verifier.run()

if __name__ == "__main__":
    main()