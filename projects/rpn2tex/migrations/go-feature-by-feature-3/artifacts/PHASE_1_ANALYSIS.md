# PHASE 1: Feature-Based Analysis for Go Migration

## Executive Summary

This document provides a comprehensive feature-by-feature analysis of the rpn2tex Python codebase to guide incremental Go migration. The implementation is organized around six key features, each described with its cross-module dependencies, implementation details, and Go migration strategy.

The codebase implements a Reverse Polish Notation (RPN) parser that converts mathematical expressions to LaTeX format. At its core, it follows a classic compiler pipeline: lexical analysis (tokenization) → parsing (AST construction) → code generation (LaTeX output).

## Architecture Overview

The rpn2tex pipeline consists of:

1. **Tokenization** (lexer.py): Convert input string to tokens
2. **Parsing** (parser.py): Convert tokens to Abstract Syntax Tree (AST)
3. **Code Generation** (latex_gen.py): Convert AST to LaTeX output
4. **Error Handling** (errors.py): Format errors with source context
5. **CLI Orchestration** (cli.py): Pipeline orchestration and I/O

Supporting modules:
- **tokens.py**: Token type definitions
- **ast_nodes.py**: AST node type definitions

---

## Feature 1: Numbers

### Feature Overview

The Numbers feature handles parsing and rendering of numeric literals (integers and decimals) in RPN expressions. This is the foundational feature upon which all other features depend.

**Examples:**
- Input: `5` → Output: `$5$`
- Input: `3.14` → Output: `$3.14$`

### Python Implementation Details

#### Token Definitions (tokens.py)

```python
class TokenType(Enum):
    NUMBER = auto()  # Lines 33
```

The `NUMBER` token type represents all numeric literals. The `Token` dataclass carries:
- `type`: TokenType (NUMBER in this case)
- `value`: String representation of the number (e.g., "5", "3.14", "-2")
- `line` and `column`: Position information for error reporting

#### Lexer Logic (lexer.py)

**Entry point:** `Lexer._scan_token()` (lines 136-175) dispatches to `_scan_number()`

**Number scanning:** `Lexer._scan_number()` (lines 177-200)
- Accepts an optional prefix (e.g., "-" for negative numbers)
- Scans integer part: `while digit: consume digit`
- Optionally scans decimal point followed by digits
- Returns a NUMBER token with the complete numeric string

**Negative number handling** (lines 158-161):
```python
if char == "-":
    self._advance()
    if not self._at_end() and self._peek().isdigit():
        return self._scan_number("-", start_line, start_column)
    return Token(TokenType.MINUS, "-", start_line, start_column)
```

This logic distinguishes between minus operator and negative number prefix.

#### AST Nodes (ast_nodes.py)

```python
@dataclass(frozen=True)
class Number(ASTNode):
    value: str  # Line 55
```

The `Number` node stores the string representation of the numeric value and inherits position information (line, column) from `ASTNode`.

#### Parser Logic (parser.py)

**Entry point:** `Parser.parse()` (lines 88-168)

**Number handling** (lines 107-113):
```python
if token.type == TokenType.NUMBER:
    num_node = Number(
        line=token.line, column=token.column, value=token.value
    )
    stack.append(num_node)
    self._advance()
```

The parser implements a stack-based RPN algorithm. When a NUMBER token is encountered, it's immediately converted to a Number AST node and pushed onto the stack.

#### LaTeX Generation (latex_gen.py)

**Visitor for Number nodes** (lines 99-109):
```python
@_visit.register
def _visit_number(self, node: Number) -> str:
    return node.value
```

The number is rendered as-is. The wrapping in `$...$` happens in the `generate()` method (line 79).

**Output wrapping** (lines 64-79):
```python
def generate(self, ast: Expr) -> str:
    content = self._visit(ast)
    return f"${content}$"
```

### Cross-Module Dependencies

```
numbers
├── tokens.py (TokenType.NUMBER)
├── lexer.py (tokenization logic)
├── ast_nodes.py (Number class)
├── parser.py (stack-based RPN parsing)
└── latex_gen.py (number rendering + output wrapping)
```

**Dependency chain:**
1. Lexer uses TokenType.NUMBER from tokens.py
2. Parser uses Token from tokens.py and Number from ast_nodes.py
3. LaTeX generator uses Number from ast_nodes.py

### Go Migration Strategy

#### 1. Token Types (tokens.go)
Create enums/constants for token types:
```go
const (
    TokenNumber TokenType = iota
    TokenPlus
    TokenMinus
    // ... other token types
)
```

Struct for Token:
```go
type Token struct {
    Type   TokenType
    Value  string
    Line   int
    Column int
}
```

#### 2. Lexer (lexer.go)

