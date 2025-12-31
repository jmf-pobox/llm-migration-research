# I/O Contract for rpn2tex Migration

## Summary

This document specifies the expected input-output behavior of the Python rpn2tex implementation. All test cases have been verified against the actual implementation and are passing (20/20 tests).

## Test Cases

### Numbers
| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5` | `$5$` | Single integer |
| `3.14` | `$3.14$` | Decimal number |

### Addition
| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 +` | `$5 + 3$` | Basic addition |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Chained additions (left-associative) |

### Subtraction
| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 -` | `$5 - 3$` | Basic subtraction |
| `5 3 - 2 -` | `$5 - 3 - 2$` | Chained subtractions (left-associative) |

### Multiplication
| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `4 7 *` | `$4 \times 7$` | Basic multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Multiplication has higher precedence than addition |

### Division
| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `10 2 /` | `$10 \div 2$` | Basic division |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Chained divisions (left-associative) |

### Operator Precedence
| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Parentheses needed: + has lower precedence than * |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Parentheses needed: + has lower precedence than * |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Parentheses needed: + has lower precedence than * |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Both operands of * need parentheses |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex: / same precedence as /, higher than + |

## Error Cases

All error cases produce exit code 1 with appropriate error messages to stderr.

### Missing Operand
| Input | Error Message | Notes |
|-------|---------------|-------|
| `5 +` | `Error: Operator '+' requires two operands` | Not enough values on stack |

### Extra Operand
| Input | Error Message | Notes |
|-------|---------------|-------|
| `5 3 + +` | `Error: Operator '+' requires two operands` | Not enough operands for operator |

### Invalid Character
| Input | Error Message | Notes |
|-------|---------------|-------|
| `5 3 @ +` | `Error: Unexpected character '@'` | Lexer rejects invalid characters |

### Just Operator
| Input | Error Message | Notes |
|-------|---------------|-------|
| `+` | `Error: Operator '+' requires two operands` | Operator with no operands |

### Empty Input
| Input | Error Message | Notes |
|-------|---------------|-------|
| `` | `Error: Empty expression` | Empty input produces error |

## Implementation Details

### Source Files
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/tokens.py` - Token definitions
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/ast_nodes.py` - AST node definitions
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/lexer.py` - Tokenization
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/parser.py` - RPN stack-based parsing
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/latex_gen.py` - LaTeX generation with precedence handling
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/cli.py` - CLI entry point

### Pipeline
```
Input text → Lexer → Tokens → Parser → AST → LaTeX Generator → Output
                      |         |                    |
                  errors.py   errors.py        precedence rules
```

### Operator Precedence (Higher number = tighter binding)
- Addition/Subtraction: precedence level 1
- Multiplication/Division: precedence level 2

### Parenthesization Rules
1. Always add parentheses when child has lower precedence than parent
2. Add parentheses on right side when operators have equal precedence (left-associativity)
3. Special handling for - and / operators (non-commutative)

### LaTeX Output Format
- All output wrapped in `$...$` (math mode delimiters)
- Operators:
  - `+` → `+`
  - `-` → `-`
  - `*` → `\times`
  - `/` → `\div`
- Parentheses formatted as `( expression )`

## Test Verification

All 20 test cases (15 successful cases + 5 error cases) have been verified:

### Execution Method
```bash
echo "input" | python -m source.cli -
```

### Test Results
- **Passed**: 20/20
- **Failed**: 0/20
- **Success Rate**: 100%

### Test Execution Environment
- Location: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex`
- Python Implementation: Source files in `/source/` directory
- CLI Interface: `python -m source.cli -` (stdin mode)

## Notes for Rust Migration

1. **Precedence Handling**: The Rust implementation must replicate the precedence checking logic in `latex_gen.py`'s `_needs_parens` method
2. **Left-Associativity**: Ensure - and / operators are handled as left-associative
3. **LaTeX Output**: Must use exact LaTeX commands: `\times` and `\div`
4. **Error Messages**: Error messages should match the format shown in error cases (with position information)
5. **Empty Input**: Must explicitly handle empty expressions
6. **Stack Validation**: Parser must validate that exactly one value remains on stack after parsing

## Date Generated
2025-12-30

## Verification Status
VERIFIED - All test cases pass against Python implementation
