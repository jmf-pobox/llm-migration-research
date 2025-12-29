# rpn2tex Feature-by-Feature Migration Specification

## Overview

The rpn2tex project is a lexer-parser-generator pipeline that converts Reverse Polish Notation (RPN) mathematical expressions to LaTeX output. The architecture follows a classic compiler pipeline:

1. **Lexer** (lexer.py): Raw text → tokens
2. **Parser** (parser.py): Tokens → Abstract Syntax Tree (AST)
3. **Generator** (latex_gen.py): AST → LaTeX output
4. **CLI** (cli.py): Orchestrates the pipeline with I/O handling
5. **Infrastructure**: Tokens (tokens.py), AST nodes (ast_nodes.py), Error handling (errors.py)

The codebase implements a stack-based RPN parser where:
- Numbers push values onto the stack
- Operators pop two operands, create an AST node, and push the result back
- Output from RPN is converted to infix notation with proper parenthesization

## Infrastructure (Common to All Features)

### Error Handling Framework

**Module:** errors.py

**Public API:**
- `ErrorFormatter` class:
  - `__init__(source: str)`
  - `format_error(message: str, line: int, column: int, *, context_lines: int = 1) -> str`
  - `_get_context(line: int, column: int, context_lines: int) -> str` (private, helper)

**Purpose:** Provides gcc/rustc-style error messages with source context and caret positioning.

**Key Implementation:**
```python
class ErrorFormatter:
    source: str
    lines: list[str]

    def format_error(message, line, column, *, context_lines=1):
        # Returns formatted error with source context
```

**Go Migration Notes:**
- Create a struct with `Source` string and `Lines` []string fields
- Implement methods for formatting errors
- Use string.Builder for efficient string concatenation
- Position tracking is 1-based (line and column)

### Token Layer Infrastructure

**Module:** tokens.py

**Public API:**
- `TokenType` enum:
  - `NUMBER` - Numeric literals
  - `PLUS` - Addition operator
  - `MINUS` - Subtraction operator
  - `MULT` - Multiplication operator
  - `DIV` - Division operator
  - `EOF` - End of input marker

- `Token` dataclass (frozen/immutable):
  - `type: TokenType`
  - `value: str`
  - `line: int` (1-based)
  - `column: int` (1-based)
  - `__repr__() -> str`

**Go Migration Notes:**
- Use `iota` for the enum-like behavior of token types
- Create a Token struct with Type int, Value string, Line int, Column int
- Implement String() method for debugging output
- Consider using const or const block for token types

### AST Node Layer Infrastructure

**Module:** ast_nodes.py

**Public API:**
- `ASTNode` base class (frozen/immutable):
  - `line: int` (1-based)
  - `column: int` (1-based)

- `Number` node (extends ASTNode):
  - `value: str` - String representation of numeric value

- `BinaryOp` node (extends ASTNode):
  - `operator: str` - Operator symbol ("+", "-", "*", "/")
  - `left: Expr` - Left operand expression
  - `right: Expr` - Right operand expression

- Type alias `Expr = Number | BinaryOp`

**Go Migration Notes:**
- All nodes are immutable in Python; Go will use structs
- Consider an interface-based approach with concrete types implementing the interface
- Position info (line, column) is embedded in each node for error reporting
- The Expr type alias becomes an interface in Go

---

## Feature 1: Numbers

### Overview
Numbers are the fundamental building blocks of expressions. They are tokenized as numeric literals (integers or decimals) and represented as leaf nodes in the AST. The feature is dependency-free and serves as the foundation for all other features.

### 1. Feature Boundary Analysis

**Code that belongs to this feature:**
- Token type: `TokenType.NUMBER`
- Token scanning: `Lexer._scan_number()`
- AST node: `Number`
- Generator: `LaTeXGenerator._visit_number()`
- Parser: Number handling in `Parser.parse()` (stack push)

**What this excludes:**
- Operators and their handling
- Operator precedence logic
- Binary operations
- Parenthesization

**Data flow:**
```
Input: "5" or "3.14"
  ↓ Lexer
Token(NUMBER, "5", line, column) or Token(NUMBER, "3.14", ...)
  ↓ Parser
Number(line, column, value="5") or Number(line, column, value="3.14")
  ↓ Generator
"5" or "3.14"
  ↓ CLI
"$5$" or "$3.14$"
```

### 2. Token Layer

**Token Definition (tokens.py):**
- `TokenType.NUMBER` enum value
- Represents both integers and decimals

**Token Examples:**
- `Token(TokenType.NUMBER, "5", 1, 1)` - Integer
- `Token(TokenType.NUMBER, "3.14", 1, 1)` - Decimal
- `Token(TokenType.NUMBER, "-42", 1, 1)` - Negative number

### 3. AST Layer

**Node Type (ast_nodes.py):**
```python
@dataclass(frozen=True)
class Number(ASTNode):
    value: str  # String representation of the number
```

**Construction:**
```python
Number(line=1, column=1, value="5")
Number(line=1, column=3, value="3.14")
```

**Key Property:** The value is stored as a string, not parsed to a numeric type. This preserves formatting (e.g., "3.14" vs "3.140").

### 4. Lexer Layer

**Function:** `Lexer._scan_number(prefix: str, start_line: int, start_column: int) -> Token`

**Location:** lexer.py lines 177-200

**Algorithm:**
1. Initialize value with prefix (usually empty, or "-" for negative numbers)
2. Scan digits before decimal point: `while self._peek().isdigit(): value += self._advance()`
3. Optional decimal point: `if self._peek() == ".": value += self._advance()`
4. Scan digits after decimal point: `while self._peek().isdigit(): value += self._advance()`
5. Return Token(TokenType.NUMBER, value, start_line, start_column)

