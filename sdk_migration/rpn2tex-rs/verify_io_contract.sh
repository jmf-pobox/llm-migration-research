#!/bin/bash

# Verify I/O Contract Cases
BINARY="./target/release/rpn2tex"
PASS=0
FAIL=0

test_case() {
    local num=$1
    local input=$2
    local expected=$3
    local should_error=$4
    
    if [ "$should_error" = "true" ]; then
        output=$(echo "$input" | $BINARY - 2>&1)
        exit_code=$?
        if [ $exit_code -eq 0 ] && echo "$output" | grep -q "Error:"; then
            echo "✓ Test $num: PASS (error case)"
            ((PASS++))
        else
            echo "✗ Test $num: FAIL"
            echo "  Exit code: $exit_code (expected 0 with error)"
            ((FAIL++))
        fi
    else
        output=$(echo "$input" | $BINARY - 2>&1)
        exit_code=$?
        if [ $exit_code -eq 0 ] && [ "$output" = "$expected" ]; then
            echo "✓ Test $num: PASS"
            ((PASS++))
        else
            echo "✗ Test $num: FAIL"
            echo "  Expected: $expected"
            echo "  Got:      $output"
            echo "  Exit:     $exit_code"
            ((FAIL++))
        fi
    fi
}

echo "Running I/O Contract Verification..."
echo "===================================="

test_case 1 "5 3 +" '$5 + 3$' false
test_case 2 "5 3 -" '$5 - 3$' false
test_case 3 "4 7 *" '$4 \times 7$' false
test_case 4 "10 2 /" '$10 \div 2$' false
test_case 5 "2 3 ^" "" true
test_case 6 "5 3 + 2 *" '$( 5 + 3 ) \times 2$' false
test_case 7 "5 3 * 2 +" '$5 \times 3 + 2$' false
test_case 8 "10 2 / 5 *" '$10 \div 2 \times 5$' false
test_case 9 "5 3 - 2 -" '$5 - 3 - 2$' false
test_case 10 "100 10 / 5 / 2 /" '$100 \div 10 \div 5 \div 2$' false
test_case 11 "1 2 + 3 + 4 +" '$1 + 2 + 3 + 4$' false
test_case 12 "2 3 4 * +" '$2 + 3 \times 4$' false
test_case 13 "2 3 + 4 *" '$( 2 + 3 ) \times 4$' false
test_case 14 "2 3 4 + *" '$2 \times ( 3 + 4 )$' false
test_case 15 "2 3 * 4 +" '$2 \times 3 + 4$' false
test_case 16 "2 3 ^ 4 *" "" true
test_case 17 "2 3 4 ^ ^" "" true
test_case 18 "3.14 2 *" '$3.14 \times 2$' false
test_case 19 "1.5 0.5 +" '$1.5 + 0.5$' false
test_case 20 "1 2 + 3 4 + *" '$( 1 + 2 ) \times ( 3 + 4 )$' false
test_case 21 "10 2 / 3 + 4 *" '$( 10 \div 2 + 3 ) \times 4$' false

echo "===================================="
echo "Results: $PASS passed, $FAIL failed"
