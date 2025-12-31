# RPN2TeX Python to Go Migration Specification

**Document Version:** 1.0
**Target Language:** Go
**Source Language:** Python
**Generated Date:** 2025-12-30

---

## I/O Contract

This I/O contract is the ground truth for behavioral validation during migration. All Go implementations must produce identical outputs.

### Overview

This document captures the exact input/output behavior of the Python rpn2tex implementation. These outputs serve as the ground truth for validating the Go implementation during migration.

### Test Cases

#### Successful Cases

| # | Input | Expected Output | Notes |
|---|-------|-----------------|-------|
| 1 | `5 3 +` | `$5 + 3$` | Simple addition |
| 2 | `5 3 -` | `$5 - 3$` | Simple subtraction |
| 3 | `4 7 *` | `$4 \times 7$` | Simple multiplication with \times operator |
| 4 | `10 2 /` | `$10 \div 2$` | Simple division with \div operator |
| 5 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Parentheses added for lower precedence operation first |
| 6 | `5 3 * 2 +` | `$5 \times 3 + 2$` | No parentheses needed (multiplication has higher precedence) |
| 7 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | Left-to-right evaluation for same precedence |
| 8 | `5 3 - 2 -` | `$5 - 3 - 2$` | Multiple subtractions left-to-right |
| 9 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Chain of divisions |
| 10 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Chain of additions |
| 11 | `2 3 4 * +` | `$2 + 3 \times 4$` | Multiplication before addition (respects precedence) |
| 12 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Parentheses for addition computed before multiplication |
| 13 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Parentheses for addition computed before outer multiplication |
| 14 | `2 3 * 4 +` | `$2 \times 3 + 4$` | Multiplication has higher precedence than addition |
| 15 | `3.14 2 *` | `$3.14 \times 2$` | Floating point number support |
| 16 | `1.5 0.5 +` | `$1.5 + 0.5$` | Floating point addition |
| 17 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Multiple parenthesized subexpressions |
| 18 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex expression with mixed operators |

#### Error Cases

| # | Input | Error Message | Notes |
|---|-------|---------------|-------|
| 1 | `2 3 ^` | `Error: Unexpected character '^'` | Exponentiation operator not supported in lexer |
| 2 | `2 3 ^ 4 *` | `Error: Unexpected character '^'` | Exponentiation operator not supported in lexer |
| 3 | `2 3 4 ^ ^` | `Error: Unexpected character '^'` | Exponentiation operator not supported in lexer |

### Output Format Specifications

#### LaTeX Math Mode
- All outputs are wrapped in `$...$` delimiters (LaTeX inline math mode)
- No newline characters in output

#### Operator Mappings
- Addition: ` + ` (space-delimited)
- Subtraction: ` - ` (space-delimited)
- Multiplication: ` \times ` (space-delimited, backslash-escaped)
- Division: ` \div ` (space-delimited, backslash-escaped)
- Exponentiation: NOT SUPPORTED (causes LexerError)

#### Parentheses
- Spaces around parenthesized expressions: `( expr )` not `(expr)`
- Parentheses added when needed to preserve operator precedence:
  - Addition/subtraction have equal precedence
  - Multiplication/division have equal precedence (higher than addition/subtraction)
  - Operations of same precedence are evaluated left-to-right

#### Floating Point Numbers
- Decimal points preserved as-is in output
- No special formatting applied

### Exit Code Summary
- **Exit Code 0:** Successful parsing and generation
- **Exit Code 1:** Lexer or parser error (e.g., unsupported character)

---

## Module Dependencies and Migration Order

```
            CLI (Entry Point)
            /   |      \
           /    |       \
     Lexer    Parser    LaTeX Generator
      |        |   |        /
      |        |   |       /
    Tokens  AST   Error Formatter
```