**Character Classes:**
- Digits: 0-9
- Decimal separator: . (single, optional)

**Entry Point:** Called from `_scan_token()` when current character is a digit or when "-" is followed by a digit

**Negative Numbers:**
- Handled specially: "-" followed immediately by a digit is treated as a negative number
- This distinguishes "-42" (negative number) from "42 -" (subtraction operator followed by number)

### 5. Parser Layer

**Location:** parser.py lines 88-168

**Algorithm:**
```
When token.type == TokenType.NUMBER:
  1. Create Number node: Number(line=token.line, column=token.column, value=token.value)
  2. Push onto stack: stack.append(num_node)
  3. Advance to next token: self._advance()
```

**Stack Operation:**
- Numbers don't pop from the stack; they push
- This is the foundation of the RPN stack-based algorithm

**Error Handling:**
- No errors specific to numbers in the parser
- Lexer handles invalid number formats

### 6. Generator Layer

**Function:** `LaTeXGenerator._visit_number(node: Number) -> str`

**Location:** latex_gen.py lines 100-109

**Implementation:**
```python
@_visit.register
def _visit_number(self, node: Number) -> str:
    return node.value
```

**LaTeX Output:** Simply returns the string value as-is. No formatting or escaping needed.

**Visitor Pattern:** Uses `@singledispatchmethod` to dispatch on node type.

### 7. Error Handling

**Lexer Errors:**
- None specific to valid numbers
- Invalid characters trigger `LexerError`

**Parser Errors:**
- None specific to numbers (they're always valid when tokenized)

**Generator Errors:**
- Not applicable to numbers

### 8. Go Migration Notes

**Type Mappings:**
- `str` (Python value field) → `string` (Go)
- Position tracking: `int` → `int` in both languages
- Immutability: Use struct with unexported fields

**Idiomatic Go Patterns:**
```go
// Token type as iota constant
const (
    TokenNumber TokenType = iota
    TokenPlus
    // ...
)

// Token struct
type Token struct {
    Type   TokenType
    Value  string
    Line   int
    Column int
}

// Number node
type Number struct {
    Line   int
    Column int
    Value  string
}

// Visitor pattern
func (gen *LaTeXGenerator) VisitNumber(node *Number) string {
    return node.Value
}
```

**Special Considerations:**
- No need for frozen/immutable types in Go; document immutability requirements
- String representation is simple: no escaping or conversion needed
- Handle floating-point representation preservation (store as string, not float64)

**Test Cases to Verify:**
- Single integer: "5" → "$5$"
- Decimal: "3.14" → "$3.14$"
- Large numbers
- Numbers with many decimal places

---

## Feature 2: Addition

### Overview
Addition is the simplest binary operator feature. It demonstrates how operators work in RPN and provides the foundation for understanding other binary operators and precedence handling.

### 1. Feature Boundary Analysis

**Code that belongs to this feature:**
- Token type: `TokenType.PLUS`
- Token recognition: `Lexer._scan_token()` "+" character
- AST construction: `Parser.parse()` when PLUS token encountered
- Operator representation: `BinaryOp` with operator="+"
- LaTeX generation: Mapping from "+" to "+" in output
- Precedence level: 1 (same as subtraction, lower than multiplication/division)

**What this excludes:**
- Other operators
- Precedence handling between different operator types
- Right-associativity or left-associativity concerns (addition is commutative)

**Data flow:**
```
Input: "5 3 +"
  ↓ Lexer
[Token(NUMBER, "5"), Token(NUMBER, "3"), Token(PLUS, "+"), Token(EOF, "")]
  ↓ Parser (RPN stack processing)
Stack: [Number(5)] → [Number(5), Number(3)] → [BinaryOp("+", Number(5), Number(3))]
  ↓ Generator
"5 + 3"
  ↓ CLI
"$5 + 3$"
```

### 2. Token Layer

**Token Definition (tokens.py):**
- `TokenType.PLUS` enum value

**Token Examples:**
- `Token(TokenType.PLUS, "+", 1, 5)`

**Lexer Recognition (lexer.py):**
```python
if char == "+":
    self._advance()
    return Token(TokenType.PLUS, "+", start_line, start_column)
```

### 3. AST Layer

**Node Type (ast_nodes.py):**
Already covered in "Numbers" section - BinaryOp with operator="+":
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str  # "+"
    left: Expr
    right: Expr
```

**Construction Example:**
```python
BinaryOp(
    line=1, column=5,
    operator="+",
    left=Number(1, 1, "5"),
    right=Number(1, 3, "3")
)
```

### 4. Lexer Layer

**Character Recognition:** Single "+" character

**Implementation (lexer.py lines 150-152):**
```python
if char == "+":
    self._advance()
    return Token(TokenType.PLUS, "+", start_line, start_column)
```

**Position Tracking:** Column is incremented by _advance()

### 5. Parser Layer

**Location:** parser.py lines 115-147

**Algorithm for Addition:**
```
When token.type == TokenType.PLUS:
  1. Check stack has at least 2 operands: len(stack) >= 2
  2. Pop right operand: right = stack.pop()
  3. Pop left operand: left = stack.pop()
  4. Get operator string from token type: operator = "+"
  5. Create BinaryOp node: op_node = BinaryOp(
       line=token.line,
       column=token.column,
       operator="+",
       left=left,
       right=right
     )
  6. Push onto stack: stack.append(op_node)
  7. Advance: self._advance()
```

**Stack Behavior (RPN):**
```
"1 2 + 3 +"  RPN → (1 + 2) + 3 infix
Stack evolution:
  Token(NUMBER, "1") → [Number(1)]
  Token(NUMBER, "2") → [Number(1), Number(2)]
  Token(PLUS, "+")   → [BinaryOp("+", Number(1), Number(2))]
  Token(NUMBER, "3") → [BinaryOp(...), Number(3)]
  Token(PLUS, "+")   → [BinaryOp("+", BinaryOp("+", 1, 2), Number(3))]
```

**Error Handling:**
- `ParserError` if less than 2 operands on stack
- Message: "Operator '+' requires two operands"

### 6. Generator Layer

**Function:** `LaTeXGenerator._visit_binary_op(node: BinaryOp) -> str` (shared)

**Location:** latex_gen.py lines 112-141

**For Addition Operator:**
```python
# In _visit_binary_op:
op_latex = self.BINARY_OPS["+"]  # Result: "+"
# Left and right operands processed recursively
# Parentheses added based on precedence
```

**Operator Mapping (latex_gen.py lines 47-52):**
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "+": "+",  # Addition
    "-": "-",
    "*": r"\times",
    "/": r"\div",
}
```

**Precedence Mapping (latex_gen.py lines 57-62):**
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,  # Addition
    "-": 1,
    "*": 2,
    "/": 2,
}
```

**Output Example:**
```
BinaryOp("+", Number("5"), Number("3"))
→ "5 + 3"
```

**Parenthesization:** Addition has precedence level 1. Lower precedence operations as children need parentheses, but not other addition/subtraction (see Feature 6 for details).

### 7. Error Handling

**Lexer:** No errors for "+" character

**Parser:**
- `ParserError` if insufficient operands on stack
- Error location points to the "+" token

**Generator:** No errors for addition

### 8. Go Migration Notes

**Type Mappings:**
- Operator string "+" remains string in Go

**Idiomatic Go Pattern:**
```go
// Token recognition in lexer
case '+':
    lex.Advance()
    return &Token{
        Type:   TokenPlus,
        Value:  "+",
        Line:   startLine,
        Column: startColumn,
    }

