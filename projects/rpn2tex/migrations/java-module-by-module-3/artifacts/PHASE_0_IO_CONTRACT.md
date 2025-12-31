# I/O Contract for rpn2tex Migration

## Overview

This document defines the expected input/output behavior for the Python rpn2tex implementation. All test cases were run against the original Python implementation to establish the ground truth for validating Java and Rust migrations.

**Generated**: 2025-12-30
**Source Implementation**: Python (rpn2tex)
**Test Framework**: Direct CLI execution with stdin input

---

## Test Cases Summary

### Successful Cases (Exit Code 0): 18 tests
- Basic arithmetic operations: addition, subtraction, multiplication, division
- Floating-point numbers
- Complex expressions with multiple operators
- Proper parenthesization when needed

### Error Cases (Exit Code 1): 3 tests
- Exponentiation operator (^) not supported in base implementation
- Character validation at lexer stage

---

## Detailed Test Cases

### Test 1: Basic Addition
```
INPUT: 5 3 +
EXPECTED OUTPUT: $5 + 3$
EXIT CODE: 0
```

### Test 2: Basic Subtraction
```
INPUT: 5 3 -
EXPECTED OUTPUT: $5 - 3$
EXIT CODE: 0
```

### Test 3: Basic Multiplication
```
INPUT: 4 7 *
EXPECTED OUTPUT: $4 \times 7$
EXIT CODE: 0
```

### Test 4: Basic Division
```
INPUT: 10 2 /
EXPECTED OUTPUT: $10 \div 2$
EXIT CODE: 0
```

### Test 5: Exponentiation (Not Supported)
```
INPUT: 2 3 ^
EXPECTED OUTPUT: Error: Unexpected character '^'
EXIT CODE: 1
NOTES: The ^ character is not recognized by the lexer.
       Error is properly formatted with location info (line 1, column 5).
```

### Test 6: Complex Expression - Addition then Multiplication
```
INPUT: 5 3 + 2 *
EXPECTED OUTPUT: $( 5 + 3 ) \times 2$
EXIT CODE: 0
NOTES: Parentheses inserted to show operation precedence.
```

### Test 7: Complex Expression - Multiplication then Addition
```
INPUT: 5 3 * 2 +
EXPECTED OUTPUT: $5 \times 3 + 2$
EXIT CODE: 0
NOTES: No parentheses needed due to operator precedence.
```

### Test 8: Complex Expression - Division then Multiplication
```
INPUT: 10 2 / 5 *
EXPECTED OUTPUT: $10 \div 2 \times 5$
EXIT CODE: 0
NOTES: Left-associative binary operators.
```

### Test 9: Complex Expression - Multiple Subtractions
```
INPUT: 5 3 - 2 -
EXPECTED OUTPUT: $5 - 3 - 2$
EXIT CODE: 0
NOTES: Left-associative chaining.
```

### Test 10: Complex Expression - Multiple Divisions
```
INPUT: 100 10 / 5 / 2 /
EXPECTED OUTPUT: $100 \div 10 \div 5 \div 2$
EXIT CODE: 0
NOTES: Chain of divisions, left-associative.
```

### Test 11: Complex Expression - Multiple Additions
```
INPUT: 1 2 + 3 + 4 +
EXPECTED OUTPUT: $1 + 2 + 3 + 4$
EXIT CODE: 0
NOTES: Chain of additions.
```

### Test 12: Complex Expression - Mixed Operators (Addition and Multiplication)
```
INPUT: 2 3 4 * +
EXPECTED OUTPUT: $2 + 3 \times 4$
EXIT CODE: 0
NOTES: Respects multiplication precedence over addition.
```

### Test 13: Complex Expression - Parenthesized Addition (Higher Precedence)
```
INPUT: 2 3 + 4 *
EXPECTED OUTPUT: $( 2 + 3 ) \times 4$
EXIT CODE: 0
NOTES: Addition happens first in RPN, so parentheses added.
```

### Test 14: Complex Expression - Parenthesized Addition (Right Operand)
```
INPUT: 2 3 4 + *
EXPECTED OUTPUT: $2 \times ( 3 + 4 )$
EXIT CODE: 0
NOTES: Addition is right operand of multiplication.
```

### Test 15: Complex Expression - Multiplication then Addition
```
INPUT: 2 3 * 4 +
EXPECTED OUTPUT: $2 \times 3 + 4$
EXIT CODE: 0
NOTES: No parentheses needed.
```

### Test 16: Exponentiation with Other Operators (Not Supported)
```
INPUT: 2 3 ^ 4 *
EXPECTED OUTPUT: Error: Unexpected character '^'
EXIT CODE: 1
NOTES: Exponentiation operator not supported (column 5).
```

### Test 17: Multiple Exponentiation (Not Supported)
```
INPUT: 2 3 4 ^ ^
EXPECTED OUTPUT: Error: Unexpected character '^'
EXIT CODE: 1
NOTES: Exponentiation operator not supported (first occurrence, column 7).
```

