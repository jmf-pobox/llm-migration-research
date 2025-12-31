# Phase 1: Feature-Based Migration Specification for rpn2tex

## Overview

This document provides a comprehensive analysis of the rpn2tex codebase organized by **features**, not by modules. Each feature specification describes which parts of each module contribute to that feature and provides implementation guidance for Rust migration.

The features are listed in dependency order, so features can be migrated in sequence with minimal rework.

---

## I/O Contract (From Phase 0)

### Verified Test Cases

#### Numbers
| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5` | `$5$` | Single integer |
| `3.14` | `$3.14$` | Decimal number |

#### Addition
| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 +` | `$5 + 3$` | Basic addition |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Chained additions (left-associative) |

#### Subtraction
| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 -` | `$5 - 3$` | Basic subtraction |
| `5 3 - 2 -` | `$5 - 3 - 2$` | Chained subtractions (left-associative) |

#### Multiplication
| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `4 7 *` | `$4 \times 7$` | Basic multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Multiplication has higher precedence than addition |

#### Division
| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `10 2 /` | `$10 \div 2$` | Basic division |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Chained divisions (left-associative) |

#### Operator Precedence
| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Parentheses needed: + has lower precedence than * |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Parentheses needed: + has lower precedence than * |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Parentheses needed: + has lower precedence than * |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Both operands of * need parentheses |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex: / same precedence as *, higher than + |

#### Error Cases
| Input | Error Message | Notes |
|-------|---------------|-------|
| `5 +` | `Error: Operator '+' requires two operands` | Not enough values on stack |
| `5 3 + +` | `Error: Operator '+' requires two operands` | Not enough operands for operator |
| `5 3 @ +` | `Error: Unexpected character '@'` | Lexer rejects invalid characters |
| `+` | `Error: Operator '+' requires two operands` | Operator with no operands |
| (empty) | `Error: Empty expression` | Empty input produces error |

---

## Feature 1: Numbers

**Description**: Parse numeric literals (integers and decimals) and output them as-is in LaTeX.

### Dependencies
- None (foundational feature)

### Test Cases
```
Input: "5"
Expected: "$5$"

Input: "3.14"
Expected: "$3.14$"
```

### Token Contributions

**File**: `source/tokens.py`

- **Token Type**: `TokenType.NUMBER`
- **Token Class**: `Token(type=NUMBER, value=<string>, line=<int>, column=<int>)`
- **Example**: `Token(TokenType.NUMBER, "5", 1, 1)`

**Python Code**:
```python
class TokenType(Enum):
    NUMBER = auto()  # Numeric values: 5, 3.14, -2
    EOF = auto()

@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str
    line: int
    column: int
```

### AST Contributions

**File**: `source/ast_nodes.py`

- **Node Type**: `Number(ASTNode)`
- **Attributes**: `value: str` (the string representation of the number)
- **Example**: `Number(line=1, column=1, value="42")`

**Python Code**:
```python
@dataclass(frozen=True)
class Number(ASTNode):
    value: str

Expr = Number | BinaryOp
```

### Lexer Contributions

**File**: `source/lexer.py`

**Key Logic** (lines 170-200):
1. Recognize digit characters (0-9)
2. Scan leading digits for integer part
3. Optionally scan decimal point and fractional digits
4. Handle negative numbers (prefix "-" followed by digits)

**Python Code**:
```python
def _scan_token(self) -> Token:
    char = self._peek()
    if char.isdigit():
        return self._scan_number("", start_line, start_column)

def _scan_number(self, prefix: str, start_line: int, start_column: int) -> Token:
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

**Edge Cases**:
- Negative numbers: Lexer checks for "-" followed by digit (`if not self._at_end() and self._peek().isdigit()`)
- Decimals: Allow "." as separator; digits after "." are part of number
- No leading zeros validation (e.g., "007" is accepted)

### Parser Contributions

**File**: `source/parser.py`

**Key Logic** (lines 107-113):
1. When a NUMBER token is encountered, create a `Number` AST node
2. Push node onto the stack
3. Advance to next token

**Python Code**:
```python
def parse(self) -> Expr:
    stack: list[Expr] = []

    while not self._at_end():
        token: Token = self._current()

        if token.type == TokenType.NUMBER:
            # Push number onto stack
            num_node = Number(
                line=token.line, column=token.column, value=token.value
            )
            stack.append(num_node)
            self._advance()
```

