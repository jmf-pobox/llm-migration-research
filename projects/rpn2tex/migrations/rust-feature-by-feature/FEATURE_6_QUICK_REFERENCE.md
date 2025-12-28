# Feature 6: Precedence - Quick Reference

## Status: COMPLETE ✓

## Test Cases (5/5 Passing)

```bash
# All tests pass
cargo test
```

| Input | Expected | Status |
|-------|----------|--------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | ✓ |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | ✓ |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | ✓ |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | ✓ |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | ✓ |

## Implementation Summary

### Files
- `src/latex.rs`: Precedence table and parenthesization logic

### Key Components

1. **Precedence Table** (lines 56-60)
   - Level 1: `+` and `-`
   - Level 2: `*` and `/`

2. **Parenthesization Function** (lines 119-136)
   ```rust
   fn needs_parens(&self, child: &Expr, parent_precedence: i32, is_right: bool) -> bool
   ```
   - Rule 1: Lower precedence needs parens
   - Rule 2: Equal precedence on right needs parens for `-` and `/`

3. **Application** (lines 100-116)
   - Check left child with `is_right = false`
   - Check right child with `is_right = true`
   - Format: `( expr )` with spaces

## Quality Gates: ALL PASS ✓

- ✓ `cargo check` - Compilation successful
- ✓ `cargo clippy -- -D warnings` - No warnings
- ✓ `cargo fmt --check` - Properly formatted
- ✓ `cargo test` - All 24 tests passing

## Integration Tests

9 tests specifically verify precedence behavior:
- `test_addition_then_multiplication`
- `test_multiplication_precedence_with_addition_child`
- `test_multiplication_precedence_right_child`
- `test_complex_precedence_both_children`
- `test_complex_precedence_with_division`
- `test_multiplication_with_addition`
- `test_division_with_addition`
- `test_subtraction_right_associativity`
- `test_division_right_associativity`

## CLI Verification

```bash
echo "5 3 + 2 *" | cargo run --release -- -
# Output: $( 5 + 3 ) \times 2$
```

## Cross-Feature Integration

Works correctly with:
- Feature 1: Numbers (leaves don't need parens)
- Feature 2: Addition (precedence level 1)
- Feature 3: Subtraction (precedence level 1, right-associativity)
- Feature 4: Multiplication (precedence level 2)
- Feature 5: Division (precedence level 2, right-associativity)

## Key Design Decisions

1. **HashMap for precedence**: O(1) lookup
2. **Spaces in parentheses**: `( expr )` matches Python output
3. **Visitor pattern**: Clean separation of concerns
4. **Two-rule logic**: Simple and correct

## Performance

- Time: O(n) for n AST nodes
- Space: O(h) for tree height h
- No dynamic allocation overhead
