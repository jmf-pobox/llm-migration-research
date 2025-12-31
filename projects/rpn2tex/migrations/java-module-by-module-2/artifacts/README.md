# RPN2TEX I/O Contract - Complete Documentation

## Overview

This directory contains the complete I/O contract for the rpn2tex application, generated from the Python reference implementation. These contracts serve as the specification for validating Java, Go, and Rust migrations.

**Generation Date**: 2025-12-29
**Source Implementation**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source`
**Test Cases Executed**: 21
**Pass Rate**: 85.7% (18 pass, 3 expected errors)

## Files in This Directory

### 1. PHASE_0_IO_CONTRACT.md (PRIMARY SPECIFICATION)

**Purpose**: Main reference document for migration validation
**Format**: Markdown with structured tables
**Content**:
- Complete test case matrix (all 21 inputs and outputs)
- Error case documentation
- Operator support matrix
- LaTeX output format specification
- Behavioral notes and implementation details

**Use This For**:
- Understanding the complete specification
- Reference during implementation
- Documentation for your migration
- Sharing with team members

### 2. QUICK_REFERENCE.md (DEVELOPER GUIDE)

**Purpose**: Condensed reference for quick lookup
**Format**: Markdown with summary tables
**Content**:
- All test cases in single table format
- Operator reference chart
- Parenthesization rules with examples
- Error format specification
- Critical implementation points

**Use This For**:
- Quick lookups during development
- Understanding key rules at a glance
- Design decisions and implementation notes
- Testing strategy guidance

### 3. EXECUTION_SUMMARY.txt (TECHNICAL DETAILS)

**Purpose**: Detailed methodology and validation criteria
**Format**: Plain text with structured sections
**Content**:
- Execution methodology
- Implementation architecture
- Operator support matrix with precedence
- Parenthesization rule explanations
- Floating-point handling
- Critical behaviors to replicate
- Validation criteria for migrations

**Use This For**:
- Understanding implementation architecture
- Deep dive into the design
- Validation criteria checklist
- Architecture documentation

### 4. io_contract.txt (MACHINE-READABLE FORMAT)

**Purpose**: Structured data format for test automation
**Format**: Machine-readable structured text
**Content**:
- All 21 test cases with individual entries
- Status, input, output, category for each test
- Summary statistics
- Operator/precedence matrices
- Notes and implementation details

**Use This For**:
- Automated test harness integration
- Test case generation tools
- CI/CD pipeline integration
- Machine parsing and validation

## Test Case Summary

### Passing Tests (18)

| Category | Count | Examples |
|----------|-------|----------|
| Basic Operations | 4 | `5 3 +`, `4 7 *`, `10 2 /`, `5 3 -` |
| Precedence Handling | 10 | `5 3 + 2 *`, `2 3 4 * +`, `10 2 / 5 *` |
| Floating Point | 2 | `3.14 2 *`, `1.5 0.5 +` |
| Complex Expressions | 2 | `1 2 + 3 4 + *`, `10 2 / 3 + 4 *` |

### Error Cases (3 - Expected)

All 3 errors are for the unsupported exponentiation operator `^`:
- `2 3 ^` - Lexer rejects `^`
- `2 3 ^ 4 *` - Lexer rejects `^`
- `2 3 4 ^ ^` - Lexer rejects `^`

**Error Format**: `Line <N>, column <N>: Unexpected character '^'`

## How to Use These Contracts

### For Java Migration

1. **Setup**: Copy `PHASE_0_IO_CONTRACT.md` to your Java project
2. **Implementation**: Use as specification while implementing lexer, parser, and generator
3. **Testing**: Run your implementation against all 21 test cases
4. **Validation**: Compare outputs with expected values in the contract
5. **Verification**: All 18 passing tests must match exactly; all 3 error tests must produce identical errors

### For Go Migration

1. **Setup**: Copy `PHASE_0_IO_CONTRACT.md` to your Go project
2. **Implementation**: Use as specification while implementing lexer, parser, and generator
3. **Testing**: Run your implementation against all 21 test cases
4. **Validation**: Compare outputs with expected values in the contract
5. **Verification**: All 18 passing tests must match exactly; all 3 error tests must produce identical errors

### For Rust Migration

1. **Setup**: Copy `PHASE_0_IO_CONTRACT.md` to your Rust project
2. **Implementation**: Use as specification while implementing lexer, parser, and generator
3. **Testing**: Run your implementation against all 21 test cases
4. **Validation**: Compare outputs with expected values in the contract
5. **Verification**: All 18 passing tests must match exactly; all 3 error tests must produce identical errors

## Critical Specifications

### Supported Operators

| Operator | Symbol | Precedence | LaTeX Output |
|----------|--------|------------|--------------|
| Addition | + | Low | ` + ` |
| Subtraction | - | Low | ` - ` |
| Multiplication | * | High | ` \times ` |
| Division | / | High | ` \div ` |

### NOT Supported

- Exponentiation (`^`) - Must reject with LexerError

### Output Format

- **Wrapper**: LaTeX math mode delimiters `$ ... $`
- **Operators**: Space before and after each operator
- **Parentheses**: `( ` opening and ` )` closing with spaces
- **Numbers**: Integers and floats preserved exactly as input

### Parenthesization Rules

1. When a lower-precedence operation is an operand of a higher-precedence operation, wrap it in parentheses
2. When operations have the same precedence, no parentheses (left-to-right reading is sufficient)
3. When higher-precedence operations are at the top level, no parentheses needed

Examples:
- `5 3 + 2 *` → `$( 5 + 3 ) \times 2$` (add is lower precedence than multiply)
- `5 3 * 2 +` → `$5 \times 3 + 2$` (multiply is higher, no parens needed)
- `10 2 / 5 *` → `$10 \div 2 \times 5$` (same precedence, left-to-right)

## Implementation Architecture

The implementation follows a three-stage pipeline:

```
INPUT (RPN string)
  |
  v
