# RPN2TeX Python to Go Migration Specification

## Executive Summary

This document provides a comprehensive guide for migrating the rpn2tex codebase from Python 3.10+ to Go. The rpn2tex project is a compiler pipeline that converts Reverse Polish Notation (RPN) expressions to LaTeX mathematical notation.

**Migration Architecture:**
- **Phase 1 (Core):** tokens.go, ast.go, errors.go
- **Phase 2 (Pipeline):** lexer.go, parser.go, latex.go
- **Phase 3 (CLI):** cmd/rpn2tex/main.go

**Key Principles:**
- Preserve exact behavioral compatibility with Python implementation
- Follow Go idioms and conventions
- Maintain error reporting fidelity (line/column tracking)
- Implement zero-allocation critical paths where beneficial

---

## Module 1: tokens.py → token.go

### Module Overview

**Purpose:** Define token types and the Token data structure that represents lexical elements in RPN expressions.

**Responsibilities:**
- Define TokenType enum with 6 variants (NUMBER, PLUS, MINUS, MULT, DIV, EOF)
- Implement immutable Token struct carrying type, value, and position information
- Provide Token string representation for debugging

**Key Design Pattern:**
- Immutable data structure using frozen dataclass (Python) → struct with no mutating methods (Go)
- Use of enum for type safety

### Data Structures

#### TokenType (Enum)
```python
class TokenType(Enum):
    NUMBER = auto()  # Numeric values: 5, 3.14, -2
    PLUS = auto()    # + (addition)
    MINUS = auto()   # - (subtraction)
    MULT = auto()    # * (multiplication)
    DIV = auto()     # / (division)
    EOF = auto()     # End of input
```

**Go Mapping:**
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

// String() method for TokenType to match Python's .name attribute
func (t TokenType) String() string { /* ... */ }
```

#### Token (Dataclass)
```python
@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str
    line: int      # 1-based
    column: int    # 1-based
```

**Go Mapping:**
```go
type Token struct {
    Type   TokenType
    Value  string
    Line   int  // 1-based
    Column int  // 1-based
}

// Implement String() for debugging output
func (t Token) String() string {
    return fmt.Sprintf("Token(%s, %q, %d:%d)",
        t.Type.String(), t.Value, t.Line, t.Column)
}
```

### Public API

**Functions:**
- None (package-level functions)

**Types:**
- `TokenType` enum: STRING representation via `String()` method
- `Token` struct: All fields public (capitalized), immutable usage enforced by convention

**Constants:**
- TokenType constants (NUMBER, PLUS, MINUS, MULT, DIV, EOF)

### Dependencies

**Internal:** None

**External:**
- Python: `enum.Enum`, `enum.auto`, `dataclasses.dataclass`
- Go: Standard library only (`fmt` for String() methods)

### Go Migration Notes

#### Type Mappings

| Python | Go | Notes |
|--------|----|----|
| `Enum` with `auto()` | `iota` constants | Provides automatic numbering |
| `@dataclass(frozen=True)` | Plain `struct` | Go structs are value types (immutable semantics) |
| `str` | `string` | Token value, operator symbols |
| `int` | `int` | Line and column numbers |
| `type(token).__name__` | `Type.String()` | Debugging representation |

#### Pattern Changes

1. **Enum representation:**
   - Python: `TokenType.PLUS.name` → Go: Create `String()` method returning enum name
   - Use string switch/case when needed for parsing

2. **Token construction:**
   - Python: `Token(TokenType.NUMBER, "42", 1, 5)` → Go: `Token{Type: NUMBER, Value: "42", Line: 1, Column: 5}`
   - Consider factory function for cleaner construction if needed

3. **Immutability:**
   - Go doesn't enforce immutability, use as value type (passed by value or pointer to prevent copies)
   - No setters; construct completely before use

### Key Implementation Details

**Immutability Semantics:**
- Python: `frozen=True` prevents any field modification
- Go: Enforce by convention - only read fields, never modify after construction
- Consider making Token an immutable value type passed by value

**String Representation:**
```python
# Python output:
>>> Token(TokenType.NUMBER, "42", 1, 5)
Token(NUMBER, '42', 1:5)

# Must match in Go:
// Go output:
Token(NUMBER, "42", 1:5)
```

**Position Tracking:**
- Lines are 1-based (convention: matches source editor line numbers)
- Columns are 1-based (convention: matches cursor position)
- Critical for error reporting accuracy

### Testing Requirements

**Key test cases (from I/O Contract):**
- Token creation preserves position (line, column)
- Token type comparison works correctly
- String representation matches format for debugging

---

## Module 2: ast_nodes.py → ast.go

### Module Overview

**Purpose:** Define Abstract Syntax Tree node types for representing parsed RPN expressions.

**Responsibilities:**
- Define base ASTNode with position information
- Implement Number and BinaryOp expression nodes
- Manage Expr type union for expression polymorphism
- Support recursive expression tree construction

**Key Design Pattern:**
- Dataclass inheritance for node type hierarchy
- Type union/sum type for expression variants
- Position tracking (line, column) on all nodes

### Data Structures

#### ASTNode (Base Class)
```python
@dataclass(frozen=True)
class ASTNode:
    line: int     # 1-based
    column: int   # 1-based
```

**Go Mapping:**
Option 1 (Embedding) - Recommended for Go idiomatic approach:
```go
type ASTNode struct {
    Line   int
    Column int
}

type Number struct {
    ASTNode
    Value string
}

type BinaryOp struct {
    ASTNode
    Operator string
    Left     Expr
    Right    Expr
}
```

Option 2 (Interface):
```go
type Expr interface {
    GetLine() int
    GetColumn() int
}
```

**Recommendation:** Use embedding for simplicity, interface for polymorphism in LaTeX generator.

#### Number (Literal)
```python
@dataclass(frozen=True)
class Number(ASTNode):
    value: str  # String representation preserves precision
```

**Key Detail:** Value is stored as string, not parsed as float, to preserve exact representation.

#### BinaryOp (Operation)
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str   # "+", "-", "*", "/"
    left: Expr      # Recursive type
    right: Expr     # Recursive type
```

**Type Union:**
```python
Expr = Number | BinaryOp  # Python 3.10+ union syntax
```

**Go Mapping:**
```go
type Expr interface{}

// Or more explicitly (Go 1.18+):
type Expr interface {
    exprNode()  // Unexported method to prevent external implementations
}
```

### Public API

**Types:**
- `ASTNode` struct: Base type with `Line` and `Column` fields
- `Number` struct: Leaf node with `Value` field
- `BinaryOp` struct: Internal node with `Operator`, `Left`, `Right` fields
- `Expr` interface: Type constraint for all expression nodes

**Functions:**
- None at module level (construction via type literals)

### Dependencies

**Internal:**
- Depends on nothing (no imports from other modules)

**External:**
- Python: `dataclasses.dataclass`
- Go: No external dependencies

### Go Migration Notes

#### Type Mappings

| Python | Go | Notes |
|--------|----|----|
| `@dataclass(frozen=True)` inheritance | `struct` embedding | Golang idiom for composition |
| `Number \| BinaryOp` union type | `interface{}` or sealed interface | Go doesn't have sum types; use interface with unexported marker method |
| `str` for operator | `string` | "+", "-", "*", "/" |
| `str` for number value | `string` | Preserve exact representation (important for LaTeX output) |
| Recursive dataclass references | Pointer indirection | Use `*BinaryOp` for left/right children |

#### Pattern Changes

1. **Inheritance to Embedding:**
```python
# Python
@dataclass(frozen=True)
class Number(ASTNode):
    value: str
```

```go
// Go
type Number struct {
    ASTNode  // Embedded, not inherited
    Value    string
}

// Access: number.Line directly accesses embedded field
```

