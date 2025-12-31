# rpn2tex I/O Contract Artifacts

This directory contains the complete I/O contract documentation and test data for the rpn2tex Python implementation. These artifacts define the expected behavior that all migrated implementations must match exactly.

## Files in this Directory

### 1. PHASE_0_IO_CONTRACT.md
The primary I/O contract specification document. This file contains:

- **Test Cases Table**: All 21 test cases organized by category with expected outputs
- **Implementation Details**: Operator mappings, precedence levels, and parenthesization rules
- **Output Format Specification**: Exact rules for formatting LaTeX output
- **Testing Results Summary**: Statistics on test execution
- **Unsupported Features**: Documentation of features not yet implemented (exponent operator)
- **Migration Guide**: Instructions for implementers in other languages

**Use this file when**: You need the authoritative specification of rpn2tex behavior

### 2. VERIFICATION_REPORT.md
Detailed test execution report showing:

- **Overall Statistics**: Pass/fail breakdown and success rates
- **Test Category Breakdown**: Performance by category (basic, chained, precedence, complex)
- **Individual Test Results**: Full details for all 21 tests with expected vs actual output
- **Verification Method**: How tests were executed
- **Key Findings**: Strengths and edge cases verified
- **Migration Recommendations**: Step-by-step guidance for implementation in other languages

**Use this file when**: You need evidence of verification or detailed analysis of test execution

### 3. TEST_DATA.json
Machine-readable test data in JSON format containing:

- **Metadata**: Test suite version, implementation name, results summary
- **Test Cases**: All 21 tests with input, expected output, actual output, and status
- **Operator Mappings**: Definition of each operator's properties (precedence, associativity, LaTeX symbol)
- **Output Format Rules**: Specification of formatting requirements

**Use this file when**: You're implementing automated testing or need to parse test data programmatically

## Quick Reference

### Test Statistics
- **Total Tests**: 21
- **Passing**: 18
- **Failing**: 0
- **Unsupported**: 3 (exponent operator `^`)
- **Success Rate**: 100% of supported features

### Operator Reference
| Input | Output | Precedence | Associativity |
|-------|--------|-----------|---------------|
| `+` | `+` | 1 (lower) | Left |
| `-` | `-` | 1 (lower) | Left |
| `*` | `\times` | 2 (higher) | Left |
| `/` | `\div` | 2 (higher) | Left |

### Output Format Rules
```
Pattern: $<expr>$
Spacing: Single space around operators
Parentheses: ( expr ) with spaces inside
Numbers: Preserved exactly as input
```

## Test Categories

### 1. Basic Operations (6 tests)
Simple binary operations with single operators and floating-point numbers.
- Examples: `5 3 +`, `3.14 2 *`
- All 6 tests passing

### 2. Chained Operations (3 tests)
Multiple operators of the same precedence level in sequence.
- Examples: `1 2 + 3 + 4 +`, `100 10 / 5 / 2 /`
- All 3 tests passing
- Verifies left-associativity

### 3. Operator Precedence (7 tests)
Tests for correct parenthesization when different precedence levels mix.
- Examples: `5 3 + 2 *`, `2 3 4 + *`
- All 7 tests passing
- Verifies precedence and right-operand handling

### 4. Complex Expressions (2 tests)
Multi-level nested operations combining multiple operator types.
- Examples: `1 2 + 3 4 + *`, `10 2 / 3 + 4 *`
- All 2 tests passing

### 5. Unsupported Features (3 tests)
Features documented for future implementation.
- Exponent operator: `2 3 ^`, `2 3 ^ 4 *`, `2 3 4 ^ ^`
- Status: UNSUPPORTED (expected)
- Error: Lexer does not recognize `^` character

## How to Use These Artifacts

### For Implementation in New Languages

1. **Start here**: Read PHASE_0_IO_CONTRACT.md for the specification
2. **Implement**: Follow the operator mappings and output format rules
3. **Test**: Use TEST_DATA.json to run all test cases
4. **Verify**: Compare your output exactly with expected_output field
5. **Document**: Reference the test case ID when reporting status

### For Validation

1. **Run all 18 supported tests** through your implementation
2. **Capture output** exactly as produced
3. **Compare** with expected_output field in TEST_DATA.json
4. **Report results** using the same format as VERIFICATION_REPORT.md
5. **Investigate failures** using the detailed test descriptions

### For Understanding the Algorithm

1. Read PHASE_0_IO_CONTRACT.md "Parenthesization Rules" section
2. Review VERIFICATION_REPORT.md "Key Findings" section
3. Study individual test cases that demonstrate the relevant behavior
4. Example for precedence: Tests 10-16 show parenthesization logic

## Implementation Checklist

When implementing rpn2tex in a new language, verify:

- [ ] Lexer correctly tokenizes input
- [ ] Parser uses stack-based RPN algorithm
- [ ] Operator precedence levels match specification (1 for +/-, 2 for */)
- [ ] Left-associativity enforced for all operators
- [ ] LaTeX operator symbols correct (\times, \div)
- [ ] Parentheses include spaces: `( expr )` not `(expr)`
- [ ] All 18 supported tests produce exact output matches
- [ ] Exponent operator (^) documented as unsupported for Phase 0

## Test Execution Environment

- **Implementation**: Python 3
- **Test Framework**: Custom Python test harness
- **Date**: 2025-12-30
- **Python Version**: 3.7+
- **Source Location**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
- **Test Script**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/verify_io_contract_extended.py`

## Support for Future Operators

These operators are documented for potential Phase 1+ implementation:

- **Exponent (^)**: Currently unsupported - would require parser changes
  - Example: `2 3 ^` should produce `$2^{3}$`
  - Precedence: Higher than all current operators
  - Associativity: Right-associative (2^3^4 = 2^(3^4))

## Files Referenced

- Python source code: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
  - `lexer.py` - Tokenization
  - `parser.py` - RPN parsing
  - `ast_nodes.py` - AST definitions
  - `latex_gen.py` - LaTeX generation
  - `cli.py` - Command-line interface

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-12-30 | Initial contract creation with 18 supported tests |

## Contact and Questions

For questions about this contract or verification issues:
1. Check PHASE_0_IO_CONTRACT.md "Notes for Migration"
2. Review VERIFICATION_REPORT.md "Key Findings"
3. Examine specific test case in TEST_DATA.json
