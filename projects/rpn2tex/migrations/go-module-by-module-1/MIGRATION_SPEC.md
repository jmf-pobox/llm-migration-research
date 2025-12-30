# rpn2tex Python to Go Migration Specification

**Document Type:** Phase 1 Migration Specification
**Target Language:** Go
**Migration Strategy:** Module-by-Module
**Source Location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
**Output Location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/go-module-by-module-1/`

---

## Table of Contents

1. [Overview](#overview)
2. [Module-by-Module Analysis](#module-by-module-analysis)
3. [Dependency Graph](#dependency-graph)
4. [Type Mappings](#type-mappings)
5. [Python Patterns and Go Equivalents](#python-patterns-and-go-equivalents)
6. [Implementation Order](#implementation-order)
7. [I/O Contract](#io-contract)
8. [Validation and Testing](#validation-and-testing)

---

## Overview

The rpn2tex project is a RPN (Reverse Polish Notation) to LaTeX expression converter. It consists of seven Python modules implementing a complete pipeline:

- **Tokens** → Token types and Token dataclass
- **Lexer** → Tokenization with position tracking
- **Parser** → Stack-based RPN parsing
- **AST Nodes** → Abstract Syntax Tree structures
- **LaTeX Generator** → AST to LaTeX conversion with precedence handling
- **Error Formatter** → User-friendly error messages with context
- **CLI** → Command-line interface orchestrating the pipeline

The implementation demonstrates:
- Enum usage for token types
- Dataclass frozen types for immutable structures
- Position tracking (line/column) for error reporting
- Visitor pattern for AST traversal
- Operator precedence rules
- Error handling with context

---

## Module-by-Module Analysis

### Module 1: tokens.py

**File Path:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/tokens.py`

#### Public API

**Enum:**
- `TokenType` - Enum with values: NUMBER, PLUS, MINUS, MULT, DIV, EOF

**Dataclass (frozen):**
- `Token(type: TokenType, value: str, line: int, column: int)`
  - `__repr__() -> str` - String representation for debugging

#### Dependencies

- Internal: None
- External: `dataclasses`, `enum`, `__future__` (annotations)

#### Key Implementation Details

```python
@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str
    line: int
    column: int
```

The Token class is **immutable** (frozen=True), representing a lexical element with position information. This is critical for error reporting.

#### Go Migration Notes

**Type Mappings:**
- `TokenType` (Enum) → `TokenType` (iota-based const)
- `Token` (frozen dataclass) → `Token` struct (with validation methods)
- `auto()` → `iota` with increment

**Pattern Changes:**
- Python enum with `auto()` → Go const with iota
- Frozen dataclass → struct (immutability enforced through method receivers)
- `__repr__()` → `String()` method (implements `fmt.Stringer` interface)
- `value: str` stored in Token for later retrieval

**Special Handling:**
- Token immutability can be enforced through design (no setter methods)
- Use string constants for operator values to avoid duplication

**Recommended Go Structure:**

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

type Token struct {
    Type   TokenType
    Value  string
    Line   int
    Column int
}

func (t Token) String() string {
    return fmt.Sprintf("Token(%s, %q, %d:%d)", t.TypeString(), t.Value, t.Line, t.Column)
}
```

---

### Module 2: ast_nodes.py

**File Path:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/ast_nodes.py`

#### Public API

**Dataclass (frozen):**
- `ASTNode(line: int, column: int)` - Base class for all AST nodes
- `Number(line: int, column: int, value: str)` - Numeric literal node
- `BinaryOp(line: int, column: int, operator: str, left: Expr, right: Expr)` - Binary operation node

**Type Alias:**
- `Expr = Number | BinaryOp` - Union type for expressions

#### Dependencies

- Internal: None
- External: `dataclasses`, `__future__` (annotations)

#### Key Implementation Details

```python
@dataclass(frozen=True)
class ASTNode:
    line: int
    column: int

@dataclass(frozen=True)
class Number(ASTNode):
    value: str

@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str
    left: Expr
    right: Expr

Expr = Number | BinaryOp
```

AST nodes are immutable and carry position information for error reporting. The `Expr` union type is used for type-safe expression handling.

#### Go Migration Notes

**Type Mappings:**
- `ASTNode` (base frozen dataclass) → `ASTNode` (interface)
- `Number` (frozen dataclass) → `Number` struct
- `BinaryOp` (frozen dataclass) → `BinaryOp` struct
- `Expr` (union type) → `Expr` (interface)