### Test 18: Floating-Point Multiplication
```
INPUT: 3.14 2 *
EXPECTED OUTPUT: $3.14 \times 2$
EXIT CODE: 0
NOTES: Decimal numbers handled correctly.
```

### Test 19: Floating-Point Addition
```
INPUT: 1.5 0.5 +
EXPECTED OUTPUT: $1.5 + 0.5$
EXIT CODE: 0
NOTES: Decimal numbers handled correctly.
```

### Test 20: Complex Expression - Two Parenthesized Additions
```
INPUT: 1 2 + 3 4 + *
EXPECTED OUTPUT: $( 1 + 2 ) \times ( 3 + 4 )$
EXIT CODE: 0
NOTES: Multiple parenthesized subexpressions in multiplication.
```

### Test 21: Complex Expression - Division with Subsequent Addition and Multiplication
```
INPUT: 10 2 / 3 + 4 *
EXPECTED OUTPUT: $( 10 \div 2 + 3 ) \times 4$
EXIT CODE: 0
NOTES: Mixed operations with proper precedence and parenthesization.
```

---

## LaTeX Output Format Details

### Operator Symbols
- Addition: `+`
- Subtraction: `-`
- Multiplication: `\times`
- Division: `\div`
- Exponentiation: Not supported (raises lexer error)

### Formatting
- All output is wrapped in inline math mode: `$...$`
- Numbers preserve their input format (integers as integers, decimals as decimals)
- Parentheses are added when needed to show operation precedence
- Space-separated in LaTeX output for readability

### Parenthesization Rules

Parentheses are inserted when:
1. A lower-precedence operation is used as an operand to a higher-precedence operation
2. Example: `(2 + 3)` when the addition result is multiplied

Parentheses are NOT inserted when:
1. Operators have the same precedence and are properly ordered
2. A higher-precedence operation is used as an operand to a lower-precedence operation
3. Example: `2 * 3 + 4` (no parentheses needed)

---

## Error Handling

### Lexer Errors
The lexer validates all characters in the input. Unrecognized characters produce errors.

**Error Format**:
```
Error: Unexpected character '<char>'
<line> | <input>
        | <pointer to error column>
```

**Supported Characters**:
- Digits: `0-9`
- Decimal point: `.`
- Whitespace: space, tab, newline
- Operators: `+`, `-`, `*`, `/`

**Unsupported Characters**:
- Exponentiation: `^`
- Any other non-alphanumeric or operator characters

---

## Test Summary Table

| # | Input | Output | Exit Code | Notes |
|---|-------|--------|-----------|-------|
| 1 | `5 3 +` | `$5 + 3$` | 0 | Basic addition |
| 2 | `5 3 -` | `$5 - 3$` | 0 | Basic subtraction |
| 3 | `4 7 *` | `$4 \times 7$` | 0 | Basic multiplication |
| 4 | `10 2 /` | `$10 \div 2$` | 0 | Basic division |
| 5 | `2 3 ^` | Error (lexer) | 1 | Unsupported operator |
| 6 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | 0 | Precedence with parens |
| 7 | `5 3 * 2 +` | `$5 \times 3 + 2$` | 0 | Precedence no parens |
| 8 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | 0 | Left-associative ops |
| 9 | `5 3 - 2 -` | `$5 - 3 - 2$` | 0 | Chained subtraction |
| 10 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | 0 | Chained division |
| 11 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | 0 | Chained addition |
| 12 | `2 3 4 * +` | `$2 + 3 \times 4$` | 0 | Mixed ops, no parens |
| 13 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | 0 | Addition parenthesized |
| 14 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | 0 | Right operand parens |
| 15 | `2 3 * 4 +` | `$2 \times 3 + 4$` | 0 | Mult then add |
| 16 | `2 3 ^ 4 *` | Error (lexer) | 1 | Unsupported operator |
| 17 | `2 3 4 ^ ^` | Error (lexer) | 1 | Unsupported operator |
| 18 | `3.14 2 *` | `$3.14 \times 2$` | 0 | Decimal numbers |
| 19 | `1.5 0.5 +` | `$1.5 + 0.5$` | 0 | Decimal addition |
| 20 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | 0 | Double parenthesized |
| 21 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | 0 | Complex mixed ops |

---

## Validation Checklist for Implementations

When implementing rpn2tex in another language (Java, Rust, etc.), verify:

- [ ] All 18 success cases produce EXACT LaTeX output
- [ ] All 3 error cases with `^` operator produce lexer errors with exit code 1
- [ ] Error messages include character location information
- [ ] Decimal numbers are preserved in output (not converted to integers)
- [ ] LaTeX symbols are correct: `\times` for multiplication, `\div` for division
- [ ] Parentheses are inserted only when necessary for precedence
- [ ] Output is wrapped in `$...$` for inline math mode
- [ ] No extra whitespace at beginning/end of output
- [ ] Exit code 0 on success, 1 on error
