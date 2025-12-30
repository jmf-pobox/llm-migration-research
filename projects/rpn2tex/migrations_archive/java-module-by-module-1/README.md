# I/O Contract Package for rpn2tex Java Migration

This directory contains the complete I/O contract generated from the Python rpn2tex implementation.

## Files in This Package

### 1. io_contract.md
The main I/O contract document containing:
- Executive summary of the Python implementation
- Complete test case matrix (21 tests)
- Detailed observations on operator behavior
- Supported and unsupported features
- Error handling specifications
- Migration notes for Java implementation

**Use this file as the authoritative specification for Java implementation validation.**

### 2. test_data.csv
Machine-readable test data in CSV format with columns:
- Test number
- Input string
- Expected output
- Status (SUCCESS/ERROR)
- Test category
- Notes

**Use this file for automated test validation and CI/CD integration.**

### 3. EXECUTION_SUMMARY.md
Executive summary of the I/O contract generation process:
- Execution method and results
- Breakdown by test category
- Overall statistics
- Key findings and patterns
- Validation protocol for Java migration
- Conclusion and next steps

**Use this file to understand the scope and methodology of the contract.**

## Quick Start for Java Developers

1. Read `io_contract.md` completely to understand expected behavior
2. Use `test_data.csv` to create automated test cases
3. Run each test input through your Java implementation
4. Compare outputs exactly with the expected outputs listed
5. Verify error handling matches the Python implementation

## Key Test Categories

| Category | Count | Expected Result |
|----------|-------|-----------------|
| Basic Operations | 4 | 100% Success |
| Precedence & Parentheses | 8 | 100% Success |
| Chained Operations | 3 | 100% Success |
| Decimal Numbers | 2 | 100% Success |
| Complex Expressions | 2 | 100% Success |
| Unsupported Operators | 2 | 100% Error (expected) |

## Supported Operators (Java must implement these)

- `+` Addition - renders as `+`
- `-` Subtraction - renders as `-`
- `*` Multiplication - renders as `\times`
- `/` Division - renders as `\div`

## Unsupported Operators (Java must reject these)

- `^` Exponentiation - should produce: "Error: Unexpected character '^'"

## Critical Implementation Details

1. **Output wrapping:** All LaTeX output must be wrapped in `$ ... $`
2. **Spaces around operators:** LaTeX output must include spaces around operators
3. **Parenthesization:** Must follow operator precedence rules exactly
4. **Error messages:** Must match Python implementation's error format
5. **Exit codes:** 0 for success, 1 for errors

## Example Test Validation

Input: `5 3 + 2 *`
Expected: `$( 5 + 3 ) \times 2$`
Why: Addition has lower precedence than multiplication, so addition operation is parenthesized

## Implementation Reference

- **Python Source:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
- **Entry Point:** `python3 -m rpn2tex.cli -` (stdin mode)
- **Architecture:** Lexer → Parser → LaTeXGenerator

## Questions or Issues?

Refer to the detailed analysis in `io_contract.md` or the execution summary in `EXECUTION_SUMMARY.md`.

Generated: 2025-12-28
Phase: Phase 0 - Python Implementation Analysis
Status: Complete and validated
