# PHASE 0: I/O Contract Generation - Artifacts

This directory contains the complete I/O contract and test execution artifacts for the rpn2tex migration from Python to Go.

## Files in This Directory

### 1. PHASE_0_IO_CONTRACT.md
**Primary artifact for migration validation**

Contains the authoritative I/O contract with:
- 18 successful test cases with exact expected outputs
- 3 error test cases with expected error messages
- Detailed specifications for LaTeX output formatting
- Operator mappings and precedence rules
- Migration validation checklist

**Use this file to validate each test case during Go implementation.**

### 2. TEST_EXECUTION_SUMMARY.md
**Executive summary of test execution**

Provides:
- Execution statistics (18 passed, 3 failed)
- Breakdown of test categories
- Key observations about operator support and formatting
- Implementation requirements for Go migration
- Verification results confirming exact output matching

**Use this file for understanding overall test coverage and implementation requirements.**

### 3. RAW_TEST_DATA.txt
**Raw test execution results**

Contains:
- All 21 test cases with input, output, error, and exit code
- Error messages with exact line/column information
- Structured format for automated parsing if needed

**Use this file for detailed reference or automated test case generation.**

## Quick Reference

### Test Statistics
- **Total Test Cases:** 21
- **Successful Cases:** 18 (85.7%)
- **Error Cases:** 3 (expected, unsupported operator)
- **Operators Tested:** +, -, *, / (^ not supported)

### Key Implementation Requirements

1. **Lexer:** Must support +, -, *, /, integer and floating-point numbers
2. **Parser:** Must implement RPN parsing with correct operator precedence
3. **Generator:** Must produce LaTeX output with $...$ delimiters
4. **Formatting:**
   - Use `\times` for multiplication
   - Use `\div` for division
   - Space-padded parentheses: `( )` format
   - Preserve decimal points in floating-point numbers

### Error Handling
- Exit code 0 for successful parsing and generation
- Exit code 1 for lexer/parser errors (e.g., unsupported `^` operator)
- Error messages sent to stderr with line/column information

## Testing Against These Artifacts

To validate the Go implementation:

1. **Run each input from PHASE_0_IO_CONTRACT.md**
2. **Compare output exactly** (whitespace matters!)
3. **Check exit codes** (0 for success, 1 for errors)
4. **Verify error messages** for error cases
5. **Confirm LaTeX formatting** (math mode delimiters, operator symbols, spacing)

## Source Implementation Reference

The Python source implementation is located at:
```
/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/
```

Key files:
- `lexer.py` - Tokenization
- `parser.py` - RPN parsing
- `latex_gen.py` - LaTeX generation
- `cli.py` - Command-line interface

## Generation Method

All test cases were executed using:
```bash
echo "INPUT" | python -m rpn2tex.cli -
```

Outputs were captured exactly as produced by the Python implementation.

## Notes

- All outputs are stripped of trailing newlines for cleaner comparison
- The `^` operator (exponentiation) is intentionally not supported in the Python implementation
- Parentheses are inserted intelligently based on operator precedence rules
- Floating-point precision is preserved exactly as input

---

Generated: 2025-12-30
Migration Path: Python -> Go
Target Directory: go-module-by-module-3
