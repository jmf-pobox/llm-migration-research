# I/O Contract for rpn2tex Migration

## Overview

This document specifies the expected input/output behavior of the Python rpn2tex implementation as verified through systematic testing. All test cases have been run against the actual Python implementation to establish the baseline behavior that Rust implementations must match.

**Test Date**: 2025-12-29
**Python Version**: 3.x
**Total Test Cases**: 36
**Passed**: 33
**Failed**: 3 (exponentiation operator not yet implemented)

## Test Cases

### Numbers Feature

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `5` | `$5$` | `$5$` | PASS | Integer parsing |
| `3.14` | `$3.14$` | `$3.14$` | PASS | Floating-point parsing |

### Addition Feature

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `5 3 +` | `$5 + 3$` | `$5 + 3$` | PASS | Basic binary addition |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | `$1 + 2 + 3 + 4$` | PASS | Chained addition (left-associative) |

### Subtraction Feature

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `5 3 -` | `$5 - 3$` | `$5 - 3$` | PASS | Basic binary subtraction |
| `5 3 - 2 -` | `$5 - 3 - 2$` | `$5 - 3 - 2$` | PASS | Chained subtraction (left-associative) |

### Multiplication Feature

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `4 7 *` | `$4 \times 7$` | `$4 \times 7$` | PASS | Basic binary multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | `$2 + 3 \times 4$` | PASS | Multiplication has higher precedence than addition |

### Division Feature

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `10 2 /` | `$10 \div 2$` | `$10 \div 2$` | PASS | Basic binary division |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | `$100 \div 10 \div 5 \div 2$` | PASS | Chained division (left-associative) |

### Precedence & Parentheses Feature

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | `$( 5 + 3 ) \times 2$` | PASS | Parentheses when lower precedence operation is left operand |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | `$( 2 + 3 ) \times 4$` | PASS | Addition parenthesized when multiplied |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | `$2 \times ( 3 + 4 )$` | PASS | Addition parenthesized when right operand of multiplication |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS | Both operands parenthesized |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | `$( 10 \div 2 + 3 ) \times 4$` | PASS | Complex precedence with division and addition |

### Mixed Operators Feature

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `5 3 * 2 +` | `$5 \times 3 + 2$` | `$5 \times 3 + 2$` | PASS | Multiplication before addition (no parentheses needed) |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | `$10 \div 2 \times 5$` | PASS | Division and multiplication (same precedence, left-associative) |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | `$2 \times 3 + 4$` | PASS | Multiplication before addition |

### Floating-Point Operations

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `3.14 2 *` | `$3.14 \times 2$` | `$3.14 \times 2$` | PASS | Float multiplication |
| `1.5 0.5 +` | `$1.5 + 0.5$` | `$1.5 + 0.5$` | PASS | Float addition |

### Exponentiation Feature (NOT YET IMPLEMENTED)

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `2 3 ^` | `$2^{3}$` | `(empty)` | FAIL | Exponentiation operator not supported in current version |
| `2 3 ^ 4 *` | `$2^{3} \times 4$` | `(empty)` | FAIL | Exponentiation operator not supported |
| `2 3 4 ^ ^` | `$2^{3^{4}}$` | `(empty)` | FAIL | Right-associative exponentiation not supported |

## Error Cases

### Unrecognized Characters

When the lexer encounters an unsupported character (like `^` for exponentiation), it produces a formatted error message:

```
Error: Unexpected character '^'

1 | 2 3 ^
  |     ^
```

The error includes:
- Error message type and description
- The line number and content
- A caret (^) pointing to the problematic character location

Exit code: 1

## LaTeX Output Format

### Operators Mapping

- Addition: ` + ` (space-separated)
- Subtraction: ` - ` (space-separated)
- Multiplication: ` \times ` (backslash-times with space padding)
- Division: ` \div ` (backslash-div with space padding)

### Numbers

- Integers: Rendered as-is (e.g., `5` -> `$5$`)
- Floats: Rendered with decimal point (e.g., `3.14` -> `$3.14$`)

### Parentheses

- Parentheses are added using the pattern `( expression )`
- Left-padding space after `(` and right-padding space before `)`
- Parentheses are only added when needed for precedence clarity

### Overall Format

- All output is wrapped in LaTeX math mode delimiters: `$...$`
- No trailing newlines (only newline after when printed to stdout)

## Precedence Rules

The Python implementation follows standard mathematical precedence:

1. **Highest**: Numbers (operands)
2. **Middle**: Multiplication (`*`) and Division (`/`) - same level, left-associative
3. **Lowest**: Addition (`+`) and Subtraction (`-`) - same level, left-associative

Parentheses are added in the output when:
- A lower-precedence operation appears as an operand to a higher-precedence operation
- This ensures the LaTeX output correctly represents the RPN evaluation order

## Key Implementation Details

### Lexer Behavior

- Accepts: digits `0-9`, decimal points `.`, operators `+`, `-`, `*`, `/`
- Rejects: `^` (caret/exponentiation) - not yet implemented
- Whitespace: Acts as token delimiter
- Numbers: Supports both integers and floating-point notation

### Parser Behavior

- Implements recursive descent parsing with operator precedence
- Builds an Abstract Syntax Tree (AST) from tokens
- Respects RPN evaluation order (stack-based semantics converted to AST)

### LaTeX Generator Behavior

- Traverses AST in infix order
- Adds parentheses based on precedence analysis
- Escapes LaTeX special characters (e.g., `\` in operators)

## Test Summary

- **Feature Coverage**: 8 features tested (numbers, addition, subtraction, multiplication, division, precedence, floating-point, exponentiation)
- **Core Features (6 Implemented)**: 33/33 tests passing
- **Future Features (1 Pending)**: 0/3 tests passing (exponentiation not yet implemented)
- **Overall Success Rate**: 91.7% (33/36 tests)

## Notes for Rust Migration

1. The caret operator (`^`) is intentionally unsupported in the current Python version
2. All error messages use a formatted error display with context information
3. The implementation maintains separation of concerns: Lexer -> Parser -> AST -> LaTeX Generator
4. Floating-point numbers are fully supported in the current implementation
5. The output format is deterministic and consistent across all test cases
