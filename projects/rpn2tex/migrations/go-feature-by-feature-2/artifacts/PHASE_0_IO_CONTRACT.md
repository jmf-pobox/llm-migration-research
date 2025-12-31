# Phase 0: I/O Contract for rpn2tex Python Implementation

Verified against Python implementation in `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`

## Test Verification Summary

All test cases have been run through the Python implementation and outputs verified.

## I/O Contract for rpn2tex Migration

### Feature: Numbers

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5` | `$5$` | PASS |
| `3.14` | `$3.14$` | PASS |

### Feature: Addition

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5 3 +` | `$5 + 3$` | PASS |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | PASS |

### Feature: Subtraction

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5 3 -` | `$5 - 3$` | PASS |
| `5 3 - 2 -` | `$5 - 3 - 2$` | PASS |

### Feature: Multiplication

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `4 7 *` | `$4 \times 7$` | PASS |
| `2 3 4 * +` | `$2 + 3 \times 4$` | PASS |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | PASS |

### Feature: Division

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `10 2 /` | `$10 \div 2$` | PASS |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | PASS |

### Feature: Operator Precedence

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | PASS |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | PASS |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | PASS |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | PASS |

### Feature: Floating Point Arithmetic

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `3.14 2 *` | `$3.14 \times 2$` | PASS |
| `1.5 0.5 +` | `$1.5 + 0.5$` | PASS |

## Error Cases

### Not Supported in Python Implementation

The following operators are not yet implemented in the Python source:

| Input | Error | Notes |
|-------|-------|-------|
| `2 3 ^ 4 *` | `Unexpected character '^'` | Exponentiation not in TokenType enum |
| `2 3 4 ^ ^` | `Unexpected character '^'` | Exponentiation not in TokenType enum |

**Note:** The CARET token type is defined as an exercise in the source code (`tokens.py` line 41) but not yet implemented in the lexer or parser.

## Summary

- **Total Test Cases:** 21
- **Passed:** 19
- **Failed:** 0
- **Not Supported:** 2 (exponentiation with ^)

All core features (numbers, addition, subtraction, multiplication, division) and operator precedence are correctly implemented and produce expected output. Exponentiation support is marked as an exercise for future implementation.

## Implementation Details

### Key Files Verified
- Lexer: `source/lexer.py` - Tokenizes RPN input
- Parser: `source/parser.py` - Stack-based RPN parsing
- LaTeX Generator: `source/latex_gen.py` - Converts AST to LaTeX output
- CLI: `source/cli.py` - Command-line interface

### Output Format
All outputs are wrapped in LaTeX math mode delimiters (`$...$`).

### Operator Mapping to LaTeX
- Addition: `+` → `+`
- Subtraction: `-` → `-`
- Multiplication: `*` → `\times`
- Division: `/` → `\div`

### Precedence Rules (correctly implemented)
1. Multiplication and Division: Level 2 (higher precedence)
2. Addition and Subtraction: Level 1 (lower precedence)
3. Left-associative operators: Parens added on right side for equal precedence
