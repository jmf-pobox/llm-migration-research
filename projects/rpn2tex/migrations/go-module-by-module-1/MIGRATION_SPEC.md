# Python-to-Go Migration Specification: rpn2tex

**Document Date**: 2025-12-29
**Migration Target**: Complete Python rpn2tex system to Go
**Specification Version**: 1.0

---

## I/O Contract (From Phase 0)

### Overview

This document specifies the exact input-output behavior of the Python rpn2tex implementation. The Go implementation must produce identical outputs for all test cases to ensure behavioral equivalence.

**Key Implementation Details:**
- Input: RPN (Reverse Polish Notation) expressions as strings
- Processing: Tokenize → Parse → Generate LaTeX
- Output: LaTeX math mode expressions wrapped in `$...$` delimiters
- Errors: Invalid input produces stderr output with formatted error messages

### Test Cases

#### Successful Cases (Exit Code 0)

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

#### Error Cases (Exit Code 1)

| Input | Expected Output (stderr) | Expected Output (stdout) | Notes |
|-------|--------------------------|--------------------------|-------|
| `2 3 ^` | `Error: Unexpected character '^'\n\n1 \| 2 3 ^\n  \|     ^` | (empty) | Caret operator not supported; lexer rejects it with position information |
| `2 3 ^ 4 *` | `Error: Unexpected character '^'\n\n1 \| 2 3 ^ 4 *\n  \|     ^` | (empty) | Error position points to first invalid caret |
| `2 3 4 ^ ^` | `Error: Unexpected character '^'\n\n1 \| 2 3 4 ^ ^\n  \|       ^` | (empty) | Error position points to first invalid caret at column 7 |

### Critical Behavioral Specifications

#### 1. Operator Support

The Python implementation supports **only four binary operators**:
- `+` (addition) → `+` in LaTeX
- `-` (subtraction) → `-` in LaTeX
- `*` (multiplication) → `\times` in LaTeX
- `/` (division) → `\div` in LaTeX

**Exponentiation (`^`) is NOT supported** and triggers a `LexerError`.

#### 2. Operator Precedence

The implementation enforces mathematical operator precedence:
- **Level 1 (lower)**: Addition (`+`) and Subtraction (`-`)
- **Level 2 (higher)**: Multiplication (`*`) and Division (`/`)

Parentheses are inserted based on:
1. **Lower precedence child expressions** always get parentheses
2. **Equal precedence on the right side** of non-commutative operators (`-`, `/`) get parentheses
3. **Equal precedence on the left side** does NOT get parentheses (left-associative)

#### 3. Output Format

- All valid expressions produce output in the format: `$<expression>$`
- Dollar signs (`$...$`) are LaTeX math mode delimiters
- Operators and numbers are separated by single spaces
- Parenthesized sub-expressions use the format: `( <expression> )`
  - Note: spaces are included inside the parentheses
- Decimal numbers preserve their decimal point exactly as input

#### 4. Numeric Types

- **Integers**: Supported (e.g., `5`, `10`, `100`)
- **Decimal numbers**: Supported (e.g., `3.14`, `1.5`, `0.5`)
- **Negative numbers**: The lexer distinguishes between:
  - Subtraction operator: standalone `-` token
  - Negative literals: `-` immediately followed by digits
- Numbers are output as strings exactly as they appear in the input

#### 5. Error Handling

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

#### 6. Whitespace Handling

- Whitespace (spaces, tabs, newlines) acts as token delimiters
- All leading/trailing/excess whitespace is normalized away
- The input is completely whitespace-agnostic after tokenization

#### 7. Line and Column Tracking

- **Line numbers** are 1-based and tracked across the entire input
- **Column numbers** are 1-based within each line
- Position information is used in error messages to pinpoint problems
- Newlines reset the column counter and increment the line counter

#### 8. LaTeX Commands

- Multiplication uses the command `\times` (with backslash)
- Division uses the command `\div` (with backslash)
- When rendering these in Go, ensure the backslash is properly represented as a single backslash in the output string

#### 9. Parenthesization Algorithm

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

#### 10. Input Method

The Python implementation accepts input via:
1. **File path**: `python -m source.cli input.rpn`
2. **stdin**: `python -m source.cli -` (reads from stdin until EOF)
3. **Piped input**: `echo "5 3 +" | python -m source.cli -`

For this I/O contract, all tests were run using stdin method.

#### 11. Output Method

1. **stdout**: LaTeX result (when `-o` not specified)
2. **stderr**: Error messages and file write confirmations
3. **Exit code**: 0 for success, 1 for error

