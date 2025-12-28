#!/bin/bash

BINARY="./target/release/rpn2tex"

echo "Testing file I/O..."
echo "==================="

# Test 1: Read from file, write to stdout
echo "Test 1: Read from file, write to stdout"
echo "5 3 +" > /tmp/test_input.rpn
output=$($BINARY /tmp/test_input.rpn)
if [ "$output" = '$5 + 3$' ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Expected '\$5 + 3\$', got '$output'"
fi

# Test 2: Read from stdin, write to file
echo "Test 2: Read from stdin, write to file"
echo "4 7 *" | $BINARY - -o /tmp/test_output.tex
if [ -f /tmp/test_output.tex ]; then
    content=$(cat /tmp/test_output.tex)
    # Note: File output should have a trailing newline
    expected=$'$4 \\times 7$\n'
    if [ "$content" = "$expected" ]; then
        echo "✓ PASS"
    else
        echo "✗ FAIL: Content mismatch"
        echo "Expected: '$expected'"
        echo "Got: '$content'"
    fi
else
    echo "✗ FAIL: Output file not created"
fi

# Test 3: Read from file, write to file
echo "Test 3: Read from file, write to file"
echo "10 2 /" > /tmp/test_input2.rpn
$BINARY /tmp/test_input2.rpn -o /tmp/test_output2.tex 2>&1 | grep -q "Generated:"
if [ $? -eq 0 ] && [ -f /tmp/test_output2.tex ]; then
    content=$(cat /tmp/test_output2.tex)
    expected=$'$10 \\div 2$\n'
    if [ "$content" = "$expected" ]; then
        echo "✓ PASS"
    else
        echo "✗ FAIL: Content mismatch"
    fi
else
    echo "✗ FAIL: Output file not created or success message not shown"
fi

# Test 4: File not found (should return exit code 1)
echo "Test 4: File not found error"
$BINARY /nonexistent/file.rpn 2>&1 | grep -q "Error:"
exit_code=$?
if [ $exit_code -eq 0 ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Should show error for missing file"
fi

# Clean up
rm -f /tmp/test_input.rpn /tmp/test_input2.rpn /tmp/test_output.tex /tmp/test_output2.tex

echo "==================="
echo "File I/O tests complete"
