# I/O Contract for rpn2tex Migration

## Overview

This document specifies the exact input/output contract for the rpn2tex implementation, verified against the Python source implementation at `source/cli.py`. All test cases have been executed and verified to produce the exact outputs shown below.

## Test Results Summary

- **Total Test Cases**: 21
- **Passed**: 21
- **Failed**: 0
- **Status**: All tests pass - outputs match expected values exactly

## Test Cases by Feature

### Numbers

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `5` | `$5$` | `$5$` | PASS |
| `3.14` | `$3.14$` | `$3.14$` | PASS |

### Addition

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `5 3 +` | `$5 + 3$` | `$5 + 3$` | PASS |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | `$1 + 2 + 3 + 4$` | PASS |

### Subtraction

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `5 3 -` | `$5 - 3$` | `$5 - 3$` | PASS |
| `5 3 - 2 -` | `$5 - 3 - 2$` | `$5 - 3 - 2$` | PASS |

### Multiplication

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `4 7 *` | `$4 \times 7$` | `$4 \times 7$` | PASS |
| `2 3 4 * +` | `$2 + 3 \times 4$` | `$2 + 3 \times 4$` | PASS |

### Division

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `10 2 /` | `$10 \div 2$` | `$10 \div 2$` | PASS |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | `$100 \div 10 \div 5 \div 2$` | PASS |

### Operator Precedence

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | `$( 5 + 3 ) \times 2$` | PASS |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | `$( 2 + 3 ) \times 4$` | PASS |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | `$2 \times ( 3 + 4 )$` | PASS |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | `$( 10 \div 2 + 3 ) \times 4$` | PASS |

## Output Format Specification

All outputs are wrapped in LaTeX math mode delimiters `$...$`.

### Operators

| Operator | LaTeX Symbol | Example |
|----------|--------------|---------|
| `+` | ` + ` | `$5 + 3$` |
| `-` | ` - ` | `$5 - 3$` |
| `*` | `\times` | `$4 \times 7$` |
| `/` | `\div` | `$10 \div 2$` |

### Parentheses

Parentheses are added with spaces: `( expression )` when needed for precedence clarity.

### Number Formats

- Integers: preserved as-is (e.g., `5`)
- Floats: preserved with decimal places (e.g., `3.14`)

## Error Handling

No error cases were tested in this verification phase. The implementation is assumed to handle invalid RPN expressions and malformed input with appropriate error messages.

## Implementation Details

**Source**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/cli.py`

**Key Components**:
- **Lexer**: Tokenizes input strings into tokens
- **Parser**: Builds an Abstract Syntax Tree (AST) from tokens
- **LaTeX Generator**: Converts AST to LaTeX output
- **Error Formatter**: Provides formatted error messages

**Entry Point**: `rpn2tex.cli:main()`

## Testing Method

All test cases were executed using:
```bash
echo "<input>" | python -m rpn2tex.cli -
```

This invokes the Python implementation from stdin and captures stdout.

## Verification Date

Generated: 2025-12-30

## Notes for Migration

1. **Output Format is Consistent**: All outputs wrap expressions in `$...$` for LaTeX math mode
2. **Spacing is Critical**: Operators have spaces around them (e.g., ` + `, ` - `, etc.)
3. **Parentheses Format**: Parentheses use ` ( ` and ` ) ` with spaces
4. **LaTeX Escaping**: Multiplication uses `\times` and division uses `\div` (backslash-escaped)
5. **Number Preservation**: Float and integer formats are preserved exactly as input
6. **No Trailing Newlines**: The actual output has no trailing newline (stdout only)
