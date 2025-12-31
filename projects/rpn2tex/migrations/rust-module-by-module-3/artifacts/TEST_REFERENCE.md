# Quick Reference: rpn2tex I/O Test Cases

## Simple Operations
| Input | Output |
|-------|--------|
| `5 3 +` | `$5 + 3$` |
| `5 3 -` | `$5 - 3$` |
| `4 7 *` | `$4 \times 7$` |
| `10 2 /` | `$10 \div 2$` |

## Single Value
| Input | Output |
|-------|--------|
| `5` | `$5$` |

## Floating Point
| Input | Output |
|-------|--------|
| `3.14 2 *` | `$3.14 \times 2$` |
| `1.5 0.5 +` | `$1.5 + 0.5$` |

## Precedence Examples
| Input | Output | Explanation |
|-------|--------|-------------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Addition first, then multiply |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | Multiplication higher precedence |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Mult before add |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Parentheses added for order |

## Chained Operations
| Input | Output |
|-------|--------|
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` |

## Complex Expressions
| Input | Output |
|-------|--------|
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` |

## Error Cases

### Empty Input
- Input: `` (empty)
- Exit Code: 1
- Error: `Error: Empty expression`

### Missing Operator
- Input: `5 3` (two numbers, no operator)
- Exit Code: 1
- Error: `Error: Invalid RPN: 2 values remain on stack (missing operators?)`

### Too Few Operands
- Input: `5 3 + +` (plus needs two operands)
- Exit Code: 1
- Error: `Error: Operator '+' requires two operands`

### Unsupported Characters
- Input: `2 3 ^` (exponentiation not supported)
- Exit Code: 1
- Error: `Error: Unexpected character '^'`

- Input: `invalid`
- Exit Code: 1
- Error: `Error: Unexpected character 'i'`

- Input: `5 @ 3`
- Exit Code: 1
- Error: `Error: Unexpected character '@'`

## LaTeX Formatting Rules

1. **Math Mode**: All outputs wrapped in `$...$`
2. **Operators**:
   - `+` becomes ` + ` (spaces)
   - `-` becomes ` - ` (spaces)
   - `*` becomes ` \times ` (with spaces)
   - `/` becomes ` \div ` (with spaces)
3. **Parentheses**: `( expr )` with spaces inside
4. **Numbers**: Rendered as-is (no rounding, no normalization)

## Implementation Notes

- **Parser Type**: Stack-based RPN
- **Supported Operators**: +, -, *, / only (NO exponentiation)
- **Exit Codes**: 0 for success, 1 for errors
- **Numeric Types**: Integers and floats both supported
- **Whitespace**: Acts as token delimiter