2. **Union Type Implementation:**
```python
# Python - simple union
Expr = Number | BinaryOp

# Usage: type checking
if isinstance(node, Number):
    ...
```

```go
// Go - interface-based polymorphism
type Expr interface {
    exprNode()  // Marker method prevents external implementations
}

// Implementation
func (*Number) exprNode()    {}
func (*BinaryOp) exprNode()  {}

// Usage: type assertion
if num, ok := node.(*Number); ok {
    ...
}
```

3. **Recursive Type Definition:**
```python
# Python - forward references handled by dataclass
class BinaryOp(ASTNode):
    left: Expr   # Expr defined later
    right: Expr
```

```go
// Go - must use pointer for recursive types
type BinaryOp struct {
    // ...
    Left  *Expr  // or define Expr interface and use pointers
    Right *Expr
}
```

4. **Constructor Pattern:**
```python
# Python - automatic via dataclass
num = Number(line=1, column=1, value="42")
```

```go
// Go - explicit struct literal or factory
num := &Number{
    ASTNode: ASTNode{Line: 1, Column: 1},
    Value:   "42",
}

// Or factory function
func NewNumber(line, column int, value string) *Number {
    return &Number{
        ASTNode: ASTNode{Line: line, Column: column},
        Value:   value,
    }
}
```

### Key Implementation Details

**Recursive Tree Structure:**
- Both left and right children of BinaryOp can be either Number or BinaryOp
- Python uses dataclass with forward reference to Expr type union
- Go must use pointers for recursive structures to avoid infinite type size

**Position Information Preservation:**
- Every node carries its source position
- Critical for error reporting (maps errors back to source)
- Must be preserved through parser and generator

**String Representation:**
- BinaryOp.operator is one of: "+", "-", "*", "/"
- Number.value is the original string representation from source (e.g., "3.14" not 3.14)
- Preserving string representation ensures LaTeX output precision matches source

**Immutability:**
- All nodes treated as immutable after construction
- No methods modify fields
- Safe to share references

### Testing Requirements

**Key test cases:**
- Number node creation with position tracking
- BinaryOp node creation with left/right children
- Tree traversal (recursive structure)
- Type assertions for polymorphism (used in LaTeX generator)
- Position information propagation

---

## Module 3: errors.py → errors.go

### Module Overview

**Purpose:** Provide compiler-style error formatting with source context and position indicators.

**Responsibilities:**
- Initialize formatter with complete source text
- Format errors with line numbers and caret positioning
- Extract context lines around error position
- Generate human-readable error output matching gcc/rustc style

**Key Design Pattern:**
- Stateful formatter initialized with source
- Context extraction (configurable context lines)
- Position-to-offset calculation for caret placement

### Data Structures

#### ErrorFormatter (Class)
```python
class ErrorFormatter:
    source: str
    lines: list[str]
```

**Go Mapping:**
```go
type ErrorFormatter struct {
    source string
    lines  []string
}

func NewErrorFormatter(source string) *ErrorFormatter {
    lines := strings.Split(source, "\n")
    return &ErrorFormatter{
        source: source,
        lines:  lines,
    }
}
```

### Public API

**Methods:**
- `__init__(source: str)` → `NewErrorFormatter(source string) *ErrorFormatter`
- `format_error(message: str, line: int, column: int, *, context_lines: int = 1) -> str`

**Note:** `_get_context` is private (internal helper).

### Dependencies

**Internal:** None

**External:**
- Python: None (standard library only: string operations)
- Go: Standard library (`strings`, `fmt`)

### Go Migration Notes

#### Type Mappings

| Python | Go | Notes |
|--------|----|----|
| `str` (source) | `string` | Immutable in both languages |
| `list[str]` (lines) | `[]string` | Split by newline (use `\n`, handle different line endings) |
| `int` (line/column) | `int` | 1-based indices |
| Method return `str` | `string` | Error message |

#### Pattern Changes

1. **Initialization:**
```python
# Python
formatter = ErrorFormatter("5 3 @")
```

```go
// Go
formatter := NewErrorFormatter("5 3 @")
```

2. **Method Call Pattern:**
```python
# Python - keyword-only argument
formatted = formatter.format_error("Error msg", 1, 5, context_lines=1)
```

```go
// Go - no keyword arguments; use optional parameter or functional options
// Option 1: Use pointer receiver with default
formatted := formatter.FormatError("Error msg", 1, 5, 1)

// Option 2: Use struct with builder
opts := ErrorOptions{ContextLines: 1}
formatted := formatter.FormatError("Error msg", 1, 5, opts)
```

3. **String Operations:**
```python
# Python - splitlines() handles various line endings
lines = source.splitlines()
```

```go
// Go - must handle line endings explicitly
lines := strings.Split(strings.TrimSuffix(source, "\n"), "\n")
// Or for Unix-style:
lines := strings.Split(source, "\n")
```

4. **String Formatting:**
```python
# Python - f-strings
parts.append(f"Error: {message}")
parts.append(f"{line_num:>{num_width}} | {line_content}")
caret_line = caret_prefix + " " * caret_pos + "^"
```

```go
// Go - fmt package
parts = append(parts, fmt.Sprintf("Error: %s", message))
parts = append(parts, fmt.Sprintf("%*d | %s", num_width, lineNum, lineContent))
caretLine = caretPrefix + strings.Repeat(" ", caretPos) + "^"
```

### Key Implementation Details

**Algorithm: Error Context Extraction**

```python
def _get_context(self, line, column, context_lines):
    error_idx = line - 1  # Convert 1-based to 0-based

    # Calculate range (clamped to valid indices)
    start_idx = max(0, error_idx - context_lines)
    end_idx = min(len(self.lines), error_idx + context_lines + 1)

    # Calculate line number width for alignment
    max_line_num = end_idx
    num_width = len(str(max_line_num))

    # Build output lines
    for idx in range(start_idx, end_idx):
        line_num = idx + 1
        line_content = self.lines[idx]

        # Format line with number
        prefix = f"{line_num:>{num_width}} | "
        result.append(f"{prefix}{line_content}")

        # Add caret on error line
        if idx == error_idx:
            caret_prefix = " " * num_width + " | "
            caret_pos = max(0, column - 1)
            caret_line = caret_prefix + " " * caret_pos + "^"
            result.append(caret_line)
```

**Critical Details:**
1. Line/column are 1-based; convert to 0-based for indexing
2. Caret position uses `column - 1` to align correctly
3. Line number width calculated for right-alignment
4. Context includes error line itself plus configurable context

**Output Format Example:**
```
Error: Unexpected character '@'

1 | 5 3 @
    ^
```

### Testing Requirements

**Key test cases:**
- Single-line error with caret positioning
- Multi-line context display
- Boundary conditions (first line, last line)
- Column position accuracy (caret alignment)
- Format string matching

---

## Module 4: lexer.py → lexer.go

### Module Overview

**Purpose:** Tokenize RPN expression text into a stream of tokens.

**Responsibilities:**
- Character-by-character scanning of input
- Position tracking (line, column) during scan
- Token creation for numbers, operators, and EOF
- Error detection and reporting with position context
- Whitespace handling as delimiter

**Key Design Pattern:**
- Stateful scanner maintaining position
- Token generation via `_scan_token()` method
- Error handling with position information

### Data Structures

#### LexerError (Exception)
```python
class LexerError(Exception):
    message: str
    line: int
    column: int
```

**Go Mapping:**
```go
type LexerError struct {
    Message string
    Line    int
    Column  int
}

func (e LexerError) Error() string {
    return fmt.Sprintf("Line %d, column %d: %s",
        e.Line, e.Column, e.Message)
}
```

