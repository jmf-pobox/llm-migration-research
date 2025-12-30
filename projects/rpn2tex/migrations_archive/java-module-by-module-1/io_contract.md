# I/O Contract for rpn2tex Migration

Generated: 2025-12-28

## Executive Summary

The Python rpn2tex implementation processes RPN (Reverse Polish Notation) expressions and converts them to LaTeX mathematical expressions. The implementation handles basic arithmetic operators (+, -, *, /) and properly manages operator precedence through parenthesization in the generated LaTeX output.

**Important Finding:** The caret operator (^) for exponentiation is NOT supported by the current implementation and produces a lexer error.

## Test Cases

| # | Input | Expected Output | Status | Notes |
|---|-------|-----------------|--------|-------|
| 1 | `5 3 +` | `$5 + 3$` | SUCCESS | Basic addition |
| 2 | `5 3 -` | `$5 - 3$` | SUCCESS | Basic subtraction |
| 3 | `4 7 *` | `$4 \times 7$` | SUCCESS | Basic multiplication with \times |
| 4 | `10 2 /` | `$10 \div 2$` | SUCCESS | Basic division with \div |
| 5 | `2 3 ^` | ERROR: Unexpected character '^' | ERROR | Exponentiation not supported |
| 6 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | SUCCESS | Parenthesizes lower precedence operations |
| 7 | `5 3 * 2 +` | `$5 \times 3 + 2$` | SUCCESS | Respects operator precedence |
| 8 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | SUCCESS | Left-to-right evaluation of same precedence |
| 9 | `5 3 - 2 -` | `$5 - 3 - 2$` | SUCCESS | Chained subtraction |
| 10 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | SUCCESS | Multiple divisions |
| 11 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | SUCCESS | Chained additions |
| 12 | `2 3 4 * +` | `$2 + 3 \times 4$` | SUCCESS | Multiplication has higher precedence than addition |
| 13 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | SUCCESS | Parenthesizes addition when multiplied |
| 14 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | SUCCESS | Parenthesizes addition in multiplication context |
| 15 | `2 3 * 4 +` | `$2 \times 3 + 4$` | SUCCESS | Multiplication before addition without parentheses |
| 16 | `2 3 ^ 4 *` | ERROR: Unexpected character '^' | ERROR | Exponentiation not supported |
| 17 | `2 3 4 ^ ^` | ERROR: Unexpected character '^' | ERROR | Exponentiation not supported |
| 18 | `3.14 2 *` | `$3.14 \times 2$` | SUCCESS | Handles decimal numbers |
| 19 | `1.5 0.5 +` | `$1.5 + 0.5$` | SUCCESS | Decimal arithmetic |
| 20 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | SUCCESS | Multiple parenthesized groups |
| 21 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | SUCCESS | Complex precedence handling |

## Summary

- **Total tests:** 21
- **Successful:** 18
- **Errors:** 3 (all related to unsupported ^ operator)

## Detailed Observations

### Supported Operators

1. **Addition (+)** - Renders as `+`
2. **Subtraction (-)** - Renders as `-`
3. **Multiplication (*)** - Renders as `\times`
4. **Division (/)** - Renders as `\div`

### LaTeX Generation Behavior

1. **Operator Precedence:** The implementation correctly respects mathematical operator precedence:
   - Multiplication and division have higher precedence than addition and subtraction
   - When lower-precedence operations are used as operands for higher-precedence operations, they are wrapped in parentheses

2. **Parenthesization Strategy:**
   - Parentheses are added around lower-precedence operations when they appear as operands to higher-precedence operations
   - Chained operations of the same precedence level do not get parenthesized
   - LaTeX expressions are wrapped in `$ ... $` delimiters for inline math mode

3. **Number Handling:**
   - Both integers and decimal numbers are supported
   - Numbers are preserved exactly as input (3.14 remains 3.14)

### Unsupported Features

- **Exponentiation (^):** The lexer explicitly rejects the caret character as an unexpected character
- This means any test case containing the ^ operator will fail with a lexer error

### Output Format

All successful outputs:
- Are wrapped in `$ ... $` for LaTeX inline math mode
- Use proper LaTeX commands: `\times` for multiplication, `\div` for division
- Include spaces around operators for readability
- Use parentheses for precedence disambiguation

## Error Handling

The implementation provides clear error messages for unsupported input:
- Error messages indicate the exact line and column of the problematic character
- Output is directed to stderr
- Exit code is 1 for errors

Example error output:
```
Error: Unexpected character '^'

1 | 2 3 ^
  |     ^
```

## Implementation Reference

- **Source location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
- **Entry point:** `cli.py` with `rpn2tex.cli` module
- **Lexer:** `lexer.py` - Tokenizes input
- **Parser:** `parser.py` - Builds AST from tokens
- **Generator:** `latex_gen.py` - Generates LaTeX from AST

## Notes for Migration

1. The Java implementation must support only the four basic operators (+, -, *, /)
2. The ^ operator should produce the same lexer error as the Python version
3. Operator precedence and parenthesization rules must match exactly
4. LaTeX output format (including `$ ... $` delimiters) must be identical
5. Error messages should match the Python implementation's format