Key functions to implement:
- `NewLexer(text string) *Lexer`: Constructor
- `(l *Lexer) Tokenize() ([]Token, error)`: Main tokenization
- `(l *Lexer) scanNumber(prefix string, startLine, startCol int) Token`: Number scanning

Implementation approach:
- Use `strings.Fields()` or manual character scanning
- Track position with `line` and `column` fields
- Handle negative numbers with lookahead (check if next char is digit)
- Return tokens as slice, with EOF token at the end

#### 3. AST Nodes (ast_nodes.go)

Define types:
```go
type Expr interface {
    // Marker interface for all expression nodes
}

type Number struct {
    Value  string
    Line   int
    Column int
}

type BinaryOp struct {
    Operator string
    Left     Expr
    Right    Expr
    Line     int
    Column   int
}
```

#### 4. Parser (parser.go)

Stack-based implementation:
```go
type Parser struct {
    tokens []Token
    pos    int
}

func (p *Parser) Parse() (Expr, error) {
    stack := []Expr{}
    for !p.isAtEnd() {
        token := p.current()
        if token.Type == TokenNumber {
            stack = append(stack, &Number{Value: token.Value, ...})
            p.advance()
        } else if /* operator */ {
            if len(stack) < 2 { return nil, error }
            // Pop, create BinaryOp, push
        }
    }
    // Validate stack length == 1
    return stack[0], nil
}
```

#### 5. LaTeX Generator (latex_gen.go)

Use type switches or interface methods:
```go
type LaTeXGenerator struct {
    precedence map[string]int
    operators  map[string]string
}

func (g *LaTeXGenerator) Generate(ast Expr) string {
    content := g.visit(ast)
    return fmt.Sprintf("$%s$", content)
}

func (g *LaTeXGenerator) visit(node Expr) string {
    switch n := node.(type) {
    case *Number:
        return n.Value
    case *BinaryOp:
        // Handle with precedence logic
    }
}
```

#### 6. Type Mappings

| Python | Go |
|--------|-----|
| `str` | `string` |
| `int` (for line/column) | `int` |
| `Enum` (TokenType) | `const` + custom type, or `iota` |
| `dataclass` | `struct` |
| `list[Token]` | `[]Token` |
| `singledispatchmethod` | type switch or interface methods |

#### 7. Testing Strategy

- Unit test: Lexer tokenizes "5" into [NUMBER, EOF]
- Unit test: Parser builds Number node from NUMBER token
- Unit test: Generator renders Number as "5"
- Integration test: "5" → "$5$"
- Integration test: "3.14" → "$3.14$"

### Key Implementation Details to Preserve

1. **String-based number values**: Numbers are stored as strings, not parsed to float64/int64. This preserves the exact input format (e.g., "3.14" stays "3.14").
2. **Position tracking**: Every token and AST node carries line/column information for error reporting.
3. **EOF token**: The token stream always ends with an EOF token, used as a sentinel.
4. **Stack discipline**: RPN parser uses a strict stack - push on operands, pop/create nodes on operators.
5. **Negative number handling**: Lexer distinguishes "-" as operator vs. negative number prefix by checking if the next character is a digit.

---

## Feature 2: Addition

### Feature Overview

The Addition feature adds support for the `+` operator in RPN expressions. This builds on the Numbers feature to create simple binary operations.

**Examples:**
- Input: `5 3 +` → Output: `$5 + 3$`
- Input: `1 2 + 3 + 4 +` → Output: `$1 + 2 + 3 + 4$`

### Python Implementation Details

#### Token Definitions (tokens.py)

```python
class TokenType(Enum):
    PLUS = auto()  # Line 36
```

#### Lexer Logic (lexer.py)

**Single-character operator scanning** (lines 150-152):
```python
if char == "+":
    self._advance()
    return Token(TokenType.PLUS, "+", start_line, start_column)
```

When the lexer encounters a `+` character, it immediately returns a PLUS token.

#### AST Nodes (ast_nodes.py)

The `BinaryOp` class represents addition (and all binary operations):

```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str  # "+", "-", "*", "/" (Line 79)
    left: Expr
    right: Expr
```

For "5 3 +", the AST is:
```
BinaryOp(operator="+", left=Number("5"), right=Number("3"))
```

#### Parser Logic (parser.py)

**Operator handling** (lines 115-147):
```python
elif token.type in (TokenType.PLUS, TokenType.MINUS, ...):
    if len(stack) < 2:
        raise ParserError(...)
    right = stack.pop()
    left = stack.pop()
    op_map = {TokenType.PLUS: "+", ...}
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

**Stack evolution for "5 3 +":**
1. See NUMBER "5": stack = [Number("5")]
2. See NUMBER "3": stack = [Number("5"), Number("3")]
3. See PLUS: pop 3 and 5, create BinaryOp("+", 5, 3), push: stack = [BinaryOp(...)]

#### LaTeX Generation (latex_gen.py)

**Binary operation visitor** (lines 111-141):
```python
@_visit.register
def _visit_binary_op(self, node: BinaryOp) -> str:
    op_latex = self.BINARY_OPS[node.operator]  # "+" for addition
    my_precedence = self.PRECEDENCE[node.operator]  # 1 for addition

    left = self._visit(node.left)
    if self._needs_parens(node.left, my_precedence, is_right=False):
        left = f"( {left} )"

    right = self._visit(node.right)
    if self._needs_parens(node.right, my_precedence, is_right=True):
        right = f"( {right} )"

    return f"{left} {op_latex} {right}"