// Parser - constant for operator types
const OpAdd = "+"

// AST node construction in parser
op := &BinaryOp{
    Line:     token.Line,
    Column:   token.Column,
    Operator: OpAdd,
    Left:     left,
    Right:    right,
}

// LaTeX generation
var LatexOps = map[string]string{
    "+": "+",
    "-": "-",
    "*": r"\times",
    "/": r"\div",
}

var Precedence = map[string]int{
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2,
}
```

**Special Considerations:**
- Addition is commutative; parenthesization only depends on precedence, not associativity
- Multiple additions parse as left-associative due to RPN stack semantics
- String representation "+", not a symbol or special character

**Test Cases to Verify:**
- Simple addition: "5 3 +" → "$5 + 3$"
- Multiple additions: "1 2 + 3 + 4 +" → "$1 + 2 + 3 + 4$"
- Addition with mixed operators (test precedence later)

---

## Feature 3: Subtraction

### Overview
Subtraction is similar to addition but is NOT commutative. It demonstrates non-commutative operator handling and introduces precedence concerns for equal-precedence operators (handling left-associativity of repeated subtractions).

### 1. Feature Boundary Analysis

**Code that belongs to this feature:**
- Token type: `TokenType.MINUS`
- Token recognition: `Lexer._scan_token()` "-" character (with ambiguity handling)
- AST construction: `Parser.parse()` when MINUS token encountered
- Operator representation: `BinaryOp` with operator="-"
- LaTeX generation: Mapping from "-" to "-"
- Precedence level: 1 (same as addition)

**Unique complexity:** "-" has dual meaning:
- As an operator: subtraction (e.g., "5 3 -")
- As part of a number: negative literal (e.g., "-42")

**Data flow:**
```
Input: "5 3 -"
  ↓ Lexer
[Token(NUMBER, "5"), Token(NUMBER, "3"), Token(MINUS, "-"), Token(EOF, "")]
  ↓ Parser
Stack: [Number(5)] → [Number(5), Number(3)] → [BinaryOp("-", Number(5), Number(3))]
  ↓ Generator
"5 - 3"
  ↓ CLI
"$5 - 3$"
```

### 2. Token Layer

**Token Definition (tokens.py):**
- `TokenType.MINUS` enum value

**Token Examples:**
- `Token(TokenType.MINUS, "-", 1, 5)` - Operator
- Negative numbers use `TokenType.NUMBER` with value starting with "-": `Token(TokenType.NUMBER, "-42", 1, 1)`

### 3. AST Layer

**Node Type:** Same as addition - `BinaryOp` with operator="-"

**Construction Example:**
```python
BinaryOp(
    line=1, column=5,
    operator="-",
    left=Number(1, 1, "5"),
    right=Number(1, 3, "3")
)
```

### 4. Lexer Layer

**Character Recognition:** Single "-" character with ambiguity resolution

**Implementation (lexer.py lines 153-162):**
```python
if char == "-":
    # Could be negative number or subtraction operator
    # In RPN, standalone "-" is always subtraction
    # Negative numbers are written as "0 5 -" or handled specially
    self._advance()
    # Check if this is a negative number (digit follows immediately)
    if not self._at_end() and self._peek().isdigit():
        # It's a negative number
        return self._scan_number("-", start_line, start_column)
    return Token(TokenType.MINUS, "-", start_line, start_column)
