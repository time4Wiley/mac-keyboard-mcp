# Verification Checklist

## 🔍 Stage-by-Stage Verification Guide

### 📊 Stage 1: Data Validation
**When**: Before any code implementation  
**Time Required**: 2-3 hours

#### Quick Verification Steps:
1. **Spot Check Keys** (15 min)
   ```bash
   # Pick 5 random keys and verify in Script Editor
   osascript -e 'tell application "System Events" to key code 0'  # Should type 'A'
   osascript -e 'tell application "System Events" to key code 49' # Should type Space
   ```

2. **Category Count** (10 min)
   - [ ] Letters: exactly 26 keys
   - [ ] Numbers: exactly 10 keys (0-9)
   - [ ] Modifiers: at least 8 keys
   - [ ] Function keys: at least F1-F12

3. **Common Shortcuts** (20 min)
   Test these combinations work:
   - [ ] Cmd+A (Select All)
   - [ ] Cmd+C (Copy)
   - [ ] Cmd+Tab (App Switcher)
   - [ ] Cmd+Space (Spotlight)

#### ✅ Pass Criteria:
- All spot checks produce expected output
- Category counts are correct
- No duplicate key codes found

---

### 🎨 Stage 2: API Design Review
**When**: After initial API design, before implementation  
**Time Required**: 1-2 hours

#### Quick Verification Steps:
1. **Clarity Test** (20 min)
   Show API to someone unfamiliar with the project:
   - [ ] Can they understand what each tool does?
   - [ ] Are parameter names self-explanatory?
   - [ ] Is the response format clear?

2. **Completeness Check** (15 min)
   - [ ] Can find any key by name?
   - [ ] Can search with typos?
   - [ ] Can parse shortcuts?
   - [ ] Can list all keys?

3. **Edge Cases** (15 min)
   What happens when:
   - [ ] Key doesn't exist?
   - [ ] Input is empty?
   - [ ] Input has special characters?

#### ✅ Pass Criteria:
- New user understands API in <5 minutes
- All common use cases covered
- Edge cases have sensible responses

---

### 🔨 Stage 3: Implementation Testing
**When**: After each module is implemented  
**Time Required**: 30 min per module

#### Module Verification:

**3.1 Key Lookup Module** (30 min)
```rust
// Test these manually:
lookup("A")        // → Some(KeyCode { code: 0, ... })
lookup("space")    // → Some(KeyCode { code: 49, ... })
lookup("cmd")      // → Some(KeyCode { code: 55, ... })
lookup("xyz123")   // → None
```

**3.2 Fuzzy Search Module** (30 min)
```rust
// Test typo tolerance:
search("comand")   // → ["Command"]
search("spce")     // → ["Space"]
search("F1")       // → ["F1", "F10", "F11", ...]
```

**3.3 Shortcut Parser** (30 min)
```rust
// Test combinations:
parse("Cmd+A")           // → {modifiers: [Cmd], key: A}
parse("Cmd+Shift+F5")    // → {modifiers: [Cmd, Shift], key: F5}
parse("⌘⇧A")            // → {modifiers: [Cmd, Shift], key: A}
```

#### ✅ Pass Criteria:
- Each module works independently
- Error messages are helpful
- Performance feels instant (<50ms)

---

### 🔌 Stage 4: Integration Testing
**When**: After MCP server is complete  
**Time Required**: 2-3 hours

#### Test Setup:
1. **Start Server** (5 min)
   ```bash
   cargo build --release
   ./target/release/mac-keyboard-mcp
   ```

2. **Test with Client** (45 min)
   ```javascript
   // test-client.js
   const client = new MCPClient();
   
   // Test each tool:
   await client.call("lookup_keycode", {key_name: "Space"});
   await client.call("search_keys", {query: "arrow"});
   await client.call("get_key_combinations", {shortcut: "Cmd+A"});
   ```

