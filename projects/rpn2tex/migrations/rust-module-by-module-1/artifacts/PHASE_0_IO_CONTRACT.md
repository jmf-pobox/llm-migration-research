# I/O Contract for rpn2tex Migration

## Overview

This document captures the exact input/output behavior of the Python rpn2tex implementation. These outputs serve as the reference specification for the Rust migration to ensure behavioral equivalence.

**Generated:** 2025-12-29
**Source:** Python implementation in `projects/rpn2tex/source/`
**Test Method:** Direct execution of lexer → parser → latex_gen pipeline

---

## Test Cases

### Test Case 1
**Input:** `5 3 +`
**Expected Output:** `$5 + 3$`
**Notes:** Basic addition

### Test Case 2
**Input:** `5 3 -`
**Expected Output:** `$5 - 3$`
**Notes:** Basic subtraction

### Test Case 3
**Input:** `4 7 *`
**Expected Output:** `$4 \times 7$`
**Notes:** Basic multiplication with \times operator

### Test Case 4
**Input:** `10 2 /`
**Expected Output:** `$10 \div 2$`
**Notes:** Basic division with \div operator

### Test Case 5
**Input:** `2 3 ^`
**Expected Output:** ERROR: Line 1, column 5: Unexpected character '^'
**Notes:** Exponentiation operator not supported in Python implementation

### Test Case 6
**Input:** `5 3 + 2 *`
**Expected Output:** `$( 5 + 3 ) \times 2$`
**Notes:** Operator precedence - addition wrapped in parentheses before multiplication

### Test Case 7
**Input:** `5 3 * 2 +`
**Expected Output:** `$5 \times 3 + 2$`
**Notes:** Operator precedence - multiplication takes precedence, no parentheses needed

### Test Case 8
**Input:** `10 2 / 5 *`
**Expected Output:** `$10 \div 2 \times 5$`
**Notes:** Left-to-right associativity for same precedence operators

### Test Case 9
**Input:** `5 3 - 2 -`
**Expected Output:** `$5 - 3 - 2$`
**Notes:** Left-to-right associativity for subtraction

### Test Case 10
**Input:** `100 10 / 5 / 2 /`
**Expected Output:** `$100 \div 10 \div 5 \div 2$`
**Notes:** Chain of divisions, left-to-right

### Test Case 11
**Input:** `1 2 + 3 + 4 +`
**Expected Output:** `$1 + 2 + 3 + 4$`
**Notes:** Chain of additions

### Test Case 12
**Input:** `2 3 4 * +`
**Expected Output:** `$2 + 3 \times 4$`
**Notes:** Multiplication has higher precedence than addition

### Test Case 13
**Input:** `2 3 + 4 *`
**Expected Output:** `$( 2 + 3 ) \times 4$`
**Notes:** Lower precedence operation wrapped in parentheses

### Test Case 14
**Input:** `2 3 4 + *`
**Expected Output:** `$2 \times ( 3 + 4 )$`
**Notes:** Right operand addition wrapped in parentheses

### Test Case 15
**Input:** `2 3 * 4 +`
**Expected Output:** `$2 \times 3 + 4$`
**Notes:** No parentheses needed for multiplication then addition

### Test Case 16
**Input:** `2 3 ^ 4 *`
**Expected Output:** ERROR: Line 1, column 5: Unexpected character '^'
**Notes:** Exponentiation not supported

### Test Case 17
**Input:** `2 3 4 ^ ^`
**Expected Output:** ERROR: Line 1, column 7: Unexpected character '^'
**Notes:** Exponentiation not supported

### Test Case 18
**Input:** `3.14 2 *`
**Expected Output:** `$3.14 \times 2$`
**Notes:** Floating point numbers supported

### Test Case 19
**Input:** `1.5 0.5 +`
**Expected Output:** `$1.5 + 0.5$`
**Notes:** Floating point addition

### Test Case 20
**Input:** `1 2 + 3 4 + *`
**Expected Output:** `$( 1 + 2 ) \times ( 3 + 4 )$`
**Notes:** Multiple groups with precedence - both additions wrapped

### Test Case 21
**Input:** `10 2 / 3 + 4 *`
**Expected Output:** `$( 10 \div 2 + 3 ) \times 4$`
**Notes:** Complex precedence - division and addition grouped, then multiplied

---

## Summary

**Total Test Cases:** 21
**Successful Cases:** 18
**Error Cases:** 3

### Error Case Details

| Input | Error Message |
|-------|---------------|
| `2 3 ^` | Line 1, column 5: Unexpected character '^' |
| `2 3 ^ 4 *` | Line 1, column 5: Unexpected character '^' |
| `2 3 4 ^ ^` | Line 1, column 7: Unexpected character '^' |

All error cases are due to the exponentiation operator `^` not being supported in the Python implementation's lexer.

---

## Output Format Specifications

### Successful LaTeX Output
- All outputs are wrapped in dollar signs: `$...$`
- Operators use LaTeX commands: `\times` for multiplication, `\div` for division, `+` for addition, `-` for subtraction
- Parentheses are rendered as literal `(` and `)` characters
- Spacing follows the pattern: ` operator ` (space on both sides)
- Parentheses include spaces: `( expression )`

### Error Output
- Errors are reported with format: `Line {line}, column {column}: {message}`
- The exponentiation operator `^` is not recognized by the lexer

---

## Notes for Rust Migration

1. **Exponentiation Support:** The Python implementation does NOT support the `^` operator. The Rust migration should either:
   - Also reject `^` to maintain behavioral equivalence, OR
   - Add support for `^` as an additional feature

2. **Output Formatting:** The LaTeX generation includes specific spacing and parentheses placement that must be replicated exactly.

3. **Operator Precedence:** The implementation correctly handles:
   - Multiplication and division at higher precedence than addition and subtraction
   - Left-to-right associativity for operators of the same precedence
   - Automatic parenthesization based on precedence rules

4. **Floating Point Support:** Both integer and floating point numbers are supported without special handling.

5. **Error Messages:** Error locations (line and column) must be captured during lexing.