**Recommended Migration Order:**
1. `tokens.py` - Core data structures (no dependencies)
2. `ast_nodes.py` - Core data structures (no dependencies)
3. `errors.py` - Error utilities (no dependencies on core modules)
4. `lexer.py` - Tokenization (depends on tokens)
5. `parser.py` - Parsing (depends on tokens, ast_nodes)
6. `latex_gen.py` - Output generation (depends on ast_nodes)
7. `cli.py` - Command-line interface (depends on all others)

---

## Module-by-Module Specification

### Module: tokens.py

#### Public API

**Classes:**
- `TokenType(Enum)` - Enumeration of token types
  - Properties: `NUMBER`, `PLUS`, `MINUS`, `MULT`, `DIV`, `EOF`
  - Method: `auto()` - Auto-increment value assignment

- `Token` - Immutable token representation
  - Constructor: `Token(type: TokenType, value: str, line: int, column: int)`
  - Properties: `type: TokenType`, `value: str`, `line: int`, `column: int`
  - Method: `__repr__() -> str` - Returns formatted token representation

#### Dependencies
- Internal: None
- External: `dataclasses` (Python standard), `enum` (Python standard), `__future__.annotations`

#### Go Migration Notes

**Type Mappings:**
- `TokenType` → Go `iota`-based constants (or `const` + `int`)
  ```go
  type TokenType int
  const (
    NUMBER TokenType = iota
    PLUS
    MINUS
    MULT
    DIV
    EOF
  )
  ```
- `Token` (Python frozen dataclass) → Go `struct` (values immutable by convention)
  ```go
  type Token struct {
    Type   TokenType
    Value  string
    Line   int
    Column int
  }
  ```
- Python `Enum.auto()` → Go `iota` for auto-incrementing constants

**Pattern Changes:**
- Python's `@dataclass(frozen=True)` enforces immutability; Go relies on naming conventions (unexported fields for privacy, immutability by design)
- Python's `__repr__` method → Go's `String()` method for `fmt.Stringer` interface
- Use string constants for operator values to match Python behavior

**Special Handling:**
- Position tracking (line, column) must be 1-based (not 0-based) to match Python
- Token `repr` format: `Token(TYPE_NAME, 'value', line:column)` should be preserved in string output for debugging

#### Key Implementation Details

The `tokens.py` module defines the lexical units recognized by the lexer. The module uses Python's `Enum` class with `auto()` for auto-incrementing token type IDs. Token positions are 1-based (line and column both start at 1). The frozen dataclass pattern ensures tokens are immutable once created.

**Critical:**
- Token positions must be 1-based for error reporting compatibility
- Operator tokens preserve their string values ("+" for PLUS, "-" for MINUS, etc.)
- EOF token uses empty string as value

---

### Module: ast_nodes.py

#### Public API

**Classes:**
- `ASTNode` - Base class for all AST nodes (frozen dataclass)
  - Properties: `line: int`, `column: int`

- `Number` - Numeric literal node
  - Constructor: `Number(line: int, column: int, value: str)`
  - Properties: `line: int`, `column: int`, `value: str`

- `BinaryOp` - Binary operation node
  - Constructor: `BinaryOp(line: int, column: int, operator: str, left: Expr, right: Expr)`
  - Properties: `line: int`, `column: int`, `operator: str`, `left: Expr`, `right: Expr`

**Type Aliases:**
- `Expr = Number | BinaryOp` - Union type for all expression nodes

#### Dependencies
- Internal: None
- External: `dataclasses` (Python standard), `__future__.annotations`

#### Go Migration Notes

**Type Mappings:**
- `ASTNode` → Go interface or embedded struct
  ```go
  type ASTNode struct {
    Line   int
    Column int
  }
  ```
- Union type `Expr = Number | BinaryOp` (Python 3.10+) → Go interface
  ```go
  type Expr interface {
    // Marker method to ensure only valid types implement this
    isExpr()
  }
  ```
- Frozen dataclass → Go struct values (pass by value, no pointers for immutability)

**Pattern Changes:**
- Python's union type `|` syntax → Go interfaces with marker methods
- No inheritance needed; embedding is sufficient for position tracking
- Pattern matching on union types → Go type assertions or type switches

