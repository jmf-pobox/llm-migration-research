# Java Module-by-Module Migration (Phase 0: I/O Contract)

## Overview

This directory contains the I/O contract for validating the Java migration of rpn2tex. The contract defines the expected behavior of the original Python implementation and serves as the source of truth for testing Java implementations.

**Migration Target**: Java
**Reference Implementation**: Python (rpn2tex)
**Contract Generation Date**: 2025-12-30

---

## Document Structure

### Primary Contract Files

#### 1. PHASE_0_IO_CONTRACT.md (Recommended)
**Location**: `artifacts/PHASE_0_IO_CONTRACT.md`
**Format**: Markdown
**Size**: 288 lines

Comprehensive I/O contract with:
- Detailed test case descriptions (21 total)
- LaTeX output format specifications
- Parenthesization rules and precedence logic
- Error handling documentation
- Validation checklist for implementations
- Summary table of all test cases

Use this for:
- Understanding the complete specification
- Detailed implementation guidance
- Reference documentation
- Human review and verification

#### 2. io_contract.txt
**Location**: `io_contract.txt`
**Format**: Plain text
**Size**: 205 lines

Structured plain text contract with:
- Test cases in simple key-value format
- Quick reference summary table
- Validation criteria
- Machine-parseable structure

Use this for:
- Automated test generation
- Script-based validation
- Quick lookups
- Integration with test frameworks

---

## Test Coverage Summary

**Total Test Cases**: 21
- **Success Cases**: 18 (Exit code 0)
- **Error Cases**: 3 (Exit code 1)

### Success Cases

| Category | Count | Examples |
|----------|-------|----------|
| Basic Operations | 4 | `5 3 +`, `5 3 -`, `4 7 *`, `10 2 /` |
| Floating Point | 2 | `3.14 2 *`, `1.5 0.5 +` |
| Complex Expressions | 12 | `5 3 + 2 *`, `2 3 4 + *`, etc. |

### Error Cases

| Input | Error | Reason |
|-------|-------|--------|
| `2 3 ^` | Lexer error | Unsupported operator |
| `2 3 ^ 4 *` | Lexer error | Unsupported operator |
| `2 3 4 ^ ^` | Lexer error | Unsupported operator |

---

## Specification Highlights

### Supported Operations

- Addition: `+` → LaTeX: `+`
- Subtraction: `-` → LaTeX: `-`
- Multiplication: `*` → LaTeX: `\times`
- Division: `/` → LaTeX: `\div`

### Not Supported

- Exponentiation: `^` → Error: "Unexpected character '^'"

### Input Format

- RPN notation with space-separated tokens
- Integer numbers (e.g., `5`, `100`)
- Decimal numbers (e.g., `3.14`, `1.5`)
- Whitespace-separated tokens

### Output Format

- Inline LaTeX math mode: `$...$`
- Space-separated terms for readability
- Parentheses inserted only when needed for precedence
- Decimal precision preserved exactly

### Error Handling

- Exit code `0` on success
- Exit code `1` on any error
- Error messages include location information (line and column)
- Lexer validates all characters

---

## Using This Contract

### For Implementation

1. Read `PHASE_0_IO_CONTRACT.md` section "Specification Highlights"
2. Review the "LaTeX Output Format Details" section
3. Study the "Parenthesization Rules" section
4. Implement operators and error handling accordingly

### For Testing

1. Run all 21 test cases from the contract
2. Compare exact output (character-by-character)
3. Verify exit codes match (0 or 1)
4. Check error messages for unsupported operators
5. Validate decimal number preservation

### For Validation

Use the provided validation checklist in `PHASE_0_IO_CONTRACT.md`:
- [ ] All 18 success cases produce EXACT LaTeX output
- [ ] All 3 error cases produce lexer errors with exit code 1
- [ ] Error messages include character location information
- [ ] Decimal numbers are preserved in output
- [ ] LaTeX symbols are correct: `\times` for *, `\div` for /
- [ ] Parentheses inserted only when necessary
- [ ] Output wrapped in `$...$`
- [ ] No extra whitespace at beginning/end
- [ ] Exit codes correct: 0 for success, 1 for error

