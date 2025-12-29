#!/bin/bash

# Test script for all 20 verified I/O contract cases
BINARY="./target/release/rpn2tex"
PASS=0
FAIL=0

test_case() {
    local input="$1"
    local expected="$2"
    local description="$3"
    
    local output
    output=$($BINARY "$input" 2>&1)
    
    if [ "$output" = "$expected" ]; then
        echo "✓ PASS: $description"
        ((PASS++))
    else
        echo "✗ FAIL: $description"
        echo "  Input:    $input"
        echo "  Expected: $expected"
        echo "  Got:      $output"
        ((FAIL++))
    fi
}

echo "Testing all 20 I/O contract cases..."
echo

# Numbers
test_case "5" '$5$' "Integer literal"
test_case "3.14" '$3.14$' "Decimal literal"

# Addition
test_case "5 3 +" '$5 + 3$' "Simple addition"
test_case "1 2 + 3 + 4 +" '$1 + 2 + 3 + 4$' "Chained addition"

# Subtraction
test_case "5 3 -" '$5 - 3$' "Simple subtraction"
test_case "5 3 - 2 -" '$5 - 3 - 2$' "Chained subtraction"

# Multiplication
test_case "4 7 *" '$4 \times 7$' "Simple multiplication"
test_case "2 3 4 * +" '$2 + 3 \times 4$' "Precedence handling (mult before add)"
test_case "3.14 2 *" '$3.14 \times 2$' "Decimal multiplication"
test_case "5 3 * 2 +" '$5 \times 3 + 2$' "Precedence handling (mult before add)"

# Division
test_case "10 2 /" '$10 \div 2$' "Simple division"
test_case "100 10 / 5 / 2 /" '$100 \div 10 \div 5 \div 2$' "Chained division"
test_case "10 2 / 5 *" '$10 \div 2 \times 5$' "Mixed precedence (same level)"

# Decimal numbers
test_case "1.5 0.5 +" '$1.5 + 0.5$' "Decimal addition"

# Operator Precedence (Parenthesization)
test_case "5 3 + 2 *" '$( 5 + 3 ) \times 2$' "Addition evaluated first (left)"
test_case "2 3 + 4 *" '$( 2 + 3 ) \times 4$' "Addition evaluated first (left)"
test_case "2 3 4 + *" '$2 \times ( 3 + 4 )$' "Addition evaluated first (right)"
test_case "1 2 + 3 4 + *" '$( 1 + 2 ) \times ( 3 + 4 )$' "Both additions evaluated first"
test_case "10 2 / 3 + 4 *" '$( 10 \div 2 + 3 ) \times 4$' "Complex precedence"

echo
echo "========================================="
echo "Results: $PASS passed, $FAIL failed"
echo "========================================="

if [ $FAIL -eq 0 ]; then
    echo "All tests passed!"
    exit 0
else
    echo "Some tests failed."
    exit 1
fi
