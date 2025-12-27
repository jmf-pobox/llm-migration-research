# I/O Contract for rpn2tex Python Implementation

## Overview
This document specifies the exact input-output behavior of the Python rpn2tex implementation. This contract serves as the specification for validating equivalent Rust implementation.

**Implementation Location:** `/Users/jfreeman/Coding/rpn2tex/src/rpn2tex/`

**Key Implementation Details:**
- CLI Entry Point: `rpn2tex.cli:main()`
- Tokenizer: Lexer class in `lexer.py`
- Parser: Stack-based RPN parser in `parser.py`
- Code Generator: LaTeX generator in `latex_gen.py`
- Supported Operators: `+`, `-`, `*`, `/` (4 binary operators only)
- Number Format: Integers and floating-point decimals (e.g., `3.14`)
- Exponentiation: NOT supported (caret `^` raises LexerError)

---

## Test Cases

### Category 1: Basic Binary Operations (Tests 1-4)

| # | Input | Expected Output | Notes |
|---|-------|-----------------|-------|
| 1 | `5 3 +` | `$5 + 3$` | Basic addition |
| 2 | `5 3 -` | `$5 - 3$` | Basic subtraction |
| 3 | `4 7 *` | `$4 \times 7$` | Multiplication uses LaTeX `\times` |
| 4 | `10 2 /` | `$10 \div 2$` | Division uses LaTeX `\div` |

### Category 2: Exponentiation (Tests 5, 16-17)

| # | Input | Expected Output | Notes |
|---|-------|-----------------|-------|
| 5 | `2 3 ^` | Error (LexerError) | Caret operator not supported; raises "Unexpected character '^'" at line 1, column 5 |
| 16 | `2 3 ^ 4 *` | Error (LexerError) | Same as test 5; error at line 1, column 5 (the caret position) |
| 17 | `2 3 4 ^ ^` | Error (LexerError) | First caret encountered at line 1, column 7 |

### Category 3: Operator Precedence with Parentheses (Tests 6-8)

| # | Input | Expected Output | Notes |
|---|-------|-----------------|-------|
| 6 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Addition has lower precedence; parentheses required |
| 7 | `5 3 * 2 +` | `$5 \times 3 + 2$` | Multiplication has higher precedence; no parentheses |
| 8 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | Equal precedence operators; left-associative chain |

### Category 4: Subtraction and Division Associativity (Tests 9-10)

| # | Input | Expected Output | Notes |
|---|-------|-----------------|-------|
| 9 | `5 3 - 2 -` | `$5 - 3 - 2$` | Left-associative subtraction; no parentheses for left side |
| 10 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Left-associative division; chain without parentheses |

**Parenthesization Rule for Right Side:** When equal-precedence subtraction or division appears on the right side of subtraction or division, parentheses ARE added. This maintains left-associativity semantics.

### Category 5: Addition Chains (Test 11)

| # | Input | Expected Output | Notes |
|---|-------|-----------------|-------|
| 11 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Addition is commutative; no parentheses needed for left-associative chain |

### Category 6: Mixed Operations with Precedence (Tests 12-15)

| # | Input | Expected Output | Notes |
|---|-------|-----------------|-------|
| 12 | `2 3 4 * +` | `$2 + 3 \times 4$` | Multiplication has higher precedence than addition |
| 13 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Addition on left of multiplication; parentheses required |
| 14 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Addition on right of multiplication; parentheses required |
| 15 | `2 3 * 4 +` | `$2 \times 3 + 4$` | Multiplication has higher precedence; no parentheses |

### Category 7: Floating-Point Numbers (Tests 18-19)

| # | Input | Expected Output | Notes |
|---|-------|-----------------|-------|
| 18 | `3.14 2 *` | `$3.14 \times 2$` | Decimal numbers supported; no conversion |
| 19 | `1.5 0.5 +` | `$1.5 + 0.5$` | Multiple decimal operands supported |

### Category 8: Complex Expressions (Tests 20-21)