**Pattern Changes:**
- Inheritance (Python dataclass inheritance) → Interface implementation (Go)
- Union type → Interface (Go's duck typing)
- Dataclass inheritance → Composition or embedded structs

**Special Handling:**
- Define `Expr` interface with common methods
- Implement `Expr` interface by Number and BinaryOp
- Use embedding for shared line/column fields

**Recommended Go Structure:**

```go
type Expr interface {
    exprNode()
    GetLine() int
    GetColumn() int
}

type Number struct {
    Line   int
    Column int
    Value  string
}

func (n *Number) exprNode()     {}
func (n *Number) GetLine() int  { return n.Line }
func (n *Number) GetColumn() int { return n.Column }

type BinaryOp struct {
    Line     int
    Column   int
    Operator string
    Left     Expr
    Right    Expr
}

func (b *BinaryOp) exprNode()     {}
func (b *BinaryOp) GetLine() int  { return b.Line }
func (b *BinaryOp) GetColumn() int { return b.Column }
```

---

### Module 3: errors.py

**File Path:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/errors.py`

#### Public API

**Class:**
- `ErrorFormatter(source: str)` - Formats errors with source context
  - `__init__(source: str)` - Initialize with source text
  - `format_error(message: str, line: int, column: int, context_lines: int = 1) -> str` - Format error with context
  - `_get_context(line: int, column: int, context_lines: int) -> str` - Extract context around error

#### Dependencies

- Internal: None
- External: `__future__` (annotations)

#### Key Implementation Details

```python
class ErrorFormatter:
    source: str
    lines: list[str]

    def __init__(self, source: str) -> None:
        self.source = source
        self.lines = source.splitlines()

    def format_error(self, message: str, line: int, column: int,
                     context_lines: int = 1) -> str:
        # Format error with context

    def _get_context(self, line: int, column: int, context_lines: int) -> str:
        # Extract source context with line numbers and caret
```

The ErrorFormatter produces gcc/rustc-style error messages with:
- Error message header
- Source lines with line numbers
- Caret (^) at error position

#### Go Migration Notes

**Type Mappings:**
- `ErrorFormatter` class → `ErrorFormatter` struct
- Instance methods → Methods on struct pointer receivers
- Default parameter `context_lines: int = 1` → Separate parameter or helper method

**Pattern Changes:**
- Class attributes → Struct fields
- Instance methods → Pointer receiver methods
- String formatting → `strings.Builder` or `fmt.Sprintf`
- List slicing and building → Manual slice building

**Special Handling:**
- `splitlines()` → `strings.Split()` with careful handling of line endings
- Line number width calculation for alignment
- Caret positioning (1-based column to 0-based spaces)

**Recommended Go Structure:**

```go
type ErrorFormatter struct {
    Source string
    Lines  []string
}

func NewErrorFormatter(source string) *ErrorFormatter {
    lines := strings.Split(source, "\n")
    return &ErrorFormatter{Source: source, Lines: lines}
}

func (ef *ErrorFormatter) FormatError(message string, line, column int) string {
    return ef.FormatErrorWithContext(message, line, column, 1)
}

func (ef *ErrorFormatter) FormatErrorWithContext(message string, line, column, contextLines int) string {
    // Implementation
}

func (ef *ErrorFormatter) getContext(line, column, contextLines int) string {
    // Implementation
}
```

---

### Module 4: lexer.py

**File Path:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/lexer.py`

#### Public API

**Exception Class:**
- `LexerError(message: str, line: int, column: int)` - Lexer error with position
  - `message: str` - Error description
  - `line: int` - Line number
  - `column: int` - Column number

**Class:**
- `Lexer(text: str)` - Tokenizes input text
  - `__init__(text: str)` - Initialize with input
  - `tokenize() -> list[Token]` - Tokenize entire input
  - `_at_end() -> bool` - Check end of input
  - `_peek() -> str` - Look at current character
  - `_advance() -> str` - Consume character
  - `_skip_whitespace() -> None` - Skip whitespace
  - `_scan_token() -> Token` - Scan next token
  - `_scan_number(prefix: str, start_line: int, start_column: int) -> Token` - Scan numeric literal

#### Dependencies

- Internal: `tokens` (Token, TokenType)
- External: `__future__` (annotations)

#### Key Implementation Details

```python
class Lexer:
    text: str
    pos: int      # Current position (0-based)
    line: int     # Current line (1-based)
    column: int   # Current column (1-based)

    def tokenize(self) -> list[Token]:
        # Main entry point
        # Returns list of tokens ending with EOF
```

The lexer:
1. Scans character by character
2. Tracks line and column numbers
3. Handles negative numbers (minus followed by digit)
4. Raises `LexerError` for invalid characters

**Critical Logic:**
- Whitespace is delimiter and ignored
- `-` followed by digit is a negative number
- `-` not followed by digit is subtraction operator
- Decimal numbers supported (integer.fractional)

#### Go Migration Notes

**Type Mappings:**
- `LexerError` exception → Custom error type (implements error interface)
- `Lexer` class → `Lexer` struct
- Instance methods → Methods on struct pointer receivers

**Pattern Changes:**
- Exception handling → Error interface returns
- String indexing (Python) → Rune iteration or byte slicing
- Mutable instance state → Struct fields
- Character checking (`isdigit()`) → Unicode checking functions

**Special Handling:**
- Line/column tracking on newlines and other characters
- Negative number detection requires lookahead
- Return errors using Go's error interface
- Handle rune vs byte differences for UTF-8

**Recommended Go Structure:**

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
    Text   string
    Pos    int
    Line   int
    Column int
}

func NewLexer(text string) *Lexer {
    return &Lexer{Text: text, Pos: 0, Line: 1, Column: 1}
}

func (l *Lexer) Tokenize() ([]Token, error) {
    // Implementation
}

