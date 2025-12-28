# Feature 4: Multiplication - Quick Reference

**Status**: ✅ Complete | **Date**: 2025-12-28

## Quick Facts

- **Operator**: `*` (asterisk)
- **LaTeX Symbol**: `\times` (not asterisk!)
- **Precedence Level**: 2 (higher than +/- at level 1)
- **Token Type**: `TokenType::Mult`
- **Tests Added**: 8 integration tests
- **Code Changes**: None (validation only)

## Key Test Cases

| Input | Output | Purpose |
|-------|--------|---------|
| `4 7 *` | `$4 \times 7$` | Basic multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Higher precedence → no parens |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Lower precedence → add parens |

## Precedence Behavior

**Critical Difference from Addition/Subtraction:**
- Multiplication has **higher precedence** (level 2 vs level 1)
- This means: `2 + 3 * 4` renders WITHOUT parentheses
- But: `(2 + 3) * 4` needs parentheses to override precedence

### Precedence Rules

1. **Higher precedence child** → No parens needed
   - `2 + 3 * 4` = `$2 + 3 \times 4$` ✅
   - The `*` is higher precedence, so no parens

2. **Lower precedence child** → Add parens
   - `(2 + 3) * 4` = `$( 2 + 3 ) \times 4$` ✅
   - The `+` is lower precedence, needs parens

3. **Same precedence** → No special handling (commutative)
   - Multiplication is commutative: `a * b = b * a`
   - No special right-side handling needed (unlike `-` and `/`)

## Implementation Checklist

All components were already implemented in Feature 1:

- ✅ **Lexer** (src/lexer.rs, lines 166-173)
  - Recognizes `*` character
  - Creates `TokenType::Mult` token

- ✅ **Parser** (src/parser.rs, lines 101, 118)
  - Handles `TokenType::Mult` in match statement
  - Creates `BinaryOp` with operator string `"*"`

- ✅ **LaTeX Generator** (src/latex.rs, lines 53, 59)
  - Maps `"*"` to `r"\times"` (LaTeX symbol)
  - Assigns precedence level 2

- ✅ **Precedence Logic** (src/latex.rs, lines 119-136)
  - Correctly adds parens when child has lower precedence
  - No parens when child has higher/equal precedence

## Test Coverage

### Integration Tests (8 total)

1. `test_io_contract_case_7_simple_multiplication`
   - Input: `4 7 *`
   - Verifies: Basic operator works

2. `test_io_contract_case_8_multiplication_with_addition`
   - Input: `2 3 4 * +`
   - Verifies: Higher precedence doesn't need parens

3. `test_multiplication_precedence_with_addition_child`
   - Input: `2 3 + 4 *`
   - Verifies: Lower precedence left child needs parens

4. `test_multiplication_precedence_right_child`
   - Input: `2 3 4 + *`
   - Verifies: Lower precedence right child needs parens

5. `test_multiplication_with_decimal`
   - Input: `3.14 2 *`
   - Verifies: Works with decimal numbers

6. `test_complex_precedence_both_children`
   - Input: `1 2 + 3 4 + *`
   - Verifies: Both children can need parens

7. `test_multiplication_then_addition`
   - Input: `5 3 * 2 +`
   - Verifies: Higher precedence as left child of addition

8. `test_addition_then_multiplication`
   - Input: `5 3 + 2 *`
   - Verifies: Lower precedence as left child of multiplication

### Unit Tests

Multiplication uses the same unit tests as other operators:
- Token generation tests (lexer.rs)
- Operator mapping tests (latex.rs)

## Usage Examples

### CLI
```bash
# Simple multiplication
echo "4 7 *" | cargo run -- -
# Output: $4 \times 7$

# Precedence demonstration
echo "2 3 4 * +" | cargo run -- -
# Output: $2 + 3 \times 4$

# Forced grouping
echo "2 3 + 4 *" | cargo run -- -
# Output: $( 2 + 3 ) \times 4$
```

### Library
```rust
use rpn2tex::{lexer::Lexer, parser::Parser, latex::LaTeXGenerator};

let input = "2 3 4 * +";
let tokens = Lexer::new(input).tokenize()?;
let ast = Parser::new(tokens).parse()?;
let latex = LaTeXGenerator::new().generate(&ast);
// latex == "$2 + 3 \\times 4$"
```

## Common Mistakes

### ❌ Wrong: Using asterisk in LaTeX
```rust
// Don't do this:
binary_ops.insert("*".to_string(), "*".to_string());
// Output would be: $2 * 3$ (wrong!)
```

### ✅ Correct: Using \times
```rust
// Do this:
binary_ops.insert("*".to_string(), r"\times".to_string());
// Output: $2 \times 3$ (correct!)
```

### ❌ Wrong: Same precedence as addition
```rust
// Don't do this:
precedence.insert("*".to_string(), 1);  // Same as +/-
// Would incorrectly parenthesize: $2 + (3 \times 4)$
```

### ✅ Correct: Higher precedence
```rust
// Do this:
precedence.insert("*".to_string(), 2);  // Higher than +/-
// Correctly renders: $2 + 3 \times 4$
```

## Debugging Tips

### Verify precedence level
```rust
let generator = LaTeXGenerator::new();
let prec = generator.precedence.get("*");
assert_eq!(*prec.unwrap(), 2);  // Should be 2!
```

### Check LaTeX symbol
```rust
let generator = LaTeXGenerator::new();
let symbol = generator.binary_ops.get("*");
assert_eq!(symbol.unwrap(), r"\times");  // Not "*"!
```

### Test precedence behavior
```bash
# This should NOT have parentheses:
echo "2 3 4 * +" | cargo run -- -
# Expected: $2 + 3 \times 4$

# This SHOULD have parentheses:
echo "2 3 + 4 *" | cargo run -- -
# Expected: $( 2 + 3 ) \times 4$
```

## Quality Gate Commands

Run these to verify everything works:

```bash
# All commands from project root
cd /Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/projects/rpn2tex/migrations/rust-feature-by-feature

# Compilation
cargo check

# Linting
cargo clippy -- -D warnings

# Formatting
cargo fmt --check

# All tests
cargo test

# Just multiplication tests
cargo test multiplication

# Specific test
cargo test test_io_contract_case_7_simple_multiplication
```

## Files to Review

| File | Lines | Purpose |
|------|-------|---------|
| `src/tokens.rs` | 26 | `TokenType::Mult` definition |
| `src/lexer.rs` | 166-173 | `*` tokenization |
| `src/parser.rs` | 101, 118 | MULT token handling |
| `src/latex.rs` | 53 | LaTeX symbol mapping |
| `src/latex.rs` | 59 | Precedence level |
| `src/latex.rs` | 119-136 | Parenthesization logic |
| `tests/io_contract.rs` | 116-204 | Integration tests |

## Next Feature

**Feature 5: Division** will be similar:
- Operator: `/`
- LaTeX symbol: `\div`
- Precedence: 2 (same as multiplication)
- Non-commutative: Like subtraction, needs special right-side handling
- Expected: Validation only (no code changes)

## Summary

✅ Multiplication is fully implemented and tested
✅ Precedence level 2 correctly assigned
✅ LaTeX `\times` symbol used
✅ All 8 integration tests passing
✅ All quality gates passing
✅ Zero code changes required

**Total time**: ~65 minutes (validation + tests + docs)
