# Subtraction Feature Migration Complete

## Feature 3: Subtraction

Successfully migrated the subtraction feature from Python to Rust.

## Implementation Summary

### Files Modified

1. **src/tokens.rs**
   - Added `Minus` variant to `TokenType` enum
   - Properly documented

2. **src/lexer.rs**
   - Updated to recognize '-' as subtraction operator when not followed by digit
   - Maintains distinction between negative numbers ("-5") and subtraction operator ("5 -")
   - Added 6 comprehensive tests for lexer subtraction handling

3. **src/parser.rs**
   - Updated to handle `TokenType::Minus` in the parser
   - Combined with Plus handling using match pattern
   - Maps to "-" operator string
   - Added 6 comprehensive tests for parser subtraction handling

4. **src/latex.rs**
   - Added "-" case to operator mapping
   - Generates " - " with proper spacing
   - Added 5 comprehensive tests for LaTeX generation

5. **src/lib.rs**
   - Added 6 integration tests for subtraction feature
   - Tests cover basic subtraction, chained subtraction, floats, errors, and negative numbers

## I/O Contract Verification

### Test Cases (All Pass)

1. **Input:** `5 3 -`
   **Output:** `$5 - 3$` ✓

2. **Input:** `5 3 - 2 -`
   **Output:** `$5 - 3 - 2$` ✓

### Edge Cases Verified

- Negative numbers vs. operator: `-5` → `$-5$` ✓
- Subtraction with floats: `5.5 2.3 -` → `$5.5 - 2.3$` ✓
- Missing operand error: `5 -` → Error ✓
- Extra operand error: `5 3 2 -` → Error ✓

### Regression Tests

All existing features continue to work:
- Numbers: `5` → `$5$` ✓
- Addition: `5 3 +` → `$5 + 3$` ✓
- Chained addition: `1 2 + 3 + 4 +` → `$1 + 2 + 3 + 4$` ✓

## Quality Gates

All quality gates pass:

1. ✓ **cargo check** - Code compiles successfully
2. ✓ **cargo clippy -- -D warnings** - No clippy warnings
3. ✓ **cargo fmt --check** - Code is properly formatted
4. ✓ **cargo test** - All 88 tests pass (100% pass rate)

## Test Coverage

- **Total tests:** 88 unit/integration tests + 19 doc tests
- **New tests added:** 23 tests specifically for subtraction feature
- **Test categories:**
  - Lexer tests: 6 new
  - Parser tests: 6 new
  - LaTeX generation tests: 5 new
  - Integration tests: 6 new

## Key Implementation Details

### Left-Associativity
Subtraction is left-associative, correctly handled by RPN stack parsing:
- `5 3 - 2 -` evaluates as `(5 - 3) - 2`
- AST structure: `BinaryOp("-", BinaryOp("-", 5, 3), 2)`

### Negative Number Disambiguation
The lexer correctly distinguishes:
- `-5` at start or after operator → NUMBER token with value "-5"
- `5 -` after whitespace/number → MINUS operator token

### Operator Mapping
- Token: `TokenType::Minus`
- AST operator string: `"-"`
- LaTeX output: `" - "` (with space padding)

## Idiomatic Rust Features Used

1. **Enums with pattern matching:** `TokenType::Plus | TokenType::Minus =>`
2. **Result types:** Proper error handling throughout
3. **Documentation:** All public items have doc comments with examples
4. **Attributes:** `#[must_use]`, `#[derive(Debug, Clone, PartialEq, Eq)]`
5. **Testing:** Comprehensive unit and integration tests

## Migration Compliance

- ✓ Follows specification from Phase 1
- ✓ Matches Python implementation behavior exactly
- ✓ Passes I/O contract test cases
- ✓ Maintains backward compatibility with existing features
- ✓ No clippy warnings or formatting issues
- ✓ All tests pass on first attempt

## Next Steps

The subtraction feature is complete and ready for use. The next feature to migrate would be:
- **Feature 4: Multiplication** (with higher precedence than addition/subtraction)
