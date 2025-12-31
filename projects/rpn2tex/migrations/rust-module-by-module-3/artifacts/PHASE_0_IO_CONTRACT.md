# I/O Contract for rpn2tex Migration

This document specifies the exact input/output behavior of the Python rpn2tex implementation. The Rust implementation must be behaviorally equivalent to pass migration validation.

**Generated:** 2025-12-30
**Source:** Python rpn2tex implementation at `/projects/rpn2tex/source/`

## Supported Features

### Operators
- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`

### Data Types
- Integers: e.g., `5`, `42`, `-3`
- Floating-point: e.g., `3.14`, `1.5`, `0.5`
- Negative numbers: e.g., `-5` (when written with hyphen immediately before digits)

### Not Implemented
- Exponentiation: `^` (not supported by Python lexer)
- Square root: `sqrt` (not supported)
- Nth root: `root` (not supported)

## Test Cases

### Valid Test Cases (Success Cases)

| Input | Expected Output | Exit Code | Notes |
|-------|-----------------|-----------|-------|
| `5 3 +` | `$5 + 3$` | 0 | Simple addition |
| `5 3 -` | `$5 - 3$` | 0 | Simple subtraction |
| `4 7 *` | `$4 \times 7$` | 0 | Simple multiplication |
| `10 2 /` | `$10 \div 2$` | 0 | Simple division |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | 0 | Operator precedence: (5+3)*2 |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | 0 | Operator precedence: 5*3+2 |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | 0 | Left-to-right: (10/2)*5 |
| `5 3 - 2 -` | `$5 - 3 - 2$` | 0 | Left-to-right: (5-3)-2 |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | 0 | Chained division |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | 0 | Chained addition |
| `2 3 4 * +` | `$2 + 3 \times 4$` | 0 | Precedence: 2+(3*4) |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | 0 | Explicit grouping via RPN |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | 0 | Grouping on right operand |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | 0 | Multiplication then addition |
| `3.14 2 *` | `$3.14 \times 2$` | 0 | Floating-point multiplication |
| `1.5 0.5 +` | `$1.5 + 0.5$` | 0 | Floating-point addition |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | 0 | Multiple subexpressions |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | 0 | Complex expression |
| `5` | `$5$` | 0 | Single number (no operation) |

### Error Cases

| Input | Expected Output | Exit Code | Error Message | Notes |
|-------|-----------------|-----------|---------------|-------|
| `` (empty) | (empty) | 1 | `Error: Empty expression` | Empty input |
| `5 3` | (empty) | 1 | `Error: Invalid RPN: 2 values remain on stack (missing operators?)` | Missing operator |
| `5 3 + +` | (empty) | 1 | `Error: Operator '+' requires two operands` | Insufficient operands for operator |
| `2 3 ^` | (empty) | 1 | `Error: Unexpected character '^'` | Unsupported operator (exponentiation) |
| `2 3 ^ 4 *` | (empty) | 1 | `Error: Unexpected character '^'` | Unsupported operator in expression |
| `2 3 4 ^ ^` | (empty) | 1 | `Error: Unexpected character '^'` | Multiple unsupported operators |
| `invalid` | (empty) | 1 | `Error: Unexpected character 'i'` | Unrecognized token |
| `5 @ 3` | (empty) | 1 | `Error: Unexpected character '@'` | Invalid character |

## LaTeX Output Format Specification

### Operator Representation
- **Addition**: ` + ` (space-padded)
- **Subtraction**: ` - ` (space-padded)
- **Multiplication**: ` \times ` (with spaces)
- **Division**: ` \div ` (with spaces)

### Math Mode Delimiters
- All outputs are wrapped in `$...$` (inline math mode)

### Parentheses Handling
- Parentheses are added when needed to preserve operator precedence
- Format: `( expr )` (spaces inside parentheses)
- Example: `$( 5 + 3 ) \times 2$`

### Numeric Literals
- Integers: rendered as-is (e.g., `5`)
- Floats: rendered as-is (e.g., `3.14`)
- Negative numbers: rendered with hyphen (e.g., `-5`)
- No scientific notation observed in test cases

## Implementation Details

### Parser Stack-Based Algorithm
1. Number tokens are pushed onto the stack
2. Operator tokens pop two operands (right then left)
3. A binary operation node is created and pushed back
4. At EOF, exactly one item should remain on stack

### Error Handling Priority
1. **Lexer errors** (invalid characters) - caught first
2. **Parser errors** (invalid RPN structure) - caught second
3. Error messages include line and column information

### Whitespace Handling
- Whitespace acts as token delimiter
- Multiple spaces are treated as single delimiter
- Newlines are accepted as delimiters
- No leading/trailing whitespace issues observed

## Behavioral Notes for Rust Implementation

1. **Exact Output Matching Required**: The Rust implementation must produce identical LaTeX output, including:
   - Exact spacing around operators
   - Exact parenthesization rules
   - Exact case for LaTeX commands (\times, \div)

2. **Error Messages Must Match**: Error messages should be identical in wording (though the formatter may differ)

3. **Exit Codes**: Must return 0 for success, 1 for errors

4. **No Exponentiation**: The Rust version should NOT implement `^` operator unless explicitly requested

5. **Numeric Precision**: Floating-point numbers are passed through as-is without rounding

6. **Negative Number Handling**: `-5 3 +` should work (negative first operand)

## Testing Checklist for Rust Implementation

- [ ] All 18 valid test cases produce exact LaTeX output
- [ ] Single number input (e.g., `5`) produces `$5$`
- [ ] Empty input produces error message
- [ ] Missing operators produce error message
- [ ] Insufficient operands produce error message
- [ ] Invalid characters produce error message
- [ ] Floating-point operations work correctly
- [ ] Operator precedence is preserved correctly
- [ ] Exit codes are correct (0 for success, 1 for error)
- [ ] Multiple whitespace between tokens is handled correctly