**Special Handling:**
- Each expression type must implement the `Expr` interface via a marker method
- Use pointer types for AST nodes since they form a tree structure (parent references child nodes)
- Preserve string values exactly (including decimal points) for floating-point literals

#### Key Implementation Details

The `ast_nodes.py` module defines immutable tree nodes for representing parsed expressions. All nodes track source position (line, column) for error reporting. The module uses Python's union type syntax (`X | Y`) for expressing that an expression can be one of several types. This pattern must be converted to Go interfaces with type assertions.

**Critical:**
- Nodes must preserve exact string representation of numbers (e.g., "3.14" stays "3.14")
- Binary operator string values are "+", "-", "*", "/" (not LaTeX form)
- All nodes are logically immutable (position/value never change after creation)
- Left and right operands are recursive Expr types (can be Number or BinaryOp)

---

### Module: errors.py

#### Public API

**Classes:**
- `ErrorFormatter` - Formats errors with source context
  - Constructor: `ErrorFormatter(source: str)`
  - Properties: `source: str`, `lines: list[str]`
  - Methods:
    - `format_error(message: str, line: int, column: int, *, context_lines: int = 1) -> str`
    - `_get_context(line: int, column: int, context_lines: int) -> str` (private)

#### Dependencies
- Internal: None
- External: None (uses only Python standard functionality)

#### Go Migration Notes

**Type Mappings:**
- `ErrorFormatter` class → Go struct with methods
  ```go
  type ErrorFormatter struct {
    Source string
    Lines  []string
  }
  ```
- Python `self` → Go receiver in method signatures

**Pattern Changes:**
- Python's `*` keyword-only arguments → Go functional options pattern or explicit parameter order
- String list slicing and manipulation → Go's `strings` package utilities
- Python's `len()`, `max()`, `min()` → Go's equivalent functions

**Special Handling:**
- Line number width calculation for aligned output
- Caret positioning must account for 1-based columns (subtract 1 for 0-based indexing)
- Context range clamping using `max()` and `min()`
- String formatting with padding for alignment

#### Key Implementation Details

The `ErrorFormatter` class provides context-aware error messages with source code display. It splits source into lines during initialization. The `format_error` method constructs multi-line output with line numbers, source context, and a caret pointing to the error location. The `_get_context` method handles the actual formatting with proper alignment.

**Critical:**
- Line numbers are 1-based in input, must be 1-based in error messages
- Column positions are 1-based (user-facing), subtract 1 for 0-based array indexing
- Caret alignment requires counting character width
- Context range is clamped to valid line indices
- Output format: `"Error: {message}\n\n{line} | {content}\n{spaces}^"`

---

### Module: lexer.py

#### Public API

**Classes:**
- `LexerError(Exception)` - Exception raised by lexer
  - Constructor: `LexerError(message: str, line: int, column: int)`
  - Properties: `message: str`, `line: int`, `column: int`

- `Lexer` - Tokenizes RPN input
  - Constructor: `Lexer(text: str)`
  - Properties: `text: str`, `pos: int`, `line: int`, `column: int`
  - Methods:
    - `tokenize() -> list[Token]` - Public API, returns all tokens
    - `_at_end() -> bool` - Private, check end of input
    - `_peek() -> str` - Private, look at current character
    - `_advance() -> str` - Private, consume character and advance position
    - `_skip_whitespace() -> None` - Private, skip whitespace
    - `_scan_token() -> Token` - Private, scan single token
    - `_scan_number(prefix: str, start_line: int, start_column: int) -> Token` - Private, scan numeric literal

#### Dependencies
- Internal: `tokens.py` (Token, TokenType)
- External: None (only Python standard)

#### Go Migration Notes

**Type Mappings:**
- `LexerError` exception → Go error type (custom error struct implementing error interface)
  ```go
  type LexerError struct {
    Message string
    Line    int
    Column  int
  }
  func (e LexerError) Error() string {
    return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
  }
  ```
