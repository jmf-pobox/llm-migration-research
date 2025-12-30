#!/bin/bash

# Test cases from Feature 2: Addition specification

echo "=== Addition Feature I/O Contract Tests ==="
echo ""

# Test 1: Basic addition
echo "Test 1: Basic addition"
echo "Input: '5 3 +'"
cargo run --quiet --example full_pipeline <<< "5 3 +" 2>&1
echo "Expected: \$5 + 3\$"
echo ""

# Test 2: Chained addition
echo "Test 2: Chained addition"
echo "Input: '1 2 + 3 + 4 +'"
cargo run --quiet --example full_pipeline <<< "1 2 + 3 + 4 +" 2>&1
echo "Expected: \$1 + 2 + 3 + 4\$"
echo ""

# Test 3: Addition with floats
echo "Test 3: Addition with floats"
echo "Input: '1.5 0.5 +'"
cargo run --quiet --example full_pipeline <<< "1.5 0.5 +" 2>&1
echo "Expected: \$1.5 + 0.5\$"
echo ""

# Test 4: Missing operand error
echo "Test 4: Missing operand (should error)"
echo "Input: '5 +'"
cargo run --quiet --example full_pipeline <<< "5 +" 2>&1
echo "Expected: Error with 'Not enough operands'"
echo ""

# Test 5: Extra operand error
echo "Test 5: Extra operand (should error)"
echo "Input: '5 3 2 +'"
cargo run --quiet --example full_pipeline <<< "5 3 2 +" 2>&1
echo "Expected: Error with 'Expected single result'"
