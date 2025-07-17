#!/bin/bash

# Continuous verification script for Mac Keyboard MCP
# Runs automated checks and generates reports

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$PROJECT_ROOT/verification_reports"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_FILE="$REPORT_DIR/verify_$TIMESTAMP.json"

# Create report directory if it doesn't exist
mkdir -p "$REPORT_DIR"

echo -e "${BLUE}Mac Keyboard MCP - Continuous Verification${NC}"
echo "=========================================="
echo "Timestamp: $(date)"
echo "Project: $PROJECT_ROOT"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
echo -e "${YELLOW}Checking prerequisites...${NC}"

if ! command_exists cargo; then
    echo -e "${RED}❌ Rust/Cargo not found${NC}"
    exit 1
else
    echo -e "${GREEN}✅ Rust/Cargo found${NC}"
fi

if ! command_exists python3; then
    echo -e "${RED}❌ Python 3 not found${NC}"
    exit 1
else
    echo -e "${GREEN}✅ Python 3 found${NC}"
fi

if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${RED}❌ This script requires macOS${NC}"
    exit 1
else
    echo -e "${GREEN}✅ Running on macOS${NC}"
fi

# Stage 1: Build verification
echo -e "\n${YELLOW}Stage 1: Build Verification${NC}"
echo "------------------------"

cd "$PROJECT_ROOT"

if cargo check 2>/dev/null; then
    echo -e "${GREEN}✅ Cargo check passed${NC}"
    BUILD_CHECK="PASS"
else
    echo -e "${RED}❌ Cargo check failed${NC}"
    BUILD_CHECK="FAIL"
fi

if cargo test --no-run 2>/dev/null; then
    echo -e "${GREEN}✅ Test compilation passed${NC}"
    TEST_COMPILE="PASS"
else
    echo -e "${RED}❌ Test compilation failed${NC}"
    TEST_COMPILE="FAIL"
fi

# Stage 2: Data verification
echo -e "\n${YELLOW}Stage 2: Data Verification${NC}"
echo "------------------------"

cd "$PROJECT_ROOT/scripts"

if python3 verify_keycodes.py --mode quick > /tmp/verify_quick.log 2>&1; then
    echo -e "${GREEN}✅ Quick key verification passed${NC}"
    DATA_VERIFY="PASS"
    cat /tmp/verify_quick.log | grep "✅\|❌" || true
else
    echo -e "${RED}❌ Quick key verification failed${NC}"
    DATA_VERIFY="FAIL"
fi

# Stage 3: Unit tests
echo -e "\n${YELLOW}Stage 3: Unit Tests${NC}"
echo "------------------------"

cd "$PROJECT_ROOT"

if cargo test --lib 2>/dev/null; then
    echo -e "${GREEN}✅ Unit tests passed${NC}"
    UNIT_TESTS="PASS"
else
    echo -e "${RED}❌ Unit tests failed${NC}"
    UNIT_TESTS="FAIL"
fi

# Stage 4: Integration tests
echo -e "\n${YELLOW}Stage 4: Integration Tests${NC}"
echo "------------------------"

if cargo test --test '*' 2>/dev/null; then
    echo -e "${GREEN}✅ Integration tests passed${NC}"
    INTEGRATION_TESTS="PASS"
else
    echo -e "${RED}❌ Integration tests failed${NC}"
    INTEGRATION_TESTS="FAIL"
fi

# Stage 5: Performance check
echo -e "\n${YELLOW}Stage 5: Performance Check${NC}"
echo "------------------------"

# Simple benchmark
BENCH_START=$(date +%s%N)
if cargo run --release --bin verify -- --mode quick >/dev/null 2>&1; then
    BENCH_END=$(date +%s%N)
    BENCH_TIME=$(( ($BENCH_END - $BENCH_START) / 1000000 ))
    echo -e "${GREEN}✅ Performance check completed in ${BENCH_TIME}ms${NC}"
    
    if [ $BENCH_TIME -lt 5000 ]; then
        PERFORMANCE="PASS"
        echo -e "${GREEN}✅ Performance within target (<5s)${NC}"
    else
        PERFORMANCE="SLOW"
        echo -e "${YELLOW}⚠️  Performance slower than target (>5s)${NC}"
    fi
else
    echo -e "${RED}❌ Performance check failed${NC}"
    PERFORMANCE="FAIL"
fi

# Generate JSON report
echo -e "\n${YELLOW}Generating report...${NC}"

cat > "$REPORT_FILE" <<EOF
{
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "platform": "$(uname -s)",
  "platform_version": "$(sw_vers -productVersion 2>/dev/null || echo 'unknown')",
  "stages": {
    "build": {
      "cargo_check": "$BUILD_CHECK",
      "test_compile": "$TEST_COMPILE"
    },
    "data": {
      "quick_verify": "$DATA_VERIFY"
    },
    "tests": {
      "unit": "$UNIT_TESTS",
      "integration": "$INTEGRATION_TESTS"
    },
    "performance": {
      "status": "$PERFORMANCE",
      "time_ms": ${BENCH_TIME:-0}
    }
  },
  "summary": {
    "all_passed": $([ "$BUILD_CHECK" = "PASS" ] && [ "$TEST_COMPILE" = "PASS" ] && [ "$DATA_VERIFY" = "PASS" ] && [ "$UNIT_TESTS" = "PASS" ] && echo "true" || echo "false")
  }
}
EOF

echo -e "${GREEN}✅ Report saved to: $REPORT_FILE${NC}"

# Summary
echo -e "\n${BLUE}Verification Summary${NC}"
echo "==================="

if [ "$BUILD_CHECK" = "PASS" ] && [ "$DATA_VERIFY" = "PASS" ] && [ "$UNIT_TESTS" = "PASS" ]; then
    echo -e "${GREEN}✅ All critical checks passed!${NC}"
    EXIT_CODE=0
else
    echo -e "${RED}❌ Some checks failed${NC}"
    EXIT_CODE=1
fi

echo ""
echo "Build:       $BUILD_CHECK"
echo "Data:        $DATA_VERIFY"
echo "Unit Tests:  $UNIT_TESTS"
echo "Integration: $INTEGRATION_TESTS"
echo "Performance: $PERFORMANCE"

# Optional: Upload results if CI environment
if [ -n "$CI" ]; then
    echo -e "\n${YELLOW}CI Environment detected, would upload results...${NC}"
fi

exit $EXIT_CODE