# I/O Contract Documentation Index

## Overview

This directory contains the complete I/O contract specification for the rpn2tex Python to Go migration. The contract defines the exact behavior, inputs, and outputs that the Go implementation must replicate to achieve behavioral equivalence.

## Files

### 1. `io_contract.md` (Primary Reference)

**Purpose**: Core I/O contract specification

**Contents**:
- Overview of the rpn2tex system architecture
- 18 passing test cases with expected outputs
- 3 error cases with expected error messages
- Critical behavioral specifications for:
  - Operator support and symbols
  - Operator precedence levels
  - Output formatting rules
  - Numeric type handling
  - Error handling and reporting
  - Whitespace processing
  - Line and column tracking
  - LaTeX command specifications
  - Parenthesization algorithm
- Implementation pipeline documentation (Lexer → Parser → Generator → CLI)
- AST structure reference
- Additional recommended test cases

**Use this document when**:
- Validating specific test case outputs
- Understanding precedence rules
- Implementing parenthesization logic
- Working on error handling
- Checking LaTeX output formatting

### 2. `PHASE_0_IO_CONTRACT.md` (Detailed Log)

**Purpose**: Detailed test execution log and migration guidance

**Contents**:
- Executive summary table (all 21 tests at a glance)
- Detailed results for each of 21 test cases
- Critical implementation details specific to Go
- Operator precedence reference table
- Validation criteria for the Go implementation
- Notes on unsupported features
- File location reference
- Module-by-module breakdown of Python source

**Use this document when**:
- Running the test suite
- Debugging individual test failures
- Understanding the exact implementation details needed for Go
- Verifying migration completeness
- Checking that no features are incorrectly implemented

### 3. `README_IO_CONTRACT.md` (This File)

**Purpose**: Navigation and quick reference

## Quick Reference

### Test Results Summary

| Category | Count | Status |
|----------|-------|--------|
| Passing tests | 18 | All outputs verified |
| Error cases | 3 | All error formats verified |
| **Total** | **21** | **100% coverage** |

### Supported Operations

```
Input Format: RPN (Reverse Polish Notation)
Output Format: LaTeX math mode

Operators:
  +  (addition)       → $...$
  -  (subtraction)    → $...$
  *  (multiplication) → $... \times ...$
  /  (division)       → $... \div ...$
```

### Error Handling

```
Invalid input: Exponentiation (^) and any unknown character
Response: LexerError with formatted source line and error pointer
Output stream: stderr
Exit code: 1
```

### Precedence Levels

```
Level 1 (Lower):  + (addition), - (subtraction)
Level 2 (Higher): * (multiplication), / (division)

All operators: Left-associative
```

### Output Format Rules

```
Basic format: $<expression>$

Spaces:
- Single space between all tokens
- Spaces inside parentheses: ( <expr> )

Parentheses added when:
1. Child has lower precedence than parent
2. Child has equal precedence and:
   - Is on the right side of - or /
   - (NOT on the left side - left-associative)
```

## Test Case Categories

### Basic Operations (4 cases)
- Simple addition: `5 3 +` → `$5 + 3$`
- Simple subtraction: `5 3 -` → `$5 - 3$`
- Simple multiplication: `4 7 *` → `$4 \times 7$`
- Simple division: `10 2 /` → `$10 \div 2$`

### Precedence & Parenthesization (9 cases)
- Lower precedence child: `5 3 + 2 *` → `$( 5 + 3 ) \times 2$`
- Higher precedence naturally: `5 3 * 2 +` → `$5 \times 3 + 2$`
- Left-associative chains: `100 10 / 5 / 2 /` → `$100 \div 10 \div 5 \div 2$`
- Multiple operations: `2 3 4 * +`, `2 3 + 4 *`, `2 3 4 + *`, etc.

### Decimal Numbers (2 cases)
- Decimal multiplication: `3.14 2 *` → `$3.14 \times 2$`
- Decimal addition: `1.5 0.5 +` → `$1.5 + 0.5$`

