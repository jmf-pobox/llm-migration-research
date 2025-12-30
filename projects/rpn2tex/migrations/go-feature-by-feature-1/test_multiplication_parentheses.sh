#!/bin/bash

test_case() {
    local name="$1"
    local input="$2"
    local expected="$3"
    
    echo -n "Test: $name"
    result=$(echo "$input" | ./rpn2tex_test -)
    
    if [ "$result" = "$expected" ]; then
        echo " ... PASS"
        return 0
    else
        echo " ... FAIL"
        echo "  Input:    $input"
        echo "  Expected: $expected"
        echo "  Got:      $result"
        return 1
    fi
}

echo "Testing Multiplication with Parenthesization"
echo "============================================="
echo ""

passes=0
fails=0

# From I/O contract - parenthesization tests
echo "Lower precedence (addition) child of higher precedence (multiplication):"
test_case "5 3 + 2 *" "5 3 + 2 *" '$( 5 + 3 ) \times 2$' && ((passes++)) || ((fails++))
test_case "2 3 + 4 *" "2 3 + 4 *" '$( 2 + 3 ) \times 4$' && ((passes++)) || ((fails++))
test_case "2 3 4 + *" "2 3 4 + *" '$2 \times ( 3 + 4 )$' && ((passes++)) || ((fails++))
test_case "1 2 + 3 4 + *" "1 2 + 3 4 + *" '$( 1 + 2 ) \times ( 3 + 4 )$' && ((passes++)) || ((fails++))

echo ""
echo "Same or higher precedence - no extra parentheses:"
test_case "5 3 * 2 +" "5 3 * 2 +" '$5 \times 3 + 2$' && ((passes++)) || ((fails++))
test_case "2 3 * 4 +" "2 3 * 4 +" '$2 \times 3 + 4$' && ((passes++)) || ((fails++))
test_case "2 3 4 * +" "2 3 4 * +" '$2 + 3 \times 4$' && ((passes++)) || ((fails++))

echo ""
echo "Results: $passes passed, $fails failed"
exit $fails
