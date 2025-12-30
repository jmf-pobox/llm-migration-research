#!/bin/bash
# I/O Contract validation script for rpn2tex
# Tests all cases from the verified I/O contract

BINARY="target/release/rpn2tex"
PASSED=0
FAILED=0

# Build first
echo "Building release binary..."
cargo build --release 2>&1 | grep -v "Finished" | grep -v "Running" || true
echo ""

# Test function
test_case() {
    local input="$1"
    local expected="$2"
    local description="$3"

    echo -n "Testing: $description... "

    # Run the binary with stdin
    local output
    output=$(echo "$input" | "$BINARY" - 2>/dev/null) || true

    if [ "$output" = "$expected" ]; then
        echo "PASS"
        PASSED=$((PASSED + 1))
    else
        echo "FAIL"
        echo "  Input:    '$input'"
        echo "  Expected: '$expected'"
        echo "  Got:      '$output'"
        FAILED=$((FAILED + 1))
    fi
}

# Test error cases
test_error_case() {
    local input="$1"
    local expected_error="$2"
    local description="$3"

    echo -n "Testing error: $description... "

    # Run the binary with stdin and capture stderr
    local stderr_output
    stderr_output=$(echo "$input" | "$BINARY" - 2>&1 >/dev/null) || true

    if echo "$stderr_output" | grep -q "$expected_error"; then
        echo "PASS"
        PASSED=$((PASSED + 1))
    else
        echo "FAIL"
        echo "  Input:    '$input'"
        echo "  Expected: '$expected_error' in stderr"
        echo "  Got:      '$stderr_output'"
        FAILED=$((FAILED + 1))
    fi
}

echo "=== Running I/O Contract Tests ==="
echo ""

# Category: Numbers
test_case "5" '$5$' "Integer literal"
test_case "3.14" '$3.14$' "Decimal literal"

# Category: Addition
test_case "5 3 +" '$5 + 3$' "Simple addition"
test_case "1 2 + 3 + 4 +" '$1 + 2 + 3 + 4$' "Chained addition"

# Category: Subtraction
test_case "5 3 -" '$5 - 3$' "Simple subtraction"
test_case "5 3 - 2 -" '$5 - 3 - 2$' "Chained subtraction"

# Category: Multiplication
test_case "4 7 *" '$4 \times 7$' "Simple multiplication"
test_case "2 3 4 * +" '$2 + 3 \times 4$' "Precedence handling"
test_case "3.14 2 *" '$3.14 \times 2$' "Decimal multiplication"
test_case "5 3 * 2 +" '$5 \times 3 + 2$' "Precedence handling"

# Category: Division
test_case "10 2 /" '$10 \div 2$' "Simple division"
test_case "100 10 / 5 / 2 /" '$100 \div 10 \div 5 \div 2$' "Chained division"
test_case "10 2 / 5 *" '$10 \div 2 \times 5$' "Mixed precedence"

# Category: Decimal Numbers
test_case "1.5 0.5 +" '$1.5 + 0.5$' "Decimal addition"

# Category: Operator Precedence (Parenthesization)
test_case "5 3 + 2 *" '$( 5 + 3 ) \times 2$' "Addition evaluated first"
test_case "2 3 + 4 *" '$( 2 + 3 ) \times 4$' "Addition evaluated first"
test_case "2 3 4 + *" '$2 \times ( 3 + 4 )$' "Addition evaluated first"
test_case "1 2 + 3 4 + *" '$( 1 + 2 ) \times ( 3 + 4 )$' "Both additions evaluated first"
test_case "10 2 / 3 + 4 *" '$( 10 \div 2 + 3 ) \times 4$' "Complex precedence"

# Category: Error cases
test_error_case "2 3 ^" "Unexpected character '^'" "Unsupported operator ^"
test_error_case "2 3 4 ^ ^" "Unexpected character '^'" "Multiple unsupported operators"
test_error_case "2 3 ^ 4 *" "Unexpected character '^'" "Unsupported operator mixed"

echo ""
echo "=== Test Summary ==="
echo "PASSED: $PASSED"
echo "FAILED: $FAILED"
echo ""

if [ $FAILED -eq 0 ]; then
    echo "✓ All tests passed!"
    exit 0
else
    echo "✗ Some tests failed"
    exit 1
fi
