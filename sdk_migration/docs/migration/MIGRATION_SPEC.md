# rpn2tex Comprehensive Migration Specification

**Version:** 1.0
**Date:** 2025-12-27
**Source Language:** Python 3.10+
**Target Language:** Rust

---

## Table of Contents

1. [I/O Contract (Phase 0)](#io-contract-phase-0)
2. [Architecture Overview](#architecture-overview)
3. [Module Dependencies](#module-dependencies)
4. [Module Specifications](#module-specifications)
5. [Migration Order](#migration-order)
6. [Critical Behaviors](#critical-behaviors)
7. [Test Strategy](#test-strategy)

---

## I/O Contract (Phase 0)

### Complete I/O Contract Reference

**Location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/IO_CONTRACT.md`

**Summary:** The Rust implementation MUST produce identical outputs to the Python implementation for all 21 test cases.

### Test Cases Quick Reference

| # | Input | Expected Output | Category |
|---|-------|-----------------|----------|
| 1 | `5 3 +` | `$5 + 3$` | Basic ops |
| 2 | `5 3 -` | `$5 - 3$` | Basic ops |
| 3 | `4 7 *` | `$4 \times 7$` | Basic ops |
| 4 | `10 2 /` | `$10 \div 2$` | Basic ops |
| 5 | `2 3 ^` | LexerError | Error case |
| 6 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Precedence |
| 7 | `5 3 * 2 +` | `$5 \times 3 + 2$` | Precedence |
| 8 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | Associativity |
| 9 | `5 3 - 2 -` | `$5 - 3 - 2$` | Associativity |
| 10 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Associativity |
| 11 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Addition chain |
| 12 | `2 3 4 * +` | `$2 + 3 \times 4$` | Precedence |
| 13 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Precedence |
| 14 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Precedence |
| 15 | `2 3 * 4 +` | `$2 \times 3 + 4$` | Precedence |
| 16 | `2 3 ^ 4 *` | LexerError | Error case |
| 17 | `2 3 4 ^ ^` | LexerError | Error case |
| 18 | `3.14 2 *` | `$3.14 \times 2$` | Floating-point |
| 19 | `1.5 0.5 +` | `$1.5 + 0.5$` | Floating-point |
| 20 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Complex |
| 21 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex |

### Critical Output Format Rules

1. **LaTeX Wrapping:** All output wrapped in `$...$`
2. **Operator Mappings:**
   - `+` → `+`
   - `-` → `-`
   - `*` → `\times`
   - `/` → `\div`
3. **Spacing:** Spaces around all operators: `5 + 3` not `5+3`
4. **Parentheses:** Format is `( expr )` with spaces inside
5. **No Trailing Newline:** Output to stdout has NO trailing newline

### Precedence and Parenthesization Rules

**Precedence Levels:**
- Level 1 (low): `+`, `-`
- Level 2 (high): `*`, `/`

**Parenthesization Rules:**
1. Lower precedence child always needs parens
2. Equal precedence on RIGHT side of `-` or `/` needs parens (left-associativity)
3. Otherwise, no parentheses

---

## Architecture Overview

### System Pipeline

```
Input Text
    ↓
 [LEXER] → Tokens (with EOF)
    ↓
 [PARSER] → AST (stack-based RPN)
    ↓
[GENERATOR] → LaTeX String
    ↓
Output Text
```

### Data Flow

1. **Lexer:** Character stream → Token stream
   - Whitespace skipping
   - Number aggregation
   - Operator recognition
   - Position tracking (line/column)

2. **Parser:** Token stream → AST
   - Stack-based RPN algorithm
   - Number → push
   - Operator → pop 2, create BinaryOp, push
   - Validate: exactly 1 value on stack at end

3. **Generator:** AST → LaTeX
   - Visitor pattern
   - Precedence-driven parenthesization
   - Operator mapping

4. **CLI:** Orchestration
   - Read input (file or stdin)
   - Call pipeline
   - Error handling and formatting
   - Write output (file or stdout)

---

## Module Dependencies

```
main.rs (CLI)
 ├── lexer.rs
 │   └── tokens.rs
 ├── parser.rs
 │   ├── tokens.rs
 │   └── ast.rs
 ├── latex.rs
 │   └── ast.rs
 └── error.rs
```

**Dependency Order for Migration:**
1. `tokens.rs` (no dependencies)
2. `ast.rs` (no dependencies)
3. `error.rs` (no dependencies)
4. `lexer.rs` (depends on tokens)
5. `parser.rs` (depends on tokens, ast)
6. `latex.rs` (depends on ast)
7. `main.rs` (depends on all)

---

## Module Specifications

### Module 1: tokens.rs

**Source:** `/Users/jfreeman/Coding/rpn2tex/src/rpn2tex/tokens.py`
**Target:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/rpn2tex-rs/src/tokens.rs`

#### Purpose
Defines token types and token representation for the lexer.

#### Public API

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Number,
    Plus,
    Minus,
    Mult,
    Div,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub type_: TokenType,
    pub value: String,
    pub line: u32,
    pub column: u32,
}
```

#### Key Implementation Details

- TokenType: 6 variants for NUMBER, PLUS, MINUS, MULT, DIV, EOF
- Token: Immutable struct with position tracking
- Field `type_` (not `type` which is Rust keyword)
- Line/column are 1-based u32 values
- Implement Display for debugging

---

### Module 2: ast.rs

**Source:** `/Users/jfreeman/Coding/rpn2tex/src/rpn2tex/ast_nodes.py`
**Target:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/rpn2tex-rs/src/ast.rs`

#### Purpose
Defines AST node types for representing parsed expressions.

#### Public API

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number {
        line: u32,
        column: u32,
        value: String,
    },
    BinaryOp {
        line: u32,
        column: u32,
        operator: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn line(&self) -> u32 { ... }
    pub fn column(&self) -> u32 { ... }
}
```

#### Key Implementation Details

- Expr enum with Number and BinaryOp variants
- Use `Box<Expr>` for recursive types
- All nodes are immutable by default in Rust
- Position tracking (line/column) preserved
- Number values are strings (not parsed to numeric types)
- Operator is string: "+", "-", "*", "/"

---

### Module 3: error.rs

**Source:** `/Users/jfreeman/Coding/rpn2tex/src/rpn2tex/errors.py`
**Target:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/rpn2tex-rs/src/error.rs`

#### Purpose
Formats parse and lexer errors with source context.

#### Public API

```rust
pub struct ErrorFormatter {
    source: String,
    lines: Vec<String>,
}

impl ErrorFormatter {
    pub fn new(source: String) -> Self { ... }

    pub fn format_error(
        &self,
        message: &str,
        line: u32,
        column: u32,
    ) -> String { ... }

    fn get_context(&self, line: u32, column: u32, context_lines: u32) -> String { ... }
}
```

#### Key Implementation Details

- Stores source text split into lines
- Formats errors with line numbers and caret
- Output format:
  ```
  Error: <message>

  <line_num> | <source_line>
             | <spaces>^
  ```
- Context lines parameter defaults to 1
- Uses 1-based line/column numbers

---

### Module 4: lexer.rs

**Source:** `/Users/jfreeman/Coding/rpn2tex/src/rpn2tex/lexer.py`
**Target:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/rpn2tex-rs/src/lexer.rs`

#### Purpose
Tokenizes RPN expressions into token stream.

#### Public API

```rust
#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub line: u32,
    pub column: u32,
}

impl std::error::Error for LexerError {}

pub struct Lexer {
    text: String,
    pos: usize,
    line: u32,
    column: u32,
}

impl Lexer {
    pub fn new(text: String) -> Self { ... }
    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> { ... }
}
```

#### Key Implementation Details

- Character-by-character scanning
- Lookahead for negative numbers: `-` followed by digit
- Whitespace: space, tab, newline, CR
- Position tracking: line/column for error reporting
- Number scanning: integers and decimals (e.g., "3.14")
- Operators: single-character tokens
- EOF token at end
- Raises LexerError for unknown characters (e.g., `^`)

#### Algorithm

```
tokenize():
  tokens = []
  while not at_end:
    skip_whitespace()
    if at_end: break
    token = scan_token()
    tokens.push(token)
  tokens.push(EOF)
  return tokens

scan_token():
  match peek():
    '+' -> advance, return PLUS
    '-' -> advance, lookahead for digit:
             if digit: scan_number("-")
             else: return MINUS
    '*' -> advance, return MULT
    '/' -> advance, return DIV
    digit -> scan_number("")
    _ -> error "Unexpected character"

scan_number(prefix):
  value = prefix
  while digit: value += advance()
  if '.':
    value += advance()
    while digit: value += advance()
  return NUMBER token
```

---

### Module 5: parser.rs

**Source:** `/Users/jfreeman/Coding/rpn2tex/src/rpn2tex/parser.py`
**Target:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/rpn2tex-rs/src/parser.rs`

#### Purpose
Parses token stream into AST using stack-based RPN algorithm.

#### Public API

```rust
#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String,
    pub token: Token,
}

impl std::error::Error for ParserError {}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { ... }
    pub fn parse(mut self) -> Result<Expr, ParserError> { ... }
}
```

#### Key Implementation Details

- Stack-based RPN parsing (not recursive descent)
- Algorithm:
  1. Initialize empty stack
  2. For each token:
     - NUMBER: create Number node, push
     - OPERATOR: pop right, pop left, create BinaryOp, push
     - EOF: break
  3. Validate: stack must have exactly 1 element
- Error cases:
  - Empty expression
  - Not enough operands for operator
  - Too many operands (stack > 1 at end)

#### Algorithm

```
parse():
  stack = []
  while not at_end:
    token = current()
    match token.type:
      NUMBER:
        node = Number{value: token.value, ...}
        stack.push(node)
        advance()
      PLUS | MINUS | MULT | DIV:
        if stack.len() < 2: error "not enough operands"
        right = stack.pop()
        left = stack.pop()
        node = BinaryOp{operator, left, right, ...}
        stack.push(node)
        advance()
      EOF: break
      _ : error "unexpected token"

  match stack.len():
    0 -> error "empty expression"
    1 -> return stack[0]
    n -> error "N values remain on stack"
```

---

### Module 6: latex.rs

**Source:** `/Users/jfreeman/Coding/rpn2tex/src/rpn2tex/latex_gen.py`
**Target:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/rpn2tex-rs/src/latex.rs`

#### Purpose
Generates LaTeX output from AST with proper precedence and parenthesization.

#### Public API

```rust
pub struct LaTeXGenerator;

impl LaTeXGenerator {
    pub fn generate(&self, ast: &Expr) -> String { ... }
}
```

#### Key Implementation Details

**Operator Mappings:**
- `"+"` → `"+"`
- `"-"` → `"-"`
- `"*"` → `r"\times"`
- `"/"` → `r"\div"`

**Precedence Levels:**
- `"+"`: 1
- `"-"`: 1
- `"*"`: 2
- `"/"`: 2

**Parenthesization Logic:**
```rust
fn needs_parens(child: &Expr, parent_precedence: u32, is_right: bool) -> bool {
    match child {
        Expr::BinaryOp { operator, .. } => {
            let child_prec = precedence(operator);

            // Lower precedence always needs parens
            if child_prec < parent_precedence {
                return true;
            }

            // Equal precedence on right of - or / needs parens
            if child_prec == parent_precedence && is_right {
                return matches!(operator.as_str(), "-" | "/");
            }

            false
        }
        _ => false,
    }
}
```

#### Algorithm

```
generate(ast):
  content = visit(ast)
  return "$" + content + "$"

visit(node):
  match node:
    Number{value} -> value
    BinaryOp{operator, left, right} ->
      my_prec = precedence(operator)
      latex_op = operator_mapping(operator)

      left_str = visit(left)
      if needs_parens(left, my_prec, false):
        left_str = "( " + left_str + " )"

      right_str = visit(right)
      if needs_parens(right, my_prec, true):
        right_str = "( " + right_str + " )"

      return left_str + " " + latex_op + " " + right_str
```

---

### Module 7: main.rs

**Source:** `/Users/jfreeman/Coding/rpn2tex/src/rpn2tex/cli.py`
**Target:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/rpn2tex-rs/src/main.rs`

#### Purpose
CLI entry point that orchestrates the complete pipeline.

#### Public API

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Input RPN file (use '-' for stdin)
    input: String,

    /// Output LaTeX file (default: stdout)
    #[clap(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    std::process::exit(run());
}

fn run() -> i32 { ... }
```

#### Key Implementation Details

- Use `clap` crate for argument parsing
- Read input: file or stdin (if input == "-")
- Pipeline: lexer → parser → generator
- Error handling:
  - Format errors with ErrorFormatter
  - Print to stderr
  - Return exit code 0 for parse errors
  - Return exit code 1 for I/O errors
- Write output: file or stdout
- Success message to stderr (if file output)

#### Algorithm

```
run():
  args = parse_args()

  // Read input
  text = if args.input == "-":
    read_stdin()
  else:
    read_file(args.input)

  // Handle I/O errors
  if error: print to stderr, return 1

  // Process pipeline
  formatter = ErrorFormatter::new(text)

  result = Lexer::new(text)
    .tokenize()
    .and_then(|tokens| Parser::new(tokens).parse())
    .map(|ast| LaTeXGenerator.generate(&ast))

  match result:
    Ok(latex) ->
      // Write output
      if args.output:
        write_file(output, latex + "\n")
        eprintln("Generated: {}", output)
      else:
        print!("{}", latex)  // No newline!
      return 0

    Err(error) ->
      eprintln(formatter.format_error(...))
      return 0  // Note: parse errors return 0!
```

---

## Migration Order

### Phase 1: Core Foundation (Parallel)

Migrate these modules in any order (no dependencies):

1. **tokens.rs** - Token and TokenType definitions
2. **ast.rs** - Expr enum with Number and BinaryOp
3. **error.rs** - ErrorFormatter struct

**Quality Gate:**
- `cargo check` passes
- `cargo clippy -- -D warnings` passes
- `cargo fmt` applied
- Unit tests pass

### Phase 2: Pipeline Components (Sequential)

Migrate in this order (respects dependencies):

4. **lexer.rs** - Depends on tokens
5. **parser.rs** - Depends on tokens, ast
6. **latex.rs** - Depends on ast

**Quality Gate (after each module):**
- `cargo check` passes
- `cargo clippy -- -D warnings` passes
- `cargo fmt` applied
- Unit tests pass
- Integration tests for completed pipeline stages

### Phase 3: CLI Integration

7. **main.rs** - Depends on all other modules
8. **lib.rs** - Public API exports

**Quality Gate:**
- All cargo checks pass
- Full I/O contract validation (all 21 tests)
- End-to-end tests with files and stdin/stdout

---

## Critical Behaviors

### 1. Operator Precedence and Parenthesization

**MOST CRITICAL:** This is the most complex logic.

- Precedence: +/- (1) < */ (2)
- Lower precedence child always needs parens
- Equal precedence on right of `-` or `/` needs parens
- Test cases 6-15 validate this extensively

### 2. LaTeX Output Format

- Spaces around all operators: `5 + 3`
- Spaces inside parentheses: `( 5 + 3 )`
- Operator mappings: `*` → `\times`, `/` → `\div`
- Wrapping: `$...$`
- NO trailing newline on stdout

### 3. Position Tracking

- 1-based line and column numbers
- Newline increments line, resets column to 1
- Other characters increment column
- Critical for error messages

### 4. Number Representation

- Keep as strings: "42", "3.14"
- No parsing to numeric types
- No rounding or formatting

### 5. RPN Stack Algorithm

- Push numbers, pop for operators
- Exactly 1 value on stack at end
- Clear error messages for invalid RPN

### 6. Error Handling

- LexerError for unknown characters
- ParserError for invalid RPN
- ErrorFormatter for context
- Exit code 0 for parse errors (!)
- Exit code 1 for I/O errors

---

## Test Strategy

### Unit Tests

Each module should have unit tests:

- **tokens.rs**: Token construction, Display
- **ast.rs**: Expr construction, accessor methods
- **error.rs**: Error formatting, caret positioning
- **lexer.rs**: Single operators, numbers, whitespace, errors
- **parser.rs**: Single number, binary ops, complex expressions, errors
- **latex.rs**: Number output, operator mapping, precedence, parentheses

### Integration Tests

Pipeline tests:

- Lexer → Parser → Generator
- Test with all 21 I/O contract inputs
- Verify exact output match
- Verify error messages

### I/O Contract Validation

Create test file with all 21 test cases:

```rust
#[test]
fn test_io_contract() {
    let test_cases = [
        ("5 3 +", Ok("$5 + 3$")),
        ("5 3 -", Ok("$5 - 3$")),
        // ... all 21 cases
    ];

    for (input, expected) in test_cases {
        let result = process(input);
        assert_eq!(result, expected);
    }
}
```

### Regression Testing

Run after each module migration to ensure nothing breaks.

---

## Summary

This specification provides complete guidance for migrating rpn2tex from Python to Rust:

1. **I/O Contract**: 21 test cases define exact behavioral requirements
2. **Architecture**: Linear pipeline with clear data flow
3. **Module Specs**: Detailed per-module implementation guidance
4. **Migration Order**: Dependency-respecting sequence
5. **Critical Behaviors**: Key complexity areas requiring careful attention
6. **Testing**: Comprehensive strategy for validation

All migrator agents should refer to this specification as the single source of truth. No need to consult Python source files directly.
