# Human-in-the-Loop Verification Plan

## Overview

This plan ensures quality and correctness at every stage of the mac-keyboard-mcp development through systematic human verification checkpoints. Each stage includes clear acceptance criteria, testing procedures, and fallback strategies.

## Verification Stages

### Stage 1: Data Validation ✓
**Goal**: Ensure key code database accuracy and completeness

#### Checkpoint 1.1: Key Code Accuracy
- **What to Verify**:
  - Each key code maps to the correct key
  - No duplicate or conflicting mappings
  - All standard keys are included
  
- **Verification Method**:
  ```bash
  # Test script to verify key codes
  cd analysis
  python3 verify_keycodes.py
  ```
  
- **Human Tasks**:
  1. Cross-reference 10 random keys with Apple documentation
  2. Test 5 common shortcuts in Script Editor
  3. Verify special keys (Space, Tab, Return) work correctly

- **Acceptance Criteria**:
  - [ ] All letter keys (A-Z) verified
  - [ ] All number keys (0-9) verified  
  - [ ] All modifier keys tested in combinations
  - [ ] Function keys F1-F12 confirmed

#### Checkpoint 1.2: Category Organization
- **What to Verify**:
  - Keys are in appropriate categories
  - No keys missing from categories
  - Category names are intuitive

- **Verification Tool**:
  ```json
  {
    "verification_report": {
      "total_keys": 100,
      "categories": {
        "letters": {"count": 26, "verified": false},
        "numbers": {"count": 10, "verified": false},
        "modifiers": {"count": 9, "verified": false}
      }
    }
  }
  ```

### Stage 2: API Design Review ✓
**Goal**: Ensure API is intuitive and complete

#### Checkpoint 2.1: Tool Functionality
- **What to Verify**:
  - Tool names are clear and descriptive
  - Parameters are necessary and sufficient
  - Return formats are consistent

- **Interactive Test Interface**:
  ```rust
  // Test harness for manual API testing
  #[cfg(test)]
  mod manual_tests {
      #[test]
      #[ignore] // Run with: cargo test -- --ignored
      fn interactive_api_test() {
          println!("Testing lookup_keycode('Space')...");
          // Present results for human verification
      }
  }
  ```

#### Checkpoint 2.2: Error Handling
- **Scenarios to Test**:
  1. Invalid key name: "XYZ123"
  2. Empty input: ""
  3. Special characters: "@#$%"
  4. Very long input: 1000+ chars
  5. Unicode input: "⌘⌥⇧"

- **Verification Checklist**:
  - [ ] Error messages are helpful
  - [ ] Suggestions provided for typos
  - [ ] No crashes or panics
  - [ ] Graceful degradation

### Stage 3: Implementation Verification ✓
**Goal**: Ensure code matches design specifications

#### Checkpoint 3.1: Core Functionality
- **Test Suite**:
  ```bash
  # Run interactive verification suite
  cargo run --example interactive_verify
  ```

- **Manual Test Cases**:
  ```yaml
  test_cases:
    - name: "Basic Lookup"
      input: "A"
      expected: { code: 0, category: "letters" }
      human_verify: true
      
    - name: "Alias Resolution"
      input: "Cmd"
      expected: { code: 55, name: "Command" }
      human_verify: true
      
    - name: "Fuzzy Search"
      input: "Comand" # Typo
      expected_suggestions: ["Command"]
      human_verify: true
  ```

#### Checkpoint 3.2: Shortcut Parsing
- **Test Combinations**:
  1. Simple: "Cmd+A"
  2. Complex: "Cmd+Shift+Option+F5"
  3. Alternative syntax: "⌘⇧A"
  4. Invalid: "Cmd+Cmd+A"