func (l *Lexer) atEnd() bool { /* ... */ }
func (l *Lexer) peek() rune { /* ... */ }
func (l *Lexer) advance() rune { /* ... */ }
func (l *Lexer) skipWhitespace() { /* ... */ }
func (l *Lexer) scanToken() (Token, error) { /* ... */ }
func (l *Lexer) scanNumber(prefix string, startLine, startColumn int) Token { /* ... */ }
```

---

### Module 5: parser.py

**File Path:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/parser.py`

#### Public API

**Exception Class:**
- `ParserError(message: str, token: Token)` - Parser error with token context
  - `message: str` - Error description
  - `token: Token` - Token where error occurred

**Class:**
- `Parser(tokens: list[Token])` - Stack-based RPN parser
  - `__init__(tokens: list[Token])` - Initialize with token list
  - `parse() -> Expr` - Parse tokens into AST
  - `_current() -> Token` - Get current token
  - `_at_end() -> bool` - Check if at EOF
  - `_advance() -> Token` - Consume and return current token

#### Dependencies

- Internal: `ast_nodes` (BinaryOp, Expr, Number), `tokens` (Token, TokenType)
- External: `__future__` (annotations)

#### Key Implementation Details

```python
class Parser:
    tokens: list[Token]
    pos: int  # Current position in token list

    def parse(self) -> Expr:
        stack: list[Expr] = []

        # For each token:
        # - NUMBER: push Number node onto stack
        # - OPERATOR: pop two operands, create BinaryOp, push result
        # - EOF: stop

        # Validate: exactly one item on stack
```

The parser implements **stack-based RPN parsing**:
1. Push numbers onto stack
2. When operator encountered: pop two operands, create BinaryOp, push result
3. At EOF, stack should contain exactly one AST node

**Error Conditions:**
- Operator with fewer than 2 stack items
- EOF with 0 items on stack (empty expression)
- EOF with >1 items on stack (missing operators)

#### Go Migration Notes

**Type Mappings:**
- `ParserError` exception → Custom error type
- `Parser` class → `Parser` struct
- `stack: list[Expr]` → `[]Expr` slice

**Pattern Changes:**
- Exception handling → Error interface returns
- Type assertion in Python → Type assertion in Go
- Dictionary mapping → Switch statement or map
- Dynamic type access → Interface methods

**Special Handling:**
- Stack operations (push/pop) → Append and slice operations
- Token type checking → Go switch on token.Type
- Error wrapping → Error message construction

**Recommended Go Structure:**

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
    Tokens []Token
    Pos    int
}

func NewParser(tokens []Token) *Parser {
    return &Parser{Tokens: tokens, Pos: 0}
}

func (p *Parser) Parse() (Expr, error) {
    stack := []Expr{}

    for !p.atEnd() {
        token := p.current()

        switch token.Type {
        case NUMBER:
            // Push number onto stack
        case PLUS, MINUS, MULT, DIV:
            // Pop operands, create BinaryOp, push
        case EOF:
            break
        default:
            return nil, &ParserError{Message: "...", Token: token}
        }
    }

    // Validate stack
}

func (p *Parser) current() Token { /* ... */ }
func (p *Parser) atEnd() bool { /* ... */ }
func (p *Parser) advance() Token { /* ... */ }
```

---

### Module 6: latex_gen.py

**File Path:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/latex_gen.py`

#### Public API

**Class:**
- `LaTeXGenerator()` - Converts AST to LaTeX
  - `generate(ast: Expr) -> str` - Generate LaTeX from AST
  - `_visit(node: Expr) -> str` - Visit AST node (dispatcher)
  - `_visit_number(node: Number) -> str` - Handle Number node
  - `_visit_binary_op(node: BinaryOp) -> str` - Handle BinaryOp node
  - `_needs_parens(child: Expr, parent_precedence: int, is_right: bool) -> bool` - Check if parens needed

**Class Attributes:**
- `BINARY_OPS: dict[str, str]` - Operator to LaTeX mapping
  - `"+"` → `"+"`
  - `"-"` → `"-"`
  - `"*"` → `r"\times"`
  - `"/"` → `r"\div"`
- `PRECEDENCE: dict[str, int]` - Operator precedence
  - `"+"`, `"-"` → `1`
  - `"*"`, `"/"` → `2`

#### Dependencies

- Internal: `ast_nodes` (BinaryOp, Expr, Number)
- External: `functools` (singledispatchmethod), `typing` (ClassVar), `__future__` (annotations)

#### Key Implementation Details

```python
class LaTeXGenerator:
    BINARY_OPS: ClassVar[dict[str, str]] = {
        "+": "+",
        "-": "-",
        "*": r"\times",
        "/": r"\div",
    }

    PRECEDENCE: ClassVar[dict[str, int]] = {
        "+": 1, "-": 1,
        "*": 2, "/": 2,
    }

    def generate(self, ast: Expr) -> str:
        content = self._visit(ast)
        return f"${content}$"

    @singledispatchmethod
    def _visit(self, node: Expr) -> str:
        raise NotImplementedError(f"No visitor for {type(node).__name__}")

    @_visit.register
    def _visit_number(self, node: Number) -> str:
        return node.value

    @_visit.register
    def _visit_binary_op(self, node: BinaryOp) -> str:
        # Handle precedence and parenthesization
        # Return infix notation with LaTeX operators
```

