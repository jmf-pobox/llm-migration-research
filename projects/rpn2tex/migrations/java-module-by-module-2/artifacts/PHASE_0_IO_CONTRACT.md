# I/O Contract for rpn2tex Migration

## Overview
This document captures the exact input/output behavior of the Python rpn2tex implementation. It serves as the baseline for validating behavioral equivalence of Java, Go, and Rust migrations.

Generated from: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source`

## Test Cases

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 +` | `$5 + 3$` | Basic addition |
| `5 3 -` | `$5 - 3$` | Basic subtraction |
| `4 7 *` | `$4 \times 7$` | Basic multiplication with \times |
| `10 2 /` | `$10 \div 2$` | Basic division with \div |
| `2 3 ^` | ERROR: Line 1, column 5: Unexpected character '^' | Exponentiation not supported |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Parentheses added for operator precedence |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | No parentheses when precedence is natural |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | Left-to-right evaluation for same precedence |
| `5 3 - 2 -` | `$5 - 3 - 2$` | Multiple subtractions |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Multiple divisions |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Multiple additions |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Multiplication has higher precedence |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Parentheses for lower precedence operation |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Parentheses around addition in multiplication |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | Multiplication before addition |
| `2 3 ^ 4 *` | ERROR: Line 1, column 5: Unexpected character '^' | Exponentiation not supported |
| `2 3 4 ^ ^` | ERROR: Line 1, column 7: Unexpected character '^' | Exponentiation not supported |
| `3.14 2 *` | `$3.14 \times 2$` | Floating point numbers supported |
| `1.5 0.5 +` | `$1.5 + 0.5$` | Floating point addition |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Multiple complex subexpressions |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Mixed operations with precedence |

## Error Cases

### Unsupported Operators
The Python implementation does NOT support the exponentiation operator `^`. Inputs containing this operator will fail at the lexer stage:
- **Error Type**: `LexerError`
- **Error Pattern**: `Line <line>, column <column>: Unexpected character '^'`
- **Affected Tests**:
  - `2 3 ^`
  - `2 3 ^ 4 *`
  - `2 3 4 ^ ^`

### Valid Operators
- `+` Addition
- `-` Subtraction
- `*` Multiplication (renders as `\times` in LaTeX)
- `/` Division (renders as `\div` in LaTeX)

## LaTeX Output Format

### Operator Mapping
- Addition `+`: Rendered as ` + ` in LaTeX
- Subtraction `-`: Rendered as ` - ` in LaTeX
- Multiplication `*`: Rendered as ` \times ` in LaTeX
- Division `/`: Rendered as ` \div ` in LaTeX

### Parentheses Rules
Parentheses are automatically added based on operator precedence:
1. **Multiplication and Division** have higher precedence than Addition and Subtraction
2. **Parentheses are added** when a lower-precedence operation appears as an operand of a higher-precedence operation
3. **No parentheses** when operations have the same precedence (natural left-to-right reading)

### Output Format
All outputs are wrapped in LaTeX math mode delimiters: `$ ... $`

## Implementation Details

### Python Entry Point
- **Module**: `rpn2tex.cli`
- **Function**: `main()`
- **Stdin Input**: Use input argument as `-`

### Processing Pipeline
1. **Lexer** (`rpn2tex.lexer.Lexer`): Tokenizes input string
2. **Parser** (`rpn2tex.parser.Parser`): Builds AST from tokens
3. **Generator** (`rpn2tex.latex_gen.LaTeXGenerator`): Generates LaTeX from AST

### Number Format Support
- Integer numbers: `5`, `10`, `100`
- Floating-point numbers: `3.14`, `1.5`, `0.5`

## Behavioral Notes

1. **Operator Precedence**: Multiplication and division have higher precedence than addition and subtraction
2. **Associativity**: All operators are left-associative
3. **RPN Evaluation**: Proper RPN evaluation respects mathematical precedence in output
4. **Whitespace**: Tokens are separated by spaces in input
5. **Error Handling**: Any lexer or parser error terminates processing and outputs to stderr
