# rpn2tex Python to Rust Migration Specification

## Executive Summary

This document provides a comprehensive specification for migrating the rpn2tex Python codebase to idiomatic Rust. The migration follows a four-phase approach with I/O contract validation to ensure behavioral equivalence.

**Migration Strategy**: Module-by-module translation in dependency order, with quality gates at each step.

**Success Criteria**: The Rust implementation must produce **identical outputs** for all test inputs defined in the I/O contract.

---

## Table of Contents

1. [I/O Contract](#io-contract)
2. [Architecture Overview](#architecture-overview)
3. [Phase 1: Core Data Types](#phase-1-core-data-types)
   - [tokens.py → tokens.rs](#module-1-tokenspy--tokensrs)
   - [ast_nodes.py → ast.rs](#module-2-ast_nodespy--astrs)
   - [errors.py → error.rs](#module-3-errorspy--errorrs)
4. [Phase 2: Processing Pipeline](#phase-2-processing-pipeline)
   - [lexer.py → lexer.rs](#module-4-lexerpy--lexerrs)
   - [parser.py → parser.rs](#module-5-parserpy--parserrs)
   - [latex_gen.py → latex.rs](#module-6-latex_genpy--latexrs)
5. [Phase 3: CLI](#phase-3-cli)
   - [cli.py → main.rs](#module-7-clipy--mainrs)
6. [Testing Strategy](#testing-strategy)
7. [Quality Gates](#quality-gates)

---

## I/O Contract

The Rust implementation must produce **exact** outputs for the following 21 test cases.

### Successful Operations (Exit Code: 0)

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 +` | `$5 + 3$` | Basic addition |
| `5 3 -` | `$5 - 3$` | Basic subtraction |
| `4 7 *` | `$4 \times 7$` | Basic multiplication with LaTeX times symbol |
| `10 2 /` | `$10 \div 2$` | Basic division with LaTeX divide symbol |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Parentheses added for precedence |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | Multiplication higher precedence, no parens |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | Left-to-right evaluation |
| `5 3 - 2 -` | `$5 - 3 - 2$` | Left-to-right subtraction |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Multiple divisions |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Multiple additions |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Multiplication has precedence |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Addition child of multiplication |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Right operand parenthesized |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | No parentheses needed |
| `3.14 2 *` | `$3.14 \times 2$` | Floating point numbers |
| `1.5 0.5 +` | `$1.5 + 0.5$` | Floating point addition |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Multiple parenthesized subexpressions |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Mixed operations with proper precedence |

### Error Cases (Exit Code: 1)

| Input | Error Message Pattern | Notes |
|-------|----------------------|-------|
| `2 3 ^` | `Error: Unexpected character '^'` with position marker | Caret not supported |
| `2 3 ^ 4 *` | `Error: Unexpected character '^'` with position marker | Error on first invalid char |
| `2 3 4 ^ ^` | `Error: Unexpected character '^'` with position marker | Error stops at first invalid |

### Critical Output Format Rules

1. **Math Mode Delimiters**: All outputs wrapped in `$...$`
2. **Operator Symbols**:
   - `+` → `+`
   - `-` → `-`
   - `*` → `\times` (NOT `*`)
   - `/` → `\div` (NOT `/`)
3. **Spacing**:
   - Single space around operators: ` + `, ` \times `, ` \div `
   - Spaces inside parentheses: `( expr )`
4. **Number Preservation**:
   - Keep exact string representation from input
   - `3.14` outputs as `3.14` (not normalized)
5. **Error Format**:
   ```
   Error: <message>

   <line_num> | <source>
     |        <caret>
   ```

---

## Architecture Overview

### Python Component Chain

```
Input String
    ↓
[Lexer] - Character-by-character scanning with position tracking
    ↓
[Token Stream] - List of Token objects
    ↓
[Parser] - Stack-based RPN parsing
    ↓
[AST] - Tree of Number and BinaryOp nodes
    ↓
[LaTeXGenerator] - Visitor pattern with precedence handling
    ↓
[LaTeX String] - Infix notation in math mode
    ↓
Output
```

### Module Dependencies

```
tokens.py (no dependencies)
    ↑
ast_nodes.py (no dependencies)
    ↑
errors.py (no dependencies)
    ↑
lexer.py (depends on: tokens)
    ↑
parser.py (depends on: tokens, ast_nodes)
    ↑
latex_gen.py (depends on: ast_nodes)
    ↑
cli.py (depends on: all above)
```

### Rust Module Structure

```
src/
├── tokens.rs       - Token types and TokenType enum
├── ast.rs          - AST node definitions (Expr enum)
├── error.rs        - ErrorFormatter struct
├── lexer.rs        - Lexer struct with tokenize()
├── parser.rs       - Parser struct with parse()
├── latex.rs        - LaTeXGenerator with precedence logic
├── main.rs         - CLI orchestration
└── lib.rs          - Library root (re-exports)
```

---

## Phase 1: Core Data Types

### Module 1: tokens.py → tokens.rs

#### Python Implementation Analysis

**Location**: `source/tokens.py` (71 lines)

**Key Components**:
1. `TokenType` enum (6 variants):
   - `NUMBER` - numeric literals
   - `PLUS`, `MINUS`, `MULT`, `DIV` - operators
   - `EOF` - end marker
2. `Token` dataclass (4 fields):
   - `type: TokenType`
   - `value: str`
   - `line: int` (1-based)
   - `column: int` (1-based)

**Features**:
- `TokenType` uses `auto()` for enum values
- `Token` is frozen (immutable)
- Custom `__repr__` for debugging

#### Rust Type Mappings

##### TokenType Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Literals
    Number,

    // Operators
    Plus,
    Minus,
    Mult,
    Div,

    // Special
    Eof,
}
```

**Rationale**:
- `Copy` - Enum is simple and Copy-able
- `PartialEq + Eq` - For matching in parser
- `Debug` - For error messages
- PascalCase names (Rust convention)

##### Token Struct

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            token_type,
            value: value.into(),
            line,
            column,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({:?}, '{}', {}:{})", self.token_type, self.value, self.line, self.column)
    }
}
```

**Rationale**:
- `Clone` - Tokens may need to be cloned
- `PartialEq + Eq` - For testing
- `usize` for line/column (natural for Rust indexing)
- `String` for value (owned, flexible)
- `Display` impl for debugging (replaces Python's `__repr__`)

#### Implementation Notes

1. **Position tracking**: Use 1-based indexing to match Python
2. **String allocation**: Accept `impl Into<String>` in constructor for ergonomics
3. **No enum variants for operators**: Keep simple enum without data

#### Validation

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Number, "42", 1, 5);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.value, "42");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 5);
    }

    #[test]
    fn test_token_display() {
        let token = Token::new(TokenType::Plus, "+", 1, 3);
        let display = format!("{}", token);
        assert!(display.contains("Plus"));
        assert!(display.contains("+"));
    }
}
```

---

### Module 2: ast_nodes.py → ast.rs

#### Python Implementation Analysis

**Location**: `source/ast_nodes.py` (91 lines)

**Key Components**:
1. `ASTNode` base class:
   - `line: int`
   - `column: int`
2. `Number` dataclass (extends ASTNode):
   - `value: str` - preserved string representation
3. `BinaryOp` dataclass (extends ASTNode):
   - `operator: str` - "+", "-", "*", "/"
   - `left: Expr` - left operand
   - `right: Expr` - right operand
4. `Expr` type alias: `Number | BinaryOp`

**Features**:
- All nodes frozen (immutable)
- Position tracking for error reporting
- Recursive structure (BinaryOp contains Expr children)

#### Rust Type Mappings

##### Expr Enum (Algebraic Data Type)

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number {
        value: String,
        line: usize,
        column: usize,
    },
    BinaryOp {
        operator: String,
        left: Box<Expr>,
        right: Box<Expr>,
        line: usize,
        column: usize,
    },
}
```

**Rationale**:
- Enum over trait - More idiomatic Rust for closed set of types
- `Box<Expr>` - Required for recursive type, heap-allocated
- `Clone` - For tree manipulation if needed
- `PartialEq` - For testing
- Each variant carries its own position fields

##### Position Accessor Methods

```rust
impl Expr {
    pub fn line(&self) -> usize {
        match self {
            Expr::Number { line, .. } => *line,
            Expr::BinaryOp { line, .. } => *line,
        }
    }

    pub fn column(&self) -> usize {
        match self {
            Expr::Number { column, .. } => *column,
            Expr::BinaryOp { column, .. } => *column,
        }
    }
}
```

#### Construction Helpers

```rust
impl Expr {
    pub fn number(value: impl Into<String>, line: usize, column: usize) -> Self {
        Expr::Number {
            value: value.into(),
            line,
            column,
        }
    }

    pub fn binary_op(
        operator: impl Into<String>,
        left: Expr,
        right: Expr,
        line: usize,
        column: usize,
    ) -> Self {
        Expr::BinaryOp {
            operator: operator.into(),
            left: Box::new(left),
            right: Box::new(right),
            line,
            column,
        }
    }
}
```

#### Pattern Matching Examples

```rust
// Visitor pattern in Rust uses match:
match expr {
    Expr::Number { value, .. } => {
        // Handle number
    }
    Expr::BinaryOp { operator, left, right, .. } => {
        // Handle binary operation
    }
}
```

#### Implementation Notes

1. **Box for recursion**: `Box<Expr>` required for recursive enum
2. **String for operator**: Keep as String for flexibility (could optimize to enum later)
3. **String for value**: Preserve exact input representation
4. **No separate base class**: Rust enums replace inheritance hierarchy

#### Validation

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_creation() {
        let num = Expr::number("42", 1, 1);
        assert_eq!(num.line(), 1);
        assert_eq!(num.column(), 1);
        match num {
            Expr::Number { value, .. } => assert_eq!(value, "42"),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_binary_op_creation() {
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let op = Expr::binary_op("+", left, right, 1, 2);

        match op {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
            _ => panic!("Expected BinaryOp"),
        }
    }
}
```

---

### Module 3: errors.py → error.rs

#### Python Implementation Analysis

**Location**: `source/errors.py` (128 lines)

**Key Components**:
1. `ErrorFormatter` class:
   - `source: str` - complete source text
   - `lines: list[str]` - split into lines
2. Methods:
   - `format_error(message, line, column, context_lines=1)` - main formatter
   - `_get_context(line, column, context_lines)` - extract context

**Algorithm** (`format_error`):
```python
def format_error(message, line, column, context_lines=1):
    parts = []
    parts.append(f"Error: {message}")
    parts.append("")  # Blank line
    context = _get_context(line, column, context_lines)
    parts.append(context)
    return "\n".join(parts)
```

**Algorithm** (`_get_context`):
```python
def _get_context(line, column, context_lines):
    # Convert to 0-based
    error_idx = line - 1

    # Calculate range
    start_idx = max(0, error_idx - context_lines)
    end_idx = min(len(lines), error_idx + context_lines + 1)

    # Calculate width for alignment
    max_line_num = end_idx
    num_width = len(str(max_line_num))

    result = []
    for idx in range(start_idx, end_idx):
        line_num = idx + 1
        line_content = lines[idx]

        # Format: "1 | source code"
        prefix = f"{line_num:>{num_width}} | "
        result.append(f"{prefix}{line_content}")

        # Add caret on error line
        if idx == error_idx:
            caret_prefix = " " * num_width + " | "
            caret_pos = max(0, column - 1)
            caret_line = caret_prefix + " " * caret_pos + "^"
            result.append(caret_line)

    return "\n".join(result)
```

**Output Format**:
```
Error: Unexpected character '^'

1 | 2 3 ^
  |     ^
```

#### Rust Type Mappings

##### ErrorFormatter Struct

```rust
pub struct ErrorFormatter {
    source: String,
    lines: Vec<String>,
}

impl ErrorFormatter {
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let lines = source.lines().map(|s| s.to_string()).collect();
        Self { source, lines }
    }

    pub fn format_error(&self, message: &str, line: usize, column: usize) -> String {
        self.format_error_with_context(message, line, column, 1)
    }

    pub fn format_error_with_context(
        &self,
        message: &str,
        line: usize,
        column: usize,
        context_lines: usize,
    ) -> String {
        let mut parts = Vec::new();

        // Error header
        parts.push(format!("Error: {}", message));
        parts.push(String::new()); // Blank line

        // Source context
        let context = self.get_context(line, column, context_lines);
        parts.push(context);

        parts.join("\n")
    }

    fn get_context(&self, line: usize, column: usize, context_lines: usize) -> String {
        // Convert to 0-based index
        let error_idx = line.saturating_sub(1);

        // Calculate range (clamped to valid indices)
        let start_idx = error_idx.saturating_sub(context_lines);
        let end_idx = std::cmp::min(self.lines.len(), error_idx + context_lines + 1);

        // Calculate line number width for alignment
        let max_line_num = end_idx;
        let num_width = max_line_num.to_string().len();

        let mut result_lines = Vec::new();

        for idx in start_idx..end_idx {
            let line_num = idx + 1; // Convert back to 1-based
            let line_content = self.lines.get(idx).map(|s| s.as_str()).unwrap_or("");

            // Format line with number: "1 | source code"
            let prefix = format!("{:>width$} | ", line_num, width = num_width);
            result_lines.push(format!("{}{}", prefix, line_content));

            // Add caret on error line
            if idx == error_idx {
                // Spaces for line number column, then position caret
                let caret_prefix = format!("{:>width$} | ", "", width = num_width);
                // Position caret at column (1-based, so column-1 spaces)
                let caret_pos = column.saturating_sub(1);
                let caret_line = format!("{}{:>width$}^", caret_prefix, "", width = caret_pos);
                result_lines.push(caret_line);
            }
        }

        result_lines.join("\n")
    }
}
```

#### Implementation Notes

1. **String ownership**: Store owned `String` and `Vec<String>`
2. **saturating_sub**: Prevent underflow with 1-based indexing
3. **Format width**: Use `{:>width$}` for right-alignment
4. **Context lines**: Default to 1, allow customization
5. **Error format**: Match Python exactly

#### Validation

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_formatter() {
        let formatter = ErrorFormatter::new("5 3 ^");
        let error = formatter.format_error("Unexpected character '^'", 1, 5);

        assert!(error.contains("Error: Unexpected character '^'"));
        assert!(error.contains("1 | 5 3 ^"));
        assert!(error.contains("^"));
    }

    #[test]
    fn test_caret_position() {
        let formatter = ErrorFormatter::new("hello world");
        let error = formatter.format_error("Test", 1, 7);

        // Caret should be at position 6 (0-based)
        let lines: Vec<&str> = error.lines().collect();
        assert!(lines[2].trim_start().starts_with("^"));
    }
}
```

---

## Phase 2: Processing Pipeline

### Module 4: lexer.py → lexer.rs

#### Python Implementation Analysis

**Location**: `source/lexer.py` (201 lines)

**Key Components**:
1. `LexerError` exception:
   - `message: str`
   - `line: int`
   - `column: int`
2. `Lexer` class:
   - `text: str` - input text
   - `pos: int` - current position (0-based)
   - `line: int` - current line (1-based)
   - `column: int` - current column (1-based)

**Methods**:
- `tokenize() -> list[Token]` - main entry point
- `_at_end() -> bool` - check if at end
- `_peek() -> str` - look at current char without consuming
- `_advance() -> str` - consume and return current char
- `_skip_whitespace()` - skip whitespace
- `_scan_token() -> Token` - scan next token
- `_scan_number(prefix, line, column) -> Token` - scan number

**Algorithm** (`tokenize`):
```python
def tokenize():
    tokens = []
    while not _at_end():
        _skip_whitespace()
        if _at_end():
            break
        tokens.append(_scan_token())
    tokens.append(Token(EOF, "", line, column))
    return tokens
```

**Algorithm** (`_scan_token`):
```python
def _scan_token():
    start_line = line
    start_column = column
    char = _peek()

    # Single-character operators
    if char == '+':
        _advance()
        return Token(PLUS, '+', start_line, start_column)
    if char == '-':
        _advance()
        # Check for negative number
        if not _at_end() and _peek().isdigit():
            return _scan_number('-', start_line, start_column)
        return Token(MINUS, '-', start_line, start_column)
    if char == '*':
        _advance()
        return Token(MULT, '*', start_line, start_column)
    if char == '/':
        _advance()
        return Token(DIV, '/', start_line, start_column)

    # Numbers
    if char.isdigit():
        return _scan_number('', start_line, start_column)

    # Unknown character
    raise LexerError(f"Unexpected character '{char}'", start_line, start_column)
```

**Algorithm** (`_scan_number`):
```python
def _scan_number(prefix, start_line, start_column):
    value = prefix

    # Integer part
    while not _at_end() and _peek().isdigit():
        value += _advance()

    # Decimal part (optional)
    if not _at_end() and _peek() == '.':
        value += _advance()  # consume '.'
        while not _at_end() and _peek().isdigit():
            value += _advance()

    return Token(NUMBER, value, start_line, start_column)
```

**Special Cases**:
- Negative numbers: `-5` is scanned as a single NUMBER token with value "-5"
- Whitespace: Acts as delimiter, otherwise ignored
- Newlines: Update line counter, reset column to 1
- Invalid characters: Raise LexerError immediately

#### Rust Type Mappings

##### LexerError Type

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl LexerError {
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line {}, column {}: {}", self.line, self.column, self.message)
    }
}

impl std::error::Error for LexerError {}
```

##### Lexer Struct

```rust
use crate::tokens::{Token, TokenType};

pub struct Lexer {
    text: String,
    chars: Vec<char>,  // For efficient indexing
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let chars = text.chars().collect();
        Self {
            text,
            chars,
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while !self.at_end() {
            self.skip_whitespace();
            if self.at_end() {
                break;
            }
            tokens.push(self.scan_token()?);
        }

        // Add EOF token
        tokens.push(Token::new(TokenType::Eof, "", self.line, self.column));
        Ok(tokens)
    }

    fn at_end(&self) -> bool {
        self.pos >= self.chars.len()
    }

    fn peek(&self) -> Option<char> {
        if self.at_end() {
            None
        } else {
            Some(self.chars[self.pos])
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.chars[self.pos];
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        let ch = self.peek().ok_or_else(|| {
            LexerError::new("Unexpected end of input", start_line, start_column)
        })?;

        // Single-character operators
        match ch {
            '+' => {
                self.advance();
                Ok(Token::new(TokenType::Plus, "+", start_line, start_column))
            }
            '-' => {
                self.advance();
                // Check for negative number
                if let Some(next_ch) = self.peek() {
                    if next_ch.is_ascii_digit() {
                        return self.scan_number("-", start_line, start_column);
                    }
                }
                Ok(Token::new(TokenType::Minus, "-", start_line, start_column))
            }
            '*' => {
                self.advance();
                Ok(Token::new(TokenType::Mult, "*", start_line, start_column))
            }
            '/' => {
                self.advance();
                Ok(Token::new(TokenType::Div, "/", start_line, start_column))
            }
            _ if ch.is_ascii_digit() => {
                self.scan_number("", start_line, start_column)
            }
            _ => {
                Err(LexerError::new(
                    format!("Unexpected character '{}'", ch),
                    start_line,
                    start_column,
                ))
            }
        }
    }

    fn scan_number(&mut self, prefix: &str, start_line: usize, start_column: usize) -> Result<Token, LexerError> {
        let mut value = prefix.to_string();

        // Integer part
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                value.push(self.advance());
            } else {
                break;
            }
        }

        // Decimal part (optional)
        if let Some('.') = self.peek() {
            value.push(self.advance()); // consume '.'
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    value.push(self.advance());
                } else {
                    break;
                }
            }
        }

        Ok(Token::new(TokenType::Number, value, start_line, start_column))
    }
}
```

#### Implementation Notes

1. **Vec<char> for indexing**: Store chars for efficient random access
2. **Result type**: Return `Result<Vec<Token>, LexerError>`
3. **Option<char>**: Use for peek to handle end-of-input
4. **Error propagation**: Use `?` operator
5. **Newline handling**: Track line and column correctly
6. **Negative numbers**: Same logic as Python

#### Edge Cases

1. **Negative numbers**: `-5` → single NUMBER token
2. **Standalone minus**: `5 - 3` → MINUS token
3. **Floats**: `3.14` → single NUMBER token
4. **Whitespace**: Ignored but tracks line/column
5. **Invalid char**: `^` → LexerError immediately

#### Validation

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic_addition() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, PLUS, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
    }

    #[test]
    fn test_tokenize_negative_number() {
        let mut lexer = Lexer::new("-5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // NUMBER(-5), EOF
        assert_eq!(tokens[0].value, "-5");
    }

    #[test]
    fn test_tokenize_float() {
        let mut lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_tokenize_invalid_char() {
        let mut lexer = Lexer::new("2 3 ^");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Unexpected character '^'"));
        assert_eq!(err.line, 1);
        assert_eq!(err.column, 5);
    }
}
```

---

### Module 5: parser.py → parser.rs

#### Python Implementation Analysis

**Location**: `source/parser.py` (184 lines)

**Key Components**:
1. `ParserError` exception:
   - `message: str`
   - `token: Token`
2. `Parser` class:
   - `tokens: list[Token]`
   - `pos: int` - current position

**Methods**:
- `parse() -> Expr` - main entry point
- `_current() -> Token` - get current token
- `_at_end() -> bool` - check if at EOF
- `_advance() -> Token` - consume and return current token

**Algorithm** (`parse`):
```python
def parse():
    stack = []

    while not _at_end():
        token = _current()

        if token.type == NUMBER:
            # Push number onto stack
            num_node = Number(line=token.line, column=token.column, value=token.value)
            stack.append(num_node)
            _advance()

        elif token.type in (PLUS, MINUS, MULT, DIV):
            # Pop two operands and create binary operation
            if len(stack) < 2:
                raise ParserError(f"Operator '{token.value}' requires two operands", token)

            right = stack.pop()
            left = stack.pop()

            # Map token type to operator string
            op_map = {PLUS: "+", MINUS: "-", MULT: "*", DIV: "/"}
            operator = op_map[token.type]

            op_node = BinaryOp(
                line=token.line,
                column=token.column,
                operator=operator,
                left=left,
                right=right
            )
            stack.append(op_node)
            _advance()

        elif token.type == EOF:
            break

        else:
            raise ParserError(f"Unexpected token '{token.value}'", token)

    # Validate final state
    if len(stack) == 0:
        raise ParserError("Empty expression", eof_token)

    if len(stack) > 1:
        raise ParserError(
            f"Invalid RPN: {len(stack)} values remain on stack (missing operators?)",
            eof_token
        )

    return stack[0]
```

**Stack Evolution Example** (`5 3 + 2 *`):
```
Token | Stack State
------|------------
5     | [5]
3     | [5, 3]
+     | [5+3]
2     | [5+3, 2]
*     | [(5+3)*2]
EOF   | [(5+3)*2]  ← Return this
```

#### Rust Type Mappings

##### ParserError Type

```rust
use crate::tokens::Token;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub message: String,
    pub token: Token,
}

impl ParserError {
    pub fn new(message: impl Into<String>, token: Token) -> Self {
        Self {
            message: message.into(),
            token,
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at line {}, column {}",
            self.message, self.token.line, self.token.column
        )
    }
}

impl std::error::Error for ParserError {}
```

##### Parser Struct

```rust
use crate::ast::Expr;
use crate::tokens::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current().clone();

            match token.token_type {
                TokenType::Number => {
                    // Push number onto stack
                    let num_node = Expr::number(token.value.clone(), token.line, token.column);
                    stack.push(num_node);
                    self.advance();
                }
                TokenType::Plus | TokenType::Minus | TokenType::Mult | TokenType::Div => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            format!("Operator '{}' requires two operands", token.value),
                            token,
                        ));
                    }

                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    // Map token type to operator string
                    let operator = match token.token_type {
                        TokenType::Plus => "+",
                        TokenType::Minus => "-",
                        TokenType::Mult => "*",
                        TokenType::Div => "/",
                        _ => unreachable!(),
                    };

                    let op_node = Expr::binary_op(
                        operator,
                        left,
                        right,
                        token.line,
                        token.column,
                    );
                    stack.push(op_node);
                    self.advance();
                }
                TokenType::Eof => break,
            }
        }

        // Validate final state
        if stack.is_empty() {
            let eof_token = self.tokens.last().unwrap().clone();
            return Err(ParserError::new("Empty expression", eof_token));
        }

        if stack.len() > 1 {
            let eof_token = self.tokens.last().unwrap().clone();
            return Err(ParserError::new(
                format!(
                    "Invalid RPN: {} values remain on stack (missing operators?)",
                    stack.len()
                ),
                eof_token,
            ));
        }

        Ok(stack.into_iter().next().unwrap())
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn at_end(&self) -> bool {
        self.tokens[self.pos].token_type == TokenType::Eof
    }

    fn advance(&mut self) {
        if !self.at_end() {
            self.pos += 1;
        }
    }
}
```

#### Implementation Notes

1. **Vec<Expr> stack**: Use Vec as stack (push/pop)
2. **Clone tokens**: Clone tokens when needed to avoid borrowing issues
3. **Pattern matching**: Use match on TokenType
4. **Error handling**: Return `Result<Expr, ParserError>`
5. **Validation**: Check stack size at end
6. **unwrap safety**: Safe after length checks

#### Edge Cases

1. **Empty input**: Stack is empty → error
2. **Insufficient operands**: `5 +` → error
3. **Excess operands**: `5 3 2 +` → error
4. **Valid RPN**: `5 3 +` → success

#### Validation

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_addition() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_precedence() {
        let mut lexer = Lexer::new("5 3 + 2 *");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp { operator, left, .. } => {
                assert_eq!(operator, "*");
                match *left {
                    Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
                    _ => panic!("Expected nested BinaryOp"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_insufficient_operands() {
        let mut lexer = Lexer::new("5 +");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
    }
}
```

---

### Module 6: latex_gen.py → latex.rs

#### Python Implementation Analysis

**Location**: `source/latex_gen.py` (185 lines)

**Key Components**:
1. `LaTeXGenerator` class
2. Class constants:
   - `BINARY_OPS: dict[str, str]` - operator to LaTeX mapping
   - `PRECEDENCE: dict[str, int]` - operator precedence levels

**Constants**:
```python
BINARY_OPS = {
    "+": "+",
    "-": "-",
    "*": r"\times",
    "/": r"\div",
}

PRECEDENCE = {
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2,
}
```

**Methods**:
- `generate(ast: Expr) -> str` - main entry point
- `_visit(node: Expr) -> str` - visitor dispatcher (singledispatch)
- `_visit_number(node: Number) -> str` - handle Number nodes
- `_visit_binary_op(node: BinaryOp) -> str` - handle BinaryOp nodes
- `_needs_parens(child: Expr, parent_precedence: int, is_right: bool) -> bool` - precedence logic

**Algorithm** (`generate`):
```python
def generate(ast):
    content = _visit(ast)
    return f"${content}$"
```

**Algorithm** (`_visit_number`):
```python
def _visit_number(node: Number) -> str:
    return node.value
```

**Algorithm** (`_visit_binary_op`):
```python
def _visit_binary_op(node: BinaryOp) -> str:
    op_latex = BINARY_OPS[node.operator]
    my_precedence = PRECEDENCE[node.operator]

    # Generate left operand, adding parens if needed
    left = _visit(node.left)
    if _needs_parens(node.left, my_precedence, is_right=False):
        left = f"( {left} )"

    # Generate right operand, adding parens if needed
    right = _visit(node.right)
    if _needs_parens(node.right, my_precedence, is_right=True):
        right = f"( {right} )"

    return f"{left} {op_latex} {right}"
```

**Algorithm** (`_needs_parens`) - **CRITICAL**:
```python
def _needs_parens(child: Expr, parent_precedence: int, is_right: bool) -> bool:
    if not isinstance(child, BinaryOp):
        return False

    child_precedence = PRECEDENCE[child.operator]

    # Lower precedence always needs parens
    if child_precedence < parent_precedence:
        return True

    # Equal precedence on right side needs parens for non-commutative operators
    # (handles left-associativity of - and /)
    return (
        child_precedence == parent_precedence
        and is_right
        and child.operator in ("-", "/")
    )
```

**Precedence Rules**:
1. Lower precedence child → needs parens
2. Equal precedence + right side + non-commutative (- or /) → needs parens
3. All other cases → no parens

**Examples**:

| Input | Tree | Parens Decision | Output |
|-------|------|-----------------|--------|
| `5 3 + 2 *` | `*(+(5,3), 2)` | + has prec 1, * has prec 2, 1<2 → parens | `$( 5 + 3 ) \times 2$` |
| `5 3 * 2 +` | `+(*(5,3), 2)` | * has prec 2, + has prec 1, 2>1 → no parens | `$5 \times 3 + 2$` |
| `5 3 - 2 -` | `-(-(5,3), 2)` | Left child, no parens | `$5 - 3 - 2$` |
| `2 3 4 + *` | `*(2, +(3,4))` | Right child, + has prec 1, * has prec 2, 1<2 → parens | `$2 \times ( 3 + 4 )$` |

#### Rust Type Mappings

##### LaTeXGenerator Struct

```rust
use crate::ast::Expr;
use std::collections::HashMap;

pub struct LaTeXGenerator {
    binary_ops: HashMap<String, String>,
    precedence: HashMap<String, i32>,
}

impl LaTeXGenerator {
    pub fn new() -> Self {
        let mut binary_ops = HashMap::new();
        binary_ops.insert("+".to_string(), "+".to_string());
        binary_ops.insert("-".to_string(), "-".to_string());
        binary_ops.insert("*".to_string(), r"\times".to_string());
        binary_ops.insert("/".to_string(), r"\div".to_string());

        let mut precedence = HashMap::new();
        precedence.insert("+".to_string(), 1);
        precedence.insert("-".to_string(), 1);
        precedence.insert("*".to_string(), 2);
        precedence.insert("/".to_string(), 2);

        Self {
            binary_ops,
            precedence,
        }
    }

    pub fn generate(&self, ast: &Expr) -> String {
        let content = self.visit(ast);
        format!("${}$", content)
    }

    fn visit(&self, node: &Expr) -> String {
        match node {
            Expr::Number { value, .. } => self.visit_number(value),
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => self.visit_binary_op(operator, left, right),
        }
    }

    fn visit_number(&self, value: &str) -> String {
        value.to_string()
    }

    fn visit_binary_op(&self, operator: &str, left: &Expr, right: &Expr) -> String {
        let op_latex = self.binary_ops.get(operator).unwrap();
        let my_precedence = *self.precedence.get(operator).unwrap();

        // Generate left operand, adding parens if needed
        let mut left_str = self.visit(left);
        if self.needs_parens(left, my_precedence, false) {
            left_str = format!("( {} )", left_str);
        }

        // Generate right operand, adding parens if needed
        let mut right_str = self.visit(right);
        if self.needs_parens(right, my_precedence, true) {
            right_str = format!("( {} )", right_str);
        }

        format!("{} {} {}", left_str, op_latex, right_str)
    }

    fn needs_parens(&self, child: &Expr, parent_precedence: i32, is_right: bool) -> bool {
        match child {
            Expr::Number { .. } => false,
            Expr::BinaryOp { operator, .. } => {
                let child_precedence = *self.precedence.get(operator.as_str()).unwrap();

                // Lower precedence always needs parens
                if child_precedence < parent_precedence {
                    return true;
                }

                // Equal precedence on right side needs parens for non-commutative operators
                // (handles left-associativity of - and /)
                child_precedence == parent_precedence
                    && is_right
                    && (operator == "-" || operator == "/")
            }
        }
    }
}

impl Default for LaTeXGenerator {
    fn default() -> Self {
        Self::new()
    }
}
```

#### Implementation Notes

1. **HashMap for mappings**: Use HashMap for operator and precedence tables
2. **Match for dispatch**: Replace singledispatch with pattern matching
3. **String formatting**: Use `format!` macro
4. **Precedence logic**: Translate exactly from Python
5. **Spacing**: Match Python exactly: ` + `, ` \times `, `( expr )`

#### Critical Parenthesization Logic

**Decision Tree**:
```
Is child a Number?
  YES → No parens
  NO  → Is child a BinaryOp?
    Get child_precedence
    Is child_precedence < parent_precedence?
      YES → Parens needed
      NO  → Is child_precedence == parent_precedence?
        AND is_right == true?
          AND child.operator in ["-", "/"]?
            YES → Parens needed
            NO  → No parens
```

#### Validation

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn parse_and_generate(input: &str) -> String {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        generator.generate(&ast)
    }

    #[test]
    fn test_simple_addition() {
        assert_eq!(parse_and_generate("5 3 +"), "$5 + 3$");
    }

    #[test]
    fn test_multiplication_symbol() {
        assert_eq!(parse_and_generate("4 7 *"), r"$4 \times 7$");
    }

    #[test]
    fn test_division_symbol() {
        assert_eq!(parse_and_generate("10 2 /"), r"$10 \div 2$");
    }

    #[test]
    fn test_precedence_parens_needed() {
        assert_eq!(parse_and_generate("5 3 + 2 *"), r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_precedence_no_parens() {
        assert_eq!(parse_and_generate("5 3 * 2 +"), r"$5 \times 3 + 2$");
    }

    #[test]
    fn test_left_associativity() {
        assert_eq!(parse_and_generate("5 3 - 2 -"), "$5 - 3 - 2$");
    }

    #[test]
    fn test_right_operand_parens() {
        assert_eq!(parse_and_generate("2 3 4 + *"), r"$2 \times ( 3 + 4 )$");
    }

    #[test]
    fn test_float() {
        assert_eq!(parse_and_generate("3.14 2 *"), r"$3.14 \times 2$");
    }

    #[test]
    fn test_complex_expression() {
        assert_eq!(
            parse_and_generate("1 2 + 3 4 + *"),
            r"$( 1 + 2 ) \times ( 3 + 4 )$"
        );
    }
}
```

---

## Phase 3: CLI

### Module 7: cli.py → main.rs

#### Python Implementation Analysis

**Location**: `source/cli.py` (115 lines)

**Key Components**:
1. `main() -> int` function
2. Command-line argument parsing with argparse
3. Pipeline orchestration: read → lex → parse → generate → write
4. Error handling with exit codes

**Algorithm** (`main`):
```python
def main():
    # Parse arguments
    parser = argparse.ArgumentParser(...)
    parser.add_argument("input", help="Input RPN file (use '-' for stdin)")
    parser.add_argument("-o", "--output", help="Output LaTeX file")
    args = parser.parse_args()

    # Read input
    if args.input == "-":
        text = sys.stdin.read()
    else:
        text = Path(args.input).read_text()

    # Process: tokenize → parse → generate
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

    # Write output
    if args.output:
        Path(args.output).write_text(latex + "\n")
        print(f"Generated: {args.output}", file=sys.stderr)
    else:
        print(latex)

    return 0
```

**Exit Codes**:
- 0: Success
- 1: Error (lexer, parser, or I/O)

**I/O Behavior**:
- Input from stdin: `-` argument
- Output to stdout: default (no `-o`)
- Output to file: with `-o` flag (adds newline)

#### Rust Implementation

##### Cargo.toml Setup

```toml
[package]
name = "rpn2tex"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.4", features = ["derive"] }

[[bin]]
name = "rpn2tex"
path = "src/main.rs"
```

##### lib.rs

```rust
pub mod tokens;
pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod latex;

pub use tokens::{Token, TokenType};
pub use ast::Expr;
pub use error::ErrorFormatter;
pub use lexer::{Lexer, LexerError};
pub use parser::{Parser, ParserError};
pub use latex::LaTeXGenerator;
```

##### main.rs

```rust
use clap::Parser as ClapParser;
use rpn2tex::{ErrorFormatter, LaTeXGenerator, Lexer, Parser};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;

#[derive(ClapParser, Debug)]
#[command(name = "rpn2tex")]
#[command(about = "Convert RPN expressions to LaTeX math mode")]
#[command(long_about = "Example: rpn2tex input.rpn -o output.tex")]
struct Args {
    /// Input RPN file (use '-' for stdin)
    #[arg(value_name = "INPUT")]
    input: String,

    /// Output LaTeX file (default: stdout)
    #[arg(short = 'o', long = "output", value_name = "OUTPUT")]
    output: Option<PathBuf>,
}

fn main() {
    let exit_code = run();
    process::exit(exit_code);
}

fn run() -> i32 {
    let args = Args::parse();

    // Read input
    let text = match read_input(&args.input) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error: {}", e);
            return 1;
        }
    };

    // Process: tokenize → parse → generate
    let formatter = ErrorFormatter::new(&text);

    let latex = match process_text(&text) {
        Ok(latex) => latex,
        Err(e) => {
            let formatted = format_error(&formatter, &e);
            eprintln!("{}", formatted);
            return 1;
        }
    };

    // Write output
    if let Err(e) = write_output(&args.output, &latex) {
        eprintln!("Error: {}", e);
        return 1;
    }

    0
}

fn read_input(input: &str) -> io::Result<String> {
    if input == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        fs::read_to_string(input).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("Failed to read '{}': {}", input, e),
            )
        })
    }
}

