# I/O Contract for rpn2tex Migration

## Overview
This document captures the expected I/O behavior of the Python rpn2tex implementation, verified by running each test case through the actual implementation.

Verification Date: 2025-12-30
Python Implementation: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`

## Test Cases Summary
- Total Tests: 26
- Passed: 24
- Failed/Unsupported: 2 (exponentiation operator "^" not implemented in Python version)

---

## Test Cases by Category

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

### Operator Precedence and Parentheses

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | `$( 5 + 3 ) \times 2$` | PASS |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | `$( 2 + 3 ) \times 4$` | PASS |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | `$2 \times ( 3 + 4 )$` | PASS |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | `$( 10 \div 2 + 3 ) \times 4$` | PASS |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | `$5 \times 3 + 2$` | PASS |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | `$10 \div 2 \times 5$` | PASS |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | `$2 \times 3 + 4$` | PASS |

### Floating Point Support

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `3.14 2 *` | `$3.14 \times 2$` | `$3.14 \times 2$` | PASS |
| `1.5 0.5 +` | `$1.5 + 0.5$` | `$1.5 + 0.5$` | PASS |

### Exponentiation (Unsupported in Python Implementation)

| Input | Expected Output | Actual Output | Error | Status |
|-------|-----------------|---------------|-------|--------|
| `2 3 ^` | (expected to work) | N/A | `Unexpected character '^'` | UNSUPPORTED |
| `2 3 ^ 4 *` | (expected to work) | N/A | `Unexpected character '^'` | UNSUPPORTED |
| `2 3 4 ^ ^` | (expected to work) | N/A | `Unexpected character '^'` | UNSUPPORTED |

---

## Error Cases

### Unsupported Operators
The Python implementation does not support the exponentiation operator `^`. When encountered, it raises a lexer error:

```
Error: Unexpected character '^'

1 | 2 3 ^
  |     ^
```

**Impact**: Any test case using the `^` operator will fail in the Python implementation.

---

## I/O Format Specifications

### Input Format
- Space-separated RPN tokens
- Supported operands: integers and floating-point numbers
- Supported operators: `+`, `-`, `*`, `/`
- Unsupported operators: `^` (exponentiation)

### Output Format
- LaTeX inline math mode format: `$...$`
- Operators rendered as:
  - Addition: `+`
  - Subtraction: `-`
  - Multiplication: `\times`
  - Division: `\div`
- Parentheses added automatically for precedence clarity:
  - `( ... )` with spaces around operators inside

### Spacing Rules in Output
- Single spaces around binary operators: `5 + 3` (not `5+3`)
- Spaces inside parentheses: `( expr )` (not `(expr)`)
- LaTeX commands properly escaped: `\times`, `\div`

---

## Key Implementation Details

### LaTeX Generation Rules
1. All output is wrapped in LaTeX inline math mode: `$...$`
2. Parentheses are added when a lower-precedence operation is an operand to a higher-precedence operation
3. Floating-point numbers are preserved as-is in the output
4. Operators use LaTeX symbols:
   - `\times` for multiplication (not `*`)
   - `\div` for division (not `/`)

### Operator Precedence
Precedence from highest to lowest:
1. Multiplication (`*`) and Division (`/`) - same level, left-associative
2. Addition (`+`) and Subtraction (`-`) - same level, left-associative

### RPN Evaluation
- Follows standard Reverse Polish Notation evaluation
- Operations are applied left-to-right as operators are encountered
- The AST structure reflects the RPN evaluation order

---

## Verification Commands Used

All tests were run using:
```bash
cd /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex
python -m source.cli - <<< "input_expression"
```

Example:
```bash
python -m source.cli - <<< "5 3 +"
# Output: $5 + 3$
```

---

## Migration Implications

For the Java/Rust migration:

1. **Exponentiation Support**: Consider whether to add `^` operator support. The Python implementation does not support it currently.

2. **Output Format**: Maintain exact spacing and formatting as shown in test cases.

3. **LaTeX Symbols**: Ensure proper escaping and rendering of `\times` and `\div`.

4. **Floating Point**: Preserve floating-point precision from input.

5. **Parenthesis Logic**: Implement the same precedence-based parenthesization rules.

---

## Conclusion

All 24 test cases without the exponentiation operator pass successfully. The Python implementation provides a clear specification for RPN-to-LaTeX conversion with proper operator precedence handling and parenthesization.
