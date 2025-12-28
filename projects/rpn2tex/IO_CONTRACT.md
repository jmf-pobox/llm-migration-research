# I/O Contract for rpn2tex Python Implementation

## Execution Environment
- **Implementation**: Python (source/)
- **CLI Entry Point**: `python projects/rpn2tex/source/cli.py`
- **Date Generated**: 2025-12-28

## Summary
This document verifies the complete I/O contract for the rpn2tex Python implementation by running each test case through the actual CLI and capturing the exact output.

---

## Test Results by Feature

### Feature: Basic Numbers

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| "5" | "$5$" | "$5$" | PASS |
| "3.14" | "$3.14$" | "$3.14$" | PASS |

### Feature: Addition

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| "5 3 +" | "$5 + 3$" | "$5 + 3$" | PASS |
| "1 2 + 3 + 4 +" | "$1 + 2 + 3 + 4$" | "$1 + 2 + 3 + 4$" | PASS |

### Feature: Subtraction

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| "5 3 -" | "$5 - 3$" | "$5 - 3$" | PASS |
| "5 3 - 2 -" | "$5 - 3 - 2$" | "$5 - 3 - 2$" | PASS |

### Feature: Multiplication

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| "4 7 *" | "$4 \times 7$" | "$4 \times 7$" | PASS |
| "2 3 4 * +" | "$2 + 3 \times 4$" | "$2 + 3 \times 4$" | PASS |
| "3.14 2 *" | "$3.14 \times 2$" | "$3.14 \times 2$" | PASS |

### Feature: Division

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| "10 2 /" | "$10 \div 2$" | "$10 \div 2$" | PASS |
| "100 10 / 5 / 2 /" | "$100 \div 10 \div 5 \div 2$" | "$100 \div 10 \div 5 \div 2$" | PASS |

### Feature: Decimal Numbers

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| "1.5 0.5 +" | "$1.5 + 0.5$" | "$1.5 + 0.5$" | PASS |
| "3.14 2 *" | "$3.14 \times 2$" | "$3.14 \times 2$" | PASS |

### Feature: Operator Precedence

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| "5 3 + 2 *" | "$( 5 + 3 ) \times 2$" | "$( 5 + 3 ) \times 2$" | PASS |
| "2 3 + 4 *" | "$( 2 + 3 ) \times 4$" | "$( 2 + 3 ) \times 4$" | PASS |
| "2 3 4 + *" | "$2 \times ( 3 + 4 )$" | "$2 \times ( 3 + 4 )$" | PASS |
| "1 2 + 3 4 + *" | "$( 1 + 2 ) \times ( 3 + 4 )$" | "$( 1 + 2 ) \times ( 3 + 4 )$" | PASS |
| "10 2 / 3 + 4 *" | "$( 10 \div 2 + 3 ) \times 4$" | "$( 10 \div 2 + 3 ) \times 4$" | PASS |

---

## Error Cases

### Unsupported Operators

The current Python implementation does NOT support the caret (^) operator for exponentiation.

| Input | Error | Status |
|-------|-------|--------|
| "2 3 ^" | Error: Unexpected character '^' at Line 1, column 5 | NOT SUPPORTED |
| "2 3 4 ^ ^" | Error: Unexpected character '^' at Line 1, column 7 | NOT SUPPORTED |
| "2 3 ^ 4 *" | Error: Unexpected character '^' at Line 1, column 5 | NOT SUPPORTED |

### Notes on Errors
- The lexer explicitly rejects the caret (^) character
- The token type and operator are not defined in the current implementation
- See `tokens.py` lines 14-16 for exercise notes about implementing CARET
- Implementations wanting to support exponentiation must add CARET token type and implement in lexer/parser/generator

---

## Verification Summary

### Total Test Cases: 21
- **Passing**: 18
- **Failing**: 0
- **Not Supported**: 3 (all exponentiation operator cases)

### Supported Features
- ✅ Single numbers (integers and decimals)
- ✅ Basic binary operators (+, -, *, /)
- ✅ Multi-operand expressions
- ✅ Correct operator precedence with parentheses
- ✅ Decimal number handling
- ✅ Negative numbers (via subtraction)

### Not Supported Features
- ❌ Exponentiation operator (^)
- ❌ Square root function (sqrt)
- ❌ Nth root function (root)

---

## Output Format Standard

All successful results follow this pattern:
```
$<LaTeX expression>$
```

Examples:
- Single number: `$5$`
- Simple operation: `$5 + 3$`
- With precedence: `$( 5 + 3 ) \times 2$`

### LaTeX Operators Used
- Addition: ` + `
- Subtraction: ` - `
- Multiplication: ` \times `
- Division: ` \div `

---

## Implementation Details

### Source Files Location
- **CLI**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/projects/rpn2tex/source/cli.py`
- **Lexer**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/projects/rpn2tex/source/lexer.py`
- **Parser**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/projects/rpn2tex/source/parser.py`
- **LaTeX Generator**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/projects/rpn2tex/source/latex_gen.py`

### How to Run
```bash
# From stdin
echo "5 3 +" | python projects/rpn2tex/source/cli.py -

# From file
python projects/rpn2tex/source/cli.py input.rpn

# With output file
python projects/rpn2tex/source/cli.py input.rpn -o output.tex
```

---

## Remarks

This I/O contract has been verified by running each test case against the actual Python implementation and capturing exact outputs. The implementation is consistent and correct for all supported features. The three test cases involving the caret (^) operator fail because that operator is not implemented, which is noted in the source code as an exercise for future enhancement.

