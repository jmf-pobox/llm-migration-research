# Phase 0 Artifacts - I/O Contract Verification

This directory contains the complete I/O contract specification for the rpn2tex migration project, verified against the Python reference implementation.

## Files in This Directory

### 1. PHASE_0_IO_CONTRACT.md (159 lines)
**The primary specification document for all migration targets.**

Contains:
- Complete test case specifications organized by operation category
- Expected outputs for all 20 passing test cases
- Error case documentation (3 exponentiation cases)
- LaTeX output format rules
- Space and punctuation specifications
- Migration checklist
- Reference implementation details

**Use this file as the authoritative spec when implementing in Go, Rust, or other languages.**

### 2. PHASE_0_VERIFICATION_SUMMARY.md (242 lines)
**Detailed verification report with results and analysis.**

Contains:
- Executive summary and statistics
- Category-by-category test results
- Detailed verification of each test category
- Output format specification
- Key implementation rules verified
- Execution commands and reference information
- Migration readiness checklist

**Use this to understand HOW the implementation should work.**

### 3. QUICK_REFERENCE.md (113 lines)
**Compact reference guide for quick lookups.**

Contains:
- All test cases organized by category in tables
- Critical implementation rules
- Operator symbols mapping
- Precedence levels
- Parentheses rules with examples
- Test verification command

**Use this for quick lookups during implementation.**

### 4. test_execution_results.json
**Machine-readable test results in JSON format.**

Contains:
- Complete test execution metadata
- All inputs, outputs, and status codes
- Timing and error information
- Organized by category

**Use for automated test validation and CI/CD pipelines.**

## Quick Navigation

### For Implementation
1. Start with: **QUICK_REFERENCE.md** (get the rules)
2. Reference: **PHASE_0_IO_CONTRACT.md** (detailed specs)
3. Understand: **PHASE_0_VERIFICATION_SUMMARY.md** (the logic)

### For Verification
1. Compare outputs against: **PHASE_0_IO_CONTRACT.md** test cases
2. Run tests and compare to: **test_execution_results.json**

### For Documentation
1. Cite: **PHASE_0_IO_CONTRACT.md** as the spec
2. Explain implementation with: **PHASE_0_VERIFICATION_SUMMARY.md**

## Test Summary

| Category | Tests | Status |
|----------|-------|--------|
| Numbers | 2 | PASS |
| Addition | 2 | PASS |
| Subtraction | 2 | PASS |
| Multiplication | 4 | PASS |
| Division | 3 | PASS |
| Precedence | 5 | PASS |
| Decimals | 2 | PASS |
| **Error Cases** | 3 | EXPECTED ERROR |
| **TOTAL** | **23** | **23/23 OK** |

## Key Specifications

### Output Format
All results wrapped in LaTeX math mode: `$expression$`

### Operators
- `+` addition → ` + ` (with spaces)
- `-` subtraction → ` - ` (with spaces)
- `*` multiplication → ` \times ` (with spaces)
- `/` division → ` \div ` (with spaces)

### Precedence (Highest to Lowest)
1. Multiplication and Division (*)
2. Addition and Subtraction (+)

### Parenthesization
- Add parentheses around lower-precedence operations when they're children of higher-precedence operations
- Format: `( expression )` (with spaces)
- Example: `5 3 + 2 *` → `$( 5 + 3 ) \times 2$`

## How to Verify Your Implementation

```bash
# For each test case from PHASE_0_IO_CONTRACT.md:
echo "INPUT_EXPRESSION" | YOUR_IMPLEMENTATION -

# Compare output to expected value
# Example:
echo "5 3 +" | your_rpn2tex_binary
# Should output: $5 + 3$
```

## Reference Implementation

**Location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`

**Language:** Python 3

**Key Classes:**
- `Lexer` - Tokenization
- `Parser` - Parsing RPN to AST
- `LaTeXGenerator` - AST to LaTeX conversion
- `ErrorFormatter` - Error message formatting

## Important Notes

1. **Exponentiation Not Supported:** The `^` operator is not implemented in the reference Python implementation and produces an error.

2. **Decimal Handling:** Decimal numbers are preserved as-is without rounding.

3. **Associativity:** All operators are left-associative (e.g., `5 3 - 2 -` = `(5 - 3) - 2`).

4. **Spaces Matter:** The output format includes specific spacing around operators and parentheses.

## Next Phases

- **Phase 1:** Feature Specification - Identify incremental features
- **Phase 2:** Implementation - Code migration in target language
- **Phase 3:** Validation - Verify against I/O contract
- **Phase 4:** Testing - Unit and integration tests

---

**Document Version:** 1.0
**Verification Date:** 2025-12-29
**Status:** Complete and Verified
**Authority:** Python reference implementation