#### Lexer (Class)
```python
class Lexer:
    text: str
    pos: int      # 0-based position
    line: int     # 1-based
    column: int   # 1-based
```

**Go Mapping:**
```go
type Lexer struct {
    text   string
    pos    int
    line   int
    column int
}

func NewLexer(text string) *Lexer {
    return &Lexer{
        text:   text,
        pos:    0,
        line:   1,
        column: 1,
    }
}
```

### Public API

**Functions:**
- `Lexer(text: str)` → `NewLexer(text string) *Lexer`
- `lexer.tokenize() -> list[Token]` → `(l *Lexer) Tokenize() ([]Token, error)`

**Error Types:**
- `LexerError` exception class → `LexerError` struct with Error() method

### Dependencies

**Internal:**
- Imports: `rpn2tex.tokens.Token`, `rpn2tex.tokens.TokenType`

**External:**
- Python: None (standard library string operations)
- Go: Standard library (`fmt` for error formatting)

### Go Migration Notes

#### Type Mappings

| Python | Go | Notes |
|--------|----|----|
| `Exception` subclass | Error interface | Implement `Error()` method |
| Exception attributes | Struct fields | Exported for context access |
| `list[Token]` return | `[]Token` | Slice type |
| `-> list[Token]` | `([]Token, error)` | Go multi-return for errors |
| `self.pos >= len(self.text)` | `l.pos >= len(l.text)` | Receiver syntax |

#### Pattern Changes

1. **Error Handling:**
```python
# Python - exceptions
class LexerError(Exception):
    def __init__(self, message, line, column):
        super().__init__(f"Line {line}, column {column}: {message}")
        self.message = message
        self.line = line
        self.column = column
```

```go
// Go - error interface
type LexerError struct {
    Message string
    Line    int
    Column  int
}

func (e LexerError) Error() string {
    return fmt.Sprintf("Line %d, column %d: %s",
        e.Line, e.Column, e.Message)
}

// Return style
return nil, &LexerError{
    Message: "Unexpected character '@'",
    Line:    1,
    Column:  5,
}
```

2. **Tokenize Return:**
```python
# Python - single return with list
def tokenize(self) -> list[Token]:
    tokens = []
    # ... build tokens
    return tokens

# Usage
tokens = lexer.tokenize()
```

```go
// Go - dual return with error
func (l *Lexer) Tokenize() ([]Token, error) {
    var tokens []Token
    // ... build tokens
    return tokens, nil
}

// Usage
tokens, err := lexer.Tokenize()
if err != nil {
    // handle error
}
```

3. **Position Tracking on Newline:**
```python
# Python
if char == "\n":
    self.line += 1
    self.column = 1
else:
    self.column += 1
```

```go
// Go - identical logic
if char == '\n' {
    l.line++
    l.column = 1
} else {
    l.column++
}
```

### Key Implementation Details

**Scanning Algorithm:**
```python
def tokenize(self):
    tokens = []
    while not self._at_end():
        self._skip_whitespace()
        if self._at_end():
            break
        tokens.append(self._scan_token())

    tokens.append(Token(TokenType.EOF, "", self.line, self.column))
    return tokens
```

**Critical Points:**
1. Whitespace is delimiter (skip) not preserved
2. EOF token added at end with current position
3. Position tracking updates on every character
4. Negative numbers: Check for digit after "-" to distinguish from minus operator

**Number Scanning:**
```python
def _scan_number(self, prefix, start_line, start_column):
    value = prefix

    # Integer part
    while not self._at_end() and self._peek().isdigit():
        value += self._advance()

    # Decimal part (optional)
    if not self._at_end() and self._peek() == ".":
        value += self._advance()
        while not self._at_end() and self._peek().isdigit():
            value += self._advance()

    return Token(TokenType.NUMBER, value, start_line, start_column)
```

**Important:** Stores number as string (e.g., "3.14"), not parsed float. This preserves exact representation.

**Operator Recognition:**
- `+` → PLUS token
- `-` → MINUS token (or negative number prefix if followed by digit)
- `*` → MULT token
- `/` → DIV token
- Other characters → LexerError

### Testing Requirements

**Key test cases (from I/O Contract):**
1. Simple expressions: "5 3 +" → [NUMBER(5), NUMBER(3), PLUS, EOF]
2. All operators: PLUS, MINUS, MULT, DIV
3. Decimal numbers: "3.14 2" → [NUMBER(3.14), NUMBER(2), EOF]
4. Position tracking: tokens have correct line/column
5. Whitespace handling: multiple spaces ignored
6. Negative numbers: "-5" → [NUMBER(-5)] when after whitespace or at start
7. Error on unknown char: "@" → LexerError with correct position
8. EOF token always present, at scanner's final position

---

## Module 5: parser.py → parser.go

### Module Overview

**Purpose:** Convert token stream to Abstract Syntax Tree using stack-based RPN parsing.

**Responsibilities:**
- Implement stack-based RPN algorithm
- Token consumption and EOF detection
- Binary operator handling with operand popping
- AST node construction
- Validation (operand count, final stack state)

**Key Design Pattern:**
- RPN parsing algorithm with explicit stack
- Token position tracking for error context
- Two-level validation (operand count, final stack)

### Data Structures

#### ParserError (Exception)
```python
class ParserError(Exception):
    message: str
    token: Token
```

**Go Mapping:**
```go
type ParserError struct {
    Message string
    Token   *Token
}

func (e ParserError) Error() string {
    return fmt.Sprintf("%s at line %d, column %d",
        e.Message, e.Token.Line, e.Token.Column)
}
```

#### Parser (Class)
```python
class Parser:
    tokens: list[Token]
    pos: int
```

**Go Mapping:**
```go
type Parser struct {
    tokens []Token
    pos    int
}

func NewParser(tokens []Token) *Parser {
    return &Parser{
        tokens: tokens,
        pos:    0,
    }
}
```

### Public API

**Functions:**
- `Parser(tokens: list[Token])` → `NewParser(tokens []Token) *Parser`
- `parser.parse() -> Expr` → `(p *Parser) Parse() (Expr, error)`

**Error Type:**
- `ParserError` exception → `ParserError` struct with Error() method

### Dependencies

**Internal:**
- Imports: `rpn2tex.ast_nodes` (BinaryOp, Expr, Number)
- Imports: `rpn2tex.tokens` (Token, TokenType)

**External:**
- Python: None (standard library)
- Go: Standard library (`fmt`)

### Go Migration Notes

#### Type Mappings

| Python | Go | Notes |
|--------|----|----|
| `list[Expr]` stack | `[]Expr` or `[]*Expr` | Slice of expression nodes |
| `-> Expr` return | `(Expr, error)` | Dual return with error |
| `isinstance(expr, BinaryOp)` | Type assertion `expr.(*BinaryOp)` | Go pattern matching |
| Dict mapping operators | String switch or map | TokenType to operator string |

#### Pattern Changes

1. **Stack-based Parsing:**
```python
# Python
stack = []
while not self._at_end():
    token = self._current()
    if token.type == TokenType.NUMBER:
        stack.append(Number(...))
    elif token.type in (TokenType.PLUS, ...):
        right = stack.pop()
        left = stack.pop()
        stack.append(BinaryOp(...))
```

```go
// Go - same logic, different syntax
var stack []Expr
for !p.atEnd() {
    token := p.current()
    switch token.Type {
    case NUMBER:
        stack = append(stack, &Number{...})
    case PLUS, MINUS, MULT, DIV:
        right := stack[len(stack)-1]
        stack = stack[:len(stack)-1]
        left := stack[len(stack)-1]
        stack = stack[:len(stack)-1]
        stack = append(stack, &BinaryOp{...})
    }
}
```