fn process_text(text: &str) -> Result<String, ProcessError> {
    // Tokenize
    let mut lexer = Lexer::new(text);
    let tokens = lexer.tokenize().map_err(ProcessError::Lexer)?;

    // Parse
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(ProcessError::Parser)?;

    // Generate LaTeX
    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}

fn write_output(output: &Option<PathBuf>, latex: &str) -> io::Result<()> {
    match output {
        Some(path) => {
            fs::write(path, format!("{}\n", latex))?;
            eprintln!("Generated: {}", path.display());
            Ok(())
        }
        None => {
            println!("{}", latex);
            Ok(())
        }
    }
}

fn format_error(formatter: &ErrorFormatter, error: &ProcessError) -> String {
    match error {
        ProcessError::Lexer(e) => {
            formatter.format_error(&e.message, e.line, e.column)
        }
        ProcessError::Parser(e) => {
            formatter.format_error(&e.message, e.token.line, e.token.column)
        }
    }
}

#[derive(Debug)]
enum ProcessError {
    Lexer(rpn2tex::LexerError),
    Parser(rpn2tex::ParserError),
}
```

#### Implementation Notes

1. **clap for argument parsing**: Use derive API for ergonomics
2. **Exit code handling**: Use `process::exit()`
3. **Error enum**: Combine LexerError and ParserError
4. **Stdin handling**: Use `io::stdin().read_to_string()`
5. **File I/O**: Use `fs::read_to_string()` and `fs::write()`
6. **Newline behavior**: Add `\n` only when writing to file

#### Validation

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_simple_expression() {
        let result = process_text("5 3 +").unwrap();
        assert_eq!(result, "$5 + 3$");
    }

    #[test]
    fn test_process_lexer_error() {
        let result = process_text("5 3 ^");
        assert!(result.is_err());
        match result.unwrap_err() {
            ProcessError::Lexer(_) => {},
            _ => panic!("Expected LexerError"),
        }
    }

    #[test]
    fn test_process_parser_error() {
        let result = process_text("5 +");
        assert!(result.is_err());
        match result.unwrap_err() {
            ProcessError::Parser(_) => {},
            _ => panic!("Expected ParserError"),
        }
    }
}
```