**Critical Logic:**
1. Visitor pattern dispatches on node type
2. BinaryOp needs precedence checking
3. Left operand with lower precedence needs parens
4. Right operand with equal precedence needs parens (for left-associative operators)
5. Special case: `-` and `/` on right side with same precedence need parens

**Examples:**
- `5 + 3 * 2` → `5 + 3 \times 2` (no parens, * binds tighter)
- `5 * 3 + 2` → `5 \times 3 + 2` (no parens, * binds tighter)
- `(5 + 3) * 2` → `( 5 + 3 ) \times 2` (parens needed, + has lower precedence)
- `5 - (3 - 2)` → `5 - ( 3 - 2 )` (parens on right, left-associative)

#### Go Migration Notes

**Type Mappings:**
- `LaTeXGenerator` class → `LaTeXGenerator` struct
- Class variables (`ClassVar`) → Package-level constants or struct fields
- `@singledispatchmethod` → Type switch or type assertion
- `dict[str, str]` → `map[string]string`

**Pattern Changes:**
- Visitor pattern with `@singledispatchmethod` → Manual type switching
- Class method dispatch → Type assertion switch or interface methods
- Python type dispatch → Go interface + type switch

**Special Handling:**
- Replace Python's `singledispatchmethod` with explicit type switching
- Precedence rules must be preserved exactly
- LaTeX escape sequences (backslashes) in strings
- Integer division handled same as subtraction

**Recommended Go Structure:**

```go
type LaTeXGenerator struct{}

var (
    BinaryOps = map[string]string{
        "+": "+",
        "-": "-",
        "*": `\times`,
        "/": `\div`,
    }

    Precedence = map[string]int{
        "+": 1, "-": 1,
        "*": 2, "/": 2,
    }
)

func (g *LaTeXGenerator) Generate(ast Expr) string {
    content := g.visit(ast)
    return "$" + content + "$"
}

func (g *LaTeXGenerator) visit(node Expr) string {
    switch n := node.(type) {
    case *Number:
        return g.visitNumber(n)
    case *BinaryOp:
        return g.visitBinaryOp(n)
    default:
        panic(fmt.Sprintf("No visitor for %T", node))
    }
}

func (g *LaTeXGenerator) visitNumber(node *Number) string {
    return node.Value
}

func (g *LaTeXGenerator) visitBinaryOp(node *BinaryOp) string {
    // Implementation
}

func (g *LaTeXGenerator) needsParens(child Expr, parentPrecedence int, isRight bool) bool {
    // Implementation
}
```

---

### Module 7: cli.py

**File Path:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/cli.py`

#### Public API

**Function:**
- `main() -> int` - Main CLI entry point
  - Returns: 0 for success, 1 for error

#### Dependencies

- Internal: `errors` (ErrorFormatter), `latex_gen` (LaTeXGenerator), `lexer` (Lexer, LexerError), `parser` (Parser, ParserError)
- External: `argparse`, `sys`, `pathlib` (Path), `__future__` (annotations)

#### Key Implementation Details

```python
def main() -> int:
    # 1. Parse command-line arguments
    parser = argparse.ArgumentParser(...)
    parser.add_argument("input", ...)
    parser.add_argument("-o", "--output", ...)
    args = parser.parse_args()

    # 2. Read input (from file or stdin)
    try:
        if args.input == "-":
            text = sys.stdin.read()
        else:
            text = Path(args.input).read_text()
    except (FileNotFoundError, PermissionError, IsADirectoryError):
        return 1

    # 3. Process: lexer → parser → generator
    formatter = ErrorFormatter(text)
    try:
        tokens = Lexer(text).tokenize()
        ast = Parser(tokens).parse()
        latex = LaTeXGenerator().generate(ast)
    except LexerError as e:
        print(formatter.format_error(e.message, e.line, e.column), file=sys.stderr)
        return 1
    except ParserError as e:
        print(formatter.format_error(e.message, e.token.line, e.token.column), file=sys.stderr)
        return 1

    # 4. Write output (to file or stdout)
    if args.output is not None:
        try:
            args.output.write_text(latex + "\n")
            print(f"Generated: {args.output}", file=sys.stderr)
        except (PermissionError, IsADirectoryError):
            return 1
    else:
        print(latex)

    return 0
```

**Pipeline:**
1. Parse arguments
2. Read input from file or stdin
3. Lexer: text → tokens
4. Parser: tokens → AST
5. LaTeX Generator: AST → LaTeX string
6. Write output to file or stdout

**Error Handling:**
- File I/O errors → return 1
- LexerError → format error with context, return 1
- ParserError → format error with context, return 1

#### Go Migration Notes

**Type Mappings:**
- `argparse.ArgumentParser` → `flag` package or `github.com/spf13/cobra`
- `sys.stdin` → `os.Stdin`
- `sys.stderr` → `os.Stderr`
- `pathlib.Path` → `os` package functions
- Exception handling → Error interface checks

**Pattern Changes:**
- argparse → flag or cobra CLI libraries
- Exception catching → Error type assertions
- Mutable path object → Immutable path strings
- Context manager handling → Explicit file operations

**Special Handling:**
- Use `flag` for simple CLI or `cobra` for complex CLI
- Error type assertions with ok pattern
- File reading/writing with explicit error handling
- Stdout/stderr output with `fmt.Print` or `log` package

**Recommended Go Structure:**

```go
func main() {
    flag.String("o", "", "Output LaTeX file (default: stdout)")
    flag.String("input", "", "Input RPN file (use '-' for stdin)")
    flag.Parse()

    if err := run(); err != nil {
        fmt.Fprintf(os.Stderr, "%v\n", err)
        os.Exit(1)
    }
    os.Exit(0)
}