3. **Stress Test** (30 min)
   - [ ] 100 rapid requests
   - [ ] Memory stays under 50MB
   - [ ] No crashes or errors

#### ✅ Pass Criteria:
- All tools respond correctly
- No memory leaks
- Handles concurrent requests

---

### 👥 Stage 5: User Testing
**When**: Before public release  
**Time Required**: 1-2 days

#### Test with 3-5 Real Users:

**5.1 Task-Based Testing** (30 min per user)
Give users these tasks without help:
1. "Find the key code for the space bar"
2. "What keys do I press for screenshot?"
3. "Show me all arrow keys"
4. "Find function key F5"

**5.2 Feedback Questions**:
- What confused you?
- What would you change?
- Would you use this again?
- Rate ease of use (1-10)

#### ✅ Pass Criteria:
- 80%+ task completion rate
- Average ease rating ≥7/10
- No critical usability issues

---

## 🚀 Quick Verification Script

Save as `verify.sh`:
```bash
#!/bin/bash

echo "🔍 Mac Keyboard MCP - Quick Verification"
echo "========================================"

# Check if server builds
echo -n "1. Building server... "
if cargo build --release 2>/dev/null; then
    echo "✅ PASS"
else
    echo "❌ FAIL"
    exit 1
fi

# Check if tests pass
echo -n "2. Running tests... "
if cargo test 2>/dev/null; then
    echo "✅ PASS"
else
    echo "❌ FAIL"
    exit 1
fi

# Check key codes
echo -n "3. Verifying sample keys... "
if cargo run --bin verify_keys 2>/dev/null; then
    echo "✅ PASS"
else
    echo "❌ FAIL"
fi

echo ""
echo "Basic verification complete!"
echo "Run 'cargo run --example interactive' for manual testing"
```

---

## 📝 Verification Report Template

```markdown
# Verification Report - [Date]

## Summary
- **Version**: 0.1.0
- **Tester**: [Name]
- **Duration**: [X hours]
- **Overall Result**: PASS / FAIL

## Stage Results

### Stage 1: Data Validation
- **Result**: PASS
- **Issues Found**: None
- **Time Taken**: 2 hours

### Stage 2: API Design
- **Result**: PASS
- **Feedback**: Tool names very clear
- **Time Taken**: 1 hour

### Stage 3: Implementation
- **Result**: PASS
- **Performance**: <20ms average
- **Time Taken**: 2 hours

### Stage 4: Integration
- **Result**: PASS  
- **Compatibility**: Works with Claude Desktop
- **Time Taken**: 2 hours

### Stage 5: User Testing
- **Result**: PASS
- **Satisfaction**: 8.5/10 average
- **Time Taken**: 1 day

## Issues & Resolutions
1. **Issue**: Fuzzy search too broad
   **Resolution**: Adjusted threshold from 0.6 to 0.8

2. **Issue**: Missing alias for "Enter"
   **Resolution**: Added "Enter" → "Return" mapping

## Recommendations
- Add more function key aliases
- Consider adding media key support
- Improve error messages for invalid shortcuts

## Sign-off
- [ ] Developer reviewed
- [ ] Tester approved
- [ ] Ready for release
```

---

## 🎯 Critical Path Checklist

**Minimum viable verification** (can ship if these pass):

1. **Data Accuracy** (MUST PASS)
   - [ ] Letter keys A-Z correct
   - [ ] Number keys 0-9 correct
   - [ ] Space, Return, Tab work

2. **Basic Functionality** (MUST PASS)
   - [ ] Can lookup any valid key
   - [ ] Returns error for invalid keys
   - [ ] MCP protocol works

3. **No Crashes** (MUST PASS)
   - [ ] Handles bad input gracefully
   - [ ] No panics in normal use
   - [ ] Memory usage stable

**Nice to have** (can fix post-launch):
- Fuzzy search accuracy
- All F-keys verified
- Performance <10ms
- Beautiful error messages