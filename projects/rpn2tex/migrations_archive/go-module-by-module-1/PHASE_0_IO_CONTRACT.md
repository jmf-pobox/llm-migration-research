# PHASE_0: I/O Contract for rpn2tex Python Implementation

**Date Generated**: December 29, 2024
**Source**: Python rpn2tex implementation in `/projects/rpn2tex/source/`
**Test Method**: Direct CLI invocation via stdin
**Total Test Cases**: 21
**Passing Cases**: 18
**Error Cases**: 3

## Summary Table

| # | Input | Status | Output | Exit Code |
|---|-------|--------|--------|-----------|
| 1 | `5 3 +` | PASS | `$5 + 3$` | 0 |
| 2 | `5 3 -` | PASS | `$5 - 3$` | 0 |
| 3 | `4 7 *` | PASS | `$4 \times 7$` | 0 |
| 4 | `10 2 /` | PASS | `$10 \div 2$` | 0 |
| 5 | `2 3 ^` | ERROR | Lexer error: unsupported `^` operator | 1 |
| 6 | `5 3 + 2 *` | PASS | `$( 5 + 3 ) \times 2$` | 0 |
| 7 | `5 3 * 2 +` | PASS | `$5 \times 3 + 2$` | 0 |
| 8 | `10 2 / 5 *` | PASS | `$10 \div 2 \times 5$` | 0 |
| 9 | `5 3 - 2 -` | PASS | `$5 - 3 - 2$` | 0 |
| 10 | `100 10 / 5 / 2 /` | PASS | `$100 \div 10 \div 5 \div 2$` | 0 |
| 11 | `1 2 + 3 + 4 +` | PASS | `$1 + 2 + 3 + 4$` | 0 |
| 12 | `2 3 4 * +` | PASS | `$2 + 3 \times 4$` | 0 |
| 13 | `2 3 + 4 *` | PASS | `$( 2 + 3 ) \times 4$` | 0 |
| 14 | `2 3 4 + *` | PASS | `$2 \times ( 3 + 4 )$` | 0 |
| 15 | `2 3 * 4 +` | PASS | `$2 \times 3 + 4$` | 0 |
| 16 | `2 3 ^ 4 *` | ERROR | Lexer error: unsupported `^` operator | 1 |
| 17 | `2 3 4 ^ ^` | ERROR | Lexer error: unsupported `^` operator | 1 |
| 18 | `3.14 2 *` | PASS | `$3.14 \times 2$` | 0 |
| 19 | `1.5 0.5 +` | PASS | `$1.5 + 0.5$` | 0 |
| 20 | `1 2 + 3 4 + *` | PASS | `$( 1 + 2 ) \times ( 3 + 4 )$` | 0 |
| 21 | `10 2 / 3 + 4 *` | PASS | `$( 10 \div 2 + 3 ) \times 4$` | 0 |

## Detailed Test Results

### Test 1: Simple Addition
```
Input:  "5 3 +"
Output: "$5 + 3$"
Exit:   0
```

### Test 2: Simple Subtraction
```
Input:  "5 3 -"
Output: "$5 - 3$"
Exit:   0
```

### Test 3: Simple Multiplication
```
Input:  "4 7 *"
Output: "$4 \times 7$"
Exit:   0
Notes:  Uses \times LaTeX command
```

### Test 4: Simple Division
```
Input:  "10 2 /"
Output: "$10 \div 2$"
Exit:   0
Notes:  Uses \div LaTeX command
```

### Test 5: Unsupported Exponentiation
```
Input:  "2 3 ^"
Stderr: Error: Unexpected character '^'

        1 | 2 3 ^
          |     ^
Exit:   1
Notes:  Exponentiation operator not supported in Python implementation
```

### Test 6: Addition then Multiplication
```
Input:  "5 3 + 2 *"
Output: "$( 5 + 3 ) \times 2$"
Exit:   0
Notes:  Parentheses added because addition (precedence 1) is operand of multiplication (precedence 2)
```

