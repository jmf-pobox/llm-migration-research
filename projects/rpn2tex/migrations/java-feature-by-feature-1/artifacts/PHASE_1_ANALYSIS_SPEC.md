# RPN2TeX Feature-Based Migration Specification

## Document Purpose

This specification organizes the rpn2tex codebase by **features** rather than modules. Each feature describes what must be implemented across all layers of the stack (tokens, AST, lexer, parser, LaTeX generator).

This document is designed so that a migrator can implement any single feature in Java using **only** the specification for that feature plus its declared dependencies.

---

## I/O Contract

### From Phase 0: PHASE_0_IO_CONTRACT.md

All implementations must pass the following test cases:

#### Numbers

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5` | `$5$` | PASS | Integer literal |
| `3.14` | `$3.14$` | PASS | Decimal literal |

#### Addition

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 +` | `$5 + 3$` | PASS | Basic addition |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | PASS | Multiple additions (left-associative) |

#### Subtraction

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 -` | `$5 - 3$` | PASS | Basic subtraction |
| `5 3 - 2 -` | `$5 - 3 - 2$` | PASS | Multiple subtractions (left-associative) |

#### Multiplication

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `4 7 *` | `$4 \times 7$` | PASS | Basic multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | PASS | Multiplication has higher precedence than addition |

#### Division

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `10 2 /` | `$10 \div 2$` | PASS | Basic division |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | PASS | Multiple divisions (left-associative) |

#### Operator Precedence

| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | PASS | Parentheses inserted for lower precedence in RPN |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | PASS | Addition requires parentheses when multiplied |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | PASS | Right operand addition requires parentheses |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS | Both operands have lower precedence operations |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | PASS | Complex expression with division, addition, and multiplication |

---

# Feature Specifications

## Feature 1: Numbers

### Overview
Parse and output numeric literals. Numbers are the fundamental building block of all expressions. In RPN, numbers are pushed onto the evaluation stack.

**Purpose**: Enable basic integer and decimal number parsing and representation.

### Token Changes

**Token Type**: `NUMBER`
- **Value**: String representation of the number (e.g., "5", "3.14", "-2")
- **Recognition**: Digit sequences, with optional decimal point
- **Example**: `Token(NUMBER, "3.14", 1, 5)`

### AST Node Changes

**Node Type**: `Number(ASTNode)`
- Immutable dataclass that extends `ASTNode`
- **Attributes**:
  - `value: str` - String representation of the number
  - `line: int` - Line number (1-based)
  - `column: int` - Column number (1-based)

**Type Alias**: `Expr = Number | BinaryOp` (initialized with `Number`)

**Python Code Reference**:
```python
@dataclass(frozen=True)
class Number(ASTNode):
    """Numeric literal node."""
    value: str
```

### Lexer Logic

**Recognition Pattern**:
1. When scanning, if current character is a digit, scan a number token
2. Handle optional minus sign for negative numbers (if not preceded by whitespace, treat as prefix)
3. Consume all consecutive digits for integer part
4. Optionally consume decimal point and following digits for decimal part

**Key Methods**:
- `_scan_number(prefix: str, start_line: int, start_column: int) -> Token`
  - Scans integer part by consuming consecutive digits
  - Optionally scans decimal part (dot + digits)
  - Returns `Token(TokenType.NUMBER, value, start_line, start_column)`

**Python Code Reference**:
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

### Parser Logic

**RPN Semantics**:
- When a `NUMBER` token is encountered, create a `Number` AST node with the token's value
- Push the node onto the stack
- Advance to next token

**Stack Behavior**:
- Numbers are pushed as-is onto the operand stack
- They are later consumed when binary operators are encountered

**Python Code Reference**:
```python
if token.type == TokenType.NUMBER:
    # Push number onto stack
    num_node = Number(
        line=token.line, column=token.column, value=token.value
    )
    stack.append(num_node)
    self._advance()
```

### LaTeX Generation

**Output Format**:
- Numbers are output as their literal value
- No special LaTeX markup needed for simple numbers
- The entire expression is wrapped in math mode (`$...$`) by the `generate()` method

**Visitor Implementation**:
- Register handler for `Number` node type
- Return the node's `value` attribute as-is

**Python Code Reference**:
```python
@_visit.register
def _visit_number(self, node: Number) -> str:
    """Generate LaTeX for a number literal."""
    return node.value
