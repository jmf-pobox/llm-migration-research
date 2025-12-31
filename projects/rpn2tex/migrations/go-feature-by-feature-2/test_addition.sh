#!/bin/bash
# Feature 2: Addition - Manual Verification Script

echo "=== Feature 2: Addition Verification ==="
echo ""

echo "Test 1: Basic Addition"
echo "Input: 5 3 +"
result=$(echo "5 3 +" | go run cmd/rpn2tex/main.go)
expected='$5 + 3$'
echo "Expected: $expected"
echo "Got:      $result"
if [ "$result" = "$expected" ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL"
fi
echo ""

echo "Test 2: Chained Addition"
echo "Input: 1 2 + 3 + 4 +"
result=$(echo "1 2 + 3 + 4 +" | go run cmd/rpn2tex/main.go)
expected='$1 + 2 + 3 + 4$'
echo "Expected: $expected"
echo "Got:      $result"
if [ "$result" = "$expected" ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL"
fi
echo ""

echo "Test 3: Number feature still works (backward compatibility)"
echo "Input: 5"
result=$(echo "5" | go run cmd/rpn2tex/main.go)
expected='$5$'
echo "Expected: $expected"
echo "Got:      $result"
if [ "$result" = "$expected" ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL"
fi
echo ""

echo "Test 4: Decimal number still works"
echo "Input: 3.14"
result=$(echo "3.14" | go run cmd/rpn2tex/main.go)
expected='$3.14$'
echo "Expected: $expected"
echo "Got:      $result"
if [ "$result" = "$expected" ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL"
fi
echo ""

echo "=== All Manual Tests Complete ==="
