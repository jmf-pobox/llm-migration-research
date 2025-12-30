# rpn2tex - Rust Feature-by-Feature Migration

A Rust implementation of the rpn2tex converter, migrated feature-by-feature from the Python reference implementation.

## Project Structure

```
.
├── Cargo.toml                      # Project configuration
├── README.md                       # This file
├── FEATURE_1_NUMBERS_REPORT.md     # Detailed migration report
├── src/
│   ├── lib.rs                      # Library root
│   ├── main.rs                     # CLI entry point
│   ├── tokens.rs                   # Token types and definitions
│   ├── ast.rs                      # Abstract Syntax Tree nodes
│   ├── error.rs                    # Error types
│   ├── lexer.rs                    # Lexical analyzer
│   ├── parser.rs                   # RPN parser
│   └── latex.rs                    # LaTeX generator
└── tests/
    └── io_contract.rs              # I/O contract validation tests
```

## Features Implemented

### Feature 1: Numbers ✓
- Integer numbers: `5` → `$5$`
- Decimal numbers: `3.14` → `$3.14$`
- Negative numbers: `-5` → `$-5$`

### Feature 2: Addition ✓
- Simple addition: `5 3 +` → `$5 + 3$`
- Chained addition: `1 2 + 3 + 4 +` → `$1 + 2 + 3 + 4$`

### Feature 3: Subtraction ✓
- Simple subtraction: `5 3 -` → `$5 - 3$`
- Chained subtraction: `5 3 - 2 -` → `$5 - 3 - 2$`
- Right associativity: `5 3 2 - -` → `$5 - ( 3 - 2 )$`

### Feature 4: Multiplication ✓
- Simple multiplication: `4 7 *` → `$4 \times 7$`
- With precedence: `2 3 4 * +` → `$2 + 3 \times 4$`
- Complex precedence: `2 3 + 4 *` → `$( 2 + 3 ) \times 4$`

### Feature 5: Division ✓
- Simple division: `10 2 /` → `$10 \div 2$`
- Chained division: `100 10 / 5 / 2 /` → `$100 \div 10 \div 5 \div 2$`
- Right associativity: `100 10 5 / /` → `$100 \div ( 10 \div 5 )$`

## Building

```bash
# Check compilation
cargo check

# Build release version
cargo build --release

# Run clippy linter
cargo clippy -- -D warnings

# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt
```

## Testing

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test io_contract

# Run with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc
```

## Usage

### Command Line

```bash
# From stdin
echo "5" | cargo run -- -

# From file
cargo run -- input.rpn

# Save to file
cargo run -- input.rpn -o output.tex

# Using the built binary
./target/release/rpn2tex input.rpn
```

### As a Library

```rust
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;
use rpn2tex::latex::LaTeXGenerator;

fn main() {
    let input = "5 3 +";

    // Tokenize
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    // Parse
    let parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    // Generate LaTeX
    let generator = LaTeXGenerator::new();
    let latex = generator.generate(&ast);

    println!("{}", latex);  // Output: $5 + 3$
}
```

## I/O Contract

The implementation is validated against the Python reference implementation:

| Feature | Input | Expected Output | Status |
|---------|-------|----------------|--------|
| Numbers | `5` | `$5$` | ✓ |
| Numbers | `3.14` | `$3.14$` | ✓ |
| Addition | `5 3 +` | `$5 + 3$` | ✓ |
| Addition | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | ✓ |
| Subtraction | `5 3 -` | `$5 - 3$` | ✓ |
| Subtraction | `5 3 - 2 -` | `$5 - 3 - 2$` | ✓ |
| Multiplication | `4 7 *` | `$4 \times 7$` | ✓ |
| Multiplication | `2 3 4 * +` | `$2 + 3 \times 4$` | ✓ |
| Division | `10 2 /` | `$10 \div 2$` | ✓ |
| Division | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | ✓ |

## Development

### Code Quality Gates

All code must pass these gates before commit:

1. **Compilation**: `cargo check`
2. **Linting**: `cargo clippy -- -D warnings`
3. **Formatting**: `cargo fmt --check`
4. **Testing**: `cargo test`

### Running Quality Gates

```bash
# Run all quality gates in sequence
cargo check && \
cargo clippy -- -D warnings && \
cargo fmt --check && \
cargo test
```

## Documentation

Generate and view the documentation:

```bash
# Generate documentation
cargo doc

# Generate and open in browser
cargo doc --open

# Include private items
cargo doc --document-private-items
```

## Architecture

### Pipeline

The conversion follows a classic compiler pipeline:

```
Input Text → Lexer → Tokens → Parser → AST → Generator → LaTeX
```

1. **Lexer** (`lexer.rs`): Converts text into tokens
2. **Parser** (`parser.rs`): Builds AST from tokens using RPN stack algorithm
3. **Generator** (`latex.rs`): Converts AST to LaTeX with precedence handling

### Key Design Decisions

1. **Error Handling**: Custom error types with position information
2. **AST Design**: Boxed recursive types for memory efficiency
3. **Token Storage**: Owned strings for simplicity
4. **Parsing Strategy**: Stack-based for RPN (simpler than recursive descent)

## Performance

Rust implementation advantages:
- Zero-cost abstractions
- No garbage collection
- Compile-time optimization
- Memory safety without runtime overhead

## Contributing

When adding new features:

1. Update the corresponding module
2. Add unit tests
3. Add integration tests to `tests/io_contract.rs`
4. Update documentation
5. Ensure all quality gates pass
6. Create a feature report (like `FEATURE_1_NUMBERS_REPORT.md`)

## License

This is a migration study project.

## References

- Python reference implementation: `../../source/`
- I/O contract: `../../io_contract.json`
- Migration strategy: Feature-by-feature approach
