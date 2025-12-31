# I/O Contract for rpn2tex Migration

## Overview

This document specifies the exact input/output behavior of the Python `rpn2tex` implementation. These test cases serve as the ground truth (golden reference) against which all migrated implementations must be validated.

**Verification Date**: 2025-12-30
**Python Implementation Status**: All basic operators verified and passing

## Test Cases

### Basic Operations

| Input | Expected Output | Category | Notes |
|-------|-----------------|----------|-------|
| `5 3 +` | `$5 + 3$` | Addition | Basic binary addition |
| `5 3 -` | `$5 - 3$` | Subtraction | Basic binary subtraction |
| `4 7 *` | `$4 \times 7$` | Multiplication | Basic multiplication with LaTeX times symbol |
| `10 2 /` | `$10 \div 2$` | Division | Basic division with LaTeX div symbol |
| `3.14 2 *` | `$3.14 \times 2$` | Multiplication | Floating-point operands |
| `1.5 0.5 +` | `$1.5 + 0.5$` | Addition | Floating-point addition |

### Left-Associative Operations (Chaining)

| Input | Expected Output | Category | Notes |
|-------|-----------------|----------|-------|
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Chained Addition | Multiple additions (left-associative) |
| `5 3 - 2 -` | `$5 - 3 - 2$` | Chained Subtraction | Multiple subtractions: 5 - 3 - 2 = -1 (left-associative) |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Chained Division | Multiple divisions (left-associative) |

### Operator Precedence (Parenthesization)

| Input | Expected Output | Category | Notes |
|-------|-----------------|----------|-------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Precedence | Addition lower than multiplication - needs parentheses |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Precedence | Addition lower than multiplication - needs parentheses |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Precedence | Addition on right side - needs parentheses |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Precedence | Multiplication higher than addition - no parentheses needed |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | Precedence | Multiplication higher than addition - no parentheses needed |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | Precedence | Multiplication higher than addition - no parentheses needed |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | Precedence | Division and multiplication same precedence - no parens |

### Complex Expressions

| Input | Expected Output | Category | Notes |
|-------|-----------------|----------|-------|
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Complex | Product of two sums |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex | Division and addition in left operand, then multiply |

## Unsupported Features

The following features are **not yet implemented** in the current Python version:

| Input | Issue | Notes |
|-------|-------|-------|
| `2 3 ^` | Exponent operator not supported | Character '^' not recognized by lexer |
| `2 3 ^ 4 *` | Exponent operator not supported | Character '^' not recognized by lexer |
| `2 3 4 ^ ^` | Exponent operator not supported | Character '^' not recognized by lexer |

## Implementation Details

### Operator Mapping

| Operator | Input Token | Output Symbol | LaTeX Command |
|----------|------------|---------------|---------------|
| Addition | `+` | `+` | `+` |
| Subtraction | `-` | `-` | `-` |
| Multiplication | `*` | `\times` | `\times` |
| Division | `/` | `\div` | `\div` |

### Precedence Levels

| Operator | Precedence Level | Associativity |
|----------|------------------|----------------|
| `+` | 1 (lower) | Left |
| `-` | 1 (lower) | Left |
| `*` | 2 (higher) | Left |
| `/` | 2 (higher) | Left |

### Parenthesization Rules

The LaTeX generator inserts parentheses based on operator precedence:

1. **Lower precedence always needs parentheses**: A sub-expression with lower precedence than its parent always gets parentheses.
   - Example: `5 3 + 2 *` → `( 5 + 3 ) \times 2`

2. **Equal precedence on right side needs parentheses for non-commutative operators**: For operators like `-` and `/` which are not commutative, equal precedence on the right side requires parentheses.
   - Example: `5 3 - 2 -` does NOT add parens because subtraction is left-associative

3. **No parentheses for higher precedence**: Higher precedence operations naturally bind tighter and don't need explicit parentheses.
   - Example: `2 3 4 * +` → `2 + 3 \times 4`

## Testing Results

### Test Summary
- Total test cases: 21
- Passed: 18
- Failed: 0
- Unsupported: 3 (exponent operator)
- Success rate: 85.7% (excluding unsupported)

### Verified Test Categories
- Basic single operators: 6/6 passing
- Chained operations: 3/3 passing
- Operator precedence: 7/7 passing
- Complex expressions: 2/2 passing
- Exponent operator: 3/3 unsupported (expected)

## Output Format

All outputs follow this format:
- **Wrapping**: LaTeX math mode delimiters `$...$`
- **Spacing**: Single space around operators
- **Parentheses**: Spaces inside parentheses: `( expr )`
- **Numbers**: Preserved exactly as input (e.g., `3.14` stays `3.14`)

## Error Handling

The implementation includes proper error handling for:

1. **Lexer errors**: Invalid characters or malformed tokens
2. **Parser errors**: Invalid RPN (e.g., insufficient operands, extra operands)

These error conditions are documented in the error handling specifications (see `errors.py`).

## Notes for Migration

When migrating to other languages (Rust, Go, Java, etc.):

1. Ensure parenthesization logic exactly matches the precedence rules
2. Use appropriate LaTeX symbols for operators in your target language
3. Preserve spacing rules (single space around operators and after commas in parentheses)
4. Test all 18 supported test cases to ensure compatibility
5. Document any differences in error messages or unsupported features
6. The exponent operator (`^`) is a future enhancement not required for Phase 0

## File Locations

- Python source: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
- This contract: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/rust-feature-by-feature-2/artifacts/PHASE_0_IO_CONTRACT.md`
- Verification script: `/Users/jfreeman/Coding/rpn2tex-rust-migration/verify_io_contract_extended.py`