When an output file is specified with `-o`, a confirmation message is written to stderr, not the LaTeX itself.

#### 12. AST Structure (Reference for Go Implementation)

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

---

## Module-by-Module Analysis

### Module 1: tokens.py

#### Purpose
Defines token types and the Token data structure used throughout the lexer and parser.

#### Public API

**Classes:**
- `TokenType(Enum)`: Enumeration of token types
  - Values: `NUMBER`, `PLUS`, `MINUS`, `MULT`, `DIV`, `EOF`
  - Extensible for future operators (CARET, SQRT, ROOT noted in comments)

- `Token(frozen dataclass)`: Immutable token representation
  - Constructor: `__init__(type: TokenType, value: str, line: int, column: int)`
  - Attributes: `type`, `value`, `line`, `column`
  - Method: `__repr__() -> str`

**Constants:**
- Token enum values for all supported operators

#### Dependencies
- **Internal**: None
- **External**:
  - `dataclasses.dataclass`
  - `enum.Enum`, `enum.auto`
  - `__future__.annotations`

#### Go Migration Notes

**Type Mappings:**
| Python | Go |
|--------|-----|
| `TokenType(Enum)` | `const` group with `iota` (or string constants) |
| `Token(frozen dataclass)` | `struct` with exported fields (capitalized) |
| `Enum.auto()` | `iota` or manual numbering |

**Pattern Changes:**
- Python dataclass → Go struct with exported fields
- Python Enum → Go `const` with custom type (recommend `type TokenType string` or `type TokenType int`)
- Python's immutability (frozen) → Go's value semantics and conventionally no mutation
- `__repr__()` → Implement `fmt.Stringer` interface

**Go Idioms:**
- Use `const` with `iota` for token type enumeration:
  ```go
  const (
    NUMBER TokenType = iota
    PLUS
    MINUS
    MULT
    DIV
    EOF
  )
  ```
- Or use string-based constants:
  ```go
  const (
    NUMBER = "NUMBER"
    PLUS = "PLUS"
    // ...
  )
  ```
- Define Token struct with exported fields and optional `String()` method for display

