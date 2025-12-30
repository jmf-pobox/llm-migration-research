# Feature 3: Subtraction - Migration Complete

## Summary

Successfully migrated the subtraction feature from Python to Go, building on Features 1 (Numbers) and 2 (Addition).

## Implementation Date
2025-12-29

## Changes Made

### 1. Token Layer (`token.go`)
- Added `MINUS` token type constant for subtraction operator

### 2. Lexer Layer (`lexer.go`)
- Implemented "-" operator recognition with proper disambiguation logic
- Added logic to distinguish between:
  - Negative numbers: `-` immediately followed by a digit (e.g., `-5`)
  - Subtraction operator: `-` followed by non-digit or whitespace
- Disambiguation placed before other operator checks for correct precedence

### 3. Parser Layer (`parser.go`)
- Added `MINUS` case to handle subtraction in RPN stack operations
- Implemented proper RPN semantics: pop right operand, pop left operand
- Order preservation: `5 3 -` correctly produces `5 - 3` (not `3 - 5`)
- Error handling for insufficient operands

### 4. LaTeX Generator (`latex.go`)
- No changes needed - existing binary operator handling works for subtraction
- Outputs ` - ` with proper spacing

### 5. Tests (`feature_3_test.go`)
- Created comprehensive test suite including:
  - I/O contract tests (simple and chained subtraction)
  - Lexer tests (token recognition)
  - Parser tests (AST construction)
  - Disambiguation tests (negative number vs subtraction operator)
  - Non-commutative tests (order preservation)
  - Error handling tests (insufficient operands)

## Test Results

### All Quality Gates Passed
- `go build ./...` - SUCCESS
- `go vet ./...` - SUCCESS
- `gofmt -l .` - SUCCESS (all files properly formatted)
- `go test ./...` - SUCCESS (all 16 test cases passed)

### I/O Contract Validation

#### Test Case 1: Simple Subtraction
```
Input:    "5 3 -"
Expected: "$5 - 3$"
Actual:   "$5 - 3$"
Status:   PASS
```

#### Test Case 2: Chained Subtraction
```
Input:    "5 3 - 2 -"
Expected: "$5 - 3 - 2$"
Actual:   "$5 - 3 - 2$"
Status:   PASS
```

### Backward Compatibility

All previous feature tests continue to pass:
- Feature 1 (Numbers): 4/4 tests passing
- Feature 2 (Addition): 4/4 tests passing
- Feature 3 (Subtraction): 6/6 tests passing

Total: 16/16 tests passing

## Key Implementation Details

### Subtraction Semantics
- **Non-commutative**: Order matters (`5 - 3` ≠ `3 - 5`)
- **Left-associative**: `5 - 3 - 2` means `(5 - 3) - 2`
- **RPN order preserved**: Stack operations correctly maintain operand order

### Disambiguation Logic
The lexer correctly handles the dual meaning of "-":
1. When `-` is followed immediately by a digit → negative number token
2. When `-` is followed by whitespace or non-digit → subtraction operator token

### Error Handling
- Insufficient operands detected at parse time
- Clear error messages with token information
- Position tracking maintained for debugging

## Files Modified
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/go-feature-by-feature-1/token.go`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/go-feature-by-feature-1/lexer.go`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/go-feature-by-feature-1/parser.go`

## Files Created
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/go-feature-by-feature-1/feature_3_test.go`

## Code Quality
- Idiomatic Go patterns used throughout
- Comprehensive test coverage
- Clear comments and documentation
- Consistent with existing codebase style

## Next Steps
Ready to proceed with:
- Feature 4: Multiplication (introduces higher precedence)
- Feature 5: Division (another non-commutative operator)
- Feature 6: Precedence and parenthesization rules

## Verification Commands

```bash
# Build
cd /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/go-feature-by-feature-1
go build ./...

# Test
go test -v

# Manual I/O verification
echo "5 3 -" | go run . -
echo "5 3 - 2 -" | go run . -
```

## Notes
- Subtraction implementation reuses the BinaryOp AST node introduced in Feature 2
- Precedence handling (parenthesization) will be fully implemented in Feature 6
- Current implementation handles same-precedence operations correctly through left-associative parsing
