# I/O Contract Artifacts for rpn2tex Migration

This directory contains the complete I/O contract specification for the rpn2tex migration from Python to Rust.

## Files in This Directory

### 1. PHASE_0_IO_CONTRACT.md (PRIMARY REFERENCE)
**The canonical I/O specification document**

This is the main contract document that defines:
- 18 successful test cases with exact LaTeX outputs
- 3 error cases (all involving unimplemented ^ operator)
- Operator precedence and parenthesization rules
- LaTeX formatting conventions

**Use this file for:** Validating that the Rust implementation produces identical outputs for all test inputs.

### 2. TEST_EXECUTION_LOG.md (DETAILED EXECUTION RECORD)
Complete execution log of all 21 test cases, including:
- Summary statistics (85.7% success rate)
- Full output for each successful test case
- Detailed error information for the 3 error cases
- Architecture overview of the Python implementation
- Key observations for the Rust migration

**Use this file for:** Understanding how each test was executed and why certain tests produce errors.

### 3. PARSING_RULES_ANALYSIS.md (IMPLEMENTATION DETAILS)
In-depth analysis of the parsing and LaTeX generation rules, including:
- RPN to AST conversion algorithm
- Operator precedence table (both + and - at level 1, * and / at level 2)
- Parenthesization algorithm with detailed rules
- Mathematical verification of each rule against test cases
- Floating-point handling
- LaTeX symbol mapping
- Space formatting conventions

**Use this file for:** Understanding the exact implementation logic needed to match the Python reference in Rust.

### 4. README.md (THIS FILE)
Quick reference guide to all I/O contract artifacts.

## How to Use These Artifacts

### For Developers Implementing Rust Version

1. **Start with PHASE_0_IO_CONTRACT.md** - Review the test cases and expected outputs
2. **Reference PARSING_RULES_ANALYSIS.md** - Understand the exact algorithm
3. **Check TEST_EXECUTION_LOG.md** - See how the Python version handles edge cases

### For Validation/Testing

1. Run your Rust implementation against all 21 test inputs
2. Compare outputs to PHASE_0_IO_CONTRACT.md test cases table
3. For the 3 error cases (with ^), ensure your implementation rejects them (message may differ)

### For Architecture Understanding

Review the architecture section in TEST_EXECUTION_LOG.md to understand:
- Lexer → Parser → LaTeX Generator pipeline
- Error handling approach
- Module responsibilities

## Quick Reference: Test Coverage

| Category | Count | Examples |
|----------|-------|----------|
| Basic Operations | 4 | Addition, subtraction, multiplication, division |
| Complex Expressions | 11 | Multiple operators, mixed precedence, associativity |
| Floating-Point | 2 | Decimals like 3.14, 1.5 |
| Compound Expressions | 1 | Multiple sub-expressions |
| Error Cases | 3 | All with unsupported ^ operator |
| **Total** | **21** | - |

## Critical Validation Points

When implementing the Rust version, ensure:

1. **Exact LaTeX Output** - All 18 successful cases must produce identical LaTeX strings
2. **Operator Symbols** - Use `\times` for * and `\div` for / (not standard symbols)
3. **Parenthesization** - Must match exactly for all test cases
4. **Floating-Point** - Decimal representation must be preserved
5. **Output Format** - All expressions must be wrapped in `$...$` delimiters
6. **Spacing** - Single spaces around operators and inside parentheses
7. **Error Handling** - The 3 ^ cases should produce errors (message can differ)

## Test Execution Environment

- **Language:** Python 3
- **Date:** 2025-12-29
- **Source:** /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/
- **Execution Method:** Direct Python module import and function calls

## Success Metrics

- **18 out of 21 test cases pass** (85.7% success rate)
- **3 error cases** properly rejected (as expected - unimplemented feature)
- **All successful cases verified** - exact LaTeX output captured
- **Complete documentation** of parsing rules and precedence

## Next Steps

After implementing the Rust version:
1. Run all 21 test inputs through the Rust implementation
2. Create a matching I/O contract for the Rust version
3. Compare outputs between Python and Rust versions
4. Generate a validation report showing 100% compatibility (all 18 successful cases match)

## Notes

- The exponentiation operator (^) is explicitly marked as "Exercise" in the tokens.py file
- This is a deliberate omission in the base implementation, not a bug
- The Rust version should follow the same scope initially
- Future enhancements can add ^ support, but that's beyond PHASE 0 scope