```

**Operator mapping** (lines 47-52):
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "+": "+",  # Addition stays as "+"
    "-": "-",
    "*": r"\times",
    "/": r"\div",
}
```

**Precedence** (lines 57-62):
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,  # Lower precedence
    "-": 1,
    "*": 2,  # Higher precedence
    "/": 2,
}
```

For "5 3 +", the output is:
- Visit left (Number "5"): "5"
- Visit right (Number "3"): "3"
- No parentheses needed
- Result: "5 + 3"
- Wrapped: "$5 + 3$"

### Cross-Module Dependencies

```
addition
├── Feature: numbers (prerequisite)
├── tokens.py (TokenType.PLUS)
├── lexer.py (operator scanning)
├── ast_nodes.py (BinaryOp class)
├── parser.py (operator handling in stack)
└── latex_gen.py (BinaryOp visitor + precedence)
```

### Go Migration Strategy

#### 1. Token Extension (tokens.go)
```go
const (
    TokenPlus TokenType = iota + 1  // After TokenNumber
    // ...
)
```

#### 2. Lexer Extension (lexer.go)
```go
case '+':
    l.advance()
    return &Token{Type: TokenPlus, Value: "+", ...}
```

#### 3. AST Extension (ast_nodes.go)

Already covered in Numbers, but ensure:
- `BinaryOp` struct with `operator string` field
- `operator` is "+" for addition

#### 4. Parser Extension (parser.go)

Add case for `TokenPlus`:
```go
case TokenPlus, TokenMinus, TokenMult, TokenDiv:
    if len(stack) < 2 {
        return nil, fmt.Errorf("not enough operands")
    }
    right := stack[len(stack)-1]
    stack = stack[:len(stack)-1]
    left := stack[len(stack)-1]
    stack = stack[:len(stack)-1]

    opStr := tokenToOperator(token.Type)  // "+"
    node := &BinaryOp{
        Operator: opStr,
        Left:     left,
        Right:    right,
        // ...
    }
    stack = append(stack, node)
    p.advance()
```

#### 5. LaTeX Generator Extension (latex_gen.go)

Ensure `BinaryOp` case in type switch:
```go
case *BinaryOp:
    opLatex := g.operators[n.Operator]  // "+" for addition
    myPrec := g.precedence[n.Operator]

    left := g.visit(n.Left)
    if g.needsParens(n.Left, myPrec, false) {
        left = fmt.Sprintf("( %s )", left)
    }
    // Similar for right

    return fmt.Sprintf("%s %s %s", left, opLatex, right)
```

### Testing Strategy

- Unit test: Lexer tokenizes "5 3 +" into [NUMBER, NUMBER, PLUS, EOF]
- Unit test: Parser builds BinaryOp("+", Number("5"), Number("3"))
- Unit test: Generator renders as "5 + 3"
- Integration test: "5 3 +" → "$5 + 3$"
- Integration test: "1 2 + 3 + 4 +" → "$1 + 2 + 3 + 4$"
  - This tests associativity: ((1 + 2) + 3) + 4 is built as nested BinaryOps and rendered left-to-right

### Key Implementation Details to Preserve

1. **Operator string mapping**: Token types map to operator strings ("+", "-", etc.) for storage in BinaryOp nodes.
2. **Precedence handling**: Addition has precedence level 1; higher precedences don't require parentheses around it.
3. **Associativity**: Addition is left-associative. For "1 2 + 3 +", the AST is:
   ```
   BinaryOp("+", BinaryOp("+", 1, 2), 3)
   ```
   This renders naturally as "(1 + 2) + 3" if parentheses are added, or "1 + 2 + 3" without (because addition associates left).

---

## Feature 3: Subtraction

### Feature Overview

The Subtraction feature adds support for the `-` operator. Unlike addition, subtraction is non-associative on the right side, requiring careful parenthesization.

**Examples:**
- Input: `5 3 -` → Output: `$5 - 3$`
- Input: `5 3 - 2 -` → Output: `$5 - 3 - 2$`

### Python Implementation Details

#### Token Definitions (tokens.py)

```python
class TokenType(Enum):
    MINUS = auto()  # Line 37