```

**Ambiguity Resolution Logic:**
- If "-" is immediately followed by a digit (no whitespace), treat as negative number
- Otherwise, treat as subtraction operator
- This works because RPN notation separates tokens with whitespace

**Position Tracking:** Column is incremented by _advance()

### 5. Parser Layer

**Location:** parser.py lines 115-147 (shared with all binary operators)

**Algorithm for Subtraction:** Identical to addition, except operator string is "-"

```
When token.type == TokenType.MINUS:
  1. Check stack has at least 2 operands
  2. Pop right operand: right = stack.pop()
  3. Pop left operand: left = stack.pop()
  4. Create BinaryOp("-", left, right)
  5. Push onto stack
  6. Advance
```

**Important RPN Semantic:** In RPN, "5 3 -" means (5 - 3), NOT (3 - 5)
- First popped operand (3) becomes right operand
- Second popped operand (5) becomes left operand

**Stack Behavior:**
```
"5 3 - 2 -"  RPN → (5 - 3) - 2 = 0 infix
Stack evolution:
  Token(NUMBER, "5") → [Number(5)]
  Token(NUMBER, "3") → [Number(5), Number(3)]
  Token(MINUS, "-")  → [BinaryOp("-", Number(5), Number(3))]
  Token(NUMBER, "2") → [BinaryOp(...), Number(2)]
  Token(MINUS, "-")  → [BinaryOp("-", BinaryOp(...), Number(2))]
Result structure: (5 - 3) - 2, which is left-associative
```

**Error Handling:**
- Same as addition: `ParserError` if insufficient operands

### 6. Generator Layer

**Operator Mapping:**
```python
BINARY_OPS["-"] = "-"
PRECEDENCE["-"] = 1
```

**Parenthesization - Critical for Subtraction:**

Subtraction requires special handling for right-associativity. The `_needs_parens()` method (lines 143-180) handles this:

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

**Why:** Right operand of subtraction needs parentheses if it's also a subtraction or division (non-commutative operators at same precedence level)

**Example:**
```
Input: "5 3 2 - -"  RPN
AST: BinaryOp("-", Number(5), BinaryOp("-", Number(3), Number(2)))
Represents: 5 - (3 - 2) = 4
Output: "5 - ( 3 - 2 )"

Without parens "5 - 3 - 2" would be interpreted as (5 - 3) - 2 = 0 (wrong!)
```

### 7. Error Handling

**Lexer:** No errors for "-" when recognized as operator

**Parser:**
- `ParserError` if insufficient operands
- Error location points to the "-" token

**Generator:** No errors for subtraction

### 8. Go Migration Notes

**Type Mappings:**
- Same as addition, but operator constant is "-"

**Idiomatic Go Pattern:**
```go
// Ambiguity resolution in lexer
case '-':
    lex.Advance()
    if !lex.AtEnd() && unicode.IsDigit(rune(lex.Peek())) {
        // Negative number
        return lex.ScanNumber("-", startLine, startColumn)
    }
    return &Token{
        Type:   TokenMinus,
        Value:  "-",
        Line:   startLine,
        Column: startColumn,
    }

// Parser handles same way as addition
// Generator needs special handling:
var NonCommutativeOps = map[string]bool{
    "-": true,
    "/": true,
}
```

**Special Considerations:**
- Negative number detection requires lookahead
- Parenthesization logic is critical for correctness
- Must preserve evaluation order (5 - 3 - 2) → ((5 - 3) - 2) with proper parens in output

**Test Cases to Verify:**
- Simple subtraction: "5 3 -" → "$5 - 3$"
- Multiple subtractions: "5 3 - 2 -" → "$5 - 3 - 2$"
- Right-associative case: "5 3 2 - -" → "$5 - ( 3 - 2 )$"
- Mixed with addition (test Feature 6)

---

## Feature 4: Multiplication

### Overview
Multiplication is a higher-precedence binary operator. It demonstrates precedence-based parenthesization and operator precedence levels. Multiplication is commutative like addition but has higher precedence than addition/subtraction.

### 1. Feature Boundary Analysis

**Code that belongs to this feature:**
- Token type: `TokenType.MULT`
- Token recognition: `Lexer._scan_token()` "*" character
- AST construction: `Parser.parse()` when MULT token encountered
- Operator representation: `BinaryOp` with operator="*"
- LaTeX generation: Mapping from "*" to `\times`
- Precedence level: 2 (higher than addition/subtraction)

**What this introduces:**
- First demonstration of precedence-based parenthesization
- Multiplication in LaTeX uses `\times` symbol, not "*"

**Data flow:**
```
Input: "4 7 *"
  ↓ Lexer
[Token(NUMBER, "4"), Token(NUMBER, "7"), Token(MULT, "*"), Token(EOF, "")]
  ↓ Parser
Stack: [Number(4)] → [Number(4), Number(7)] → [BinaryOp("*", Number(4), Number(7))]
  ↓ Generator
"4 \times 7"
  ↓ CLI
"$4 \times 7$"
```

### 2. Token Layer

**Token Definition (tokens.py):**
- `TokenType.MULT` enum value

**Token Examples:**
- `Token(TokenType.MULT, "*", 1, 5)`

**Lexer Recognition (lexer.py lines 163-165):**
```python
if char == "*":
    self._advance()
    return Token(TokenType.MULT, "*", start_line, start_column)
```

### 3. AST Layer

**Node Type:** `BinaryOp` with operator="*"

**Construction Example:**
```python
BinaryOp(
    line=1, column=5,
    operator="*",
    left=Number(1, 1, "4"),
    right=Number(1, 3, "7")
)
```

### 4. Lexer Layer

**Character Recognition:** Single "*" character (no ambiguity)

**Implementation:** Straightforward single-character operator

### 5. Parser Layer

**Algorithm:** Identical to addition and subtraction, operator string is "*"

```
When token.type == TokenType.MULT:
  Create BinaryOp("*", left, right) and push to stack