func run() error {
    // Read input
    var text string
    if *input == "-" {
        b, err := io.ReadAll(os.Stdin)
        if err != nil {
            return fmt.Errorf("error reading stdin: %w", err)
        }
        text = string(b)
    } else {
        b, err := os.ReadFile(*input)
        if err != nil {
            return fmt.Errorf("error reading file: %w", err)
        }
        text = string(b)
    }

    // Process
    formatter := NewErrorFormatter(text)
    tokens, err := NewLexer(text).Tokenize()
    if err != nil {
        // Handle LexerError
    }

    ast, err := NewParser(tokens).Parse()
    if err != nil {
        // Handle ParserError
    }

    latex := NewLaTeXGenerator().Generate(ast)

    // Write output
    if *output != "" {
        if err := os.WriteFile(*output, []byte(latex+"\n"), 0644); err != nil {
            return fmt.Errorf("error writing output: %w", err)
        }
        fmt.Fprintf(os.Stderr, "Generated: %s\n", *output)
    } else {
        fmt.Println(latex)
    }

    return nil
}
```

---

## Dependency Graph

```
cli.py
├── errors.py (ErrorFormatter)
├── latex_gen.py (LaTeXGenerator)
│   └── ast_nodes.py (Expr, Number, BinaryOp)
├── lexer.py (Lexer, LexerError)
│   └── tokens.py (Token, TokenType)
└── parser.py (Parser, ParserError)
    ├── ast_nodes.py (Expr, Number, BinaryOp)
    └── tokens.py (Token, TokenType)

Core dependencies:
- tokens.py: No internal dependencies (foundational)
- ast_nodes.py: No internal dependencies (foundational)
- errors.py: No internal dependencies (foundational)
- lexer.py: Depends on tokens.py
- parser.py: Depends on tokens.py, ast_nodes.py
- latex_gen.py: Depends on ast_nodes.py
- cli.py: Depends on all other modules
```

**Migration Order (respecting dependencies):**
1. `tokens.py` → `token.go` (no dependencies)
2. `ast_nodes.py` → `ast.go` (no dependencies)
3. `errors.py` → `errors.go` (no dependencies)
4. `lexer.py` → `lexer.go` (depends on token.go)
5. `parser.py` → `parser.go` (depends on token.go, ast.go)
6. `latex_gen.py` → `latex.go` (depends on ast.go)
7. `cli.py` → `cmd/rpn2tex/main.go` (depends on all modules)

---

## Type Mappings

### Basic Types

| Python | Go | Notes |
|--------|----|----|
| `str` | `string` | Immutable in both |
| `int` | `int` | May need `int` or `int64` |
| `list[T]` | `[]T` | Slices |
| `dict[K, V]` | `map[K]V` | Maps |
| `tuple` | Struct or named tuple | No native tuple |
| `bool` | `bool` | Same semantics |
| `Enum` | `const` with iota | Use const blocks |
| `@dataclass(frozen=True)` | Struct with no setters | Enforce immutability |

### Exception Handling

| Python | Go | Pattern |
|--------|----|----|
| `try/except Exception` | `if err != nil` | Error interface |
| `raise CustomError(msg)` | `return fmt.Errorf("%w", err)` | Error wrapping |
| `exception.field` | `err.(*CustomError).field` | Type assertion |

### Method Declarations

| Python | Go | Pattern |
|--------|----|----|
| `def method(self):` | `func (r *Receiver) Method()` | Pointer receiver for mutability |
| `def method(self):` (no mutation) | `func (r Receiver) Method()` | Value receiver for read-only |
| `@staticmethod` | Package function | No receiver |
| `@classmethod` | Constructor function | New* pattern |
| `@property` | Get* method | Getter method |
| `@singledispatchmethod` | Type switch | Manual dispatching |

---

## Python Patterns and Go Equivalents

### Pattern 1: Enum with auto()

**Python:**
```python
from enum import Enum, auto

class TokenType(Enum):
    NUMBER = auto()
    PLUS = auto()
    EOF = auto()
```

**Go:**
```go
type TokenType int

const (
    NUMBER TokenType = iota
    PLUS
    EOF
)
```

### Pattern 2: Frozen Dataclass

**Python:**
```python
from dataclasses import dataclass

@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str
    line: int
    column: int
```

**Go:**
```go
type Token struct {
    Type   TokenType
    Value  string
    Line   int
    Column int
}