LEXER (Tokenization)
  - Accepts: +, -, *, /, numbers, whitespace
  - Rejects: ^ (and other unsupported characters)
  |
  v
PARSER (AST Construction)
  - Builds expression tree from token stream
  - Validates RPN structure
  |
  v
GENERATOR (LaTeX Output)
  - Traverses AST
  - Applies precedence-based parenthesization
  - Produces LaTeX math mode string
  |
  v
OUTPUT (LaTeX string or ERROR)
```

## Validation Checklist for Migrations

Before considering a migration complete, verify:

- [ ] All 18 passing test cases produce exact output match
- [ ] All 3 error cases produce exact error message match
- [ ] Multiplication renders as ` \times ` (not \cdot, *, or other)
- [ ] Division renders as ` \div ` (not /, ÷, or other)
- [ ] Exponentiation (`^`) produces LexerError with line/column
- [ ] Floating-point numbers preserved digit-for-digit
- [ ] Parentheses format correct: `( ` and ` )`
- [ ] All operators have space before and after
- [ ] Output wrapped in `$ ... $` math mode delimiters
- [ ] Error messages follow format: `Line <N>, column <N>: <message>`
- [ ] Left-associativity implemented correctly
- [ ] Precedence rules (mult/div > add/sub) implemented

## Key Behavioral Notes

1. **No Evaluation**: The tool performs syntactic transformation from RPN to infix LaTeX. No arithmetic evaluation occurs.

2. **Exact Preservation**: All numbers are preserved exactly as input. No rounding, conversion, or simplification.

3. **Whitespace Handling**: Tokens are whitespace-separated in input; output has spaces around all operators.

4. **Left-Associativity**: All operators are left-associative (important for multiple operations of same precedence).

5. **Parenthesization**: Only added when mathematically necessary; no unnecessary parentheses.

## Common Pitfalls to Avoid

1. **Wrong Operator Symbols**
   - Wrong: `*` instead of `\times`, `/` instead of `\div`
   - Right: Always use `\times` and `\div` in LaTeX output

2. **Missing Spaces**
   - Wrong: `$5+3$` or `$(5+3)*2$`
   - Right: `$5 + 3$` or `$( 5 + 3 ) \times 2$`

3. **Over-Parenthesization**
   - Wrong: `$( 5 \times 3 ) + 2$` (unnecessary parens)
   - Right: `$5 \times 3 + 2$` (parens not needed)

4. **Under-Parenthesization**
   - Wrong: `$5 + 3 \times 2$` (misleading, looks like 11 not 16)
   - Right: `$( 5 + 3 ) \times 2$` (correct precedence shown)

5. **Floating-Point Rounding**
   - Wrong: Converting `3.14` or `1.5` to different representation
   - Right: Preserve exactly as `3.14` and `1.5`

6. **Supporting `^` Operator**
   - Wrong: Accepting `^` and generating LaTeX
   - Right: Reject `^` with LexerError

## Questions and Support

For questions about the specification:
- Refer to `QUICK_REFERENCE.md` for common patterns
- Check `EXECUTION_SUMMARY.txt` for detailed technical notes
- Review `PHASE_0_IO_CONTRACT.md` for complete specification
- Run the Python reference implementation to test behavior

---

**Contract Version**: 1.0
**Last Updated**: 2025-12-29
**Status**: Ready for migration implementation and validation
