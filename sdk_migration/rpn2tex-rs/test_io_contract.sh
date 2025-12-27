#!/bin/bash

# Test I/O Contract Cases
BINARY="./target/release/rpn2tex"

# Success cases
echo "Test 1: 5 3 +"
echo "5 3 +" | $BINARY - 
echo ""

echo "Test 2: 5 3 -"
echo "5 3 -" | $BINARY -
echo ""

echo "Test 3: 4 7 *"
echo "4 7 *" | $BINARY -
echo ""

echo "Test 4: 10 2 /"
echo "10 2 /" | $BINARY -
echo ""

echo "Test 5: 2 3 ^ (should error)"
echo "2 3 ^" | $BINARY - 2>&1
echo ""

echo "Test 6: 5 3 + 2 *"
echo "5 3 + 2 *" | $BINARY -
echo ""

echo "Test 7: 5 3 * 2 +"
echo "5 3 * 2 +" | $BINARY -
echo ""

echo "Test 8: 10 2 / 5 *"
echo "10 2 / 5 *" | $BINARY -
echo ""

echo "Test 9: 5 3 - 2 -"
echo "5 3 - 2 -" | $BINARY -
echo ""

echo "Test 10: 100 10 / 5 / 2 /"
echo "100 10 / 5 / 2 /" | $BINARY -
echo ""

echo "Test 11: 1 2 + 3 + 4 +"
echo "1 2 + 3 + 4 +" | $BINARY -
echo ""

echo "Test 12: 2 3 4 * +"
echo "2 3 4 * +" | $BINARY -
echo ""

echo "Test 13: 2 3 + 4 *"
echo "2 3 + 4 *" | $BINARY -
echo ""

echo "Test 14: 2 3 4 + *"
echo "2 3 4 + *" | $BINARY -
echo ""

echo "Test 15: 2 3 * 4 +"
echo "2 3 * 4 +" | $BINARY -
echo ""

echo "Test 16: 2 3 ^ 4 * (should error)"
echo "2 3 ^ 4 *" | $BINARY - 2>&1
echo ""

echo "Test 17: 2 3 4 ^ ^ (should error)"
echo "2 3 4 ^ ^" | $BINARY - 2>&1
echo ""

echo "Test 18: 3.14 2 *"
echo "3.14 2 *" | $BINARY -
echo ""

echo "Test 19: 1.5 0.5 +"
echo "1.5 0.5 +" | $BINARY -
echo ""

echo "Test 20: 1 2 + 3 4 + *"
echo "1 2 + 3 4 + *" | $BINARY -
echo ""

echo "Test 21: 10 2 / 3 + 4 *"
echo "10 2 / 3 + 4 *" | $BINARY -
echo ""