### Complex Expressions (2 cases)
- Multiple groups: `1 2 + 3 4 + *` → `$( 1 + 2 ) \times ( 3 + 4 )$`
- Mixed precedence: `10 2 / 3 + 4 *` → `$( 10 \div 2 + 3 ) \times 4$`

### Error Cases (3 cases)
- Simple exponentiation: `2 3 ^` → LexerError
- Exponentiation in middle: `2 3 ^ 4 *` → LexerError
- Multiple exponents: `2 3 4 ^ ^` → LexerError

## Python Source Code Organization

```
/projects/rpn2tex/source/
├── cli.py              # Entry point and orchestration
├── lexer.py            # Tokenization (text → tokens)
├── parser.py           # Parsing (tokens → AST)
├── latex_gen.py        # Generation (AST → LaTeX)
├── ast_nodes.py        # AST node definitions
├── tokens.py           # Token types
├── errors.py           # Error formatting
└── __init__.py         # Module exports
```

## Critical Points for Go Implementation

### 1. Exact Output Matching
The Go implementation must produce **byte-for-byte identical output**, including:
- Exact spacing (single spaces between all tokens)
- Exact parenthesis formatting with spaces inside
- Exact LaTeX commands (`\times` and `\div` with single backslash)
- Exact error messages and formatting

### 2. Operator Precedence
Implement the exact algorithm from `latex_gen.py`:
```python
def needs_parens(child, parent_precedence, is_right):
    if child is not BinaryOp:
        return False

    child_prec = precedence[child.operator]

    # Lower precedence always needs parens
    if child_prec < parent_prec:
        return True

    # Equal precedence on right needs parens for - and /
    return (child_prec == parent_prec and
            is_right and
            child.operator in {"-", "/"})
```

### 3. Number as Strings
Numbers must be stored and output as strings to preserve exact formatting:
- `3.14` stays as `3.14`
- `5` stays as `5`
- Decimal points are preserved exactly

### 4. Error Handling
Error messages must:
- Be written to stderr (not stdout)
- Use exact text: `Error: Unexpected character '<char>'`
- Include visual pointer with exact formatting
- Exit with code 1 (success exits with code 0)

### 5. Stream Separation
- **stdout**: Only the LaTeX result
- **stderr**: Only error messages (no LaTeX output on error)

## Running Tests Against the Python Implementation

To verify this contract against the Python source:

```bash
cd /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex

# Simple addition test
echo "5 3 +" | python -m source.cli -

# Error case test
echo "2 3 ^" | python -m source.cli - 2>&1

# Complex expression test
echo "1 2 + 3 4 + *" | python -m source.cli -
```

## Validation Checklist for Go Implementation

Use this checklist to validate the Go implementation:

- [ ] All 18 passing test cases produce identical stdout
- [ ] All 3 error cases produce identical error messages
- [ ] Exit codes are correct (0 for success, 1 for error)
- [ ] Parenthesization matches exactly (with spaces: `( expr )`)
- [ ] LaTeX operators are correct (`\times` and `\div` with backslash)
- [ ] Decimal numbers preserve formatting exactly
- [ ] Error messages go to stderr, not stdout
- [ ] Error pointer positions are correct (1-based columns)
- [ ] Whitespace handling is identical
- [ ] Left-associativity is correct for chains of operators
- [ ] Negative number handling is correct (vs subtraction operator)
- [ ] No output to stdout when an error occurs

## Document Updates

This contract was generated on **December 29, 2024** by running all 21 test cases against the Python reference implementation.

**Generation Method**: Direct CLI invocation via stdin with stdout/stderr capture

**Verification**: All sample outputs cross-verified against actual Python execution

## Related Documentation

- **Source code**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
- **Go implementation**: (To be created)
- **Test runner**: (To be created)
- **Migration status**: See parent directory for progress tracking

## Contact & Questions

For questions about this contract:
1. Review the detailed notes in each test case
2. Consult the "Critical Behavioral Specifications" section in `io_contract.md`
3. Check the implementation details in `PHASE_0_IO_CONTRACT.md`
4. Examine the actual Python source code in the source directory
