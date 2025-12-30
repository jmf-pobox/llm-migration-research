# I/O Contract for rpn2tex Python to Go Migration

## Overview

This document specifies the exact input-output behavior of the Python rpn2tex implementation. The Go implementation must produce identical outputs for all test cases to ensure behavioral equivalence.

**Key Implementation Details:**
- Input: RPN (Reverse Polish Notation) expressions as strings
- Processing: Tokenize → Parse → Generate LaTeX
- Output: LaTeX math mode expressions wrapped in `$...$` delimiters
- Errors: Invalid input produces stderr output with formatted error messages

## Test Cases

### Successful Cases (Exit Code 0)

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 +` | `$5 + 3$` | Simple addition |
| `5 3 -` | `$5 - 3$` | Simple subtraction |
| `4 7 *` | `$4 \times 7$` | Simple multiplication; uses `\times` LaTeX command |
| `10 2 /` | `$10 \div 2$` | Simple division; uses `\div` LaTeX command |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Parenthesization: addition (precedence 1) needs parens when child of multiplication (precedence 2) |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | No parentheses: multiplication has higher precedence than addition |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | Left-associative: same precedence operators, division on left doesn't need parens |
| `5 3 - 2 -` | `$5 - 3 - 2$` | Left-associative: `(5 - 3) - 2` evaluates correctly left-to-right |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Chain of divisions, left-associative |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Associative operator, no parens needed |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Precedence: multiplication before addition; no parens on right side of addition |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Parenthesization: lower-precedence addition is operand of multiplication |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Right-side parenthesization: addition needs parens on right operand of multiplication |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | No parens: multiplication has higher precedence than addition |
| `3.14 2 *` | `$3.14 \times 2$` | Decimal numbers: floating-point literals are supported |
| `1.5 0.5 +` | `$1.5 + 0.5$` | Decimal addition: decimal point is preserved as-is in output |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Multiple parenthesizations: both operands of multiplication are additions |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex precedence: division then addition (both lower precedence than multiplication) |

### Error Cases (Exit Code 1)

| Input | Expected Output (stderr) | Expected Output (stdout) | Notes |
|-------|--------------------------|--------------------------|-------|
| `2 3 ^` | `Error: Unexpected character '^'\n\n1 \| 2 3 ^\n  \|     ^` | (empty) | Caret operator not supported; lexer rejects it with position information |
| `2 3 ^ 4 *` | `Error: Unexpected character '^'\n\n1 \| 2 3 ^ 4 *\n  \|     ^` | (empty) | Error position points to first invalid caret |
| `2 3 4 ^ ^` | `Error: Unexpected character '^'\n\n1 \| 2 3 4 ^ ^\n  \|       ^` | (empty) | Error position points to first invalid caret at column 7 |

## Critical Behavioral Specifications

### 1. Operator Support

The Python implementation supports **only four binary operators**:
- `+` (addition) → `+` in LaTeX
- `-` (subtraction) → `-` in LaTeX
- `*` (multiplication) → `\times` in LaTeX
- `/` (division) → `\div` in LaTeX

**Exponentiation (`^`) is NOT supported** and triggers a `LexerError`.

### 2. Operator Precedence

The implementation enforces mathematical operator precedence:
- **Level 1 (lower)**: Addition (`+`) and Subtraction (`-`)
- **Level 2 (higher)**: Multiplication (`*`) and Division (`/`)

Parentheses are inserted based on:
1. **Lower precedence child expressions** always get parentheses
2. **Equal precedence on the right side** of non-commutative operators (`-`, `/`) get parentheses
3. **Equal precedence on the left side** does NOT get parentheses (left-associative)

### 3. Output Format

- All valid expressions produce output in the format: `$<expression>$`
- Dollar signs (`$...$`) are LaTeX math mode delimiters
- Operators and numbers are separated by single spaces
- Parenthesized sub-expressions use the format: `( <expression> )`
  - Note: spaces are included inside the parentheses
- Decimal numbers preserve their decimal point exactly as input

### 4. Numeric Types

- **Integers**: Supported (e.g., `5`, `10`, `100`)
- **Decimal numbers**: Supported (e.g., `3.14`, `1.5`, `0.5`)
- **Negative numbers**: The lexer distinguishes between:
  - Subtraction operator: standalone `-` token
  - Negative literals: `-` immediately followed by digits
- Numbers are output as strings exactly as they appear in the input

### 5. Error Handling

- **Invalid characters** (anything not: digits, `.`, `-`, `+`, `*`, `/`, whitespace) trigger a `LexerError`
- Error messages include:
  - Human-readable error description: `Error: Unexpected character '<char>'`
  - Visual indicator with line and column numbers (1-based indexing)
  - Example format:
    ```
    Error: Unexpected character '^'

    1 | 2 3 ^
      |     ^
    ```
- Errors are written to **stderr**, not stdout
- Error exit code is **1**; success exit code is **0**

### 6. Whitespace Handling

- Whitespace (spaces, tabs, newlines) acts as token delimiters
- All leading/trailing/excess whitespace is normalized away
- The input is completely whitespace-agnostic after tokenization

### 7. Line and Column Tracking

- **Line numbers** are 1-based and tracked across the entire input
- **Column numbers** are 1-based within each line
- Position information is used in error messages to pinpoint problems
- Newlines reset the column counter and increment the line counter

### 8. LaTeX Commands

- Multiplication uses the command `\times` (with backslash)
- Division uses the command `\div` (with backslash)
- When rendering these in Go, ensure the backslash is properly represented as a single backslash in the output string

### 9. Parenthesization Algorithm

The `_needs_parens()` method in `latex_gen.py` determines if parentheses are needed:

```python
def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
    if not isinstance(child, BinaryOp):
        return False

    child_precedence = self.PRECEDENCE[child.operator]

    # Lower precedence always needs parens
    if child_precedence < parent_precedence:
        return True

    # Equal precedence on right side needs parens for non-commutative operators
    return (
        child_precedence == parent_precedence
        and is_right
        and child.operator in ("-", "/")
    )
