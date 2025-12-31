# rpn2tex - Go Implementation (Feature 1: Numbers)

This is an incremental Go migration of the rpn2tex Python project, implementing Feature 1 (Numbers) from the feature-based analysis.

## Feature Status

- [x] **Feature 1: Numbers** - Parse and output numeric literals (integers and floats)
- [ ] Feature 2: Addition
- [ ] Feature 3: Subtraction
- [ ] Feature 4: Multiplication
- [ ] Feature 5: Division
- [ ] Feature 6: Precedence

## Current Capabilities

The current implementation can:
- Parse integer literals (e.g., `5`)
- Parse float literals (e.g., `3.14`)
- Generate LaTeX output wrapped in `$...$`

### Example Usage

```bash
# Build the binary
go build -o rpn2tex cmd/rpn2tex/main.go

# From stdin
echo "5" | ./rpn2tex
# Output: $5$

# From command-line arguments
./rpn2tex 3.14
# Output: $3.14$
```

## Project Structure

```
go-feature-by-feature-3/
├── go.mod                  # Module definition
├── token.go                # Token type definitions
├── ast.go                  # AST node interface and types
├── errors.go               # Custom error types
├── lexer.go                # Lexical analysis
├── parser.go               # RPN parsing
├── latex.go                # LaTeX generation
├── lexer_test.go           # Lexer unit tests
├── parser_test.go          # Parser unit tests
├── latex_test.go           # Generator unit tests
├── integration_test.go     # End-to-end tests
└── cmd/
    └── rpn2tex/
        └── main.go         # CLI entry point
```

## Build and Test

### Prerequisites
- Go 1.21 or later

### Build
```bash
go build ./...
```

### Test
```bash
# Run all tests
go test ./...

# Run tests with coverage
go test -cover ./...

# Run tests verbosely
go test -v ./...
```

### Quality Gates
```bash
# Build check
go build ./...

# Vet check
go vet ./...

# Format check
gofmt -l .

# All tests
go test ./...
```

## Test Results

### Quality Gates: ALL PASS
- Build: ✓
- Vet: ✓
- Format: ✓
- Tests: ✓ (84.5% coverage)

### I/O Contract: VALIDATED
| Input | Expected | Actual | Status |
|-------|----------|--------|--------|
| `5` | `$5$` | `$5$` | ✓ PASS |
| `3.14` | `$3.14$` | `$3.14$` | ✓ PASS |

## Architecture

### Lexer
- Tokenizes input strings into tokens
- Handles whitespace correctly
- Tracks position information for error reporting

### Parser
- Implements RPN stack-based algorithm
- Validates expression completeness
- Returns AST root node

### LaTeX Generator
- Converts AST to LaTeX output
- Wraps expressions in `$...$` for math mode
- Uses type switch for extensibility

### CLI
- Accepts input from stdin or command-line arguments
- Outputs to stdout without trailing newline
- Errors go to stderr with exit code 1

## Go Idioms

This implementation follows Go best practices:
- Error returns instead of exceptions
- Interface-based AST design
- Type switches for polymorphism
- PascalCase/camelCase naming conventions
- Doc comments on exported types
- Table-driven tests
- Standard package structure (cmd/ for binaries)

## Next Steps

The next features to implement are:
1. Addition (`+` operator)
2. Subtraction (`-` operator)
3. Multiplication (`*` operator)
4. Division (`/` operator)
5. Precedence and parenthesization

Each feature builds incrementally on the previous ones, following the dependency chain defined in the analysis specification.

## License

This is a migration exercise for the rpn2tex project.