**Special Handling:**
- The frozen dataclass property in Python is naturally enforced in Go through struct immutability conventions (don't modify struct fields after creation)
- Export all fields (capitalized) since Go doesn't have private fields within structs

#### Key Implementation Details
- Token is a simple immutable data container carrying lexical information
- The `value` field holds the raw string representation of the token (e.g., "5", "+", "3.14")
- Position tracking (line, column) is 1-based for human-readable error messages
- Designed to be used downstream by Parser

#### Recommended Migration Order
**Migrate FIRST** - No dependencies on other modules; required by lexer and parser.

---

### Module 2: errors.py

#### Purpose
Provides error formatting with source context for compiler-like error messages.

#### Public API

**Classes:**
- `ErrorFormatter`: Error message formatter with source context
  - Constructor: `__init__(source: str) -> None`
  - Attributes: `source: str`, `lines: list[str]`
  - Method: `format_error(message: str, line: int, column: int, *, context_lines: int = 1) -> str`
  - Method: `_get_context(line: int, column: int, context_lines: int) -> str` (private)

#### Dependencies
- **Internal**: None
- **External**: `__future__.annotations`

#### Go Migration Notes

**Type Mappings:**
| Python | Go |
|--------|-----|
| `list[str]` | `[]string` |
| `dict[str, T]` | `map[string]T` |
| Keyword arguments (`*,`) | Named return values or additional parameters |

**Pattern Changes:**
- Python's string methods `.splitlines()` → Go's `strings.Split(source, "\n")`
- String formatting with f-strings → `fmt.Sprintf()` or string builders
- List slicing → Manual slice operations or helper functions

**Go Idioms:**
- Use `strings` package for string utilities
- Use `fmt` package for formatting
- Return `(string, error)` tuple pattern (if error handling is needed)
- Store lines as `[]string` split on newlines

**Special Handling:**
- The `context_lines` parameter with default value becomes a separate parameter in Go (Go doesn't support default parameters)
- Line number width calculation for alignment should use string formatting

#### Key Implementation Details

**Core Algorithm:**
1. Split source text into lines on initialization
2. Given an error position (line, column), extract context
3. Build formatted output with:
   - Error message header
   - Blank line for separation
   - Source context with line numbers and caret indicator
4. The caret position is calculated from the 1-based column number
5. Alignment is based on the width of the largest line number

**Important Details:**
- The error formatter does NOT raise errors; it formats them for display
- Lines are stored as separate strings after splitting
- The context can show multiple lines (before/after), controlled by `context_lines` parameter (default 1)
- Caret positioning: spaces are added (column - 1) times before the `^` character

#### Recommended Migration Order
**Migrate THIRD** - Used by CLI for error display; no other module dependencies.

---

### Module 3: ast_nodes.py

#### Purpose
Defines Abstract Syntax Tree node types for representing parsed RPN expressions.

#### Public API

**Classes:**
- `ASTNode(frozen dataclass)`: Base class for all AST nodes
  - Attributes: `line: int`, `column: int`

- `Number(ASTNode, frozen dataclass)`: Numeric literal node
  - Attributes: `value: str` (in addition to inherited `line`, `column`)

- `BinaryOp(ASTNode, frozen dataclass)`: Binary operation node
  - Attributes: `operator: str`, `left: Expr`, `right: Expr`

**Type Aliases:**
- `Expr = Number | BinaryOp` (discriminated union)

**Constants:**
- None

#### Dependencies
- **Internal**: None (forward reference via type alias)
- **External**:
  - `dataclasses.dataclass`
  - `__future__.annotations`

#### Go Migration Notes

**Type Mappings:**
| Python | Go |
|--------|-----|
| `ASTNode` base class | Interface approach or embedded struct |
| `frozen dataclass` | Struct (value semantics) |
| `Expr = Number \| BinaryOp` | Interface `type Expr interface{}` or use type switch |
| `str` | `string` |

**Pattern Changes:**
- Python's class inheritance → Go's struct embedding or interface definitions
- Python's type union → Go interface or type switch pattern
- Frozen dataclass immutability → Convention: don't modify struct fields after creation

**Go Idioms:**

Option 1 - Interface approach (recommended for visitor pattern in latex_gen):
```go
type Expr interface {
  isExpr()  // marker method
}

type ASTNode struct {
  Line   int
  Column int
}

type Number struct {
  ASTNode
  Value string
}

func (n *Number) isExpr() {}

type BinaryOp struct {
  ASTNode
  Operator string
  Left     Expr
  Right    Expr
}

func (b *BinaryOp) isExpr() {}
```

Option 2 - Struct embedding (simpler):
```go
type Number struct {
  Line   int
  Column int
  Value  string
}

type BinaryOp struct {
  Line     int
  Column   int
  Operator string
  Left     Expr
  Right    Expr
}

type Expr interface{}  // or use type assertion
```

**Special Handling:**
- The `Expr` type union needs representation in Go. Since Go doesn't have union types, use an interface approach
- All fields should be exported (capitalized)
- The visitor pattern in `latex_gen.py` uses type switches in Go

#### Key Implementation Details

**Immutability:**
- All nodes are immutable (frozen dataclass in Python)
- Position information preserved for all nodes (useful for error reporting)
- Numbers store values as strings (NOT parsed to float/int) to preserve exact formatting

**Node Hierarchy:**
```
Expr (interface/base type)
├── Number
└── BinaryOp
    ├── left: Expr
    └── right: Expr
```

#### Recommended Migration Order
**Migrate SECOND** - Required by parser (which builds these nodes) and latex_gen (which traverses them).

---

### Module 4: lexer.py

#### Purpose
Converts raw text input into a stream of tokens for the parser.

#### Public API

**Classes:**

`LexerError(Exception)`: Exception raised on invalid input
- Attributes: `message: str`, `line: int`, `column: int`
- Constructor: `__init__(message: str, line: int, column: int) -> None`
- Inherited methods: `__str__()` (via Exception)

`Lexer`: Character-by-character tokenizer
- Constructor: `__init__(text: str) -> None`
- Method: `tokenize() -> list[Token]`
- Method: `_at_end() -> bool` (private)
- Method: `_peek() -> str` (private)
- Method: `_advance() -> str` (private)
- Method: `_skip_whitespace() -> None` (private)
- Method: `_scan_token() -> Token` (private)
- Method: `_scan_number(prefix: str, start_line: int, start_column: int) -> Token` (private)

#### Dependencies
- **Internal**: `tokens` module (uses `Token`, `TokenType`)
- **External**: `__future__.annotations`

#### Go Migration Notes

**Type Mappings:**
| Python | Go |
|--------|-----|
| `Exception` | Built-in `error` interface or custom error type |
| `list[Token]` | `[]Token` |
| `str` | `string` |

**Pattern Changes:**
- Python exception class → Go error type (return as error from functions)
- Character indexing `text[pos]` → Rune slicing or `[]byte` conversion
- String length `len(text)` → Unicode-aware: may need `utf8.RuneCountInString()` or `[]rune`
- Instance variables with type annotations → Struct fields in Go

**Go Idioms:**

```go
type LexerError struct {
  Message string
  Line    int
  Column  int
}

func (e *LexerError) Error() string {
  return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
}

type Lexer struct {
  text   string
  pos    int
  line   int
  column int
}

func (l *Lexer) Tokenize() ([]Token, error) {
  // Implementation with error return
}
```

**Special Handling:**
- Go strings are UTF-8; need to be careful with rune (character) handling
- For this RPN use case, ASCII is sufficient, so `string` indexing is fine
- Returning errors in Go: use `error` interface pattern, not exceptions
- Method receivers (`self` in Python) become the first receiver parameter in Go

#### Key Implementation Details

**Tokenization Algorithm:**
1. Initialize: position, line, column tracking
2. Skip whitespace to find next token start
3. Scan token based on first character:
   - `+`, `*`, `/`: Single-char operators
   - `-`: Could be operator or negative number literal
     - If followed immediately by digit → negative number
     - Otherwise → subtraction operator
   - Digit: Start number scanning
   - Other: Raise `LexerError`
4. For numbers:
   - Consume integer digits
   - If `.` found, consume decimal digits
   - Return NUMBER token with accumulated value

**Position Tracking:**
- Line increments on `\n`
- Column resets to 1 on `\n`
- Column increments on other characters
- Start position captured at token scan time

**Critical Behavior:**
- Negative numbers: `-` directly followed by digit(s) → negative literal token
- Standalone `-` without following digit → subtraction operator
- Whitespace (space, tab, `\n`, `\r`) is delimiter
- Single `.` in number is allowed (e.g., `.5` or `3.`)

#### Recommended Migration Order
**Migrate FOURTH** - Depends on `tokens` module; used by parser.

---

### Module 5: parser.py

#### Purpose
Converts token stream into an Abstract Syntax Tree using stack-based RPN parsing.

#### Public API

**Classes:**

`ParserError(Exception)`: Exception raised on invalid RPN syntax
- Attributes: `message: str`, `token: Token`
- Constructor: `__init__(message: str, token: Token) -> None`

`Parser`: RPN stack-based parser
- Constructor: `__init__(tokens: list[Token]) -> None`
- Method: `parse() -> Expr`
- Method: `_current() -> Token` (private)
- Method: `_at_end() -> bool` (private)
- Method: `_advance() -> Token` (private)

#### Dependencies
- **Internal**: `tokens` module (uses `Token`, `TokenType`); `ast_nodes` module (uses `Number`, `BinaryOp`, `Expr`)
- **External**: `__future__.annotations`

#### Go Migration Notes

**Type Mappings:**
| Python | Go |
|--------|-----|
| `ParserError(Exception)` | Custom error struct |
| `list[Token]` | `[]Token` |
| `Expr` | Interface type or type union |

**Pattern Changes:**
- Python exception class → Go error type
- Dictionary mapping `token_type -> operator_str` → Switch statement or map
- Return type `Expr` → Interface type in Go

**Go Idioms:**

```go
type ParserError struct {
  Message string
  Token   Token
}

func (e *ParserError) Error() string {
  return fmt.Sprintf("%s at line %d, column %d",
    e.Message, e.Token.Line, e.Token.Column)
}

type Parser struct {
  tokens []Token
  pos    int
}

func (p *Parser) Parse() (Expr, error) {
  // Implementation with error return
}
```

**Special Handling:**
- Go doesn't have exceptions; use `(result, error)` return pattern
- Token type to operator mapping: use switch statement or map
- Stack operations: use slice with append/pop pattern
- Type assertions needed when working with `Expr` interface

#### Key Implementation Details

**RPN Parsing Algorithm:**
1. Initialize empty operand stack
2. For each token:
   - If NUMBER: Create Number node, push to stack
   - If OPERATOR: Pop 2 operands, create BinaryOp, push result
   - If EOF: Break from loop
3. Validation:
   - Stack must have exactly 1 element at end
   - Empty stack → "Empty expression" error
   - Multiple items remaining → "too many values on stack" error
   - Not enough operands for operator → specific error

**Stack Order (Right-Associative in Code):**
- First pop: right operand (popped last, pushed second)
- Second pop: left operand (popped first, pushed first)

**Error Cases:**
- Operator without 2 operands: "Operator 'X' requires two operands"
- Empty expression: "Empty expression"
- Remaining values: "Invalid RPN: N values remain on stack (missing operators?)"
- Unexpected token types: Handled gracefully (shouldn't occur with valid lexer)

#### Recommended Migration Order
**Migrate FIFTH** - Depends on `tokens` and `ast_nodes`; used by CLI and latex_gen.

---

### Module 6: latex_gen.py

#### Purpose
Converts AST to LaTeX math mode output with proper parenthesization based on operator precedence.

#### Public API

**Classes:**

`LaTeXGenerator`: AST visitor that generates LaTeX
- Constructor: `__init__() -> None` (no arguments)
- Method: `generate(ast: Expr) -> str` (public entry point)
- Method: `_visit(node: Expr) -> str` (dispatcher via `@singledispatchmethod`)
- Method: `_visit_number(node: Number) -> str` (registered visitor)
- Method: `_visit_binary_op(node: BinaryOp) -> str` (registered visitor)
- Method: `_needs_parens(child: Expr, parent_precedence: int, *, is_right: bool) -> bool` (private)

**Class Attributes (Constants):**
- `BINARY_OPS: dict[str, str]` - Maps operators to LaTeX commands
- `PRECEDENCE: dict[str, int]` - Operator precedence levels

#### Dependencies
- **Internal**: `ast_nodes` module (uses `BinaryOp`, `Number`, `Expr`)
- **External**:
  - `functools.singledispatchmethod`
  - `typing.ClassVar`
  - `__future__.annotations`

#### Go Migration Notes

**Type Mappings:**
| Python | Go |
|--------|-----|
| `@singledispatchmethod` | Type switch on `Expr` interface |
| `ClassVar[dict[str, int]]` | Package-level const (for precedence) or const map |
| `dict[str, str]` | `map[string]string` |

**Pattern Changes:**
- Python's visitor pattern via `@singledispatchmethod` → Go's type assertion/switch pattern
- Type dispatching → Explicit type checks with `switch v.(type)` or `if n, ok := expr.(*Number)`
- Method registration → Direct implementation in type switch
- Keyword-only argument (`is_right`) → Named parameter or explicit boolean parameter

**Go Idioms:**

```go
type LaTeXGenerator struct{}

func (g *LaTeXGenerator) Generate(ast Expr) string {
  content := g.visit(ast)
  return "$" + content + "$"
}

func (g *LaTeXGenerator) visit(node Expr) string {
  switch n := node.(type) {
  case *Number:
    return n.Value
  case *BinaryOp:
    return g.visitBinaryOp(n)
  default:
    panic(fmt.Sprintf("No visitor for %T", node))
  }
}

func (g *LaTeXGenerator) visitBinaryOp(node *BinaryOp) string {
  // Implementation
}

// Package-level constants
const (
  PrecedenceLow  = 1
  PrecedenceHigh = 2
)

var BinaryOps = map[string]string{
  "+": "+",
  "-": "-",
  "*": `\times`,
  "/": `\div`,
}

var Precedence = map[string]int{
  "+": PrecedenceLow,
  "-": PrecedenceLow,
  "*": PrecedenceHigh,
  "/": PrecedenceHigh,
}
```

**Special Handling:**
- The LaTeX backslash in Go raw strings: use backticks or double-escape
- `\times` in Go: Use raw string `` `\times` `` or `"\\times"`
- `\div` in Go: Use raw string `` `\div` `` or `"\\div"`
- Keyword-only arguments in Python → Optional parameters or separate methods in Go

#### Key Implementation Details

**Parenthesization Logic:**

```
NEEDS_PARENS if:
1. Child is BinaryOp AND child_precedence < parent_precedence
   → Always add parens (higher precedence parent binds tighter)

2. Child is BinaryOp AND child_precedence == parent_precedence AND is_right AND operator in {"-", "/"}
   → Add parens (right-associativity handling for non-commutative ops)

Otherwise: No parens
```

**Precedence Levels:**
- Level 1: `+`, `-` (addition/subtraction)
- Level 2: `*`, `/` (multiplication/division)
- Higher level = tighter binding

**Output Format:**
- Numbers: output value as-is (string representation)
- Binary operations: `<left> <op> <right>`
- Parenthesized expressions: `( <expr> )` (note spaces inside)
- Final output: `$<expression>$`

**Critical Examples:**
- `5 3 +` → BinaryOp(+, 5, 3) → "5 + 3" → "$5 + 3$"
- `5 3 + 2 *` → BinaryOp(*, BinaryOp(+, 5, 3), 2)
  - Left child (+) has lower precedence than parent (*) → add parens
  - "( 5 + 3 ) × 2" → "$( 5 + 3 ) \times 2$"
- `5 3 - 2 -` → BinaryOp(-, BinaryOp(-, 5, 3), 2)
  - Right child (-) has equal precedence and is right side → add parens? NO!
  - Not on right side; left child doesn't need parens
  - "5 - 3 - 2" → "$5 - 3 - 2$"

#### Recommended Migration Order
**Migrate SIXTH** - Depends on `ast_nodes`; used by CLI.

---

### Module 7: cli.py

#### Purpose
Command-line interface orchestrating the entire pipeline: read input → tokenize → parse → generate → output.

#### Public API

**Functions:**
- `main() -> int` - Entry point for CLI
  - Returns: 0 for success, 1 for error
  - Handles argument parsing, file I/O, pipeline execution, error formatting, and output

**Module-level:**
- `if __name__ == "__main__":` block calls `sys.exit(main())`

#### Dependencies
- **Internal**: All other modules
  - `tokens` (used via lexer)
  - `ast_nodes` (used via parser)
  - `errors.ErrorFormatter`
  - `lexer.Lexer`, `lexer.LexerError`
  - `parser.Parser`, `parser.ParserError`
  - `latex_gen.LaTeXGenerator`

- **External**:
  - `argparse` (CLI argument parsing)
  - `sys` (stdin, stderr, exit codes)
  - `pathlib.Path` (file I/O)

#### Go Migration Notes

**Type Mappings:**
| Python | Go |
|--------|-----|
| `argparse.ArgumentParser` | `flag` package or `github.com/spf13/cobra` |
| `sys.stdin.read()` | `os.Stdin` or `io.ReadAll` |
| `Path.read_text()` | `os.ReadFile()` |
| `print(..., file=sys.stderr)` | `fmt.Fprintf(os.Stderr, ...)` |
| `sys.exit()` | `os.Exit()` or return from main |

**Pattern Changes:**
- Python's `sys.stdin.read()` → Go's `io.ReadAll(os.Stdin)` → string conversion
- File path handling → `os.ReadFile()` or file operations
- Error handling → Check errors explicitly with `if err != nil`
- Argument parsing → Standard library `flag` or external package like `cobra`
- Multiple return values → Go's `(result, error)` pattern

**Go Idioms:**

```go
package main

import (
  "flag"
  "fmt"
  "io"
  "os"
)

func main() {
  os.Exit(run(os.Args[1:]))
}

func run(args []string) int {
  // Parse arguments
  fs := flag.NewFlagSet("rpn2tex", flag.ContinueOnError)
  output := fs.String("o", "", "Output file")
  fs.Parse(args)

  // Get input file from remaining args
  if fs.NArg() != 1 {
    fmt.Fprintf(os.Stderr, "Usage: rpn2tex [options] <input>\n")
    return 1
  }
  inputFile := fs.Arg(0)

  // Read input
  text, err := readInput(inputFile)
  if err != nil {
    fmt.Fprintf(os.Stderr, "Error: %v\n", err)
    return 1
  }

  // Process pipeline
  // ...

  // Write output
  // ...

  return 0
}

func readInput(file string) (string, error) {
  if file == "-" {
    data, err := io.ReadAll(os.Stdin)
    return string(data), err
  }
  data, err := os.ReadFile(file)
  return string(data), err
}
```

**Special Handling:**
- Error messages to stderr: use `fmt.Fprintf(os.Stderr, ...)`
- Exit codes: return from main function (Go's convention)
- File not found → `os.IsNotExist(err)`
- Permission denied → `os.IsPermission(err)`
- Is directory → `os.IsPermission(err)` or check file info

#### Key Implementation Details

**Pipeline Stages:**
1. **Argument parsing**: Input file, optional output file with `-o` flag
2. **Input reading**: File or stdin (detected by `"-"`)
3. **Tokenization**: Lexer converts text → tokens
4. **Parsing**: Parser converts tokens → AST
5. **Generation**: LaTeX generator converts AST → LaTeX string
6. **Output**: Write to file (if `-o` specified) or stdout
7. **Error handling**: Format and display errors to stderr

**Error Handling:**
- File I/O errors (FileNotFoundError, PermissionError, IsADirectoryError)
- LexerError: Format with ErrorFormatter, output to stderr, exit(1)
- ParserError: Format with ErrorFormatter, output to stderr, exit(1)
- Other errors: Generic error messages to stderr

**Output Behavior:**
- Success path: LaTeX to stdout (unless `-o` specified)
- With `-o`: LaTeX to file, confirmation to stderr
- Error path: Error message to stderr, nothing to stdout, exit(1)

**Critical Details:**
- EOF token has position info; used for error reporting
- The `text` string passed to ErrorFormatter and Lexer is raw input (before tokenization)
- Output file writing appends newline: `latex + "\n"`

#### Recommended Migration Order
**Migrate SEVENTH (LAST)** - Depends on all other modules; orchestrates the entire system.

---

## Dependency Graph

```
tokens.py (no dependencies)
    ↓
lexer.py (depends on tokens)
    ↓
    ├→ parser.py (depends on tokens, ast_nodes)
    │      ↓
    │      └→ latex_gen.py (depends on ast_nodes)
    │             ↓
    │             └→ cli.py (depends on all above + errors)
    │
    ├→ ast_nodes.py (no dependencies)
    │
    └→ errors.py (no dependencies)
```

## Migration Order (Recommended)

**Critical path (required for compilation):**

1. `tokens.py` - Token definitions (no deps)
2. `ast_nodes.py` - AST node structures (no deps)
3. `errors.py` - Error formatter (no deps)
4. `lexer.py` - Tokenizer (deps: tokens)
5. `parser.py` - RPN parser (deps: tokens, ast_nodes)
6. `latex_gen.py` - LaTeX generation (deps: ast_nodes)
7. `cli.py` - CLI orchestrator (deps: all)

**Parallel migration possible:**
- `tokens.py` + `ast_nodes.py` + `errors.py` can be done in parallel
- `lexer.py` after `tokens.py`
- `parser.py` after `tokens.py` + `ast_nodes.py`
- `latex_gen.py` after `ast_nodes.py`
- `cli.py` only after all others

---

## Python-to-Go Pattern Summary

### Data Structures

| Python Pattern | Go Equivalent | Notes |
|---|---|---|
| frozen dataclass | struct (value semantics) | Immutability via convention |
| Enum with auto() | const with iota | Type-safe enumeration |
| Base class + subclasses | Interface + implementations | Polymorphism |
| Exception class | error interface | Use `(result, error)` pattern |
| @dataclass | struct | Explicit field definitions |
| Type union (`A \| B`) | interface{} or tagged union | Use type switch or assertion |

### Functions & Methods

| Python Pattern | Go Equivalent | Notes |
|---|---|---|
| `self` parameter | receiver parameter | `func (r Receiver) Method()` |
| `@staticmethod` | package-level function | No receiver |
| `@classmethod` | constructor or package function | Use NewType() convention |
| `def __init__()` | `NewType()` function | Constructor pattern |
| `@property` | getter method | `func (r R) FieldName() Type` |
| Keyword-only args (`*,`) | Additional parameters | Go doesn't support defaults |
| Default parameters | Named return values or optional | Explicit parameter lists |

### Error Handling

| Python Pattern | Go Equivalent | Notes |
|---|---|---|
| try/except | if err != nil checks | Explicit error propagation |
| raise Exception | return err | Error as return value |
| Exception attributes | error struct fields | Implement Error() method |
| Exception hierarchy | Interface-based | Use error interface |

### String & I/O

| Python Pattern | Go Equivalent | Notes |
|---|---|---|
| f-strings | fmt.Sprintf() | String formatting |
| str.split() | strings.Split() | String utilities |
| str.splitlines() | strings.Split(s, "\n") | Line splitting |
| sys.stdin.read() | io.ReadAll(os.Stdin) | stdin reading |
| Path.read_text() | os.ReadFile() | File reading |
| print(..., file=sys.stderr) | fmt.Fprintf(os.Stderr, ...) | stderr output |

### Visitor Pattern

| Python | Go |
|---|---|
| @singledispatchmethod | Type switch or type assertion |
| Method dispatch by type | Explicit type checking |
| isinstance(obj, Type) | ok := obj.(*Type); if ok |
| Registered implementations | Method implementations per type |

---

## Go Module Structure (Recommended)

Create a Go package `rpn2tex` with these files:

```
rpn2tex/
├── tokens.go        (TokenType, Token)
├── ast_nodes.go     (Expr interface, Number, BinaryOp)
├── errors.go        (ErrorFormatter)
├── lexer.go         (Lexer, LexerError)
├── parser.go        (Parser, ParserError)
├── latex_gen.go     (LaTeXGenerator)
├── cli.go           (main(), run())
├── main.go          (entry point)
└── go.mod           (module definition)
```

Or consolidate further:
- Combine `tokens.go` + `ast_nodes.go` into `types.go`
- Combine `errors.go` directly into each error-handling file
- Keep `lexer.go`, `parser.go`, `latex_gen.go` separate for clarity
- Put CLI in `main.go`

---

## Testing Strategy

### Unit Tests Required

1. **Tokens**: Verify Token creation and string representation
2. **AST**: Verify node creation with proper position tracking
3. **Lexer**: Test all token types, position tracking, error cases
4. **Parser**: Test RPN parsing, error cases, AST structure
5. **LaTeX Gen**: Test parenthesization logic, all operators, precedence
6. **CLI**: End-to-end tests matching I/O contract

### Integration Tests (vs I/O Contract)

Must pass all 21 test cases from I/O contract exactly:
- 18 successful cases (stdout + exit 0)
- 3 error cases (stderr + exit 1)

### Validation Checklist

- [ ] All exit codes correct (0 for success, 1 for error)
- [ ] Stdout vs stderr routed correctly
- [ ] Error messages formatted exactly
- [ ] Caret positioning in errors
- [ ] Operator LaTeX commands (`\times`, `\div` with proper backslash)
- [ ] Parenthesization logic correct (all 18 precedence test cases)
- [ ] Whitespace handling (delimiters, normalization)
- [ ] Line/column tracking (1-based)
- [ ] Decimal number preservation
- [ ] Negative number handling

---

## Key Algorithms to Preserve

### 1. Operator Precedence Rules

**Python:**
```python
PRECEDENCE = {"+": 1, "-": 1, "*": 2, "/": 2}

def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
    if not isinstance(child, BinaryOp):
        return False
    child_precedence = self.PRECEDENCE[child.operator]
    if child_precedence < parent_precedence:
        return True
    return child_precedence == parent_precedence and is_right and child.operator in ("-", "/")
```

**Critical behavior:**
- Child with lower precedence → always parenthesize
- Child with equal precedence + is_right=True + operator in {-, /} → parenthesize
- Otherwise → no parentheses
- This enforces left-associativity for non-commutative operators

### 2. RPN Parsing Algorithm

**Python:**
```python
stack = []
for token in tokens:
    if token.type == NUMBER:
        stack.append(Number(...))
    elif token.type in (PLUS, MINUS, MULT, DIV):
        right = stack.pop()
        left = stack.pop()
        stack.append(BinaryOp(op, left, right))
    elif token.type == EOF:
        break

# Validate: stack must have exactly 1 element
```

**Critical behavior:**
- Two pops: first pop = right operand, second pop = left operand
- Stack validation: must be exactly 1 element at end
- Error messages for insufficient operands and too many values

### 3. Lexer Position Tracking

**Python:**
```python
if char == "\n":
    self.line += 1
    self.column = 1
else:
    self.column += 1
```

**Critical behavior:**
- Line numbers 1-based
- Column numbers 1-based
- Column resets to 1 after newline
- Position captured at token start time

### 4. Error Formatter Context Display

**Python:**
```python
prefix = f"{line_num:>{num_width}} | "
# Caret position: (column - 1) spaces before "^"
caret_line = caret_prefix + " " * caret_pos + "^"
```

**Critical behavior:**
- Line number width calculated from largest line number
- Caret positioned at (column - 1) in the caret line
- Format: "1 | source" for line, "  | ^" for caret

---

## Performance Considerations

For the rpn2tex use case, performance is not critical, but:
- String concatenation: Use strings.Builder in Go (not repeated `+`)
- Token list: Pre-allocated if input size is known
- No optimization needed for small expressions

---

## Common Pitfalls to Avoid

1. **Backslash in LaTeX commands**
   - Python raw string: `r"\times"`
   - Go raw string: `` `\times` ``
   - Go regular string: `"\\times"` (double backslash)
   - Ensure output contains single backslash

2. **String vs numeric representation**
   - Keep numbers as strings throughout (don't parse to float)
   - Output number.value directly without conversion

3. **Line/column tracking**
   - Use 1-based indexing consistently
   - Column resets on newline
   - Capture position at token start, not end

4. **Operator precedence edge cases**
   - Right-associativity handling: only for `-` and `/`
   - Both have precedence 2 with `*`
   - Addition has precedence 1 (lower)

5. **Error handling in pipeline**
   - LexerError has message, line, column
   - ParserError has message, token (which has line, column)
   - Both must be caught separately and formatted

6. **File I/O edge cases**
   - Stdin detection: check for `-` argument
   - Output file: write LaTeX + newline
   - Confirmation message goes to stderr, not stdout

7. **Whitespace normalization**
   - All whitespace (space, tab, newline, carriage return) is delimiter
   - Multiple whitespace becomes single delimiter
   - Output preserves single spaces between tokens/operators

---

## References

- Python I/O Contract: Lines 1-233 of this document
- Python source modules: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
  - tokens.py
  - ast_nodes.py
  - errors.py
  - lexer.py
  - parser.py
  - latex_gen.py
  - cli.py

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-12-29 | Initial comprehensive migration specification |