// No setter methods - immutability enforced through design
func (t Token) String() string {
    return fmt.Sprintf("Token(%s, %q, %d:%d)", t.Type.String(), t.Value, t.Line, t.Column)
}
```

### Pattern 3: Custom Exception with Fields

**Python:**
```python
class LexerError(Exception):
    message: str
    line: int
    column: int

    def __init__(self, message: str, line: int, column: int):
        super().__init__(f"Line {line}, column {column}: {message}")
        self.message = message
        self.line = line
        self.column = column
```

**Go:**
```go
type LexerError struct {
    Message string
    Line    int
    Column  int
}

func (e *LexerError) Error() string {
    return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
}
```

### Pattern 4: Visitor Pattern with singledispatchmethod

**Python:**
```python
from functools import singledispatchmethod

class Visitor:
    @singledispatchmethod
    def visit(self, node: Expr) -> str:
        raise NotImplementedError(f"No visitor for {type(node).__name__}")

    @visit.register
    def _visit_number(self, node: Number) -> str:
        return str(node.value)

    @visit.register
    def _visit_binary_op(self, node: BinaryOp) -> str:
        return f"({self.visit(node.left)} {node.operator} {self.visit(node.right)})"
```

**Go:**
```go
func (v *Visitor) Visit(node Expr) string {
    switch n := node.(type) {
    case *Number:
        return strconv.Itoa(n.Value)
    case *BinaryOp:
        left := v.Visit(n.Left)
        right := v.Visit(n.Right)
        return fmt.Sprintf("(%s %s %s)", left, n.Operator, right)
    default:
        panic(fmt.Sprintf("No visitor for %T", node))
    }
}
```

### Pattern 5: Stack Operations

**Python:**
```python
stack: list[Expr] = []
stack.append(item)
item = stack.pop()
```

**Go:**
```go
stack := []Expr{}
stack = append(stack, item)
item = stack[len(stack)-1]
stack = stack[:len(stack)-1]
```

### Pattern 6: String Formatting with Position

**Python:**
```python
def _get_context(self, line: int, column: int, context_lines: int) -> str:
    error_idx = line - 1
    start_idx = max(0, error_idx - context_lines)
    end_idx = min(len(self.lines), error_idx + context_lines + 1)
    max_line_num = end_idx
    num_width = len(str(max_line_num))

    result_lines: list[str] = []
    for idx in range(start_idx, end_idx):
        line_num = idx + 1
        line_content = self.lines[idx] if idx < len(self.lines) else ""
        prefix = f"{line_num:>{num_width}} | "
        result_lines.append(f"{prefix}{line_content}")

        if idx == error_idx:
            caret_prefix = " " * num_width + " | "
            caret_pos = max(0, column - 1)
            caret_line = caret_prefix + " " * caret_pos + "^"
            result_lines.append(caret_line)

    return "\n".join(result_lines)
```

**Go:**
```go
func (ef *ErrorFormatter) getContext(line, column, contextLines int) string {
    errorIdx := line - 1
    startIdx := max(0, errorIdx-contextLines)
    endIdx := min(len(ef.Lines), errorIdx+contextLines+1)
    maxLineNum := endIdx
    numWidth := len(strconv.Itoa(maxLineNum))

    var resultLines []string
    for idx := startIdx; idx < endIdx; idx++ {
        lineNum := idx + 1
        lineContent := ""
        if idx < len(ef.Lines) {
            lineContent = ef.Lines[idx]
        }
        prefix := fmt.Sprintf("%*s | ", numWidth, strconv.Itoa(lineNum))
        resultLines = append(resultLines, prefix+lineContent)

        if idx == errorIdx {
            caretPrefix := strings.Repeat(" ", numWidth) + " | "
            caretPos := max(0, column-1)
            caretLine := caretPrefix + strings.Repeat(" ", caretPos) + "^"
            resultLines = append(resultLines, caretLine)
        }
    }

    return strings.Join(resultLines, "\n")
}
```

### Pattern 7: Token Type Mapping

**Python:**
```python
op_map = {
    TokenType.PLUS: "+",
    TokenType.MINUS: "-",
    TokenType.MULT: "*",
    TokenType.DIV: "/",
}
operator = op_map[token.type]
```

**Go:**
```go
var opMap = map[TokenType]string{
    PLUS:  "+",
    MINUS: "-",
    MULT:  "*",
    DIV:   "/",
}
operator := opMap[token.Type]
```

---

## Implementation Order

Follow this order to respect dependencies and enable testing:

### Phase 1: Foundation Types
1. `token.go` (tokens.py)
   - TokenType constants
   - Token struct
   - Helper methods

2. `ast.go` (ast_nodes.py)
   - Expr interface
   - Number struct
   - BinaryOp struct
   - Type assertion helpers

3. `errors.go` (errors.py)
   - ErrorFormatter struct
   - Error formatting methods

### Phase 2: Lexer and Parser
4. `lexer.go` (lexer.py)
   - LexerError type
   - Lexer struct
   - Tokenization logic
   - Character scanning helpers

5. `parser.go` (parser.py)
   - ParserError type
   - Parser struct
   - Stack-based parsing
   - AST construction

### Phase 3: Code Generation
6. `latex.go` (latex_gen.py)
   - LaTeXGenerator struct
   - Visitor pattern implementation
   - Precedence handling
   - LaTeX string generation

### Phase 4: CLI and Integration
7. `cmd/rpn2tex/main.go` (cli.py)
   - Argument parsing
   - File I/O
   - Pipeline orchestration
   - Error handling

### Testing Strategy
- Unit tests for each module after implementation
- Integration tests combining modules
- I/O contract validation tests
- CLI end-to-end tests

---

## I/O Contract

This section contains the exact I/O contract that all Go implementations must satisfy. The test cases below define the complete behavioral specification.

### Test Cases Summary

**Total Test Cases:** 21
- **Successful Cases:** 18 (exit code 0)
- **Error Cases:** 3 (exit code 1)

### Successful Cases

All of these must produce the exact LaTeX output shown and exit with code 0:

| # | Input | Expected Output |
|---|-------|-----------------|
| 1 | `5 3 +` | `$5 + 3$` |
| 2 | `5 3 -` | `$5 - 3$` |
| 3 | `4 7 *` | `$4 \times 7$` |
| 4 | `10 2 /` | `$10 \div 2$` |
| 5 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` |
| 6 | `5 3 * 2 +` | `$5 \times 3 + 2$` |
| 7 | `10 2 / 5 *` | `$10 \div 2 \times 5$` |
| 8 | `5 3 - 2 -` | `$5 - 3 - 2$` |
| 9 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` |
| 10 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` |
| 11 | `2 3 4 * +` | `$2 + 3 \times 4$` |
| 12 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` |
| 13 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` |
| 14 | `2 3 * 4 +` | `$2 \times 3 + 4$` |
| 15 | `3.14 2 *` | `$3.14 \times 2$` |
| 16 | `1.5 0.5 +` | `$1.5 + 0.5$` |
| 17 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` |
| 18 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` |