```

### 6. Generator Layer

**Operator Mapping (latex_gen.py):**
```python
BINARY_OPS["*"] = r"\times"  # LaTeX multiplication symbol
PRECEDENCE["*"] = 2          # Higher precedence than addition
```

**Output with Precedence:**

The key feature of multiplication is how it interacts with addition/subtraction:

```
Input RPN: "2 3 4 * +"
AST: BinaryOp("+", Number(2), BinaryOp("*", Number(3), Number(4)))
Output: "2 + 3 \times 4"
Reason: Multiplication has precedence 2, addition has precedence 1
        The multiplication child of addition doesn't need parentheses
        because higher precedence binds tighter
```

```
Input RPN: "5 3 + 2 *"
AST: BinaryOp("*", BinaryOp("+", Number(5), Number(3)), Number(2))
Output: "( 5 + 3 ) \times 2"
Reason: Addition child has precedence 1, multiplication parent has precedence 2
        Lower precedence child of higher-precedence parent needs parentheses
```

### 7. Error Handling

**Lexer:** No errors for "*" character

**Parser:**
- `ParserError` if insufficient operands

**Generator:** No errors for multiplication

### 8. Go Migration Notes

**Type Mappings:**
- Operator string "*" → string "\\times" in LaTeX output

**Idiomatic Go Pattern:**
```go
// Lexer
case '*':
    lex.Advance()
    return &Token{
        Type:   TokenMult,
        Value:  "*",
        Line:   startLine,
        Column: startColumn,
    }

// Generator
var LatexOps = map[string]string{
    "+": "+",
    "-": "-",
    "*": "\\times",
    "/": "\\div",
}

var Precedence = map[string]int{
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2,
}
```

**Special Considerations:**
- LaTeX output uses `\times`, not "*"
- Precedence level 2 is critical for parenthesization logic
- Commutative operator (but left-associative parsing from RPN)

**Test Cases to Verify:**
- Simple multiplication: "4 7 *" → "$4 \\times 7$"
- Multiplication with addition: "2 3 4 * +" → "$2 + 3 \\times 4$"
- Addition then multiplication: "5 3 + 2 *" → "$( 5 + 3 ) \\times 2$"

---

## Feature 5: Division

### Overview
Division is a higher-precedence operator like multiplication, but is non-commutative (like subtraction). It combines the precedence behavior of multiplication with the associativity concerns of subtraction. Division maps to the `\div` LaTeX symbol.

### 1. Feature Boundary Analysis

**Code that belongs to this feature:**
- Token type: `TokenType.DIV`
- Token recognition: `Lexer._scan_token()` "/" character
- AST construction: `Parser.parse()` when DIV token encountered
- Operator representation: `BinaryOp` with operator="/"
- LaTeX generation: Mapping from "/" to `\div`
- Precedence level: 2 (same as multiplication)
- Non-commutative: Right-associativity special handling

**Unique aspects:**
- Highest-level introduction to non-commutative, higher-precedence operators
- Right operand of division at same precedence needs parentheses (like subtraction)

**Data flow:**
```
Input: "10 2 /"
  ↓ Lexer
[Token(NUMBER, "10"), Token(NUMBER, "2"), Token(DIV, "/"), Token(EOF, "")]
  ↓ Parser
Stack: [Number(10)] → [Number(10), Number(2)] → [BinaryOp("/", Number(10), Number(2))]
  ↓ Generator
"10 \div 2"
  ↓ CLI
"$10 \div 2$"
```

### 2. Token Layer

**Token Definition (tokens.py):**
- `TokenType.DIV` enum value

**Token Examples:**
- `Token(TokenType.DIV, "/", 1, 5)`

**Lexer Recognition (lexer.py lines 166-168):**
```python
if char == "/":
    self._advance()
    return Token(TokenType.DIV, "/", start_line, start_column)
```

### 3. AST Layer

**Node Type:** `BinaryOp` with operator="/"

**Construction Example:**
```python
BinaryOp(
    line=1, column=5,
    operator="/",
    left=Number(1, 1, "10"),
    right=Number(1, 3, "2")
)
```

### 4. Lexer Layer

**Character Recognition:** Single "/" character (no ambiguity)

**Implementation:** Straightforward single-character operator

### 5. Parser Layer

**Algorithm:** Same as multiplication, operator string is "/"

```
When token.type == TokenType.DIV:
  Create BinaryOp("/", left, right) and push to stack
```

**RPN Semantics:** "10 2 /" means (10 / 2), not (2 / 10)

### 6. Generator Layer

**Operator Mapping (latex_gen.py):**
```python
BINARY_OPS["/"] = r"\div"    # LaTeX division symbol
PRECEDENCE["/"] = 2          # Same precedence as multiplication
```

**Parenthesization - Critical for Division:**

Division requires parentheses on the right operand when the child is also division or subtraction (non-commutative, same-precedence operators):

```python
return (
    child_precedence == parent_precedence
    and is_right
    and child.operator in ("-", "/")
)
```

**Example:**
```
Input RPN: "100 10 / 5 / 2 /"
AST: BinaryOp("/", BinaryOp("/", BinaryOp("/", Number(100), Number(10)), Number(5)), Number(2))
Output: "100 \div 10 \div 5 \div 2"
Represents: ((100 / 10) / 5) / 2, which is left-associative (no parens needed)

Input RPN: "100 10 5 / /"
AST: BinaryOp("/", Number(100), BinaryOp("/", Number(10), Number(5)))
Output: "100 \div ( 10 \div 5 )"
Represents: 100 / (10 / 5)
Parens needed because right child is division at same precedence
```

**Precedence with other operators:**
```
Input RPN: "100 10 / 3 + 4 *"
AST: BinaryOp("*", BinaryOp("+", BinaryOp("/", Number(100), Number(10)), Number(3)), Number(4))
Output: "( 100 \div 10 + 3 ) \times 4"
Reasoning:
  - Division has precedence 2 (highest)
  - Addition has precedence 1
  - Addition + number = "100 \div 10 + 3"
  - Multiplication parent has precedence 2, addition child has precedence 1
  - Lower precedence child needs parentheses
