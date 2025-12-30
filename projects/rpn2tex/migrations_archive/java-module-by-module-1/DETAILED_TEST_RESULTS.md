# Detailed Test Results - Python rpn2tex Implementation

Generated: 2025-12-28
Total Tests Executed: 21

## Test Results (Detailed Output)

### Test 1: Basic Addition
```
Input:    5 3 +
Output:   $5 + 3$
Status:   SUCCESS
```

### Test 2: Basic Subtraction
```
Input:    5 3 -
Output:   $5 - 3$
Status:   SUCCESS
```

### Test 3: Basic Multiplication
```
Input:    4 7 *
Output:   $4 \times 7$
Status:   SUCCESS
Note:     Uses LaTeX \times command
```

### Test 4: Basic Division
```
Input:    10 2 /
Output:   $10 \div 2$
Status:   SUCCESS
Note:     Uses LaTeX \div command
```

### Test 5: Exponentiation (Unsupported)
```
Input:    2 3 ^
Output:   Error: Unexpected character '^'
Status:   ERROR (Expected)
Note:     Caret operator not supported
```

### Test 6: Operator Precedence - Addition + Multiplication
```
Input:    5 3 + 2 *
Output:   $( 5 + 3 ) \times 2$
Status:   SUCCESS
Note:     Addition (lower precedence) is parenthesized when used in multiplication
```

### Test 7: Operator Precedence - Multiplication + Addition
```
Input:    5 3 * 2 +
Output:   $5 \times 3 + 2$
Status:   SUCCESS
Note:     Multiplication doesn't need parentheses before addition
```

### Test 8: Left-to-Right Evaluation
```
Input:    10 2 / 5 *
Output:   $10 \div 2 \times 5$
Status:   SUCCESS
Note:     Same precedence operators chain left-to-right without parentheses
```

### Test 9: Chained Subtraction
```
Input:    5 3 - 2 -
Output:   $5 - 3 - 2$
Status:   SUCCESS
Note:     Chained operations of same type don't need extra parentheses
```

### Test 10: Chained Division
```
Input:    100 10 / 5 / 2 /
Output:   $100 \div 10 \div 5 \div 2$
Status:   SUCCESS
Note:     Multiple divisions chain without parentheses
```

### Test 11: Chained Addition
```
Input:    1 2 + 3 + 4 +
Output:   $1 + 2 + 3 + 4$
Status:   SUCCESS
Note:     Multiple additions chain without parentheses
```

### Test 12: Precedence - Multiplication + Addition (2)
```
Input:    2 3 4 * +
Output:   $2 + 3 \times 4$
Status:   SUCCESS
Note:     Multiplication computed first, no parentheses needed
```

### Test 13: Precedence - Addition + Multiplication (2)
```
Input:    2 3 + 4 *
Output:   $( 2 + 3 ) \times 4$
Status:   SUCCESS
Note:     Addition parenthesized as operand to multiplication
```

### Test 14: Precedence - Multiplication + Addition (3)
```
Input:    2 3 4 + *
Output:   $2 \times ( 3 + 4 )$
Status:   SUCCESS
Note:     Addition on right side of multiplication, parenthesized
```

### Test 15: Precedence - Multiplication + Addition (4)
```
Input:    2 3 * 4 +
Output:   $2 \times 3 + 4$
Status:   SUCCESS
Note:     Multiplication first, no parentheses needed
```

### Test 16: Exponentiation with Multiplication (Unsupported)
```
Input:    2 3 ^ 4 *
Output:   Error: Unexpected character '^'
Status:   ERROR (Expected)
Note:     Contains unsupported ^ operator
```

### Test 17: Double Exponentiation (Unsupported)
```
Input:    2 3 4 ^ ^
Output:   Error: Unexpected character '^'
Status:   ERROR (Expected)
Note:     Contains unsupported ^ operator
```

### Test 18: Decimal Multiplication
```
Input:    3.14 2 *
Output:   $3.14 \times 2$
Status:   SUCCESS
Note:     Floating-point numbers are preserved exactly
```

### Test 19: Decimal Addition
```
Input:    1.5 0.5 +
Output:   $1.5 + 0.5$
Status:   SUCCESS
Note:     Decimal numbers handled correctly
```

