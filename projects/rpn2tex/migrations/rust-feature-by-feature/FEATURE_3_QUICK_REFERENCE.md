# Feature 3: Subtraction - Quick Reference

## Status: ✅ COMPLETE

## I/O Contract Tests (2/2 Passing)

```bash
# Test 1: Simple subtraction
Input:  "5 3 -"
Output: "$5 - 3$"
Status: ✅ PASS

# Test 2: Chained subtraction
Input:  "5 3 - 2 -"
Output: "$5 - 3 - 2$"
Status: ✅ PASS
```

## Quality Gates (4/4 Passing)

| Gate | Result |
|------|--------|
| `cargo check` | ✅ PASS |
| `cargo clippy -- -D warnings` | ✅ PASS |
| `cargo fmt --check` | ✅ PASS |
| `cargo test` | ✅ PASS (36 tests) |

## Test Statistics

- Unit tests: 11
- Integration tests: 9 (+5 new)
- Doc tests: 16
- Total: 36 tests
- Pass rate: 100%

## Files Modified

### Tests
- `tests/io_contract.rs`: Added 5 integration tests

### Documentation
- `FEATURE_3_SUBTRACTION_REPORT.md`
- `FEATURE_3_COMPLETION_SUMMARY.md`
- `FEATURE_3_QUICK_REFERENCE.md`
- `MIGRATION_STATUS.md` (updated)

### Source Code
- No changes (all functionality already present)

## Key Features Validated

1. ✅ Operator precedence (level 1)
2. ✅ Left-associativity
3. ✅ Right-associativity handling (parentheses)
4. ✅ Negative number disambiguation
5. ✅ Mixed operations with addition

## Example Outputs

```bash
# Simple
$ echo "5 3 -" | cargo run -- -
$5 - 3$

# Chained
$ echo "5 3 - 2 -" | cargo run -- -
$5 - 3 - 2$

# Right associativity (requires parens)
$ echo "5 3 2 - -" | cargo run -- -
$5 - ( 3 - 2 )$

# Negative number
$ echo "-5 3 -" | cargo run -- -
$-5 - 3$

# Mixed with addition
$ echo "10 3 - 2 +" | cargo run -- -
$10 - 3 + 2$
```

## Next Feature

Feature 4: Multiplication
- Verify `*` operator
- Precedence level 2
- LaTeX output: `\times`