```

#### Lexer Logic (lexer.py)

Subtraction shares the same scanning code as addition (lines 153-162):
```python
if char == "-":
    self._advance()
    if not self._at_end() and self._peek().isdigit():
        return self._scan_number("-", start_line, start_column)
    return Token(TokenType.MINUS, "-", start_line, start_column)
```

The only difference from addition is the negative number handling. If "-" is followed immediately (no whitespace) by a digit, it's part of a negative number literal. Otherwise, it's a MINUS operator.

#### AST Nodes (ast_nodes.py)

Same as Addition: `BinaryOp` with `operator="-"`

#### Parser Logic (parser.py)

Same as Addition (lines 115-147), but the operator string is "-":
```python
op_map = {
    TokenType.PLUS: "+",
    TokenType.MINUS: "-",  # Mapped to "-" operator
    ...
}
```

#### LaTeX Generation (latex_gen.py)

**Operator mapping** (lines 47-52):
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "-": "-",  # Subtraction stays as "-"
}
```

**Precedence** (lines 57-62):
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "-": 1,  # Same level as addition
}
```

**Parentheses logic** (lines 143-180):
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

This is the critical difference: subtraction requires parentheses on the right side at equal precedence.

**Example: "5 3 - 2 -"**
- AST: `BinaryOp("-", BinaryOp("-", 5, 3), 2)`
- Rendering:
  - Visit left: `BinaryOp("-", 5, 3)` → "5 - 3" (no parens needed on left)
  - Visit right: `2` → "2" (no parens needed)
  - Result: "5 - 3 - 2"

**Example: "5 3 - 2 - 1 +" (if implemented)**
- AST would have the subtraction chain built correctly due to stack-based RPN parsing
- Left-associativity is implicit in the stack order

### Cross-Module Dependencies

```
subtraction
├── Feature: numbers (prerequisite)
├── tokens.py (TokenType.MINUS)
├── lexer.py (operator scanning + negative number handling)
├── ast_nodes.py (BinaryOp class)
├── parser.py (operator handling)
└── latex_gen.py (BinaryOp visitor with right-associativity check)
```

Key difference from Addition: **LaTeX generator must handle non-commutativity** of subtraction.

### Go Migration Strategy

#### 1. Token Extension
Add `TokenMinus` to token constants (same pattern as TokenPlus)

#### 2. Lexer Extension
```go
case '-':
    l.advance()
    // Lookahead: check if next char is digit
    if !l.isAtEnd() && unicode.IsDigit(rune(l.text[l.pos])) {
        return l.scanNumber("-", startLine, startCol)
    }
    return &Token{Type: TokenMinus, Value: "-", ...}
```

#### 3. Parser Extension
Same as Addition - the operator string is determined by token type

#### 4. LaTeX Generator Extension
Critical: Ensure the `needsParens` logic includes the right-associativity check:
```go
func (g *LaTeXGenerator) needsParens(child Expr, parentPrec int, isRight bool) bool {
    binOp, ok := child.(*BinaryOp)
    if !ok {
        return false
    }

    childPrec := g.precedence[binOp.Operator]
    if childPrec < parentPrec {
        return true
    }

    // Check for non-commutative operators
    return childPrec == parentPrec && isRight &&
           (binOp.Operator == "-" || binOp.Operator == "/")
}
```

### Testing Strategy

- Unit test: Lexer tokenizes "5 3 -" into [NUMBER, NUMBER, MINUS, EOF]
- Unit test: Lexer handles "-5" as negative number, not operator
- Unit test: Parser builds correct AST with left-associativity
- Unit test: Generator applies right-associativity parentheses correctly
- Integration test: "5 3 -" → "$5 - 3$"
- Integration test: "5 3 - 2 -" → "$5 - 3 - 2$"

### Key Implementation Details to Preserve

1. **Negative number ambiguity**: Lexer must distinguish "-" operator from negative number prefix using lookahead.
2. **Right-associativity parentheses**: The generator must add parentheses on the right side for subtraction at equal precedence levels.
3. **Left-to-right evaluation**: Despite the operator being non-associative on the right, RPN guarantees correct parse order at the parser level.

---

## Feature 4: Multiplication

### Feature Overview

The Multiplication feature adds support for the `*` operator, which has higher precedence than addition and subtraction.

**Examples:**
- Input: `4 7 *` → Output: `$4 \times 7$`
- Input: `2 3 4 * +` → Output: `$2 + 3 \times 4$` (precedence: 3*4 happens first)

### Python Implementation Details

#### Token Definitions (tokens.py)

```python
class TokenType(Enum):
    MULT = auto()  # Line 38
```

#### Lexer Logic (lexer.py)

Same pattern as other operators (lines 163-165):
```python
if char == "*":
    self._advance()
    return Token(TokenType.MULT, "*", start_line, start_column)