### Test 20: Complex Expression - Multiple Groups
```
Input:    1 2 + 3 4 + *
Output:   $( 1 + 2 ) \times ( 3 + 4 )$
Status:   SUCCESS
Note:     Two parenthesized addition groups multiplied together
```

### Test 21: Complex Expression - Mixed Operations
```
Input:    10 2 / 3 + 4 *
Output:   $( 10 \div 2 + 3 ) \times 4$
Status:   SUCCESS
Note:     Division then addition, grouped and multiplied
```

## Summary Statistics

### By Status
- Successful: 18 tests (85.7%)
- Expected Errors: 3 tests (14.3%)
- Unexpected Errors: 0 tests

### By Category
| Category | Count | Success Rate |
|----------|-------|--------------|
| Basic Operations | 4 | 100% |
| Precedence & Parentheses | 8 | 100% |
| Chained Operations | 3 | 100% |
| Decimal Numbers | 2 | 100% |
| Complex Expressions | 2 | 100% |
| Unsupported Operators | 2 | N/A (errors expected) |

## Patterns Observed

### 1. LaTeX Output Format
All successful outputs follow this pattern:
```
$ <operand> <operator> <operand> $
```
with spaces around operators for readability.

### 2. Operator Rendering
- Addition: `+` (no change)
- Subtraction: `-` (no change)
- Multiplication: `\times` (LaTeX command)
- Division: `\div` (LaTeX command)

### 3. Parenthesization Rules
- **Rule 1:** Lower-precedence operations are parenthesized when used as operands to higher-precedence operations
- **Rule 2:** Same-precedence operations chain without parentheses
- **Rule 3:** Higher-precedence operations don't need parentheses when followed by lower-precedence
- **Rule 4:** All parentheses are wrapped in `( ... )`

### 4. Precedence Hierarchy
From highest to lowest:
1. Multiplication (*) and Division (/) - same level
2. Addition (+) and Subtraction (-) - same level

### 5. Number Handling
- Integer numbers: preserved as-is (e.g., 5 → 5)
- Decimal numbers: preserved exactly (e.g., 3.14 → 3.14)
- No precision issues observed
- No number formatting applied

## Error Handling

All error cases involve the unsupported ^ operator:
```
Error: Unexpected character '^'

<line number> | <input with error>
  <spaces>    | <caret under error position>
```

Error characteristics:
- Clear error message
- Line and column information
- Visual indicator of error position
- Sent to stderr (not stdout)
- Exit code: 1

## Validation Checklist for Java Implementation

Use this checklist to validate the Java implementation:

- [ ] Test 1 produces exactly: `$5 + 3$`
- [ ] Test 2 produces exactly: `$5 - 3$`
- [ ] Test 3 produces exactly: `$4 \times 7$`
- [ ] Test 4 produces exactly: `$10 \div 2$`
- [ ] Test 5 produces error with "Unexpected character '^'"
- [ ] Test 6 produces exactly: `$( 5 + 3 ) \times 2$`
- [ ] Test 7 produces exactly: `$5 \times 3 + 2$`
- [ ] Test 8 produces exactly: `$10 \div 2 \times 5$`
- [ ] Test 9 produces exactly: `$5 - 3 - 2$`
- [ ] Test 10 produces exactly: `$100 \div 10 \div 5 \div 2$`
- [ ] Test 11 produces exactly: `$1 + 2 + 3 + 4$`
- [ ] Test 12 produces exactly: `$2 + 3 \times 4$`
- [ ] Test 13 produces exactly: `$( 2 + 3 ) \times 4$`
- [ ] Test 14 produces exactly: `$2 \times ( 3 + 4 )$`
- [ ] Test 15 produces exactly: `$2 \times 3 + 4$`
- [ ] Test 16 produces error with "Unexpected character '^'"
- [ ] Test 17 produces error with "Unexpected character '^'"
- [ ] Test 18 produces exactly: `$3.14 \times 2$`
- [ ] Test 19 produces exactly: `$1.5 + 0.5$`
- [ ] Test 20 produces exactly: `$( 1 + 2 ) \times ( 3 + 4 )$`
- [ ] Test 21 produces exactly: `$( 10 \div 2 + 3 ) \times 4$`

