# PHASE 1: Feature-by-Feature Migration Specification for rpn2tex (Python to Go)

**Date Generated:** 2025-12-30
**Source Implementation:** Python (source/)
**Target Language:** Go
**Organization:** Feature-by-Feature (Dependency Order)

---

## Table of Contents

1. [Overview](#overview)
2. [I/O Contract](#io-contract)
3. [Feature: Numbers](#feature-numbers)
4. [Feature: Addition](#feature-addition)
5. [Feature: Subtraction](#feature-subtraction)
6. [Feature: Multiplication](#feature-multiplication)
7. [Feature: Division](#feature-division)
8. [Feature: Precedence](#feature-precedence)
9. [Feature Dependencies](#feature-dependencies)
10. [Go Idioms and Patterns](#go-idioms-and-patterns)
11. [Error Handling Strategy](#error-handling-strategy)
12. [Migration Checklist](#migration-checklist)

---

## Overview

The rpn2tex system converts Reverse Polish Notation (RPN) mathematical expressions into LaTeX output. The Python implementation uses a pipeline architecture:

```
Input Text → Lexer → Tokens → Parser → AST → LaTeX Generator → LaTeX Output
```

This feature-by-feature migration specification organizes the work by features (numbers, operators, precedence) rather than by modules. Each feature may span multiple modules, and features have dependencies that must be respected during implementation.

**Key Design Principles:**
- Stack-based RPN parsing (simpler than recursive descent)
- Visitor pattern for AST traversal
- Position tracking for error reporting
- Operator precedence with parenthesization

---

## I/O Contract

### Test Cases (Verified Against Python Implementation)

#### Numbers

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5` | `$5$` | PASS | Single integer literal |
| `3.14` | `$3.14$` | PASS | Floating-point literal |

#### Addition

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 +` | `$5 + 3$` | PASS | Simple addition |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | PASS | Chained addition (left-associative) |

#### Subtraction

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 -` | `$5 - 3$` | PASS | Simple subtraction |
| `5 3 - 2 -` | `$5 - 3 - 2$` | PASS | Chained subtraction (left-associative) |

#### Multiplication

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `4 7 *` | `$4 \times 7$` | PASS | Simple multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | PASS | Addition and multiplication (respects precedence) |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | PASS | Multiplication and addition (respects precedence) |
| `3.14 2 *` | `$3.14 \times 2$` | PASS | Multiplication with floating-point |

#### Division

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `10 2 /` | `$10 \div 2$` | PASS | Simple division |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | PASS | Chained division (left-associative) |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | PASS | Division and multiplication (same precedence, left-to-right) |

#### Operator Precedence (Parentheses Required)

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | PASS | Addition result needs parentheses when multiplied |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | PASS | Addition result needs parentheses when multiplied |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | PASS | Addition result needs parentheses in second operand |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS | Both operands are sums needing parentheses |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | PASS | Mixed operators with parentheses |

#### Floating-Point

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `1.5 0.5 +` | `$1.5 + 0.5$` | PASS | Floating-point operands in addition |

**Summary:**
- **Total Test Cases:** 21
- **Passing:** 18
- **Failing:** 0
- **Errors (Expected):** 3 (exponentiation operator not implemented)
- **Coverage:** 100% of implemented features

---

## Feature: Numbers

### Feature Scope
Parse and output numeric literals (both integers and floating-point decimals).

### Cross-Module Implementation

#### 1. tokens.py (TokenType Definition)
```python
class TokenType(Enum):
    NUMBER = auto()  # Numeric values: 5, 3.14, -2
```

**Key Point:** `NUMBER` is a single token type for all numeric values.

#### 2. lexer.py (Tokenization)
```python
def _scan_number(self, prefix: str, start_line: int, start_column: int) -> Token:
    """Scan a numeric literal."""
    value = prefix

    # Integer part
    while not self._at_end() and self._peek().isdigit():
        value += self._advance()

    # Decimal part (optional)
    if not self._at_end() and self._peek() == ".":
        value += self._advance()  # consume '.'
        while not self._at_end() and self._peek().isdigit():
            value += self._advance()

    return Token(TokenType.NUMBER, value, start_line, start_column)
```

**Key Points:**
- Supports integer part: `5`, `42`, `100`
- Supports decimal part: `3.14`, `0.5`
- Negative numbers: Recognized when `-` is followed by digit immediately
- Returns token with string value (not converted to numeric type)

#### 3. ast_nodes.py (AST Representation)
```python
@dataclass(frozen=True)
class Number(ASTNode):
    """Numeric literal node."""
    value: str  # String representation of the number
```

**Key Points:**
- Number stores value as string
- Immutable dataclass with position tracking (line, column)

#### 4. parser.py (AST Construction)
```python
if token.type == TokenType.NUMBER:
    num_node = Number(
        line=token.line, column=token.column, value=token.value
    )
    stack.append(num_node)
    self._advance()
```

**Key Points:**
- Simple push onto stack in RPN parsing
- No validation or conversion to numeric type

#### 5. latex_gen.py (Output Generation)
```python
@_visit.register
def _visit_number(self, node: Number) -> str:
    """Generate LaTeX for a number literal."""
    return node.value
```

**Key Points:**
- Direct pass-through of string value
- No numeric formatting applied

### Go Migration Strategy

#### Type Mapping

| Python | Go | Notes |
|--------|-----|-------|
| `str` (token value) | `string` | Store numeric literal as string, not numeric type |
| `Token` with `TokenType.NUMBER` | `TokenType` enum with `Number` variant | Enum-based token type system |
| `Number` dataclass | `Number` struct | Use struct with embedded position |

#### Implementation Pattern (Go)

```go
// Token type enum
type TokenType int
const (
    TokenNumber TokenType = iota
    // ...
)

// Token struct
type Token struct {
    Type   TokenType
    Value  string
    Line   int
    Column int
}

// AST node struct
type Number struct {
    Line   int
    Column int
    Value  string
}

// Lexer method
func (l *Lexer) scanNumber(prefix string, startLine, startColumn int) Token {
    value := prefix

    // Integer part
    for !l.atEnd() && isDigit(l.peek()) {
        value += string(l.advance())
    }

    // Decimal part
    if !l.atEnd() && l.peek() == '.' {
        value += string(l.advance())
        for !l.atEnd() && isDigit(l.peek()) {
            value += string(l.advance())
        }
    }

    return Token{Type: TokenNumber, Value: value, Line: startLine, Column: startColumn}
}

// Parser logic
case TokenNumber:
    node := &Number{
        Line:   token.Line,
        Column: token.Column,
        Value:  token.Value,
    }
    stack = append(stack, node)
    p.advance()

// Generator
func (g *Generator) visitNumber(n *Number) string {
    return n.Value
}
```

#### Key Implementation Notes

1. **String Representation:** Keep numeric values as strings throughout the pipeline. Do not convert to `float64` or `int` unless needed for computation.

2. **Decimal Handling:** The lexer must recognize the dot (`.`) as part of the number, not a separate token. Test cases include `3.14`.

3. **Negative Numbers:** The lexer handles `-` specially:
   - If `-` is followed immediately by a digit, it's part of a negative number
   - Otherwise, it's the subtraction operator

   Go implementation:
   ```go
   if char == '-' {
       l.advance()
       if !l.atEnd() && isDigit(l.peek()) {
           return l.scanNumber("-", startLine, startColumn)
       }
       return Token{Type: TokenMinus, Value: "-", Line: startLine, Column: startColumn}
   }
   ```

4. **Position Tracking:** Numbers must track their starting line and column for error reporting.

### Test Cases for Feature

```
Input: "5" → Token(NUMBER, "5", 1:1) → Number(1, 1, "5") → "$5$"
Input: "3.14" → Token(NUMBER, "3.14", 1:1) → Number(1, 1, "3.14") → "$3.14$"
Input: "-5" → Token(NUMBER, "-5", 1:1) → Number(1, 1, "-5") → "$-5$"
```

### Dependency Graph
- **No dependencies:** This is a foundational feature that other operators depend on.
- **Depended on by:** Addition, Subtraction, Multiplication, Division

---

## Feature: Addition

### Feature Scope
Implement the `+` operator for RPN addition expressions.

### Cross-Module Implementation

#### 1. tokens.py (TokenType Definition)
```python
class TokenType(Enum):
    PLUS = auto()  # + (addition)
```

#### 2. lexer.py (Tokenization)
```python
if char == "+":
    self._advance()
    return Token(TokenType.PLUS, "+", start_line, start_column)
```

**Key Points:**
- Single character operator
- Always recognized as operator (no special cases like MINUS)

#### 3. parser.py (AST Construction - Stack-Based)
```python
elif token.type in (TokenType.PLUS, TokenType.MINUS, ...):
    if len(stack) < 2:
        raise ParserError(f"Operator '{token.value}' requires two operands", token)

    right = stack.pop()
    left = stack.pop()

    op_map = {
        TokenType.PLUS: "+",
        # ...
    }
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
```

**RPN Stack Evolution Example:**
```
Input: "5 3 +"
Step 1: Push 5 → stack: [5]
Step 2: Push 3 → stack: [5, 3]
Step 3: See +, pop 3 and 5, create BinaryOp("+", 5, 3) → stack: [5+3]
Result: BinaryOp("+", Number("5"), Number("3"))
```

#### 4. ast_nodes.py (AST Representation)
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    """Binary operation node."""
    operator: str  # The operator string ("+", "-", "*", "/")
    left: Expr
    right: Expr
```

#### 5. latex_gen.py (Output Generation)
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "+": "+",
    # ...
}

PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,  # Low precedence
    # ...
}

@_visit.register
def _visit_binary_op(self, node: BinaryOp) -> str:
    """Generate LaTeX for a binary operation."""
    op_latex = self.BINARY_OPS[node.operator]
    my_precedence = self.PRECEDENCE[node.operator]

    left = self._visit(node.left)
    if self._needs_parens(node.left, my_precedence, is_right=False):
        left = f"( {left} )"

    right = self._visit(node.right)
    if self._needs_parens(node.right, my_precedence, is_right=True):
        right = f"( {right} )"

    return f"{left} {op_latex} {right}"
```

**Key Points:**
- Addition has precedence level 1 (lower than multiplication/division)
- Output uses `+` directly (not special LaTeX command)
- Spacing: `" + "` (space-operator-space)

### Go Migration Strategy

#### Type Mapping

| Python | Go | Notes |
|--------|-----|-------|
| `TokenType.PLUS` enum variant | `TokenPlus` const in `TokenType` | Represents `+` operator token |
| `BinaryOp` dataclass with operator field | `BinaryOp` struct | Same structure for left/right operands |
| `BINARY_OPS` dict | Map or method with switch statement | Maps operator string to LaTeX |
| `PRECEDENCE` dict | Map or const array | Operator precedence levels |

#### Implementation Pattern (Go)

```go
// Token type
type TokenType int
const (
    TokenPlus TokenType = iota
    // ...
)

// Lexer
if char == '+' {
    l.advance()
    return Token{Type: TokenPlus, Value: "+", Line: startLine, Column: startColumn}
}

// AST Node
type BinaryOp struct {
    Line     int
    Column   int
    Operator string
    Left     Expr
    Right    Expr
}

// Parser (RPN stack-based)
case TokenPlus:
    if len(stack) < 2 {
        return nil, fmt.Errorf("operator '%s' requires two operands", token.Value)
    }
    right := stack[len(stack)-1]
    stack = stack[:len(stack)-1]
    left := stack[len(stack)-1]
    stack = stack[:len(stack)-1]

    node := &BinaryOp{
        Line:     token.Line,
        Column:   token.Column,
        Operator: "+",
        Left:     left,
        Right:    right,
    }
    stack = append(stack, node)
    p.advance()

// Generator
var binaryOps = map[string]string{
    "+": "+",
    // ...
}

var precedence = map[string]int{
    "+": 1,
    // ...
}

func (g *Generator) visitBinaryOp(node *BinaryOp) string {
    opLatex := binaryOps[node.Operator]
    myPrec := precedence[node.Operator]

    left := g.visit(node.Left)
    if g.needsParens(node.Left, myPrec, false) {
        left = "( " + left + " )"
    }

    right := g.visit(node.Right)
    if g.needsParens(node.Right, myPrec, true) {
        right = "( " + right + " )"
    }

    return left + " " + opLatex + " " + right
}
```

#### Key Implementation Notes

1. **Token Recognition:** The `+` character must be recognized at the lexer level and converted to a `TokenPlus` token.

2. **Binary Operation Creation:** In RPN parsing, when we see `+`, we pop two operands from the stack (right first, then left) and create a `BinaryOp` node. This order is critical because the stack is LIFO (Last In, First Out).

3. **Precedence:** Addition has precedence level 1, which is lower than multiplication and division (level 2). This affects parenthesization during LaTeX generation.

4. **Associativity:** Addition is left-associative. For chained operations like `1 2 + 3 + 4 +`, the AST structure is:
   ```
   BinaryOp("+",
       BinaryOp("+",
           BinaryOp("+",
               Number("1"),
               Number("2")
           ),
           Number("3")
       ),
       Number("4")
   )
   ```
   This naturally left-associates because we process left-to-right in RPN.

5. **LaTeX Output:** No special LaTeX command needed (unlike `*` and `/`). Simple `+` with spacing.

### Test Cases for Feature

```
Input: "5 3 +"
Expected: "$5 + 3$"
Stack trace:
  1. NUMBER 5 → stack: [Number("5")]
  2. NUMBER 3 → stack: [Number("5"), Number("3")]
  3. PLUS → pop 3, pop 5 → stack: [BinaryOp("+", Number("5"), Number("3"))]
  4. EOF → return root

Input: "1 2 + 3 + 4 +"
Expected: "$1 + 2 + 3 + 4$"
Stack trace:
  1. NUMBER 1 → stack: [N1]
  2. NUMBER 2 → stack: [N1, N2]
  3. PLUS → stack: [B1("+", N1, N2)]
  4. NUMBER 3 → stack: [B1, N3]
  5. PLUS → stack: [B2("+", B1, N3)]
  6. NUMBER 4 → stack: [B2, N4]
  7. PLUS → stack: [B3("+", B2, N4)]
  8. EOF → return B3
```

### Dependency Graph
- **Depends on:** Numbers (must have operands)
- **Depended on by:** Precedence (precedence handling includes addition)

---

## Feature: Subtraction

### Feature Scope
Implement the `-` operator for RPN subtraction expressions.

### Cross-Module Implementation

#### 1. tokens.py (TokenType Definition)
```python
class TokenType(Enum):
    MINUS = auto()  # - (subtraction)
```

#### 2. lexer.py (Tokenization)
```python
if char == "-":
    self._advance()
    # Check if this is a negative number (digit follows immediately)
    if not self._at_end() and self._peek().isdigit():
        # It's a negative number
        return self._scan_number("-", start_line, start_column)
    return Token(TokenType.MINUS, "-", start_line, start_column)
```

**Key Points:**
- More complex than `+` because `-` can be:
  1. Subtraction operator (standalone)
  2. Part of a negative number (followed immediately by digit)
- The lookahead must check if next character is a digit

#### 3. parser.py & ast_nodes.py & latex_gen.py

Same structure as Addition (reuses `BinaryOp` node type).

```python
op_map = {
    TokenType.MINUS: "-",
    # ...
}

BINARY_OPS: ClassVar[dict[str, str]] = {
    "-": "-",
    # ...
}

PRECEDENCE: ClassVar[dict[str, int]] = {
    "-": 1,  # Same precedence as addition
    # ...
}

# In _needs_parens, special handling for right-associativity:
return (
    child_precedence == parent_precedence
    and is_right
    and child.operator in ("-", "/")
)
```

**Key Points:**
- Subtraction is left-associative but non-commutative
- Right operand needs parens when at equal precedence
- Example: `5 - (3 - 2)` needs parens on the right

### Go Migration Strategy

#### Type Mapping

| Python | Go | Notes |
|--------|-----|-------|
| `TokenType.MINUS` | `TokenMinus` const | Subtraction operator token |
| Negative number detection | Lookahead in lexer | Must check next char is digit |
| Right-assoc logic in `_needs_parens` | Same logic in `needsParens` | Special case for `/` and `-` |

#### Implementation Pattern (Go)

```go
// Lexer with lookahead
if char == '-' {
    l.advance()
    if !l.atEnd() && isDigit(l.peek()) {
        return l.scanNumber("-", startLine, startColumn)
    }
    return Token{Type: TokenMinus, Value: "-", Line: startLine, Column: startColumn}
}

// Precedence and LaTeX mapping
var precedence = map[string]int{
    "-": 1,  // Same as +
    // ...
}

var binaryOps = map[string]string{
    "-": "-",
    // ...
}

// Right-associativity handling
func (g *Generator) needsParens(child Expr, parentPrec int, isRight bool) bool {
    binOp, ok := child.(*BinaryOp)
    if !ok {
        return false
    }

    childPrec := precedence[binOp.Operator]

    if childPrec < parentPrec {
        return true
    }

    // Right side equality case (for non-commutative operators)
    if childPrec == parentPrec && isRight {
        if binOp.Operator == "-" || binOp.Operator == "/" {
            return true
        }
    }

    return false
}
```

#### Key Implementation Notes

1. **Negative Number vs. Operator:** The critical distinction is whether `-` is followed immediately (no whitespace) by a digit. This requires single-character lookahead in the lexer.

2. **Precedence Level:** Subtraction has the same precedence as addition (level 1), lower than multiplication and division (level 2).

3. **Right-Associativity Handling:** Because subtraction is non-commutative:
   - `5 - 3 - 2` should be `5 - 3 - 2` (left-associative, no extra parens)
   - `5 - (3 - 2)` needs parens on the right operand
   - The generator checks if the right operand is a subtraction/division at equal precedence

4. **Test Case:** `5 3 - 2 -` produces `$5 - 3 - 2$` (no extra parens because it's left-associative)

### Test Cases for Feature

```
Input: "5 3 -"
Expected: "$5 - 3$"

Input: "5 3 - 2 -"
Expected: "$5 - 3 - 2$"
Stack trace:
  1. NUMBER 5 → stack: [N5]
  2. NUMBER 3 → stack: [N5, N3]
  3. MINUS → pop 3, pop 5 → stack: [B1("-", N5, N3)]
  4. NUMBER 2 → stack: [B1, N2]
  5. MINUS → pop 2, pop B1 → stack: [B2("-", B1, N2)]
  6. EOF → return B2
  LaTeX gen: visit(B2) = "-" is at equal prec as child, right side, non-commutative
  → right = "2" (no parens needed because child is Number, not BinaryOp)
  → left = visit(B1) = "5 - 3"
  → result = "5 - 3 - 2"

Input: "5 3 2 - -"
Expected: "$5 - ( 3 - 2 )$"
Stack trace:
  1. NUMBER 5 → stack: [N5]
  2. NUMBER 3 → stack: [N5, N3]
  3. NUMBER 2 → stack: [N5, N3, N2]
  4. MINUS → pop 2, pop 3 → stack: [N5, B1("-", N3, N2)]
  5. MINUS → pop B1, pop N5 → stack: [B2("-", N5, B1)]
  6. EOF → return B2
  LaTeX gen: visit(B2) where right = B1("-", N3, N2)
  → needsParens(B1, 1, isRight=true) = true (because operator is "-")
  → right = "( 3 - 2 )"
  → result = "5 - ( 3 - 2 )"
```

### Dependency Graph
- **Depends on:** Numbers, Lexer with lookahead capability
- **Depended on by:** Precedence

---

## Feature: Multiplication

### Feature Scope
Implement the `*` operator for RPN multiplication expressions.

### Cross-Module Implementation

#### 1. tokens.py (TokenType Definition)
```python
class TokenType(Enum):
    MULT = auto()  # * (multiplication)
```

#### 2. lexer.py (Tokenization)
```python
if char == "*":
    self._advance()
    return Token(TokenType.MULT, "*", start_line, start_column)
```

**Key Points:**
- Simple single-character token
- No special lookahead needed

#### 3. parser.py & ast_nodes.py

Same `BinaryOp` structure (reuses existing AST node).

```python
op_map = {
    TokenType.MULT: "*",
    # ...
}
```

#### 4. latex_gen.py (Output Generation - Critical!)
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "*": r"\times",
    # ...
}

PRECEDENCE: ClassVar[dict[str, int]] = {
    "*": 2,  # HIGH precedence
    # ...
}
```

**Key Points:**
- Multiplication has precedence level 2 (higher than addition/subtraction)
- LaTeX output uses `\times` (special LaTeX command)
- This higher precedence triggers parenthesization in generator

### Go Migration Strategy

#### Type Mapping

| Python | Go | Notes |
|--------|-----|-------|
| `TokenType.MULT` | `TokenMult` const | Multiplication operator |
| `"*"` → `r"\times"` in BINARY_OPS | `"*"` → `"\\times"` in map | Escape backslash in Go strings |
| Precedence level 2 | `2` in precedence map | Higher than addition |

#### Implementation Pattern (Go)

```go
// Lexer
if char == '*' {
    l.advance()
    return Token{Type: TokenMult, Value: "*", Line: startLine, Column: startColumn}
}

// Parser (same as addition/subtraction)
case TokenMult:
    if len(stack) < 2 {
        return nil, fmt.Errorf("operator '*' requires two operands", token.Value)
    }
    right := stack[len(stack)-1]
    stack = stack[:len(stack)-1]
    left := stack[len(stack)-1]
    stack = stack[:len(stack)-1]

    node := &BinaryOp{
        Line:     token.Line,
        Column:   token.Column,
        Operator: "*",
        Left:     left,
        Right:    right,
    }
    stack = append(stack, node)
    p.advance()

// Generator
var binaryOps = map[string]string{
    "*": "\\times",  // Escape backslash
    // ...
}

var precedence = map[string]int{
    "*": 2,  // High precedence
    // ...
}

func (g *Generator) visitBinaryOp(node *BinaryOp) string {
    opLatex := binaryOps[node.Operator]
    myPrec := precedence[node.Operator]

    left := g.visit(node.Left)
    if g.needsParens(node.Left, myPrec, false) {
        left = "( " + left + " )"
    }

    right := g.visit(node.Right)
    if g.needsParens(node.Right, myPrec, true) {
        right = "( " + right + " )"
    }

    return left + " " + opLatex + " " + right
}
```

#### Key Implementation Notes

1. **Higher Precedence:** Multiplication has precedence level 2, which is higher than addition/subtraction (level 1). This means:
   - `2 + 3 * 4` → `2 + ( 3 * 4 )` (no parens on the multiplication result)
   - `5 3 + 2 *` → `( 5 + 3 ) * 2` (parens on the addition result)

2. **LaTeX Command:** The output uses `\times` (backslash-times). In Go strings, this must be escaped: `"\\times"`.

3. **Associativity:** Multiplication is left-associative and commutative (order doesn't matter for correctness, but we generate left-to-right anyway).

4. **Precedence Interaction:** The precedence system is the key to understanding when parentheses are needed. Test the `_needs_parens` logic thoroughly.

### Test Cases for Feature

```
Input: "4 7 *"
Expected: "$4 \\times 7$"

Input: "2 3 4 * +"
Expected: "$2 + 3 \\times 4$"
Stack trace:
  1. NUMBER 2 → stack: [N2]
  2. NUMBER 3 → stack: [N2, N3]
  3. NUMBER 4 → stack: [N2, N3, N4]
  4. MULT → pop 4, pop 3 → stack: [N2, B1("*", N3, N4)]
  5. PLUS → pop B1, pop N2 → stack: [B2("+", N2, B1)]
  6. EOF → return B2
  LaTeX gen: visit(B2) where operator = "+"
    → left = "2"
    → right = visit(B1) = "3 \\times 4"
    → needsParens(B1, 1, false) = false (child_prec 2 > parent_prec 1)
    → result = "2 + 3 \\times 4"

Input: "5 3 + 2 *"
Expected: "$( 5 + 3 ) \\times 2$"
Stack trace:
  1. NUMBER 5 → stack: [N5]
  2. NUMBER 3 → stack: [N5, N3]
  3. PLUS → pop 3, pop 5 → stack: [B1("+", N5, N3)]
  4. NUMBER 2 → stack: [B1, N2]
  5. MULT → pop 2, pop B1 → stack: [B2("*", B1, N2)]
  6. EOF → return B2
  LaTeX gen: visit(B2) where operator = "*"
    → left = visit(B1) = "5 + 3"
    → needsParens(B1, 2, false) = true (child_prec 1 < parent_prec 2)
    → left = "( 5 + 3 )"
    → right = "2"
    → result = "( 5 + 3 ) \\times 2"

Input: "3.14 2 *"
Expected: "$3.14 \\times 2$"
(Tests floating-point operands)
```

### Dependency Graph
- **Depends on:** Numbers, Lexer, Parser (BinaryOp)
- **Depended on by:** Precedence

---

## Feature: Division

### Feature Scope
Implement the `/` operator for RPN division expressions.

### Cross-Module Implementation

#### 1. tokens.py (TokenType Definition)
```python
class TokenType(Enum):
    DIV = auto()  # / (division)
```

#### 2. lexer.py (Tokenization)
```python
if char == "/":
    self._advance()
    return Token(TokenType.DIV, "/", start_line, start_column)
```

**Key Points:**
- Simple single-character token
- No ambiguity (unlike `-`)

#### 3. parser.py & ast_nodes.py

Same `BinaryOp` structure.

```python
op_map = {
    TokenType.DIV: "/",
    # ...
}
```

#### 4. latex_gen.py (Output Generation)
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "/": r"\div",
    # ...
}

PRECEDENCE: ClassVar[dict[str, int]] = {
    "/": 2,  # Same as multiplication
    # ...
}

# In _needs_parens, division is included in right-associativity special case:
return (
    child_precedence == parent_precedence
    and is_right
    and child.operator in ("-", "/")
)
```

**Key Points:**
- Division has same precedence as multiplication (level 2)
- LaTeX output uses `\div` (special LaTeX command)
- Non-commutative: right operand needs parens at equal precedence

### Go Migration Strategy

#### Type Mapping

| Python | Go | Notes |
|--------|-----|-------|
| `TokenType.DIV` | `TokenDiv` const | Division operator |
| `"/"` → `r"\div"` in BINARY_OPS | `"/"` → `"\\div"` in map | Escape backslash |
| Precedence level 2 | `2` in precedence map | Equal to multiplication |
| Right-associativity handling | Same logic as subtraction | Special case in `needsParens` |

#### Implementation Pattern (Go)

```go
// Lexer
if char == '/' {
    l.advance()
    return Token{Type: TokenDiv, Value: "/", Line: startLine, Column: startColumn}
}

// Parser
case TokenDiv:
    if len(stack) < 2 {
        return nil, fmt.Errorf("operator '/' requires two operands", token.Value)
    }
    right := stack[len(stack)-1]
    stack = stack[:len(stack)-1]
    left := stack[len(stack)-1]
    stack = stack[:len(stack)-1]

    node := &BinaryOp{
        Line:     token.Line,
        Column:   token.Column,
        Operator: "/",
        Left:     left,
        Right:    right,
    }
    stack = append(stack, node)
    p.advance()

// Generator
var binaryOps = map[string]string{
    "/": "\\div",  // Escape backslash
    // ...
}

var precedence = map[string]int{
    "/": 2,  // Same as multiplication
    // ...
}

// The needsParens logic already handles division's right-associativity
```

#### Key Implementation Notes

1. **Precedence:** Division has the same precedence as multiplication (level 2). This affects ordering:
   - `10 2 / 5 *` → `10 ÷ 2 × 5` (left-to-right, no parens)
   - `5 3 + 2 /` → `( 5 + 3 ) ÷ 2` (parens on lower precedence operand)

2. **Right-Associativity:** Division is non-commutative and left-associative:
   - `100 10 / 5 /` → `100 ÷ 10 ÷ 5` (left-to-right, no parens)
   - `100 10 5 / /` → `100 ÷ ( 10 ÷ 5 )` (parens on right)

3. **LaTeX Command:** Uses `\div` (backslash-div). Must escape in Go: `"\\div"`.

4. **Comparison with Multiplication:** Division mirrors multiplication's precedence but adds the right-associativity constraint because `/` is non-commutative.

### Test Cases for Feature

```
Input: "10 2 /"
Expected: "$10 \\div 2$"

Input: "100 10 / 5 / 2 /"
Expected: "$100 \\div 10 \\div 5 \\div 2$"
(Tests left-associativity at same precedence)

Input: "10 2 / 5 *"
Expected: "$10 \\div 2 \\times 5$"
(Tests equal precedence with different operators)

Input: "100 10 5 / /"
Expected: "$100 \\div ( 10 \\div 5 )$"
(Tests right-associativity constraint)
Stack trace:
  1. NUMBER 100 → stack: [N100]
  2. NUMBER 10 → stack: [N100, N10]
  3. NUMBER 5 → stack: [N100, N10, N5]
  4. DIV → pop 5, pop 10 → stack: [N100, B1("/", N10, N5)]
  5. DIV → pop B1, pop N100 → stack: [B2("/", N100, B1)]
  6. EOF → return B2
  LaTeX gen: visit(B2) where operator = "/"
    → left = "100"
    → right = visit(B1) = "10 \\div 5"
    → needsParens(B1, 2, true) = true (child_prec == parent_prec && is_right && operator = "/")
    → right = "( 10 \\div 5 )"
    → result = "100 \\div ( 10 \\div 5 )"
```

### Dependency Graph
- **Depends on:** Numbers, Lexer, Parser (BinaryOp)
- **Depended on by:** Precedence

---

## Feature: Precedence

### Feature Scope
Handle operator precedence in LaTeX output by inserting parentheses where needed.

### Cross-Module Implementation

#### 1. latex_gen.py (Core Implementation)
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2,
}

def _needs_parens(
    self, child: Expr, parent_precedence: int, *, is_right: bool
) -> bool:
    """Determine if a child expression needs parentheses.

    Parentheses are needed when:
    1. Child has lower precedence than parent
    2. Child has equal precedence and is on the right side
       (for left-associative operators like -)
    """
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

@_visit.register
def _visit_binary_op(self, node: BinaryOp) -> str:
    """Generate LaTeX for a binary operation."""
    op_latex = self.BINARY_OPS[node.operator]
    my_precedence = self.PRECEDENCE[node.operator]

    # Generate left operand, adding parens if needed
    left = self._visit(node.left)
    if self._needs_parens(node.left, my_precedence, is_right=False):
        left = f"( {left} )"

    # Generate right operand, adding parens if needed
    right = self._visit(node.right)
    if self._needs_parens(node.right, my_precedence, is_right=True):
        right = f"( {right} )"

    return f"{left} {op_latex} {right}"
```

**Key Points:**
- Precedence is defined at the class level as a constant dictionary
- `_needs_parens` is a helper method called during LaTeX generation
- Parentheses are determined by two rules:
  1. If child operator has lower precedence than parent, add parens
  2. If child and parent have equal precedence AND child is on the right side AND operator is non-commutative (-, /), add parens

#### 2. ast_nodes.py (No direct role, but critical structure)
The precedence logic depends on:
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str  # Used to look up precedence
    left: Expr     # Checked with is_right=False
    right: Expr    # Checked with is_right=True
```

### Go Migration Strategy

#### Type Mapping

| Python | Go | Notes |
|--------|-----|-------|
| `PRECEDENCE` dict | Map or switch statement | `map[string]int` in Go |
| `_needs_parens` method | `needsParens` function | Separate function or method on generator |
| `is_right` parameter | `isRight` boolean parameter | Same logic |
| `isinstance(child, BinaryOp)` | Type assertion | `child, ok := expr.(*BinaryOp)` |

#### Implementation Pattern (Go)

```go
var precedence = map[string]int{
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2,
}

type Generator struct {
    // ...
}

func (g *Generator) visitBinaryOp(node *BinaryOp) string {
    opLatex := binaryOps[node.Operator]
    myPrec := precedence[node.Operator]

    left := g.visit(node.Left)
    if g.needsParens(node.Left, myPrec, false) {
        left = "( " + left + " )"
    }

    right := g.visit(node.Right)
    if g.needsParens(node.Right, myPrec, true) {
        right = "( " + right + " )"
    }

    return left + " " + opLatex + " " + right
}

func (g *Generator) needsParens(child Expr, parentPrec int, isRight bool) bool {
    binOp, ok := child.(*BinaryOp)
    if !ok {
        return false
    }

    childPrec := precedence[binOp.Operator]

    // Lower precedence always needs parens
    if childPrec < parentPrec {
        return true
    }

    // Equal precedence on right side needs parens for non-commutative operators
    if childPrec == parentPrec && isRight {
        if binOp.Operator == "-" || binOp.Operator == "/" {
            return true
        }
    }

    return false
}
```

#### Key Implementation Notes

1. **Precedence Table:** The mapping of operators to precedence levels is fundamental:
   - Addition/Subtraction: level 1 (lower, looser binding)
   - Multiplication/Division: level 2 (higher, tighter binding)

2. **Two-Part Check:** The `_needs_parens` logic has two parts:
   - **Part 1 (Line 171):** If child's precedence is strictly less than parent's, add parens
   - **Part 2 (Lines 176-180):** If precedences are equal AND child is on the right AND operator is non-commutative, add parens

3. **Non-Commutative Operators:** Only `-` and `/` trigger the right-side rule because:
   - Addition and multiplication are commutative (order doesn't matter)
   - Subtraction and division are not commutative (order matters)

4. **Non-Binary Children:** If the child is not a `BinaryOp` (e.g., it's a `Number`), no parentheses are needed.

5. **Spacing:** Parentheses are formatted with spaces inside: `"( ... )"` not `"(...)"`.

### Test Cases for Feature

This feature is thoroughly tested by the test cases in other features. Key scenarios:

```
Scenario 1: Lower precedence child (addition under multiplication)
Input: "5 3 + 2 *"
Rule: child_prec (1) < parent_prec (2) → add parens
Result: "$( 5 + 3 ) \\times 2$"

Scenario 2: Equal precedence, left side (no parens)
Input: "5 3 - 2 -"
Rule: child_prec == parent_prec, is_right=false → no parens
Result: "$5 - 3 - 2$"

Scenario 3: Equal precedence, right side, non-commutative (parens required)
Input: "5 3 2 - -"
Rule: child_prec == parent_prec, is_right=true, operator="-" → add parens
Result: "$5 - ( 3 - 2 )$"

Scenario 4: Mixed operators with equal precedence
Input: "10 2 / 5 *"
Rule: child_prec == parent_prec, is_right=false (left side) → no parens
Result: "$10 \\div 2 \\times 5$"

Scenario 5: Nested with multiple precedence levels
Input: "10 2 / 3 + 4 *"
Stack: N10, N2 → B1("/"), N3 → B2("+"), N4 → B3("*")
Tree: BinaryOp("*", BinaryOp("+", BinaryOp("/", 10, 2), 3), 4)
LaTeX:
  - Visit B3("*", B2("+", B1("/", 10, 2), 3), 4)
  - left = visit(B2)
    - left = visit(B1) = "10 \\div 2" (no parens, higher precedence)
    - right = "3"
    - needsParens(B1, 1, false) = true (prec 2 > parent prec 1) NO! WRONG!
    - Actually: parent = "+", child = "/", child_prec=2 > parent_prec=1, so NO parens
    - Result: "10 \\div 2 + 3"
  - needsParens(B2, 2, false) = true (prec 1 < parent prec 2)
  - left = "( 10 \\div 2 + 3 )"
  - right = "4"
  - Result: "$( 10 \\div 2 + 3 ) \\times 4$"
```

### Dependency Graph
- **Depends on:** All binary operators (Addition, Subtraction, Multiplication, Division)
- **Depends on:** AST structure with `BinaryOp` nodes
- **No dependers:** Precedence is the final feature in the pipeline

---

## Feature Dependencies

### Dependency Graph

```
Numbers (foundational)
    ↓
Lexer, Parser, AST (required by all operators)
    ↓
Addition ──┐
Subtraction│
           ├→ Precedence (final synthesis)
Multiplication
           │
Division ──┘

All Features → LaTeX Generation → CLI Output
```

### Dependency Matrix

| Feature | Depends On | Required By |
|---------|-----------|------------|
| Numbers | None | All operators, LaTeX gen |
| Addition | Numbers, Lexer, Parser, BinaryOp | Precedence, Tests |
| Subtraction | Numbers, Lexer, Parser, BinaryOp, Lookahead | Precedence, Tests |
| Multiplication | Numbers, Lexer, Parser, BinaryOp | Precedence, Tests |
| Division | Numbers, Lexer, Parser, BinaryOp | Precedence, Tests |
| Precedence | All operators, BinaryOp | LaTeX gen, Tests |
| LaTeX Gen | All features | CLI, Output |
| CLI | All features | Main entry point |

### Implementation Order

Recommended feature implementation order for Go migration:

1. **Numbers** - Foundational; no dependencies
2. **Lexer & Parser Infrastructure** - Set up token handling and RPN stack
3. **Addition** - Simplest operator; validates lexer/parser/AST flow
4. **Subtraction** - Adds complexity with lookahead and right-associativity
5. **Multiplication** - Introduces higher precedence level
6. **Division** - Mirrors multiplication; completes operator set
7. **Precedence** - Ties all operators together; tested by existing test cases
8. **LaTeX Generation** - Uses all features; final output formatting
9. **CLI** - Orchestrates entire pipeline

---

## Go Idioms and Patterns

### Error Handling

#### Python Pattern

```python
class LexerError(Exception):
    def __init__(self, message: str, line: int, column: int) -> None:
        super().__init__(f"Line {line}, column {column}: {message}")
        self.message = message
        self.line = line
        self.column = column

# Usage
try:
    tokens = lexer.tokenize()
except LexerError as e:
    formatter.format_error(e.message, e.line, e.column)
```

#### Go Pattern (Idiomatic)

```go
// Option 1: Custom error type (recommended for detailed errors)
type LexerError struct {
    Message string
    Line    int
    Column  int
}

func (e *LexerError) Error() string {
    return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
}

// Usage
tokens, err := lexer.Tokenize()
if err != nil {
    if lexErr, ok := err.(*LexerError); ok {
        formatted := formatter.FormatError(lexErr.Message, lexErr.Line, lexErr.Column)
        fmt.Fprintln(os.Stderr, formatted)
    }
    return 1
}

// Option 2: errors.New with fmt.Errorf (simpler)
return fmt.Errorf("line %d, column %d: %w", line, column, err)
```

**Key Go Idiom:** Use named return values with error as the last return value:
```go
func (l *Lexer) Tokenize() ([]Token, error) {
    // ...
    if err != nil {
        return nil, &LexerError{Message: "...", Line: l.line, Column: l.column}
    }
    return tokens, nil
}
```

### Type System: Enums

#### Python Pattern

```python
from enum import Enum, auto

class TokenType(Enum):
    NUMBER = auto()
    PLUS = auto()
    EOF = auto()

# Usage
if token.type == TokenType.NUMBER:
    # ...
```

#### Go Pattern

```go
// Option 1: iota with const block (most idiomatic)
type TokenType int

const (
    TokenNumber TokenType = iota
    TokenPlus
    TokenMinus
    TokenMult
    TokenDiv
    TokenEOF
)

func (t TokenType) String() string {
    switch t {
    case TokenNumber:
        return "NUMBER"
    case TokenPlus:
        return "PLUS"
    default:
        return "UNKNOWN"
    }
}

// Usage
if token.Type == TokenNumber {
    // ...
}

// Option 2: String-based (for flexibility, but less performant)
type TokenType string

const (
    TokenNumber TokenType = "NUMBER"
    TokenPlus   TokenType = "PLUS"
)
```

**Recommendation:** Use `iota` with integer constants for performance. Implement `String()` method for debugging.

### Data Structures: Immutable AST

#### Python Pattern

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class Number(ASTNode):
    value: str
    # Immutable, hashable
```

#### Go Pattern

```go
// Option 1: Struct with unexported fields (prevents accidental modification)
type Number struct {
    line   int
    column int
    value  string
}

func NewNumber(line, column int, value string) *Number {
    return &Number{line: line, column: column, value: value}
}

// Accessors if needed
func (n *Number) Value() string { return n.value }

// Option 2: Just use structs with capitalized fields (Go convention)
type Number struct {
    Line   int
    Column int
    Value  string
}

// Make receiver read-only by not modifying in methods
func (n *Number) String() string {
    return fmt.Sprintf("Number(%d, %d, %q)", n.Line, n.Column, n.Value)
}
```

**Recommendation:** Use structs with capitalized fields (Go idiom). Immutability is conventional rather than enforced in Go. Use receiver methods (not pointers) for read-only operations.

### Visitor Pattern

#### Python Pattern

```python
from functools import singledispatchmethod

class Generator:
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

#### Go Pattern (Type Assertion/Switch)

```go
// Option 1: Type assertion with switch (most Go-like)
func (g *Generator) Visit(expr Expr) string {
    switch n := expr.(type) {
    case *Number:
        return n.Value
    case *BinaryOp:
        return g.visitBinaryOp(n)
    default:
        panic(fmt.Sprintf("no visitor for %T", expr))
    }
}

func (g *Generator) visitBinaryOp(node *BinaryOp) string {
    // ...
}

// Option 2: Interface with methods (alternative)
type Expr interface {
    Accept(v *Generator) string
}

func (n *Number) Accept(v *Generator) string {
    return v.visitNumber(n)
}

func (b *BinaryOp) Accept(v *Generator) string {
    return v.visitBinaryOp(b)
}

// Usage
result := expr.Accept(generator)
```

**Recommendation:** Use type assertion with switch for simplicity. The interface approach is more extensible but adds more boilerplate.

### Stack-Based Parsing

#### Python Pattern

```python
def parse(self) -> Expr:
    stack: list[Expr] = []

    while not self._at_end():
        token = self._current()

        if token.type == TokenType.NUMBER:
            stack.append(Number(...))
        elif token.type == TokenType.PLUS:
            right = stack.pop()
            left = stack.pop()
            stack.append(BinaryOp("+", left, right))

        self._advance()

    return stack[0]
```

#### Go Pattern

```go
func (p *Parser) Parse() (Expr, error) {
    stack := []Expr{}

    for !p.atEnd() {
        token := p.current()

        switch token.Type {
        case TokenNumber:
            node := &Number{
                Line:   token.Line,
                Column: token.Column,
                Value:  token.Value,
            }
            stack = append(stack, node)
            p.advance()

        case TokenPlus:
            if len(stack) < 2 {
                return nil, fmt.Errorf("not enough operands for +")
            }
            right := stack[len(stack)-1]
            stack = stack[:len(stack)-1]
            left := stack[len(stack)-1]
            stack = stack[:len(stack)-1]

            node := &BinaryOp{
                Line:     token.Line,
                Column:   token.Column,
                Operator: "+",
                Left:     left,
                Right:    right,
            }
            stack = append(stack, node)
            p.advance()
        }
    }

    if len(stack) != 1 {
        return nil, fmt.Errorf("invalid RPN")
    }
    return stack[0], nil
}
```

**Key Go Idioms:**
- Slices as stacks: `append(slice, item)` for push, `slice[:len(slice)-1]` for pop
- Always return `(value, error)` tuple
- Check error immediately: `if len(stack) < 2 { return nil, err }`

### String Building

#### Python Pattern

```python
return f"{left} {op_latex} {right}"
```

#### Go Pattern

```go
// Option 1: String concatenation (simple, acceptable for small strings)
return left + " " + opLatex + " " + right

// Option 2: fmt.Sprintf (more flexible)
return fmt.Sprintf("%s %s %s", left, opLatex, right)

// Option 3: strings.Builder (most efficient for large strings)
var sb strings.Builder
sb.WriteString(left)
sb.WriteString(" ")
sb.WriteString(opLatex)
sb.WriteString(" ")
sb.WriteString(right)
return sb.String()
```

**Recommendation:** Use simple concatenation for this project (strings are small). Use `strings.Builder` if generating large LaTeX documents.

---

## Error Handling Strategy

### Error Types Required

1. **LexerError** - Invalid characters or token format
   - Attributes: message, line, column
   - Usage: `fmt.Errorf("Line %d, column %d: %s", line, column, message)`

2. **ParserError** - Invalid RPN structure
   - Attributes: message, line, column (from token)
   - Usage: `fmt.Errorf("Not enough operands for '%s' at line %d, column %d", op, line, col)`

3. **ErrorFormatter** - Context-aware error display
   - Attributes: source text
   - Method: `FormatError(message string, line int, column int) string`

### Error Flow

```
Input Text
    ↓
Lexer (may return LexerError)
    ↓
Parser (may return ParserError)
    ↓
Generator (unlikely to error)
    ↓
CLI (catches errors, formats with ErrorFormatter, writes to stderr, returns exit code 1)
```

### Go Implementation Pattern

```go
package main

import (
    "fmt"
    "os"
)

type LexerError struct {
    Message string
    Line    int
    Column  int
}

func (e *LexerError) Error() string {
    return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
}

type ParserError struct {
    Message string
    Line    int
    Column  int
}

func (e *ParserError) Error() string {
    return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
}

type ErrorFormatter struct {
    source string
    lines  []string
}

func NewErrorFormatter(source string) *ErrorFormatter {
    return &ErrorFormatter{
        source: source,
        lines:  strings.Split(source, "\n"),
    }
}

func (f *ErrorFormatter) FormatError(message string, line int, column int) string {
    // ... implementation
}

// In CLI main function
func main() {
    // ...
    tokens, err := lexer.Tokenize()
    if err != nil {
        formatted := formatter.FormatError(err.Error(), err.Line, err.Column)
        fmt.Fprintln(os.Stderr, formatted)
        os.Exit(1)
    }

    ast, err := parser.Parse()
    if err != nil {
        // Similar error handling
        os.Exit(1)
    }
    // ...
}
```

---

## Migration Checklist

### Phase 1: Core Infrastructure

- [ ] Set up Go project structure
  - [ ] `main.go` entry point
  - [ ] `tokens/tokens.go` - TokenType enum and Token struct
  - [ ] `lexer/lexer.go` - Lexer class and LexerError
  - [ ] `parser/parser.go` - Parser class and ParserError
  - [ ] `ast/ast.go` - AST node types (ASTNode, Number, BinaryOp)
  - [ ] `generator/generator.go` - LaTeX generator
  - [ ] `errors/errors.go` - ErrorFormatter
  - [ ] `cli/cli.go` - CLI orchestration

- [ ] Implement TokenType enum with iota
  - [ ] NUMBER
  - [ ] PLUS, MINUS, MULT, DIV
  - [ ] EOF
  - [ ] String() method for debugging

- [ ] Implement Token struct
  - [ ] Type, Value, Line, Column fields
  - [ ] Proper struct initialization

### Phase 2: Feature - Numbers

- [ ] Lexer: Implement `Tokenize()` method
  - [ ] Character-by-character scanning
  - [ ] `_skipWhitespace()` helper
  - [ ] `_peek()` and `_advance()` helpers
  - [ ] `_atEnd()` check

- [ ] Lexer: Implement `scanNumber()` method
  - [ ] Integer part scanning
  - [ ] Decimal point handling
  - [ ] Negative number detection (lookahead for `-`)

- [ ] Parser: Implement `Parse()` method
  - [ ] Stack-based RPN parsing
  - [ ] NUMBER token handling (push onto stack)
  - [ ] EOF handling
  - [ ] Stack validation

- [ ] Generator: Implement `visitNumber()` method
  - [ ] Simple pass-through of Number.Value

- [ ] Test Numbers
  - [ ] `"5"` → `"$5$"`
  - [ ] `"3.14"` → `"$3.14$"`

### Phase 3: Feature - Addition

- [ ] Lexer: Add `+` character recognition
  - [ ] Return TokenPlus

- [ ] Parser: Add PLUS token handling
  - [ ] Pop two operands from stack
  - [ ] Create BinaryOp("+", left, right)
  - [ ] Push result back onto stack

- [ ] Generator: Add `"+"` to BINARY_OPS map
  - [ ] Map `"+"` to `"+"`
  - [ ] Add precedence 1

- [ ] Test Addition
  - [ ] `"5 3 +"` → `"$5 + 3$"`
  - [ ] `"1 2 + 3 + 4 +"` → `"$1 + 2 + 3 + 4$"`

### Phase 4: Feature - Subtraction

- [ ] Lexer: Add `-` character handling with lookahead
  - [ ] Check if followed by digit (negative number)
  - [ ] Return TokenMinus or scanNumber("-")

- [ ] Parser: Add MINUS token handling (same as PLUS)
  - [ ] Create BinaryOp("-", left, right)

- [ ] Generator: Add `"-"` to BINARY_OPS and PRECEDENCE
  - [ ] Map `"-"` to `"-"`
  - [ ] Add precedence 1
  - [ ] Add to right-associativity special case in needsParens

- [ ] Test Subtraction
  - [ ] `"5 3 -"` → `"$5 - 3$"`
  - [ ] `"5 3 - 2 -"` → `"$5 - 3 - 2$"`

### Phase 5: Feature - Multiplication

- [ ] Lexer: Add `*` character recognition
  - [ ] Return TokenMult

- [ ] Parser: Add MULT token handling (same as PLUS)
  - [ ] Create BinaryOp("*", left, right)

- [ ] Generator: Add `"*"` to BINARY_OPS and PRECEDENCE
  - [ ] Map `"*"` to `"\\times"` (escape backslash)
  - [ ] Add precedence 2 (higher than addition)

- [ ] Test Multiplication
  - [ ] `"4 7 *"` → `"$4 \\times 7$"`
  - [ ] `"2 3 4 * +"` → `"$2 + 3 \\times 4$"`
  - [ ] `"5 3 + 2 *"` → `"$( 5 + 3 ) \\times 2$"`

### Phase 6: Feature - Division

- [ ] Lexer: Add `/` character recognition
  - [ ] Return TokenDiv

- [ ] Parser: Add DIV token handling (same as MULT)
  - [ ] Create BinaryOp("/", left, right)

- [ ] Generator: Add `"/"` to BINARY_OPS and PRECEDENCE
  - [ ] Map `"/"` to `"\\div"` (escape backslash)
  - [ ] Add precedence 2 (same as multiplication)
  - [ ] Add to right-associativity special case in needsParens

- [ ] Test Division
  - [ ] `"10 2 /"` → `"$10 \\div 2$"`
  - [ ] `"100 10 / 5 / 2 /"` → `"$100 \\div 10 \\div 5 \\div 2$"`
  - [ ] `"10 2 / 5 *"` → `"$10 \\div 2 \\times 5$"`

### Phase 7: Feature - Precedence

- [ ] Generator: Implement `needsParens()` method
  - [ ] Check if child is BinaryOp
  - [ ] Compare child and parent precedence
  - [ ] Implement right-associativity rule for `-` and `/`

- [ ] Generator: Update `visitBinaryOp()` to use needsParens
  - [ ] Call needsParens for left operand (isRight=false)
  - [ ] Call needsParens for right operand (isRight=true)
  - [ ] Wrap in parens `"( " + expr + " )"` if needed

- [ ] Test Precedence
  - [ ] `"5 3 + 2 *"` → `"$( 5 + 3 ) \\times 2$"`
  - [ ] `"2 3 4 + *"` → `"$2 \\times ( 3 + 4 )$"`
  - [ ] `"1 2 + 3 4 + *"` → `"$( 1 + 2 ) \\times ( 3 + 4 )$"`
  - [ ] `"10 2 / 3 + 4 *"` → `"$( 10 \\div 2 + 3 ) \\times 4$"`

### Phase 8: CLI and Integration

- [ ] Implement `ErrorFormatter`
  - [ ] `FormatError()` method
  - [ ] `_getContext()` helper for source extraction
  - [ ] Caret positioning logic

- [ ] Implement CLI main function
  - [ ] Parse command-line arguments (input file, output file)
  - [ ] Handle stdin (`"-"` input)
  - [ ] Read input file with error handling
  - [ ] Call Lexer, Parser, Generator in sequence
  - [ ] Catch LexerError and ParserError
  - [ ] Format errors with ErrorFormatter
  - [ ] Write output to file or stdout
  - [ ] Return proper exit codes

- [ ] Test CLI
  - [ ] All test cases from I/O Contract
  - [ ] Error cases
  - [ ] File I/O
  - [ ] Stdin/stdout

### Phase 9: Testing and Validation

- [ ] Create test files for each feature
  - [ ] `tokens_test.go`
  - [ ] `lexer_test.go`
  - [ ] `parser_test.go`
  - [ ] `generator_test.go`
  - [ ] `integration_test.go`

- [ ] Run all I/O Contract test cases
  - [ ] 21 total test cases
  - [ ] All should pass (except intentionally unimplemented features)

- [ ] Validate error handling
  - [ ] Unexpected character error
  - [ ] Not enough operands error
  - [ ] Invalid RPN error

- [ ] Performance testing (optional)
  - [ ] Ensure no memory leaks
  - [ ] Check parsing speed on large inputs

---

## Summary Table: Feature-by-Feature Implementation

| Feature | Token | Parser Logic | AST Node | LaTeX Output | Precedence | Test Count |
|---------|-------|--------------|----------|--------------|-----------|-----------|
| Numbers | NUMBER | Push stack | Number | Pass-through | N/A | 2 |
| Addition | PLUS | Pop 2, create BinaryOp | BinaryOp | `+` | 1 | 2 |
| Subtraction | MINUS | Pop 2, create BinaryOp | BinaryOp | `-` | 1 | 2 |
| Multiplication | MULT | Pop 2, create BinaryOp | BinaryOp | `\times` | 2 | 4 |
| Division | DIV | Pop 2, create BinaryOp | BinaryOp | `\div` | 2 | 3 |
| Precedence | N/A | N/A | N/A | Use needsParens | 1/2 | 5 |
| **TOTAL** | 5 | All use stack | 1+1 | All generate | Variable | 18 |

---

## References

### Python Source Files Analyzed
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/tokens.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/lexer.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/parser.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/ast_nodes.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/latex_gen.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/errors.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/cli.py`

### I/O Contract Source
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/go-feature-by-feature-3/artifacts/PHASE_0_IO_CONTRACT.md`

### Key Go Resources
- [Effective Go - Error Handling](https://golang.org/doc/effective_go#errors)
- [Effective Go - Interfaces](https://golang.org/doc/effective_go#interfaces_and_types)
- [Go Code Review Comments](https://github.com/golang/go/wiki/CodeReviewComments)

---

**Document Version:** 1.0
**Generated:** 2025-12-30
**Status:** Ready for PHASE 2 (Feature Implementation)