- **Verification Interface**:
  ```rust
  // Visual shortcut tester
  fn test_shortcut_interactive() {
      println!("Enter shortcut: ");
      let input = read_line();
      let result = parse_shortcut(&input);
      
      println!("Parsed as:");
      println!("  Modifiers: {:?}", result.modifiers);
      println!("  Main key: {:?}", result.key);
      println!("  Key codes: {:?}", result.to_keycodes());
      
      println!("\nIs this correct? (y/n)");
  }
  ```

### Stage 4: Integration Testing ✓
**Goal**: Verify MCP server works with real clients

#### Checkpoint 4.1: Client Compatibility
- **Test Clients**:
  1. Claude Desktop
  2. Custom test client
  3. Command-line MCP client

- **Test Protocol**:
  ```bash
  # 1. Start server
  cargo run --release
  
  # 2. In another terminal, run test client
  node test-client.js
  
  # 3. Verify responses match expected format
  ```

#### Checkpoint 4.2: Performance Testing
- **Metrics to Verify**:
  - Response time < 50ms for lookups
  - Memory usage < 50MB
  - No memory leaks over 1000 requests

- **Human Verification**:
  - [ ] Responses feel instant
  - [ ] No noticeable lag
  - [ ] Smooth operation

### Stage 5: User Acceptance Testing ✓
**Goal**: Ensure real users find it helpful

#### Checkpoint 5.1: Usability Testing
- **Test Scenarios**:
  1. "Find the key code for space bar"
  2. "What's the shortcut for Cmd+Shift+4?"
  3. "Show me all function keys"
  4. "I meant Commnd" (typo handling)

- **Feedback Form**:
  ```markdown
  ## MCP Server Usability Test
  
  1. Was the tool easy to understand? (1-5)
  2. Did it find the keys you needed? (Y/N)
  3. Were error messages helpful? (1-5)
  4. What confused you?
  5. What would you improve?
  ```

#### Checkpoint 5.2: Documentation Review
- **Items to Verify**:
  - [ ] README is clear and complete
  - [ ] Examples work as shown
  - [ ] Installation steps are accurate
  - [ ] API documentation matches implementation

## Verification Tools

### 1. Interactive Test Runner
```rust
// src/bin/verify.rs
use mac_keyboard_mcp::verification::*;

fn main() {
    println!("Mac Keyboard MCP - Verification Tool");
    println!("=====================================\n");
    
    loop {
        println!("Select test:");
        println!("1. Verify key codes");
        println!("2. Test API endpoints");
        println!("3. Check error handling");
        println!("4. Performance test");
        println!("5. Exit");
        
        match read_choice() {
            1 => verify_keycodes_interactive(),
            2 => test_api_interactive(),
            3 => test_errors_interactive(),
            4 => performance_test_interactive(),
            5 => break,
            _ => println!("Invalid choice"),
        }
    }
}
```

### 2. Automated Verification Assistant
```python
# verify_assistant.py
import json
import subprocess
import time

class VerificationAssistant:
    def __init__(self):
        self.results = []
    
    def test_keycode(self, key_name, expected_code):
        """Test a single key code with human confirmation"""
        print(f"\nTesting: {key_name}")
        print(f"Expected code: {expected_code}")
        
        # Show AppleScript to test
        script = f'tell application "System Events" to key code {expected_code}'
        print(f"AppleScript: {script}")
        
        # Ask for confirmation
        result = input("Did this produce the correct key? (y/n/skip): ")
        
        self.results.append({
            "key": key_name,
            "code": expected_code,
            "verified": result == 'y',
            "skipped": result == 'skip'
        })
        
        return result == 'y'
    
    def generate_report(self):
        """Generate verification report"""
        verified = sum(1 for r in self.results if r['verified'])
        skipped = sum(1 for r in self.results if r['skipped'])
        failed = len(self.results) - verified - skipped
        
        report = {
            "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"),
            "total_tested": len(self.results),
            "verified": verified,
            "failed": failed,
            "skipped": skipped,
            "details": self.results
        }
        
        with open('verification_report.json', 'w') as f:
            json.dump(report, f, indent=2)
        
        print(f"\nVerification Summary:")
        print(f"  Verified: {verified}")
        print(f"  Failed: {failed}")
        print(f"  Skipped: {skipped}")
```