### LaTeX Generator Contributions

**File**: `source/latex_gen.py`

**Key Logic** (lines 99-109):
1. Implement a visitor for `Number` nodes
2. Return the number value as-is
3. Wrap final result in `$...$` delimiters

**Python Code**:
```python
def generate(self, ast: Expr) -> str:
    content = self._visit(ast)
    return f"${content}$"

@_visit.register
def _visit_number(self, node: Number) -> str:
    """Generate LaTeX for a number literal."""
    return node.value
```

### Error Handling

**File**: `source/errors.py`

- Empty expression error: Parser detects `len(stack) == 0` at end of parsing
- Invalid character error: Lexer raises `LexerError` for unrecognized characters

**Python Code** (Parser, lines 156-158):
```python
if len(stack) == 0:
    eof_token = self.tokens[-1]
    raise ParserError("Empty expression", eof_token)
```

### Implementation Notes

**Rust Considerations**:
1. **String Parsing**: Use Rust string methods or `parse()` for converting to f64 if validation needed
2. **Immutability**: Mimic `frozen=True` dataclass behavior with structs (Rust structs are immutable by default)
3. **Position Tracking**: Use struct fields for line/column (same as Python)
4. **Type Safety**: Rust's type system will enforce correct field presence automatically

**Key Algorithmic Details**:
- Numbers are stored as string values (not parsed to numeric types) - maintain this for precision
- Column tracking increments for each character consumed
- Line tracking resets column to 1 when newline encountered
- Decimal point is part of the number value, not a separate token

---

## Feature 2: Addition

**Description**: Support the addition operator (+) which combines two operands.

### Dependencies
- Feature 1: Numbers (needed as operands)

### Test Cases
```
Input: "5 3 +"
Expected: "$5 + 3$"

Input: "1 2 + 3 + 4 +"
Expected: "$1 + 2 + 3 + 4$"
```

### Token Contributions

**File**: `source/tokens.py`

- **Token Type**: `TokenType.PLUS`
- **Example**: `Token(TokenType.PLUS, "+", 1, 5)`

**Python Code**:
```python
class TokenType(Enum):
    PLUS = auto()  # + (addition)
```

### AST Contributions

**File**: `source/ast_nodes.py`

- **Node Type**: `BinaryOp(ASTNode)` - reused for all binary operators
- **Attributes**:
  - `operator: str` = "+"
  - `left: Expr` (left operand)
  - `right: Expr` (right operand)

**Python Code**:
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str  # "+", "-", "*", "/"
    left: Expr
    right: Expr
```

### Lexer Contributions

**File**: `source/lexer.py`

**Key Logic** (lines 150-152):
1. Recognize "+" character
2. Return PLUS token

**Python Code**:
```python
def _scan_token(self) -> Token:
    char = self._peek()
    if char == "+":
        self._advance()
        return Token(TokenType.PLUS, "+", start_line, start_column)
