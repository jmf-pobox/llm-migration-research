# I/O Contract for rpn2tex Migration

**Date Generated:** 2025-12-30
**Source Implementation:** Python (source/)
**Test Method:** CLI execution via stdin

## Test Cases

### Numbers

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `5` | `$5$` | `$5$` | PASS | Single integer literal |
| `3.14` | `$3.14$` | `$3.14$` | PASS | Floating-point literal |

### Addition

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `5 3 +` | `$5 + 3$` | `$5 + 3$` | PASS | Simple addition |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | `$1 + 2 + 3 + 4$` | PASS | Chained addition (left-associative) |

### Subtraction

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `5 3 -` | `$5 - 3$` | `$5 - 3$` | PASS | Simple subtraction |
| `5 3 - 2 -` | `$5 - 3 - 2$` | `$5 - 3 - 2$` | PASS | Chained subtraction (left-associative) |

### Multiplication

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `4 7 *` | `$4 \times 7$` | `$4 \times 7$` | PASS | Simple multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | `$2 + 3 \times 4$` | PASS | Addition and multiplication (respects precedence) |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | `$5 \times 3 + 2$` | PASS | Multiplication and addition (respects precedence) |
| `3.14 2 *` | `$3.14 \times 2$` | `$3.14 \times 2$` | PASS | Multiplication with floating-point |

### Division

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `10 2 /` | `$10 \div 2$` | `$10 \div 2$` | PASS | Simple division |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | `$100 \div 10 \div 5 \div 2$` | PASS | Chained division (left-associative) |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | `$10 \div 2 \times 5$` | PASS | Division and multiplication (same precedence, left-to-right) |

### Operator Precedence (Parentheses Required)

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | `$( 5 + 3 ) \times 2$` | PASS | Addition result needs parentheses when multiplied |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | `$( 2 + 3 ) \times 4$` | PASS | Addition result needs parentheses when multiplied |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | `$2 \times ( 3 + 4 )$` | PASS | Addition result needs parentheses in second operand |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS | Both operands are sums needing parentheses |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | `$( 10 \div 2 + 3 ) \times 4$` | PASS | Mixed operators with parentheses |

## Error Cases

### Exponentiation Operator (Not Implemented)

The Python implementation does not support the exponentiation operator (^). This is marked as an exercise feature in the source code.

| Input | Error Message | Status | Notes |
|-------|---------------|--------|-------|
| `2 3 ^` | `Error: Unexpected character '^'` | EXPECTED | Operator not implemented in Python version |
| `2 3 ^ 4 *` | `Error: Unexpected character '^'` | EXPECTED | Operator not implemented in Python version |
| `2 3 4 ^ ^` | `Error: Unexpected character '^'` | EXPECTED | Operator not implemented in Python version |

### Floating-Point Addition

| Input | Expected Output | Actual Output | Status | Notes |
|-------|-----------------|---------------|--------|-------|
| `1.5 0.5 +` | `$1.5 + 0.5$` | `$1.5 + 0.5$` | PASS | Floating-point operands in addition |

## Summary

- **Total Test Cases:** 21
- **Passing:** 18
- **Failing:** 0
- **Errors (Expected):** 3 (exponentiation operator not implemented)
- **Coverage:** 100% of implemented features

### Key Observations

1. **Operator Support:** The Python implementation correctly supports +, -, *, / operators
2. **Precedence Handling:** LaTeX output correctly uses parentheses for operator precedence
3. **Number Support:** Both integers and floating-point numbers are supported
4. **Associativity:** Operations are correctly left-associative
5. **Missing Features:** Exponentiation operator (^) is listed as an exercise feature not implemented in the source

### Output Format

All outputs are wrapped in LaTeX math mode delimiters (`$...$`), enabling direct use in LaTeX documents.

### Notes for Migration

When migrating to other languages (Go, Java, Rust), ensure:
- Exponentiation operator (^) support (if implementing exercise features)
- Same LaTeX output format with proper spacing
- Correct handling of operator precedence
- Support for both integer and floating-point operands
- Proper left-associativity for chained operations