### 3. Visual Verification Dashboard
```html
<!-- verification_dashboard.html -->
<!DOCTYPE html>
<html>
<head>
    <title>Mac Keyboard MCP - Verification Dashboard</title>
    <style>
        .key { 
            display: inline-block; 
            padding: 10px; 
            margin: 5px; 
            border: 1px solid #ccc;
            cursor: pointer;
        }
        .verified { background-color: #90EE90; }
        .failed { background-color: #FFB6C1; }
        .pending { background-color: #FFFACD; }
    </style>
</head>
<body>
    <h1>Key Code Verification Dashboard</h1>
    <div id="categories"></div>
    <div id="status"></div>
    
    <script>
        // Load key data and create interactive verification UI
        async function loadKeys() {
            const response = await fetch('/api/keys');
            const data = await response.json();
            
            // Create clickable keys for verification
            Object.entries(data.categories).forEach(([category, keys]) => {
                const section = document.createElement('div');
                section.innerHTML = `<h2>${category}</h2>`;
                
                keys.forEach(key => {
                    const keyElement = document.createElement('span');
                    keyElement.className = 'key pending';
                    keyElement.textContent = `${key.name} (${key.code})`;
                    keyElement.onclick = () => verifyKey(key);
                    section.appendChild(keyElement);
                });
                
                document.getElementById('categories').appendChild(section);
            });
        }
        
        async function verifyKey(key) {
            const result = await testKeyCode(key.code);
            updateKeyStatus(key, result);
        }
    </script>
</body>
</html>
```

## Verification Checkpoints Summary

### Pre-Development
- [ ] Key code data sources verified
- [ ] API design reviewed by potential users
- [ ] Test plan approved

### During Development
- [ ] Stage 1: Data validation complete
- [ ] Stage 2: API design verified
- [ ] Stage 3: Implementation checked
- [ ] Stage 4: Integration tested
- [ ] Stage 5: User acceptance confirmed

### Post-Development
- [ ] All verification reports reviewed
- [ ] Known issues documented
- [ ] Future improvements identified

## Fallback Procedures

### If Verification Fails
1. **Data Issues**: 
   - Recheck against multiple sources
   - Test on actual macOS system
   - Consult Apple documentation

2. **API Issues**:
   - Gather user feedback
   - Iterate on design
   - A/B test alternatives

3. **Implementation Issues**:
   - Add more test cases
   - Refactor problematic code
   - Increase logging detail

4. **Integration Issues**:
   - Test with different MCP clients
   - Check protocol compliance
   - Debug with packet capture

## Continuous Verification

### Automated Monitoring
```yaml
# .github/workflows/verification.yml
name: Continuous Verification

on:
  push:
    branches: [main]
  schedule:
    - cron: '0 0 * * 0' # Weekly

jobs:
  verify:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Run verification tests
        run: |
          cargo test --features verification
          cargo run --bin verify -- --automated
      
      - name: Upload verification report
        uses: actions/upload-artifact@v3
        with:
          name: verification-report
          path: verification_report.json
```

### User Feedback Loop
1. **In-App Feedback**:
   ```rust
   // Add to MCP responses
   "feedback_url": "https://github.com/user/mac-keyboard-mcp/issues/new?template=verification.md"
   ```

2. **Analytics** (opt-in):
   - Track most requested keys
   - Monitor error rates
   - Identify missing features

3. **Regular Reviews**:
   - Monthly verification report analysis
   - Quarterly user survey
   - Annual comprehensive audit

## Success Metrics

### Quantitative
- 95%+ key codes verified correct
- <50ms average response time
- 0 critical bugs in production
- 90%+ user satisfaction rate

### Qualitative
- Users find it intuitive
- Error messages are helpful
- Documentation is clear
- Integration is smooth