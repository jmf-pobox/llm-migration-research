#!/bin/bash
# Validate all I/O contract test cases

BINARY="./target/release/rpn2tex"
PASSED=0
FAILED=0

test_case() {
    local input="$1"
    local expected="$2"
    local description="$3"
    
    local output=$($BINARY "$input" 2>&1)
    
    if [ "$output" = "$expected" ]; then
        echo "✓ PASS: $description"
        ((PASSED++))
    else
        echo "✗ FAIL: $description"
        echo "  Input:    $input"
        echo "  Expected: $expected"
        echo "  Got:      $output"
        ((FAILED++))
    fi
}

echo "=== Feature 1-6 I/O Contract Validation ==="
echo

# Basic Operations (6 tests)
test_case "5 3 +" '$5 + 3$' "Basic addition"
test_case "5 3 -" '$5 - 3$' "Basic subtraction"
test_case "4 7 *" '$4 \times 7$' "Basic multiplication"
test_case "10 2 /" '$10 \div 2$' "Basic division"
test_case "3.14 2 *" '$3.14 \times 2$' "Float multiplication"
test_case "1.5 0.5 +" '$1.5 + 0.5$' "Float addition"

# Chained Operations (3 tests)
test_case "1 2 + 3 + 4 +" '$1 + 2 + 3 + 4$' "Chained addition"
test_case "5 3 - 2 -" '$5 - 3 - 2$' "Chained subtraction"
test_case "100 10 / 5 / 2 /" '$100 \div 10 \div 5 \div 2$' "Chained division"

# Operator Precedence (7 tests)
test_case "5 3 + 2 *" '$( 5 + 3 ) \times 2$' "Precedence: addition under multiplication (left)"
test_case "2 3 + 4 *" '$( 2 + 3 ) \times 4$' "Precedence: addition under multiplication (left 2)"
test_case "2 3 4 + *" '$2 \times ( 3 + 4 )$' "Precedence: addition under multiplication (right)"
test_case "2 3 4 * +" '$2 + 3 \times 4$' "Precedence: multiplication over addition"
test_case "2 3 * 4 +" '$2 \times 3 + 4$' "Precedence: multiplication then addition"
test_case "5 3 * 2 +" '$5 \times 3 + 2$' "Precedence: multiplication then addition 2"
test_case "10 2 / 5 *" '$10 \div 2 \times 5$' "Precedence: division and multiplication same level"

# Complex Expressions (2 tests)
test_case "1 2 + 3 4 + *" '$( 1 + 2 ) \times ( 3 + 4 )$' "Complex: both sides parenthesized"
test_case "10 2 / 3 + 4 *" '$( 10 \div 2 + 3 ) \times 4$' "Complex: mixed operations"

echo
echo "=== Results ==="
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo "Total:  $((PASSED + FAILED))"

if [ $FAILED -eq 0 ]; then
    echo "✓ All tests passed!"
    exit 0
else
    echo "✗ Some tests failed"
    exit 1
fi
