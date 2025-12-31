#!/bin/bash
# Comprehensive validation script for all 21 I/O contract test cases
# This script validates that Main.java produces EXACT outputs for all test cases

set -e

PROJECT_DIR="/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/java-module-by-module-3"
cd "$PROJECT_DIR"

# Ensure the project is built
echo "Building project..."
./gradlew compileJava -q

# ANSI color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASSED=0
FAILED=0

# Function to run a test case
run_test() {
    local test_num=$1
    local input=$2
    local expected_output=$3
    local expected_exit=$4
    local description=$5

    echo -n "Test $test_num: $description... "

    # Run the command and capture output and exit code
    actual_output=$(echo "$input" | java -cp build/classes/java/main com.rpn2tex.Main - 2>&1)
    actual_exit=$?

    # Check exit code
    if [ "$actual_exit" -ne "$expected_exit" ]; then
        echo -e "${RED}FAILED${NC}"
        echo "  Expected exit code: $expected_exit"
        echo "  Actual exit code: $actual_exit"
        FAILED=$((FAILED + 1))
        return
    fi

    # For success cases, check exact output
    if [ "$expected_exit" -eq 0 ]; then
        if [ "$actual_output" = "$expected_output" ]; then
            echo -e "${GREEN}PASSED${NC}"
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}FAILED${NC}"
            echo "  Expected: $expected_output"
            echo "  Actual:   $actual_output"
            FAILED=$((FAILED + 1))
        fi
    else
        # For error cases, just check that output contains "Error:"
        if echo "$actual_output" | grep -q "Error:"; then
            echo -e "${GREEN}PASSED${NC}"
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}FAILED${NC}"
            echo "  Expected error message with 'Error:' prefix"
            echo "  Actual:   $actual_output"
            FAILED=$((FAILED + 1))
        fi
    fi
}

echo "========================================="
echo "I/O Contract Validation - 21 Test Cases"
echo "========================================="
echo ""

# Success cases (18 tests)
run_test 1 "5 3 +" "\$5 + 3\$" 0 "Basic addition"
run_test 2 "5 3 -" "\$5 - 3\$" 0 "Basic subtraction"
run_test 3 "4 7 *" "\$4 \\times 7\$" 0 "Basic multiplication"
run_test 4 "10 2 /" "\$10 \\div 2\$" 0 "Basic division"
run_test 6 "5 3 + 2 *" "\$( 5 + 3 ) \\times 2\$" 0 "Precedence with parens"
run_test 7 "5 3 * 2 +" "\$5 \\times 3 + 2\$" 0 "Precedence no parens"
run_test 8 "10 2 / 5 *" "\$10 \\div 2 \\times 5\$" 0 "Left-associative ops"
run_test 9 "5 3 - 2 -" "\$5 - 3 - 2\$" 0 "Chained subtraction"
run_test 10 "100 10 / 5 / 2 /" "\$100 \\div 10 \\div 5 \\div 2\$" 0 "Chained division"
run_test 11 "1 2 + 3 + 4 +" "\$1 + 2 + 3 + 4\$" 0 "Chained addition"
run_test 12 "2 3 4 * +" "\$2 + 3 \\times 4\$" 0 "Mixed ops, no parens"
run_test 13 "2 3 + 4 *" "\$( 2 + 3 ) \\times 4\$" 0 "Addition parenthesized"
run_test 14 "2 3 4 + *" "\$2 \\times ( 3 + 4 )\$" 0 "Right operand parens"
run_test 15 "2 3 * 4 +" "\$2 \\times 3 + 4\$" 0 "Mult then add"
run_test 18 "3.14 2 *" "\$3.14 \\times 2\$" 0 "Decimal numbers"
run_test 19 "1.5 0.5 +" "\$1.5 + 0.5\$" 0 "Decimal addition"
run_test 20 "1 2 + 3 4 + *" "\$( 1 + 2 ) \\times ( 3 + 4 )\$" 0 "Double parenthesized"
run_test 21 "10 2 / 3 + 4 *" "\$( 10 \\div 2 + 3 ) \\times 4\$" 0 "Complex mixed ops"

# Error cases (3 tests)
run_test 5 "2 3 ^" "" 1 "Unsupported operator (^)"
run_test 16 "2 3 ^ 4 *" "" 1 "Unsupported operator in expression"
run_test 17 "2 3 4 ^ ^" "" 1 "Multiple unsupported operators"

echo ""
echo "========================================="
echo "Results: ${GREEN}$PASSED passed${NC}, ${RED}$FAILED failed${NC} out of 21 tests"
echo "========================================="

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All I/O contract tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed. Please review the output above.${NC}"
    exit 1
fi
