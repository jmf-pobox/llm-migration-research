#!/bin/bash

# I/O Contract Validation Script for rpn2tex Rust implementation
# Tests all 21 test cases from IO_CONTRACT.md

BINARY="/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/rpn2tex-rs/target/release/rpn2tex"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASSED=0
FAILED=0

# Test function
test_case() {
    local num=$1
    local input=$2
    local expected=$3
    local is_error=$4

    echo -n "Test $num: '$input' ... "

    if [ "$is_error" = "true" ]; then
        # For error cases, just check that it produces an error
        result=$(echo "$input" | $BINARY 2>&1)
        if echo "$result" | grep -q "Error"; then
            echo -e "${GREEN}PASS${NC} (error case)"
            ((PASSED++))
        else
            echo -e "${RED}FAIL${NC}"
            echo "  Expected: Error"
            echo "  Got: $result"
            ((FAILED++))
        fi
    else
        result=$(echo "$input" | $BINARY 2>&1)
        if [ "$result" = "$expected" ]; then
            echo -e "${GREEN}PASS${NC}"
            ((PASSED++))
        else
            echo -e "${RED}FAIL${NC}"
            echo "  Expected: $expected"
            echo "  Got: $result"
            ((FAILED++))
        fi
    fi
}

echo "===== rpn2tex I/O Contract Validation ====="
echo ""

# Category 1: Basic Binary Operations
echo "=== Category 1: Basic Binary Operations ==="
test_case 1 "5 3 +" "\$5 + 3\$" false
test_case 2 "5 3 -" "\$5 - 3\$" false
test_case 3 "4 7 *" "\$4 \\times 7\$" false
test_case 4 "10 2 /" "\$10 \\div 2\$" false
echo ""

# Category 2: Exponentiation (Errors)
echo "=== Category 2: Exponentiation (Error Cases) ==="
test_case 5 "2 3 ^" "LexerError" true
test_case 16 "2 3 ^ 4 *" "LexerError" true
test_case 17 "2 3 4 ^ ^" "LexerError" true
echo ""

# Category 3: Operator Precedence
echo "=== Category 3: Operator Precedence ==="
test_case 6 "5 3 + 2 *" "\$( 5 + 3 ) \\times 2\$" false
test_case 7 "5 3 * 2 +" "\$5 \\times 3 + 2\$" false
test_case 8 "10 2 / 5 *" "\$10 \\div 2 \\times 5\$" false
echo ""

# Category 4: Subtraction and Division Associativity
echo "=== Category 4: Subtraction and Division Associativity ==="
test_case 9 "5 3 - 2 -" "\$5 - 3 - 2\$" false
test_case 10 "100 10 / 5 / 2 /" "\$100 \\div 10 \\div 5 \\div 2\$" false
echo ""

# Category 5: Addition Chains
echo "=== Category 5: Addition Chains ==="
test_case 11 "1 2 + 3 + 4 +" "\$1 + 2 + 3 + 4\$" false
echo ""

# Category 6: Mixed Operations with Precedence
echo "=== Category 6: Mixed Operations with Precedence ==="
test_case 12 "2 3 4 * +" "\$2 + 3 \\times 4\$" false
test_case 13 "2 3 + 4 *" "\$( 2 + 3 ) \\times 4\$" false
test_case 14 "2 3 4 + *" "\$2 \\times ( 3 + 4 )\$" false
test_case 15 "2 3 * 4 +" "\$2 \\times 3 + 4\$" false
echo ""

# Category 7: Floating-Point Numbers
echo "=== Category 7: Floating-Point Numbers ==="
test_case 18 "3.14 2 *" "\$3.14 \\times 2\$" false
test_case 19 "1.5 0.5 +" "\$1.5 + 0.5\$" false
echo ""

# Category 8: Complex Expressions
echo "=== Category 8: Complex Expressions ==="
test_case 20 "1 2 + 3 4 + *" "\$( 1 + 2 ) \\times ( 3 + 4 )\$" false
test_case 21 "10 2 / 3 + 4 *" "\$( 10 \\div 2 + 3 ) \\times 4\$" false
echo ""

# Summary
echo "======================================"
echo "TOTAL: $((PASSED + FAILED)) tests"
echo -e "${GREEN}PASSED: $PASSED${NC}"
if [ $FAILED -gt 0 ]; then
    echo -e "${RED}FAILED: $FAILED${NC}"
    exit 1
else
    echo -e "${YELLOW}All tests passed!${NC}"
    exit 0
fi
