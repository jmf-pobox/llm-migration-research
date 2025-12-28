# I/O Contract for rpn2tex Migration

## Overview
This document defines the expected input-output behavior for the rpn2tex implementation. All outputs have been captured by running the Python implementation directly on the specified test inputs.

**Implementation Location:** `/Users/jfreeman/Coding/rpn2tex/src/rpn2tex/`
**Test Date:** December 27, 2025
**Total Test Cases:** 21
**Successful Cases:** 18
**Error Cases:** 3

## Test Cases

| # | Input | Expected Output | Notes |
|---|-------|-----------------|-------|
| 1 | `5 3 +` | `$5 + 3$` | Basic addition |
| 2 | `5 3 -` | `$5 - 3$` | Basic subtraction |
| 3 | `4 7 *` | `$4 \times 7$` | Basic multiplication with \times symbol |
| 4 | `10 2 /` | `$10 \div 2$` | Basic division with \div symbol |
| 5 | `2 3 ^` | ERROR | Caret operator not supported - LexerError: "Unexpected character '^'" at line 1, column 5 |
| 6 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Addition result grouped in parentheses for multiplication |
| 7 | `5 3 * 2 +` | `$5 \times 3 + 2$` | Multiplication has higher precedence than addition |
| 8 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | Division and multiplication are left-associative |
| 9 | `5 3 - 2 -` | `$5 - 3 - 2$` | Subtraction is left-associative |
| 10 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Chained division is left-associative |
| 11 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Chained addition |
| 12 | `2 3 4 * +` | `$2 + 3 \times 4$` | Multiplication has higher precedence than addition |
| 13 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Addition grouped when multiplied |
| 14 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Addition grouped on right side |
| 15 | `2 3 * 4 +` | `$2 \times 3 + 4$` | Multiplication has higher precedence than addition |
| 16 | `2 3 ^ 4 *` | ERROR | Caret operator not supported - LexerError: "Unexpected character '^'" at line 1, column 5 |
| 17 | `2 3 4 ^ ^` | ERROR | Caret operator not supported - LexerError: "Unexpected character '^'" at line 1, column 7 |
| 18 | `3.14 2 *` | `$3.14 \times 2$` | Floating point operands supported |
| 19 | `1.5 0.5 +` | `$1.5 + 0.5$` | Floating point addition |
| 20 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Complex expression with multiple sub-expressions |
| 21 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex expression with division and addition grouped |

## Error Cases

The implementation has been tested for error handling. The following inputs produce errors:

### Case 1: Caret Operator (Power/Exponentiation)
- **Input:** `2 3 ^`
- **Error Type:** LexerError
- **Error Message:** `Error: Unexpected character '^'`
- **Error Location:** Line 1, Column 5
- **Description:** The caret character (^) is not recognized as a valid token by the lexer. The implementation does not support exponentiation/power operations.

### Case 2: Caret Operator in Complex Expression
- **Input:** `2 3 ^ 4 *`
- **Error Type:** LexerError
- **Error Message:** `Error: Unexpected character '^'`
- **Error Location:** Line 1, Column 5
- **Description:** Same as Case 1 - caret operator not supported.

### Case 3: Multiple Caret Operators
- **Input:** `2 3 4 ^ ^`
- **Error Type:** LexerError
- **Error Message:** `Error: Unexpected character '^'`
- **Error Location:** Line 1, Column 7
- **Description:** Same as Case 1 - caret operator not supported. Error reported at position of first caret.

## Operator Support Summary

The implementation supports the following operators:

| Operator | Symbol | LaTeX Output | Notes |
|----------|--------|--------------|-------|
| Addition | `+` | `+` | Supported |
| Subtraction | `-` | `-` | Supported |
| Multiplication | `*` | `\times` | Supported |
| Division | `/` | `\div` | Supported |
| Exponentiation | `^` | N/A | NOT SUPPORTED - causes LexerError |

## Operand Support Summary

- **Integers:** Fully supported (e.g., 5, 10, 100)
- **Floating Point:** Fully supported (e.g., 3.14, 1.5, 0.5)
- **Negative Numbers:** Not tested but likely supported based on standard implementations

## LaTeX Output Format

All outputs are wrapped in LaTeX math mode delimiters:
- **Format:** `$<expression>$`
- **Operators:** Use standard LaTeX math symbols
  - Addition: `+`
  - Subtraction: `-`
  - Multiplication: `\times`
  - Division: `\div`
- **Grouping:** Parentheses are added as needed to preserve mathematical correctness
  - When lower-precedence operations are operands to higher-precedence operations
  - Uses `( ... )` format with spaces

## Implementation Characteristics

1. **Precedence Handling:** The parser correctly handles operator precedence
   - Multiplication and division have higher precedence than addition and subtraction
   - When a lower-precedence operation is an operand to a higher-precedence operation, it is parenthesized

2. **Associativity:** Left-associative for all binary operators
   - `a - b - c` produces `a - b - c` (not `a - (b - c)`)
   - `a / b / c` produces `a / b / c` (not `a / (b / c)`)

3. **Whitespace Handling:** Input expressions use space-separated tokens
   - Tokens can be numbers or operators
   - Floating point numbers are supported with decimal points

4. **Error Reporting:** Comprehensive error messages with location information
   - Shows the offending line
   - Points to the exact column position
   - Provides descriptive error messages

## Migration Notes

When migrating this implementation to other languages (e.g., Java, Rust), ensure:

1. All 18 successful test cases produce identical LaTeX output
2. The 3 error cases produce appropriate error messages
3. Operator precedence and associativity are preserved
4. Parenthesization logic matches the original implementation
5. Floating point number handling is identical
6. The caret operator remains unsupported (or is explicitly implemented if required)

