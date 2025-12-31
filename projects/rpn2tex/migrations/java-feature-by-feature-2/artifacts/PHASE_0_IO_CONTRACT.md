# I/O Contract for rpn2tex Migration

## Test Execution Summary

Date: 2025-12-30
Implementation: Python (`/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`)
Test Command: `echo "<input>" | python -m source.cli -`
Total Tests: 21
Passed: 18
Failed: 3 (exponentiation operator not supported)

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
| `5 3 * 2 +` | `$5 \times 3 + 2$` | `$5 \times 3 + 2$` | PASS |

### Division

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `10 2 /` | `$10 \div 2$` | `$10 \div 2$` | PASS |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | `$100 \div 10 \div 5 \div 2$` | PASS |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | `$10 \div 2 \times 5$` | PASS |

### Exponentiation (Not Supported)

| Input | Expected Output | Actual Output | Status | Error |
|-------|-----------------|---------------|--------|-------|
| `2 3 ^` | `$2 ^ {3}$` | (empty) | FAIL | Unexpected character '^' |
| `2 3 ^ 4 *` | `$( 2 ^ {3} ) \times 4$` | (empty) | FAIL | Unexpected character '^' |
| `2 3 4 ^ ^` | `$2 ^ {3 ^ {4}}$` | (empty) | FAIL | Unexpected character '^' |

### Operator Precedence

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | `$( 5 + 3 ) \times 2$` | PASS |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | `$( 2 + 3 ) \times 4$` | PASS |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | `$2 \times ( 3 + 4 )$` | PASS |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | `$2 \times 3 + 4$` | PASS |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | `$( 10 \div 2 + 3 ) \times 4$` | PASS |

### Floating Point Numbers

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `3.14 2 *` | `$3.14 \times 2$` | `$3.14 \times 2$` | PASS |
| `1.5 0.5 +` | `$1.5 + 0.5$` | `$1.5 + 0.5$` | PASS |

## Implementation Notes

### Supported Operators

- **Addition (+)**: Properly generates `+` in LaTeX
- **Subtraction (-)**: Properly generates `-` in LaTeX
- **Multiplication (*)**: Properly generates `\times` in LaTeX
- **Division (/)**: Properly generates `\div` in LaTeX
- **Exponentiation (^)**: NOT SUPPORTED - Lexer rejects `^` as unexpected character

### Supported Operands

- Integers (e.g., `5`, `3`)
- Floating-point numbers (e.g., `3.14`, `1.5`)

### LaTeX Generation Rules

1. **Basic operators**: Infix notation with proper spacing
2. **Parentheses**: Automatically added when needed based on operator precedence
   - Addition/subtraction have lower precedence than multiplication/division
   - Parentheses are added to preserve correct evaluation order
3. **Output format**: Always wrapped in math mode delimiters `$...$`

## Migration Considerations

### For Java Implementation

1. All supported test cases in Python should produce identical LaTeX output in Java
2. The exponentiation operator (^) is NOT currently implemented - migrations should NOT include this feature in Phase 0
3. Floating-point number support must be maintained
4. Parenthesization rules follow standard mathematical precedence and must be exactly matched
5. LaTeX escape sequences (particularly `\times` and `\div`) must be handled correctly in Java

### Error Handling

The Python implementation produces detailed error messages:
- Lexer errors report unexpected characters with line/column information
- Parser errors report token problems
- Format: "Error: <message>\n<source line>\n<pointer to error>"

### Test Execution Method

To run tests against the Python implementation:

```bash
cd /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex
echo "<rpn_expression>" | python -m source.cli -
```

## Summary

The Python reference implementation successfully handles:
- All basic arithmetic operators (+, -, *, /)
- Integer and floating-point operands
- Complex expressions with correct operator precedence handling
- Proper LaTeX formatting and escaping

The implementation is ready for use as a reference for migrations. Phase 0 should focus on replicating all supported features (not exponentiation).