2. **Error Handling:**
```python
# Python - raise exception
raise ParserError("Error message", token)
```

```go
// Go - return error
return nil, &ParserError{
    Message: "Error message",
    Token:   &token,
}
```

3. **Operator Mapping:**
```python
# Python - dict
op_map = {
    TokenType.PLUS: "+",
    TokenType.MINUS: "-",
    TokenType.MULT: "*",
    TokenType.DIV: "/",
}
operator = op_map[token.type]
```

```go
// Go - switch or map
var operator string
switch token.Type {
case PLUS:
    operator = "+"
case MINUS:
    operator = "-"
case MULT:
    operator = "*"
case DIV:
    operator = "/"
}

// Or use map (slower):
var opMap = map[TokenType]string{
    PLUS:  "+",
    MINUS: "-",
    MULT:  "*",
    DIV:   "/",
}
operator := opMap[token.Type]
```

### Key Implementation Details

**RPN Parsing Algorithm:**

```python
def parse(self):
    stack = []

    while not self._at_end():
        token = self._current()

        if token.type == TokenType.NUMBER:
            # Push number onto stack
            num_node = Number(line=token.line, column=token.column, value=token.value)
            stack.append(num_node)
            self._advance()

        elif token.type in (TokenType.PLUS, TokenType.MINUS, TokenType.MULT, TokenType.DIV):
            # Pop two operands and create binary operation
            if len(stack) < 2:
                raise ParserError(f"Operator '{token.value}' requires two operands", token)

            right = stack.pop()
            left = stack.pop()

            operator = op_map[token.type]

            op_node = BinaryOp(
                line=token.line,
                column=token.column,
                operator=operator,
                left=left,
                right=right,
            )
            stack.append(op_node)
            self._advance()

        elif token.type == TokenType.EOF:
            break

    # Validate final state
    if len(stack) == 0:
        raise ParserError("Empty expression", self.tokens[-1])

    if len(stack) > 1:
        raise ParserError(
            f"Invalid RPN: {len(stack)} values remain on stack",
            self.tokens[-1],
        )

    return stack[0]
```

**Algorithm Explanation:**
1. Maintain operand stack
2. On NUMBER: create Number node, push to stack
3. On OPERATOR: pop 2 operands (error if < 2), create BinaryOp, push back
4. On EOF: exit loop
5. Validate: exactly 1 item on stack (otherwise empty or too many operands)

**Error Conditions:**
- Too few operands for operator (stack < 2) → ParserError at operator position
- Empty expression (final stack size 0) → ParserError at EOF
- Too many operands (final stack size > 1) → ParserError at EOF (missing operators)

**Position Preservation:**
- Each BinaryOp captures operator's position (line, column)
- Each Number captures token's position
- Used by LaTeX generator for potential error reporting

### Testing Requirements

**Key test cases (from I/O Contract):**
1. Simple: "5 3 +" → BinaryOp("+", Number("5"), Number("3"))
2. Complex: "5 3 + 2 *" → BinaryOp("*", BinaryOp("+", Number("5"), Number("3")), Number("2"))
3. Left-associative: "5 3 - 2 -" → BinaryOp("-", BinaryOp("-", Number("5"), Number("3")), Number("2"))
4. Mixed precedence: "2 3 4 * +" → BinaryOp("+", Number("2"), BinaryOp("*", Number("3"), Number("4")))
5. Error: Too few operands - single operator with no operands
6. Error: Too many operands - missing operator
7. Error: Empty expression
8. Decimal numbers preserved as strings in AST

---

## Module 6: latex_gen.py → latex.go

### Module Overview

**Purpose:** Convert AST to LaTeX mathematical notation with proper operator precedence handling.

**Responsibilities:**
- Visitor pattern dispatch for different AST node types
- LaTeX operator symbol mapping (*, / → \times, \div)
- Parenthesization logic based on operator precedence
- Left-associativity handling (- and /)

**Key Design Pattern:**
- Single dispatch visitor pattern (`@singledispatchmethod` in Python)
- Precedence table for parenthesis insertion
- Recursive tree traversal

### Data Structures

#### LaTeXGenerator (Class)
```python
class LaTeXGenerator:
    BINARY_OPS: ClassVar[dict[str, str]] = {
        "+": "+",
        "-": "-",
        "*": r"\times",
        "/": r"\div",
    }

    PRECEDENCE: ClassVar[dict[str, int]] = {
        "+": 1,
        "-": 1,
        "*": 2,
        "/": 2,
    }
```

**Go Mapping:**
```go
type LaTeXGenerator struct{}

var (
    binaryOps = map[string]string{
        "+": "+",
        "-": "-",
        "*": `\times`,
        "/": `\div`,
    }

    precedence = map[string]int{
        "+": 1,
        "-": 1,
        "*": 2,
        "/": 2,
    }
)

// Or as receiver fields:
type LaTeXGenerator struct {
    binaryOps map[string]string
    precedence map[string]int
}

func NewLaTeXGenerator() *LaTeXGenerator {
    return &LaTeXGenerator{
        binaryOps: map[string]string{...},
        precedence: map[string]int{...},
    }
}
```

### Public API

**Methods:**
- `LaTeXGenerator()` → `NewLaTeXGenerator() *LaTeXGenerator`
- `generator.generate(ast: Expr) -> str` → `(g *LaTeXGenerator) Generate(ast Expr) (string, error)`
- `_visit()` dispatcher → Private method or interface dispatch

**Note:** `_visit` and `_needs_parens` are private implementation details.

### Dependencies

**Internal:**
- Imports: `rpn2tex.ast_nodes` (BinaryOp, Expr, Number)

**External:**
- Python: `functools.singledispatchmethod`, `typing.ClassVar`
- Go: Standard library only

### Go Migration Notes

#### Type Mappings

| Python | Go | Notes |
|--------|----|----|
| `@singledispatchmethod` | Interface dispatch or type switch | Multiple dispatch patterns |
| `ClassVar[dict]` | Package-level `var` or receiver field | Static/global constants |
| `-> str` | `(string, error)` | Add error return for consistency |
| Instance method with dispatch | Method on interface receiver | Type assertion in switch |

#### Pattern Changes

1. **Single Dispatch Visitor:**
```python
# Python - singledispatchmethod decorator
class LaTeXGenerator:
    @singledispatchmethod
    def _visit(self, node: Expr) -> str:
        raise NotImplementedError(f"No visitor for {type(node).__name__}")

    @_visit.register
    def _visit_number(self, node: Number) -> str:
        return node.value

    @_visit.register
    def _visit_binary_op(self, node: BinaryOp) -> str:
        # ...
```

```go
// Go - type switch (explicit but clear)
func (g *LaTeXGenerator) visit(node Expr) (string, error) {
    switch n := node.(type) {
    case *Number:
        return g.visitNumber(n)
    case *BinaryOp:
        return g.visitBinaryOp(n)
    default:
        return "", fmt.Errorf("no visitor for %T", node)
    }
}

func (g *LaTeXGenerator) visitNumber(node *Number) (string, error) {
    return node.Value, nil
}

func (g *LaTeXGenerator) visitBinaryOp(node *BinaryOp) (string, error) {
    // ...
}
```

2. **Operator Mapping:**
```python
# Python - class variable
BINARY_OPS: ClassVar[dict[str, str]] = {
    "+": "+",
    "-": "-",
    "*": r"\times",
    "/": r"\div",
}

op_latex = self.BINARY_OPS[node.operator]
```

```go
// Go - package-level map or receiver field
var binaryOps = map[string]string{
    "+": "+",
    "-": "-",
    "*": `\times`,  // Raw string literal
    "/": `\div`,
}

opLatex := binaryOps[node.Operator]
```