- `Lexer` class → Go struct with methods
  - Position tracking: `Pos`, `Line`, `Column` as struct fields

**Pattern Changes:**
- Python exception hierarchy → Go error interface
- Whitespace checking via string membership (`in " \t\n\r"`) → Go `strings.ContainsRune()` or equivalent
- Character methods (`isdigit()`) → Go `unicode` package or simple char comparison
- String slicing for negative numbers → Go manual position tracking

**Special Handling:**
- Position tracking: increment column on normal characters, reset to 1 and increment line on newline
- Negative number handling: check if "-" is followed immediately by digit (no space)
- Decimal point handling: consume "." and continue reading digits
- EOF token must be added at end with empty value

#### Key Implementation Details

The `Lexer` class performs character-by-character scanning of RPN input, producing a stream of tokens. It tracks source position (line, column) for error reporting. Key responsibilities:

1. **Position Tracking:** Line and column are 1-based. Column resets to 1 on newline, increments otherwise.
2. **Whitespace Handling:** Spaces, tabs, newlines, carriage returns are delimiters (skipped).
3. **Number Scanning:** Handles integers and decimals. A "-" followed by a digit is a negative number, not a subtraction operator.
4. **Operator Tokenization:** "+", "-", "*", "/" are recognized as operators (except "-" followed by digit).
5. **Error Handling:** Any unrecognized character raises `LexerError` with position.

**Critical:**
- Line and column are 1-based (not 0-based)
- Negative numbers: "-" immediately followed by digit is a number token, not MINUS operator
- EOF token appended with empty value
- Position must be accurate for error reporting
- Whitespace is only delimiter; no significance in output

---

### Module: parser.py

#### Public API

**Classes:**
- `ParserError(Exception)` - Exception raised by parser
  - Constructor: `ParserError(message: str, token: Token)`
  - Properties: `message: str`, `token: Token`

- `Parser` - Converts token stream to AST
  - Constructor: `Parser(tokens: list[Token])`
  - Properties: `tokens: list[Token]`, `pos: int`
  - Methods:
    - `parse() -> Expr` - Public API, parses tokens into AST
    - `_current() -> Token` - Private, get current token
    - `_at_end() -> bool` - Private, check if at EOF
    - `_advance() -> Token` - Private, consume token and advance

#### Dependencies
- Internal: `tokens.py` (Token, TokenType), `ast_nodes.py` (BinaryOp, Expr, Number)
- External: None (only Python standard)

#### Go Migration Notes

**Type Mappings:**
- `ParserError` exception → Go error type
  ```go
  type ParserError struct {
    Message string
    Token   *Token
  }
  func (e ParserError) Error() string {
    return fmt.Sprintf("%s at line %d, column %d", e.Message, e.Token.Line, e.Token.Column)
  }
  ```
- `Parser` class → Go struct with methods
  - Use slice of `*Token` pointers for recursive structure sharing

**Pattern Changes:**
- Python list stack operations → Go slice append/pop patterns
- Dictionary mapping for operator conversion → Go switch/case or map
- Exception raising → Go error return values

**Special Handling:**
- Stack implementation: use Go slice with append/pop
- Operator mapping: map TokenType to operator string
- Validation: stack must have exactly 1 element at end; error if 0 or > 1

#### Key Implementation Details

The `Parser` implements a stack-based RPN parser. Algorithm:

1. **Scan tokens left-to-right:**
   - NUMBER: push onto stack
   - OPERATOR: pop 2 operands, create BinaryOp, push result
   - EOF: stop
2. **Validation:**
   - At least 2 operands before each operator
   - Exactly 1 element remains on stack at end

The parser constructs an AST tree where operators are internal nodes and numbers are leaves. Left operand is popped first, then right (stack LIFO).

**Critical:**
- Stack order matters: first pop is RIGHT operand, second pop is LEFT operand
- Operator strings mapped: PLUS→"+", MINUS→"-", MULT→"*", DIV→"/"
- Empty expression error: 0 elements on stack at EOF
- Incomplete expression error: >1 elements on stack at EOF (missing operators)
- Each operator requires exactly 2 operands