```

### Dependencies
- None. **This is the foundational feature.**

### Test Cases (from I/O Contract)

| Input | Output |
|-------|--------|
| `5` | `$5$` |
| `3.14` | `$3.14$` |

### Key Algorithms

1. **Decimal Number Recognition**:
   - Digits followed by optional period followed by optional digits
   - Pattern: `\d+(\.\d+)?`

2. **Negative Number Handling**:
   - If minus sign is immediately followed by digit (no whitespace), treat as prefix
   - Otherwise, treat minus as subtraction operator

3. **Position Tracking**:
   - Mark start position before consuming any characters
   - Position refers to where number token begins, not any prefix

### Error Handling

**Lexer Errors** (from `lexer.py`):
- Raised by `LexerError(message: str, line: int, column: int)`
- Invalid characters should trigger error at position of invalid character

---

## Feature 2: Addition

### Overview
Implement the addition operator (+). Enables simple arithmetic expressions with two operands.

**Purpose**: Enable basic two-operand arithmetic with the plus symbol.

### Token Changes

**Token Type**: `PLUS`
- **Value**: "+"
- **Recognition**: The character "+"
- **Example**: `Token(PLUS, "+", 1, 3)`

### AST Node Changes

**Node Type**: `BinaryOp(ASTNode)`
- Immutable dataclass that extends `ASTNode`
- **Attributes**:
  - `operator: str` - The operator string: "+"
  - `left: Expr` - Left operand (Number or BinaryOp)
  - `right: Expr` - Right operand (Number or BinaryOp)

**Type Alias**: `Expr = Number | BinaryOp` (extend with `BinaryOp`)

**Python Code Reference**:
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    """Binary operation node."""
    operator: str
    left: Expr
    right: Expr
```

### Lexer Logic

**Recognition**:
- When current character is "+", emit `PLUS` token
- Single character, no lookahead needed

**Python Code Reference**:
```python
if char == "+":
    self._advance()
    return Token(TokenType.PLUS, "+", start_line, start_column)
```

### Parser Logic

**RPN Semantics**:
1. When a `PLUS` token is encountered, require at least 2 items on stack
2. Pop two operands: `right = stack.pop()`, then `left = stack.pop()`
3. Create `BinaryOp(operator="+", left=left, right=right)`
4. Push result back onto stack
5. Advance to next token

**Stack Behavior**:
- Input: `5 3 +` produces stack evolution:
  - After `5`: `[Number("5")]`
  - After `3`: `[Number("5"), Number("3")]`
  - After `+`: `[BinaryOp("+", Number("5"), Number("3"))]`

**Error Handling**:
- If fewer than 2 items on stack, raise `ParserError("Operator '+' requires two operands", token)`

**Python Code Reference**:
```python
elif token.type in (TokenType.PLUS, TokenType.MINUS, TokenType.MULT, TokenType.DIV):
    if len(stack) < 2:
        raise ParserError(f"Operator '{token.value}' requires two operands", token)
    right = stack.pop()
    left = stack.pop()
    op_map = {TokenType.PLUS: "+", ...}
    operator = op_map[token.type]
    op_node = BinaryOp(line=token.line, column=token.column, operator=operator, left=left, right=right)
    stack.append(op_node)
    self._advance()
```

### LaTeX Generation

**Output Format**:
- `left + right` with spaces around the operator
- No parentheses needed (addition is lowest precedence)
- Uses plain "+" symbol

**Visitor Implementation**:
- Handler for `BinaryOp` with operator "+"
- Generate LaTeX for left and right operands
- Check if parentheses needed based on precedence
- Concatenate: `"{left} + {right}"`

**Precedence Level**: 1 (lowest)

**Parenthesization Rules**:
- Addition never needs parentheses on its operands when it's a top-level or lower-precedence context
- If addition is a child of multiplication or division, parentheses needed

**Python Code Reference**:
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2,
}