---

## Testing Strategy

### Unit Tests

Each module should have comprehensive unit tests covering:
1. Happy path functionality
2. Edge cases
3. Error conditions
4. Boundary conditions

### Integration Tests

Create `tests/integration_test.rs`:

```rust
use rpn2tex::{ErrorFormatter, LaTeXGenerator, Lexer, Parser};

fn process(input: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| format!("Parser error: {}", e))?;

    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}

#[test]
fn test_io_contract_case_1() {
    assert_eq!(process("5 3 +").unwrap(), "$5 + 3$");
}

#[test]
fn test_io_contract_case_2() {
    assert_eq!(process("5 3 -").unwrap(), "$5 - 3$");
}

#[test]
fn test_io_contract_case_3() {
    assert_eq!(process("4 7 *").unwrap(), r"$4 \times 7$");
}

#[test]
fn test_io_contract_case_4() {
    assert_eq!(process("10 2 /").unwrap(), r"$10 \div 2$");
}

#[test]
fn test_io_contract_case_5() {
    assert_eq!(process("5 3 + 2 *").unwrap(), r"$( 5 + 3 ) \times 2$");
}

#[test]
fn test_io_contract_case_6() {
    assert_eq!(process("5 3 * 2 +").unwrap(), r"$5 \times 3 + 2$");
}

#[test]
fn test_io_contract_case_7() {
    assert_eq!(process("10 2 / 5 *").unwrap(), r"$10 \div 2 \times 5$");
}

#[test]
fn test_io_contract_case_8() {
    assert_eq!(process("5 3 - 2 -").unwrap(), "$5 - 3 - 2$");
}

#[test]
fn test_io_contract_case_9() {
    assert_eq!(process("100 10 / 5 / 2 /").unwrap(), r"$100 \div 10 \div 5 \div 2$");
}

#[test]
fn test_io_contract_case_10() {
    assert_eq!(process("1 2 + 3 + 4 +").unwrap(), "$1 + 2 + 3 + 4$");
}

#[test]
fn test_io_contract_case_11() {
    assert_eq!(process("2 3 4 * +").unwrap(), r"$2 + 3 \times 4$");
}

#[test]
fn test_io_contract_case_12() {
    assert_eq!(process("2 3 + 4 *").unwrap(), r"$( 2 + 3 ) \times 4$");
}

#[test]
fn test_io_contract_case_13() {
    assert_eq!(process("2 3 4 + *").unwrap(), r"$2 \times ( 3 + 4 )$");
}

#[test]
fn test_io_contract_case_14() {
    assert_eq!(process("2 3 * 4 +").unwrap(), r"$2 \times 3 + 4$");
}

#[test]
fn test_io_contract_case_15() {
    assert_eq!(process("3.14 2 *").unwrap(), r"$3.14 \times 2$");
}

#[test]
fn test_io_contract_case_16() {
    assert_eq!(process("1.5 0.5 +").unwrap(), "$1.5 + 0.5$");
}

#[test]
fn test_io_contract_case_17() {
    assert_eq!(process("1 2 + 3 4 + *").unwrap(), r"$( 1 + 2 ) \times ( 3 + 4 )$");
}

#[test]
fn test_io_contract_case_18() {
    assert_eq!(process("10 2 / 3 + 4 *").unwrap(), r"$( 10 \div 2 + 3 ) \times 4$");
}

#[test]
fn test_io_contract_error_case_1() {
    let result = process("2 3 ^");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unexpected character '^'"));
}

#[test]
fn test_io_contract_error_case_2() {
    let result = process("2 3 ^ 4 *");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unexpected character '^'"));
}

#[test]
fn test_io_contract_error_case_3() {
    let result = process("2 3 4 ^ ^");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unexpected character '^'"));
}
```

