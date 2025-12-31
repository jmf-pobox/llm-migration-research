#!/bin/bash

cd /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/java-feature-by-feature-2

echo "=== Feature 4: Multiplication - I/O Contract Validation ==="
echo ""

echo "Test Case 1: '4 7 *' → '\$4 \\times 7\$'"
result1=$(echo "4 7 *" | build/install/rpn2tex/bin/rpn2tex)
expected1='$4 \times 7$'
if [ "$result1" = "$expected1" ]; then
  echo "✓ PASS: $result1"
else
  echo "✗ FAIL: Expected '$expected1', got '$result1'"
fi

echo ""
echo "Test Case 2: '2 3 4 * +' → '\$2 + 3 \\times 4\$'"
result2=$(echo "2 3 4 * +" | build/install/rpn2tex/bin/rpn2tex)
expected2='$2 + 3 \times 4$'
if [ "$result2" = "$expected2" ]; then
  echo "✓ PASS: $result2"
else
  echo "✗ FAIL: Expected '$expected2', got '$result2'"
fi

echo ""
echo "Additional test cases from specification:"
echo ""

echo "Test Case 3: '5 3 * 2 +' → '\$5 \\times 3 + 2\$'"
result3=$(echo "5 3 * 2 +" | build/install/rpn2tex/bin/rpn2tex)
expected3='$5 \times 3 + 2$'
if [ "$result3" = "$expected3" ]; then
  echo "✓ PASS: $result3"
else
  echo "✗ FAIL: Expected '$expected3', got '$result3'"
fi

echo ""
echo "Test Case 4: '5 3 + 2 *' → '\$( 5 + 3 ) \\times 2\$' (precedence test)"
result4=$(echo "5 3 + 2 *" | build/install/rpn2tex/bin/rpn2tex)
expected4='$( 5 + 3 ) \times 2$'
if [ "$result4" = "$expected4" ]; then
  echo "✓ PASS: $result4"
else
  echo "✗ FAIL: Expected '$expected4', got '$result4'"
fi

echo ""
echo "=== All I/O contract tests completed ==="