### Error Cases

All of these must produce error messages containing the shown text and exit with code 1:

| # | Input | Error Message Must Contain | Notes |
|---|-------|----------------------------|-------|
| 1 | `2 3 ^` | `Unexpected character '^'` | Exponentiation not supported |
| 2 | `2 3 ^ 4 *` | `Unexpected character '^'` | Exponentiation in expression |
| 3 | `2 3 4 ^ ^` | `Unexpected character '^'` | Multiple exponentiation operators |

### LaTeX Formatting Rules

1. **Math Mode Delimiters:** Output wrapped in `$...$`
2. **Operator Symbols:**
   - Addition: `+` (single character)
   - Subtraction: `-` (single character)
   - Multiplication: `\times` (literal backslash-times)
   - Division: `\div` (literal backslash-div)
3. **Spacing:** Exactly one space around operators
   - Format: `operand operator operand` with single space before and after operator
   - Example: `$5 + 3$` not `$5+3$` or `$5  +  3$`
4. **Parentheses for Precedence:**
   - Format: `( expr ) operator operand` or `operand operator ( expr )`
   - Note: spaces inside parentheses: `( ` and ` )`
   - Added when child precedence is lower than parent
   - Added on right operand when child precedence equals parent for `-` and `/`

### Operator Precedence and Associativity

**Precedence Levels:**
- Level 1 (lowest): `+`, `-` (addition/subtraction)
- Level 2 (highest): `*`, `/` (multiplication/division)

**Associativity:** All operators are left-associative

**Examples:**
- `5 + 3 - 2` → left-to-right: `(5 + 3) - 2` → output: `$5 + 3 - 2$` (no parens needed)
- `5 - 3 - 2` → left-to-right: `(5 - 3) - 2` → output: `$5 - 3 - 2$` (no parens needed)
- `5 - (3 - 2)` → RPN: `5 3 2 - -` → output: `$5 - ( 3 - 2 )$` (parens on right side)
- `10 / 2 / 5` → left-to-right: `(10 / 2) / 5` → output: `$10 \div 2 \div 5$` (no parens needed)
- `10 / (2 / 5)` → RPN: `10 2 5 / /` → output: `$10 \div ( 2 \div 5 )$` (parens on right side)

### Key Implementation Requirements

1. **Exact Output Matching:** Character-for-character identical output
2. **Floating Point Preservation:** Numbers like `3.14` and `1.5` must be output exactly as-is
3. **Exit Codes:**
   - 0 for successful parsing and generation
   - 1 for any error (lexer, parser, file I/O)
4. **Error Messages:** Must contain the exact phrase `Unexpected character '^'` for invalid characters
5. **No Extra Output:** Only the LaTeX expression to stdout, no additional messages
6. **Whitespace Handling:**
   - Input: Whitespace is delimiter, consecutive whitespace is treated as single delimiter
   - Output: Exact formatting as specified (single space around operators, space inside parens)

### Validation Test Template

```
Test: Input -> Expected Output
- Run: rpn2tex <<< "<input>"
- Expected exit code: <code>
- Expected stdout: "<expected>"
- Or expected stderr contains: "<error_msg>"
```

---

## Validation and Testing

### Unit Test Structure

Each module should have comprehensive unit tests:

**test_token.go:**
- TokenType constants correctly defined
- Token struct creation and String() method
- Token field access

**test_ast.go:**
- Number struct creation
- BinaryOp struct creation
- Expr interface implementation

