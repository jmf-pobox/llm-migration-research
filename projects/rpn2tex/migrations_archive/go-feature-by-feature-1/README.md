# rpn2tex - Go Migration (Feature 1: Numbers)

This is the Go implementation of rpn2tex, migrated feature-by-feature from the Python source.

## Current Status

**Implemented Features:**
- ✅ Feature 1: Numbers (integers and decimals)

**Infrastructure:**
- ✅ Token types and lexer
- ✅ AST nodes and parser
- ✅ LaTeX generator
- ✅ Error formatting
- ✅ CLI entry point

## Project Structure

```
.
├── go.mod              # Go module definition
├── token.go            # Token types and definitions
├── ast.go              # AST node interface and implementations
├── errors.go           # Error formatting with source context
├── lexer.go            # Lexical analysis
├── parser.go           # RPN parser
├── latex.go            # LaTeX generator
├── rpn2tex_test.go     # Unit tests
├── integration_test.go # Integration and I/O contract tests
└── cmd/
    └── rpn2tex/
        └── main.go     # CLI entry point
```

## Building

```bash
# Build the library
go build ./...

# Build the CLI binary
go build -o rpn2tex ./cmd/rpn2tex
```

## Running

```bash
# Single integer
./rpn2tex "5"
# Output: $5$

# Decimal number
./rpn2tex "3.14"
# Output: $3.14$
```

## Testing

```bash
# Run all tests
go test ./...

# Run with verbose output
go test -v ./...

# Run I/O contract tests
go test -v -run TestIOContract ./...
```

## Quality Gates

All quality gates pass:

```bash
# Compilation
go build ./...          # ✅ Compiles without errors

# Linting
go vet ./...            # ✅ No issues

# Formatting
gofmt -l .              # ✅ All files properly formatted

# Tests
go test ./...           # ✅ All tests pass
```

## I/O Contract

The implementation satisfies the I/O contract for Feature 1 (Numbers):

| Input   | Expected Output | Status |
|---------|----------------|--------|
| `5`     | `$5$`          | ✅ PASS |
| `3.14`  | `$3.14$`       | ✅ PASS |

## Architecture

The implementation follows a classic compiler pipeline:

1. **Lexer** (`lexer.go`): Raw text → tokens
2. **Parser** (`parser.go`): Tokens → Abstract Syntax Tree (AST)
3. **Generator** (`latex.go`): AST → LaTeX output
4. **CLI** (`cmd/rpn2tex/main.go`): Orchestrates the pipeline

### Token Layer

- `TokenType`: Enum-like constants using `iota`
- `Token`: Struct with Type, Value, Line, Column
- Supports: NUMBER, EOF

### AST Layer

- `Expr`: Interface for all expression nodes
- `NumberNode`: Represents numeric literals
- Position tracking (line, column) for error reporting

### Lexer

- Character-by-character scanning
- Handles integers and decimals
- Tracks position (1-based line and column)

### Parser

- Stack-based RPN parsing
- Creates AST nodes from tokens
- Error handling with position information

### Generator

- Visitor pattern for AST traversal
- Type assertion-based dispatch
- Simple string generation for numbers

## Idiomatic Go Patterns Used

1. **Package Structure**: Library code in root package, CLI in `cmd/`
2. **Naming**: Exported (public) names use PascalCase
3. **Error Handling**: Return `error` as last return value
4. **Interfaces**: Small, focused `Expr` interface
5. **Testing**: Table-driven tests with `t.Run()`
6. **Documentation**: Doc comments starting with identifier name

## Next Features

The following features are planned for future implementation:

- Feature 2: Addition (`+`)
- Feature 3: Subtraction (`-`)
- Feature 4: Multiplication (`*`)
- Feature 5: Division (`/`)
- Feature 6: Operator Precedence and Parenthesization

## References

- Phase 1 Specification: `artifacts/PHASE_1_FEATURE_SPECIFICATION.md`
- Source Language: Python (rpn2tex)
- Target Language: Go 1.21+
