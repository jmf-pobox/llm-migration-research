# I/O Contract for rpn2tex Migration

## Overview

This document captures the exact input/output behavior of the Python rpn2tex implementation. These outputs serve as the ground truth for validating the Go implementation during migration.

**Date Generated:** 2025-12-30
**Implementation:** Python rpn2tex
**Test Framework:** Automated test execution with exact output capture

## Test Cases

### Successful Cases

| # | Input | Expected Output | Notes |
|---|-------|-----------------|-------|
| 1 | `5 3 +` | `$5 + 3$` | Simple addition |
| 2 | `5 3 -` | `$5 - 3$` | Simple subtraction |
| 3 | `4 7 *` | `$4 \times 7$` | Simple multiplication with \times operator |
| 4 | `10 2 /` | `$10 \div 2$` | Simple division with \div operator |
| 5 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Parentheses added for lower precedence operation first |
| 6 | `5 3 * 2 +` | `$5 \times 3 + 2$` | No parentheses needed (multiplication has higher precedence) |
| 7 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | Left-to-right evaluation for same precedence |
| 8 | `5 3 - 2 -` | `$5 - 3 - 2$` | Multiple subtractions left-to-right |
| 9 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Chain of divisions |
| 10 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Chain of additions |
| 11 | `2 3 4 * +` | `$2 + 3 \times 4$` | Multiplication before addition (respects precedence) |
| 12 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Parentheses for addition computed before multiplication |
| 13 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Parentheses for addition computed before outer multiplication |
| 14 | `2 3 * 4 +` | `$2 \times 3 + 4$` | Multiplication has higher precedence than addition |
| 15 | `3.14 2 *` | `$3.14 \times 2$` | Floating point number support |
| 16 | `1.5 0.5 +` | `$1.5 + 0.5$` | Floating point addition |
| 17 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Multiple parenthesized subexpressions |
| 18 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex expression with mixed operators |

### Error Cases

| # | Input | Error Message | Notes |
|---|-------|---------------|-------|
| 1 | `2 3 ^` | `Error: Unexpected character '^'` | Exponentiation operator not supported in lexer |
| 2 | `2 3 ^ 4 *` | `Error: Unexpected character '^'` | Exponentiation operator not supported in lexer |
| 3 | `2 3 4 ^ ^` | `Error: Unexpected character '^'` | Exponentiation operator not supported in lexer |

## Output Format Specifications

### LaTeX Math Mode
- All outputs are wrapped in `$...$` delimiters (LaTeX inline math mode)
- No newline characters in output

### Operator Mappings
- Addition: ` + ` (space-delimited)
- Subtraction: ` - ` (space-delimited)
- Multiplication: ` \times ` (space-delimited, backslash-escaped)
- Division: ` \div ` (space-delimited, backslash-escaped)
- Exponentiation: NOT SUPPORTED (causes LexerError)

### Parentheses
- Spaces around parenthesized expressions: `( expr )` not `(expr)`
- Parentheses added when needed to preserve operator precedence:
  - Addition/subtraction have equal precedence
  - Multiplication/division have equal precedence (higher than addition/subtraction)
  - Operations of same precedence are evaluated left-to-right

### Floating Point Numbers
- Decimal points preserved as-is in output
- No special formatting applied

## Testing Methodology

All test inputs were executed using:
```bash
echo "INPUT" | python -m rpn2tex.cli -
```

Outputs capture:
- Standard output (stdout)
- Standard error (stderr) for error cases
- Exit codes

## Exit Code Summary

- **Exit Code 0:** Successful parsing and generation
- **Exit Code 1:** Lexer or parser error (e.g., unsupported character)

## Migration Validation Checklist

When migrating to Go, the implementation must:

- [ ] Support all 18 successful test cases with identical LaTeX output
- [ ] Handle floating point numbers correctly
- [ ] Implement proper operator precedence rules
- [ ] Add parentheses only when necessary to preserve operation order
- [ ] Return exit code 1 for unsupported characters (exponentiation)
- [ ] Produce error messages for invalid syntax
- [ ] Wrap output in `$...$` math mode delimiters
- [ ] Use `\times` for multiplication and `\div` for division

## Notes

### Unsupported Features
The Python implementation does not support the exponentiation operator `^`. Any input containing this character will fail at the lexer stage with an "Unexpected character" error.

### Precedence Rules Observed
1. Multiplication (`\times`) and Division (`\div`) have higher precedence than Addition (`+`) and Subtraction (`-`)
2. Operations of the same precedence level are evaluated left-to-right
3. Parentheses are inserted in the output only when necessary to preserve the correct evaluation order

### Output Whitespace
- Operators are consistently surrounded by single spaces
- Parentheses are surrounded by spaces: ` ( ` and ` ) `
- No trailing whitespace in output