| # | Input | Expected Output | Notes |
|---|-------|-----------------|-------|
| 20 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Two addition sub-expressions with multiplication; both need parentheses |
| 21 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Mixed division and addition on left of multiplication |

---

## LaTeX Output Format

### Output Structure
All valid expressions produce output of the form: `$<content>$`
- Wrapped in single dollar signs for LaTeX inline math mode
- NO newline at end of output
- Single space around all binary operators in output

### Operator Mappings
| RPN Operator | LaTeX Output |
|--------------|--------------|
| `+` | `+` |
| `-` | `-` |
| `*` | `\times` |
| `/` | `\div` |

### Parenthesization Rules

1. **Precedence Levels:**
   - Level 1 (low): `+`, `-`
   - Level 2 (high): `*`, `/`

2. **Parentheses Added When:**
   - Child expression has lower precedence than parent (always)
   - Child expression has equal precedence and appears on RIGHT side of `-` or `/` (left-associativity)

3. **Parentheses Format:**
   - Format: `( <expression> )` with spaces inside parentheses
   - Example: `$( 5 + 3 ) \times 2$`

---

## Error Cases

### Unsupported Operators
The following inputs produce **LexerError** with exit code 0 (error message to stderr):

| Input | Error Message | Position |
|-------|---------------|----------|
| `2 3 ^` | `Error: Unexpected character '^'` at line 1, column 5 | First caret encountered |
| `2 3 ^ 4 *` | `Error: Unexpected character '^'` at line 1, column 5 | First caret encountered |
| `2 3 4 ^ ^` | `Error: Unexpected character '^'` at line 1, column 7 | First caret encountered |

### Other Known Parser Errors (Not Tested)
While not in the test set, the parser handles:
- Empty expressions: "Empty expression" error
- Insufficient operands: "Operator 'X' requires two operands" error
- Too many operands: "Invalid RPN: N values remain on stack (missing operators?)" error

---

## Implementation Constraints

### Number Format
- Integers: Any sequence of digits (e.g., `42`, `1000`, `0`)
- Floating-Point: Integer part + decimal point + fractional digits (e.g., `3.14`, `0.5`)
- Negative Numbers: Supported via `-` prefix when followed immediately by digit (scanner lookahead)
- Scientific Notation: NOT supported

### Whitespace Handling
- Single space or tab used as token delimiter
- Whitespace is ignored; any amount acceptable
- Newlines treated as whitespace (tracked for error reporting)

### Token Stream Processing
- All operators are binary (no unary prefix operators)
- RPN: Numbers first, then operators
- Stack-based parsing requires exactly one value after all tokens processed

### Line and Column Tracking
- Line numbers: 1-based
- Column numbers: 1-based
- Error messages include line:column format for precise diagnostics

---

## Exit Codes

| Code | Meaning | When |
|------|---------|------|
| 0 | Success | Valid RPN expression processed successfully |
| 0 | Error | Invalid input (error details sent to stderr, stdout empty) |
| 1 | Fatal Error | File I/O errors, permission denied, etc. (currently tests all use stdin, so N/A) |

**Note:** The error exit code behavior appears unconventional (0 for parse errors). This is observed behavior from test execution.

---

## Regression Test Coverage

This I/O contract covers:
- 21 test cases executed
- 4 supported binary operators
- Operator precedence validation
- Parenthesization rules (precedence + associativity)
- Floating-point number support
- Complex nested expressions
- Error handling for unsupported operators

### Tests by Category
- Basic operations: 4 tests (1-4)
- Exponentiation (unsupported): 3 tests (5, 16-17)
- Precedence/parentheses: 8 tests (6-14)
- Associativity: 2 tests (9-10)
- Floating-point: 2 tests (18-19)
- Complex expressions: 2 tests (20-21)

---

## Validation Notes

- All outputs verified against actual Python implementation execution
- Outputs captured from: `echo "INPUT" | python -c "from rpn2tex.cli import main; import sys; sys.argv = ['rpn2tex', '-']; main()"`
- Exit codes and stderr messages verified
- LaTeX escaping verified (backslashes properly represented)
- Whitespace handling validated (spaces around operators consistent)

