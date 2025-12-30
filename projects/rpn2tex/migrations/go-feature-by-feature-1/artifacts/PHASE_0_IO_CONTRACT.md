# I/O Contract for rpn2tex Migration

## Verification Summary

All test cases have been executed against the Python reference implementation. This document serves as the authoritative specification for expected outputs during the migration to Go and other languages.

**Verification Date:** 2025-12-29
**Implementation Tested:** Python rpn2tex (source/)
**Total Test Cases:** 21
**Passing Tests:** 18
**Failing Tests:** 3 (exponentiation operator not implemented)

## Test Cases - Basic Operations

### Numbers
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5` | `$5$` | PASS | Single integer |
| `3.14` | `$3.14$` | PASS | Decimal number |

### Addition
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 +` | `$5 + 3$` | PASS | Simple addition |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | PASS | Chained addition |

### Subtraction
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 -` | `$5 - 3$` | PASS | Simple subtraction |
| `5 3 - 2 -` | `$5 - 3 - 2$` | PASS | Chained subtraction |

### Multiplication
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `4 7 *` | `$4 \times 7$` | PASS | Simple multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | PASS | Multiplication has higher precedence |

### Division
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `10 2 /` | `$10 \div 2$` | PASS | Simple division |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | PASS | Chained division |

### Precedence and Parentheses
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | PASS | Addition wrapped in parentheses before multiplication |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | PASS | Addition wrapped before multiplication |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | PASS | Addition wrapped on right operand |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS | Both operands are wrapped expressions |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | PASS | Mixed division and addition in left operand |

### Mixed Operations (Precedence)
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 * 2 +` | `$5 \times 3 + 2$` | PASS | Multiplication evaluated first, no parens needed |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | PASS | Same precedence, left to right |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | PASS | Multiplication before addition |

### Decimal Numbers
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `3.14 2 *` | `$3.14 \times 2$` | PASS | Decimal number in multiplication |
| `1.5 0.5 +` | `$1.5 + 0.5$` | PASS | Decimal numbers in addition |

## Error Cases - Exponentiation Not Supported

The following test cases produce errors because the exponentiation operator (^) is not implemented in the Python reference implementation.

| Input | Error Message | Status | Notes |
|-------|---------------|--------|-------|
| `2 3 ^` | `Error: Unexpected character '^'` | ERROR | Exponentiation operator not supported |
| `2 3 ^ 4 *` | `Error: Unexpected character '^'` | ERROR | Exponentiation operator not supported |
| `2 3 4 ^ ^` | `Error: Unexpected character '^'` | ERROR | Exponentiation operator not supported |

## Lexer Output Format

When the exponentiation operator (^) is encountered during lexing, the error message follows this format:

```
Error: Unexpected character '^'

1 | 2 3 ^
  |     ^
```

The error indicates:
- The line number (1-indexed)
- The line content
- A caret (^) pointing to the problematic character

## I/O Contract Validation Summary

### Arithmetic Operators Supported
- Addition (+) → outputs `+` with space padding
- Subtraction (-) → outputs `-` with space padding
- Multiplication (*) → outputs `\times` with space padding
- Division (/) → outputs `\div` with space padding

### Number Format
- Integers: output as-is (e.g., `5` → `5`)
- Decimals: output as-is (e.g., `3.14` → `3.14`)
- Wrapped in LaTeX math mode: `$...$`

### Parentheses Rules
1. Parentheses are added when a lower-precedence operation is a child of a higher-precedence operation
2. Precedence: Multiplication/Division > Addition/Subtraction
3. Format: `( left ) operator right` or `left operator ( right )`
4. Spaces included: `( `, ` )`, ` operator `

### Output Format
- All output is wrapped in `$...$` (LaTeX inline math mode)
- Binary operations output as: `left operator right`
- Spaces surround operators: ` + `, ` - `, ` \times `, ` \div `
- Parentheses include spaces: `( expression )`

## Migration Checklist

When migrating to Go or other languages, verify:

- [ ] All 18 passing test cases produce identical output
- [ ] Exponentiation operator handling matches (error or implementation)
- [ ] Decimal number handling is identical
- [ ] Precedence rules are correctly implemented
- [ ] LaTeX output format is exactly matched
- [ ] Space placement in output is identical
- [ ] Error messages for unsupported operators match

## Reference Implementation Details

**Source Files:**
- `/projects/rpn2tex/source/tokens.py` - Token definitions
- `/projects/rpn2tex/source/ast_nodes.py` - AST node classes
- `/projects/rpn2tex/source/errors.py` - Error handling
- `/projects/rpn2tex/source/lexer.py` - Tokenization (Lexer class)
- `/projects/rpn2tex/source/parser.py` - Parsing (Parser class)
- `/projects/rpn2tex/source/latex_gen.py` - LaTeX generation (LaTeXGenerator class)
- `/projects/rpn2tex/source/cli.py` - Command-line interface

**Execution Method:**
```bash
echo "INPUT_EXPRESSION" | python3 -m source.cli -
```

## Test Execution Command

To verify this I/O contract against the Python implementation:

```bash
cd /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex
echo "5 3 +" | python3 -m source.cli -
# Output: $5 + 3$
```

---

*This document was generated as part of Phase 0 of the rpn2tex migration process.*
*It serves as the definitive specification for expected I/O behavior across all target implementations.*