```

#### AST Nodes (ast_nodes.py)

Same `BinaryOp` with `operator="*"`

#### Parser Logic (parser.py)

Same stack-based approach (lines 115-147), with operator string "*"

#### LaTeX Generation (latex_gen.py)

**Operator mapping** (lines 47-52):
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "*": r"\times",  # Multiplication uses \times in LaTeX
}
```

**Precedence** (lines 57-62):
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "*": 2,  # Higher precedence than addition (1)
}
```

**Example: "2 3 4 * +"**
- AST: `BinaryOp("+", 2, BinaryOp("*", 3, 4))`
- Rendering:
  - Visit left: "2"
  - Visit right: `BinaryOp("*", 3, 4)` → "3 \times 4"
    - Higher precedence (2) than parent (1), so no parens
  - Result: "2 + 3 \times 4"

**Example: "5 3 + 2 *"**
- AST: `BinaryOp("*", BinaryOp("+", 5, 3), 2)`
- Rendering:
  - Visit left: `BinaryOp("+", 5, 3)` → "5 + 3"
    - Lower precedence (1) than parent (2), so parens: "( 5 + 3 )"
  - Visit right: "2"
  - Result: "( 5 + 3 ) \times 2"

### Cross-Module Dependencies

```
multiplication
├── Feature: numbers (prerequisite)
├── tokens.py (TokenType.MULT)
├── lexer.py (operator scanning)
├── ast_nodes.py (BinaryOp class)
├── parser.py (operator handling)
└── latex_gen.py (BinaryOp visitor with precedence check)
```

Multiplication introduces **precedence logic** that becomes critical for correct parenthesization.

### Go Migration Strategy

#### 1. Token Extension
Add `TokenMult` to constants

#### 2. Lexer Extension
```go
case '*':
    l.advance()
    return &Token{Type: TokenMult, Value: "*", ...}
```

#### 3. Parser Extension
Same as previous operators

#### 4. LaTeX Generator Extension
```go
operators := map[string]string{
    "*": r"\times",
}

precedence := map[string]int{
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2,
}
```

The key is ensuring the `needsParens` function correctly compares precedence levels:
- If child precedence < parent precedence, add parens
- If child precedence > parent precedence, no parens
- If equal, check for right-associativity and non-commutativity

### Testing Strategy

- Unit test: Lexer tokenizes "4 7 *" correctly
- Unit test: Parser builds correct AST
- Unit test: Generator applies precedence-based parentheses
- Integration test: "4 7 *" → "$4 \times 7$"
- Integration test: "2 3 4 * +" → "$2 + 3 \times 4$"
- Integration test: "5 3 + 2 *" → "$( 5 + 3 ) \times 2$"
- Integration test: "2 3 + 4 *" → "$( 2 + 3 ) \times 4$"
- Integration test: "2 3 4 + *" → "$2 \times ( 3 + 4 )$"
- Integration test: "1 2 + 3 4 + *" → "$( 1 + 2 ) \times ( 3 + 4 )$"

### Key Implementation Details to Preserve

1. **LaTeX symbol**: Multiplication uses `\times`, not `*`
2. **Precedence level**: Multiplication (2) binds tighter than addition/subtraction (1)
3. **No special associativity**: Unlike subtraction, multiplication doesn't require right-side parentheses at equal precedence (it's commutative)

---

## Feature 5: Division

### Feature Overview

The Division feature adds support for the `/` operator. Like multiplication, it has higher precedence than addition/subtraction. Like subtraction, it's non-associative on the right.

**Examples:**
- Input: `10 2 /` → Output: `$10 \div 2$`
- Input: `100 10 / 5 / 2 /` → Output: `$100 \div 10 \div 5 \div 2$` (left-associative)

### Python Implementation Details

#### Token Definitions (tokens.py)

```python
class TokenType(Enum):
    DIV = auto()  # Line 39
```

#### Lexer Logic (lexer.py)

Same pattern as multiplication (lines 166-168):
```python
if char == "/":
    self._advance()
    return Token(TokenType.DIV, "/", start_line, start_column)
```

#### AST Nodes (ast_nodes.py)

Same `BinaryOp` with `operator="/"`

#### Parser Logic (parser.py)

Same stack-based approach (lines 115-147), with operator string "/"

#### LaTeX Generation (latex_gen.py)

**Operator mapping** (lines 47-52):
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "/": r"\div",  # Division uses \div in LaTeX
}
```

