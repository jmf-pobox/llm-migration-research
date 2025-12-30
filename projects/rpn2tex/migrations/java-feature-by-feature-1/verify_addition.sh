#!/bin/bash
set -e

echo "=== Addition Feature I/O Contract Verification ==="
echo ""

# Build the project
echo "Building project..."
./gradlew build -q > /dev/null 2>&1
echo "Build successful."
echo ""

# Function to test an input
test_input() {
    local input="$1"
    local expected="$2"
    echo "Test: '$input'"
    echo "Expected: $expected"
    
    # Create temp file
    echo "$input" > /tmp/rpn_test_input.txt
    
    # Run the program
    actual=$(./gradlew run -q --args="/tmp/rpn_test_input.txt" 2>/dev/null | tail -1)
    
    echo "Actual:   $actual"
    
    if [ "$actual" = "$expected" ]; then
        echo "✓ PASS"
    else
        echo "✗ FAIL"
        exit 1
    fi
    echo ""
}

# Test numbers (should still work)
echo "--- Numbers Feature (Regression Test) ---"
test_input "5" '$5$'
test_input "3.14" '$3.14$'

# Test addition
echo "--- Addition Feature ---"
test_input "5 3 +" '$5 + 3$'
test_input "1 2 + 3 + 4 +" '$1 + 2 + 3 + 4$'

echo "=== All tests passed! ==="