### Test 7: Multiplication then Addition
```
Input:  "5 3 * 2 +"
Output: "$5 \times 3 + 2$"
Exit:   0
Notes:  No parentheses needed; multiplication naturally has higher precedence
```

### Test 8: Division then Multiplication
```
Input:  "10 2 / 5 *"
Output: "$10 \div 2 \times 5$"
Exit:   0
Notes:  Left-associative: (10 / 2) * 5; no parentheses needed for left operand
```

### Test 9: Subtraction then Subtraction
```
Input:  "5 3 - 2 -"
Output: "$5 - 3 - 2$"
Exit:   0
Notes:  Left-associative: (5 - 3) - 2; equal precedence on left doesn't need parentheses
```

### Test 10: Chain of Divisions
```
Input:  "100 10 / 5 / 2 /"
Output: "$100 \div 10 \div 5 \div 2$"
Exit:   0
Notes:  Left-associative: (((100 / 10) / 5) / 2); no parentheses for left-associative chain
```

### Test 11: Chain of Additions
```
Input:  "1 2 + 3 + 4 +"
Output: "$1 + 2 + 3 + 4$"
Exit:   0
Notes:  Commutative and associative; no parentheses needed
```

### Test 12: Mixed Multiplication and Addition (mult higher precedence)
```
Input:  "2 3 4 * +"
Output: "$2 + 3 \times 4$"
Exit:   0
Notes:  Multiplication has higher precedence; addition is parent; no parentheses needed for addition on right
```

### Test 13: Addition then Multiplication
```
Input:  "2 3 + 4 *"
Output: "$( 2 + 3 ) \times 4$"
Exit:   0
Notes:  Addition (precedence 1) is left operand of multiplication (precedence 2); needs parentheses
```

### Test 14: Multiplication with Addition on Right
```
Input:  "2 3 4 + *"
Output: "$2 \times ( 3 + 4 )$"
Exit:   0
Notes:  Addition (precedence 1) is right operand of multiplication (precedence 2); needs parentheses
```

### Test 15: Multiplication then Addition
```
Input:  "2 3 * 4 +"
Output: "$2 \times 3 + 4$"
Exit:   0
Notes:  Multiplication has higher precedence; no parentheses needed
```

### Test 16: Exponentiation in Middle
```
Input:  "2 3 ^ 4 *"
Stderr: Error: Unexpected character '^'

        1 | 2 3 ^ 4 *
          |     ^
Exit:   1
Notes:  Same error as Test 5; caret not supported
```

### Test 17: Multiple Exponentiations
```
Input:  "2 3 4 ^ ^"
Stderr: Error: Unexpected character '^'

        1 | 2 3 4 ^ ^
          |       ^
Exit:   1
Notes:  Lexer stops at first invalid character (column 7)
```

### Test 18: Decimal Multiplication
```
Input:  "3.14 2 *"
Output: "$3.14 \times 2$"
Exit:   0
Notes:  Decimal point preserved as-is in output
```

### Test 19: Decimal Addition
```
Input:  "1.5 0.5 +"
Output: "$1.5 + 0.5$"
Exit:   0
Notes:  Multiple decimals supported; each preserves decimal notation
```

### Test 20: Two Additions then Multiplication
```
Input:  "1 2 + 3 4 + *"
Output: "$( 1 + 2 ) \times ( 3 + 4 )$"
Exit:   0
Notes:  Both operands of multiplication are additions; both need parentheses
        Precedence rules apply symmetrically to both sides
```

### Test 21: Complex Precedence
```
Input:  "10 2 / 3 + 4 *"
Output: "$( 10 \div 2 + 3 ) \times 4$"
Exit:   0
Notes:  Division then addition (both lower precedence than multiplication)
        The combined (div + add) is left operand of multiplication; needs parentheses
        But division and addition are evaluated left-to-right with division first (higher precedence)
```

## Critical Implementation Details for Go Migration

### 1. LaTeX Output Format