**Precedence** (lines 57-62):
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "/": 2,  # Same level as multiplication
}
```

**Right-associativity handling** (lines 176-180):
```python
return (
    child_precedence == parent_precedence
    and is_right
    and child.operator in ("-", "/")  # Division is in the list!
)
```

**Example: "100 10 / 5 / 2 /"**
- AST: `BinaryOp("/", BinaryOp("/", BinaryOp("/", 100, 10), 5), 2)`
- Rendering:
  - Fully left-associative, no right-side parentheses needed
  - Result: "100 \div 10 \div 5 \div 2"

### Cross-Module Dependencies

```
division
├── Feature: numbers (prerequisite)
├── tokens.py (TokenType.DIV)
├── lexer.py (operator scanning)
├── ast_nodes.py (BinaryOp class)
├── parser.py (operator handling)
└── latex_gen.py (BinaryOp visitor with precedence and right-associativity)
```

Division combines features of both multiplication (higher precedence) and subtraction (right-associativity).

### Go Migration Strategy

#### 1. Token Extension
Add `TokenDiv` to constants

#### 2. Lexer Extension
```go
case '/':
    l.advance()
    return &Token{Type: TokenDiv, Value: "/", ...}
```

#### 3. Parser Extension
Same as previous operators

#### 4. LaTeX Generator Extension
```go
operators["/"] = r"\div"
precedence["/"] = 2
```

Ensure `needsParens` handles division as a non-commutative operator:
```go
return childPrec == parentPrec && isRight &&
       (binOp.Operator == "-" || binOp.Operator == "/")
```

### Testing Strategy

- Unit test: Lexer tokenizes "10 2 /" correctly
- Unit test: Parser builds correct AST
- Integration test: "10 2 /" → "$10 \div 2$"
- Integration test: "100 10 / 5 / 2 /" → "$100 \div 10 \div 5 \div 2$"
- Precedence test: Division with addition (e.g., "10 2 / 3 +" with multiplication: "10 2 / 3 + 4 *")

### Key Implementation Details to Preserve

1. **LaTeX symbol**: Division uses `\div`, not `/`
2. **Precedence level**: Division (2) same as multiplication, both higher than addition/subtraction (1)
3. **Right-associativity**: Division requires parentheses on the right at equal precedence, just like subtraction

---

## Feature 6: Precedence and Parenthesization

### Feature Overview

The Precedence feature ensures that operator precedence is correctly handled during LaTeX generation, inserting parentheses only when necessary to preserve the AST structure in infix notation.

**Examples:**
- Input: `5 3 + 2 *` → Output: `$( 5 + 3 ) \times 2$` (parens on lower-precedence operand)
- Input: `2 3 4 + *` → Output: `$2 \times ( 3 + 4 )$` (parens on right-side operand even though lower precedence)
- Input: `10 2 / 3 + 4 *` → Output: `$( 10 \div 2 + 3 ) \times 4$` (complex precedence)

### Python Implementation Details

#### LaTeX Generator (latex_gen.py)

**Precedence table** (lines 54-62):
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,  # Addition/subtraction: lower precedence
    "-": 1,
    "*": 2,  # Multiplication/division: higher precedence
    "/": 2,
}
```

**Parentheses logic** (lines 143-180):
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

**Rules:**
1. If child has lower precedence than parent, add parentheses
2. If child has equal precedence as parent AND is on the right side AND child operator is non-commutative (- or /), add parentheses
3. Otherwise, no parentheses

**Binary operation visitor** (lines 111-141):
```python
@_visit.register
def _visit_binary_op(self, node: BinaryOp) -> str:
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

### Examples Traced Through

**Case 1: "5 3 + 2 *"**
- AST: `BinaryOp("*", BinaryOp("+", 5, 3), 2)`
- Process:
  - Parent: BinaryOp("*"), precedence 2
  - Left: BinaryOp("+", 5, 3), precedence 1
    - `_needs_parens(BinaryOp("+"), 2, is_right=False)`: 1 < 2 → True, add parens
    - Rendered left: "( 5 + 3 )"
  - Right: Number("2")
    - Not a BinaryOp, no parens
    - Rendered right: "2"
  - Result: "( 5 + 3 ) \times 2"

**Case 2: "2 3 4 + *"**
- AST: `BinaryOp("*", 2, BinaryOp("+", 3, 4))`
- Process:
  - Parent: BinaryOp("*"), precedence 2
  - Left: Number("2")
    - Not a BinaryOp, no parens
    - Rendered left: "2"
  - Right: BinaryOp("+", 3, 4), precedence 1
    - `_needs_parens(BinaryOp("+"), 2, is_right=True)`: 1 < 2 → True, add parens
    - Rendered right: "( 3 + 4 )"
  - Result: "2 \times ( 3 + 4 )"

**Case 3: "5 3 - 2 -" (subtraction right-associativity)**
- AST: `BinaryOp("-", BinaryOp("-", 5, 3), 2)`
- Process:
  - Parent: BinaryOp("-"), precedence 1
  - Left: BinaryOp("-", 5, 3), precedence 1
    - `_needs_parens(BinaryOp("-"), 1, is_right=False)`: 1 == 1 and is_right=False → No parens (left side doesn't need them)
    - Rendered left: "5 - 3"
  - Right: Number("2")
    - Not a BinaryOp, no parens
    - Rendered right: "2"
  - Result: "5 - 3 - 2"

**Case 4: "10 2 / 3 + 4 *"**
- Tokenize: 10 2 / 3 + 4 *
- Parse:
  1. 10: stack = [10]
  2. 2: stack = [10, 2]
  3. /: stack = [10/2]
  4. 3: stack = [10/2, 3]
  5. +: stack = [(10/2)+3]
  6. 4: stack = [(10/2)+3, 4]
  7. *: stack = [((10/2)+3)*4]
- AST: `BinaryOp("*", BinaryOp("+", BinaryOp("/", 10, 2), 3), 4)`
- Rendering:
  - Parent: BinaryOp("*"), precedence 2
  - Left: BinaryOp("+", BinaryOp("/", 10, 2), 3), precedence 1
    - `_needs_parens(BinaryOp("+"), 2, is_right=False)`: 1 < 2 → True, add parens
    - Render left's left: BinaryOp("/", 10, 2), precedence 2
      - `_needs_parens(BinaryOp("/"), 1, is_right=False)`: 2 > 1 → No parens
      - Render: "10 \div 2"
    - Render left's right: 3 → "3"
    - Render left: "10 \div 2 + 3"
    - With parens: "( 10 \div 2 + 3 )"
  - Right: 4 → "4"
  - Result: "( 10 \div 2 + 3 ) \times 4"

### Cross-Module Dependencies

```
precedence
├── Feature: numbers, addition, subtraction, multiplication, division (prerequisites)
└── latex_gen.py (precedence table + parentheses logic)
```

Precedence is entirely contained within the LaTeX generator. It uses the precedence table and the AST structure to determine when parentheses are needed.

### Go Migration Strategy

#### 1. Precedence Table (latex_gen.go)

```go
type LaTeXGenerator struct {
    precedence map[string]int
    operators  map[string]string
}