---

### Module: latex_gen.py

#### Public API

**Classes:**
- `LaTeXGenerator` - Converts AST to LaTeX
  - Class constants:
    - `BINARY_OPS: dict[str, str]` - Operator to LaTeX mapping
    - `PRECEDENCE: dict[str, int]` - Operator precedence levels
  - Methods:
    - `generate(ast: Expr) -> str` - Public API, generates LaTeX from AST
    - `_visit(node: Expr) -> str` - Visitor dispatcher (singledispatchmethod)
    - `_visit_number(node: Number) -> str` - Handler for Number nodes
    - `_visit_binary_op(node: BinaryOp) -> str` - Handler for BinaryOp nodes
    - `_needs_parens(child: Expr, parent_precedence: int, *, is_right: bool) -> bool` - Helper for parenthesization

#### Dependencies
- Internal: `ast_nodes.py` (BinaryOp, Expr, Number)
- External: `functools` (singledispatchmethod), `typing` (ClassVar)

#### Go Migration Notes

**Type Mappings:**
- Python `ClassVar` → Go package-level constants
  ```go
  var (
    BinaryOps = map[string]string{"+": "+", "-": "-", "*": `\times`, "/": `\div`}
    Precedence = map[string]int{"+": 1, "-": 1, "*": 2, "/": 2}
  )
  ```
- `@singledispatchmethod` visitor pattern → Go type switch or explicit type checking

**Pattern Changes:**
- Python `@singledispatchmethod` → Go type switch on interface{} or type assertion
- Python operator mapping dictionaries → Go maps or switch statements
- Generator class with state → Go functions or methods on struct (if needed for future extensions)

**Special Handling:**
- Parenthesis logic: only add parens when strictly necessary
  - Lower precedence child needs parens
  - Equal precedence on right side only for non-commutative operators ("-" and "/")
- Operator LaTeX escaping: backslash-escaped in strings (`\times`, `\div`)
- Output format: `${content}$` wrapping

#### Key Implementation Details

The `LaTeXGenerator` converts an AST to infix notation with appropriate parenthesization. It uses the visitor pattern to dispatch based on node type.

**Precedence Rules:**
- Addition (+) and Subtraction (-): precedence level 1
- Multiplication (*) and Division (/): precedence level 2
- Higher level = tighter binding

**Parenthesization Logic:**
1. Child with LOWER precedence than parent → ALWAYS needs parens
2. Child with EQUAL precedence and on RIGHT side of NON-COMMUTATIVE operator → needs parens
   - Non-commutative: "-" and "/"
   - Commutative: "+" and "*" (but "+" has no right-associativity issue, only "-" does)

**Critical:**
- LaTeX operators: "+" stays "+", "-" stays "-", "*" becomes `\times`, "/" becomes `\div`
- Parens format: `( expr )` with spaces around expression
- Output format: `${expr}$` with math mode delimiters
- No trailing spaces in output
- Numbers preserve exact string values (including decimals like "3.14")

**Example Walkthrough for `5 3 + 2 *`:**
```
AST: BinaryOp("*", BinaryOp("+", Number("5"), Number("3")), Number("2"))

Generate root (*):
  - op_latex = "\times"
  - my_precedence = 2
  - left = generate(BinaryOp("+", ...))
    - op_latex = "+"
    - my_precedence = 1
    - left = "5", no parens (Number)
    - right = "3", no parens (Number)
    - result: "5 + 3"
  - needs_parens(BinaryOp("+"), 2, is_right=False)?
    - child_precedence = 1 < 2 (parent) → TRUE
    - add parens: "( 5 + 3 )"
  - right = "2", no parens (Number)
  - result: "( 5 + 3 ) \times 2"

Output: "$( 5 + 3 ) \times 2$"
```

---

### Module: cli.py

#### Public API