```

This means:
- Subtraction is left-associative: `5 - 3 - 2` → `$5 - 3 - 2$` (not `$(5 - (3 - 2))$`)
- Division is left-associative: `100 / 10 / 5` → `$100 \div 10 \div 5$`
- Addition does NOT get parens when on the right of addition: `1 + 2 + 3` → `$1 + 2 + 3$`
- Subtraction gets parens when on the right of another subtraction: handled by left-associativity

## Input Method

The Python implementation accepts input via:
1. **File path**: `python -m source.cli input.rpn`
2. **stdin**: `python -m source.cli -` (reads from stdin until EOF)
3. **Piped input**: `echo "5 3 +" | python -m source.cli -`

For this I/O contract, all tests were run using stdin method.

## Output Method

1. **stdout**: LaTeX result (when `-o` not specified)
2. **stderr**: Error messages and file write confirmations
3. **Exit code**: 0 for success, 1 for error

When an output file is specified with `-o`, a confirmation message is written to stderr, not the LaTeX itself.

## AST Structure (Reference for Go Implementation)

The Python implementation uses an abstract syntax tree with these node types:

```python
# From ast_nodes.py
class Expr: pass  # Base class

class Number(Expr):
    line: int
    column: int
    value: str  # stored as string, not numeric

class BinaryOp(Expr):
    line: int
    column: int
    operator: str  # one of: "+", "-", "*", "/"
    left: Expr
    right: Expr
```

Key point: **Numbers are stored as strings**, not as numeric types. This preserves decimal formatting exactly.

## Implementation Pipeline

1. **Lexer** (`lexer.py`): Raw text → List of tokens
   - Handles character scanning with position tracking
   - Distinguishes between operators and numbers
   - Raises `LexerError` for invalid characters

2. **Parser** (`parser.py`): List of tokens → AST
   - Implements RPN parsing with a stack
   - Validates token sequences
   - Raises `ParserError` for syntactic issues

3. **LaTeX Generator** (`latex_gen.py`): AST → LaTeX string
   - Uses visitor pattern with `singledispatchmethod`
   - Applies operator precedence rules
   - Wraps output in `$...$` delimiters

4. **CLI** (`cli.py`): Orchestrates the pipeline
   - Handles file I/O
   - Formats and displays errors
   - Manages exit codes

## Testing Notes

All 21 test cases were executed against the Python reference implementation on 2024-12-29. The outputs shown in this contract are the exact outputs produced, including:
- Exact whitespace and formatting
- Exact LaTeX command formatting (with backslashes)
- Exact error messages and positioning

The Go implementation must produce byte-for-byte identical output to be considered correct.

## Additional Test Cases Recommended for Go Implementation

Beyond these 21 cases, consider testing:
1. Empty input → should error
2. Single number → should output `$<number>$`
3. Invalid operator count → insufficient operands
4. Multiple expressions on separate lines → handling of newlines
5. Very long expressions → stress test the stack
6. Very large/small numbers → numeric precision
7. Numbers with leading zeros → `007` → `$007$`
8. Negative numbers → `-5 3 +` behavior
