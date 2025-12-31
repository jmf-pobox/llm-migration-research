# I/O Contract for rpn2tex Migration - PHASE 0

## Overview

This document specifies the exact input-output behavior of the Python reference implementation of rpn2tex. This contract serves as the canonical specification for validating the Rust migration implementation.

**Generated from:** Python rpn2tex source implementation
**Test Date:** 2025-12-29
**Total Test Cases:** 21
**Successful Cases:** 18
**Error Cases:** 3

## Test Cases

| # | Input | Expected LaTeX Output | Notes |
|---|-------|----------------------|-------|
| 1 | `5 3 +` | `$5 + 3$` | Basic addition |
| 2 | `5 3 -` | `$5 - 3$` | Basic subtraction |
| 3 | `4 7 *` | `$4 \times 7$` | Basic multiplication with proper LaTeX symbol |
| 4 | `10 2 /` | `$10 \div 2$` | Basic division with proper LaTeX symbol |
| 5 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Parentheses added for operator precedence |
| 6 | `5 3 * 2 +` | `$5 \times 3 + 2$` | No parentheses needed (multiplication before addition) |
| 7 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | Left-to-right evaluation for same precedence |
| 8 | `5 3 - 2 -` | `$5 - 3 - 2$` | Subtraction is left-associative |
| 9 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Multiple divisions are left-associative |
| 10 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Multiple additions without unnecessary parentheses |
| 11 | `2 3 4 * +` | `$2 + 3 \times 4$` | Multiplication precedence over addition (no parentheses) |
| 12 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Addition has lower precedence, parentheses added |
| 13 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Right operand addition needs parentheses |
| 14 | `2 3 * 4 +` | `$2 \times 3 + 4$` | No parentheses needed (multiplication before addition) |
| 15 | `3.14 2 *` | `$3.14 \times 2$` | Floating-point numbers supported |
| 16 | `1.5 0.5 +` | `$1.5 + 0.5$` | Floating-point addition |
| 17 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Multiple additions with multiplication |
| 18 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex expression with multiple operators |

## Error Cases

These test cases demonstrate inputs that the Python reference implementation rejects:

| Input | Error Message | Reason |
|-------|---------------|--------|
| `2 3 ^` | `Line 1, column 5: Unexpected character '^'` | Exponentiation operator (^) not implemented in base Python version |
| `2 3 ^ 4 *` | `Line 1, column 5: Unexpected character '^'` | Exponentiation operator (^) not implemented |
| `2 3 4 ^ ^` | `Line 1, column 7: Unexpected character '^'` | Exponentiation operator (^) not implemented |

## Notes on Implementation

### Supported Operators
- **Addition (+):** Rendered as `+` in LaTeX
- **Subtraction (-):** Rendered as `-` in LaTeX
- **Multiplication (*):** Rendered as `\times` in LaTeX
- **Division (/):** Rendered as `\div` in LaTeX

### Not Implemented
- **Exponentiation (^):** The tokens.py file indicates this is an exercise feature (see comment "Exercise: Add CARET, SQRT, ROOT token types here")
- **Square root (sqrt):** Not implemented in base version
- **Nth root (root):** Not implemented in base version

### Parenthesization Rules
The implementation correctly adds parentheses based on operator precedence:
1. Multiplication and division have higher precedence than addition and subtraction
2. Operators of the same precedence are evaluated left-to-right
3. Parentheses are inserted only when necessary (based on the right operand's precedence)

### Floating-Point Support
- Numbers can contain decimal points (e.g., 3.14, 1.5)
- Decimal representation is preserved in the output

## LaTeX Output Format
All outputs are wrapped in LaTeX math mode delimiters: `$ ... $`

## For Migration Validation
When implementing the Rust version, ensure:
1. All 18 successful test cases produce identical LaTeX output
2. The 3 error cases should raise appropriate errors (may differ slightly in error messages)
3. Operator symbols match exactly: `\times` for multiplication, `\div` for division
4. Parenthesization logic matches exactly - test cases show where parentheses are or aren't added
5. Floating-point numbers are handled correctly
6. All outputs are wrapped in `$...$` delimiters