**Functions:**
- `main() -> int` - Entry point for CLI
  - Returns exit code (0 for success, 1 for error)

#### Dependencies
- Internal: `errors.py` (ErrorFormatter), `latex_gen.py` (LaTeXGenerator), `lexer.py` (Lexer, LexerError), `parser.py` (Parser, ParserError)
- External: `argparse`, `sys`, `pathlib` (Path)

#### Go Migration Notes

**Type Mappings:**
- Python `argparse.ArgumentParser` → Go `flag` package or third-party library (`github.com/spf13/cobra` or `github.com/urfave/cli`)
- Python `Path` from `pathlib` → Go `os` or `io` packages for file operations
- Python `sys.stdin.read()` → Go `io.ReadAll(os.Stdin)`
- Python `sys.stderr` → Go `os.Stderr`

**Pattern Changes:**
- Python exception catching → Go error checking with `if err != nil`
- Python file I/O (`read_text()`, `write_text()`) → Go `ioutil.ReadFile()`, `ioutil.WriteFile()`
- Python exit codes → Go `os.Exit(code)` in main
- Python argument parsing → Go flag package or CLI library

**Special Handling:**
- Stdin detection: `-` as input filename
- Error messages to stderr, output to stdout (or file)
- Exit code 0 on success, 1 on error
- File I/O error handling: FileNotFoundError, PermissionError, IsADirectoryError
- Pipeline orchestration: lexer → parser → generator

#### Key Implementation Details

The CLI is the orchestration layer that:

1. **Parses arguments:** Input file (or "-" for stdin), optional output file
2. **Reads input:** From file or stdin
3. **Processes:** Tokenize → Parse → Generate LaTeX
4. **Outputs:** To stdout (or file) and status to stderr
5. **Error handling:** Catches LexerError and ParserError, formats with context, returns exit code 1

**Critical:**
- Stdin indicated by input filename "-"
- File errors handled gracefully with informative messages
- Errors written to stderr, output to stdout
- Exit code 0 on success, 1 on any error
- Error formatting includes source context using ErrorFormatter
- Success message to stderr when writing to file (optional but matches Python behavior)
- Pipeline must maintain exact order: Lexer → Parser → LaTeXGenerator

---

## Design Patterns and Idioms: Python to Go

### 1. Immutable Data Structures

**Python Pattern:**
```python
@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str
```

**Go Pattern:**
```go
type Token struct {
    Type   TokenType
    Value  string
}

// Immutability by convention:
// - Use value types (not pointers) where practical
// - Never modify fields after creation
// - Document as immutable in comments
```

### 2. Union Types

**Python Pattern:**
```python
Expr = Number | BinaryOp  # Union type alias
```

**Go Pattern:**
```go
type Expr interface {
    isExpr()  // Marker method
}

func (n *Number) isExpr() {}
func (b *BinaryOp) isExpr() {}

// Use type assertions:
if num, ok := expr.(*Number); ok {
    // Handle Number
}
```

### 3. Visitor Pattern with Singledispatch

**Python Pattern:**
```python
@singledispatchmethod
def _visit(self, node: Expr) -> str:
    raise NotImplementedError(...)

@_visit.register
def _visit_number(self, node: Number) -> str:
    return node.value
```

**Go Pattern:**
```go
func (g *LaTeXGenerator) Visit(expr Expr) string {
    switch e := expr.(type) {
    case *Number:
        return g.visitNumber(e)
    case *BinaryOp:
        return g.visitBinaryOp(e)
    default:
        panic(fmt.Sprintf("No visitor for %T", expr))
    }
}

func (g *LaTeXGenerator) visitNumber(n *Number) string {
    return n.Value
}
```

### 4. Exception Handling

**Python Pattern:**
```python
try:
    # code
except LexerError as e:
    # handle
except ParserError as e:
    # handle
```