### CLI Tests

Create shell scripts or use Rust's `Command` to test CLI:

```rust
#[test]
fn test_cli_stdin() {
    use std::process::{Command, Stdio};
    use std::io::Write;

    let mut child = Command::new("target/debug/rpn2tex")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"5 3 +").unwrap();
    drop(stdin);

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "$5 + 3$");
}
```

---

## Quality Gates

After migrating each module, run these checks:

### 1. Compilation Check
```bash
cargo check
```

### 2. Linting
```bash
cargo clippy -- -D warnings
```

### 3. Formatting
```bash
cargo fmt --check
```

### 4. Unit Tests
```bash
cargo test --lib
```

### 5. Integration Tests
```bash
cargo test --test integration_test
```

### 6. I/O Contract Validation

Run all 21 test cases from the I/O contract:

```bash
# Success cases
echo "5 3 +" | cargo run -- - | diff - <(echo '$5 + 3$')
echo "4 7 *" | cargo run -- - | diff - <(echo '$4 \times 7$')
# ... all 18 success cases

# Error cases
echo "2 3 ^" | cargo run -- - 2>&1 | grep "Unexpected character '^'"
# ... all 3 error cases
```

### 7. Full Build
```bash
cargo build --release
```

---

## Migration Checklist

### Phase 0: I/O Contract ✓
- [x] Generate I/O contract from Python implementation
- [x] Document all 21 test cases with expected outputs

