#!/bin/bash

# I/O Contract Validation Script
# Tests all 21 cases from the I/O contract

set -e

BINARY="./rpn2tex-rs/target/release/rpn2tex"
TOTAL=0
PASSED=0
FAILED=0

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "=== rpn2tex I/O Contract Validation ==="
echo

# Test helper function
test_case() {
    local num=$1
    local input=$2
    local expected=$3
    local description=$4

    TOTAL=$((TOTAL + 1))

    local output=$(echo "$input" | $BINARY - 2>&1)

    if [ "$output" = "$expected" ]; then
        echo -e "${GREEN}✓${NC} Test $num: $description"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗${NC} Test $num: $description"
        echo "  Input:    $input"
        echo "  Expected: $expected"
        echo "  Got:      $output"
        FAILED=$((FAILED + 1))
    fi
}

# Category 1: Basic Binary Operations
test_case 1 "5 3 +" "\$5 + 3\$" "Basic addition"
test_case 2 "5 3 -" "\$5 - 3\$" "Basic subtraction"
test_case 3 "4 7 *" "\$4 \\times 7\$" "Basic multiplication"
test_case 4 "10 2 /" "\$10 \\div 2\$" "Basic division"

# Category 2: Exponentiation (Error Cases)
test_case 5 "2 3 ^" "Error: Unexpected character '^'

1 | 2 3 ^
        ^" "Unsupported operator ^"

test_case 16 "2 3 ^ 4 *" "Error: Unexpected character '^'

1 | 2 3 ^ 4 *
        ^" "Unsupported operator ^ (with more tokens)"

test_case 17 "2 3 4 ^ ^" "Error: Unexpected character '^'

1 | 2 3 4 ^ ^
          ^" "Unsupported operator ^ (different position)"

# Category 3: Operator Precedence
test_case 6 "5 3 + 2 *" "\$( 5 + 3 ) \\times 2\$" "Addition before multiplication"
test_case 7 "5 3 * 2 +" "\$5 \\times 3 + 2\$" "Multiplication before addition"
test_case 8 "10 2 / 5 *" "\$10 \\div 2 \\times 5\$" "Equal precedence left-to-right"

# Category 4: Associativity
test_case 9 "5 3 - 2 -" "\$5 - 3 - 2\$" "Left-associative subtraction"
test_case 10 "100 10 / 5 / 2 /" "\$100 \\div 10 \\div 5 \\div 2\$" "Left-associative division chain"

# Category 5: Addition Chains
test_case 11 "1 2 + 3 + 4 +" "\$1 + 2 + 3 + 4\$" "Addition chain (commutative)"

# Category 6: Mixed Operations
test_case 12 "2 3 4 * +" "\$2 + 3 \\times 4\$" "Mixed: mult then add"
test_case 13 "2 3 + 4 *" "\$( 2 + 3 ) \\times 4\$" "Add on left of mult"
test_case 14 "2 3 4 + *" "\$2 \\times ( 3 + 4 )\$" "Add on right of mult"
test_case 15 "2 3 * 4 +" "\$2 \\times 3 + 4\$" "Mult before add"

# Category 7: Floating-Point
test_case 18 "3.14 2 *" "\$3.14 \\times 2\$" "Decimal numbers"
test_case 19 "1.5 0.5 +" "\$1.5 + 0.5\$" "Multiple decimals"

# Category 8: Complex Expressions
test_case 20 "1 2 + 3 4 + *" "\$( 1 + 2 ) \\times ( 3 + 4 )\$" "Two additions multiplied"
test_case 21 "10 2 / 3 + 4 *" "\$( 10 \\div 2 + 3 ) \\times 4\$" "Complex mixed expression"

echo
echo "=== Results ==="
echo "Total:  $TOTAL"
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"

if [ $FAILED -eq 0 ]; then
    echo
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo
    echo -e "${RED}Some tests failed.${NC}"
    exit 1
fi