**Go Pattern:**
```go
if err != nil {
    switch err := err.(type) {
    case *LexerError:
        // handle LexerError
    case *ParserError:
        // handle ParserError
    default:
        // handle other errors
    }
}

// Or custom error types implementing error interface:
type LexerError struct {
    Message string
    Line    int
    Column  int
}
func (e *LexerError) Error() string { ... }
```

### 5. String Formatting and Alignment

**Python Pattern:**
```python
num_width = len(str(max_line_num))
prefix = f"{line_num:>{num_width}} | "
```

**Go Pattern:**
```go
numWidth := len(strconv.Itoa(maxLineNum))
prefix := fmt.Sprintf("%*d | ", numWidth, lineNum)
```

### 6. Class Methods with State

**Python Pattern:**
```python
class Lexer:
    def __init__(self, text: str):
        self.text = text
        self.pos = 0
```

**Go Pattern:**
```go
type Lexer struct {
    text string
    pos  int
}

func NewLexer(text string) *Lexer {
    return &Lexer{text: text, pos: 0}
}
```

### 7. Stack-Based Algorithms

**Python Pattern:**
```python
stack: list[Expr] = []
stack.append(item)
right = stack.pop()
left = stack.pop()
```

**Go Pattern:**
```go
var stack []Expr
stack = append(stack, item)
right := stack[len(stack)-1]
stack = stack[:len(stack)-1]
left := stack[len(stack)-1]
stack = stack[:len(stack)-1]
```

### 8. Dictionary/Map Usage

**Python Pattern:**
```python
op_map = {
    TokenType.PLUS: "+",
    TokenType.MINUS: "-",
}
operator = op_map[token.type]
```

**Go Pattern:**
```go
var opMap = map[TokenType]string{
    PLUS: "+",
    MINUS: "-",
}
operator := opMap[tokenType]

// Or use switch:
var operator string
switch tokenType {
case PLUS:
    operator = "+"
case MINUS:
    operator = "-"
}
```

### 9. Keyword Arguments

**Python Pattern:**
```python
def format_error(self, message: str, line: int, column: int, *, context_lines: int = 1):
    # * forces context_lines to be keyword-only
```

**Go Pattern:**
```go
func (f *ErrorFormatter) FormatError(message string, line, column int, contextLines int) string {
    // Use default value in function body or use options pattern
    if contextLines == 0 {
        contextLines = 1  // Default
    }
}

// Or use options pattern:
func (f *ErrorFormatter) FormatError(message string, line, column int, opts ...Option) string {
    o := defaultOptions()
    for _, opt := range opts {
        opt(&o)
    }
}
```

### 10. List Comprehension and String Methods

**Python Pattern:**
```python
self.lines = source.splitlines()
if char in " \t\n\r":
    # skip
if char.isdigit():
    # handle
```

**Go Pattern:**
```go
f.lines = strings.Split(source, "\n")
if strings.ContainsRune(" \t\n\r", rune(char)) {
    // skip
}
if unicode.IsDigit(rune(char)) {
    // handle
}
```

---

## Precedence and Associativity

### Operator Precedence (Higher = Tighter Binding)

| Level | Operators | Example |
|-------|-----------|---------|
| 2 (Higher) | `*`, `/` | `2 * 3 + 4` → `(2 * 3) + 4` |
| 1 (Lower) | `+`, `-` | `5 + 3 - 2` → `(5 + 3) - 2` |

### Associativity Rules for Parenthesization

**Left-Associative Operators:** `-` and `/`
- `5 - 3 - 2` means `(5 - 3) - 2`, NOT `5 - (3 - 2)`
- When parent is `-` and right child is also `-`, DO add parens to preserve order

**Commutative Operators:** `+` and `*`
- `5 + 3 - 2` is unambiguous (lower precedence on right needs parens)
- `5 * 3 / 2` needs careful handling (division is left-associative)

**Parenthesization Decision:**
1. If child precedence < parent precedence → ADD parens
2. If child precedence == parent precedence AND is_right AND operator in ("-", "/") → ADD parens
3. Otherwise → NO parens

---

## Testing and Validation