All operators use specific LaTeX commands:
- Addition: plain `+`
- Subtraction: plain `-`
- Multiplication: `\times` (backslash-times)
- Division: `\div` (backslash-div)

When implementing in Go, ensure:
- String literals use proper escaping: `"\\times"` for the backslash
- Output is not post-processed; these are the exact strings used

### 2. Parenthesization Rules

The Python implementation uses a sophisticated precedence-based parenthesization algorithm:

```
If child is BinaryOp:
  - If child_precedence < parent_precedence: ADD PARENS
  - If child_precedence == parent_precedence AND is_right_child AND parent in {"-", "/"}: ADD PARENS
  - Otherwise: NO PARENS

If child is Number: NEVER ADD PARENS
```

This correctly handles left-associativity for subtraction and division.

### 3. Number Handling

Numbers are stored and output as strings. This means:
- `3.14` outputs as `3.14` (exact preservation)
- Integers like `5` output as `5`
- Leading zeros: `007` would output as `007` (though not tested)
- Negative numbers: `-5 3 +` would work if lexer recognizes `-5` as a single token

### 4. Whitespace Normalization

The implementation completely normalizes away whitespace:
- Multiple spaces: `5  3  +` → `$5 + 3$` (exactly one space between tokens in output)
- Tabs and newlines: treated as regular whitespace delimiters
- This is all handled in the lexer; the parser and generator see clean token streams

### 5. Error Messages Format

Errors have a specific three-line format:
```
Error: <message>

<line_num> | <source_line>
<padding>  | <pointer>
```

The pointer line has:
- Leading spaces matching the width of `<line_num> | `
- A caret `^` positioned at the error column (1-based)

### 6. Exit Codes

- Success (all valid expressions): exit code 0
- Any error (lexer, parser, or file I/O): exit code 1

### 7. stdout vs stderr

- **stdout**: LaTeX output only (for successful processing)
- **stderr**: Error messages, file write confirmations, and other status info

This separation is critical for piping and scripting.

## Operator Precedence Table

| Operator | Precedence Level | Associativity | LaTeX Command |
|----------|------------------|---------------|---------------|
| `+` | 1 (lowest) | Left | `+` |
| `-` | 1 (lowest) | Left | `-` |
| `*` | 2 (highest) | Left | `\times` |
| `/` | 2 (highest) | Left | `\div` |

Higher precedence number means tighter binding (multiplication/division bind tighter than addition/subtraction).

## File Locations

- **Source implementation**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
  - `cli.py`: CLI entry point and orchestration
  - `lexer.py`: Tokenization (text → tokens)
  - `parser.py`: Parsing (tokens → AST)
  - `latex_gen.py`: Generation (AST → LaTeX)
  - `ast_nodes.py`: AST node definitions
  - `tokens.py`: Token type definitions
  - `errors.py`: Error formatting utilities

- **This contract**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/go-module-by-module-1/io_contract.md`

- **Test execution log**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/go-module-by-module-1/PHASE_0_IO_CONTRACT.md` (this file)

## Validation Criteria for Go Implementation

The Go implementation is considered correct if and only if:

1. **All 18 passing test cases** produce byte-for-byte identical output to the Python implementation
2. **All 3 error cases** produce error messages with:
   - Identical error text
   - Identical source line formatting
   - Identical error pointer positioning
   - Exit code 1
3. **Formatting is exact**:
   - No extra/missing spaces in output
   - Parentheses format: `( <expr> )` with spaces inside
   - LaTeX commands properly formatted with backslashes
4. **Exit codes**: 0 for success, 1 for any error
5. **Stream separation**: LaTeX to stdout, errors to stderr

## Notes on Unsupported Features

The Python implementation explicitly does NOT support:
- Exponentiation (`^` operator) - returns Lexer error
- Unary operators (unary minus, unary plus)
- Functions (sqrt, sin, cos, etc.)
- Multiple-character operators
- String expressions or variables
- Special constants (pi, e, etc.)

The Go implementation should maintain this exact same feature set and error behavior.