```

### 7. Error Handling

**Lexer:** No errors for "/" character

**Parser:**
- `ParserError` if insufficient operands

**Generator:** No errors for division

### 8. Go Migration Notes

**Type Mappings:**
- Operator string "/" → string "\\div" in LaTeX output

**Idiomatic Go Pattern:**
```go
// Lexer
case '/':
    lex.Advance()
    return &Token{
        Type:   TokenDiv,
        Value:  "/",
        Line:   startLine,
        Column: startColumn,
    }

// Generator precedence and LaTeX mapping already shown above
// Note: "/" appears in NonCommutativeOps for parenthesization logic
```

**Special Considerations:**
- LaTeX output uses `\div`, not "/"
- Precedence level 2 (same as multiplication)
- Non-commutative: right operand needs parentheses when child is "/" or "-"
- Left-associative parsing from RPN

**Test Cases to Verify:**
- Simple division: "10 2 /" → "$10 \\div 2$"
- Multiple divisions: "100 10 / 5 / 2 /" → "$100 \\div 10 \\div 5 \\div 2$"
- Right-associative case: "100 10 5 / /" → "$100 \\div ( 10 \\div 5 )$"
- Division with multiplication: "10 2 / 5 *" → "$10 \\div 2 \\times 5$"

---

## Feature 6: Operator Precedence and Parenthesization

### Overview
This feature describes how different operators interact when combined in expressions. It's the glue that makes the combination of all operators work correctly, ensuring that "2 + 3 * 4" produces "$2 + 3 \\times 4$" (not "$(2 + 3) \\times 4$") and "5 3 + 2 *" produces "$( 5 + 3 ) \\times 2$".

### 1. Feature Boundary Analysis

**Code that belongs to this feature:**
- Generator only: `LaTeXGenerator._needs_parens()` method
- Generator class variables: `PRECEDENCE` and `BINARY_OPS` tables
- Generator method: `LaTeXGenerator._visit_binary_op()` (the application of precedence rules)

**What this feature does NOT include:**
- Individual operator behavior
- Token/AST layer (those are per-operator)
- Parser stack semantics

**Note:** This is purely a generator-level feature. The parser doesn't care about precedence—it just follows RPN semantics. The generator uses precedence rules to decide when to add parentheses.

**Data flow:**
```
AST: BinaryOp("*", BinaryOp("+", ...), ...)
  ↓ Generator visits each node
  ↓ When generating BinaryOp, checks child precedence vs parent
  ↓ Applies parenthesization rules
Output: "$( 5 + 3 ) \\times 2$"
```

### 2. Token Layer
Not applicable - this feature is purely about output formatting.

### 3. AST Layer

**Requirement:** AST nodes must have an `operator` field for BinaryOp nodes so the generator can look up precedence.

Already satisfied by the existing `BinaryOp` structure.

### 4. Lexer Layer
Not applicable to this feature.

### 5. Parser Layer

**Requirement:** Parser must produce correct RPN-based AST structure. The generator's parenthesization logic depends on the AST structure, not token order.

Example: For input "5 3 + 2 *", the parser must produce:
```
BinaryOp("*", BinaryOp("+", Number(5), Number(3)), Number(2))
```

This structure encodes the correct evaluation order, and the generator's parenthesization rules will add parentheses around the addition.

### 6. Generator Layer

**Location:** latex_gen.py lines 54-180

**Core Algorithm:**

The `_visit_binary_op` method applies precedence rules when visiting BinaryOp nodes:

```python
@_visit.register
def _visit_binary_op(self, node: BinaryOp) -> str:
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

**Precedence Rules (lambda-if style in Python):**

```python
def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
    if not isinstance(child, BinaryOp):
        return False  # Numbers never need parentheses

    child_precedence = self.PRECEDENCE[child.operator]

    # Rule 1: Lower precedence always needs parentheses
    if child_precedence < parent_precedence:
        return True

    # Rule 2: Equal precedence on right side needs parentheses for non-commutative ops
    return (
        child_precedence == parent_precedence
        and is_right
        and child.operator in ("-", "/")
    )
```

**Precedence Table:**
```
Level 1 (Low):    + (addition), - (subtraction)
Level 2 (High):   * (multiplication), / (division)
```

**Non-Commutative Operators:** "-" and "/"
- These operators need special handling for right operand at same precedence
- Addition "+" and multiplication "*" are commutative, so no special handling

### 7. Parenthesization Examples

**Case 1: Higher precedence child doesn't need parentheses**
```
Input RPN: "2 3 4 * +"
AST: BinaryOp("+", Number(2), BinaryOp("*", Number(3), Number(4)))

Generation:
- Visit BinaryOp("+", ...)
- my_precedence = 1
- Visit left: Number(2) → "2" (no parens)
- Visit right: BinaryOp("*", ...) → "3 \times 4"
  - child_precedence = 2 (higher than parent 1)
  - _needs_parens returns False (higher precedence doesn't need parens)
- Result: "2 + 3 \times 4"
```

