# Subtraction Feature: File Changes Summary

## Modified Files

### 1. /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/rust-feature-by-feature-1/src/tokens.rs
- Added `TokenType::Minus` variant to the enum
- Properly documented as "Subtraction operator (-)"

### 2. /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/rust-feature-by-feature-1/src/lexer.rs
- Updated lexer to return `TokenType::Minus` token when '-' is not followed by digit
- Changed error case to return success with Minus token
- Added 6 new tests for subtraction tokenization

### 3. /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/rust-feature-by-feature-1/src/parser.rs
- Updated match pattern to handle both `Plus` and `Minus` tokens
- Maps `Minus` token to "-" operator string
- Added 6 new tests for subtraction parsing

### 4. /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/rust-feature-by-feature-1/src/latex.rs
- Added "-" case to operator mapping in `visit_binary_op`
- Generates " - " with proper spacing
- Added 5 new tests for LaTeX generation of subtraction

### 5. /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/rust-feature-by-feature-1/src/lib.rs
- Added 6 integration tests for the complete subtraction pipeline
- Tests cover basic cases, chained operations, floats, and error conditions

## Test Statistics

- **Before:** 65 tests
- **After:** 88 tests
- **New tests:** 23 tests specifically for subtraction
- **Pass rate:** 100% (all tests pass)

## Key Code Snippets

### Token Type Addition
```rust
pub enum TokenType {
    Number,
    Plus,
    Minus,  // NEW
    Eof,
}
```

### Lexer Change
```rust
// Negative numbers vs. subtraction operator (lookahead required)
if ch == '-' {
    self.advance();
    if !self.at_end() && self.peek().is_some_and(|c| c.is_ascii_digit()) {
        return self.scan_number("-".to_string(), start_line, start_column);
    }
    // Otherwise, it's a subtraction operator
    return Ok(Token::new(TokenType::Minus, "-", start_line, start_column));
}
```

### Parser Change
```rust
TokenType::Plus | TokenType::Minus => {
    let operator = if token.type_() == TokenType::Plus {
        "+"
    } else {
        "-"
    };
    // ... rest of operator handling
}
```

### LaTeX Generator Change
```rust
let op_latex = match node.operator() {
    "+" => "+",
    "-" => "-",  // NEW
    _ => node.operator(),
};
```

## Validation Results

✓ cargo check - Compiles without errors
✓ cargo clippy -- -D warnings - No warnings
✓ cargo fmt --check - Properly formatted
✓ cargo test - All 88 tests pass
✓ I/O Contract - Both test cases pass
✓ Regression - Existing features still work