func NewLaTeXGenerator() *LaTeXGenerator {
    return &LaTeXGenerator{
        precedence: map[string]int{
            "+": 1,
            "-": 1,
            "*": 2,
            "/": 2,
        },
        operators: map[string]string{
            "+": " + ",
            "-": " - ",
            "*": r"\times",
            "/": r"\div",
        },
    }
}
```

#### 2. Parentheses Logic

```go
func (g *LaTeXGenerator) needsParens(child Expr, parentPrec int, isRight bool) bool {
    binOp, ok := child.(*BinaryOp)
    if !ok {
        return false
    }

    childPrec := g.precedence[binOp.Operator]

    // Lower precedence always needs parens
    if childPrec < parentPrec {
        return true
    }

    // Equal precedence on right side needs parens for non-commutative operators
    if childPrec == parentPrec && isRight {
        return binOp.Operator == "-" || binOp.Operator == "/"
    }

    return false
}
```

#### 3. Binary Operation Rendering

```go
func (g *LaTeXGenerator) visitBinaryOp(node *BinaryOp) string {
    opLatex := g.operators[node.Operator]
    myPrec := g.precedence[node.Operator]

    left := g.visit(node.Left)
    if g.needsParens(node.Left, myPrec, false) {
        left = fmt.Sprintf("( %s )", left)
    }

    right := g.visit(node.Right)
    if g.needsParens(node.Right, myPrec, true) {
        right = fmt.Sprintf("( %s )", right)
    }

    return fmt.Sprintf("%s %s %s", left, opLatex, right)
}
```

### Testing Strategy

- Unit test: Precedence table lookup
- Unit test: `needsParens` logic for all cases
  - Lower precedence child: True
  - Equal precedence, left side, any operator: False
  - Equal precedence, right side, + or *: False
  - Equal precedence, right side, - or /: True
  - Higher precedence child: False
- Integration tests for complex expressions
  - "5 3 + 2 *" → "$( 5 + 3 ) \times 2$"
  - "2 3 4 + *" → "$2 \times ( 3 + 4 )$"
  - "10 2 / 3 + 4 *" → "$( 10 \div 2 + 3 ) \times 4$"
  - "1 2 + 3 4 + *" → "$( 1 + 2 ) \times ( 3 + 4 )$"

### Key Implementation Details to Preserve

1. **Precedence table is global**: All operators have fixed precedence levels
2. **Parentheses format**: Always " ( " and " ) " with spaces
3. **Right-associativity only for - and /**: + and * don't need right-side parentheses at equal precedence
4. **Recursive application**: The visitor recursively applies precedence logic to all sub-expressions

---

# I/O Contract

This section includes the I/O Contract from PHASE 0 for behavioral validation during migration.

## Test Results Summary

- **Total Test Cases**: 21
- **Passed**: 21
- **Failed**: 0
- **Status**: All tests pass - outputs match expected values exactly

## Test Cases by Feature

### Numbers

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5` | `$5$` | PASS |
| `3.14` | `$3.14$` | PASS |