3. **Precedence Lookup:**
```python
# Python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1, "-": 1,
    "*": 2, "/": 2,
}

my_precedence = self.PRECEDENCE[node.operator]
```

```go
// Go
var precedence = map[string]int{
    "+": 1, "-": 1,
    "*": 2, "/": 2,
}

myPrecedence := precedence[node.Operator]
```

### Key Implementation Details

**Generate Function Flow:**
```python
def generate(self, ast: Expr) -> str:
    content = self._visit(ast)
    return f"${content}$"
```

Wraps generated LaTeX in `$...$` delimiters (math mode).

**Visitor Dispatch for Numbers:**
```python
@_visit.register
def _visit_number(self, node: Number) -> str:
    return node.value
```

Numbers output verbatim (preserves "3.14", "-5", etc.).

**Visitor for Binary Operations:**
```python
@_visit.register
def _visit_binary_op(self, node: BinaryOp) -> str:
    op_latex = self.BINARY_OPS[node.operator]
    my_precedence = self.PRECEDENCE[node.operator]

    # Process left operand
    left = self._visit(node.left)
    if self._needs_parens(node.left, my_precedence, is_right=False):
        left = f"( {left} )"

    # Process right operand
    right = self._visit(node.right)
    if self._needs_parens(node.right, my_precedence, is_right=True):
        right = f"( {right} )"

    return f"{left} {op_latex} {right}"
```

**Parenthesization Logic:**
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

**Critical Details:**
- Spaces around operators: `5 + 3` not `5+3`
- Spaces inside parens: `( 5 + 3 )` not `(5+3)`
- Number nodes never get parens (leaves)
- Right-associative rule: `5 - (3 - 2)` needs parens; `(5 - 3) - 2` doesn't

**Examples:**
```
"5 3 +"       → "5 + 3"           (no parens needed)
"5 3 + 2 *"   → "( 5 + 3 ) * 2"   (+ has lower precedence than *)
"5 3 * 2 +"   → "5 * 3 + 2"       (no parens needed)
"5 3 - 2 -"   → "5 - 3 - 2"       (left-associative, no parens)
```

### Testing Requirements

**Key test cases (from I/O Contract - 18 passing tests):**
1. Basic: "5 3 +" → "$5 + 3$"
2. Subtraction: "5 3 -" → "$5 - 3$"
3. Multiplication: "4 7 *" → "$4 \times 7$"
4. Division: "10 2 /" → "$10 \div 2$"
5. Precedence: "5 3 + 2 *" → "$( 5 + 3 ) \times 2$"
6. Mixed: "2 3 4 * +" → "$2 + 3 \times 4$"
7. Left-associative: "5 3 - 2 -" → "$5 - 3 - 2$"
8. Complex: "10 2 / 3 + 4 *" → "$( 10 \div 2 + 3 ) \times 4$"
9. Decimals: "3.14 2 *" → "$3.14 \times 2$"
10. Multiple ops: "1 2 + 3 + 4 +" → "$1 + 2 + 3 + 4$"

---

## Module 7: cli.py → cmd/rpn2tex/main.go

### Module Overview

**Purpose:** Command-line interface orchestrating the complete pipeline.

**Responsibilities:**
- Argument parsing (input file, output file)
- File I/O (stdin, read file, write file)
- Pipeline orchestration (lex → parse → generate)
- Error handling and formatting
- Exit code management

**Key Design Pattern:**
- Main function returns exit code (for exit(code) in tests)
- Error handler for formatted output
- File I/O with exception handling

### Data Structures

No data structures defined in this module. Uses types from other modules.

### Public API

**Functions:**
- `main() -> int` → `func main()` with exit code via `os.Exit()`

**Note:** Python returns exit code from main(), Go's main() doesn't return (calls os.Exit directly or uses alternate pattern).

### Dependencies

**Internal:**
- `rpn2tex.errors.ErrorFormatter`
- `rpn2tex.latex_gen.LaTeXGenerator`
- `rpn2tex.lexer.Lexer, LexerError`
- `rpn2tex.parser.Parser, ParserError`

**External:**
- Python: `argparse`, `sys`, `pathlib.Path`
- Go: `flag`, `os`, `fmt`, `log`, standard library

### Go Migration Notes

#### Type Mappings

| Python | Go | Notes |
|--------|----|----|
| `sys.stdin.read()` | `io.ReadAll(os.Stdin)` | Read from stdin |
| `Path(file).read_text()` | `os.ReadFile()` | Read file |
| `Path(file).write_text()` | `os.WriteFile()` | Write file |
| `argparse.ArgumentParser` | `flag.FlagSet` | CLI argument parsing |
| `sys.stderr`/`sys.stdout` | `os.Stderr`/`os.Stdout` | Output streams |
| `return 1` | `os.Exit(1)` | Exit with code |

#### Pattern Changes

1. **Argument Parsing:**
```python
# Python - argparse
parser = argparse.ArgumentParser(description="...", prog="rpn2tex")
parser.add_argument("input", type=str, help="...")
parser.add_argument("-o", "--output", type=Path, help="...")
args = parser.parse_args()
```

```go
// Go - flag package
import "flag"

func main() {
    var outputPath string
    flag.StringVar(&outputPath, "o", "", "Output LaTeX file")
    flag.StringVar(&outputPath, "output", "", "Output LaTeX file (long form)")
    flag.Parse()

    args := flag.Args()
    if len(args) != 1 {
        fmt.Fprintf(os.Stderr, "Usage: rpn2tex [options] <input>\n")
        os.Exit(1)
    }
    inputPath := args[0]
}

// Or use a third-party package like github.com/spf13/cobra or github.com/jessevdk/go-flags
```

2. **File Reading:**
```python
# Python
try:
    if args.input == "-":
        text = sys.stdin.read()
    else:
        input_path = Path(args.input)
        text = input_path.read_text()
except FileNotFoundError:
    print(f"Error: ...", file=sys.stderr)
    return 1
```

```go
// Go
var text string
var err error

if inputPath == "-" {
    data, err := io.ReadAll(os.Stdin)
    if err != nil {
        fmt.Fprintf(os.Stderr, "Error reading stdin: %v\n", err)
        os.Exit(1)
    }
    text = string(data)
} else {
    data, err := os.ReadFile(inputPath)
    if err != nil {
        if errors.Is(err, os.ErrNotExist) {
            fmt.Fprintf(os.Stderr, "Error: Input file not found: %s\n", inputPath)
        } else if errors.Is(err, os.ErrPermission) {
            fmt.Fprintf(os.Stderr, "Error: Permission denied: %s\n", inputPath)
        } else {
            fmt.Fprintf(os.Stderr, "Error reading file: %v\n", err)
        }
        os.Exit(1)
    }
    text = string(data)
}
```

3. **Pipeline Orchestration:**
```python
# Python
formatter = ErrorFormatter(text)
try:
    lexer = Lexer(text)
    tokens = lexer.tokenize()

    parser_obj = Parser(tokens)
    ast = parser_obj.parse()

    generator = LaTeXGenerator()
    latex = generator.generate(ast)

except LexerError as e:
    formatted = formatter.format_error(e.message, e.line, e.column)
    print(formatted, file=sys.stderr)
    return 1
except ParserError as e:
    formatted = formatter.format_error(e.message, e.token.line, e.token.column)
    print(formatted, file=sys.stderr)
    return 1
```

