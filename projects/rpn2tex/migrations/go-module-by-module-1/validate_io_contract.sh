#!/bin/bash

# I/O Contract Validation Script
# Tests all 21 test cases from the I/O contract

BINARY="./rpn2tex_final"
PASSED=0
FAILED=0

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

test_success() {
    local expected="$1"
    shift
    local input=("$@")
    
    output=$("$BINARY" "${input[@]}" 2>&1)
    exit_code=$?
    
    if [ $exit_code -eq 0 ] && [ "$output" = "$expected" ]; then
        echo -e "${GREEN}✓${NC} PASS: ${input[*]}"
        ((PASSED++))
    else
        echo -e "${RED}✗${NC} FAIL: ${input[*]}"
        echo "  Expected: $expected"
        echo "  Got:      $output"
        echo "  Exit:     $exit_code"
        ((FAILED++))
    fi
}

test_error() {
    local error_contains="$1"
    shift
    local input=("$@")
    
    output=$("$BINARY" "${input[@]}" 2>&1)
    exit_code=$?
    
    if [ $exit_code -eq 1 ] && echo "$output" | grep -q "$error_contains"; then
        echo -e "${GREEN}✓${NC} PASS: ${input[*]} (error case)"
        ((PASSED++))
    else
        echo -e "${RED}✗${NC} FAIL: ${input[*]} (error case)"
        echo "  Should contain: $error_contains"
        echo "  Got:            $output"
        echo "  Exit:           $exit_code"
        ((FAILED++))
    fi
}

echo "Starting I/O Contract Validation..."
echo "===================================="
echo ""

# Successful cases
test_success '$5 + 3$' 5 3 +
test_success '$5 - 3$' 5 3 -
test_success '$4 \times 7$' 4 7 '*'
test_success '$10 \div 2$' 10 2 /
test_success '$( 5 + 3 ) \times 2$' 5 3 + 2 '*'
test_success '$5 \times 3 + 2$' 5 3 '*' 2 +
test_success '$10 \div 2 \times 5$' 10 2 / 5 '*'
test_success '$5 - 3 - 2$' 5 3 - 2 -
test_success '$100 \div 10 \div 5 \div 2$' 100 10 / 5 / 2 /
test_success '$1 + 2 + 3 + 4$' 1 2 + 3 + 4 +
test_success '$2 + 3 \times 4$' 2 3 4 '*' +
test_success '$( 2 + 3 ) \times 4$' 2 3 + 4 '*'
test_success '$2 \times ( 3 + 4 )$' 2 3 4 + '*'
test_success '$2 \times 3 + 4$' 2 3 '*' 4 +
test_success '$3.14 \times 2$' 3.14 2 '*'
test_success '$1.5 + 0.5$' 1.5 0.5 +
test_success '$( 1 + 2 ) \times ( 3 + 4 )$' 1 2 + 3 4 + '*'
test_success '$( 10 \div 2 + 3 ) \times 4$' 10 2 / 3 + 4 '*'

# Error cases
test_error "Unexpected character '^'" 2 3 '^'
test_error "Unexpected character '^'" 2 3 '^' 4 '*'
test_error "Unexpected character '^'" 2 3 4 '^' '^'

echo ""
echo "===================================="
echo "Results: $PASSED passed, $FAILED failed"
echo "===================================="

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed.${NC}"
    exit 1
fi
