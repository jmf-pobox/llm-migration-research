# Quick Reference: I/O Contract for rpn2tex

## All Test Cases at a Glance

### Numbers (2 tests)
| Input | Output |
|-------|--------|
| `5` | `$5$` |
| `3.14` | `$3.14$` |

### Addition (2 tests)
| Input | Output |
|-------|--------|
| `5 3 +` | `$5 + 3$` |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` |

### Subtraction (2 tests)
| Input | Output |
|-------|--------|
| `5 3 -` | `$5 - 3$` |
| `5 3 - 2 -` | `$5 - 3 - 2$` |

### Multiplication (4 tests)
| Input | Output |
|-------|--------|
| `4 7 *` | `$4 \times 7$` |
| `2 3 4 * +` | `$2 + 3 \times 4$` |
| `5 3 * 2 +` | `$5 \times 3 + 2$` |
| `2 3 * 4 +` | `$2 \times 3 + 4$` |

### Division (3 tests)
| Input | Output |
|-------|--------|
| `10 2 /` | `$10 \div 2$` |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` |

### Precedence (5 tests)
| Input | Output |
|-------|--------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` |

### Decimals (2 tests)
| Input | Output |
|-------|--------|
| `3.14 2 *` | `$3.14 \times 2$` |
| `1.5 0.5 +` | `$1.5 + 0.5$` |

### Error Cases (3 tests - exponentiation not supported)
| Input | Error |
|-------|-------|
| `2 3 ^` | `Error: Unexpected character '^'` |
| `2 3 ^ 4 *` | `Error: Unexpected character '^'` |
| `2 3 4 ^ ^` | `Error: Unexpected character '^'` |

## Critical Implementation Rules

### 1. Operator Symbols
```
+ → " + "
- → " - "
* → " \times "
/ → " \div "
```

### 2. Precedence
```
HIGHEST: * and /
LOWEST:  + and -
```

### 3. Parentheses Rules
```
Wrap lower-precedence when it's child of higher-precedence:

5 3 + 2 *  →  ( 5 + 3 ) * 2        (left operand wrapped)
2 3 4 + *  →  2 * ( 3 + 4 )        (right operand wrapped)
2 3 + 4 *  →  ( 2 + 3 ) * 4        (left operand wrapped)
```

### 4. Output Format
```
Every result: $expression$
```

## Test Verification Command

```bash
cd /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex
echo "INPUT" | python3 -m source.cli -
```

## Implementation Modules (Reference)

```
source/
├── tokens.py           # Token definitions
├── lexer.py            # Tokenization (Lexer class)
├── parser.py           # Parsing (Parser class)
├── ast_nodes.py        # AST nodes
├── latex_gen.py        # LaTeX generation (LaTeXGenerator class)
├── errors.py           # Error handling
└── cli.py              # CLI interface
```

---

**Total: 20 passing tests, 3 expected error cases**
**All verified against Python implementation**