@_visit.register
def _visit_binary_op(self, node: BinaryOp) -> str:
    """Generate LaTeX for a binary operation."""
    op_latex = self.BINARY_OPS[node.operator]  # "+"
    my_precedence = self.PRECEDENCE[node.operator]
    left = self._visit(node.left)
    if self._needs_parens(node.left, my_precedence, is_right=False):
        left = f"( {left} )"
    right = self._visit(node.right)
    if self._needs_parens(node.right, my_precedence, is_right=True):
        right = f"( {right} )"
    return f"{left} {op_latex} {right}"
```

### Dependencies
- **Must implement first**: Feature 1 (Numbers)

### Test Cases (from I/O Contract)

| Input | Output |
|-------|--------|
| `5 3 +` | `$5 + 3$` |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` |

### Key Algorithms

1. **Stack-based RPN Evaluation**:
   - Operands pushed as encountered
   - Operator pops right, then left (important order!)
   - Result pushed back

2. **Associativity**:
   - Left-associative: `a + b + c` = `(a + b) + c`
   - In RPN: `1 2 + 3 +` evaluates left-to-right naturally

### Implementation Notes

- Token value is stored but not necessarily used (we use `operator` field)
- Position tracking on `BinaryOp` uses operator token position, not left operand
- Addition is commutative (order doesn't matter mathematically, but RPN preserves order)

---

## Feature 3: Subtraction

### Overview
Implement the subtraction operator (-). Enables two-operand subtraction with left-associativity.

**Purpose**: Enable subtraction expressions while correctly handling left-associativity and ensuring parentheses are added when subtraction is a right operand of subtraction.

### Token Changes

**Token Type**: `MINUS`
- **Value**: "-"
- **Recognition**: The character "-"
- **Special Case**: Distinguish from negative number prefix (if digit immediately follows with no whitespace, it's a negative number)
- **Example**: `Token(MINUS, "-", 1, 5)`

### AST Node Changes

**Node Type**: `BinaryOp(ASTNode)` (reuse from Addition)
- **Operator**: "-"
- Left and right operands are general `Expr` types

### Lexer Logic

**Recognition**:
- When current character is "-", need to distinguish two cases:
  1. **Negative number**: If immediately followed by digit (no whitespace), call `_scan_number("-", ...)`
  2. **Subtraction operator**: Otherwise, emit `MINUS` token

**Python Code Reference**:
```python
if char == "-":
    self._advance()
    # Check if this is a negative number (digit follows immediately)
    if not self._at_end() and self._peek().isdigit():
        # It's a negative number
        return self._scan_number("-", start_line, start_column)
    return Token(TokenType.MINUS, "-", start_line, start_column)
```

### Parser Logic

**RPN Semantics**:
- Identical to Addition (both are binary operators)
- When `MINUS` token encountered:
  1. Check stack has ≥2 items
  2. Pop right, then pop left
  3. Create `BinaryOp(operator="-", left=left, right=right)`
  4. Push result back
  5. Advance

**Important**: Order matters for subtraction! `5 - 3 ≠ 3 - 5`
- In RPN: `5 3 -` correctly evaluates as `(5 - 3)`
- Stack pop order: right first, left second

**Python Code Reference** (same pattern as addition):
```python
elif token.type == TokenType.MINUS:
    if len(stack) < 2:
        raise ParserError(f"Operator '-' requires two operands", token)
    right = stack.pop()  # Important: right first!
    left = stack.pop()   # Then left
    op_node = BinaryOp(line=token.line, column=token.column,
                       operator="-", left=left, right=right)
    stack.append(op_node)
    self._advance()
```

### LaTeX Generation

**Output Format**:
- `left - right` with spaces around the operator
- Uses plain "-" symbol

**Precedence Level**: 1 (same as addition)

**Parenthesization Rules** (critical for subtraction):
- Subtraction is **left-associative**: `5 - 3 - 2 = (5 - 3) - 2`
- Right operand of subtraction **MUST** have parentheses if it's also subtraction/division
  - `5 - (3 - 2)` needs parens on right
  - `5 - (3 / 2)` needs parens on right
- Left operand never needs parens
- When parent is multiplication/division, parens needed on subtraction operand

**Python Code Reference**:
```python
PRECEDENCE: ClassVar[dict[str, int]] = {"+": 1, "-": 1, "*": 2, "/": 2}

def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
    """Parentheses needed when:
    1. Lower precedence than parent
    2. Equal precedence on right side AND operator is - or / (non-associative)
    """
    if not isinstance(child, BinaryOp):
        return False
    child_precedence = self.PRECEDENCE[child.operator]
    if child_precedence < parent_precedence:
        return True
    return (child_precedence == parent_precedence and is_right
            and child.operator in ("-", "/"))
```

### Dependencies
- **Must implement first**: Feature 1 (Numbers)
- Can implement simultaneously with: Feature 2 (Addition)

### Test Cases (from I/O Contract)

| Input | Output |
|-------|--------|
| `5 3 -` | `$5 - 3$` |
| `5 3 - 2 -` | `$5 - 3 - 2$` |

### Key Algorithms

1. **Left-Associativity**:
   - Multiple subtractions naturally left-associate in RPN
   - `1 2 - 3 -` → `(1-2)-3` automatically via stack semantics

2. **Non-Associativity Parenthesization**:
   - When subtraction is right child of subtraction/division, add parentheses
   - When subtraction is left child, no parentheses
   - Pattern: `5 - (3 - 2)` vs. `(5 - 3) - 2`

### Implementation Notes

- Lexer must carefully distinguish "-" as operator vs. prefix (lookahead for digit)
- Parser must pop in correct order: right, then left
- LaTeX generator must apply parenthesis rule for right operands

---

## Feature 4: Multiplication

### Overview
Implement the multiplication operator (*). Introduces **higher precedence** than addition/subtraction.

**Purpose**: Enable multiplication expressions and demonstrate precedence-based parenthesization.

### Token Changes

**Token Type**: `MULT`
- **Value**: "*"
- **Recognition**: The character "*"
- **Example**: `Token(MULT, "*", 1, 5)`

### AST Node Changes

**Node Type**: `BinaryOp(ASTNode)` (reuse)
- **Operator**: "*"

### Lexer Logic

**Recognition**:
- When current character is "*", emit `MULT` token
- Single character, no special cases

**Python Code Reference**:
```python
if char == "*":
    self._advance()
    return Token(TokenType.MULT, "*", start_line, start_column)
```

### Parser Logic

**RPN Semantics**:
- Identical to addition/subtraction
- When `MULT` token encountered: pop right, pop left, create `BinaryOp("*", left, right)`, push

**Key Difference from Addition/Subtraction**:
- Multiplication has **higher precedence**
- This affects how the parser's stack evolves
- Example: `2 3 + 4 *` → `(2 + 3) * 4`

**Stack Example**:
```
Input: "2 3 + 4 *"
After 2:    [2]
After 3:    [2, 3]
After +:    [5]           <- Addition forced, precedence rules
After 4:    [5, 4]
After *:    [20]          <- Multiplication of previous result
Result: (2 + 3) * 4
```

### LaTeX Generation

**Output Format**:
- `left \times right` (LaTeX multiplication symbol)
- Spaces around operator

**Precedence Level**: 2 (higher than addition/subtraction)

**Parenthesization Rules**:
- When multiplication is the parent operator:
  - Left operand needs parens if it's addition/subtraction
  - Right operand needs parens if it's addition/subtraction
  - Addition/subtraction have precedence 1 < 2
- When multiplication is child of another operator:
  - No parentheses needed (multiplication is higher precedence)

**Example Parenthesizations**:
- `( 5 + 3 ) \times 2` - addition needs parens (lower precedence)
- `2 \times ( 3 + 4 )` - addition needs parens on right
- `2 \times 3 \times 4` - no parens (same precedence, left-associative)

**Python Code Reference**:
```python
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
```

### Dependencies
- **Must implement first**: Feature 1 (Numbers)
- **Should implement before**: Feature 6 (Precedence) to understand it fully
- Can implement simultaneously with: Features 2 & 3 (Addition/Subtraction)

### Test Cases (from I/O Contract)

| Input | Output |
|-------|--------|
| `4 7 *` | `$4 \times 7$` |
| `2 3 4 * +` | `$2 + 3 \times 4$` |

### Key Algorithms

1. **Precedence Levels**:
   - Addition/subtraction: level 1
   - Multiplication/division: level 2
   - Higher level = binds tighter = evaluated first

2. **Natural RPN Precedence**:
   - RPN naturally expresses precedence through evaluation order
   - `2 3 4 * +` naturally evaluates `3*4` first, then adds to `2`

### Implementation Notes

- Multiplication is **commutative** (order doesn't affect result), but RPN preserves order
- LaTeX symbol is `\times` not `*`
- Parenthesization depends critically on precedence levels

---

## Feature 5: Division

### Overview
Implement the division operator (/). Same precedence as multiplication, with left-associativity requiring careful parenthesization on right operand.

**Purpose**: Complete the set of basic arithmetic operations with special handling for non-associative right operands.

### Token Changes

**Token Type**: `DIV`
- **Value**: "/"
- **Recognition**: The character "/"
- **Example**: `Token(DIV, "/", 1, 5)`

### AST Node Changes

**Node Type**: `BinaryOp(ASTNode)` (reuse)
- **Operator**: "/"

### Lexer Logic

**Recognition**:
- When current character is "/", emit `DIV` token
- Single character, no special cases

**Python Code Reference**:
```python
if char == "/":
    self._advance()
    return Token(TokenType.DIV, "/", start_line, start_column)
```

### Parser Logic

**RPN Semantics**:
- Identical to multiplication
- When `DIV` token encountered: pop right, pop left, create `BinaryOp("/", left, right)`, push

**Important**: Order matters for division! `10 / 2 ≠ 2 / 10`
- In RPN: `10 2 /` correctly evaluates as `(10 / 2)`

**Stack Example**:
```
Input: "100 10 / 5 / 2 /"
After 100:   [100]
After 10:    [100, 10]
After /:     [10]        <- 100/10 = 10
After 5:     [10, 5]
After /:     [2]         <- 10/5 = 2
After 2:     [2, 2]
After /:     [1]         <- 2/2 = 1
Result: ((100 / 10) / 5) / 2
```

### LaTeX Generation

**Output Format**:
- `left \div right` (LaTeX division symbol)
- Spaces around operator

**Precedence Level**: 2 (same as multiplication)

**Parenthesization Rules** (critical for division):
- Division is **left-associative**: `a / b / c = (a / b) / c`
- Right operand of division **MUST** have parentheses if it's also division/subtraction
  - `5 / (3 / 2)` needs parens on right
  - `5 / (3 - 2)` needs parens on right
- When division is child of multiplication, no parentheses needed (same precedence, left-associative)
- When division is child of addition/subtraction, parentheses needed (lower precedence)

**Example Parenthesizations**:
- `100 \div 10 \div 5 \div 2` - no parens (same precedence, left-associative)
- `( 10 \div 2 + 3 ) \times 4` - division needs parens as part of addition

**Python Code Reference** (reuses existing precedence and parenthesis logic):
```python
# Division same precedence as multiplication (2)
# Non-associative rule: if right child has same precedence and is / or -, needs parens
return (child_precedence == parent_precedence and is_right
        and child.operator in ("-", "/"))
```

### Dependencies
- **Must implement first**: Feature 1 (Numbers)
- **Should implement after**: Features 2, 3, 4 to understand precedence fully

### Test Cases (from I/O Contract)

| Input | Output |
|-------|--------|
| `10 2 /` | `$10 \div 2$` |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` |

### Key Algorithms

1. **Left-Associativity**:
   - Multiple divisions naturally left-associate in RPN
   - `100 10 / 5 / 2 /` → `((100/10)/5)/2`

2. **Non-Associativity Parenthesization**:
   - Same rule as subtraction: right operand of division needs parens if it's also division/subtraction
   - Mathematically: `a / (b / c) ≠ (a / b) / c`

### Implementation Notes

- Division is **not commutative**: order matters critically
- LaTeX symbol is `\div` not `/`
- Precedence equal to multiplication, but parenthesization rules for non-associativity apply

---

## Feature 6: Precedence and Parenthesization

### Overview
Implement operator precedence and automatic parenthesization. This is not a separate feature but rather the interaction of all operators. It must be implemented across the LaTeX generator to produce correct output.

**Purpose**: Ensure complex expressions produce correctly parenthesized LaTeX output based on operator precedence levels.

### Token Changes
None new. Uses existing tokens: PLUS, MINUS, MULT, DIV

### AST Node Changes
No new nodes. Precedence handling is about how existing `BinaryOp` nodes are rendered, not how they're structured.

### Lexer Logic
No changes. Lexer tokenizes without caring about precedence.

### Parser Logic

**RPN Semantics**:
- RPN **naturally** handles precedence during evaluation
- Parser uses simple stack-based algorithm: push numbers, pop operands for operators
- **No precedence rules needed in parser** - they're implicit in RPN structure
- The AST structure directly reflects the intended evaluation order

**Example**:
```
RPN: "2 3 4 * +"
Parse:
  2 -> [2]
  3 -> [2, 3]
  4 -> [2, 3, 4]
  * -> [2, 12]    <- 3*4 evaluated first (higher precedence)
  + -> [14]       <- then 2+12
AST: BinaryOp("+", 2, BinaryOp("*", 3, 4))
```

The AST structure itself encodes precedence - multiplication is deeper in the tree.

### LaTeX Generation

This is where precedence **really matters** - for inserting parentheses in output.

**Precedence Levels**:
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,    # Lowest
    "-": 1,
    "*": 2,    # Highest (binds tighter)
    "/": 2,
}
```

**Parenthesization Algorithm**:

A child expression needs parentheses when:

1. **Lower Precedence**: Child has lower precedence than parent
   - Example: `5 + 3` needs parens when inside `(5 + 3) * 2`
   - Rule: `child_precedence < parent_precedence` → add parens

2. **Right Associativity Special Case**: Child has equal precedence AND is right operand AND operator is non-associative (- or /)
   - Example: `5 - (3 - 2)` needs parens on right for subtraction
   - Rule: `child_precedence == parent_precedence AND is_right AND child.operator in ("-", "/")`

**Python Code Reference**:

```python
def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
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
    # (handles left-associativity of - and /)
    return (child_precedence == parent_precedence and is_right
            and child.operator in ("-", "/"))

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

### Test Cases (from I/O Contract)

| Input | Output | Explanation |
|-------|--------|-------------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Addition (level 1) is child of multiplication (level 2), needs parens |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Same: addition as left child of multiplication |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Addition as right child of multiplication, also needs parens |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Both operands are additions under multiplication |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex: division then addition (precedence 2→1), then multiply; addition needs parens under multiplication |

### Dependencies
- **Must implement after**: Features 1-5 (all operators)
- **Requires understanding of**: Operator associativity and precedence

### Key Algorithms

1. **Recursive Parenthesization**:
   - Check if child needs parens based on precedence relationship
   - Recursively process all sub-expressions
   - Add parens only where needed

2. **Precedence Precedence**:
   - Higher number = higher precedence = binds tighter
   - `*` and `/` have higher precedence than `+` and `-`
   - Multiplication/division evaluated before addition/subtraction

3. **Associativity Handling**:
   - Left-associative: `5 - 3 - 2 = (5 - 3) - 2`
   - In RPN, this is natural: `5 3 - 2 -` produces correct order
   - For LaTeX: right child of `-` or `/` needs parens if it's also `-` or `/`

### Implementation Notes

- **RPN simplifies this**: The AST structure created by RPN parsing naturally encodes the correct precedence
- **No precedence climbing needed**: Parser doesn't need precedence-climbing algorithm (that's for infix parsing)
- **LaTeX generation is where precedence matters**: Use precedence and associativity rules to insert parentheses

---

## Cross-Feature Dependencies

```
Feature 1: Numbers
    ↓
Features 2, 3, 4, 5: (+, -, *, /) [can be parallel]
    ↓
Feature 6: Precedence [depends on all others]
```

## Migration Order Recommendation

**Phase 1**: Implement Features 1-2 (Numbers, Addition)
- Minimal but demonstrates full pipeline

**Phase 2**: Add Features 3-5 (Subtraction, Multiplication, Division)
- Each can be added independently after Phase 1

**Phase 3**: Polish Feature 6 (Precedence)
- May be partially working from Phase 2
- Final refinement of parenthesization logic

---

## Global Architecture Overview

### Module Dependencies (Python)

```
cli.py (entry point)
  → lexer.py (tokenization)
  → parser.py (AST building)
  → latex_gen.py (code generation)
  → errors.py (error formatting)

Supporting modules:
  tokens.py (token definitions)
  ast_nodes.py (AST node definitions)
```

### Core Data Structures

**Token**: `(type: TokenType, value: str, line: int, column: int)`
- Immutable
- Position information for error reporting

**AST Node**: Base `ASTNode(line: int, column: int)`
- Concrete types: `Number(value: str)`, `BinaryOp(operator: str, left: Expr, right: Expr)`
- All immutable dataclasses
- Type alias: `Expr = Number | BinaryOp`

**Pipeline**:
```
Raw Text
  → Lexer: Text → [Token]
  → Parser: [Token] → Expr (AST)
  → LaTeXGenerator: Expr → str
  → Output: LaTeX math string
```

### Error Handling

**Lexer Errors**: `LexerError(message: str, line: int, column: int)`
- Invalid characters or malformed numbers

**Parser Errors**: `ParserError(message: str, token: Token)`
- Invalid RPN: not enough operands, too many operands, etc.

**Error Formatter**: Provides gcc/rustc-style output with:
- Line numbers
- Source context
- Caret pointing to error position

---

## Java Migration Considerations

### Type System Mapping

| Python | Java |
|--------|------|
| `Enum` | `enum` or `static final int` constants |
| `@dataclass(frozen=True)` | Record (Java 16+) or immutable class |
| `Union[A, B]` | Abstract base class with subclasses |
| `list[T]` | `List<T>` or `ArrayList<T>` |
| `dict[K, V]` | `Map<K, V>` or `HashMap<K, V>` |
| `str.isdigit()` | `Character.isDigit()` |
| `str.splitlines()` | `String.split("\n")` |
| Exception hierarchy | `Exception` subclasses |

### Pattern Changes

**String Handling**:
- Python's dynamic strings → Java's explicit `char` vs `String` types
- Need explicit character-by-character scanning loops

**Stack-based RPN**:
- Python `list` with `append()`/`pop()` → Java `Stack<T>` or `ArrayList<T>` with `add()`/`remove()`

**Visitor Pattern**:
- Python `@singledispatchmethod` → Java method overloading or explicit `instanceof` checks

**Immutability**:
- Python frozen dataclasses → Java records or immutable classes
- No setters, all fields final

**Error Context**:
- Python `dataclass` attributes → Java field storage in exception subclasses

### Special Java Considerations

1. **Enum Mapping**: TokenType enum works naturally in Java
2. **Type Safety**: Stricter typing requires careful handling of `Expr` type union
3. **Generics**: `List<? extends Expr>` for polymorphic collections
4. **Immutability**: Use `record` (Java 16+) for AST nodes
5. **Function References**: No direct equivalent to `@singledispatchmethod`; use method overloading or visitor pattern
6. **String Position Tracking**: Similar logic but must handle Java's `char` indexing

---

## Summary

This specification documents the rpn2tex system as **6 features**:

1. **Numbers**: Parse and output numeric literals
2. **Addition**: Two-operand addition with `+`
3. **Subtraction**: Two-operand subtraction with `-` (left-associative, non-associative right)
4. **Multiplication**: Two-operand multiplication with `*` (higher precedence)
5. **Division**: Two-operand division with `/` (higher precedence, non-associative right)
6. **Precedence**: Automatic parenthesization based on operator precedence levels

Each feature is fully documented with:
- Token changes needed
- AST node changes needed
- Lexer logic with code examples
- Parser logic with code examples
- LaTeX generation with precedence rules
- Dependencies on other features
- Test cases from the I/O contract
- Key algorithms and implementation notes

A Java migrator should be able to implement any feature by following this specification, referencing the original Python implementation as needed.