**Case 2: Lower precedence child needs parentheses**
```
Input RPN: "5 3 + 2 *"
AST: BinaryOp("*", BinaryOp("+", Number(5), Number(3)), Number(2))

Generation:
- Visit BinaryOp("*", ...)
- my_precedence = 2
- Visit left: BinaryOp("+", ...) → "5 + 3"
  - child_precedence = 1 (lower than parent 2)
  - _needs_parens returns True (lower precedence needs parens)
  - left becomes "( 5 + 3 )"
- Visit right: Number(2) → "2" (no parens)
- Result: "( 5 + 3 ) \times 2"
```

**Case 3: Equal precedence, commutative operator (addition)**
```
Input RPN: "1 2 + 3 +"
AST: BinaryOp("+", BinaryOp("+", Number(1), Number(2)), Number(3))

Generation:
- Visit BinaryOp("+", ...)
- my_precedence = 1
- Visit left: BinaryOp("+", ...) → "1 + 2"
  - child_precedence = 1 (equal)
  - is_right = False (left child)
  - _needs_parens returns False (not right side)
- Visit right: Number(3) → "3"
  - child_precedence = 1 (equal)
  - is_right = True (right child)
  - child.operator = "+" (not in ("-", "/"))
  - _needs_parens returns False (commutative operator)
- Result: "1 + 2 + 3"
```

**Case 4: Equal precedence, non-commutative operator (subtraction)**
```
Input RPN: "5 3 2 - -"
AST: BinaryOp("-", Number(5), BinaryOp("-", Number(3), Number(2)))

Generation:
- Visit BinaryOp("-", ...)
- my_precedence = 1
- Visit left: Number(5) → "5" (no parens)
- Visit right: BinaryOp("-", ...) → "3 - 2"
  - child_precedence = 1 (equal)
  - is_right = True (right child)
  - child.operator = "-" (in ("-", "/"))
  - _needs_parens returns True (non-commutative on right)
  - right becomes "( 3 - 2 )"
- Result: "5 - ( 3 - 2 )"
```

**Case 5: Mixed operators at different precedence levels**
```
Input RPN: "10 2 / 3 + 4 *"
AST: BinaryOp("*", BinaryOp("+", BinaryOp("/", Number(10), Number(2)), Number(3)), Number(4))

Generation:
Step 1: Visit BinaryOp("*", ...)
  - my_precedence = 2
  - Visit left: BinaryOp("+", ...) needs evaluation
  - Visit right: Number(4) → "4"

Step 2: Evaluate left: BinaryOp("+", ...)
  - my_precedence = 1
  - Visit left: BinaryOp("/", ...) needs evaluation
  - Visit right: Number(3) → "3"

Step 3: Evaluate BinaryOp("/", ...)
  - my_precedence = 2
  - Visit left: Number(10) → "10"
  - Visit right: Number(2) → "2"
  - Result: "10 \div 2"

Step 2 continued: BinaryOp("+", ...)
  - left from step 3: "10 \div 2"
    - child_precedence = 2 (higher than parent 1)
    - _needs_parens returns False
  - right: "3"
  - Result: "10 \div 2 + 3"

Step 1 continued: BinaryOp("*", ...)
  - left from step 2: "10 \div 2 + 3"
    - child_precedence = 1 (lower than parent 2)
    - _needs_parens returns True
    - left becomes "( 10 \div 2 + 3 )"
  - right: "4"
  - Result: "( 10 \div 2 + 3 ) \times 4"
```

### 8. Error Handling

No error cases specific to precedence handling. Errors are generated at the lexer/parser levels.

### 9. Go Migration Notes

**Type Mappings:**
- `dict[str, int]` (Python) → `map[string]int` (Go)
- `dict[str, str]` (Python) → `map[string]string` (Go)

**Idiomatic Go Pattern:**
```go
// Class variables become package-level constants/vars
const (
    PrecedenceLow  = 1
    PrecedenceHigh = 2
)

var (
    LatexOps = map[string]string{
        "+": "+",
        "-": "-",
        "*": "\\times",
        "/": "\\div",
    }

    Precedence = map[string]int{
        "+": 1,
        "-": 1,
        "*": 2,
        "/": 2,
    }

    NonCommutativeOps = map[string]bool{
        "-": true,
        "/": true,
    }
)

// Methods become receiver methods
func (gen *LaTeXGenerator) VisitBinaryOp(node *BinaryOp) string {
    opLatex := LatexOps[node.Operator]
    myPrecedence := Precedence[node.Operator]

    left := gen.Visit(node.Left)
    if gen.NeedsParens(node.Left, myPrecedence, false) {
        left = fmt.Sprintf("( %s )", left)
    }

    right := gen.Visit(node.Right)
    if gen.NeedsParens(node.Right, myPrecedence, true) {
        right = fmt.Sprintf("( %s )", right)
    }

    return fmt.Sprintf("%s %s %s", left, opLatex, right)
}

func (gen *LaTeXGenerator) NeedsParens(child Expr, parentPrecedence int, isRight bool) bool {
    binOp, ok := child.(*BinaryOp)
    if !ok {
        return false
    }

    childPrecedence := Precedence[binOp.Operator]
    if childPrecedence < parentPrecedence {
        return true
    }

    return childPrecedence == parentPrecedence &&
           isRight &&
           NonCommutativeOps[binOp.Operator]
}
```

**Special Considerations:**
- Use interface-based visitor pattern in Go for dispatching on node types
- Create a helper map for non-commutative operators rather than hardcoding in conditional
- Test extensively with various precedence combinations
- Consider whether to use strings for operators or create an Operator enum

**Test Cases to Verify:**
- Simple precedence: "2 3 4 * +" → "$2 + 3 \\times 4$"
- Parens needed: "5 3 + 2 *" → "$( 5 + 3 ) \\times 2$"
- Multiple additions: "1 2 + 3 + 4 +" → "$1 + 2 + 3 + 4$"
- Non-commutative right: "5 3 2 - -" → "$5 - ( 3 - 2 )$"
- Mixed operators: "10 2 / 3 + 4 *" → "$( 10 \\div 2 + 3 ) \\times 4$"

