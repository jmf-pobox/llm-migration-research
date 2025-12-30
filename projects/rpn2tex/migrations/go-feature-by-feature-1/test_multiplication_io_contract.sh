#!/bin/bash

# Test multiplication I/O contract
echo "Testing Feature 4: Multiplication I/O Contract"
echo "=============================================="
echo ""

test_case() {
    local name="$1"
    local input="$2"
    local expected="$3"
    
    echo -n "Test: $name ... "
    result=$(echo "$input" | ./rpn2tex_test -)
    
    if [ "$result" = "$expected" ]; then
        echo "PASS"
        return 0
    else
        echo "FAIL"
        echo "  Input:    $input"
        echo "  Expected: $expected"
        echo "  Got:      $result"
        return 1
    fi
}

# Test cases from the I/O contract
passes=0
fails=0

# Simple multiplication
test_case "4 7 *" "4 7 *" '$4 \times 7$' && ((passes++)) || ((fails++))

# Multiplication has higher precedence than addition
test_case "2 3 4 * +" "2 3 4 * +" '$2 + 3 \times 4$' && ((passes++)) || ((fails++))

# Decimal multiplication
test_case "3.14 2 *" "3.14 2 *" '$3.14 \times 2$' && ((passes++)) || ((fails++))

# Chained multiplication
test_case "2 3 * 4 *" "2 3 * 4 *" '$2 \times 3 \times 4$' && ((passes++)) || ((fails++))

# Multiplication with addition (right side)
test_case "5 3 * 2 +" "5 3 * 2 +" '$5 \times 3 + 2$' && ((passes++)) || ((fails++))

echo ""
echo "Results: $passes passed, $fails failed"
exit $fails
