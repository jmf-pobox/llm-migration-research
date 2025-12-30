#!/bin/bash

# I/O Contract Validation Script
# Tests all 21 test cases from the I/O contract

set -e

BINARY="./target/release/rpn2tex"
PASS_COUNT=0
FAIL_COUNT=0

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

test_case() {
    local num=$1
    local input=$2
    local expected=$3
    local should_error=$4

    echo -n "Test $num: "

    if [ "$should_error" = "true" ]; then
        # Error case - expect exit code 1 and error message
        if result=$(echo "$input" | $BINARY - 2>&1); then
            echo -e "${RED}FAIL${NC} - Expected error but succeeded"
            FAIL_COUNT=$((FAIL_COUNT + 1))
            return 1
        else
            if echo "$result" | grep -q "Unexpected character"; then
                echo -e "${GREEN}PASS${NC} - Error case handled correctly"
                PASS_COUNT=$((PASS_COUNT + 1))
                return 0
            else
                echo -e "${RED}FAIL${NC} - Wrong error message"
                echo "Got: $result"
                FAIL_COUNT=$((FAIL_COUNT + 1))
                return 1
            fi
        fi
    else
        # Success case - expect exit code 0 and exact output
        result=$(echo "$input" | $BINARY - 2>&1)
        if [ "$result" = "$expected" ]; then
            echo -e "${GREEN}PASS${NC}"
            PASS_COUNT=$((PASS_COUNT + 1))
            return 0
        else
            echo -e "${RED}FAIL${NC}"
            echo "  Input:    $input"
            echo "  Expected: $expected"
            echo "  Got:      $result"
            FAIL_COUNT=$((FAIL_COUNT + 1))
            return 1
        fi
    fi
}

echo "========================================"
echo "  I/O Contract Validation - 21 Tests"
echo "========================================"
echo ""

# Build release binary first
echo "Building release binary..."
cargo build --release --quiet
echo ""

# Test cases
test_case 1  "5 3 +"           '$5 + 3$'                                    false
test_case 2  "5 3 -"           '$5 - 3$'                                    false
test_case 3  "4 7 *"           '$4 \times 7$'                               false
test_case 4  "10 2 /"          '$10 \div 2$'                                false
test_case 5  "2 3 ^"           ""                                           true
test_case 6  "5 3 + 2 *"       '$( 5 + 3 ) \times 2$'                       false
test_case 7  "5 3 * 2 +"       '$5 \times 3 + 2$'                           false
test_case 8  "10 2 / 5 *"      '$10 \div 2 \times 5$'                       false
test_case 9  "5 3 - 2 -"       '$5 - 3 - 2$'                                false
test_case 10 "100 10 / 5 / 2 /" '$100 \div 10 \div 5 \div 2$'              false
test_case 11 "1 2 + 3 + 4 +"   '$1 + 2 + 3 + 4$'                            false
test_case 12 "2 3 4 * +"       '$2 + 3 \times 4$'                           false
test_case 13 "2 3 + 4 *"       '$( 2 + 3 ) \times 4$'                       false
test_case 14 "2 3 4 + *"       '$2 \times ( 3 + 4 )$'                       false
test_case 15 "2 3 * 4 +"       '$2 \times 3 + 4$'                           false
test_case 16 "2 3 ^ 4 *"       ""                                           true
test_case 17 "2 3 4 ^ ^"       ""                                           true
test_case 18 "3.14 2 *"        '$3.14 \times 2$'                            false
test_case 19 "1.5 0.5 +"       '$1.5 + 0.5$'                                false
test_case 20 "1 2 + 3 4 + *"   '$( 1 + 2 ) \times ( 3 + 4 )$'               false
test_case 21 "10 2 / 3 + 4 *"  '$( 10 \div 2 + 3 ) \times 4$'               false

echo ""
echo "========================================"
echo -e "  Results: ${GREEN}$PASS_COUNT passed${NC}, ${RED}$FAIL_COUNT failed${NC}"
echo "========================================"

if [ $FAIL_COUNT -eq 0 ]; then
    echo -e "${GREEN}All I/O contract tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed.${NC}"
    exit 1
fi
