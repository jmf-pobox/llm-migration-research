# I/O Contract Documentation

## Quick Reference

The I/O contract defines the expected input-output behavior of the rpn2tex implementation. This documentation enables validation of migrations to other programming languages.

**Status:** COMPLETE AND VALIDATED
**All 21 test cases verified:** PASS

## Files

### 1. io_contract.md (5.7 KB)
Human-readable comprehensive contract with:
- 21 test cases in detailed table format
- Error case documentation
- Operator and operand support specifications
- LaTeX output format reference
- Implementation characteristics
- Migration guidelines

**Use for:** Reference documentation, understanding expected behavior, migration planning

### 2. io_contract.json (4.2 KB)
Machine-readable JSON contract with:
- Metadata about the source implementation
- 21 structured test cases with inputs, expected outputs, and exit codes
- Summary statistics
- Easy programmatic access to test data

**Use for:** Automated test harness development, validation scripts, CI/CD integration

### 3. IO_CONTRACT_SUMMARY.md (This directory)
Complete report including:
- Executive summary
- Test results breakdown
- Implementation details and module information
- Output format specifications
- Key observations about precedence and associativity
- Migration checklist
- Next steps

**Use for:** Understanding the full context, migration planning, team communication

## Quick Start

To understand the expected behavior:

1. **For a quick overview:** Read the test cases in `io_contract.md` (lines 21-47)
2. **For implementation details:** Review "Implementation Characteristics" section in `io_contract.md`
3. **For automation:** Use `io_contract.json` in your test validation scripts

## Test Coverage

```
Total:      21 test cases
Success:    18 (85.7%)
Errors:      3 (14.3%)
```

All successful test cases generate valid LaTeX expressions. All error cases involve unsupported operators (caret/^).

## Example Test Cases

### Basic Operations
- `5 3 +` → `$5 + 3$` (addition)
- `4 7 *` → `$4 \times 7$` (multiplication)
- `3.14 2 *` → `$3.14 \times 2$` (floating-point)

### Precedence and Grouping
- `5 3 + 2 *` → `$( 5 + 3 ) \times 2$` (lower-precedence grouped)
- `2 3 4 * +` → `$2 + 3 \times 4$` (higher-precedence not grouped)
- `1 2 + 3 4 + *` → `$( 1 + 2 ) \times ( 3 + 4 )$` (both sides grouped)

### Error Cases
- `2 3 ^` → LexerError: "Unexpected character '^'"

## Implementation Location

**Source:** `/Users/jfreeman/Coding/rpn2tex/src/rpn2tex/`

Key modules:
- `cli.py` - Command-line interface entry point
- `lexer.py` - Tokenization (identifies operators and operands)
- `parser.py` - RPN expression parsing
- `latex_gen.py` - LaTeX code generation
- `errors.py` - Error handling and formatting

## Supported Operations

```
Addition:       + → $+$
Subtraction:    - → $-$
Multiplication: * → $\times$
Division:       / → $\div$
Exponentiation: ^ → NOT SUPPORTED (LexerError)
```

## Validation History

- **Created:** December 27, 2025
- **Tests Run:** 21 input expressions through Python implementation
- **Validation:** All 21 test cases re-verified for accuracy
- **Result:** 100% match with contract specification

## Using the Contract for Migrations

### For Java Migration
1. Review `io_contract.md` for expected behavior
2. Run your Java implementation against each test case in `io_contract.json`
3. Ensure outputs match exactly (including LaTeX symbols and spacing)
4. Document any intentional deviations in your migration notes

### For Rust Migration
1. Load test cases from `io_contract.json`
2. Run each input through your Rust implementation
3. Compare outputs programmatically
4. Verify error handling matches expected messages

### For Other Languages
Same process as above - the JSON contract is language-agnostic.

## Key Behavioral Points to Preserve

1. **Operator Precedence:** Multiplication/Division before Addition/Subtraction
2. **Associativity:** All operators are left-associative
3. **Parenthesization:** Added only when needed for correctness
4. **LaTeX Format:** Always wrapped in `$...$` delimiters
5. **Symbol Mapping:** Use `\times` and `\div` for mult/div
6. **Spacing:** Single spaces around operators
7. **Floating-Point:** Support decimal numbers like 3.14, 1.5
8. **Error Messages:** Include location info (line, column) and visual indicator

## Contact & Questions

Refer to the implementation source at `/Users/jfreeman/Coding/rpn2tex/` for detailed code documentation.

---

Generated: December 27, 2025
Contract Version: 1.0
Status: VALIDATED