---

## I/O Contract

The following test cases are from the Phase 0 validation run of the Python implementation. These serve as the ground truth for the Go migration.

### Numbers

**Test: Single integer**
- Input: `5`
- Expected: `$5$`

**Test: Decimal number**
- Input: `3.14`
- Expected: `$3.14$`

### Addition

**Test: Simple addition**
- Input: `5 3 +`
- Expected: `$5 + 3$`

**Test: Multiple additions**
- Input: `1 2 + 3 + 4 +`
- Expected: `$1 + 2 + 3 + 4$`

### Subtraction

**Test: Simple subtraction**
- Input: `5 3 -`
- Expected: `$5 - 3$`

**Test: Multiple subtractions**
- Input: `5 3 - 2 -`
- Expected: `$5 - 3 - 2$`

### Multiplication

**Test: Simple multiplication**
- Input: `4 7 *`
- Expected: `$4 \times 7$`

**Test: Multiplication with addition**
- Input: `2 3 4 * +`
- Expected: `$2 + 3 \times 4$`

### Division

**Test: Simple division**
- Input: `10 2 /`
- Expected: `$10 \div 2$`

**Test: Multiple divisions**
- Input: `100 10 / 5 / 2 /`
- Expected: `$100 \div 10 \div 5 \div 2$`

### Operator Precedence

**Test: Addition then multiplication**
- Input: `5 3 + 2 *`
- Expected: `$( 5 + 3 ) \times 2$`

**Test: Addition then multiplication (variant)**
- Input: `2 3 + 4 *`
- Expected: `$( 2 + 3 ) \times 4$`

**Test: Multiplication of sum**
- Input: `2 3 4 + *`
- Expected: `$2 \times ( 3 + 4 )$`

**Test: Product of two sums**
- Input: `1 2 + 3 4 + *`
- Expected: `$( 1 + 2 ) \times ( 3 + 4 )$`

**Test: Complex precedence**
- Input: `10 2 / 3 + 4 *`
- Expected: `$( 10 \div 2 + 3 ) \times 4$`

### Mixed Operations

**Test: Multiplication then addition**
- Input: `5 3 * 2 +`
- Expected: `$5 \times 3 + 2$`

**Test: Division and multiplication**
- Input: `10 2 / 5 *`
- Expected: `$10 \div 2 \times 5$`

**Test: Multiplication then addition (variant)**
- Input: `2 3 * 4 +`
- Expected: `$2 \times 3 + 4$`

### Floating Point

**Test: Float multiplication**
- Input: `3.14 2 *`
- Expected: `$3.14 \times 2$`

**Test: Float addition**
- Input: `1.5 0.5 +`
- Expected: `$1.5 + 0.5$`

### Error Cases

**Test: Unsupported caret operator**
- Input: `2 3 ^`
- Expected Error: `Error: Unexpected character '^'`

**Test: Caret with other operators**
- Input: `2 3 ^ 4 *`
- Expected Error: `Error: Unexpected character '^'`

---

## Cross-Feature Dependencies

The dependency chain for implementing features in order:

```
Infrastructure (errors, tokens, ast_nodes)
    ↓
Feature 1: Numbers (required by all features)
    ↓
Feature 2: Addition (simple operator, foundation for others)
    ├→ Feature 3: Subtraction (similar to addition, non-commutative)
    ├→ Feature 4: Multiplication (higher precedence)
    ├→ Feature 5: Division (higher precedence, non-commutative)
    ↓
Feature 6: Operator Precedence (depends on all operators being present)
    ↓
Full CLI Integration (reads all features, pipes through pipeline)
```

**Recommended Implementation Order:**
1. Infrastructure: Set up error formatting, token types, AST nodes
2. Numbers: Implement complete end-to-end for single numbers
3. Addition: Expand to handle one binary operator
4. Subtraction: Add non-commutative operator handling
5. Multiplication: Add higher precedence
6. Division: Combine non-commutativity with higher precedence
7. Operator Precedence: Implement parenthesization rules
8. CLI: Wire everything together

**Testing Strategy:**
- Each feature should have a dedicated test file
- Features 2-5 should be tested both in isolation and in combination with previous features
- Feature 6 requires comprehensive test coverage of operator combinations
- Use the I/O Contract test cases as regression tests throughout

---

## Summary of Key Patterns for Go Migration

### 1. Token Types
- Use `iota` for auto-incrementing token type constants
- Create Token struct with Type, Value, Line, Column fields
- Implement String() method for debugging

### 2. AST Nodes
- Use interface-based design for Expr types
- Create concrete structs for Number and BinaryOp
- All nodes should have Position() method returning (line, column)

### 3. Lexer
- Manual character-by-character scanning (no regex library needed)
- Track position explicitly with line and column
- Handle lookahead for "-" ambiguity resolution

### 4. Parser
- Use explicit stack ([]Expr slice)
- RPN algorithm: push numbers, pop two for operators
- Return single Expr from parse() or error

### 5. Generator
- Use interface-based visitor pattern
- Dispatch on node type using type assertions
- Use maps for operator→LaTeX and operator→precedence mappings
- Implement parenthesization rules in separate method

### 6. Error Handling
- Create error types with Position information
- Format errors with source context
- Use strings.Builder for efficient concatenation

### 7. CLI
- Use flag or cobra for CLI argument parsing
- Follow same pipeline: Lexer → Parser → Generator → Output
- Handle file I/O with proper error messages
- Exit codes: 0 for success, 1 for errors