**test_errors.go:**
- ErrorFormatter initialization
- FormatError with single-line input
- FormatError with multi-line input
- Caret positioning
- Context lines parameter

**test_lexer.go:**
- Simple tokens (numbers, operators)
- Multiple tokens in sequence
- Whitespace handling
- Floating-point numbers
- Negative numbers
- Invalid characters
- EOF token
- LexerError fields

**test_parser.go:**
- Single number
- Binary operations
- Chained operations
- Stack-based evaluation
- Error cases (not enough operands, extra operands)
- Empty expression

**test_latex.go:**
- Number nodes to LaTeX
- Binary operations to LaTeX
- Precedence handling
- Left-associativity
- Parenthesization
- All operator symbols

**test_cli.go:**
- Stdin input
- File input
- File output
- Stdout output
- File I/O errors
- Pipeline integration
- Exit codes

### I/O Contract Validation Tests

Create specific tests for each I/O contract case:

**io_contract_test.go:**
```go
func TestIOContract_Addition(t *testing.T) {
    // Test: 5 3 + -> $5 + 3$
}

func TestIOContract_Multiplication(t *testing.T) {
    // Test: 4 7 * -> $4 \times 7$
}

func TestIOContract_Precedence(t *testing.T) {
    // Test: 5 3 + 2 * -> $( 5 + 3 ) \times 2$
}

func TestIOContract_FloatingPoint(t *testing.T) {
    // Test: 3.14 2 * -> $3.14 \times 2$
}

func TestIOContract_ErrorInvalidChar(t *testing.T) {
    // Test: 2 3 ^ -> Error: Unexpected character '^'
}
```

### Integration Test Structure

**integration_test.go:**
- End-to-end pipeline tests
- CLI invocation tests
- File I/O tests
- Error propagation tests

### Test Coverage Goals

- **Lexer:** 95%+ coverage (all token types, all error cases)
- **Parser:** 95%+ coverage (all operations, all error cases)
- **LaTeX Generator:** 95%+ coverage (all node types, all precedence cases)
- **ErrorFormatter:** 90%+ coverage (various input sizes and error positions)
- **CLI:** 80%+ coverage (main paths, not all file permission combinations)

### Running Tests

```bash
# Unit tests
go test ./...

# With coverage
go test -cover ./...

# I/O contract validation
go test -run IOContract ./...

# Integration tests
go test -run Integration ./...
```

---

## Summary of Critical Implementation Points

1. **Immutability:** Token and AST nodes are immutable - enforce through design (no setters)
2. **Position Tracking:** Line and column (1-based) must be tracked throughout lexing
3. **Stack-Based Parsing:** Parser uses a stack to accumulate operands and build AST
4. **Precedence Rules:** Parentheses must be added correctly based on operator precedence
5. **Left-Associativity:** For `-` and `/`, equal precedence on right side needs parentheses
6. **Exact Output Format:** LaTeX output must match character-for-character (spacing matters!)
7. **Error Handling:** Errors must include position information for context
8. **Operator Symbols:** Multiplication and division use LaTeX escape sequences (`\times`, `\div`)
9. **Floating Point:** Numbers must be preserved exactly as in input
10. **Exit Codes:** 0 for success, 1 for any error

---

## Go-Specific Recommendations

### Package Structure
```
rpn2tex/
├── token.go          # Token types and Token struct
├── ast.go            # AST node definitions
├── errors.go         # Error formatting
├── lexer.go          # Tokenization
├── parser.go         # Parsing
├── latex.go          # LaTeX generation
├── cmd/rpn2tex/
│   └── main.go       # CLI entry point
├── token_test.go
├── ast_test.go
└── ... (other tests)
```

### Coding Standards

1. **Error Handling:** Use `fmt.Errorf` with `%w` verb for error wrapping
2. **Method Receivers:** Use pointer receivers for methods that don't mutate (Go convention)
3. **Naming:** Exported names (PascalCase), unexported names (camelCase)
4. **Constants:** Use const blocks for grouped constants
5. **Interfaces:** Define narrow interfaces where needed (e.g., Expr)
6. **Comments:** Document all exported types and functions
7. **Tests:** Follow `*_test.go` file naming convention

### Dependencies to Avoid

- Avoid external dependencies for core functionality
- Use only stdlib: `fmt`, `os`, `io`, `flag`, `strings`, `unicode`, `path/filepath`
- For testing: use stdlib `testing` package

### Key stdlib Packages

| Task | Package | Functions |
|------|---------|-----------|
| Output | `fmt` | `Printf`, `Fprintf`, `Sprintf` |
| File I/O | `os` | `Open`, `Create`, `ReadFile`, `WriteFile` |
| Strings | `strings` | `Split`, `Join`, `Repeat`, `Contains` |
| Conversions | `strconv` | `Itoa`, `FormatInt` |
| Unicode | `unicode` | `IsDigit`, `IsSpace` |

---

## Document Version and History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-12-29 | Initial specification based on Python source analysis |

---

**End of Migration Specification**

This document provides the complete technical specification for migrating rpn2tex from Python to Go. Use this as the definitive guide for all implementation decisions and validations.
