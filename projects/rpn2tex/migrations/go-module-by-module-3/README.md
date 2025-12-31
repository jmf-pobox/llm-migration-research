# RPN2TeX - Go Implementation

A command-line tool that converts Reverse Polish Notation (RPN) arithmetic expressions to LaTeX math notation.

## Features

- Converts RPN expressions to infix LaTeX notation
- Supports basic arithmetic operators: `+`, `-`, `*`, `/`
- Handles integer and floating-point numbers
- Automatically adds parentheses to preserve operator precedence
- Provides detailed error messages with source context
- Reads from stdin or file

## Installation

### Build from Source

```bash
go build -o rpn2tex cmd/rpn2tex/main.go
```

### Install

```bash
go install ./cmd/rpn2tex
```

## Usage

### Read from stdin

```bash
echo "5 3 +" | ./rpn2tex -
# Output: $5 + 3$
```

### Read from file

```bash
echo "5 3 + 2 *" > input.txt
./rpn2tex input.txt
# Output: $( 5 + 3 ) \times 2$
```

### Help

```bash
./rpn2tex
# Usage: rpn2tex <input-file|->
#   Use '-' to read from stdin
```

## Examples

### Simple Operations

```bash
# Addition
echo "5 3 +" | ./rpn2tex -
# Output: $5 + 3$

# Subtraction
echo "5 3 -" | ./rpn2tex -
# Output: $5 - 3$

# Multiplication
echo "4 7 *" | ./rpn2tex -
# Output: $4 \times 7$

# Division
echo "10 2 /" | ./rpn2tex -
# Output: $10 \div 2$
```

### Complex Expressions

```bash
# Parentheses added for lower precedence
echo "5 3 + 2 *" | ./rpn2tex -
# Output: $( 5 + 3 ) \times 2$

# No parentheses needed (multiplication has higher precedence)
echo "5 3 * 2 +" | ./rpn2tex -
# Output: $5 \times 3 + 2$

# Left-to-right evaluation for same precedence
echo "10 2 / 5 *" | ./rpn2tex -
# Output: $10 \div 2 \times 5$

# Multiple parenthesized subexpressions
echo "1 2 + 3 4 + *" | ./rpn2tex -
# Output: $( 1 + 2 ) \times ( 3 + 4 )$
```

### Floating-Point Numbers

```bash
echo "3.14 2 *" | ./rpn2tex -
# Output: $3.14 \times 2$

echo "1.5 0.5 +" | ./rpn2tex -
# Output: $1.5 + 0.5$
```

### Negative Numbers

```bash
echo "-5 3 +" | ./rpn2tex -
# Output: $-5 + 3$
```

## Error Handling

The tool provides detailed error messages with source context:

```bash
echo "2 3 ^" | ./rpn2tex -
# Error: Unexpected character '^'
#
# 1 | 2 3 ^
#         ^
# (exit code: 1)
```

## Testing

### Run Unit Tests

```bash
go test ./...
```

### Run Tests with Coverage

```bash
go test ./... -cover
```

### Run I/O Contract Tests

```bash
./test_io_contract.sh
```

## Project Structure

```
.
├── cmd/
│   └── rpn2tex/
│       ├── main.go         # CLI entry point
│       └── main_test.go    # CLI tests
├── ast.go                  # AST node definitions
├── ast_test.go             # AST tests
├── errors.go               # Error formatting
├── errors_test.go          # Error formatter tests
├── latex.go                # LaTeX generator
├── latex_test.go           # LaTeX generator tests
├── lexer.go                # Lexical analyzer
├── lexer_test.go           # Lexer tests
├── parser.go               # RPN parser
├── parser_test.go          # Parser tests
├── token.go                # Token definitions
├── token_test.go           # Token tests
├── integration_test.go     # Integration tests
├── test_io_contract.sh     # I/O contract validation
└── README.md               # This file
```

## Implementation Details

### Architecture

The tool follows a classic compiler pipeline:

1. **Lexer** (`lexer.go`) - Tokenizes the input string
2. **Parser** (`parser.go`) - Parses tokens into an Abstract Syntax Tree (AST)
3. **LaTeX Generator** (`latex.go`) - Converts AST to LaTeX notation
4. **CLI** (`cmd/rpn2tex/main.go`) - Orchestrates the pipeline

### Operator Precedence

- Multiplication (`*`) and Division (`/`): Precedence level 2 (higher)
- Addition (`+`) and Subtraction (`-`): Precedence level 1 (lower)

### Associativity

- All operators are left-associative
- Subtraction and division require special handling for right operands with equal precedence

### Design Principles

1. **Immutability** - All data structures are immutable by convention
2. **Exact String Preservation** - Floating-point numbers are stored as strings to preserve exact decimal representation
3. **Position Tracking** - Line and column numbers are 1-based for user-friendly error messages
4. **Idiomatic Go** - Follows Go best practices and conventions

## Quality Metrics

- **Test Coverage**: 90.8% (core library), 37.0% (CLI)
- **Lines of Code**: ~1200 (including tests)
- **I/O Contract**: 21/21 test cases passing (100%)

## Exit Codes

- `0` - Success
- `1` - Error (lexer error, parser error, file error, etc.)

## Supported Operators

| Operator | RPN | LaTeX | Description |
|----------|-----|-------|-------------|
| Addition | `+` | `+` | Add two numbers |
| Subtraction | `-` | `-` | Subtract two numbers |
| Multiplication | `*` | `\times` | Multiply two numbers |
| Division | `/` | `\div` | Divide two numbers |

## Limitations

- Exponentiation (`^`) is not supported
- Only basic arithmetic operators are supported
- No support for functions (sin, cos, etc.)
- No support for variables

## License

This is a migration project for academic/research purposes.

## Migration Notes

This implementation is a direct migration from Python to Go, maintaining exact behavioral compatibility with the original Python implementation. All 21 test cases from the I/O contract pass with identical outputs.

**Key Migration Decisions:**

1. **Package Structure**: Used `cmd/rpn2tex/main.go` for CLI entry point (Go convention)
2. **Error Handling**: Converted Python exceptions to Go error returns
3. **Type System**: Used interfaces for union types (Expr interface)
4. **Visitor Pattern**: Replaced Python's `@singledispatchmethod` with Go type switches
5. **Testing**: Comprehensive table-driven tests following Go conventions