---

## Key Implementation Details

### Parenthesization Logic

Parentheses are added when:
- A lower-precedence operation is an operand to a higher-precedence operation

Example:
- `5 3 + 2 *` → `$( 5 + 3 ) \times 2$` (addition parenthesized)
- `5 3 * 2 +` → `$5 \times 3 + 2$` (no parentheses needed)

### Operator Precedence

Standard mathematical precedence:
1. Multiplication `*` and Division `/` (highest)
2. Addition `+` and Subtraction `-` (lowest)

### Associativity

All operators are left-associative:
- `5 3 - 2 -` → `$5 - 3 - 2$` (evaluated left to right)
- `10 2 / 5 *` → `$10 \div 2 \times 5$` (evaluated left to right)

---

## Error Message Format

When an invalid character is encountered, the error message includes:

```
Error: Unexpected character '<char>'
<line> | <input text>
        | <pointer to error position>
```

Example for `2 3 ^`:
```
Error: Unexpected character '^'

1 | 2 3 ^
  |     ^
```

---

## File Locations

```
projects/rpn2tex/migrations/java-module-by-module-3/
├── INDEX.md                                    (this file)
├── io_contract.txt                             (plain text contract)
├── artifacts/
│   └── PHASE_0_IO_CONTRACT.md                 (markdown contract)
├── logs/                                       (migration logs)
└── [other migration artifacts]
```

---

## Reference Information

### Source Implementation

**Language**: Python
**Location**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
**CLI Entry**: `source/cli.py`

Key modules:
- `lexer.py` - Tokenization and character validation
- `parser.py` - RPN parsing and AST generation
- `latex_gen.py` - LaTeX code generation
- `cli.py` - Command-line interface

### Test Methodology

- All tests executed via direct Python execution
- Input provided via stdin
- Output captured from stdout
- Errors captured from stderr
- Exit codes recorded

---

## Next Steps

1. Review both contract documents
2. Implement Java version following the specification
3. Run all 21 test cases against Java implementation
4. Compare outputs to this contract
5. Document any discrepancies
6. Iterate until all test cases pass

---

## Quick Reference: All Test Cases

| # | Input | Output | Code |
|---|-------|--------|------|
| 1 | `5 3 +` | `$5 + 3$` | 0 |
| 2 | `5 3 -` | `$5 - 3$` | 0 |
| 3 | `4 7 *` | `$4 \times 7$` | 0 |
| 4 | `10 2 /` | `$10 \div 2$` | 0 |
| 5 | `2 3 ^` | Error | 1 |
| 6 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | 0 |
| 7 | `5 3 * 2 +` | `$5 \times 3 + 2$` | 0 |
| 8 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | 0 |
| 9 | `5 3 - 2 -` | `$5 - 3 - 2$` | 0 |
| 10 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | 0 |
| 11 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | 0 |
| 12 | `2 3 4 * +` | `$2 + 3 \times 4$` | 0 |
| 13 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | 0 |
| 14 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | 0 |
| 15 | `2 3 * 4 +` | `$2 \times 3 + 4$` | 0 |
| 16 | `2 3 ^ 4 *` | Error | 1 |
| 17 | `2 3 4 ^ ^` | Error | 1 |
| 18 | `3.14 2 *` | `$3.14 \times 2$` | 0 |
| 19 | `1.5 0.5 +` | `$1.5 + 0.5$` | 0 |
| 20 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | 0 |
| 21 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | 0 |

---

## Support and Questions

For questions about:
- **Specification**: See PHASE_0_IO_CONTRACT.md sections on format and rules
- **Test cases**: See the detailed test case section in PHASE_0_IO_CONTRACT.md
- **Implementation**: Review the source implementation in the Python rpn2tex module
- **Validation**: Use the checklist provided in PHASE_0_IO_CONTRACT.md
