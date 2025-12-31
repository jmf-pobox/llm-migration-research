# RPN2TEX I/O Contract - Quick Reference

## Test Results Summary

**21 test cases executed | 18 pass | 3 error (expected)**

## All Test Cases at a Glance

| # | Input | Output | Status |
|----|-------|--------|--------|
| 1 | `5 3 +` | `$5 + 3$` | PASS |
| 2 | `5 3 -` | `$5 - 3$` | PASS |
| 3 | `4 7 *` | `$4 \times 7$` | PASS |
| 4 | `10 2 /` | `$10 \div 2$` | PASS |
| 5 | `2 3 ^` | LexerError | ERROR |
| 6 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | PASS |
| 7 | `5 3 * 2 +` | `$5 \times 3 + 2$` | PASS |
| 8 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | PASS |
| 9 | `5 3 - 2 -` | `$5 - 3 - 2$` | PASS |
| 10 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | PASS |
| 11 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | PASS |
| 12 | `2 3 4 * +` | `$2 + 3 \times 4$` | PASS |
| 13 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | PASS |
| 14 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | PASS |
| 15 | `2 3 * 4 +` | `$2 \times 3 + 4$` | PASS |
| 16 | `2 3 ^ 4 *` | LexerError | ERROR |
| 17 | `2 3 4 ^ ^` | LexerError | ERROR |
| 18 | `3.14 2 *` | `$3.14 \times 2$` | PASS |
| 19 | `1.5 0.5 +` | `$1.5 + 0.5$` | PASS |
| 20 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS |
| 21 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | PASS |

## Operator Reference

| Operator | LaTeX Output | Precedence | Support |
|----------|--------------|------------|---------|
| `+` | ` + ` | Low | Supported |
| `-` | ` - ` | Low | Supported |
| `*` | ` \times ` | High | Supported |
| `/` | ` \div ` | High | Supported |
| `^` | - | - | NOT Supported |

## Parenthesization Rules

1. **Multiplication/Division > Addition/Subtraction**: Add parentheses when lower precedence is operand of higher
2. **Same Precedence**: Left-to-right, no parentheses
3. **High at Top Level**: No parentheses when multiplication/division operators at top level

Examples:
- `5 3 + 2 *` → `$( 5 + 3 ) \times 2$` (parentheses around lower precedence)
- `5 3 * 2 +` → `$5 \times 3 + 2$` (no parentheses, natural precedence)
- `10 2 / 5 *` → `$10 \div 2 \times 5$` (same precedence, left-to-right)

## Error Format

All errors are LexerError with format:
```
Line <N>, column <N>: Unexpected character '<char>'
```

Example:
```
Line 1, column 5: Unexpected character '^'
```

## Implementation Notes

- **Source**: Python at `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source`
- **Pipeline**: Lexer → Parser → LaTeXGenerator
- **Input**: Space-separated RPN tokens
- **Output**: LaTeX math mode (wrapped in `$ ... $`)
- **Floating Point**: Preserved exactly as input
- **Evaluation**: No computation, only syntactic transformation

## Critical Implementation Points for Migrations

1. Use exact LaTeX symbols: `\times` for *, `\div` for /
2. Maintain space around all operators: `$5 + 3$` not `$5+3$`
3. Parentheses format: `( ` and ` )` with spaces
4. Reject `^` with LexerError including line/column
5. Preserve all digits in floating-point numbers