### Addition

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5 3 +` | `$5 + 3$` | PASS |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | PASS |

### Subtraction

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5 3 -` | `$5 - 3$` | PASS |
| `5 3 - 2 -` | `$5 - 3 - 2$` | PASS |

### Multiplication

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `4 7 *` | `$4 \times 7$` | PASS |
| `2 3 4 * +` | `$2 + 3 \times 4$` | PASS |

### Division

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `10 2 /` | `$10 \div 2$` | PASS |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | PASS |

### Operator Precedence

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | PASS |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | PASS |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | PASS |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | PASS |

## Output Format Specification

### Operators

| Operator | LaTeX Symbol | Example |
|----------|--------------|---------|
| `+` | ` + ` | `$5 + 3$` |
| `-` | ` - ` | `$5 - 3$` |
| `*` | `\times` | `$4 \times 7$` |
| `/` | `\div` | `$10 \div 2$` |

### Parentheses

Parentheses are added with spaces: `( expression )` when needed for precedence clarity.

### Number Formats

- Integers: preserved as-is (e.g., `5`)
- Floats: preserved with decimal places (e.g., `3.14`)

## Implementation Details

**Source**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/cli.py`

**Key Components**:
- **Lexer**: Tokenizes input strings into tokens
- **Parser**: Builds an Abstract Syntax Tree (AST) from tokens
- **LaTeX Generator**: Converts AST to LaTeX output
- **Error Formatter**: Provides formatted error messages

**Entry Point**: `rpn2tex.cli:main()`

## Notes for Migration

1. **Output Format is Consistent**: All outputs wrap expressions in `$...$` for LaTeX math mode
2. **Spacing is Critical**: Operators have spaces around them (e.g., ` + `, ` - `, etc.)
3. **Parentheses Format**: Parentheses use ` ( ` and ` ) ` with spaces
4. **LaTeX Escaping**: Multiplication uses `\times` and division uses `\div` (backslash-escaped)
5. **Number Preservation**: Float and integer formats are preserved exactly as input
6. **No Trailing Newlines**: The actual output has no trailing newline (stdout only)

---

# Cross-Feature Dependency Graph

```
Feature 1: Numbers (foundational)
    ↓
Feature 2: Addition (depends on Numbers)
    ↓
Feature 3: Subtraction (depends on Numbers)
    ↓
Feature 4: Multiplication (depends on Numbers)
    ↓
Feature 5: Division (depends on Numbers)
    ↓
Feature 6: Precedence (depends on all others)
```

Each feature should be implemented and tested independently in sequence.

---

# Module-to-Feature Mapping

| Module | Features |
|--------|----------|
| tokens.py | 1, 2, 3, 4, 5 (token definitions) |
| lexer.py | 1, 2, 3, 4, 5 (tokenization logic) |
| ast_nodes.py | 1, 2, 3, 4, 5 (AST node definitions) |
| parser.py | 1, 2, 3, 4, 5 (RPN parsing stack) |
| latex_gen.py | 1, 2, 3, 4, 5, 6 (rendering + precedence) |
| errors.py | Not feature-specific (error formatting) |
| cli.py | Not feature-specific (orchestration) |

---

# Implementation Checklist for Go Migration

For each feature (1-6):

- [ ] Define token types in tokens.go
- [ ] Implement lexer logic in lexer.go
- [ ] Define AST node types in ast_nodes.go (or reuse existing)
- [ ] Implement parser logic in parser.go
- [ ] Implement LaTeX generation logic in latex_gen.go
- [ ] Unit tests for each component
- [ ] Integration tests for feature combinations
- [ ] Validate output against I/O Contract

---

# Key Go Idioms and Patterns

1. **Error Handling**: Go doesn't have exceptions. Return `(result, error)` tuples.
2. **Interfaces**: Use interfaces for the AST node types instead of Python's Union type.
3. **Type Assertions**: Use type switches (`switch v := x.(type)`) instead of Python's `isinstance()`.
4. **Struct Tags**: Use struct tags for parsing configuration (if needed).
5. **Slices vs Arrays**: Use slices for dynamic collections (tokens, stack).
6. **String Building**: Use `strings.Builder` for efficient string concatenation.
7. **Constants**: Use `const` and `iota` for token types and precedence values.
8. **Methods vs Functions**: Prefer methods on types for visitor pattern.

---

# Summary

This feature-based analysis provides a clear roadmap for migrating rpn2tex from Python to Go. Each feature is self-contained yet has well-defined dependencies, allowing incremental implementation and testing. The I/O Contract ensures that the final Go implementation produces identical output to the Python reference implementation.

Key success factors:
1. Implement features in dependency order
2. Validate each feature against the I/O Contract
3. Preserve precedence logic exactly
4. Handle operator associativity correctly
5. Maintain output format consistency (spaces, LaTeX symbols, etc.)