### Phase 1: Core Data Types
- [ ] Migrate tokens.py → tokens.rs
  - [ ] TokenType enum
  - [ ] Token struct
  - [ ] Unit tests
  - [ ] Pass quality gates
- [ ] Migrate ast_nodes.py → ast.rs
  - [ ] Expr enum with Number and BinaryOp variants
  - [ ] Helper methods
  - [ ] Unit tests
  - [ ] Pass quality gates
- [ ] Migrate errors.py → error.rs
  - [ ] ErrorFormatter struct
  - [ ] format_error method
  - [ ] Unit tests
  - [ ] Pass quality gates

### Phase 2: Processing Pipeline
- [ ] Migrate lexer.py → lexer.rs
  - [ ] Lexer struct
  - [ ] LexerError type
  - [ ] tokenize method
  - [ ] Unit tests
  - [ ] Pass quality gates
- [ ] Migrate parser.py → parser.rs
  - [ ] Parser struct
  - [ ] ParserError type
  - [ ] parse method
  - [ ] Unit tests
  - [ ] Pass quality gates
- [ ] Migrate latex_gen.py → latex.rs
  - [ ] LaTeXGenerator struct
  - [ ] Precedence logic (CRITICAL)
  - [ ] needs_parens method
  - [ ] Unit tests
  - [ ] Pass quality gates

