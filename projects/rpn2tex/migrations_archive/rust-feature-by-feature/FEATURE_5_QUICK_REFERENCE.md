# Feature 5: Division - Quick Reference

## Test Cases

```bash
# Case 1: Simple division
echo "10 2 /" | cargo run -- -
# Expected: $10 \div 2$

# Case 2: Chained division
echo "100 10 / 5 / 2 /" | cargo run -- -
# Expected: $100 \div 10 \div 5 \div 2$
```

## Key Points

- **Operator**: `/` → `\div` (LaTeX)
- **Precedence**: Level 2 (same as multiplication)
- **Associativity**: Left-associative
- **Non-commutative**: `a / (b / c)` ≠ `(a / b) / c`

## Implementation

### Token
```rust
TokenType::Div
```

### Lexer
```rust
'/' => Token::new(TokenType::Div, "/".to_string(), ...)
```

### Parser
```rust
TokenType::Div => BinaryOp { operator: "/", ... }
```

### LaTeX
```rust
"/" => r"\div"      // LaTeX symbol
precedence: 2       // Same as multiplication
needs_parens: true  // For right-side division
```

## Parenthesization Rules

| Expression | Output | Reason |
|------------|--------|--------|
| `10 / 2 + 3` | `$10 \div 2 + 3$` | Higher precedence (no parens) |
| `10 / (2 + 3)` | `$10 \div ( 2 + 3 )$` | Lower precedence child (parens) |
| `100 / 10 / 5` | `$100 \div 10 \div 5$` | Left-associative (no parens) |
| `100 / (10 / 5)` | `$100 \div ( 10 \div 5 )$` | Right-associative (parens) |
| `10 / 2 * 3` | `$10 \div 2 \times 3$` | Same precedence (no parens) |

## Examples

```rust
// Simple division
"10 2 /"                // $10 \div 2$

// Chained division (left-associative)
"100 10 / 5 / 2 /"      // $100 \div 10 \div 5 \div 2$

// Right-side division (requires parens)
"100 10 5 / /"          // $100 \div ( 10 \div 5 )$

// With addition (higher precedence)
"10 2 / 3 +"            // $10 \div 2 + 3$

// Addition child (requires parens)
"10 2 3 + /"            // $10 \div ( 2 + 3 )$

// Mixed with multiplication
"10 2 / 3 *"            // $10 \div 2 \times 3$

// Complex expression
"10 2 / 3 + 4 *"        // $( 10 \div 2 + 3 ) \times 4$
```

## Tests

```bash
# Run all tests
cargo test

# Run division tests only
cargo test division

# Run specific test
cargo test test_io_contract_case_9_simple_division
```

## Files

- `src/tokens.rs` - TokenType::Div
- `src/lexer.rs` - Tokenizes `/`
- `src/parser.rs` - Handles DIV token
- `src/latex.rs` - Maps to `\div`, precedence 2
- `tests/io_contract.rs` - 7 test cases