### Critical Test Cases from I/O Contract

**Must Pass:**
- Simple operations: `5 3 +` → `$5 + 3$`
- Precedence: `5 3 + 2 *` → `$( 5 + 3 ) \times 2$`
- Left-associativity: `5 3 - 2 -` → `$5 - 3 - 2$`
- Mixed operators: `10 2 / 3 + 4 *` → `$( 10 \div 2 + 3 ) \times 4$`
- Floating-point: `3.14 2 *` → `$3.14 \times 2$`
- Chained operations: `1 2 + 3 + 4 +` → `$1 + 2 + 3 + 4$`

**Error Handling:**
- Unsupported operator: `2 3 ^` → `Error: Unexpected character '^'` with exit code 1
- Error format must include source context with line number and caret

---

## Key Implementation Constraints

### Immutability and Thread Safety

- All data structures are logically immutable
- Once created, Token, ASTNode objects never change
- Go: Use value types and never modify fields after creation

### Precision and String Preservation

- Floating-point numbers are stored as strings, not parsed to float64
- This preserves exact decimal representation (e.g., "3.14" stays "3.14", not "3.1400000000000001")
- No arithmetic is performed; only string assembly

### Position Tracking

- Line and column are 1-based throughout (user-facing)
- Position tracking is critical for error reporting
- Must be accurate to character level

### Output Format

- LaTeX math mode delimiters: `$...$`
- Operator spacing: ` + `, ` - `, ` \times `, ` \div ` (spaces around operators)
- Parentheses spacing: `( expr )` (spaces inside)
- No trailing whitespace
- No newline in output (added by CLI if needed)

---

## Summary Table: File by File

| File | Type | Key Classes/Functions | Dependencies | Lines |
|------|------|----------------------|--------------|-------|
| tokens.py | Constants | TokenType, Token | None | 71 |
| ast_nodes.py | Data Structures | ASTNode, Number, BinaryOp | None | 91 |
| errors.py | Utilities | ErrorFormatter | None | 128 |
| lexer.py | Core | Lexer, LexerError | tokens | 201 |
| parser.py | Core | Parser, ParserError | tokens, ast_nodes | 184 |
| latex_gen.py | Core | LaTeXGenerator | ast_nodes | 185 |
| cli.py | Entry Point | main() | errors, latex_gen, lexer, parser | 115 |

**Total Implementation Lines:** ~975 (excluding blank lines and comments)

---

## Next Steps for Migrators

1. **Start with Layer 0:** Migrate `tokens.py` and `ast_nodes.py` first (no dependencies)
2. **Migrate Layer 1:** Migrate `errors.py` (independent utility)
3. **Migrate Layer 2:** Migrate `lexer.py` and `parser.py` (depend on tokens/ast)
4. **Migrate Layer 3:** Migrate `latex_gen.py` (depends on ast)
5. **Migrate Layer 4:** Migrate `cli.py` (depends on all others)
6. **Test at Each Step:** Use I/O contract test cases to validate each module
7. **Integration Test:** Run full pipeline with all 18 success cases and 3 error cases

---

## Additional Notes

### Python-Specific Features Not Directly Translatable

1. **Dataclass Freezing:** Go structs don't enforce immutability; rely on code discipline
2. **Singledispatch:** Go uses type switches instead; very different pattern
3. **Union Types:** Go uses interfaces; requires marker methods
4. **Exception Handling:** Different error semantics; Go uses error returns

### Recommended Go Version

- Go 1.18+ for better error handling and generics (if needed)
- Use `errors.Is()` and `errors.As()` for error type checking
- Consider `github.com/spf13/cobra` or `github.com/urfave/cli` for CLI if argparse-like features needed

### Performance Considerations

- Stack-based parsing is inherently efficient
- String concatenation for LaTeX output is acceptable (small outputs)
- No need for string builder optimization unless processing very large expressions
- Consider using `strings.Builder` for large-scale string concatenation if needed in future

---

**Document Complete**