### Phase 3: CLI
- [ ] Migrate cli.py → main.rs
  - [ ] CLI argument parsing
  - [ ] Pipeline orchestration
  - [ ] Error handling
  - [ ] Integration tests
  - [ ] Pass quality gates

### Phase 4: Final Validation
- [ ] All I/O contract tests pass (21/21)
- [ ] cargo clippy passes with no warnings
- [ ] cargo fmt check passes
- [ ] cargo test passes (all tests)
- [ ] cargo build --release succeeds
- [ ] Manual testing of CLI

---

## Appendix: Common Rust Patterns

### Error Handling

```rust
// Define custom errors
#[derive(Debug)]
pub struct MyError {
    message: String,
}

impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Use Result<T, E>
fn my_function() -> Result<Value, MyError> {
    // ...
}

// Propagate errors with ?
let result = some_function()?;

// Match on Result
match my_function() {
    Ok(value) => { /* handle success */ }
    Err(e) => { /* handle error */ }
}
```

### Ownership and Borrowing

```rust
// Immutable borrow
fn read_value(x: &String) {
    println!("{}", x);
}

// Mutable borrow
fn modify_value(x: &mut String) {
    x.push_str("!");
}

// Take ownership
fn consume_value(x: String) {
    // x is moved here
}

// Clone to avoid move
let x = String::from("hello");
let y = x.clone(); // x is still valid
```

### Pattern Matching

```rust
// Match on enum
match expr {
    Expr::Number { value, .. } => { /* handle number */ }
    Expr::BinaryOp { operator, left, right, .. } => { /* handle binary op */ }
}

// Match with guards
match token.token_type {
    TokenType::Number => { /* handle number */ }
    TokenType::Plus | TokenType::Minus => { /* handle + or - */ }
    _ => { /* handle everything else */ }
}
```

### Collections

```rust
// Vec (growable array)
let mut vec = Vec::new();
vec.push(value);
let last = vec.pop();

// HashMap
let mut map = HashMap::new();
map.insert(key, value);
let val = map.get(&key);
```

---

## End of Migration Specification

This specification provides all the information needed to migrate rpn2tex from Python to Rust while ensuring behavioral equivalence through I/O contract validation.
