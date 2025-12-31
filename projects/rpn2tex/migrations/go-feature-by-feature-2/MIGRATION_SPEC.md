# rpn2tex Python-to-Go Migration Specification
## Feature-by-Feature Analysis

**Document Version**: 1.0
**Date**: 2025-12-30
**Target Language**: Go
**Architecture Style**: Feature-Based Migration

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Feature Specifications](#feature-specifications)
   - [Feature 1: Numbers](#feature-1-numbers)
   - [Feature 2: Addition](#feature-2-addition)
   - [Feature 3: Subtraction](#feature-3-subtraction)
   - [Feature 4: Multiplication](#feature-4-multiplication)
   - [Feature 5: Division](#feature-5-division)
   - [Feature 6: Precedence](#feature-6-precedence)
3. [Cross-Cutting Concerns](#cross-cutting-concerns)
4. [I/O Contract](#io-contract)
5. [Implementation Roadmap](#implementation-roadmap)

---

## Architecture Overview

The rpn2tex system implements a classic three-stage compiler pipeline for Reverse Polish Notation (RPN) expressions:

```
Input Text
    ↓
Lexer (tokenization)
    ↓
Parser (AST construction)
    ↓
LaTeX Generator (output)
    ↓
Output LaTeX
```

### Core Modules

| Module | Purpose | Key Types |
|--------|---------|-----------|
| **tokens.py** | Token definitions | `TokenType`, `Token` |
| **ast_nodes.py** | AST node types | `ASTNode`, `Number`, `BinaryOp`, `Expr` |
| **lexer.py** | Tokenization | `Lexer`, `LexerError` |
| **parser.py** | RPN parsing | `Parser`, `ParserError` |
| **latex_gen.py** | Output generation | `LaTeXGenerator` |
| **errors.py** | Error formatting | `ErrorFormatter` |
| **cli.py** | Command-line interface | `main()` function |

### Data Flow

1. **Input** → Text string
2. **Lexer** → Token stream (with position info)
3. **Parser** → AST (with position info)
4. **Generator** → LaTeX string
5. **CLI** → File/stdout output

---

## Feature Specifications

## Feature 1: Numbers

### 1.1 Feature Boundary

The Numbers feature enables parsing and outputting numeric literals (both integers and floating-point decimals).

**End-to-end behavior:**
- Input: `5` or `3.14`
- Output: `$5$` or `$3.14$`

### 1.2 Cross-Module Components

| Module | Component | Role |
|--------|-----------|------|
| **tokens.py** | `TokenType.NUMBER` | Token type enum variant |
| **tokens.py** | `Token` dataclass | Stores token (type, value, line, column) |
| **lexer.py** | `Lexer._scan_number()` | Scans number characters |
| **lexer.py** | `Lexer._peek()`, `Lexer._advance()` | Character scanning primitives |
| **ast_nodes.py** | `Number` dataclass | AST node for numeric literals |
| **parser.py** | Number handling in `parse()` | Creates `Number` nodes from tokens |
| **latex_gen.py** | `_visit_number()` | Outputs number as-is in LaTeX |

### 1.3 Data Structures

#### Token Representation
```python
@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str              # "5" or "3.14"
    line: int               # 1-based
    column: int             # 1-based
```

#### AST Node Representation
```python
@dataclass(frozen=True)
class Number(ASTNode):
    value: str              # "5" or "3.14"
    line: int
    column: int
```

### 1.4 Algorithm Details

#### Lexing (lexer.py: lines 170-200)

```
SCAN_NUMBER(prefix, start_line, start_column):
  value := prefix

  # Scan integer part
  while not at_end() and peek().isdigit():
    value += advance()

  # Scan decimal part (optional)
  if not at_end() and peek() == '.':
    value += advance()  # consume '.'
    while not at_end() and peek().isdigit():
      value += advance()

  return Token(NUMBER, value, start_line, start_column)
```

**Special cases:**
- Negative numbers: `-` followed immediately by digit is treated as part of number
- Decimal point: Optional, must be followed by at least one digit

#### Parsing (parser.py: lines 107-113)

```
PARSE():
  stack := []

  while not at_end():
    token := current()

    if token.type == NUMBER:
      node := Number(line=token.line, column=token.column, value=token.value)
      stack.push(node)
      advance()
```

#### Code Generation (latex_gen.py: lines 99-109)

```
_VISIT_NUMBER(node: Number) -> str:
  return node.value  # Return numeric string as-is
```

### 1.5 Dependencies

- **None** - Numbers are a base feature with no dependencies on other operators

### 1.6 Test Cases

| Input | Expected Output | Category |
|-------|-----------------|----------|
| `5` | `$5$` | Basic integer |
| `3.14` | `$3.14$` | Floating-point |

---

## Feature 2: Addition

### 2.1 Feature Boundary

The Addition feature adds support for the `+` operator. In RPN, `a b +` means a + b.

**End-to-end behavior:**
- Input: `5 3 +`
- Output: `$5 + 3$`

### 2.2 Cross-Module Components

| Module | Component | Role |
|--------|-----------|------|
| **tokens.py** | `TokenType.PLUS` | Token type enum variant |
| **tokens.py** | `Token` dataclass | Stores "+" token |
| **lexer.py** | Char "+" → `TokenType.PLUS` (lines 150-152) | Recognizes + in input |
| **ast_nodes.py** | `BinaryOp` dataclass | Stores operator node |
| **parser.py** | Binary op handling (lines 115-147) | Creates `BinaryOp` nodes |
| **parser.py** | Op map: `PLUS: "+"` (line 132) | Maps token to operator string |
| **latex_gen.py** | `BINARY_OPS: "+" → "+"` | LaTeX output for + |
| **latex_gen.py** | `PRECEDENCE: "+" → 1` | Operator precedence |
| **latex_gen.py** | `_visit_binary_op()` | Generates LaTeX with parens as needed |

### 2.3 Data Structures

#### Token
```python
Token(TokenType.PLUS, "+", line, column)
```

#### AST Node
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str      # "+"
    left: Expr         # Left operand (Number or BinaryOp)
    right: Expr        # Right operand (Number or BinaryOp)
    line: int
    column: int
```

### 2.4 Algorithm Details

#### Lexing (lexer.py: lines 150-152)

```
if char == '+':
  advance()
  return Token(TokenType.PLUS, "+", start_line, start_column)
```

#### Parsing (parser.py: lines 115-147)

RPN parsing uses a stack. When an operator is encountered, it pops two operands and creates a BinaryOp node.

```
PARSE():
  stack := []

  while not at_end():
    token := current()

    if token.type == NUMBER:
      stack.push(Number(...))

    elif token.type in (PLUS, MINUS, MULT, DIV):
      if stack.len() < 2:
        raise ParserError("Operator requires two operands", token)

      right := stack.pop()
      left := stack.pop()
      operator := token_to_operator(token.type)  # "+", "-", "*", "/"

      node := BinaryOp(
        operator=operator,
        left=left,
        right=right,
        line=token.line,
        column=token.column
      )
      stack.push(node)
```

#### Code Generation (latex_gen.py: lines 111-141)

```
_VISIT_BINARY_OP(node: BinaryOp) -> str:
  op_latex := BINARY_OPS[node.operator]  # "+" stays "+"
  precedence := PRECEDENCE[node.operator]  # 1

  left := visit(node.left)
  if needs_parens(node.left, precedence, is_right=False):
    left := "( " + left + " )"

  right := visit(node.right)
  if needs_parens(node.right, precedence, is_right=True):
    right := "( " + right + " )"

  return left + " " + op_latex + " " + right
```

### 2.5 Dependencies

- **Depends on:** Numbers feature
- **Used by:** Precedence feature (to handle mixed operations)

### 2.6 Test Cases

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 +` | `$5 + 3$` | Basic addition |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Chained addition (left-associative) |

---

## Feature 3: Subtraction

### 3.1 Feature Boundary

The Subtraction feature adds support for the `-` operator. In RPN, `a b -` means a - b.

**End-to-end behavior:**
- Input: `5 3 -`
- Output: `$5 - 3$`

### 3.2 Cross-Module Components

| Module | Component | Role |
|--------|-----------|------|
| **tokens.py** | `TokenType.MINUS` | Token type enum variant |
| **tokens.py** | `Token` dataclass | Stores "-" token |
| **lexer.py** | Char "-" handling (lines 153-162) | Recognizes - as operator or prefix |
| **ast_nodes.py** | `BinaryOp` dataclass | Stores operator node |
| **parser.py** | Binary op handling (lines 115-147) | Creates `BinaryOp` nodes |
| **parser.py** | Op map: `MINUS: "-"` (line 133) | Maps token to operator string |
| **latex_gen.py** | `BINARY_OPS: "-" → "-"` | LaTeX output for - |
| **latex_gen.py** | `PRECEDENCE: "-" → 1` | Operator precedence (same as +) |
| **latex_gen.py** | Parens logic (lines 176-180) | Right-associative parens for - |

### 3.3 Data Structures

#### Token
```python
Token(TokenType.MINUS, "-", line, column)
```

#### AST Node
```python
BinaryOp(operator="-", left=..., right=..., line=..., column=...)
```

### 3.4 Algorithm Details

#### Lexing (lexer.py: lines 153-162)

**Special handling:** The `-` character is ambiguous - it can be:
1. A subtraction operator (binary: `5 3 -`)
2. A negative number prefix (unary: `-5`)

The lexer disambiguates by checking if a digit immediately follows:

```
if char == '-':
  advance()

  if not at_end() and peek().isdigit():
    # It's a negative number
    return scan_number("-", start_line, start_column)
  else:
    # It's a subtraction operator
    return Token(TokenType.MINUS, "-", start_line, start_column)
```

#### Parsing

Identical to Addition - both are binary operators with precedence level 1.

```
if token.type == MINUS:
  right := stack.pop()
  left := stack.pop()
  node := BinaryOp(operator="-", left=left, right=right, ...)
  stack.push(node)
```

#### Code Generation

Important: Subtraction is **left-associative** and **non-commutative**.

The generator must add parentheses on the RIGHT side when subtraction appears on the right side of another subtraction (or division):

```
NEEDS_PARENS(child, parent_precedence, is_right):
  if child is not BinaryOp:
    return False

  child_precedence := PRECEDENCE[child.operator]

  # Lower precedence always needs parens
  if child_precedence < parent_precedence:
    return True

  # Equal precedence on right side needs parens for - and /
  if child_precedence == parent_precedence and is_right:
    if child.operator in ("-", "/"):
      return True

  return False
```

**Example:**
- Input: `5 3 - 2 -` (means (5 - 3) - 2)
- Output: `$5 - 3 - 2$` (left parens not shown)

### 3.5 Dependencies

- **Depends on:** Numbers feature
- **Related to:** Addition (same precedence level)
- **Affects:** Precedence feature (requires right-associativity handling)

### 3.6 Test Cases

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 -` | `$5 - 3$` | Basic subtraction |
| `5 3 - 2 -` | `$5 - 3 - 2$` | Left-associative (no parens on left) |

---

## Feature 4: Multiplication

### 4.1 Feature Boundary

The Multiplication feature adds support for the `*` operator. In RPN, `a b *` means a × b.

**End-to-end behavior:**
- Input: `4 7 *`
- Output: `$4 \times 7$`

### 4.2 Cross-Module Components

| Module | Component | Role |
|--------|-----------|------|
| **tokens.py** | `TokenType.MULT` | Token type enum variant |
| **tokens.py** | `Token` dataclass | Stores "*" token |
| **lexer.py** | Char "*" → `TokenType.MULT` (lines 163-165) | Recognizes * in input |
| **ast_nodes.py** | `BinaryOp` dataclass | Stores operator node |
| **parser.py** | Binary op handling (lines 115-147) | Creates `BinaryOp` nodes |
| **parser.py** | Op map: `MULT: "*"` (line 134) | Maps token to operator string |
| **latex_gen.py** | `BINARY_OPS: "*" → "\times"` | LaTeX output (special symbol) |
| **latex_gen.py** | `PRECEDENCE: "*" → 2` | Higher precedence than +/- |
| **latex_gen.py** | `_visit_binary_op()` | Generates LaTeX with correct parens |

### 4.3 Data Structures

#### Token
```python
Token(TokenType.MULT, "*", line, column)
```

#### AST Node
```python
BinaryOp(operator="*", left=..., right=..., line=..., column=...)
```

### 4.4 Algorithm Details

#### Lexing (lexer.py: lines 163-165)

```
if char == '*':
  advance()
  return Token(TokenType.MULT, "*", start_line, start_column)
```

#### Parsing

Identical to addition and subtraction at the parsing level:

```
if token.type == MULT:
  right := stack.pop()
  left := stack.pop()
  node := BinaryOp(operator="*", left=left, right=right, ...)
  stack.push(node)
```

#### Code Generation (latex_gen.py)

**Key difference from +/-:** Multiplication has higher precedence (level 2 vs level 1).

```
PRECEDENCE:
  "+": 1
  "-": 1
  "*": 2  ← Higher binding
  "/": 2  ← Higher binding
```

LaTeX output uses `\times` symbol instead of `*`:

```
BINARY_OPS["*"] → r"\times"  # Backslash-escaped raw string
```

Parentheses logic:
- Lower-precedence operations on either side need parens
- Higher-precedence or standalone numbers need no parens

**Examples:**
- `2 3 4 * +` → `$2 + 3 \times 4$` (no parens needed - * binds tighter)
- `5 3 + 2 *` → `$( 5 + 3 ) \times 2$` (parens needed - + has lower precedence)

### 4.5 Dependencies

- **Depends on:** Numbers feature
- **Interacts with:** Addition/Subtraction (different precedence)
- **Used by:** Precedence feature

### 4.6 Test Cases

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `4 7 *` | `$4 \times 7$` | Basic multiplication with LaTeX symbol |
| `2 3 4 * +` | `$2 + 3 \times 4$` | * binds tighter (no parens) |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | * on left side (no parens) |

---

## Feature 5: Division

### 5.1 Feature Boundary

The Division feature adds support for the `/` operator. In RPN, `a b /` means a ÷ b.

**End-to-end behavior:**
- Input: `10 2 /`
- Output: `$10 \div 2$`

### 5.2 Cross-Module Components

| Module | Component | Role |
|--------|-----------|------|
| **tokens.py** | `TokenType.DIV` | Token type enum variant |
| **tokens.py** | `Token` dataclass | Stores "/" token |
| **lexer.py** | Char "/" → `TokenType.DIV` (lines 166-168) | Recognizes / in input |
| **ast_nodes.py** | `BinaryOp` dataclass | Stores operator node |
| **parser.py** | Binary op handling (lines 115-147) | Creates `BinaryOp` nodes |
| **parser.py** | Op map: `DIV: "/"` (line 135) | Maps token to operator string |
| **latex_gen.py** | `BINARY_OPS: "/" → "\div"` | LaTeX output (special symbol) |
| **latex_gen.py** | `PRECEDENCE: "/" → 2` | Same as multiplication |
| **latex_gen.py** | Right-assoc parens (lines 176-180) | Parens on right for same precedence |

### 5.3 Data Structures

#### Token
```python
Token(TokenType.DIV, "/", line, column)
```

#### AST Node
```python
BinaryOp(operator="/", left=..., right=..., line=..., column=...)
```

### 5.4 Algorithm Details

#### Lexing (lexer.py: lines 166-168)

```
if char == '/':
  advance()
  return Token(TokenType.DIV, "/", start_line, start_column)
```

#### Parsing

Same as other binary operators:

```
if token.type == DIV:
  right := stack.pop()
  left := stack.pop()
  node := BinaryOp(operator="/", left=left, right=right, ...)
  stack.push(node)
```

#### Code Generation (latex_gen.py)

**Key properties:**
- Same precedence as multiplication (level 2)
- Uses `\div` symbol in LaTeX
- Left-associative and non-commutative (needs right-side parens)

```
BINARY_OPS["/"] → r"\div"
PRECEDENCE["/"] = 2  # Same as *
```

Parenthesization:
- Equal precedence on right side → add parens (for left-associativity)
- Lower precedence on either side → add parens

**Examples:**
- `100 10 / 5 / 2 /` → `$100 \div 10 \div 5 \div 2$` (left-assoc, no parens shown)
- `10 2 / 3 +` → `$10 \div 2 + 3$` (/ has higher precedence)

### 5.5 Dependencies

- **Depends on:** Numbers feature
- **Interacts with:** Multiplication (same precedence)
- **Used by:** Precedence feature

### 5.6 Test Cases

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `10 2 /` | `$10 \div 2$` | Basic division with LaTeX symbol |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Chained division (left-assoc) |

---

## Feature 6: Precedence

### 6.1 Feature Boundary

The Precedence feature ensures that complex expressions with mixed operators produce correct output by adding parentheses when needed.

**Core principle:** Operator precedence determines when parentheses are inserted.

**End-to-end behavior:**
- Input: `5 3 + 2 *`
- RPN meaning: (5 + 3) × 2
- Output: `$( 5 + 3 ) \times 2$` ← Parens added

### 6.2 Cross-Module Components

| Module | Component | Role |
|--------|-----------|------|
| **latex_gen.py** | `PRECEDENCE` dict (lines 57-62) | Precedence levels for operators |
| **latex_gen.py** | `_visit_binary_op()` (lines 111-141) | Main logic for inserting parens |
| **latex_gen.py** | `_needs_parens()` (lines 143-180) | Decision logic for parenthesization |

### 6.3 Data Structures

#### Precedence Table
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,   # Low precedence
    "-": 1,   # Low precedence (same as +)
    "*": 2,   # High precedence
    "/": 2,   # High precedence (same as *)
}
```

### 6.4 Algorithm Details

#### Parenthesization Logic (latex_gen.py: lines 143-180)

The `_needs_parens()` method determines if a child expression needs parentheses based on:

1. **Precedence comparison:** child_precedence vs parent_precedence
2. **Associativity handling:** special rules for equal precedence on the right side

```
NEEDS_PARENS(child, parent_precedence, is_right):
  # If child is not a binary operation, it never needs parens
  if not isinstance(child, BinaryOp):
    return False

  child_precedence := PRECEDENCE[child.operator]

  # RULE 1: Lower precedence always needs parens
  if child_precedence < parent_precedence:
    return True

  # RULE 2: Equal precedence on right side needs parens
  # for non-commutative operators (- and /)
  if child_precedence == parent_precedence and is_right:
    if child.operator in ("-", "/"):
      return True

  return False
```

#### Binary Operation Code Generation (latex_gen.py: lines 111-141)

```
_VISIT_BINARY_OP(node: BinaryOp) -> str:
  op_latex := BINARY_OPS[node.operator]
  precedence := PRECEDENCE[node.operator]

  # Process left operand
  left := visit(node.left)
  if needs_parens(node.left, precedence, is_right=False):
    left := "( " + left + " )"

  # Process right operand
  right := visit(node.right)
  if needs_parens(node.right, precedence, is_right=True):
    right := "( " + right + " )"

  # Assemble output: "left op right"
  return left + " " + op_latex + " " + right
```

#### Precedence Resolution Examples

**Example 1: Lower precedence child**
```
Input: 5 3 + 2 *
AST: BinaryOp("*", BinaryOp("+", 5, 3), 2)
Processing:
  - Visit * node
  - PRECEDENCE["*"] = 2
  - Visit left: BinaryOp("+", 5, 3)
    - child_precedence = 1
    - parent_precedence = 2
    - 1 < 2 → TRUE (needs parens)
    - Result: "( 5 + 3 )"
  - Visit right: 2 (just a number, no parens)
Output: $( 5 + 3 ) \times 2$
```

**Example 2: Right-associativity of subtraction**
```
Input: 5 3 - 2 -
AST: BinaryOp("-", BinaryOp("-", 5, 3), 2)
Processing:
  - Visit outer - node
  - PRECEDENCE["-"] = 1
  - Visit left: BinaryOp("-", 5, 3)
    - child_precedence = 1
    - parent_precedence = 1
    - is_right = False
    - 1 == 1 and is_right=False → No parens (left side exception)
    - Result: "5 - 3"
  - Visit right: 2 (number, no parens)
Output: $5 - 3 - 2$
```

**Example 3: Nested different precedences**
```
Input: 10 2 / 3 + 4 *
AST: BinaryOp("*", BinaryOp("+", BinaryOp("/", 10, 2), 3), 4)
Processing:
  - Visit * node (precedence 2)
    - Left: BinaryOp("+", ...) with precedence 1
      - 1 < 2 → Needs parens
      - Recurse into +
        - Left: BinaryOp("/", 10, 2) with precedence 2
          - 2 > 1 → No parens
        - Right: 3 (number)
        - Result: "10 \div 2 + 3"
      - After parens: "( 10 \div 2 + 3 )"
    - Right: 4 (number)
Output: $( 10 \div 2 + 3 ) \times 4$
```

### 6.5 Dependencies

- **Depends on:** All four operators (+, -, *, /)
- **Required for:** Correct output with mixed operators
- **No features depend on this** - it's a property of all binary operators

### 6.6 Test Cases

| Input | Expected Output | Explanation |
|-------|-----------------|-------------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Lower precedence on left needs parens |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Same as above |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Lower precedence on right needs parens |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Both sides need parens |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Mixed precedence levels |

---

## Cross-Cutting Concerns

### Position Tracking

All tokens and AST nodes track their position in the source (line and column, 1-based).

**Implementation:**
- Lexer: `self.line`, `self.column` updated during `_advance()`
- Parser: Passes position from token to AST node
- Errors: Position used for error context display

**Go mapping:**
- Position struct with `Line int, Column int`
- Embedded in Token and all AST node types

### Error Handling

Two main error types:

1. **LexerError** (lexer.py: lines 21-43)
   - Raised when unexpected character encountered
   - Stores: message, line, column

2. **ParserError** (parser.py: lines 34-54)
   - Raised when invalid RPN encountered
   - Stores: message, token

Both propagate to CLI which formats and displays them.

**Go mapping:**
- Custom error types implementing `error` interface
- Include position and message fields

### Error Formatting

ErrorFormatter (errors.py) provides context-aware error messages:

```python
formatter.format_error(message, line, column) -> str
```

**Output example:**
```
Error: Unexpected character '@'

1 | 5 3 @
        ^
```

### CLI Pipeline

The main() function (cli.py: lines 30-110) orchestrates:

1. Parse command-line arguments (input file, output file)
2. Read input (file or stdin)
3. Tokenize → Parse → Generate
4. Write output (file or stdout)
5. Handle errors with proper exit codes

---

## I/O Contract

### Feature: Numbers

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5` | `$5$` | PASS |
| `3.14` | `$3.14$` | PASS |

### Feature: Addition

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5 3 +` | `$5 + 3$` | PASS |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | PASS |

### Feature: Subtraction

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5 3 -` | `$5 - 3$` | PASS |
| `5 3 - 2 -` | `$5 - 3 - 2$` | PASS |

### Feature: Multiplication

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `4 7 *` | `$4 \times 7$` | PASS |
| `2 3 4 * +` | `$2 + 3 \times 4$` | PASS |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | PASS |

### Feature: Division

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `10 2 /` | `$10 \div 2$` | PASS |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | PASS |

### Feature: Operator Precedence

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | PASS |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | PASS |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | PASS |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | PASS |

### Feature: Floating Point Arithmetic

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `3.14 2 *` | `$3.14 \times 2$` | PASS |
| `1.5 0.5 +` | `$1.5 + 0.5$` | PASS |

### Summary

- **Total Test Cases:** 18
- **All Passing:** Yes
- **Test Coverage:** 100% of Phase 0 features

---

## Implementation Roadmap

### Phase 1: Token & AST Definitions (No parsing logic needed)

**Goal:** Define the data structures

**Modules:**
- `tokens.go`: TokenType enum, Token struct
- `ast_nodes.go`: ASTNode, Number, BinaryOp, Expr type alias

**Dependencies:** Go standard library only

**Success criteria:** Types compile and are usable

### Phase 2: Lexer Implementation

**Goal:** Convert text to token stream

**Modules:**
- `lexer.go`: Lexer struct, tokenize(), _scanNumber(), _advance(), _peek()
- `errors.go`: LexerError type

**Dependencies:** tokens.go

**Success criteria:**
- Can tokenize all single tokens
- Handles negative numbers correctly
- Produces correct position info

**Test subset:**
- Single numbers: `5` → token
- Operators: `+`, `-`, `*`, `/` → tokens
- Negative numbers: `-3` → single token

### Phase 3: Parser Implementation

**Goal:** Convert token stream to AST

**Modules:**
- `parser.go`: Parser struct, parse(), stack management

**Dependencies:** tokens.go, ast_nodes.go, lexer.go

**Success criteria:**
- Stack-based RPN parsing works
- Produces correct AST structure
- Reports operand count errors

**Test subset:**
- `5` → Number node
- `5 3 +` → BinaryOp(+, 5, 3)

### Phase 4: LaTeX Generator Implementation

**Goal:** Convert AST to LaTeX output

**Modules:**
- `latex_gen.go`: LaTeXGenerator struct, generate(), _visit*() methods

**Dependencies:** ast_nodes.go

**Success criteria:**
- Numbers output as-is
- Operators map to correct symbols
- Parenthesization logic works
- Output wrapped in $...$ delimiters

**Test subset:**
- Numbers: `$5$`, `$3.14$`
- Operators: `$5 + 3$`, `$5 \times 3$`
- Precedence: `$( 5 + 3 ) \times 2$`

### Phase 5: CLI & Integration

**Goal:** Wire everything together

**Modules:**
- `cli.go`: main() function, file I/O, error formatting
- `errors.go`: ErrorFormatter implementation

**Dependencies:** All above modules

**Success criteria:**
- All 18 I/O contract tests pass
- Error messages display correctly
- File I/O works
- Exit codes correct

### Testing Strategy

**For each phase:**
1. Write unit tests for new functionality
2. Run I/O contract tests
3. Verify error cases

**Final verification:**
```bash
# All 18 tests must pass with exact output match
for test in test_cases:
    output = rpn2tex(test.input)
    assert output == test.expected_output
```

---

## Python-to-Go Type Mappings

### Basic Types

| Python | Go | Notes |
|--------|----|----|
| `str` | `string` | Immutable by default |
| `list[T]` | `[]T` | Slice type |
| `dict[K, V]` | `map[K]V` | Reference type |
| `int` | `int` | Same semantics |
| `float` | `float64` | Use float64 for decimals |
| `bool` | `bool` | Same semantics |
| `Enum` | `const` or `iota` | Use iota for enum-like values |

### Dataclass Patterns

**Python:**
```python
@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str
    line: int
    column: int
```

**Go (struct embedding recommended):**
```go
type Token struct {
    Type   TokenType
    Value  string
    Line   int
    Column int
}
```

### Class Patterns

**Python (with methods):**
```python
class Lexer:
    text: str
    pos: int

    def __init__(self, text: str):
        self.text = text
        self.pos = 0

    def tokenize(self) -> list[Token]:
        pass
```

**Go (receiver pattern):**
```go
type Lexer struct {
    text string
    pos  int
}

func NewLexer(text string) *Lexer {
    return &Lexer{text: text, pos: 0}
}

func (l *Lexer) Tokenize() []Token {
    // ...
}
```

### Enum Patterns

**Python:**
```python
class TokenType(Enum):
    NUMBER = auto()
    PLUS = auto()
    EOF = auto()
```

**Go (iota pattern):**
```go
type TokenType int

const (
    TokenNumber TokenType = iota
    TokenPlus
    TokenEOF
)

func (t TokenType) String() string {
    // ...
}
```

### Union Types

**Python:**
```python
Expr = Number | BinaryOp
```

**Go (interface-based):**
```go
type Expr interface {
    // Marker methods
    isExpr()
}

func (n *Number) isExpr()   {}
func (b *BinaryOp) isExpr() {}

// Type assertions to use:
// switch expr := e.(type) {
// case *Number:
//     // handle Number
// case *BinaryOp:
//     // handle BinaryOp
// }
```

### Error Handling

**Python:**
```python
class LexerError(Exception):
    def __init__(self, message: str, line: int, column: int):
        super().__init__(f"Line {line}, column {column}: {message}")
        self.message = message
        self.line = line
        self.column = column

# Usage:
try:
    lexer.tokenize()
except LexerError as e:
    print(e)
```

**Go (custom error types):**
```go
type LexerError struct {
    Message string
    Line    int
    Column  int
}

func (e *LexerError) Error() string {
    return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
}

// Usage:
tokens, err := lexer.Tokenize()
if err != nil {
    var lexErr *LexerError
    if errors.As(err, &lexErr) {
        // handle LexerError specifically
    }
}
```

### Method Dispatch Pattern

**Python (singledispatchmethod):**
```python
from functools import singledispatchmethod

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
        pass
```

**Go (type assertion with switch):**
```go
func (g *LaTeXGenerator) visit(expr Expr) string {
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

func (g *LaTeXGenerator) visitBinaryOp(b *BinaryOp) string {
    // ...
}
```

---

## Code Organization for Go

### Directory Structure

```
rpn2tex-go/
├── main.go              # CLI entry point
├── tokens.go            # Token types
├── ast_nodes.go         # AST node definitions
├── lexer.go             # Lexer implementation
├── parser.go            # Parser implementation
├── latex_gen.go         # LaTeX generator
├── errors.go            # Error types and formatting
└── cmd/
    └── rpn2tex/
        └── main.go      # Command wrapper
```

### Package Organization

- Single package `rpn2tex` or `main` (if CLI) for simplicity
- Public types start with uppercase (Token, Lexer, etc.)
- Private helpers start with lowercase (_peek, _advance, etc.)

---

## Key Implementation Notes for Go Migrators

### 1. String Handling

Go strings are UTF-8 by default but must be indexed as byte arrays or rune slices.

**For character-by-character scanning:**
```go
// DON'T do this (doesn't work for multi-byte characters):
// char := l.text[l.pos]

// DO this instead:
runes := []rune(l.text)
char := runes[l.pos]
```

### 2. Stack Implementation

Use slices as stacks (push with append, pop with len/slice):

```go
stack := []Expr{}
stack = append(stack, expr)           // Push
if len(stack) > 0 {
    expr := stack[len(stack)-1]       // Peek
    stack = stack[:len(stack)-1]      // Pop
}
```

### 3. Struct Receivers vs Pointers

Use pointer receivers for structs that are modified:

```go
// For Lexer (mutable):
func (l *Lexer) Tokenize() []Token { ... }

// For Generator (generally immutable, but can use pointers for convention):
func (g *LaTeXGenerator) Generate(ast Expr) string { ... }
```

### 4. Type Assertions in Visitor Pattern

Remember to check type assertions:

```go
switch e := expr.(type) {
case *Number:
    return e.Value
case *BinaryOp:
    // Handle BinaryOp
    childPrec := g.PRECEDENCE[e.Operator]
    // ...
default:
    panic(fmt.Sprintf("Unknown expression type: %T", expr))
}
```

### 5. Regex or String Scanning

For number parsing, Go's `strconv` package is useful:

```go
// Scan number characters manually
var value string
for !l.atEnd() && isDigit(l.peek()) {
    value += string(l.advance())
}
if !l.atEnd() && l.peek() == '.' {
    value += string(l.advance())
    for !l.atEnd() && isDigit(l.peek()) {
        value += string(l.advance())
    }
}
```

### 6. Constants for Operator Maps

Define operator precedence and LaTeX mappings as package-level constants or fields:

```go
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
```

### 7. Error Context

Store error information that can be formatted later:

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

---

## Testing Checklist

For each feature implemented, verify:

**Feature 1: Numbers**
- [ ] Single digit integers parse correctly
- [ ] Multi-digit integers parse correctly
- [ ] Decimal numbers parse correctly
- [ ] Negative numbers recognized (- followed by digit)
- [ ] Number output matches input exactly

**Feature 2: Addition**
- [ ] Single addition produces correct output
- [ ] Chained additions left-associate correctly
- [ ] Output format: `left + right`

**Feature 3: Subtraction**
- [ ] Single subtraction produces correct output
- [ ] Chained subtractions left-associate without parens
- [ ] Output format: `left - right`

**Feature 4: Multiplication**
- [ ] Single multiplication uses \times symbol
- [ ] Multiplication binds tighter than addition

**Feature 5: Division**
- [ ] Single division uses \div symbol
- [ ] Division binds tighter than addition
- [ ] Chained divisions left-associate correctly

**Feature 6: Precedence**
- [ ] Lower precedence operations parenthesized
- [ ] Higher precedence operations not parenthesized
- [ ] Right-side equal precedence for - and / parenthesized
- [ ] Nested operations parenthesized correctly

**End-to-End**
- [ ] All 18 test cases pass with exact output
- [ ] Error cases detected and reported
- [ ] CLI input/output works
- [ ] Decimal numbers work
- [ ] Spacing in output matches exactly

---

## Document Information

**Source Implementation:** Python (rpn2tex)
**Target Implementation:** Go
**Strategy:** Feature-by-feature migration
**Total Features:** 6 core features
**Test Coverage:** 18 verified test cases
**Documentation Status:** Complete

---

**Generated:** 2025-12-30
**For questions or clarifications, refer to the Python source code in:**
`/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