```

### Parser Contributions

**File**: `source/parser.py`

**Key Logic** (lines 115-147):
1. When PLUS token encountered, pop two operands from stack
2. Create BinaryOp node with operator "+"
3. Push result back onto stack

**Python Code**:
```python
elif token.type in (
    TokenType.PLUS,
    TokenType.MINUS,
    TokenType.MULT,
    TokenType.DIV,
):
    # Pop two operands and create binary operation
    if len(stack) < 2:
        raise ParserError(
            f"Operator '{token.value}' requires two operands", token
        )

    right = stack.pop()
    left = stack.pop()

    op_map = {
        TokenType.PLUS: "+",
        TokenType.MINUS: "-",
        TokenType.MULT: "*",
        TokenType.DIV: "/",
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

**Stack Semantics** (RPN):
- Input: `5 3 +`
- After 5: `[5]`
- After 3: `[5, 3]`
- After +: Pop 3 (right), pop 5 (left), create `BinaryOp("+", 5, 3)`, push result
- Result: `[BinaryOp("+", 5, 3)]`

### LaTeX Generator Contributions

**File**: `source/latex_gen.py`

**Key Logic** (lines 111-141):
1. Generate LaTeX for left operand
2. Generate LaTeX for right operand
3. Combine with operator
4. Check if parentheses needed based on precedence

**Python Code**:
```python
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

**Precedence Details** (Addition):
- Precedence level: 1 (lowest)
- Addition is left-associative: `5 - 3 - 2` parses as `(5 - 3) - 2`
- For addition, right operand of equal precedence doesn't need parens (commutative)
  - `1 + 2 + 3` → `1 + 2 + 3` (no parens needed)

### Error Handling

**Error Case**: Not enough operands
- Input: `5 +`
- Error: `Operator '+' requires two operands`
- Location: Parser detects `len(stack) < 2`

**Python Code**:
```python
if len(stack) < 2:
    raise ParserError(
        f"Operator '{token.value}' requires two operands", token
    )
```

### Implementation Notes

**Rust Considerations**:
1. **Enum for Operators**: Use Rust `enum` for operator types rather than string mapping
   - Could be: `enum BinaryOperator { Add, Sub, Mul, Div }`
   - But keep string representation for AST nodes if desired
2. **Stack Operations**: Leverage Vec::pop() (returns Option)
3. **Pattern Matching**: Use Rust's match for token type checking
4. **Left-Associativity**: Inherent in stack-based parsing - no special handling needed

**Key Algorithmic Details**:
- RPN (Reverse Polish Notation) makes operator precedence implicit in the parse tree
- The stack naturally handles evaluation order
- No lookahead or precedence climbing needed (unlike infix parsing)

---

## Feature 3: Subtraction

**Description**: Support the subtraction operator (-) which subtracts right operand from left.

### Dependencies
- Feature 1: Numbers
- Feature 2: Addition (for comparison of operator precedence)

### Test Cases
```
Input: "5 3 -"
Expected: "$5 - 3$"

Input: "5 3 - 2 -"
Expected: "$5 - 3 - 2$"
```

### Token Contributions

**File**: `source/tokens.py`

- **Token Type**: `TokenType.MINUS`
- **Example**: `Token(TokenType.MINUS, "-", 1, 5)`

**Python Code**:
```python
class TokenType(Enum):
    MINUS = auto()  # - (subtraction)
```

**Lexer Complexity**: Distinguishing "-" operator from negative number prefix
- In RPN context, "-" at token boundary is always an operator (or negative number prefix)
- Negative numbers written as "0 5 -" (compute 0-5) or direct "-5" if no space before digit

### AST Contributions

**File**: `source/ast_nodes.py`

- **Node Type**: `BinaryOp` (same as addition)
- **operator**: "-"

### Lexer Contributions

**File**: `source/lexer.py`

**Key Logic** (lines 153-162):
1. Recognize "-" character
2. Check if digit follows (might be negative number)
3. If digit follows, treat as negative number prefix
4. Otherwise, return MINUS token

**Python Code**:
```python
if char == "-":
    # Could be negative number or subtraction operator
    # In RPN, standalone "-" is always subtraction
    self._advance()
    # Check if this is a negative number (digit follows immediately)
    if not self._at_end() and self._peek().isdigit():
        # It's a negative number
        return self._scan_number("-", start_line, start_column)
    return Token(TokenType.MINUS, "-", start_line, start_column)
```

**Ambiguity Resolution**:
- `-5` with no preceding whitespace: negative number
- `5 -3` with space before `-3`: still negative number (checked immediately after `-`)
- `5 -` with space after: subtraction operator
- `5 - 3`: subtraction operator (space after "-")

**Edge Case**: The check `self._peek().isdigit()` means:
- `- 5` (space before digit) → treated as operator (not negative number)
- `-5` (no space) → treated as negative number

### Parser Contributions

**File**: `source/parser.py`

- Uses same logic as addition (lines 115-147)
- Maps `TokenType.MINUS` to operator string "-"
- No special handling needed

### LaTeX Generator Contributions

**File**: `source/latex_gen.py`

**Key Logic** (same as addition):
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "-": "-",  # Subtraction operator
}

PRECEDENCE: ClassVar[dict[str, int]] = {
    "-": 1,  # Same as addition
}
```

**Precedence Details** (Subtraction):
- Precedence level: 1 (same as addition)
- Left-associative: `5 - 3 - 2` = `(5 - 3) - 2` = `2 - 2` = `0`
- Right side with equal precedence NEEDS parens: `5 - (3 - 2)` ≠ `5 - 3 - 2`

**Python Code** (Parenthesization for subtraction):
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

**Example**: `5 3 - 2 -`
- Parse tree: `BinaryOp("-", BinaryOp("-", 5, 3), 2)`
- Left side (BinaryOp with "-"): No parens needed (left operand doesn't need parens)
- Right side (just "2"): No parens needed (not a BinaryOp)
- Result: `5 - 3 - 2` (no parens)

### Error Handling

**Same as addition**: `Operator '-' requires two operands`

### Implementation Notes

**Rust Considerations**:
1. **Negative Number Detection**: Implement lookahead in lexer
   - Peek after consuming "-" to check for digit
   - Use `peek_char()` or similar method
2. **Left-Associativity**: Handled automatically by stack-based parsing
3. **Non-Commutative Nature**: Encoded in `_needs_parens` logic for parenthesization

**Key Algorithmic Details**:
- Subtraction is NOT commutative: `5 - 3` ≠ `3 - 5`
- Left-associative: `a - b - c` = `(a - b) - c`
- This affects parenthesization of right operand in LaTeX generation

---

## Feature 4: Multiplication

**Description**: Support the multiplication operator (*) with higher precedence than addition/subtraction.

### Dependencies
- Feature 1: Numbers
- Features 2-3: Addition/Subtraction (for precedence comparison)

### Test Cases
```
Input: "4 7 *"
Expected: "$4 \\times 7$"

Input: "2 3 4 * +"
Expected: "$2 + 3 \\times 4$"

Input: "5 3 + 2 *"
Expected: "$( 5 + 3 ) \\times 2$"
```

### Token Contributions

**File**: `source/tokens.py`

- **Token Type**: `TokenType.MULT`
- **Example**: `Token(TokenType.MULT, "*", 1, 5)`

**Python Code**:
```python
class TokenType(Enum):
    MULT = auto()  # * (multiplication)
```

### AST Contributions

**File**: `source/ast_nodes.py`

- **Node Type**: `BinaryOp`
- **operator**: "*"

### Lexer Contributions

**File**: `source/lexer.py`

**Key Logic** (lines 163-165):
```python
if char == "*":
    self._advance()
    return Token(TokenType.MULT, "*", start_line, start_column)
```

- Simple single-character recognition
- No ambiguity (unlike "-")

### Parser Contributions

**File**: `source/parser.py`

- Uses same logic as addition/subtraction
- Maps `TokenType.MULT` to operator string "*"

### LaTeX Generator Contributions

**File**: `source/latex_gen.py`

**Key Logic**:
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "*": r"\times",  # LaTeX multiplication symbol
}

PRECEDENCE: ClassVar[dict[str, int]] = {
    "*": 2,  # Higher precedence than + and -
}
```

**Precedence Details** (Multiplication):
- Precedence level: 2 (higher than addition/subtraction at level 1)
- Left-associative (like subtraction)
- Higher precedence means tighter binding

**Parenthesization Examples**:
- `2 3 4 * +` → `2 + 3 * 4` (no parens; * binds tighter)
  - Parse: `BinaryOp("+", 2, BinaryOp("*", 3, 4))`
  - LaTeX: `2 + 3 \times 4`

- `5 3 + 2 *` → `( 5 + 3 ) * 2` (parens needed; + has lower precedence)
  - Parse: `BinaryOp("*", BinaryOp("+", 5, 3), 2)`
  - Left child (BinaryOp with "+"): precedence 1 < parent 2 → needs parens
  - LaTeX: `( 5 + 3 ) \times 2`

**Python Code**:
```python
def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
    if not isinstance(child, BinaryOp):
        return False

    child_precedence = self.PRECEDENCE[child.operator]

    # Lower precedence always needs parens
    if child_precedence < parent_precedence:
        return True

    # For * and / with equal precedence on right: depends on associativity
    return (
        child_precedence == parent_precedence
        and is_right
        and child.operator in ("-", "/")  # Note: "*" not in this list
    )
```

### Error Handling

**Same as other operators**: `Operator '*' requires two operands`

### Implementation Notes

**Rust Considerations**:
1. **Operator Precedence Table**: Store as HashMap or static array
   - Allows easy extension for future operators
2. **Higher Precedence Value**: Rust can enforce this with type system if desired
3. **LaTeX Escaping**: Use raw strings (r"...") for backslash in LaTeX

**Key Algorithmic Details**:
- Multiplication has higher precedence (level 2) vs addition (level 1)
- This is critical for correct parenthesization without explicit precedence climbing
- In RPN, precedence is implicit in the parse tree structure (not in parsing algorithm)

---

## Feature 5: Division

**Description**: Support the division operator (/) with same precedence as multiplication.

### Dependencies
- Feature 1: Numbers
- Features 2-4: Other operators (for precedence comparison)

### Test Cases
```
Input: "10 2 /"
Expected: "$10 \\div 2$"

Input: "100 10 / 5 / 2 /"
Expected: "$100 \\div 10 \\div 5 \\div 2$"

Input: "10 2 / 3 + 4 *"
Expected: "$( 10 \\div 2 + 3 ) \\times 4$"
```

### Token Contributions

**File**: `source/tokens.py`

- **Token Type**: `TokenType.DIV`
- **Example**: `Token(TokenType.DIV, "/", 1, 5)`

**Python Code**:
```python
class TokenType(Enum):
    DIV = auto()  # / (division)
```

### AST Contributions

**File**: `source/ast_nodes.py`

- **Node Type**: `BinaryOp`
- **operator**: "/"

### Lexer Contributions

**File**: `source/lexer.py`

**Key Logic** (lines 166-168):
```python
if char == "/":
    self._advance()
    return Token(TokenType.DIV, "/", start_line, start_column)
```

- Simple single-character recognition

### Parser Contributions

**File**: `source/parser.py`

- Uses same logic as other operators
- Maps `TokenType.DIV` to operator string "/"

### LaTeX Generator Contributions

**File**: `source/latex_gen.py`

**Key Logic**:
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "/": r"\div",  # LaTeX division symbol
}

PRECEDENCE: ClassVar[dict[str, int]] = {
    "/": 2,  # Same as multiplication
}
```

**Precedence Details** (Division):
- Precedence level: 2 (same as multiplication)
- Left-associative: `100 10 / 5 / 2 /` = `((100 / 10) / 5) / 2` = `1 / 2` = `0.5`
- Non-commutative: `10 / 2` ≠ `2 / 10`

**Parenthesization Examples**:
- `100 10 / 5 / 2 /` → `100 \div 10 \div 5 \div 2` (no parens, left-associative)
  - Parse tree: `BinaryOp("/", BinaryOp("/", BinaryOp("/", 100, 10), 5), 2)`
  - No parens needed because left operands are in order

- `10 2 / 3 + 4 *` → `( 10 \div 2 + 3 ) \times 4`
  - Parse: `BinaryOp("*", BinaryOp("+", BinaryOp("/", 10, 2), 3), 4)`
  - Left child of "*" (BinaryOp with "+"): precedence 1 < 2 → needs parens
  - LaTeX: `( 10 \div 2 + 3 ) \times 4`

**Python Code** (same _needs_parens logic):
```python
# For division with equal precedence on right: needs parens (non-commutative)
return (
    child_precedence == parent_precedence
    and is_right
    and child.operator in ("-", "/")
)
```

Example: `10 5 / 2 /`
- Want: `10 / 5 / 2` = `(10 / 5) / 2` = `1`
- Not: `10 / (5 / 2)` = `10 / 2.5` = `4`
- Parse tree: `BinaryOp("/", BinaryOp("/", 10, 5), 2)`
- Right child (just "2"): not a BinaryOp → no parens
- Result: `10 \div 5 \div 2` (correct)

### Error Handling

**Same as other operators**: `Operator '/' requires two operands`

### Implementation Notes

**Rust Considerations**:
1. **Same Precedence as Multiplication**: Simplifies precedence table
   - Both at level 2
2. **Non-Commutative Like Subtraction**: Affects right-side parenthesization
3. **Division by Zero**: Not validated (Python implementation doesn't check either)

**Key Algorithmic Details**:
- Division and multiplication have same precedence (level 2)
- This means `2 * 3 / 4` is `(2 * 3) / 4 = 6 / 4 = 1.5`
- Order matters: they're both left-associative

---

## Feature 6: Precedence Handling

**Description**: Handle operator precedence to add parentheses only where needed in LaTeX output.

### Dependencies
- Features 1-5: All operators must be implemented first

### Test Cases (Complex Precedence)
```
Input: "5 3 + 2 *"
Expected: "$( 5 + 3 ) \\times 2$"

Input: "2 3 + 4 *"
Expected: "$( 2 + 3 ) \\times 4$"

Input: "2 3 4 + *"
Expected: "$2 \\times ( 3 + 4 )$"

Input: "1 2 + 3 4 + *"
Expected: "$( 1 + 2 ) \\times ( 3 + 4 )$"

Input: "10 2 / 3 + 4 *"
Expected: "$( 10 \\div 2 + 3 ) \\times 4$"
```

### Dependencies Graph
```
        Precedence Handling
           /     |     \
          /      |       \
    [+, -, *, /]  with precedence levels
         |
    Precedence Table (static)
         |
    LaTeX Generator (_needs_parens)
```

### Shared Infrastructure

**File**: `source/latex_gen.py`

**Precedence Table**:
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2,
}
```

**Operator LaTeX Mapping**:
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "+": "+",
    "-": "-",
    "*": r"\times",
    "/": r"\div",
}
```

### LaTeX Generator Contributions

**File**: `source/latex_gen.py` (lines 143-180)

**Key Method**: `_needs_parens(child, parent_precedence, is_right)`

**Logic**:
```python
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
```

**Integration in _visit_binary_op**:
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

### Detailed Analysis of Test Cases

**Case 1**: `5 3 + 2 *`
- Parse: `BinaryOp("*", BinaryOp("+", 5, 3), 2)`
- Parent: "*" (precedence 2)
- Left child: `BinaryOp("+", 5, 3)` (precedence 1)
  - `child_precedence (1) < parent_precedence (2)` → **needs parens**
  - Left: `( 5 + 3 )`
- Right child: `2` (not BinaryOp) → no parens
  - Right: `2`
- Result: `( 5 + 3 ) \times 2` ✓

**Case 2**: `2 3 4 + *`
- Parse: `BinaryOp("*", 2, BinaryOp("+", 3, 4))`
- Parent: "*" (precedence 2)
- Left child: `2` (not BinaryOp) → no parens
- Right child: `BinaryOp("+", 3, 4)` (precedence 1)
  - `child_precedence (1) < parent_precedence (2)` → **needs parens**
  - Right: `( 3 + 4 )`
- Result: `2 \times ( 3 + 4 )` ✓

**Case 3**: `1 2 + 3 4 + *`
- Parse: `BinaryOp("*", BinaryOp("+", 1, 2), BinaryOp("+", 3, 4))`
- Parent: "*" (precedence 2)
- Left child: `BinaryOp("+", 1, 2)` (precedence 1)
  - Lower precedence → **needs parens**: `( 1 + 2 )`
- Right child: `BinaryOp("+", 3, 4)` (precedence 1)
  - Lower precedence → **needs parens**: `( 3 + 4 )`
- Result: `( 1 + 2 ) \times ( 3 + 4 )` ✓

**Case 4**: `10 2 / 3 + 4 *`
- Parse: `BinaryOp("*", BinaryOp("+", BinaryOp("/", 10, 2), 3), 4)`
- Parent: "*" (precedence 2)
- Left child: `BinaryOp("+", ..., 3)` (precedence 1)
  - Lower precedence → **needs parens**: `( 10 \div 2 + 3 )`
- Right child: `4` → no parens
- Result: `( 10 \div 2 + 3 ) \times 4` ✓

### Edge Cases

**No Parentheses Needed**:
- `2 3 4 * +` → `2 + 3 \times 4`
  - Parent "+" (precedence 1), right child "*" (precedence 2 > 1)
  - Higher precedence child doesn't need parens

- `1 2 + 3 + 4 +` → `1 + 2 + 3 + 4`
  - All same precedence, left-associative
  - Additions on right side don't need parens (commutative at same level)

**Right-Side Parentheses for Non-Commutative Ops**:
- `5 3 - 2 -` → `5 - 3 - 2` (left-associative, no parens on right)
  - Parent "-", right child "2" (not BinaryOp) → no parens

- If we had `5 3 2 - -` (not valid RPN for this expression):
  - Parent "-", right child `BinaryOp("-", 3, 2)` (same precedence)
  - `is_right=True` and operator in ("-", "/") → **needs parens**: `5 - ( 3 - 2 )`

### Implementation Notes

**Rust Considerations**:
1. **Precedence Table**: Use static HashMap or array
   ```rust
   lazy_static::lazy_static! {
       static ref PRECEDENCE: HashMap<String, u32> = {
           let mut m = HashMap::new();
           m.insert("+".to_string(), 1);
           m.insert("-".to_string(), 1);
           m.insert("*".to_string(), 2);
           m.insert("/".to_string(), 2);
           m
       };
   }
   ```
   Or use enum for stronger typing:
   ```rust
   enum BinOp { Add, Sub, Mul, Div }
   impl BinOp {
       fn precedence(&self) -> u32 {
           match self {
               BinOp::Add | BinOp::Sub => 1,
               BinOp::Mul | BinOp::Div => 2,
           }
       }
   }
   ```

2. **Visitor Pattern**: Rust trait system can replicate `@singledispatchmethod`
   ```rust
   trait AstVisitor {
       fn visit_number(&mut self, n: &Number) -> String;
       fn visit_binary_op(&mut self, op: &BinaryOp) -> String;
   }
   ```

3. **Pattern Matching**: Use `match` for operator checking
   ```rust
   if let BinaryOp { operator, .. } = child {
       // ...
   }
   ```

**Key Algorithmic Details**:
- Precedence is **independent of parsing** in RPN (unlike infix)
  - Parse tree structure determines precedence implicitly
  - Parenthesization is purely **cosmetic** (for human readability)
- The algorithm ensures:
  1. Lower-precedence sub-expressions always get parens
  2. Right-side equal-precedence gets parens only for non-associative/non-commutative ops
  3. Left-side operands never get parens (by definition of how we parse)

---

## Cross-Cutting Concerns

### Error Handling Pipeline

**Error Types**:

1. **Lexer Errors**: Invalid characters
   - Raised in: `lexer.py`, line 175
   - Exception: `LexerError(message, line, column)`
   - Formatted by: `ErrorFormatter.format_error()`

2. **Parser Errors**: Stack underflow, empty expression, extra operands
   - Raised in: `parser.py`, lines 123-166
   - Exception: `ParserError(message, token)`
   - Formatted by: `ErrorFormatter.format_error()`

**Error Flow**:
```python
# In cli.py
try:
    tokens = lexer.tokenize()  # May raise LexerError
    ast = parser.parse()       # May raise ParserError
    latex = generator.generate(ast)  # Generally safe (except NotImplementedError)
except LexerError as e:
    formatted = formatter.format_error(e.message, e.line, e.column)
    print(formatted, file=sys.stderr)
    return 1
except ParserError as e:
    formatted = formatter.format_error(e.message, e.token.line, e.token.column)
    print(formatted, file=sys.stderr)
    return 1
```

**Error Formatting** (`errors.py`):
- Extracts context lines around error
- Positions caret under error location
- Formats with line numbers for alignment

### Position Tracking

**Responsibility Chain**:
1. **Lexer** maintains position (line, column)
   - Increments column for each character
   - Resets column to 1 on newline
   - Tracks position when creating tokens

2. **Token** carries position information
   - `Token.line`: 1-based line number
   - `Token.column`: 1-based column number

3. **AST Node** carries position information
   - Inherited from `ASTNode` base class
   - Used for error reporting if needed

4. **Error Formatter** uses position to display context
   - Creates caret at error column
   - Shows surrounding lines

**Python Code Example**:
```python
# Lexer position tracking
def _advance(self) -> str:
    char = self.text[self.pos]
    self.pos += 1
    if char == "\n":
        self.line += 1
        self.column = 1
    else:
        self.column += 1
    return char

# Token creation carries position
return Token(TokenType.PLUS, "+", start_line, start_column)

# AST node creation carries position
num_node = Number(
    line=token.line, column=token.column, value=token.value
)
```

### Token Type to Operator String Mapping

**In Parser** (`parser.py`, lines 130-137):
```python
op_map = {
    TokenType.PLUS: "+",
    TokenType.MINUS: "-",
    TokenType.MULT: "*",
    TokenType.DIV: "/",
}
operator = op_map[token.type]
```

This mapping must be consistent across:
1. **Parser** (TokenType → operator string)
2. **LaTeX Generator** (operator string → LaTeX)
3. **LaTeX Generator** (operator string → precedence level)

**Rust Consideration**: Use enum with impl to enforce consistency:
```rust
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    fn from_token_type(tt: TokenType) -> Self { /* ... */ }
    fn to_latex_string(&self) -> &'static str { /* ... */ }
    fn precedence(&self) -> u32 { /* ... */ }
}
```

---

## Module Integration Points

### Data Flow
```
CLI
 |
 +-- ErrorFormatter
      |
 +-- Lexer ──→ Token* (uses tokens.py)
      |
 +-- Parser ──→ Expr (uses ast_nodes.py)
      |
 +-- LaTeXGenerator ──→ String
```

### File Dependencies
```
cli.py
├── errors.py (ErrorFormatter)
├── lexer.py (Lexer, LexerError)
│   └── tokens.py (Token, TokenType)
├── parser.py (Parser, ParserError)
│   ├── tokens.py (Token, TokenType)
│   └── ast_nodes.py (ASTNode, Number, BinaryOp, Expr)
└── latex_gen.py (LaTeXGenerator)
    └── ast_nodes.py (Number, BinaryOp, Expr)
```

### Immutability Contract

All Python classes use `@dataclass(frozen=True)` or are meant to be immutable:
- `Token`: frozen dataclass
- `Number`: frozen dataclass
- `BinaryOp`: frozen dataclass
- `ASTNode`: frozen dataclass (base)

**Rust Translation**: Use immutable structs and values (Rust default):
```rust
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number { value: String, line: usize, column: usize },
    BinaryOp {
        operator: String,
        left: Box<Expr>,
        right: Box<Expr>,
        line: usize,
        column: usize,
    },
}
```

---

## Rust Migration Strategy

### Phase 1 (This Document): Planning
- Analyze feature dependencies ✓
- Identify cross-cutting concerns ✓
- Document I/O contract ✓

### Phase 2: Infrastructure
1. Create token types and Token struct
2. Create AST node types (Number, BinaryOp, Expr)
3. Create error types (LexerError, ParserError, ErrorFormatter)

### Phase 3: Feature Implementation (in order)
1. **Numbers**: Lexer scanning + Parser handling + LaTeX generation
2. **Addition**: Token recognition + Operator parsing + LaTeX output
3. **Subtraction**: Add negative number handling to lexer
4. **Multiplication**: Add token + operator handling
5. **Division**: Add token + operator handling
6. **Precedence**: Implement _needs_parens logic

### Phase 4: Integration
1. Create CLI interface
2. Connect pipeline: input → lexer → parser → generator → output
3. Error handling integration

### Phase 5: Testing
- Unit tests for each module
- Integration tests for features
- I/O contract verification (20 test cases)

---

## Code Organization (Suggested Rust Structure)

```
src/
├── lib.rs
├── tokens.rs
├── ast.rs
├── lexer.rs
├── parser.rs
├── generator.rs
├── errors.rs
└── cli.rs

tests/
├── io_contract.rs
└── integration.rs
```

Each file corresponds roughly to its Python counterpart, with Rust idioms applied.

---

## Key Type Mappings

| Python | Rust |
|--------|------|
| `enum.auto()` | `pub enum TokenType { ... }` |
| `@dataclass(frozen=True)` | `struct` (immutable by default) |
| `list[T]` | `Vec<T>` |
| `dict[K, V]` | `HashMap<K, V>` or match arms |
| Union types (`A \| B`) | `enum Expr { A(...), B(...) }` |
| `@singledispatchmethod` | Trait with impl for each type |
| Exception handling (try/except) | `Result<T, E>` or panic macros |
| f-strings | `format!()` macro |
| `Path.read_text()` | `fs::read_to_string()` |
| `sys.stdin.read()` | `io::stdin().read_to_string()` |

---

## Summary of Features

| Feature | Modules Involved | Complexity | Priority |
|---------|------------------|-----------|----------|
| Numbers | tokens, lexer, parser, ast, latex_gen | Low | 1 |
| Addition | tokens, lexer, parser, latex_gen | Low | 2 |
| Subtraction | tokens, lexer (negatives), parser, latex_gen | Medium | 3 |
| Multiplication | tokens, lexer, parser, latex_gen | Low | 4 |
| Division | tokens, lexer, parser, latex_gen | Low | 5 |
| Precedence | latex_gen (mainly) | Medium | 6 |

**Total Implementation Scope**: ~1000 lines of Python code
- tokens.py: ~70 lines
- ast_nodes.py: ~90 lines
- errors.py: ~130 lines
- lexer.py: ~200 lines
- parser.py: ~185 lines
- latex_gen.py: ~185 lines
- cli.py: ~115 lines

---

## Verification Checklist

For each feature migration, ensure:

- [ ] Token types defined correctly
- [ ] AST nodes created/updated
- [ ] Lexer recognizes tokens
- [ ] Parser creates correct AST structure
- [ ] LaTeX generator produces correct output
- [ ] Error handling works
- [ ] All test cases from I/O contract pass
- [ ] Position tracking maintained for error reporting

---

**Document Version**: 1.0
**Date**: 2025-12-30
**Target Language**: Rust
**Source Language**: Python
**Project**: rpn2tex