```go
// Go
formatter := NewErrorFormatter(text)

lexer := NewLexer(text)
tokens, err := lexer.Tokenize()
if err != nil {
    lexerErr := err.(*LexerError)
    formatted := formatter.FormatError(lexerErr.Message, lexerErr.Line, lexerErr.Column)
    fmt.Fprintf(os.Stderr, "%s\n", formatted)
    os.Exit(1)
}

parser := NewParser(tokens)
ast, err := parser.Parse()
if err != nil {
    parserErr := err.(*ParserError)
    formatted := formatter.FormatError(parserErr.Message, parserErr.Token.Line, parserErr.Token.Column)
    fmt.Fprintf(os.Stderr, "%s\n", formatted)
    os.Exit(1)
}

generator := NewLaTeXGenerator()
latex, err := generator.Generate(ast)
if err != nil {
    fmt.Fprintf(os.Stderr, "Error generating LaTeX: %v\n", err)
    os.Exit(1)
}
```

4. **Output Writing:**
```python
# Python
if args.output is not None:
    try:
        args.output.write_text(latex + "\n")
        print(f"Generated: {args.output}", file=sys.stderr)
    except PermissionError:
        print(f"Error: Permission denied writing: {args.output}", file=sys.stderr)
        return 1
else:
    print(latex)
```

```go
// Go
if outputPath != "" {
    err := os.WriteFile(outputPath, []byte(latex+"\n"), 0644)
    if err != nil {
        fmt.Fprintf(os.Stderr, "Error writing file: %v\n", err)
        os.Exit(1)
    }
    fmt.Fprintf(os.Stderr, "Generated: %s\n", outputPath)
} else {
    fmt.Println(latex)
}
```

### Key Implementation Details

**Pipeline Flow:**
1. Parse CLI arguments
2. Read input (stdin or file)
3. Tokenize (Lexer)
4. Parse tokens to AST (Parser)
5. Generate LaTeX (Generator)
6. Write output (stdout or file)

**Error Handling:**
- LexerError → format with context, exit 1
- ParserError → format with context, exit 1
- File I/O errors → report and exit 1
- Success → output LaTeX and exit 0

**File I/O Patterns:**
- Input "-" → stdin
- Output omitted → stdout
- Both support file paths

**Status Messages:**
- Errors go to stderr
- Generated file path goes to stderr
- LaTeX output goes to stdout (or written to file)

### Testing Requirements

**Key test cases (from I/O Contract - end-to-end integration):**
1. "5 3 +" → stdout: "$5 + 3$"
2. All 18 passing cases from LaTeX generator
3. 3 failing cases (^ not supported) → stderr error message + exit 1
4. File reading
5. File writing
6. stdin reading (-)

---

## I/O Contract

### Overview

This section documents the I/O contract from Phase 0 that defines the expected behavior the Go implementation must satisfy. This is the ground truth for behavioral compatibility testing.

### Implementation Details

- **Source:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
- **Language:** Python 3.10+
- **Entry Point:** `rpn2tex.cli.main()`
- **Input Method:** stdin via `-` argument
- **Output Format:** LaTeX math mode expressions (e.g., `$...$`)

### Test Results Summary

- **Total Tests:** 21
- **Passed:** 18
- **Failed:** 3

### Test Cases

| # | Input | Expected Output | Status |
|---|-------|-----------------|--------|
| 1 | `5 3 +` | `$5 + 3$` | PASS |
| 2 | `5 3 -` | `$5 - 3$` | PASS |
| 3 | `4 7 *` | `$4 \times 7$` | PASS |
| 4 | `10 2 /` | `$10 \div 2$` | PASS |
| 5 | `2 3 ^` | `ERROR: Error: Unexpected character '^'` | FAIL |
| 6 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | PASS |
| 7 | `5 3 * 2 +` | `$5 \times 3 + 2$` | PASS |
| 8 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | PASS |
| 9 | `5 3 - 2 -` | `$5 - 3 - 2$` | PASS |
| 10 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | PASS |
| 11 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | PASS |
| 12 | `2 3 4 * +` | `$2 + 3 \times 4$` | PASS |
| 13 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | PASS |
| 14 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | PASS |
| 15 | `2 3 * 4 +` | `$2 \times 3 + 4$` | PASS |
| 16 | `2 3 ^ 4 *` | `ERROR: Error: Unexpected character '^'` | FAIL |
| 17 | `2 3 4 ^ ^` | `ERROR: Unexpected character '^'` | FAIL |
| 18 | `3.14 2 *` | `$3.14 \times 2$` | PASS |
| 19 | `1.5 0.5 +` | `$1.5 + 0.5$` | PASS |
| 20 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS |
| 21 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | PASS |

### Success Cases (18 Passing Tests)

**Test 1: Basic Addition**
- Input: `5 3 +`
- Expected Output: `$5 + 3$`

**Test 2: Subtraction**
- Input: `5 3 -`
- Expected Output: `$5 - 3$`

**Test 3: Multiplication**
- Input: `4 7 *`
- Expected Output: `$4 \times 7$`

**Test 4: Division**
- Input: `10 2 /`
- Expected Output: `$10 \div 2$`

**Test 6: Operator Precedence (Addition + Multiplication)**
- Input: `5 3 + 2 *`
- Expected Output: `$( 5 + 3 ) \times 2$`
- Note: Addition result parenthesized because multiplication has higher precedence

**Test 7: Operator Precedence (Multiplication + Addition)**
- Input: `5 3 * 2 +`
- Expected Output: `$5 \times 3 + 2$`
- Note: No parentheses; multiplication higher precedence naturally

**Test 8: Left-to-right Division and Multiplication**
- Input: `10 2 / 5 *`
- Expected Output: `$10 \div 2 \times 5$`
- Note: Same precedence, left-associative

**Test 9: Left-associative Subtraction**
- Input: `5 3 - 2 -`
- Expected Output: `$5 - 3 - 2$`
- Note: Reads as (5 - 3) - 2, no parentheses needed

**Test 10: Multiple Divisions**
- Input: `100 10 / 5 / 2 /`
- Expected Output: `$100 \div 10 \div 5 \div 2$`
- Note: All same precedence, left-associative

**Test 11: Multiple Additions**
- Input: `1 2 + 3 + 4 +`
- Expected Output: `$1 + 2 + 3 + 4$`

**Test 12: Operator Precedence (Addition inside Multiplication)**
- Input: `2 3 4 * +`
- Expected Output: `$2 + 3 \times 4$`
- Note: Multiplication naturally higher precedence

**Test 13: Parentheses for Lower Precedence Left Operand**
- Input: `2 3 + 4 *`
- Expected Output: `$( 2 + 3 ) \times 4$`
- Note: Addition has lower precedence than multiplication

**Test 14: Parentheses for Lower Precedence Right Operand**
- Input: `2 3 4 + *`
- Expected Output: `$2 \times ( 3 + 4 )$`
- Note: Addition on right side of multiplication

**Test 15: Mixed Operations**
- Input: `2 3 * 4 +`
- Expected Output: `$2 \times 3 + 4$`
- Note: Multiplication naturally higher precedence

**Test 18: Decimal Number Multiplication**
- Input: `3.14 2 *`
- Expected Output: `$3.14 \times 2$`
- Note: Decimal preserved exactly as in input

**Test 19: Decimal Number Addition**
- Input: `1.5 0.5 +`
- Expected Output: `$1.5 + 0.5$`

**Test 20: Two Additions Multiplied**
- Input: `1 2 + 3 4 + *`
- Expected Output: `$( 1 + 2 ) \times ( 3 + 4 )$`
- Note: Both additions parenthesized

**Test 21: Complex Expression**
- Input: `10 2 / 3 + 4 *`
- Expected Output: `$( 10 \div 2 + 3 ) \times 4$`
- Note: Division and addition grouped, then multiplied

### Error Cases (3 Failing Tests)

These inputs are intentionally not supported by the current Python implementation. The Go implementation should maintain the same behavior (reject with error).

