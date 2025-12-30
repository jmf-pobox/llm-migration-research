# I/O Contract for rpn2tex Migration

## Overview
This document defines the input/output contract for the rpn2tex implementation. All test cases have been executed against the Python reference implementation to establish the expected behavior.

**Test Date:** 2025-12-29
**Implementation Tested:** Python (source/)
**Total Test Cases:** 15
**Result:** All tests PASSED

---

## Test Cases

### Numbers

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5` | `$5$` | PASS | Integer literal |
| `3.14` | `$3.14$` | PASS | Decimal literal |

### Addition

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 +` | `$5 + 3$` | PASS | Basic addition |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | PASS | Multiple additions (left-associative) |

### Subtraction

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 -` | `$5 - 3$` | PASS | Basic subtraction |
| `5 3 - 2 -` | `$5 - 3 - 2$` | PASS | Multiple subtractions (left-associative) |

### Multiplication

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `4 7 *` | `$4 \times 7$` | PASS | Basic multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | PASS | Multiplication has higher precedence than addition |

### Division

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `10 2 /` | `$10 \div 2$` | PASS | Basic division |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | PASS | Multiple divisions (left-associative) |

### Operator Precedence

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | PASS | Parentheses inserted for lower precedence in RPN |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | PASS | Addition requires parentheses when multiplied |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | PASS | Right operand addition requires parentheses |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS | Both operands have lower precedence operations |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | PASS | Complex expression with division, addition, and multiplication |

---

## Error Cases

No error cases were tested in this phase. The implementation successfully handles:
- Integer and floating-point numbers
- All four basic operators: +, -, *, /
- Correct operator precedence
- Proper parenthesization of sub-expressions

---

## LaTeX Output Format

All outputs are wrapped in math mode delimiters (`$...$`):
- Addition: `+` (plus sign)
- Subtraction: `-` (minus sign)
- Multiplication: `\times` (times symbol)
- Division: `\div` (division symbol)
- Parentheses: `( ... )` (with spaces around the expression)

---

## Implementation Details

### Reference Implementation
- **Language:** Python
- **Location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
- **Entry Point:** `rpn2tex.cli` module

### Test Execution Method
Each test case was executed using:
```bash
echo "<input>" | python3 -m rpn2tex.cli -
```

### Validation Criteria
- Output must match exactly (including whitespace and special characters)
- All mathematical operators must use correct LaTeX symbols
- Parentheses must be inserted correctly based on operator precedence
- Floating-point numbers must be preserved in output

---

## Summary

All 15 test cases have been verified against the Python reference implementation. The outputs establish the expected behavior for:
1. Basic number parsing (integers and decimals)
2. Arithmetic operations (addition, subtraction, multiplication, division)
3. Correct operator precedence handling
4. Proper parenthesization in LaTeX output

These test cases form the baseline contract for migration validation to other languages (Java, Go, Rust, etc.).
