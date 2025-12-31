#!/bin/bash

# Test script to validate I/O contract for rpn2tex CLI

set -e

cd "$(dirname "$0")"

# Build the CLI
go build -o rpn2tex cmd/rpn2tex/main.go

echo "Running I/O contract tests..."
echo

# Success cases
declare -a inputs=(
    "5 3 +"
    "5 3 -"
    "4 7 *"
    "10 2 /"
    "5 3 + 2 *"
    "5 3 * 2 +"
    "10 2 / 5 *"
    "5 3 - 2 -"
    "100 10 / 5 / 2 /"
    "1 2 + 3 + 4 +"
    "2 3 4 * +"
    "2 3 + 4 *"
    "2 3 4 + *"
    "2 3 * 4 +"
    "3.14 2 *"
    "1.5 0.5 +"
    "1 2 + 3 4 + *"
    "10 2 / 3 + 4 *"
)

declare -a expected=(
    '$5 + 3$'
    '$5 - 3$'
    '$4 \times 7$'
    '$10 \div 2$'
    '$( 5 + 3 ) \times 2$'
    '$5 \times 3 + 2$'
    '$10 \div 2 \times 5$'
    '$5 - 3 - 2$'
    '$100 \div 10 \div 5 \div 2$'
    '$1 + 2 + 3 + 4$'
    '$2 + 3 \times 4$'
    '$( 2 + 3 ) \times 4$'
    '$2 \times ( 3 + 4 )$'
    '$2 \times 3 + 4$'
    '$3.14 \times 2$'
    '$1.5 + 0.5$'
    '$( 1 + 2 ) \times ( 3 + 4 )$'
    '$( 10 \div 2 + 3 ) \times 4$'
)

PASSED=0
FAILED=0

for i in "${!inputs[@]}"; do
    input="${inputs[$i]}"
    expected_output="${expected[$i]}"

    # Run the command
    actual_output=$(echo "$input" | ./rpn2tex -)

    if [ "$actual_output" = "$expected_output" ]; then
        echo "✓ Test $((i+1)): PASSED"
        echo "  Input: $input"
        PASSED=$((PASSED+1))
    else
        echo "✗ Test $((i+1)): FAILED"
        echo "  Input: $input"
        echo "  Expected: $expected_output"
        echo "  Got:      $actual_output"
        FAILED=$((FAILED+1))
    fi
done

# Error cases
echo
echo "Testing error cases..."
echo

declare -a error_inputs=(
    "2 3 ^"
    "2 3 ^ 4 *"
    "2 3 4 ^ ^"
)

for i in "${!error_inputs[@]}"; do
    input="${error_inputs[$i]}"

    # Run the command and capture exit code
    set +e
    output=$(echo "$input" | ./rpn2tex - 2>&1)
    exit_code=$?
    set -e

    if [ $exit_code -eq 1 ] && echo "$output" | grep -q "Unexpected character"; then
        echo "✓ Error test $((i+1)): PASSED"
        echo "  Input: $input"
        PASSED=$((PASSED+1))
    else
        echo "✗ Error test $((i+1)): FAILED"
        echo "  Input: $input"
        echo "  Expected exit code 1 and 'Unexpected character' error"
        echo "  Got exit code: $exit_code"
        echo "  Output: $output"
        FAILED=$((FAILED+1))
    fi
done

echo
echo "========================================"
echo "Results: $PASSED passed, $FAILED failed"
echo "========================================"

if [ $FAILED -eq 0 ]; then
    echo "All tests passed!"
    exit 0
else
    echo "Some tests failed!"
    exit 1
fi
