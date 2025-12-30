# PHASE 1: Comprehensive Migration Specification for rpn2tex Python to Rust

**Document Version:** 1.0
**Date:** 2025-12-29
**Purpose:** Complete specification for migrating rpn2tex from Python to Rust

---

## Executive Summary

This document provides a comprehensive specification for migrating the rpn2tex codebase from Python to Rust. The migration follows a module-by-module approach with strict dependency ordering to ensure stable incremental builds and testing.

The rpn2tex project is a Reverse Polish Notation (RPN) expression parser that converts RPN mathematical expressions to LaTeX format. The architecture consists of 7 modules organized in a clear pipeline:

1. **Tokens** (core) - Token type definitions
2. **AST Nodes** (core) - Abstract Syntax Tree node definitions
3. **Errors** (core) - Error formatting utilities
4. **Lexer** (pipeline) - Tokenization
5. **Parser** (pipeline) - AST construction
6. **LaTeX Generator** (pipeline) - Output generation
7. **CLI** (interface) - Command-line orchestration

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [I/O Contract Reference](#io-contract-reference)
3. [Per-Module Specifications](#per-module-specifications)
4. [Migration Order & Dependencies](#migration-order--dependencies)
5. [Rust Project Structure](#rust-project-structure)
6. [Quality Gates & Validation](#quality-gates--validation)
7. [Key Migration Challenges](#key-migration-challenges)
8. [Type Mappings](#type-mappings)
9. [Pattern Changes](#pattern-changes)

---

## Architecture Overview

### System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        CLI Module                            │
│  (argparse, file I/O, error formatting, orchestration)      │
└──────────────────┬──────────────────────────────────────────┘
                   │
     ┌─────────────┴──────────────┐
     │                            │
     v                            v
┌──────────────┐          ┌──────────────────┐
│  Lexer       │          │  ErrorFormatter  │
│  (tokenize)  │          │  (format errors) │
└──────┬───────┘          └──────────────────┘
       │
       v
┌─────────────────────────────────┐
│       Parser                      │
│  (stack-based RPN to AST)        │
└────────────┬──────────────────────┘
             │
             v
┌──────────────────────────┐
│  LaTeX Generator          │
│  (visitor pattern, AST    │
│   to LaTeX conversion)    │
└──────────────────────────┘

Core Dependencies (used by all):
    ├── Tokens (TokenType enum, Token struct)
    ├── AST Nodes (ASTNode, Number, BinaryOp, Expr type)
    └── Errors (LexerError, ParserError)
```

### Data Flow Pipeline

```
Raw Text Input
    │
    v
Lexer::tokenize()
    │ (produces list[Token])
    v
Parser::parse()
    │ (produces Expr AST)
    v
LaTeXGenerator::generate()
    │ (produces String)
    v
stdout or File Output
```

### Module Dependency Graph

```
                ┌─────────────┐
                │  Tokens     │ ← Core (no dependencies)
                └──────┬──────┘
                       │
         ┌─────────────┼──────────────┐
         │             │              │
         v             v              v
    ┌────────┐   ┌──────────┐   ┌─────────┐
    │ Lexer  │   │ ASTNodes │   │ Errors  │ ← Core
    └────┬───┘   └────┬─────┘   └────┬────┘
         │            │              │
         └────────────┼──────────────┘
                      │
                      v
                  ┌────────┐
                  │ Parser │ ← Depends on: Tokens, ASTNodes
                  └────┬───┘
                       │
                       v
              ┌──────────────────┐
              │ LaTeX Generator  │ ← Depends on: ASTNodes
              └─────────┬────────┘
                        │
                        v
              ┌──────────────────┐
              │  CLI/Main        │ ← Depends on all
              └──────────────────┘
```

### Key Abstractions

1. **TokenType (Enum)**: Core lexical element categories
   - NUMBER, PLUS, MINUS, MULT, DIV, EOF

2. **Token (Struct)**: Lexical token with position information
   - type, value, line, column

3. **ASTNode (Trait/Base)**: Base for all AST nodes
   - line, column tracking for error reporting

4. **Expr (Type Alias)**: Union of all expression types
   - Number | BinaryOp (extendable for future node types)

5. **BinaryOp (Struct)**: Binary operation in AST
   - operator, left, right operands

6. **Lexer**: Character-by-character scanner
   - Position tracking (line, column)
   - Whitespace handling
   - Token generation

7. **Parser**: Stack-based RPN parser
   - Stack-based accumulation
   - Token consumption
   - AST construction

8. **LaTeXGenerator**: Visitor pattern for AST traversal
   - Operator precedence handling
   - Parenthesization logic
   - LaTeX output generation

---

## I/O Contract Reference

This section includes the complete I/O contract from Phase 0. The Rust implementation MUST produce identical output for all test cases.

### Test Cases (21 Total)

#### Test Case 1: Basic Addition
- **Input:** `5 3 +`
- **Expected Output:** `$5 + 3$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Basic binary operation

#### Test Case 2: Basic Subtraction
- **Input:** `5 3 -`
- **Expected Output:** `$5 - 3$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Subtraction operator

#### Test Case 3: Basic Multiplication
- **Input:** `4 7 *`
- **Expected Output:** `$4 \times 7$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** LaTeX multiplication operator

#### Test Case 4: Basic Division
- **Input:** `10 2 /`
- **Expected Output:** `$10 \div 2$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** LaTeX division operator

#### Test Case 5: Unsupported Exponentiation (Error)
- **Input:** `2 3 ^`
- **Expected Output:** `ERROR: Line 1, column 5: Unexpected character '^'`
- **Module Path:** Lexer (error path)
- **Key Feature:** Error handling and positioning

#### Test Case 6: Precedence with Multiplication
- **Input:** `5 3 + 2 *`
- **Expected Output:** `$( 5 + 3 ) \times 2$`
- **Module Path:** Lexer → Parser → LaTeX Generator (precedence logic)
- **Key Feature:** Operator precedence, automatic parenthesization

#### Test Case 7: No Parentheses Needed
- **Input:** `5 3 * 2 +`
- **Expected Output:** `$5 \times 3 + 2$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Precedence rules (multiplication before addition)

#### Test Case 8: Division and Multiplication Chain
- **Input:** `10 2 / 5 *`
- **Expected Output:** `$10 \div 2 \times 5$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Left-to-right associativity, same precedence

#### Test Case 9: Subtraction Chain
- **Input:** `5 3 - 2 -`
- **Expected Output:** `$5 - 3 - 2$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Left-to-right associativity for subtraction

#### Test Case 10: Division Chain
- **Input:** `100 10 / 5 / 2 /`
- **Expected Output:** `$100 \div 10 \div 5 \div 2$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Multiple divisions, left-to-right

#### Test Case 11: Addition Chain
- **Input:** `1 2 + 3 + 4 +`
- **Expected Output:** `$1 + 2 + 3 + 4$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Multiple additions

#### Test Case 12: Multiplication Precedence
- **Input:** `2 3 4 * +`
- **Expected Output:** `$2 + 3 \times 4$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Multiplication has higher precedence than addition

#### Test Case 13: Addition Wrapped by Multiplication
- **Input:** `2 3 + 4 *`
- **Expected Output:** `$( 2 + 3 ) \times 4$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Parenthesization of lower precedence operation

#### Test Case 14: Right Operand Addition Wrapped
- **Input:** `2 3 4 + *`
- **Expected Output:** `$2 \times ( 3 + 4 )$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Parenthesization on right side

#### Test Case 15: Multiplication then Addition
- **Input:** `2 3 * 4 +`
- **Expected Output:** `$2 \times 3 + 4$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** No parentheses for higher precedence followed by lower

#### Test Case 16: Exponentiation with Multiplication (Error)
- **Input:** `2 3 ^ 4 *`
- **Expected Output:** `ERROR: Line 1, column 5: Unexpected character '^'`
- **Module Path:** Lexer (error path)
- **Key Feature:** Error detection

#### Test Case 17: Multiple Exponentiations (Error)
- **Input:** `2 3 4 ^ ^`
- **Expected Output:** `ERROR: Line 1, column 7: Unexpected character '^'`
- **Module Path:** Lexer (error path)
- **Key Feature:** Error detection at correct position

#### Test Case 18: Floating Point Multiplication
- **Input:** `3.14 2 *`
- **Expected Output:** `$3.14 \times 2$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Floating point number support

#### Test Case 19: Floating Point Addition
- **Input:** `1.5 0.5 +`
- **Expected Output:** `$1.5 + 0.5$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Floating point arithmetic

#### Test Case 20: Multiple Groups with Precedence
- **Input:** `1 2 + 3 4 + *`
- **Expected Output:** `$( 1 + 2 ) \times ( 3 + 4 )$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Both operands need parentheses

#### Test Case 21: Complex Precedence
- **Input:** `10 2 / 3 + 4 *`
- **Expected Output:** `$( 10 \div 2 + 3 ) \times 4$`
- **Module Path:** Lexer → Parser → LaTeX Generator
- **Key Feature:** Division and addition grouped, then multiplied

### I/O Contract Summary

| Category | Count |
|----------|-------|
| Successful LaTeX outputs | 18 |
| Error cases | 3 |
| Total test cases | 21 |

### Error Format Specification

All error messages must follow the format:
```
Line {line_number}, column {column_number}: {message}
```

Where:
- `line_number` is 1-based
- `column_number` is 1-based (position of first unexpected character)
- `message` describes the error

### LaTeX Output Format Specification

Successful outputs must follow these rules:
1. Wrap entire expression in dollar signs: `$...$`
2. Use LaTeX operators:
   - Addition: `+` (no special formatting)
   - Subtraction: `-` (no special formatting)
   - Multiplication: `\times` (backslash-times)
   - Division: `\div` (backslash-div)
3. Parentheses format: spaces inside parens `( expression )`
4. Operator spacing: always space before and after operator
5. Number format: preserve as-is from input (including decimals)

---

## Per-Module Specifications

### Module 1: tokens.rs

#### Purpose
Defines the core lexical token types and token structure used throughout the system. Provides the building blocks for all parsing operations.

#### Public API

**Enum: TokenType**
```rust
pub enum TokenType {
    // Literals
    NUMBER,     // Numeric values: 5, 3.14, -2

    // Operators
    PLUS,       // + (addition)
    MINUS,      // - (subtraction)
    MULT,       // * (multiplication)
    DIV,        // / (division)

    // Special
    EOF,        // End of input
}
```

**Struct: Token**
```rust
pub struct Token {
    pub type_: TokenType,    // The token type
    pub value: String,       // String representation of token
    pub line: usize,         // 1-based line number
    pub column: usize,       // 1-based column number
}

impl Token {
    pub fn new(type_: TokenType, value: String, line: usize, column: usize) -> Token
    pub fn debug_repr(&self) -> String  // For debugging output
}
```

#### Data Structures

**TokenType Enum**
- Represents the 6 token categories recognized by the lexer
- Used for pattern matching and token classification
- No associated data
- Derive: Clone, Copy, Debug, PartialEq, Eq

**Token Struct**
- Represents a single lexical token
- Immutable (all fields public, should be treated as read-only)
- Fields:
  - `type_`: Token classification (enum)
  - `value`: String content (e.g., "5", "+", "3.14")
  - `line`: Source line number (1-based)
  - `column`: Source column number (1-based)
- Derive: Clone, Debug, PartialEq, Eq

#### Key Algorithms
- No algorithms; this is a data definition module
- Token creation is straightforward struct initialization

#### Dependencies
- Internal: None
- External: std (for Debug, Clone traits)

#### Rust Mapping

**Python → Rust Type Mapping:**
```python
class TokenType(Enum):          →  pub enum TokenType { ... }
    auto()                       →  Variant without associated data

@dataclass(frozen=True)         →  pub struct Token {
class Token:                        pub type_: TokenType,
    type: TokenType             →  pub value: String,
    value: str                  →  pub line: usize,
    line: int                   →  pub column: usize,
    column: int                 →  }
```

**Special Handling:**
- Python's `auto()` enum values → Rust unit variants
- Python's dataclass frozen → Rust struct with public fields (immutable by default)
- Python's `int` (1-based) → Rust `usize` (1-based, positional)
- Python's `str` → Rust `String` (owned, or `&str` for literals in match)

#### Quality Gates
- `cargo check` - must compile
- `cargo clippy` - no warnings
- `cargo fmt` - formatted code
- No unit tests required for data definitions

---

### Module 2: ast_nodes.rs

#### Purpose
Defines the Abstract Syntax Tree node types that represent parsed mathematical expressions. Forms the core data structure that bridges the parser and LaTeX generator.

#### Public API

**Base Trait: ASTNode**
```rust
pub trait ASTNode: Send + Sync {
    fn line(&self) -> usize;
    fn column(&self) -> usize;
}
```

**Struct: Number** (implements ASTNode)
```rust
pub struct Number {
    pub line: usize,      // 1-based line number
    pub column: usize,    // 1-based column number
    pub value: String,    // String representation of number
}

impl ASTNode for Number {
    fn line(&self) -> usize { self.line }
    fn column(&self) -> usize { self.column }
}
```

**Struct: BinaryOp** (implements ASTNode)
```rust
pub struct BinaryOp {
    pub line: usize,      // 1-based line number
    pub column: usize,    // 1-based column number
    pub operator: String, // "+", "-", "*", "/"
    pub left: Box<Expr>,  // Left operand (heap-allocated)
    pub right: Box<Expr>, // Right operand (heap-allocated)
}

impl ASTNode for BinaryOp {
    fn line(&self) -> usize { self.line }
    fn column(&self) -> usize { self.column }
}
```

**Enum: Expr**
```rust
pub enum Expr {
    Number(Box<Number>),
    BinaryOp(Box<BinaryOp>),
}

impl Expr {
    pub fn as_binary_op(&self) -> Option<&BinaryOp>
    pub fn as_number(&self) -> Option<&Number>
    pub fn line(&self) -> usize     // Delegate to inner node
    pub fn column(&self) -> usize   // Delegate to inner node
}
```

#### Data Structures

**ASTNode Trait**
- Base trait for all AST nodes
- Provides line/column accessors for error reporting
- Enables polymorphic behavior

**Number Struct**
- Leaf node representing numeric literals
- Fields immutable
- Stores value as string (preserves original formatting)
- Derive: Clone, Debug, PartialEq

**BinaryOp Struct**
- Internal node representing binary operations
- Fields immutable
- Uses `Box<Expr>` for children (recursive type, heap allocation)
- Derive: Clone, Debug, PartialEq

**Expr Enum**
- Tagged union of all expression types
- Extensible for future node types (Exponent, SquareRoot, etc.)
- All variants heap-allocated (Box for consistency)
- Derive: Clone, Debug, PartialEq

#### Key Algorithms
- No algorithms; this is a data definition module
- Pattern matching on Expr enum used by Parser and LaTeX Generator

#### Dependencies
- Internal: None
- External: std (for Box, Clone, Debug)

#### Rust Mapping

**Python → Rust Type Mapping:**
```python
@dataclass(frozen=True)
class ASTNode:                  →  pub trait ASTNode {
    line: int                       fn line(&self) -> usize;
    column: int                     fn column(&self) -> usize;
                                }

class Number(ASTNode):          →  pub struct Number {
    value: str                      pub value: String,
                                    pub line: usize,
                                    pub column: usize,
                                }

class BinaryOp(ASTNode):        →  pub struct BinaryOp {
    operator: str                   pub operator: String,
    left: Expr                      pub left: Box<Expr>,
    right: Expr                     pub right: Box<Expr>,
                                    pub line: usize,
                                    pub column: usize,
                                }

Expr = Number | BinaryOp        →  pub enum Expr {
                                    Number(Box<Number>),
                                    BinaryOp(Box<BinaryOp>),
                                }
```

**Special Handling:**
- Python's `@dataclass` inheritance → Rust trait for polymorphism
- Python's `|` type union → Rust `enum`
- Recursive type (BinaryOp contains Expr) → Use `Box<Expr>` for heap allocation
- All variants in enum should be heap-allocated for memory consistency

#### Quality Gates
- `cargo check` - must compile
- `cargo clippy` - no warnings
- `cargo fmt` - formatted code
- No unit tests required for data definitions

---

### Module 3: errors.rs

#### Purpose
Provides error formatting utilities for presenting parse and lexer errors with source context, similar to compiler error output (gcc/rustc style).

#### Public API

**Struct: LexerError**
```rust
pub struct LexerError {
    pub message: String,  // Description of error
    pub line: usize,      // 1-based line number
    pub column: usize,    // 1-based column number
}

impl LexerError {
    pub fn new(message: &str, line: usize, column: usize) -> LexerError

    pub fn with_context(&self, source: &str) -> String
        // Returns formatted error with source context
}

impl Display for LexerError { ... }
impl Error for LexerError { ... }
```

**Struct: ParserError**
```rust
pub struct ParserError {
    pub message: String,  // Description of error
    pub line: usize,      // 1-based line number
    pub column: usize,    // 1-based column number
}

impl ParserError {
    pub fn new(message: &str, line: usize, column: usize) -> ParserError

    pub fn with_context(&self, source: &str) -> String
        // Returns formatted error with source context
}

impl Display for ParserError { ... }
impl Error for ParserError { ... }
```

**Struct: ErrorFormatter**
```rust
pub struct ErrorFormatter {
    source: String,       // Complete source text
    lines: Vec<String>,   // Source split by lines
}

impl ErrorFormatter {
    pub fn new(source: &str) -> ErrorFormatter
        // Initialize with source text

    pub fn format_error(
        &self,
        message: &str,
        line: usize,
        column: usize,
        context_lines: usize,  // Lines before/after error
    ) -> String
        // Format error with source context
}
```

#### Data Structures

**LexerError Struct**
- Represents lexical analysis errors
- Contains error message and position information
- Implements std::error::Error and Display traits

**ParserError Struct**
- Represents parsing errors
- Contains error message and position information
- Implements std::error::Error and Display traits

**ErrorFormatter Struct**
- Utility for formatting errors with source context
- Caches split lines for efficiency
- No mutable state after construction

#### Key Algorithms

**Error Context Extraction**
```
1. Receive line number (1-based), column number (1-based)
2. Convert line to 0-based index
3. Calculate range: [max(0, line-context), min(len, line+context)]
4. For each line in range:
   - Format with line number
   - If error line: append caret line with column alignment
5. Return formatted string
```

**Line Number Width Calculation**
- Calculate width of largest line number for alignment
- Use this width to align line numbers and caret

**Caret Positioning**
- Position caret at column-1 spaces (since column is 1-based)
- Use max(0, column-1) to avoid negative indices

#### Dependencies
- Internal: None
- External: std (Error, Display, fmt)

#### Rust Mapping

**Python → Rust Type Mapping:**
```python
class LexerError(Exception):    →  pub struct LexerError {
    message: str                    pub message: String,
    line: int                       pub line: usize,
    column: int                     pub column: usize,
                                }
                                    impl Error for LexerError { ... }
                                    impl Display for LexerError { ... }

class ErrorFormatter:           →  pub struct ErrorFormatter {
    source: str                     source: String,
    lines: list[str]                lines: Vec<String>,
                                }

    format_error(...) -> str   →    pub fn format_error(...) -> String
```

**Special Handling:**
- Python exceptions → Rust Result<T, Error> with custom error types
- Python's `list[str]` → Rust `Vec<String>`
- Python string methods → Rust string methods (similar API)
- String formatting (padding, spacing) → Use `format!`, `padding`

#### Quality Gates
- `cargo check` - must compile
- `cargo clippy` - no warnings
- `cargo fmt` - formatted code
- Unit tests for error formatting:
  - Test context extraction
  - Test caret positioning
  - Test multiline error output

---

### Module 4: lexer.rs

#### Purpose
Converts raw text input into a stream of tokens. Performs character-by-character scanning with position tracking (line/column). Handles number parsing (integers and decimals) and operator recognition.

#### Public API

**Struct: Lexer**
```rust
pub struct Lexer {
    text: String,         // Input text to tokenize
    pos: usize,           // Current position (0-based)
    line: usize,          // Current line (1-based)
    column: usize,        // Current column (1-based)
}

impl Lexer {
    pub fn new(text: &str) -> Lexer
        // Initialize lexer with input text

    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError>
        // Tokenize entire input, returns tokens or error
        // Tokens end with EOF token
}
```

**Custom Error: LexerError** (defined in errors module)
```rust
pub enum TokenError {
    LexerError {
        message: String,
        line: usize,
        column: usize,
    },
}
```

#### Data Structures

**Lexer Struct**
- State machine for tokenization
- Mutable state: pos, line, column
- Immutable state: text
- Private methods for internal scanning

#### Key Algorithms

**Tokenization Loop**
```
while not at_end():
    skip_whitespace()
    if at_end(): break
    token = scan_token()
    add to tokens list

add EOF token
return tokens
```

**Token Scanning (_scan_token)**
```
save start_line, start_column
peek at current character

match character:
    '+' → return PLUS token
    '-' → check if negative number or MINUS
              if digit follows: scan_number("-")
              else: return MINUS token
    '*' → return MULT token
    '/' → return DIV token
    digit → scan_number("")
    other → raise LexerError
```

**Number Scanning (_scan_number)**
```
value = prefix (e.g., "-" or "")

consume digits (integer part):
    while digit: value += advance()

if current char is '.':
    value += advance()  // consume '.'
    consume digits (decimal part):
        while digit: value += advance()

return NUMBER token with value
```

**Position Tracking**
```
on advance():
    if char == '\n':
        line += 1
        column = 1
    else:
        column += 1
```

**Whitespace Handling**
```
while not at_end() and peek() in " \t\n\r":
    advance()
```

#### Dependencies
- Internal: tokens (Token, TokenType), errors (LexerError)
- External: None

#### Rust Mapping

**Python → Rust Type Mapping:**
```python
class Lexer:                    →  pub struct Lexer {
    text: str                       text: String,
    pos: int                        pos: usize,
    line: int                       line: usize,
    column: int                     column: usize,
                                }

    def tokenize(...) -> list  →  pub fn tokenize(mut self)
                                      -> Result<Vec<Token>, LexerError>

    def _at_end(...)            →  fn at_end(&self) -> bool
    def _peek(...)              →  fn peek(&self) -> char
    def _advance(...)           →  fn advance(&mut self) -> char
    def _skip_whitespace(...)   →  fn skip_whitespace(&mut self)
    def _scan_token(...)        →  fn scan_token(&mut self)
                                      -> Result<Token, LexerError>
    def _scan_number(...)       →  fn scan_number(...)
                                      -> Token
```

**Special Handling:**
- Python's lookahead at_end() → Rust's at_end() with bounds check
- Python's character properties (isdigit) → Rust's char methods (is_ascii_digit)
- Python's negative number detection → Rust's same approach but with explicit char checking
- String iteration → Rust's char indexing with bounds checking
- Floating point detection → Look for '.' character

#### Quality Gates
- `cargo check` - must compile
- `cargo clippy` - no warnings
- `cargo fmt` - formatted code
- Unit tests:
  - Test simple tokenization: "5 3 +"
  - Test floating point: "3.14 2"
  - Test negative numbers: "-5 3"
  - Test unknown character error: "2 3 @" → LexerError
  - Test position tracking (line/column)
- I/O Contract validation:
  - Test cases 1-4, 6-9, 18-21 (success cases)
  - Test cases 5, 16-17 (error cases with correct positioning)

---

### Module 5: parser.rs

#### Purpose
Converts a token stream into an Abstract Syntax Tree using a stack-based RPN (Reverse Polish Notation) algorithm. Validates RPN syntax and provides detailed error messages for invalid expressions.

#### Public API

**Struct: Parser**
```rust
pub struct Parser {
    tokens: Vec<Token>,   // Input token stream
    pos: usize,           // Current position in tokens
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser
        // Initialize parser with token stream

    pub fn parse(mut self) -> Result<Expr, ParserError>
        // Parse tokens into AST
        // Returns root expression or error
}
```

**Custom Error: ParserError** (defined in errors module)
```rust
pub struct ParserError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}
```

#### Data Structures

**Parser Struct**
- State machine for RPN parsing
- Stack-based accumulation (not stored in struct, used locally)
- Position tracking for navigation
- Immutable token stream

#### Key Algorithms

**RPN Parsing Algorithm**
```
stack = []

while not at EOF:
    token = current()

    match token.type:
        NUMBER:
            create Number node from token
            push onto stack
            advance()

        OPERATOR (+, -, *, /):
            if stack.len() < 2:
                error "Not enough operands"

            right = stack.pop()
            left = stack.pop()

            create BinaryOp node:
                operator = map token type to operator string
                left = left node
                right = right node
                line = token.line
                column = token.column

            push BinaryOp onto stack
            advance()

        EOF:
            break

        OTHER:
            error "Unexpected token"

// Validate final state
if stack.is_empty():
    error "Empty expression"
if stack.len() > 1:
    error "Invalid RPN: N values remain (missing operators?)"

return stack[0]  // Root of AST
```

**Token Type Mapping**
```
TokenType::PLUS   → "+"
TokenType::MINUS  → "-"
TokenType::MULT   → "*"
TokenType::DIV    → "/"
```

**Error Messages**
- "Operator '{op}' requires two operands" - insufficient operands
- "Empty expression" - no tokens
- "Invalid RPN: {N} values remain on stack (missing operators?)" - too many operands
- "Unexpected token '{value}'" - unhandled token type

#### Dependencies
- Internal: tokens (Token, TokenType), ast_nodes (Number, BinaryOp, Expr), errors (ParserError)
- External: None

#### Rust Mapping

**Python → Rust Type Mapping:**
```python
class Parser:                   →  pub struct Parser {
    tokens: list[Token]             tokens: Vec<Token>,
    pos: int                        pos: usize,
                                }

    def parse(...) -> Expr      →  pub fn parse(mut self)
                                      -> Result<Expr, ParserError>

    def _current(...) -> Token  →  fn current(&self) -> &Token
    def _at_end(...) -> bool    →  fn at_end(&self) -> bool
    def _advance(...) -> Token  →  fn advance(&mut self) -> Token

    local: stack: list[Expr]    →  local: stack: Vec<Expr>
```

**Special Handling:**
- Python's list pop() → Rust's Vec::pop() returns Option<T>
- Python's list append → Rust's Vec::push()
- Python's empty check → Rust's Vec::is_empty() or Vec::len()
- Pattern matching on token type → Rust's match expression
- Error creation → ParserError::new() function

#### Quality Gates
- `cargo check` - must compile
- `cargo clippy` - no warnings
- `cargo fmt` - formatted code
- Unit tests:
  - Test single number: "5" → Number(5)
  - Test simple operation: "5 3 +" → BinaryOp(+, 5, 3)
  - Test complex RPN: "5 3 + 2 *" → BinaryOp(*, BinaryOp(+, 5, 3), 2)
  - Test error: not enough operands "3 +"
  - Test error: too many operands "5 3 2"
  - Test error: empty input
- I/O Contract validation:
  - All test cases 1-21 (both success and error paths)
  - Verify AST structure for complex expressions

---

### Module 6: latex_gen.rs

#### Purpose
Converts an Abstract Syntax Tree into LaTeX mathematical notation. Implements operator precedence-aware parenthesization and LaTeX operator mapping. Uses a visitor pattern for extensible AST traversal.

#### Public API

**Struct: LaTeXGenerator**
```rust
pub struct LaTeXGenerator;

impl LaTeXGenerator {
    pub fn new() -> LaTeXGenerator

    pub fn generate(&self, ast: &Expr) -> String
        // Convert AST to LaTeX output
        // Returns string like "$5 + 3$"

    // Private methods for visitor pattern
    fn visit(&self, node: &Expr) -> String
    fn visit_number(&self, node: &Number) -> String
    fn visit_binary_op(&self, node: &BinaryOp) -> String
    fn needs_parens(
        &self,
        child: &Expr,
        parent_precedence: usize,
        is_right: bool,
    ) -> bool
}
```

#### Data Structures

**LaTeXGenerator Struct**
- Stateless generator (can be reused for multiple ASTs)
- Holds operator precedence table
- Holds operator to LaTeX mapping

**Precedence Levels**
```rust
const PRECEDENCE: {
    "+": 1,  // Addition/subtraction (lower precedence)
    "-": 1,
    "*": 2,  // Multiplication/division (higher precedence)
    "/": 2,
}
```

**Operator Mapping**
```rust
const BINARY_OPS: {
    "+": "+",
    "-": "-",
    "*": r"\times",
    "/": r"\div",
}
```

#### Key Algorithms

**LaTeX Generation (_visit_binary_op)**
```
get operator LaTeX representation from map
get operator precedence from table

// Recursively generate left operand
left_text = visit(left)
if needs_parens(left, my_precedence, is_right=false):
    left_text = "( " + left_text + " )"

// Recursively generate right operand
right_text = visit(right)
if needs_parens(right, my_precedence, is_right=true):
    right_text = "( " + right_text + " )"

return "{left_text} {op_latex} {right_text}"
```

**Parenthesization Logic (_needs_parens)**
```
if child is not BinaryOp:
    return false  // Numbers don't need parens

child_precedence = precedence[child.operator]

// Lower precedence always needs parens
if child_precedence < parent_precedence:
    return true

// Equal precedence: right side needs parens for non-commutative operators
// This handles left-associativity of - and /
if child_precedence == parent_precedence and is_right:
    if child.operator in ("-", "/"):
        return true

return false
```

**Precedence Rules**
1. Numbers: no parens
2. Child operator has lower precedence than parent: add parens
3. Child operator has equal precedence:
   - Left side: no parens (left-associative)
   - Right side: add parens if operator is - or / (non-commutative)
   - Right side: no parens if operator is + or * (commutative)

**Examples**
```
5 + 3 * 2
- Precedence: * (2) > + (1)
- * is parent of +
- Left of * (5) is number, no parens
- Right of * (3 + 2) is lower precedence, needs parens
- Result: "5 + ( 3 \times 2 )" ← Wait, this is wrong!
- Actually: "5 + 3 \times 2" (multiplication has HIGHER precedence, so it binds tighter, no parens needed)
- The 3*2 is NOT a parent of 5+, so no parens

5 3 + 2 *
- AST: BinaryOp(*, BinaryOp(+, 5, 3), 2)
- Precedence: * (2) > + (1)
- * is parent
- Left of * is + (lower precedence), needs parens → ( 5 + 3 )
- Right of * is 2 (number), no parens
- Result: "( 5 + 3 ) \times 2" ✓

5 3 - 2 -
- AST: BinaryOp(-, BinaryOp(-, 5, 3), 2)
- Precedence: - (1) = - (1)
- Right - is child of left -
- Equal precedence on right side, operator is -, needs parens
- But we're visiting the INNER -, its children are 5 and 3, not parents
- No wait, we start with outer BinaryOp(-, inner, 2)
  - Left of outer - is inner BinaryOp(-, 5, 3)
    - Visit inner: 5 - 3
    - Equal precedence on LEFT side: no parens
  - Right of outer - is 2 (number): no parens
- Result: "5 - 3 - 2" ✓
```

#### Dependencies
- Internal: ast_nodes (Expr, BinaryOp, Number), tokens (Token, TokenType)
- External: None (standard library only)

#### Rust Mapping

**Python → Rust Type Mapping:**
```python
class LaTeXGenerator:            →  pub struct LaTeXGenerator;

    BINARY_OPS: ClassVar[dict]  →  const BINARY_OPS: &[(&str, &str)] = &[...]
    PRECEDENCE: ClassVar[dict]  →  const PRECEDENCE: &[(&str, usize)] = &[...]

    @singledispatchmethod
    def _visit(self, node)       →  fn visit(&self, node: &Expr) -> String {
                                        match node {
                                            Expr::Number(n) => self.visit_number(n),
                                            Expr::BinaryOp(op) => self.visit_binary_op(op),
                                        }
                                    }

    @_visit.register
    def _visit_number(...)       →  fn visit_number(&self, node: &Number) -> String

    @_visit.register
    def _visit_binary_op(...)    →  fn visit_binary_op(&self, node: &BinaryOp) -> String
```

**Special Handling:**
- Python's @singledispatchmethod → Rust's match on enum
- Python's ClassVar[dict] → Rust's const arrays or const functions
- Python's string f-strings → Rust's format! macro
- Python's operator.get() → Rust's match or HashMap lookup
- Dictionary lookups → Use arrays with search or HashMaps for O(1) lookup

#### Quality Gates
- `cargo check` - must compile
- `cargo clippy` - no warnings
- `cargo fmt` - formatted code
- Unit tests:
  - Test number generation: "5" → "5"
  - Test simple operation: "5 + 3" → "$5 + 3$"
  - Test precedence: "5 + 3 * 2" → "$5 + 3 \times 2$"
  - Test parenthesization: "(5 + 3) * 2" → "$( 5 + 3 ) \times 2$"
  - Test left-associativity: "5 - 3 - 2" → "$5 - 3 - 2$"
  - Test floating point: "3.14" → "3.14"
- I/O Contract validation:
  - All successful test cases 1-4, 6-15, 18-21
  - Verify exact output format (spaces, operators, parens)

---

### Module 7: cli.rs (main.rs)

#### Purpose
Command-line interface entry point. Orchestrates the entire pipeline: read input (file or stdin), tokenize, parse, generate LaTeX, and write output. Handles error reporting and exit codes.

#### Public API

**Function: main**
```rust
fn main() -> i32
    // Entry point for CLI
    // Returns exit code: 0 for success, 1 for error
```

**Function: run** (recommended refactoring)
```rust
fn run() -> Result<String, Box<dyn Error>>
    // Core pipeline logic
    // Returns LaTeX output or error
```

#### Data Structures
- Uses argparse-like library for argument parsing
- Represents command-line arguments

#### Key Algorithms

**CLI Pipeline**
```
1. Parse command-line arguments:
   - input: file path or "-" for stdin
   - output: optional output file path (default: stdout)

2. Read input:
   if input == "-":
       read from stdin
   else:
       read from file

   handle errors:
       - FileNotFoundError
       - PermissionError
       - IsADirectoryError

3. Process (with ErrorFormatter):
   - Lexer::tokenize(input) → tokens or error
   - Parser::parse(tokens) → AST or error
   - LaTeXGenerator::generate(AST) → string

4. Handle errors:
   if LexerError:
       format with ErrorFormatter
       print to stderr
       return 1
   if ParserError:
       format with ErrorFormatter
       print to stderr
       return 1

5. Write output:
   if output file specified:
       write LaTeX to file
       print message to stderr
       handle write errors
   else:
       print to stdout

6. Return exit code
```

#### Dependencies
- Internal: tokens (Token), ast_nodes (Expr), errors (LexerError, ParserError, ErrorFormatter), lexer (Lexer), parser (Parser), latex_gen (LaTeXGenerator)
- External: std (file I/O, stderr/stdout, Path), clap or similar for argument parsing

#### Rust Mapping

**Python → Rust Type Mapping:**
```python
def main() -> int:              →  fn main() -> i32 {

    argparse.ArgumentParser()   →  clap::Command::new() or structopt/clap derive

    sys.stdin.read()            →  io::stdin().read_to_string()
    Path(args.input).read_text()→  fs::read_to_string()

    try/except (Python)         →  Result<T, E> or match

    print(..., file=sys.stderr) →  eprintln!()
    print(...)                  →  println!()

    sys.exit(code)              →  return code
```

**Special Handling:**
- Python's argparse → Rust crates: clap, structopt, or pico-args
- Python's exception handling → Rust's Result<T, E> enum
- Python's file I/O → Rust's std::fs, std::io
- Path handling → std::path::Path or pathlib equivalent
- Error formatting → ErrorFormatter from errors module

#### Quality Gates
- `cargo check` - must compile
- `cargo clippy` - no warnings
- `cargo fmt` - formatted code
- Integration tests:
  - Test with file input: rpn2tex input.rpn
  - Test with stdin: echo "5 3 +" | rpn2tex -
  - Test with output file: rpn2tex input.rpn -o output.tex
  - Test error cases: invalid file, permission denied
  - Test error output to stderr
  - Test exit codes
- I/O Contract validation:
  - All 21 test cases must produce exact output
  - Error messages must match format

---

## Migration Order & Dependencies

### Dependency Resolution

```
1. tokens.rs     (no dependencies)
   ├─→ Provides: TokenType, Token

2. ast_nodes.rs  (no dependencies)
   ├─→ Provides: ASTNode trait, Number, BinaryOp, Expr enum

3. errors.rs     (no dependencies)
   ├─→ Provides: LexerError, ParserError, ErrorFormatter

4. lexer.rs      (depends on: tokens.rs, errors.rs)
   ├─→ Consumes: Token, TokenType, LexerError
   ├─→ Provides: Lexer struct, tokenize() method

5. parser.rs     (depends on: tokens.rs, ast_nodes.rs, errors.rs)
   ├─→ Consumes: Token, TokenType, Number, BinaryOp, Expr, ParserError
   ├─→ Provides: Parser struct, parse() method

6. latex_gen.rs  (depends on: ast_nodes.rs)
   ├─→ Consumes: Expr, BinaryOp, Number
   ├─→ Provides: LaTeXGenerator struct, generate() method

7. main.rs       (depends on all modules)
   ├─→ Consumes: all public APIs
   ├─→ Provides: executable binary
```

### Why This Order?

**Justification:**

1. **Tokens First** - No dependencies, foundational data type
2. **AST Nodes Second** - No dependencies, foundational data type
3. **Errors Third** - No dependencies, used by lexer and parser
4. **Lexer Fourth** - Depends on tokens and errors; no complex interdependencies
5. **Parser Fifth** - Depends on tokens, AST nodes, and errors; must come after all of them
6. **LaTeX Generator Sixth** - Only depends on AST nodes; can be developed independently
7. **CLI Last** - Orchestrates everything; must come after all other modules

**Stability Rationale:**
- Core data structures (1-3) have no dependencies and won't change
- Pipeline modules (4-6) can each be built and tested independently
- Each module adds new functionality without breaking previous work
- CLI (7) is the integration point; tested last

---

## Rust Project Structure

### Cargo.toml

```toml
[package]
name = "rpn2tex"
version = "0.1.0"
edition = "2021"

[dependencies]
# For CLI argument parsing (choose one):
# clap = { version = "4.x", features = ["derive"] }
# or use pico-args or structopt

[dev-dependencies]
# For integration tests
# (none required initially)

[lib]
name = "rpn2tex"
path = "src/lib.rs"

[[bin]]
name = "rpn2tex"
path = "src/main.rs"
```

### Module Organization

```
src/
├── lib.rs          (library root, exports public API)
├── main.rs         (CLI entry point)
├── tokens.rs       (Token, TokenType definitions)
├── ast_nodes.rs    (AST node types)
├── errors.rs       (Error types and formatting)
├── lexer.rs        (Lexer implementation)
├── parser.rs       (Parser implementation)
└── latex_gen.rs    (LaTeX generation)

tests/
├── io_contract.rs  (I/O contract validation)
└── integration.rs  (end-to-end tests)
```

### lib.rs

```rust
// Public API exports
pub mod tokens;
pub mod ast_nodes;
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod latex_gen;

// Re-export common types for convenience
pub use tokens::{Token, TokenType};
pub use ast_nodes::{Expr, Number, BinaryOp, ASTNode};
pub use errors::{LexerError, ParserError, ErrorFormatter};
pub use lexer::Lexer;
pub use parser::Parser;
pub use latex_gen::LaTeXGenerator;
```

### main.rs

```rust
use rpn2tex::{Lexer, Parser, LaTeXGenerator, ErrorFormatter, LexerError, ParserError};
use std::fs;
use std::io;
use std::path::Path;

fn main() -> i32 {
    // CLI implementation
    // (see cli.rs specification)
}
```

### Visibility Rules

- **Public** (pub): All public APIs for each module
  - Structs: Token, Number, BinaryOp, Lexer, Parser, LaTeXGenerator, ErrorFormatter
  - Enums: TokenType, Expr
  - Traits: ASTNode
  - Functions: new(), parse(), tokenize(), generate(), etc.

- **Private**: All internal helper functions
  - _scan_token(), _peek(), _advance(), _visit(), _needs_parens(), etc.

---

## Quality Gates & Validation

### Per-Module Quality Checklist

#### Phase: Code Compilation
- [ ] `cargo check` passes for module
- [ ] No syntax errors
- [ ] All dependencies resolved

#### Phase: Linting & Formatting
- [ ] `cargo clippy --all-targets` produces no warnings
- [ ] `cargo fmt` is applied
- [ ] No unsafe code blocks (unless documented)
- [ ] Proper error handling (no unwrap() in library code)

#### Phase: Unit Testing
- [ ] All public functions have unit tests
- [ ] Test cases cover:
  - Happy path (normal operation)
  - Edge cases (empty input, boundary values)
  - Error cases (invalid input)
- [ ] Code coverage for critical paths ≥ 80%

#### Phase: I/O Contract Validation

**For Lexer:**
- [ ] Test Case 1: "5 3 +" tokenizes correctly
- [ ] Test Case 5: "2 3 ^" produces LexerError with correct position
- [ ] Test Case 18: "3.14 2 *" handles floating point
- [ ] Position tracking (line/column) is accurate

**For Parser:**
- [ ] Test Case 1-4: Simple operations parse to correct AST
- [ ] Test Case 6: "5 3 + 2 *" builds correct tree structure
- [ ] Test Case 20: "1 2 + 3 4 + *" handles multiple operands
- [ ] Error messages include correct token position

**For LaTeX Generator:**
- [ ] Test Case 1-4: Basic operations produce correct LaTeX
- [ ] Test Case 6: Parenthesization is correct
- [ ] Test Case 9: "5 3 - 2 -" produces left-associative output
- [ ] Test Case 14: Right operand parenthesization correct
- [ ] All 18 successful cases produce exact matching output

**For CLI:**
- [ ] Test Case 1-21: Full pipeline produces correct output
- [ ] Error cases produce correct error format
- [ ] Exit codes are correct (0 for success, 1 for error)
- [ ] File I/O works (input file, output file, stdin/stdout)

### Integration Testing Strategy

**Level 1: Module Integration Tests**
```rust
#[test]
fn test_lexer_to_parser() {
    let lexer = Lexer::new("5 3 +");
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    assert_eq!(/*...*/);
}
```

**Level 2: Full Pipeline Tests**
```rust
#[test]
fn test_full_pipeline() {
    let input = "5 3 +";
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let gen = LaTeXGenerator::new();
    let latex = gen.generate(&ast);
    assert_eq!(latex, "$5 + 3$");
}
```

**Level 3: I/O Contract Validation**
```rust
#[test]
fn io_contract_test_case_1() {
    assert_pipeline("5 3 +", Ok("$5 + 3$"));
}

#[test]
fn io_contract_test_case_5() {
    assert_pipeline("2 3 ^", Err("Line 1, column 5: Unexpected character '^'"));
}
```

---

## Key Migration Challenges

### 1. Dynamic Typing to Static Typing

**Challenge:** Python allows any type in lists and as function parameters. Rust requires explicit types.

**Solution:**
- Use `Box<Expr>` for recursive AST nodes instead of Python's direct object references
- Use enums (Expr) for tagged unions instead of Python's `|` type hints
- Use Result<T, E> for error handling instead of exceptions

**Example:**
```python
# Python: Any type in list
stack: list[Expr] = []
stack.append(Number(...))
stack.append(BinaryOp(...))
```

```rust
// Rust: Explicit enum type
let mut stack: Vec<Expr> = Vec::new();
stack.push(Expr::Number(Box::new(Number { ... })));
stack.push(Expr::BinaryOp(Box::new(BinaryOp { ... })));
```

### 2. Exception Handling to Result Types

**Challenge:** Python uses exceptions for error handling; Rust uses Result<T, E>.

**Solution:**
- Create custom error types (LexerError, ParserError) that implement std::error::Error
- Use Result<T, E> return types for fallible operations
- Use ? operator for error propagation
- Use match for error handling in CLI

**Example:**
```python
# Python: Exception
try:
    tokens = lexer.tokenize()
except LexerError as e:
    print(e)
```

```rust
// Rust: Result
match lexer.tokenize() {
    Ok(tokens) => { /* continue */ },
    Err(e) => { eprintln!("{}", e); return 1; }
}
```

### 3. Mutable State Management

**Challenge:** Python allows free mutation; Rust requires explicit `mut` and borrowing rules.

**Solution:**
- Mark struct fields that need to change as mutable in methods
- Use `&mut self` for methods that modify state
- Consume self (move) when appropriate (e.g., Parser::parse)
- Use cell/RefCell only when interior mutability is truly necessary (avoid for this project)

**Example:**
```python
# Python: Implicit mutation
class Lexer:
    def __init__(self, text: str):
        self.pos = 0

    def _advance(self):
        self.pos += 1  # Mutation
```

```rust
// Rust: Explicit mutable reference
pub struct Lexer {
    pos: usize,
}

impl Lexer {
    fn advance(&mut self) -> char {
        self.pos += 1;  // Requires &mut self
    }
}
```

### 4. String Handling

**Challenge:** Python uses immutable strings; Rust distinguishes between String (owned) and &str (borrowed).

**Solution:**
- Use `String` for owned string data (input text, token values)
- Use `&str` for string literals and borrowed references
- Use `format!()` macro for string building (similar to f-strings)
- Use `.chars()` for character iteration, not indexing

**Example:**
```python
# Python: String indexing
text = "5 3 +"
for i in range(len(text)):
    char = text[i]
```

```rust
// Rust: Explicit character handling
let text = "5 3 +";
for (i, char) in text.chars().enumerate() {
    // Use char directly
}

// Or with position tracking:
let mut pos = 0;
while pos < text.len() {
    let char = text[pos..pos+1].chars().next().unwrap();
    pos += 1;
}
```

### 5. List/Dict Operations to Vec/HashMap

**Challenge:** Python lists and dicts have different performance characteristics and APIs.

**Solution:**
- Use `Vec<T>` for dynamic arrays (similar to Python list)
- Use `HashMap<K, V>` for key-value pairs (similar to Python dict)
- Use array for small fixed-size collections (precedence table)
- Pre-allocate Vec with capacity when size is known

**Example:**
```python
# Python: Dict lookup
PRECEDENCE = {"+": 1, "-": 1, "*": 2, "/": 2}
prec = PRECEDENCE[op]
```

```rust
// Rust: Match-based or const array lookup
const PRECEDENCE: &[(&str, usize)] = &[
    ("+", 1), ("-", 1), ("*", 2), ("/", 2)
];

let prec = PRECEDENCE.iter()
    .find(|(op, _)| *op == operator)
    .map(|(_, p)| *p);
```

### 6. Number Type Conversions

**Challenge:** Python's int/float are flexible; Rust requires explicit types.

**Solution:**
- Store numbers as String to preserve original formatting
- Use usize for line/column numbers (always positive)
- Use i64 or f64 only if numeric operations needed (not required for this project)

**Example:**
```python
# Python: Flexible numbers
class Number(ASTNode):
    value: str  # "42" or "3.14"
```

```rust
// Rust: Same approach (store as string)
pub struct Number {
    pub value: String,  // "42" or "3.14"
}
```

### 7. Iterator Patterns vs Loops

**Challenge:** Python uses iteration; Rust has powerful iterator adaptors.

**Solution:**
- Use explicit `while` or `for` loops for scanning (mimics Python)
- Use iterator methods (.map(), .filter(), .collect()) for transformations
- Use `.chars()`, `.lines()`, `.split()` for string operations

**Example:**
```python
# Python: Explicit loop
while not self._at_end():
    self._skip_whitespace()
    if self._at_end():
        break
    tokens.append(self._scan_token())
```

```rust
// Rust: Similar explicit loop
while !self.at_end() {
    self.skip_whitespace();
    if self.at_end() {
        break;
    }
    tokens.push(self.scan_token()?);
}
```

### 8. Position Tracking Across Newlines

**Challenge:** Multi-line input requires careful position tracking.

**Solution:**
- Track line number (increment on '\n')
- Track column number (reset to 1 on '\n')
- Both are 1-based for human-friendly error messages

**Example:**
```rust
fn advance(&mut self) -> char {
    let char = self.text[self.pos..self.pos+1].chars().next().unwrap();
    self.pos += 1;

    if char == '\n' {
        self.line += 1;
        self.column = 1;
    } else {
        self.column += 1;
    }

    char
}
```

### 9. Visitor Pattern Implementation

**Challenge:** Python uses @singledispatchmethod for visitor pattern; Rust has different idioms.

**Solution:**
- Use match on enum for simple type dispatch
- Implement separate methods for each variant
- Pass self for maintaining state (operator tables)

**Example:**
```python
# Python: Visitor with dispatch
@singledispatchmethod
def _visit(self, node: Expr) -> str:
    raise NotImplementedError()

@_visit.register
def _visit_number(self, node: Number) -> str:
    return node.value
```

```rust
// Rust: Enum match
fn visit(&self, node: &Expr) -> String {
    match node {
        Expr::Number(n) => self.visit_number(n),
        Expr::BinaryOp(op) => self.visit_binary_op(op),
    }
}

fn visit_number(&self, node: &Number) -> String {
    node.value.clone()
}
```

---

## Type Mappings

### Complete Python to Rust Type Mapping

| Python Type | Rust Type | Notes |
|-------------|-----------|-------|
| `int` (line/column) | `usize` | Always non-negative, positions are 1-based |
| `int` (generic) | `i32` or `i64` | Not needed for this project |
| `float` | `f64` | Not needed; stored as string |
| `str` | `String` (owned) or `&str` (borrowed) | Use String for owned data, &str for literals |
| `list[T]` | `Vec<T>` | Dynamic array |
| `dict[K, V]` | `HashMap<K, V>` or match on array | Key-value map |
| `Enum` (token) | `enum TokenType { ... }` | Unit variants |
| `@dataclass` | `pub struct` | Public fields by default |
| `@dataclass(frozen=True)` | `pub struct` | Immutable by default |
| `Optional[T]` | `Option<T>` | None or Some(value) |
| `T \| U` (union) | `enum { Variant1(T), Variant2(U) }` | Tagged union with Box for recursion |
| Exception | `Result<T, E>` where E impl Error | Error handling |
| `super().__init__()` | No equivalent | Rust doesn't have inheritance; use composition |

### Per-Module Type Mapping

**tokens.rs**
```
TokenType(Enum) → pub enum TokenType { ... }
Token(dataclass) → pub struct Token { pub type_: TokenType, ... }
```

**ast_nodes.rs**
```
ASTNode(dataclass base) → pub trait ASTNode { fn line(&self) -> usize; ... }
Number(dataclass) → pub struct Number { pub value: String, ... }
BinaryOp(dataclass) → pub struct BinaryOp { pub left: Box<Expr>, ... }
Expr (type alias) → pub enum Expr { Number(Box<Number>), BinaryOp(Box<BinaryOp>) }
```

**errors.rs**
```
LexerError(Exception) → pub struct LexerError { pub message: String, ... }
                        impl Error + Display for LexerError
ParserError(Exception) → pub struct ParserError { pub message: String, ... }
                         impl Error + Display for ParserError
ErrorFormatter(class) → pub struct ErrorFormatter { source: String, ... }
```

**lexer.rs**
```
Lexer(class) → pub struct Lexer { text: String, pos: usize, ... }
LexerError (exception) → Result<Vec<Token>, LexerError>
```

**parser.rs**
```
Parser(class) → pub struct Parser { tokens: Vec<Token>, pos: usize }
ParserError (exception) → Result<Expr, ParserError>
```

**latex_gen.rs**
```
LaTeXGenerator(class) → pub struct LaTeXGenerator
BINARY_OPS (dict) → const BINARY_OPS: &[...] or HashMap
PRECEDENCE (dict) → const PRECEDENCE: &[...] or HashMap
```

---

## Pattern Changes

### 1. Class to Struct + Impl Blocks

**Python Pattern:**
```python
class Lexer:
    def __init__(self, text: str):
        self.text = text
        self.pos = 0

    def tokenize(self) -> list[Token]:
        # method implementation
```

**Rust Pattern:**
```rust
pub struct Lexer {
    text: String,
    pos: usize,
}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        Lexer {
            text: text.to_string(),
            pos: 0,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> {
        // method implementation
    }
}
```

### 2. Exception to Result

**Python Pattern:**
```python
try:
    tokens = lexer.tokenize()
except LexerError as e:
    print(e.message)
    exit(1)
```

**Rust Pattern:**
```rust
match lexer.tokenize() {
    Ok(tokens) => { /* continue */ },
    Err(e) => {
        eprintln!("{}", e);
        return 1;
    }
}

// Or with ? operator:
let tokens = lexer.tokenize()?;
```

### 3. Dispatch Method to Match

**Python Pattern:**
```python
@singledispatchmethod
def _visit(self, node: Expr) -> str:
    raise NotImplementedError()

@_visit.register
def _visit_number(self, node: Number) -> str:
    return node.value
```

**Rust Pattern:**
```rust
fn visit(&self, node: &Expr) -> String {
    match node {
        Expr::Number(n) => n.value.clone(),
        Expr::BinaryOp(op) => self.visit_binary_op(op),
    }
}
```

### 4. Stack Operations

**Python Pattern:**
```python
stack: list[Expr] = []
stack.append(node)
value = stack.pop()
if len(stack) < 2:
    error
```

**Rust Pattern:**
```rust
let mut stack: Vec<Expr> = Vec::new();
stack.push(node);
let value = stack.pop().ok_or(ParserError::new(...))?;
if stack.len() < 2 {
    return Err(ParserError::new(...));
}
```

### 5. String Building

**Python Pattern:**
```python
value = ""
while not self._at_end() and self._peek().isdigit():
    value += self._advance()
```

**Rust Pattern:**
```rust
let mut value = String::new();
while !self.at_end() && self.peek().is_ascii_digit() {
    value.push(self.advance());
}
```

### 6. Character Testing

**Python Pattern:**
```python
if char.isdigit():
    # handle digit
if char in " \t\n\r":
    # handle whitespace
```

**Rust Pattern:**
```rust
if char.is_ascii_digit() {
    // handle digit
}
if matches!(char, ' ' | '\t' | '\n' | '\r') {
    // handle whitespace
}
```

### 7. File I/O

**Python Pattern:**
```python
if args.input == "-":
    text = sys.stdin.read()
else:
    text = Path(args.input).read_text()

if args.output:
    Path(args.output).write_text(latex + "\n")
```

**Rust Pattern:**
```rust
let text = if args.input == "-" {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    buffer
} else {
    fs::read_to_string(&args.input)?
};

if let Some(output) = args.output {
    fs::write(&output, format!("{}\n", latex))?;
}
```

### 8. Argument Parsing

**Python Pattern:**
```python
import argparse

parser = argparse.ArgumentParser(...)
parser.add_argument("input", type=str, help="...")
parser.add_argument("-o", "--output", type=Path, help="...")
args = parser.parse_args()
```

**Rust Pattern (using clap):**
```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "rpn2tex")]
#[command(about = "Convert RPN to LaTeX")]
struct Args {
    /// Input file (use '-' for stdin)
    input: String,

    /// Output file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

let args = Args::parse();
```

---

## Implementation Priorities

### Must-Have (Phase 1)
1. All token and AST types (tokens.rs, ast_nodes.rs)
2. Basic error handling (errors.rs)
3. Lexer with position tracking (lexer.rs)
4. Stack-based parser (parser.rs)
5. LaTeX generation with operator precedence (latex_gen.rs)
6. CLI with file I/O (main.rs)

### Nice-to-Have (Future)
1. Support for ^ (exponentiation) operator
2. Support for sqrt() function
3. Support for nth root
4. Optimized LaTeX output (minimal parens)
5. Custom output formatting options

### Not In Scope (Phase 1)
1. Performance optimization
2. Parallel processing
3. REPL interface
4. Alternative output formats

---

## Testing Strategy Summary

### Unit Tests (Per Module)
- tokens.rs: No tests (data definitions)
- ast_nodes.rs: No tests (data definitions)
- errors.rs: 3-5 tests for formatting
- lexer.rs: 8-10 tests for tokenization and errors
- parser.rs: 8-10 tests for AST construction and errors
- latex_gen.rs: 8-10 tests for LaTeX generation and precedence

### Integration Tests
- Full pipeline: 5 comprehensive end-to-end tests
- I/O contract: 21 contract tests (all must pass)

### Acceptance Criteria
- All 21 I/O contract tests pass with exact matching output
- All modules compile without warnings
- Code is formatted with rustfmt
- No clippy warnings (except justified exceptions)
- Exit codes correct (0 for success, 1 for errors)

---

## Conclusion

This specification provides a comprehensive roadmap for migrating rpn2tex from Python to Rust. The module-by-module approach ensures:

1. **Incremental Progress** - Each module can be built, tested, and validated independently
2. **Dependency Management** - Clear ordering prevents circular dependencies
3. **Quality Control** - Each module has defined quality gates before proceeding
4. **Behavioral Equivalence** - The I/O contract ensures exact compatibility
5. **Maintainability** - Clear structure and documentation for future developers

The Rust implementation should follow the specifications exactly, maintaining:
- Identical input/output behavior
- Same error messages and positioning
- Consistent code organization
- Idiomatic Rust patterns

Reference this document when implementing; it is the single source of truth for the migration.
