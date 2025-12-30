# I/O Contract for rpn2tex Migration

## Overview
This document specifies the expected input/output behavior of the Python rpn2tex implementation. All Go implementations must produce identical LaTeX output for these test cases.

**Implementation:** Python rpn2tex
**Source:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`

---

## Test Cases

### Successful Cases

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 +` | `$5 + 3$` | Basic addition |
| `5 3 -` | `$5 - 3$` | Basic subtraction |
| `4 7 *` | `$4 \times 7$` | Basic multiplication |
| `10 2 /` | `$10 \div 2$` | Basic division |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Precedence: addition then multiply |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | Precedence: multiply then addition |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | Precedence: division and multiply left-to-right |
| `5 3 - 2 -` | `$5 - 3 - 2$` | Chained subtraction left-to-right |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Chained division left-to-right |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Chained addition |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Mixed precedence: multiply before addition |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Parentheses around addition due to multiplication |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Parentheses around addition as right operand of multiply |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | No parentheses needed for multiply before addition |
| `3.14 2 *` | `$3.14 \times 2$` | Floating point numbers |
| `1.5 0.5 +` | `$1.5 + 0.5$` | Floating point addition |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Multiple expressions with precedence |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex precedence: division/add/multiply |

---

## Error Cases

| Input | Exit Code | Error Message | Notes |
|-------|-----------|---------------|-------|
| `2 3 ^` | 1 | `Error: Unexpected character '^'` | Exponentiation operator not supported |
| `2 3 ^ 4 *` | 1 | `Error: Unexpected character '^'` | Exponentiation in complex expression |
| `2 3 4 ^ ^` | 1 | `Error: Unexpected character '^'` | Multiple exponentiation operators |

---

## LaTeX Formatting Notes

1. **Math Mode Delimiters:** All output is wrapped in `$...$` for inline math mode
2. **Operator Symbols:**
   - Addition: `+`
   - Subtraction: `-`
   - Multiplication: `\times`
   - Division: `\div`
3. **Spacing:** Single space around operators (e.g., `$5 + 3$`)
4. **Parentheses:** Added when necessary to preserve correct operator precedence
   - Parentheses use format: `( expr ) operator operand` or `operand operator ( expr )`

---

## Summary Statistics

- Total Test Cases: 21
- Successful Cases: 18 (exit code 0)
- Error Cases: 3 (exit code 1)
- Support for:
  - Basic arithmetic (addition, subtraction, multiplication, division)
  - Floating point numbers
  - Operator precedence and parenthesization
  - Chained operations

---

## Implementation Details for Validation

When validating the Go implementation:
1. All LaTeX output must match exactly, character for character
2. All successful cases must exit with code 0
3. All error cases must exit with code 1
4. Error messages for exponentiation must contain "Unexpected character '^'"
5. Floating point numbers must be preserved as-is (e.g., `3.14`, `1.5`)
