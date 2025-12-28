# I/O Contract Generation Report

## Executive Summary

Successfully generated a comprehensive I/O contract for the rpn2tex Python implementation by running all 21 test inputs through the actual implementation. The contract captures exact input-output mappings for validation of migrations to other languages.

**Status:** COMPLETE
**Test Date:** December 27, 2025
**Test Coverage:** 21 test cases (18 successful, 3 error cases)

## Generated Artifacts

### 1. io_contract.md
**Path:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/io_contract.md`

Comprehensive markdown document containing:
- Detailed test case table with inputs, expected outputs, and notes
- Error case documentation with error types and messages
- Operator support matrix
- Operand support analysis
- LaTeX output format specification
- Implementation characteristics analysis
- Migration guidelines for target languages

### 2. io_contract.json
**Path:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/io_contract.json`

Machine-readable JSON contract for programmatic validation:
- Metadata about the source implementation
- Structured test cases with exit codes and error messages
- Summary statistics
- Suitable for automated test harness integration

## Test Results Summary

```
Total Test Cases:     21
Successful:           18 (85.7%)
Error Cases:           3 (14.3%)
```

### Success Cases (18 total)

All basic arithmetic operations work correctly:
- Basic operations: addition, subtraction, multiplication, division
- Complex expressions: proper precedence handling and parenthesization
- Floating-point operands: fully supported
- Operator chaining: proper left-associativity

**Example outputs:**
- `5 3 +` → `$5 + 3$`
- `5 3 + 2 *` → `$( 5 + 3 ) \times 2$`
- `3.14 2 *` → `$3.14 \times 2$`

### Error Cases (3 total)

All three error cases involve the unsupported caret operator (^):

| Input | Error | Location |
|-------|-------|----------|
| `2 3 ^` | Unexpected character '^' | Line 1, Column 5 |
| `2 3 ^ 4 *` | Unexpected character '^' | Line 1, Column 5 |
| `2 3 4 ^ ^` | Unexpected character '^' | Line 1, Column 7 |

**Error Type:** LexerError
**Error Message Format:** `Error: Unexpected character '<char>'` with visual indicator

## Implementation Details

### Source Location
- **Path:** `/Users/jfreeman/Coding/rpn2tex/src/rpn2tex/`
- **Entry Point:** `cli.py` (main function)
- **CLI Module:** `rpn2tex.cli`
- **Usage:** `python3 -m rpn2tex.cli - < input.txt`

### Core Modules
- `lexer.py` - Tokenization (does not recognize ^ operator)
- `parser.py` - RPN parsing and AST construction
- `latex_gen.py` - LaTeX code generation
- `ast_nodes.py` - AST node definitions
- `errors.py` - Error handling and formatting

### Supported Features

**Operators:**
- Addition (+) → `+`
- Subtraction (-) → `-`
- Multiplication (*) → `\times`
- Division (/) → `\div`
- Exponentiation (^) → NOT SUPPORTED

**Operands:**
- Integer numbers (e.g., 5, 10, 100)
- Floating-point numbers (e.g., 3.14, 1.5, 0.5)

**Operators Precedence:**
1. Multiplication and Division (higher precedence)
2. Addition and Subtraction (lower precedence)

**Associativity:** Left-associative for all binary operators

## Output Format Specifications

### LaTeX Math Mode
All outputs are wrapped in LaTeX math delimiters:
```
$<expression>$
```

### Parenthesization Rules
- Parentheses are added when lower-precedence operations are operands to higher-precedence operations
- Format: `( ... )` with surrounding spaces
- Example: `$( 5 + 3 ) \times 2$`

### Spacing
- Operators are surrounded by single spaces
- Examples: `5 + 3`, `5 \times 3`, `5 \div 2`

## Test Execution Method

Each test was executed as follows:

```bash
echo "<input>" | python3 -m rpn2tex.cli -
```

Where:
- `<input>` is the RPN expression
- `-` tells the CLI to read from stdin
- Output is captured from stdout
- Errors (if any) are captured from stderr

## Validation Use Cases

This I/O contract can be used to:

1. **Migrate to Other Languages:** Ensure migrations to Java, Rust, Go, etc. produce identical outputs
2. **Regression Testing:** Validate that changes to the Python implementation don't break existing behavior
3. **Reference Implementation:** Define authoritative behavior for new language implementations
4. **Error Testing:** Validate that error conditions are handled consistently
5. **Performance Baselines:** Compare execution time across language implementations

## Key Observations

### Precedence Handling
The implementation correctly implements standard mathematical operator precedence:
- `2 3 4 * +` → `$2 + 3 \times 4$` (multiplication before addition)
- `2 3 + 4 *` → `$( 2 + 3 ) \times 4$` (parentheses added to preserve semantics)

### Associativity
All operators are left-associative:
- `5 3 - 2 -` → `$5 - 3 - 2$` (not `5 - (3 - 2)`)
- `100 10 / 5 / 2 /` → `$100 \div 10 \div 5 \div 2$` (not right-associative)

### Error Handling
- Comprehensive error messages with line and column information
- Visual error indicators showing the exact position
- Clear error classification (LexerError vs ParserError)

### Floating-Point Support
- Numbers with decimal points are properly tokenized
- Floating-point operands work with all operators
- No apparent precision issues in the tested range

## Migration Checklist

When migrating rpn2tex to other languages, use this checklist:

- [ ] All 18 successful test cases produce identical LaTeX output
- [ ] Output strings match exactly (including spaces and LaTeX commands)
- [ ] All 3 error cases produce appropriate error messages
- [ ] Error messages include location information (line and column)
- [ ] Operator precedence is correctly implemented
- [ ] Operator associativity is left-associative
- [ ] Parenthesization logic matches the original (added when needed)
- [ ] Floating-point number parsing is correct
- [ ] Caret operator (^) handling is consistent (error or feature)
- [ ] CLI accepts stdin input via "-" argument
- [ ] Exit codes are correct (0 for success, 1 for error)

## Files Generated

```
/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/
├── io_contract.md          (Comprehensive markdown contract)
├── io_contract.json        (Machine-readable JSON contract)
└── IO_CONTRACT_SUMMARY.md  (This file)
```

## Next Steps

1. Use `io_contract.md` for human-readable reference
2. Use `io_contract.json` for automated validation testing
3. Run migrations against these contracts to verify correctness
4. Add any new test cases discovered during migration to both files
5. Version control these contracts for ongoing validation