**Test 5: Exponentiation Operator (Not Supported)**
- Input: `2 3 ^`
- Status: FAIL (Exit Code: 1)
- Expected Error Output:
  ```
  Error: Unexpected character '^'

  1 | 2 3 ^
    |     ^
  ```

**Test 16: Exponentiation in Expression (Not Supported)**
- Input: `2 3 ^ 4 *`
- Status: FAIL (Exit Code: 1)
- Expected Error Output:
  ```
  Error: Unexpected character '^'

  1 | 2 3 ^ 4 *
    |     ^
  ```

**Test 17: Multiple Exponentiation (Not Supported)**
- Input: `2 3 4 ^ ^`
- Status: FAIL (Exit Code: 1)
- Expected Error Output:
  ```
  Error: Unexpected character '^'

  1 | 2 3 4 ^ ^
    |       ^
  ```

### Validation Criteria

**For all 18 passing test cases:**
1. Input is correctly tokenized
2. Tokens are correctly parsed into AST
3. AST is correctly converted to LaTeX
4. Operator symbols match exactly: `+`, `-`, `\times`, `\div`
5. Parentheses placement matches exactly (including spaces around parens)
6. Decimal numbers preserved exactly as in input
7. Exit code is 0
8. Output goes to stdout (no stderr message)

**For the 3 failing test cases:**
1. Lexer raises error on unsupported character `^`
2. Error message format: "Error: Unexpected character '^'"
3. Context shown with line numbers and caret pointer
4. Error goes to stderr
5. Exit code is 1
6. Exact formatting of error context must match

### Module-to-Test Coverage

| Module | Relevant Tests | Purpose |
|--------|---|---------|
| tokens.py | 1-4, 18-19 | Token creation with position tracking |
| lexer.py | 1-21 | Character recognition, tokenization, error on ^ |
| parser.py | 1-21 | RPN stack-based parsing, validation |
| ast_nodes.py | 1-21 | AST node creation, tree structure |
| latex_gen.py | 1-4, 6-15, 18-21 | LaTeX output, precedence, parentheses |
| errors.py | 5, 16-17 | Error formatting with context |
| cli.py | 1-21 | End-to-end integration, pipeline |

---

## Migration Strategy Overview

### Phase 1: Core Types (Foundation)

**Order:** tokens.go → ast.go → errors.go

**Dependencies:**
- `tokens.go`: No dependencies
- `ast.go`: Depends on no other modules (self-contained type hierarchy)
- `errors.go`: No dependencies on other modules (string manipulation only)

**Why This Order:**
- tokens must be defined first (used by lexer and parser)
- ast can be defined independently (type definitions only)
- errors is independent utility

### Phase 2: Pipeline (Processing)

**Order:** lexer.go → parser.go → latex.go

**Dependencies:**
- `lexer.go`: Imports tokens
- `parser.go`: Imports tokens, ast
- `latex.go`: Imports ast

**Why This Order:**
- lexer depends on tokens (must come after)
- parser depends on both tokens and ast (must come after both)
- latex depends on ast (can be parallel with parser, but after ast)

### Phase 3: CLI (Integration)

**Order:** cmd/rpn2tex/main.go

**Dependencies:**
- main.go: Imports tokens, ast, errors, lexer, parser, latex

**Why Last:**
- Orchestrates all modules together
- Depends on everything
- Can only be completed after all modules exist

### Go Package Structure

```
cmd/rpn2tex/
  main.go                 # CLI entry point

rpn2tex/
  token.go               # Phase 1
  ast.go                 # Phase 1
  errors.go              # Phase 1
  lexer.go               # Phase 2
  parser.go              # Phase 2
  latex.go               # Phase 2

go.mod                    # Module definition
go.sum                    # Dependency checksums
```

### Testing Strategy

**Unit Tests (Phase 1):**
- token_test.go: TokenType, Token struct
- ast_test.go: ASTNode, Number, BinaryOp, Expr interface
- errors_test.go: ErrorFormatter output formatting

**Integration Tests (Phase 2):**
- lexer_test.go: Tokenization of full expressions
- parser_test.go: AST construction from tokens
- latex_test.go: LaTeX generation from AST

**End-to-End Tests (Phase 3):**
- main_test.go: Full pipeline with I/O contract validation
- Use the 21 test cases from I/O Contract

---

## Python-to-Go Idiom Mappings

### Classes and Structs

| Pattern | Python | Go |
|---------|--------|----|-|
| Simple data | `@dataclass` | `struct` |
| Immutable data | `@dataclass(frozen=True)` | `struct` (enforce by convention) |
| Constructor | `def __init__` | `NewTypeName()` function |
| Class method | `@classmethod` | Receiver function |
| Instance method | `def method(self)` | `(t *Type) Method()` receiver |
| Property | `@property` | Getter method `(t *Type) GetField() Type` |

### Inheritance and Polymorphism

| Pattern | Python | Go |
|---------|--------|----|-|
| Base class | `class Base:` | Embedding or interface |
| Inheritance | `class Child(Base):` | `struct { Base; ... }` |
| Override | Method with same name | Implement interface methods |
| Polymorphism | `isinstance()` | Type assertion `.(Type)` |
| Duck typing | No explicit interface | `interface{}` with type switch |

### Error Handling

| Pattern | Python | Go |
|---------|--------|----|-|
| Raise exception | `raise MyError(...)` | `return ..., &MyError{...}` |
| Catch exception | `except MyError as e:` | `if err, ok := err.(*MyError); ok` |
| Exception attributes | `e.message`, `e.code` | `err.Message`, `err.Code` |
| Error string | `str(e)` | `err.Error()` (interface) |

### Collections

| Pattern | Python | Go |
|---------|--------|----|-|
| Dynamic list | `list[T]` | `[]T` slice |
| Dictionary | `dict[K, V]` | `map[K]V` |
| Append | `list.append(x)` | `slice = append(slice, x)` |
| Pop | `list.pop()` | `item := list[len(list)-1]; list = list[:len(list)-1]` |
| Length | `len(list)` | `len(slice)` |
| Iteration | `for x in list:` | `for _, x := range slice:` |

### String Operations

| Pattern | Python | Go |
|---------|--------|----|-|
| String formatting | `f"{x}"` | `fmt.Sprintf("%v", x)` |
| String split | `"a,b".split(",")` | `strings.Split("a,b", ",")` |
| String repeat | `"x" * 5` | `strings.Repeat("x", 5)` |
| Format with padding | `f"{x:>5}"` | `fmt.Sprintf("%5d", x)` |
| Character code | `ord(c)` | `c` (bytes.Buffer recommended) |

### Dispatch and Polymorphism

| Pattern | Python | Go |
|---------|--------|----|-|
| `@singledispatchmethod` | Method dispatch by type | Interface methods or type switch |
| `isinstance(x, Type)` | Type checking | Type assertion `x.(*Type)` |
| Multiple handlers | `@method.register(Type)` | Method set on Type |
| Fallback handler | Base method | Default case in switch |

### File I/O

| Pattern | Python | Go |
|---------|--------|----|-|
| Read file | `Path(file).read_text()` | `os.ReadFile(file)` |
| Write file | `Path(file).write_text(x)` | `os.WriteFile(file, data, perm)` |
| Stdin | `sys.stdin.read()` | `io.ReadAll(os.Stdin)` |
| Stdout | `print(x)` | `fmt.Println(x)` |
| Stderr | `print(x, file=sys.stderr)` | `fmt.Fprintf(os.Stderr, ...)` |

### Argument Parsing

