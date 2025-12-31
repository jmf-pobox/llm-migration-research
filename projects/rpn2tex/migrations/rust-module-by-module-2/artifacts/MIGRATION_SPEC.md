# rpn2tex Python-to-Rust Migration Specification

**Document Version:** 1.0
**Date:** 2025-12-29
**Status:** Ready for Migration

---

## Table of Contents

1. [Overview](#overview)
2. [Module Analysis](#module-analysis)
3. [Dependency Graph](#dependency-graph)
4. [I/O Contract](#io-contract)
5. [Rust-Specific Considerations](#rust-specific-considerations)
6. [Migration Order](#migration-order)
7. [Testing Strategy](#testing-strategy)

---

## Overview

The rpn2tex project is a RPN (Reverse Polish Notation) to LaTeX converter that processes mathematical expressions in postfix notation and outputs them in standard infix notation with proper LaTeX formatting.

**Architecture Flow:**
```
Input Text → Lexer → Tokens → Parser → AST → LaTeX Generator → Output
```

The codebase consists of 7 Python modules designed for educational clarity and separation of concerns. The migration must preserve all behavioral characteristics while adapting to Rust idioms and performance characteristics.

**Key Statistics:**
- Total modules: 7
- Lines of code: ~600
- Primary data structures: Enums, immutable dataclasses, unions
- Core algorithms: Stack-based RPN parsing, visitor pattern AST traversal

---

## Module Analysis

### Module 1: tokens.py

**File:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/tokens.py`

**Purpose:** Defines token types and token data structure for lexical analysis.

**Public API:**

**Enums:**
- `TokenType` - Enum with variants:
  - `NUMBER` - Numeric literals (integers and decimals)
  - `PLUS` - Addition operator (+)
  - `MINUS` - Subtraction operator (-)
  - `MULT` - Multiplication operator (*)
  - `DIV` - Division operator (/)
  - `EOF` - End of file marker

**Dataclasses:**
- `Token(frozen=True)` - Immutable token representation
  - `type: TokenType` - Token type
  - `value: str` - String value of token
  - `line: int` - 1-based line number
  - `column: int` - 1-based column number
  - `__repr__(self) -> str` - Debug representation

**Dependencies:**
- Internal: None
- External: `dataclasses`, `enum`, `__future__` annotations

**Rust Migration Notes:**

**Type Mappings:**
- `TokenType` Enum → Rust `enum TokenType` (C-style enum)
- `Token` dataclass → Rust `#[derive(Clone, Copy, Debug)]` struct (frozen/immutable)
- `str` → `String` for owned values or `&str` for references depending on context
- `int` → `u32` (line/column are always positive, use unsigned for safety)

**Pattern Changes:**
- Python's `@dataclass(frozen=True)` → Rust struct with `#[derive(Clone, Copy, Debug, PartialEq, Eq)]`
- Python's enum with `auto()` → Rust enum (no auto-numbering needed)
- Python's `__repr__` method → Rust `impl Display` or `impl Debug`

**Special Handling:**
- Token immutability is automatic in Rust structs (by default, unless marked mutable)
- Line/column should use `u32` rather than `usize` to match Python's 1-based indexing philosophy
- Consider whether `value: String` should be `value: Box<str>` for memory efficiency in a tree structure
- The frozen/immutable nature of Token is preserved by Rust's ownership model - no additional attributes needed

**Key Implementation Details:**

The Token struct represents a single lexical unit with position information. Position tracking (line, column) is crucial for error reporting. Both line and column are 1-based (matching compiler conventions like gcc/rustc).

The TokenType enum defines all possible token categories. In the base implementation, only basic operators (+, -, *, /) and numbers are supported. The comments reference exercise tokens (CARET, SQRT, ROOT) that are not yet implemented.

---

### Module 2: ast_nodes.py

**File:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/ast_nodes.py`

**Purpose:** Defines Abstract Syntax Tree node types for representing parsed expressions.

**Public API:**

**Dataclasses:**
- `ASTNode(frozen=True)` - Base class for all AST nodes
  - `line: int` - 1-based line number
  - `column: int` - 1-based column number

- `Number(ASTNode, frozen=True)` - Numeric literal
  - `value: str` - String representation of number
  - Inherits: `line`, `column`

- `BinaryOp(ASTNode, frozen=True)` - Binary operation
  - `operator: str` - Operator symbol ("+", "-", "*", "/")
  - `left: Expr` - Left operand (recursive)
  - `right: Expr` - Right operand (recursive)
  - Inherits: `line`, `column`

**Type Aliases:**
- `Expr = Number | BinaryOp` - Union of all expression types (Python 3.10+ syntax)

**Dependencies:**
- Internal: None
- External: `dataclasses`, `__future__` annotations

**Rust Migration Notes:**

**Type Mappings:**
- `ASTNode` dataclass base class → Rust trait (or embedded struct)
- `Number` and `BinaryOp` → Rust enum variants or separate structs
- `Expr` union type → Rust enum `Expr` with variants for `Number` and `BinaryOp`
- `str` value fields → `String` (owned) or `Cow<str>` (for efficiency)
- Inheritance pattern → Enum with embedded position data or trait objects

**Pattern Changes:**
- Python's `@dataclass(frozen=True)` with inheritance → Rust enum with position data in each variant
- Python's union type `Number | BinaryOp` → Rust enum `Expr { Number(...), BinaryOp(...) }`
- The base class approach (ASTNode) is less idiomatic in Rust - prefer an enum or trait pattern

**Special Handling:**

In Python, inheritance provides a shared `line` and `column` across all node types. In Rust, the most idiomatic approach is:

Option 1 (Recommended): Use an enum with position data in each variant
```rust
pub enum Expr {
    Number { line: u32, column: u32, value: String },
    BinaryOp { line: u32, column: u32, operator: String, left: Box<Expr>, right: Box<Expr> },
}
```

Option 2: Use a wrapper struct with Box<T>
```rust
pub struct ASTNode {
    pub line: u32,
    pub column: u32,
    pub expr: ExprKind,
}

pub enum ExprKind {
    Number(String),
    BinaryOp { operator: String, left: Box<Expr>, right: Box<Expr> },
}
```

Option 1 is preferred for better cache locality and simpler pattern matching.

**Recursive Types:** Both options require `Box<Expr>` for left/right operands to avoid infinite size issues.

**Key Implementation Details:**

AST nodes represent the structure of parsed RPN expressions. The recursive structure of BinaryOp enables representation of arbitrarily complex expressions. Position information (line, column) is preserved at each node level to enable precise error reporting.

Numbers are stored as strings (not parsed to f64 or i64) to preserve exact representation for LaTeX output (e.g., "3.14" remains "3.14", not "3.1400000000000001").

The Expr type alias serves as the primary type for all expression nodes and is used throughout the parser and generator modules.

---

### Module 3: errors.py

**File:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/errors.py`

**Purpose:** Provides error formatting with source context for gcc/rustc-style error messages.

**Public API:**

**Classes:**
- `ErrorFormatter` - Formats parse errors with source context
  - `source: str` - Complete source text being parsed
  - `lines: list[str]` - Source split into lines
  - `__init__(source: str) -> None` - Constructor
  - `format_error(message: str, line: int, column: int, *, context_lines: int = 1) -> str` - Format error with context
  - `_get_context(line: int, column: int, context_lines: int) -> str` - Extract context around error (private)

**Dependencies:**
- Internal: None
- External: None (pure Python)

**Rust Migration Notes:**

**Type Mappings:**
- `str` → `String` or `&str` depending on ownership
- `list[str]` → `Vec<String>` or `Vec<&str>`
- Method self parameter → Rust reference `&self`

**Pattern Changes:**
- Python's instance variables initialized in `__init__` → Rust struct fields
- String formatting with f-strings → Rust format! macros or str concatenation
- Line/column iteration → Rust range iteration with enumerate

**Special Handling:**
- Source ownership: ErrorFormatter should take `String` or `&str`
  - If `String`: ErrorFormatter owns source and must split into lines (Vec<String>)
  - If `&str`: Lines must use `str::lines()` iterator or cache split lines
- String concatenation and line number padding requires attention to alignment
- The `_get_context` method builds formatted output line by line - use `String` buffer and `push_str()` or collect lines into Vec

**Key Implementation Details:**

The ErrorFormatter produces user-friendly error messages with visual context, similar to compiler error output. Key responsibilities:

1. **Source splitting:** Cache the source text split into lines for efficient access
2. **Context extraction:** Show the error line plus configurable context lines before/after
3. **Caret positioning:** Place a `^` character at the exact column of the error
4. **Line numbering:** Format line numbers with padding for alignment

The algorithm:
1. Convert line/column from 1-based (user-friendly) to 0-based indexing
2. Clamp context range to valid indices
3. Calculate line number width for alignment
4. Build output with formatted lines and caret pointer

---

### Module 4: lexer.py

**File:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/lexer.py`

**Purpose:** Tokenizes RPN input text into a stream of tokens.

**Public API:**

**Exceptions:**
- `LexerError(Exception)` - Raised on invalid input
  - `message: str` - Error description
  - `line: int` - Error line (1-based)
  - `column: int` - Error column (1-based)
  - `__init__(message: str, line: int, column: int) -> None`

**Classes:**
- `Lexer` - Tokenizes RPN input
  - `text: str` - Input text
  - `pos: int` - Current position (0-based)
  - `line: int` - Current line (1-based)
  - `column: int` - Current column (1-based)
  - `__init__(text: str) -> None` - Constructor
  - `tokenize(self) -> list[Token]` - Tokenize entire input (public)
  - `_at_end(self) -> bool` - Check end of input (private)
  - `_peek(self) -> str` - Look at current char (private)
  - `_advance(self) -> str` - Consume char and return it (private)
  - `_skip_whitespace(self) -> None` - Skip whitespace (private)
  - `_scan_token(self) -> Token` - Scan next token (private)
  - `_scan_number(self, prefix: str, start_line: int, start_column: int) -> Token` - Scan number literal (private)

**Dependencies:**
- Internal: `tokens` (imports Token, TokenType)
- External: None

**Rust Migration Notes:**

**Type Mappings:**
- `str` → `&str` for input (borrowed reference to avoid copying)
- `list[Token]` → `Vec<Token>`
- Exception class → Rust custom error type with `thiserror` crate or enum
- Instance variables → Struct fields
- Method return types straightforward

**Pattern Changes:**
- Python exceptions → Rust `Result<T, LexerError>` or custom error enum
- Character methods (`isdigit()`, `in "..."`) → Rust char methods and patterns
- String concatenation for building token values → `String::new()` and `push()/push_str()`

**Special Handling:**

Error handling is a key difference:
- Python raises `LexerError` exception
- Rust should return `Result<Vec<Token>, LexerError>`
- Error type should derive/implement `std::error::Error` and `Display`
- Consider using `thiserror` crate for clean error implementation

Character classification:
- Python: `char.isdigit()` → Rust: `char.is_ascii_digit()` (for ASCII digits)
- Python: `char in " \t\n\r"` → Rust: `matches!(char, ' ' | '\t' | '\n' | '\r')`

String building:
- Token values are built character by character
- Use `String::new()` and `push()/push_str()` for efficiency
- Alternatively use an iterator-based approach

**Key Implementation Details:**

The Lexer performs character-by-character scanning of RPN input, producing a token stream. Key aspects:

**Scanning algorithm:**
1. Skip whitespace to find next token
2. Identify token type by first character:
   - Operators: +, -, *, / (single character)
   - Numbers: digits or - followed by digits
3. For numbers: scan integer part, then optional decimal part
4. Special case: minus sign can be subtraction operator OR negative number prefix
5. Unknown characters raise LexerError

**Position tracking:**
- Line and column track current position for error reporting
- Newlines increment line counter and reset column to 1
- Other characters increment column counter
- Positions are 1-based (line 1, column 1 for start)

**Token stream:**
- All valid tokens preceded by optional whitespace
- Stream ends with EOF token (empty value, final position)

**Negative numbers:**
- Minus sign followed immediately (no whitespace) by digit is a negative number
- Otherwise minus sign is subtraction operator
- This allows expressions like "0 5 -" or "-5 3 +" to work correctly

---

### Module 5: parser.py

**File:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/parser.py`

**Purpose:** Converts token stream into an Abstract Syntax Tree using stack-based RPN parsing.

**Public API:**

**Exceptions:**
- `ParserError(Exception)` - Raised on invalid RPN
  - `message: str` - Error description
  - `token: Token` - Token where error occurred
  - `__init__(message: str, token: Token) -> None`

**Classes:**
- `Parser` - RPN parser
  - `tokens: list[Token]` - Token stream from lexer
  - `pos: int` - Current position in token list (0-based)
  - `__init__(tokens: list[Token]) -> None` - Constructor
  - `parse(self) -> Expr` - Parse tokens into AST (public)
  - `_current(self) -> Token` - Get current token (private)
  - `_at_end(self) -> bool` - Check if at EOF (private)
  - `_advance(self) -> Token` - Consume token and advance (private)

**Dependencies:**
- Internal: `ast_nodes` (imports BinaryOp, Expr, Number), `tokens` (imports Token, TokenType)
- External: None

**Rust Migration Notes:**

**Type Mappings:**
- `list[Token]` → `Vec<Token>` or `&[Token]`
- `list[Expr]` (stack) → `Vec<Expr>`
- Exception type → Custom Rust error type
- `dict` (op_map) → Rust match expression or HashMap

**Pattern Changes:**
- Stack-based algorithm translates directly from Python to Rust
- Exception handling → Result types
- Dictionary lookup → match expressions (preferred for small, compile-time known sets)
- Instance mutation → straightforward in Rust

**Special Handling:**

The stack-based parsing is very natural in Rust:

```rust
let mut stack: Vec<Expr> = Vec::new();
// ... push to stack
let right = stack.pop().unwrap_or_else(|| { /* error */ });
let left = stack.pop().unwrap_or_else(|| { /* error */ });
```

Op mapping - use match instead of dictionary:
```rust
let operator = match token.type {
    TokenType::Plus => "+",
    TokenType::Minus => "-",
    TokenType::Mult => "*",
    TokenType::Div => "/",
    _ => unreachable!(),
};
```

**Key Implementation Details:**

Parser implements the classic RPN (postfix) parsing algorithm:

**Algorithm:**
1. Initialize empty stack
2. Scan tokens left to right:
   - If NUMBER: push Number node onto stack
   - If OPERATOR:
     - Verify stack has ≥2 elements (otherwise error)
     - Pop right operand
     - Pop left operand
     - Create BinaryOp node with both operands
     - Push BinaryOp back onto stack
   - If EOF: break
3. Validate final state:
   - Stack must have exactly 1 element (the root expression)
   - Empty stack = empty expression (error)
   - >1 element = missing operators (error)
4. Return stack[0] as the AST root

**Error cases:**
- Too few operands for operator (need 2, have <2)
- Empty expression (no tokens)
- Too many values on stack (missing operators)

**Stack validation** is crucial - must happen before attempting pop operations to give good error messages.

---

### Module 6: latex_gen.py

**File:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/latex_gen.py`

**Purpose:** Converts AST to LaTeX source code with proper operator precedence and parenthesization.

**Public API:**

**Classes:**
- `LaTeXGenerator` - AST to LaTeX converter
  - `BINARY_OPS: ClassVar[dict[str, str]]` - Operator to LaTeX mapping
    - Maps: "+", "-", "*", "/"
    - LaTeX: "+", "-", r"\times", r"\div"
  - `PRECEDENCE: ClassVar[dict[str, int]]` - Operator precedence
    - Addition/subtraction: level 1
    - Multiplication/division: level 2
  - `generate(self, ast: Expr) -> str` - Generate LaTeX (public)
  - `_visit(self, node: Expr) -> str` - Visit dispatcher (private, singledispatchmethod)
  - `_visit_number(self, node: Number) -> str` - Visit Number node
  - `_visit_binary_op(self, node: BinaryOp) -> str` - Visit BinaryOp node
  - `_needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool` - Determine if parens needed

**Dependencies:**
- Internal: `ast_nodes` (imports BinaryOp, Expr, Number)
- External: `functools` (singledispatchmethod), `typing` (ClassVar)

**Rust Migration Notes:**

**Type Mappings:**
- `ClassVar[dict[str, str]]` → Rust `const` with HashMap or match statement
- `@singledispatchmethod` pattern → Rust match expression or trait objects
- `str` (operator) → Could use enum instead of string for type safety

**Pattern Changes:**
- Python's singledispatchmethod → Rust pattern matching on Expr enum
- Class variables → Module-level constants
- Dictionary lookups → match expressions or static HashMaps

**Special Handling:**

The singledispatchmethod pattern is Python-specific. In Rust, use pattern matching:

```rust
fn visit(&self, node: &Expr) -> String {
    match node {
        Expr::Number { value, .. } => value.clone(),
        Expr::BinaryOp { operator, left, right, .. } => {
            // Handle binary op
        }
    }
}
```

Operator strings could be replaced with an enum for type safety:

```rust
pub enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
}
```

However, keeping strings may be acceptable if the operator field comes from the parser.

**Key Implementation Details:**

The LaTeX generator performs tree traversal with operator precedence-based parenthesization.

**Precedence levels:**
- Level 1: Addition (+) and subtraction (-)
- Level 2: Multiplication (*) and division (/)
- Higher precedence = tighter binding

**Parenthesization rules:**
1. Wrap operand in parens if its operator has LOWER precedence than parent
2. Wrap RIGHT operand in parens if equal precedence AND operator is left-associative (- or /)
3. LEFT operand with equal precedence never needs parens
4. Non-BinaryOp children (Numbers) never need parens

**Algorithm:**
```
visit(node):
  if Number: return value
  if BinaryOp:
    left_text = visit(left)
    if needs_parens(left, my_precedence, is_right=false):
      left_text = "( " + left_text + " )"
    right_text = visit(right)
    if needs_parens(right, my_precedence, is_right=true):
      right_text = "( " + right_text + " )"
    return left_text + " " + op_latex + " " + right_text
```

**LaTeX mappings:**
- "*" → r"\times" (raw string, literal backslash)
- "/" → r"\div" (raw string, literal backslash)
- "+" → "+" (simple plus)
- "-" → "-" (simple minus)

**Output format:**
- All output wrapped in "$...$" (LaTeX inline math mode delimiters)
- Spaces around operators for readability
- Spaces inside parentheses: "( ... )" not "(...)"

---

### Module 7: cli.py

**File:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/cli.py`

**Purpose:** Command-line interface that orchestrates the entire conversion pipeline.

**Public API:**

**Functions:**
- `main() -> int` - CLI entry point
  - Returns: 0 for success, 1 for error
  - Argument parsing with argparse
  - File I/O error handling
  - Pipeline orchestration: read → tokenize → parse → generate
  - Stderr output for status messages and errors

**Dependencies:**
- Internal: `errors` (ErrorFormatter), `latex_gen` (LaTeXGenerator), `lexer` (Lexer, LexerError), `parser` (Parser, ParserError)
- External: `argparse`, `sys`, `pathlib`

**Rust Migration Notes:**

**Type Mappings:**
- `argparse.ArgumentParser` → `clap` crate (recommended) or manual arg parsing
- `Path` → `std::path::PathBuf`
- `sys.stdin/stdout/stderr` → `std::io` module
- `int` return → Same (Rust main can return u8 or i32)
- File I/O exceptions → Rust Result types

**Pattern Changes:**
- Exception handling → Result types and error propagation
- Argument parsing → Move to separate function or use clap builder pattern
- File operations → `std::fs` module functions
- String output → println!/eprintln! macros

**Special Handling:**

Input handling:
- "-" for stdin vs file paths requires conditional logic
- Stdin reading: `std::io::read_to_string()` or BufReader
- File reading: `std::fs::read_to_string()` (simple) or handle errors explicitly

Error handling flow:
```
Read input:
  - FileNotFoundError → "Input file not found"
  - PermissionError → "Permission denied"
  - IsADirectoryError → "Expected a file, got directory"

Lexing:
  - LexerError → format with ErrorFormatter, stderr, exit 1

Parsing:
  - ParserError → format with ErrorFormatter, stderr, exit 1

Writing output:
  - PermissionError → "Permission denied"
  - IsADirectoryError → "Cannot write to directory"
```

**Key Implementation Details:**

The CLI orchestrates the entire processing pipeline with proper error handling and user feedback.

**Pipeline steps:**
1. **Parse arguments:** Input file path, optional output file path
2. **Read input:** From file or stdin
3. **Tokenize:** Lexer converts text to tokens
4. **Parse:** Parser converts tokens to AST
5. **Generate:** LaTeXGenerator converts AST to LaTeX string
6. **Write output:** To file or stdout

**Error handling strategy:**
- File I/O errors: catch and print to stderr, exit 1
- Lexer errors: format with context, print to stderr, exit 1
- Parser errors: format with context, print to stderr, exit 1
- Success: print LaTeX to stdout (or file), print status to stderr, exit 0

**Status messages:**
- Errors go to stderr
- LaTeX output goes to stdout (unless -o specified)
- Success message "Generated: <path>" goes to stderr

---

## Dependency Graph

```
cli.py
├── errors.py
├── latex_gen.py
│   └── ast_nodes.py
├── lexer.py
│   └── tokens.py
└── parser.py
    ├── ast_nodes.py
    └── tokens.py

Dependency Levels (build order):
1. tokens.py (no dependencies)
2. ast_nodes.py (no dependencies)
3. errors.py (no dependencies)
4. lexer.py (depends on tokens.py)
5. parser.py (depends on ast_nodes.py, tokens.py)
6. latex_gen.py (depends on ast_nodes.py)
7. cli.py (depends on all others)
```

**Recommended Rust module organization:**
```
src/
├── lib.rs (module declarations and public exports)
├── tokens.rs (or tokens/)
├── ast_nodes.rs (or ast/)
├── errors.rs
├── lexer.rs
├── parser.rs
├── latex_gen.rs (or generator/)
└── main.rs (CLI entry point)
```

---

## I/O Contract

This is the complete I/O contract from Phase 0. All Rust implementations MUST produce identical output for all test cases and handle error cases appropriately.

### Overview

This document specifies the exact input-output behavior of the Python reference implementation of rpn2tex. This contract serves as the canonical specification for validating the Rust migration implementation.

**Generated from:** Python rpn2tex source implementation
**Test Date:** 2025-12-29
**Total Test Cases:** 21
**Successful Cases:** 18
**Error Cases:** 3

### Successful Test Cases

All successful test cases must produce identical LaTeX output:

| # | Input | Expected LaTeX Output | Notes |
|---|-------|----------------------|-------|
| 1 | `5 3 +` | `$5 + 3$` | Basic addition |
| 2 | `5 3 -` | `$5 - 3$` | Basic subtraction |
| 3 | `4 7 *` | `$4 \times 7$` | Basic multiplication with proper LaTeX symbol |
| 4 | `10 2 /` | `$10 \div 2$` | Basic division with proper LaTeX symbol |
| 5 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Parentheses added for operator precedence |
| 6 | `5 3 * 2 +` | `$5 \times 3 + 2$` | No parentheses needed (multiplication before addition) |
| 7 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | Left-to-right evaluation for same precedence |
| 8 | `5 3 - 2 -` | `$5 - 3 - 2$` | Subtraction is left-associative |
| 9 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Multiple divisions are left-associative |
| 10 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Multiple additions without unnecessary parentheses |
| 11 | `2 3 4 * +` | `$2 + 3 \times 4$` | Multiplication precedence over addition (no parentheses) |
| 12 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Addition has lower precedence, parentheses added |
| 13 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Right operand addition needs parentheses |
| 14 | `2 3 * 4 +` | `$2 \times 3 + 4$` | No parentheses needed (multiplication before addition) |
| 15 | `3.14 2 *` | `$3.14 \times 2$` | Floating-point numbers supported |
| 16 | `1.5 0.5 +` | `$1.5 + 0.5$` | Floating-point addition |
| 17 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Multiple additions with multiplication |
| 18 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex expression with multiple operators |

### Error Cases

These test cases demonstrate inputs that the Python reference implementation rejects. The Rust implementation should also reject these, though exact error messages may differ:

| Input | Expected Error | Reason |
|-------|----------------|--------|
| `2 3 ^` | `Line 1, column 5: Unexpected character '^'` | Exponentiation operator (^) not implemented |
| `2 3 ^ 4 *` | `Line 1, column 5: Unexpected character '^'` | Exponentiation operator (^) not implemented |
| `2 3 4 ^ ^` | `Line 1, column 7: Unexpected character '^'` | Exponentiation operator (^) not implemented |

### Implementation Notes

**Supported Operators:**
- **Addition (+):** Rendered as `+` in LaTeX
- **Subtraction (-):** Rendered as `-` in LaTeX
- **Multiplication (*):** Rendered as `\times` in LaTeX
- **Division (/):** Rendered as `\div` in LaTeX

**Not Implemented:**
- **Exponentiation (^):** The tokens.py file indicates this is an exercise feature
- **Square root (sqrt):** Not implemented in base version
- **Nth root (root):** Not implemented in base version

**Parenthesization Rules:**
The implementation correctly adds parentheses based on operator precedence:
1. Multiplication and division have higher precedence than addition and subtraction
2. Operators of the same precedence are evaluated left-to-right
3. Parentheses are inserted only when necessary (based on the right operand's precedence)

**Floating-Point Support:**
- Numbers can contain decimal points (e.g., 3.14, 1.5)
- Decimal representation is preserved in the output

**LaTeX Output Format:**
All outputs are wrapped in LaTeX math mode delimiters: `$ ... $`

### For Migration Validation

When implementing the Rust version, ensure:
1. All 18 successful test cases produce identical LaTeX output
2. The 3 error cases should raise appropriate errors (may differ slightly in error messages)
3. Operator symbols match exactly: `\times` for multiplication, `\div` for division
4. Parenthesization logic matches exactly - test cases show where parentheses are or aren't added
5. Floating-point numbers are handled correctly
6. All outputs are wrapped in `$...$` delimiters

---

## Rust-Specific Considerations

### 1. Error Handling Strategy

**Python approach:** Exceptions with try/except
**Rust approach:** Result<T, E> types with ? operator

Create a custom error type that implements Display and Error:

```rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RpnError {
    LexerError { message: String, line: u32, column: u32 },
    ParserError { message: String, line: u32, column: u32 },
    IoError(std::io::Error),
}

impl fmt::Display for RpnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RpnError::LexerError { message, line, column } => {
                write!(f, "Line {}, column {}: {}", line, column, message)
            }
            // ... other variants
        }
    }
}

impl Error for RpnError {}
```

Alternatively, use the `thiserror` crate:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpnError {
    #[error("Line {line}, column {column}: {message}")]
    LexerError { message: String, line: u32, column: u32 },
    #[error("Line {line}, column {column}: {message}")]
    ParserError { message: String, line: u32, column: u32 },
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

### 2. String Handling

**Python:** Single str type for all string purposes
**Rust:** Multiple options based on ownership and lifetime

- **Owned strings:** `String` - Use for values that need to be stored
- **Borrowed strings:** `&str` - Use for references and function parameters
- **Static strings:** `&'static str` - Use for constants and literals

Recommendations:
- Token values: `String` (owned, constructed during tokenization)
- Operator strings: Consider enum instead for type safety
- Error messages: `String` or &str depending on lifetime
- LaTeX output: `String` for constructed output

### 3. Type System and Immutability

**Python:** Frozen dataclasses for immutability
**Rust:** Immutability by default

In Rust, all bindings are immutable by default. No special annotation needed like `@frozen`. However:

- For AST nodes, consider using `Copy` and `Clone` traits for small types
- Recursive types (BinaryOp with left/right children) need `Box<Expr>` for heap allocation
- Consider whether to use enums or trait objects for flexibility

### 4. Pattern Matching

**Python:** Uses `isinstance()` and getattr
**Rust:** Pattern matching with match expressions

```rust
// Python
if isinstance(node, BinaryOp):
    operator = node.operator
    left = node.left

// Rust
match node {
    Expr::BinaryOp { operator, left, right, .. } => {
        // use operator, left, right
    }
    Expr::Number { value, .. } => {
        // use value
    }
}
```

### 5. Visitor Pattern

**Python:** Uses @singledispatchmethod for dynamic dispatch
**Rust:** Two approaches:
  - Pattern matching (simpler, preferred)
  - Trait objects with Box<dyn Trait> (more flexible)

For rpn2tex, pattern matching is simpler and more idiomatic:

```rust
// Instead of singledispatchmethod
impl LaTeXGenerator {
    fn visit(&self, node: &Expr) -> String {
        match node {
            Expr::Number { value, .. } => value.clone(),
            Expr::BinaryOp { operator, left, right, .. } => {
                // Handle binary op
            }
        }
    }
}
```

### 6. Class Variables and Constants

**Python:** ClassVar in class definition
**Rust:** Const or static

```rust
// Operator mapping
const BINARY_OPS: &[(&str, &str)] = &[
    ("+", "+"),
    ("-", "-"),
    ("*", r"\times"),
    ("/", r"\div"),
];

// Or use a function that returns a HashMap
fn get_binary_ops() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("+", "+");
    // ...
    map
}
```

### 7. File I/O

**Python:** Path.read_text(), exceptions
**Rust:** std::fs::read_to_string() or File API

```rust
use std::fs;

// Simple approach
let text = fs::read_to_string(&path)
    .map_err(|e| RpnError::IoError(e))?;

// With explicit error types
match fs::read_to_string(&path) {
    Ok(text) => { /* process */ }
    Err(e) if e.kind() == io::ErrorKind::NotFound => {
        eprintln!("Error: Input file not found: {:?}", path);
    }
    // ... other error cases
}
```

### 8. Line and Column Tracking

**Python:** 1-based indexing (line 1, column 1 for start)
**Rust:** Use u32 for positions to match Python semantics

Maintain 1-based indexing in Rust, not 0-based:
- Start: line = 1, column = 1
- Newline handling: column resets to 1, line increments
- Caret positioning: use (column - 1) to convert to 0-based for spacing

### 9. Command-Line Arguments

**Python:** argparse module
**Rust:** clap crate (recommended) or std::env::args()

Using clap (derive API):

```rust
use clap::Parser;

#[derive(Parser)]
#[command(about = "Convert RPN expressions to LaTeX math mode")]
struct Args {
    #[arg(help = "Input RPN file (use '-' for stdin)")]
    input: String,

    #[arg(short, long, help = "Output LaTeX file (default: stdout)")]
    output: Option<PathBuf>,
}
```

### 10. Main Function Return Type

**Python:** main() -> int
**Rust:** main() can return:
  - `()` (implicit 0 exit code)
  - `Result<(), Box<dyn Error>>` (automatic error propagation)
  - `ExitCode` (type-safe, nightly-only)

For rpn2tex, explicit `-> u32` or u8 with manual return statements is fine, or use Result-based approach.

---

## Migration Order

The recommended order for implementing Rust modules, respecting dependencies:

### Phase 1: Foundation (No dependencies)
1. `tokens.rs` - Token types and TokenType enum
2. `ast_nodes.rs` - AST node definitions
3. `errors.rs` - Error formatting infrastructure

### Phase 2: Core Pipeline
4. `lexer.rs` - Tokenization (depends on tokens.rs)
5. `parser.rs` - Parsing (depends on ast_nodes.rs, tokens.rs)
6. `latex_gen.rs` - Code generation (depends on ast_nodes.rs)

### Phase 3: Integration
7. `main.rs` (or `lib.rs` + `main.rs`) - CLI (depends on all others)

### Build Verification
After each phase:
- Write unit tests for the module
- Verify it compiles and links with previous modules
- Run integration tests if applicable

---

## Testing Strategy

### Unit Tests

Create test modules for each Rust file. Example structure:

```rust
// In lexer.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_addition() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 4); // 5, 3, +, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
    }
}
```

### Integration Tests

Create `tests/` directory with end-to-end tests:

```rust
// tests/integration_test.rs
#[test]
fn test_simple_addition() {
    let input = "5 3 +";
    let output = rpn2tex::convert(input).expect("conversion failed");
    assert_eq!(output, "$5 + 3$");
}
```

### I/O Contract Validation

Implement all 18 successful test cases from the I/O contract as integration tests.

### Error Cases

Test all 3 error cases to ensure they produce appropriate errors (message content may differ).

---

## Key Behavioral Requirements

### Must Preserve

1. **Exact LaTeX output for all 18 test cases** - Character-perfect match
2. **Operator symbols:**
   - Multiplication: `\times` (not `\cdot` or `*`)
   - Division: `\div` (not `/`)
3. **Parenthesization logic:**
   - Spaces inside parens: `( 5 + 3 )` not `(5+3)`
   - Only add when needed based on precedence
   - Left-associative handling for - and /
4. **Floating-point representation:**
   - Store numbers as strings to preserve exact representation
   - "3.14" remains "3.14", not formatted as float
5. **Error messages:**
   - Include line and column numbers (1-based)
   - Format: "Line X, column Y: error message"
6. **Position tracking:**
   - 1-based line and column numbering
   - Newlines properly tracked
   - Error locations must be precise

### May Differ from Python

1. **Error message wording** - Can be more descriptive or idiomatic
2. **Internal representation** - Different structures are fine if behavior is identical
3. **Error type names** - Rust error types can have different names
4. **Source context formatting** - ErrorFormatter output may differ slightly in formatting

### Performance Considerations

While the Python implementation is slow, the Rust version should be fast. However, correctness is paramount. If there's a trade-off between clarity and performance for a particular module, choose clarity and benchmark before optimizing.

---

## Migration Checklist

- [ ] Phase 1: Foundation modules (tokens, ast_nodes, errors)
  - [ ] Exact type mappings verified
  - [ ] Immutability preserved
  - [ ] Position information (line, column) correct
  - [ ] Unit tests written

- [ ] Phase 2: Core pipeline (lexer, parser, latex_gen)
  - [ ] Lexer produces identical token streams
  - [ ] Parser builds identical ASTs
  - [ ] LaTeX generator produces exact output
  - [ ] All 18 I/O contract cases pass
  - [ ] Error handling works for 3 error cases
  - [ ] Integration tests written

- [ ] Phase 3: CLI integration
  - [ ] Argument parsing works
  - [ ] File I/O error handling correct
  - [ ] Pipeline orchestration correct
  - [ ] Exit codes correct (0 for success, 1 for error)
  - [ ] Error output goes to stderr
  - [ ] Success output goes to stdout
  - [ ] End-to-end tests pass

- [ ] Final validation
  - [ ] All 21 I/O contract test cases pass
  - [ ] Code compiles without warnings
  - [ ] Documentation complete
  - [ ] Performance acceptable (should be much faster than Python)

---

## Common Migration Pitfalls to Avoid

1. **String ownership mistakes:**
   - Don't use String when &str is sufficient
   - Remember to clone/copy when needed

2. **Index out of bounds:**
   - Always check lengths before indexing
   - Use get() and pattern matching instead of direct indexing when uncertain

3. **Forgetting Box for recursive types:**
   - BinaryOp with left/right children MUST use Box<Expr> to have fixed size

4. **Precedence of - and / not handled correctly:**
   - Must check is_right flag for equal precedence operators
   - Test cases 8 and 9 specifically validate this

5. **Output format differences:**
   - Spaces matter: "( 5 + 3 )" not "(5+3)" or "( 5+3 )"
   - LaTeX symbols must be exact: r"\times" not r"\cdot"

6. **1-based vs 0-based indexing confusion:**
   - Position data (line, column) is 1-based
   - Internal scanning uses 0-based (pos field)
   - Be explicit about conversions

7. **Enum variant naming:**
   - Keep TokenType variant names the same (NUMBER, PLUS, etc.)
   - This makes test case mapping straightforward

8. **Missing EOF token:**
   - Lexer must always append an EOF token
   - Parser depends on finding EOF to know when to stop

9. **Not validating stack state in parser:**
   - Must check stack size before and after parsing
   - Empty stack or >1 element remaining are errors

10. **Error context formatting:**
    - Line numbers must align correctly
    - Caret must point to exact column (1-based)
    - Context lines must be shown with proper numbers

---

## References and Resources

### Rust Learning
- The Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/

### Relevant Crates
- `thiserror` - Ergonomic error handling
- `clap` - Command-line argument parsing
- `anyhow` - Flexible error handling (alternative to custom types)

### Testing
- Rust Book: Testing chapter
- https://doc.rust-lang.org/book/ch11-00-testing.html

---

**Document Complete**

This specification provides sufficient detail for migrators to implement the Rust version without needing to reference the Python source files. Each module description includes:
- Public API with signatures
- Dependencies (both internal and external)
- Type mappings from Python to Rust
- Pattern changes required for idiomatic Rust
- Special handling notes
- Key algorithmic details

The I/O Contract serves as the behavioral specification that Rust implementations must satisfy exactly for successful test cases, with flexibility on error message wording for error cases.
