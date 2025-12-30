#!/bin/bash
# Verification script for the numbers feature migration

set -e

echo "=========================================="
echo "Numbers Feature Migration Verification"
echo "=========================================="
echo ""

# Build the project
echo "1. Building project..."
./gradlew compileJava --quiet
echo "   ✓ Compilation successful"
echo ""

# Run tests
echo "2. Running tests..."
./gradlew test --quiet
echo "   ✓ All tests passed"
echo ""

# Run checkstyle
echo "3. Running checkstyle..."
./gradlew checkstyleMain --quiet
echo "   ✓ Code style validation passed"
echo ""

# Test I/O contract cases
echo "4. Testing I/O contract cases..."

# Test case 1: "5" -> "$5$"
echo -n "   Testing: '5' -> '\$5\$' ... "
echo "5" > /tmp/test_input_1.txt
RESULT=$(./gradlew -q run --args="/tmp/test_input_1.txt" 2>/dev/null)
if [ "$RESULT" = "\$5\$" ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL (got: $RESULT)"
    exit 1
fi
rm /tmp/test_input_1.txt

# Test case 2: "3.14" -> "$3.14$"
echo -n "   Testing: '3.14' -> '\$3.14\$' ... "
echo "3.14" > /tmp/test_input_2.txt
RESULT=$(./gradlew -q run --args="/tmp/test_input_2.txt" 2>/dev/null)
if [ "$RESULT" = "\$3.14\$" ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL (got: $RESULT)"
    exit 1
fi
rm /tmp/test_input_2.txt

echo ""
echo "=========================================="
echo "✓ All verification checks passed!"
echo "=========================================="
echo ""
echo "Migration Summary:"
echo "  - Feature: Numbers (parse and output numeric literals)"
echo "  - Files created: 8 source files, 5 test files"
echo "  - Quality gates: All passed"
echo "  - I/O contract: 2/2 test cases passing"
echo ""