| Pattern | Python | Go |
|---------|--------|----|-|
| argparse | `argparse.ArgumentParser` | `flag` package or `cobra` |
| Positional arg | `parser.add_argument("file")` | `flag.Args()[0]` |
| Optional arg | `parser.add_argument("-o", "--output")` | `flag.StringVar(&var, "o", default, help)` |
| Parse | `parser.parse_args()` | `flag.Parse()` |

---

## Error Handling Strategy

### Python Exception Hierarchy

```
Exception
  ├─ LexerError (custom, with attributes)
  └─ ParserError (custom, with attributes)
```

### Go Error Pattern

```go
// Define custom error types
type LexerError struct {
    Message string
    Line    int
    Column  int
}

func (e LexerError) Error() string {
    return fmt.Sprintf("Line %d, column %d: %s",
        e.Line, e.Column, e.Message)
}

// Return errors from functions
func (l *Lexer) Tokenize() ([]Token, error) {
    if /* error condition */ {
        return nil, &LexerError{
            Message: "...",
            Line:    l.line,
            Column:  l.column,
        }
    }
    return tokens, nil
}

// Handle errors at call site
tokens, err := lexer.Tokenize()
if err != nil {
    // Can access fields via type assertion
    lexErr := err.(*LexerError)
    // ...
}
```

### Error Propagation in CLI

```go
// Lexer error
tokens, err := lexer.Tokenize()
if err != nil {
    lexErr := err.(*LexerError)
    formatted := formatter.FormatError(lexErr.Message, lexErr.Line, lexErr.Column)
    fmt.Fprintf(os.Stderr, "%s\n", formatted)
    os.Exit(1)
}

// Parser error
ast, err := parser.Parse()
if err != nil {
    parserErr := err.(*ParserError)
    formatted := formatter.FormatError(parserErr.Message, parserErr.Token.Line, parserErr.Token.Column)
    fmt.Fprintf(os.Stderr, "%s\n", formatted)
    os.Exit(1)
}
```

---

## Performance Considerations

### Zero-Allocation Opportunities

1. **Token Slice**: Pre-allocate capacity when possible
   ```go
   tokens := make([]Token, 0, 100)  // Pre-allocate for typical input
   ```

2. **String Building**: Consider `strings.Builder` for LaTeX generation
   ```go
   var sb strings.Builder
   sb.WriteString("$")
   sb.WriteString(latex)
   sb.WriteString("$")
   return sb.String()
   ```

3. **Precedence Lookup**: Use constants instead of map lookups if performance-critical
   ```go
   const (
       precPlus = 1
       precMult = 2
   )
   ```

### Memory-Efficient Patterns

1. **Position Tracking**: Line/column integers lightweight, no optimization needed
2. **AST Nodes**: Use pointers for child nodes (BinaryOp.Left, Right are *Expr)
3. **String Values**: Keep as strings (no parsing to float until needed)

---

## Testing Implementation Guide

### Test Case Structure

Each test should:
1. Create input (token list or AST)
2. Call the function/method
3. Assert output matches expected
4. For error cases, check error type and message

### Example Test Pattern

```go
func TestLexerTokenize(t *testing.T) {
    tests := []struct {
        name    string
        input   string
        want    []Token
        wantErr bool
    }{
        {
            name:  "simple addition",
            input: "5 3 +",
            want: []Token{
                {Type: NUMBER, Value: "5", Line: 1, Column: 1},
                {Type: NUMBER, Value: "3", Line: 1, Column: 3},
                {Type: PLUS, Value: "+", Line: 1, Column: 5},
                {Type: EOF, Value: "", Line: 1, Column: 7},
            },
            wantErr: false,
        },
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            lexer := NewLexer(tt.input)
            got, err := lexer.Tokenize()
            if (err != nil) != tt.wantErr {
                t.Errorf("Tokenize() error = %v, wantErr %v", err, tt.wantErr)
                return
            }
            if !reflect.DeepEqual(got, tt.want) {
                t.Errorf("Tokenize() = %v, want %v", got, tt.want)
            }
        })
    }
}
```

---

## Compilation and Deployment

### Go Module Setup

```go
module github.com/rpn2tex/rpn2tex

go 1.21

// No external dependencies needed for base implementation
```

### Building

```bash
go build -o rpn2tex ./cmd/rpn2tex
```

### Testing

```bash
go test ./...
go test -v ./...
go test -cover ./...
```

### Cross-Compilation

```bash
GOOS=darwin GOARCH=amd64 go build -o rpn2tex ./cmd/rpn2tex  # macOS
GOOS=linux GOARCH=amd64 go build -o rpn2tex ./cmd/rpn2tex   # Linux
GOOS=windows GOARCH=amd64 go build -o rpn2tex.exe ./cmd/rpn2tex  # Windows
```

---

## Summary of Key Differences

| Aspect | Python | Go | Migration Approach |
|--------|--------|----|-|
| Enums | `Enum` with `auto()` | `iota` constants | Map auto() to iota |
| Immutable data | `@dataclass(frozen=True)` | struct (convention) | No setters; value semantics |
| Type unions | `A \| B` | interface{} | Interface with unexported method |
| Exceptions | raise/except | return error | Dual return (value, error) |
| Methods | `def method(self)` | `(r *Type) Method()` | Receiver functions |
| Dispatching | `@singledispatchmethod` | interface or type switch | Type assertions in switch |
| Strings | f-strings | fmt.Sprintf | Use fmt package |
| Collections | list, dict | slice, map | Direct equivalents |
| File I/O | Path(...).read_text() | os.ReadFile() | Use os/io packages |
| CLI | argparse | flag or cobra | flag.FlagSet |

---

## Appendix: File Size and Complexity Estimates

| Module | Python LoC | Go LoC (Est.) | Complexity |
|--------|-----------|--------------|-----------|
| tokens.py | 71 | 120 | Low |
| ast_nodes.py | 91 | 150 | Low |
| errors.py | 128 | 200 | Medium |
| lexer.py | 201 | 300 | High |
| parser.py | 184 | 280 | High |
| latex_gen.py | 185 | 280 | High |
| cli.py | 114 | 200 | Medium |
| **Total** | **974** | **1,530** | - |

*Note: Go implementations typically 1.5-1.7x larger due to explicit type annotations and error handling.*

---

## Verification Checklist

Before considering migration complete, verify:

- [ ] All 18 passing test cases produce identical output
- [ ] All 3 failing test cases produce identical error messages
- [ ] Error formatting matches exactly (line numbers, caret position, spacing)
- [ ] Exit codes are correct (0 for success, 1 for errors)
- [ ] Decimal numbers preserved exactly (3.14 not 3.1400...)
- [ ] Parentheses placement matches exactly (including spaces)
- [ ] LaTeX operators mapped correctly (× → \times, ÷ → \div)
- [ ] Position tracking accurate (line/column off-by-one errors)
- [ ] File I/O works (stdin, stdout, file read/write)
- [ ] Error types distinguishable (LexerError vs ParserError)
- [ ] Package structure matches Go conventions
- [ ] No compiler warnings or linting issues
- [ ] Tests pass with -race flag
- [ ] Memory profiling shows no leaks

---

## References and Further Reading

### Go Idioms
- https://go.dev/doc/effective_go
- https://github.com/golang/go/wiki/CodeReviewComments

### Error Handling in Go
- https://gobyexample.com/errors
- https://dave.cheney.net/2016/04/27/dont-just-check-errors-handle-them-gracefully

### Go Testing
- https://golang.org/pkg/testing/
- https://gobyexample.com/testing

### Type System
- https://go.dev/doc/tutorial/generics
- https://go.dev/spec#Interface_types

---

**Document Version:** 1.0
**Created:** 2025-12-29
**Migration Target:** Go 1.21+
**Python Source:** Python 3.10+
**Status:** Ready for Phase 1 Migration
