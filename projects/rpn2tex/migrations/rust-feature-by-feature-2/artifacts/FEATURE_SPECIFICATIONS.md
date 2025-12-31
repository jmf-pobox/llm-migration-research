# rpn2tex Feature Specifications for Rust Migration

**Document Version**: 1.0
**Date**: 2025-12-30
**Analysis Scope**: Python source code -> Rust migration specifications
**Organization**: Feature-based (not module-based)

---

## Overview

This document analyzes the rpn2tex codebase organized by FEATURE, with each feature specification being self-contained. The feature-based approach enables:

1. **Clear feature boundaries** - each feature maps to a discrete capability
2. **Dependency tracking** - features explicitly list which other features they depend on
3. **Focused implementation** - Rust implementation can proceed feature-by-feature
4. **Verification strategy** - test cases directly validate feature completeness

---

# Feature 1: Numbers

**Purpose**: Parse and output numeric literals (integers and floating-point)

## Files Touched

- `tokens.py` - Token type definitions
- `lexer.py` - Lexer scanning logic
- `parser.py` - AST node creation
- `ast_nodes.py` - Number node definition
- `latex_gen.py` - Number output generation
- `cli.py` - Pipeline integration

## Dependencies

- None (foundational feature)

## Token Definitions

**From `tokens.py`:**

```python
class TokenType(Enum):
    NUMBER = auto()  # Numeric values: 5, 3.14, -2

@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str  # String representation of the number
    line: int
    column: int
```

**Token values captured**:
- Integers: `"5"`, `"42"`, `"0"`
- Floats: `"3.14"`, `"1.5"`, `"0.5"`
- Negatives: `"-2"`, `"-3.14"`

## AST Nodes

**From `ast_nodes.py`:**

```python
@dataclass(frozen=True)
class Number(ASTNode):
    """Numeric literal node."""
    value: str  # String representation preserved

@dataclass(frozen=True)
class ASTNode:
    line: int      # 1-based line number
    column: int    # 1-based column number
```

**Node invariants**:
- `value` field contains the exact string representation from input
- `line` and `column` track position for error reporting
- Immutable (frozen dataclass)

## Lexer Logic

**From `lexer.py` lines 177-200:**

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

**Key logic**:
1. Scan digits before decimal point
2. Optionally scan `.` followed by more digits
3. Preserve exact string representation
4. Return NUMBER token with position info

**Special case - negative numbers** (lines 153-162):

```python
if char == "-":
    self._advance()
    # Check if this is a negative number (digit follows immediately)
    if not self._at_end() and self._peek().isdigit():
        # It's a negative number
        return self._scan_number("-", start_line, start_column)
    return Token(TokenType.MINUS, "-", start_line, start_column)
```

**Negative number rule**:
- `"-"` followed immediately by digit = negative number
- `"-"` followed by whitespace = subtraction operator
- Example: `"3 -2"` -> NUMBER(-2), but `"3 - 2"` -> MINUS operator

## Parser Logic

**From `parser.py` lines 88-168:**

```python
def parse(self) -> Expr:
    """Parse tokens into an AST."""
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

**Key logic**:
1. When NUMBER token encountered, create Number node
2. Preserve position information from token
3. Preserve string value exactly
4. Push onto evaluation stack

## Generator Logic

**From `latex_gen.py` lines 99-109:**

```python
@_visit.register
def _visit_number(self, node: Number) -> str:
    """Generate LaTeX for a number literal."""
    return node.value
```

**Output rule**: Numbers are output as-is without modification or formatting.

## Error Handling

**Lexer errors** (when number cannot be scanned):
- Invalid characters during digit scanning -> handled by caller
- No specific number validation errors (whitespace/EOF terminates number)

**Parser errors** (impossible for valid NUMBER tokens):
- Numbers never cause parser errors
- Stack operations on numbers never fail

## I/O Contract Test Cases

**Basic numbers:**

| Input | Expected | Notes |
|-------|----------|-------|
| `5 3 +` | `$5 + 3$` | Integer literals |
| `3.14 2 *` | `$3.14 \times 2$` | Float literals |
| `1.5 0.5 +` | `$1.5 + 0.5$` | Float addition |

**Behavior**:
- Numbers preserved exactly as input
- Position tracking maintains line/column info
- Negative numbers handled as single token

## Rust Migration Notes

### Type Mappings

| Python | Rust | Notes |
|--------|------|-------|
| `str` (token.value) | `String` | Preserve exact input |
| `int` (line/column) | `u32` or `usize` | Position tracking |
| `Token` dataclass | `struct Token` | Immutable struct |
| `Number` dataclass | `struct Number` | AST node |

### Pattern Changes

1. **String preservation**: Unlike numeric evaluation languages, rpn2tex preserves the exact string representation. This is critical for LaTeX output (e.g., `3.14` must output as `3.14`, not `3.1400000`).

2. **Negative number handling**: The lookahead check (`peek().isdigit()`) after `-` must be preserved exactly. The lexer has context-dependent tokenization.

3. **Decimal point scanning**: The `.` character must be recognized during number scanning, not as a separate token.

### Special Handling

1. **String ownership**: Store `String` or `&str` based on token lifetime strategy
2. **Position tracking**: Use `usize` for line/column (1-based, as in Python)
3. **Immutability**: Both Token and Number should be immutable (Rust encourages this naturally)

---

# Feature 2: Addition

**Purpose**: Implement addition operator (+) for RPN expressions

## Files Touched

- `tokens.py` - PLUS token type
- `lexer.py` - Lexer scanning of `+`
- `parser.py` - BinaryOp creation
- `ast_nodes.py` - BinaryOp node definition
- `latex_gen.py` - LaTeX generation for `+`
- `cli.py` - Pipeline integration

## Dependencies

- **numbers** - Must parse the operands

## Token Definitions

**From `tokens.py` lines 36:**

```python
class TokenType(Enum):
    PLUS = auto()  # + (addition)
```

**Token representation**:
- Type: `TokenType.PLUS`
- Value: `"+"`
- Line/Column: Position in source

## AST Nodes

**From `ast_nodes.py` lines 58-82:**

```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    """Binary operation node."""
    operator: str  # "+"
    left: Expr     # First operand
    right: Expr    # Second operand
```

**Node structure**:
- `operator = "+"`
- `left` = first popped operand (right-most in RPN)
- `right` = second popped operand (left-most in RPN)

## Lexer Logic

**From `lexer.py` lines 150-152:**

```python
if char == "+":
    self._advance()
    return Token(TokenType.PLUS, "+", start_line, start_column)
```

**Key logic**:
- Single character token
- No lookahead required
- No ambiguity with other characters

## Parser Logic

**From `parser.py` lines 115-147:**

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

**RPN stack algorithm**:
1. When `+` encountered, check stack has >= 2 elements
2. Pop right operand (top of stack)
3. Pop left operand (next)
4. Create BinaryOp("+", left, right)
5. Push result back

**Operand order**:
- In RPN `5 3 +`, evaluation is `5 + 3`
- Stack before operator: `[5, 3]`
- Pop 3 (right), pop 5 (left)
- Result: BinaryOp("+", Number(5), Number(3))

## Generator Logic

**From `latex_gen.py` lines 112-141:**

```python
@_visit.register
def _visit_binary_op(self, node: BinaryOp) -> str:
    """Generate LaTeX for a binary operation."""
    op_latex = self.BINARY_OPS[node.operator]  # "+" for addition
    my_precedence = self.PRECEDENCE[node.operator]  # 1 for addition

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

**LaTeX mapping**:

```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "+": "+",
    ...
}

PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,  # Lower precedence (level 1)
    ...
}
```

**Output format**:
- Single space before and after operator
- Parentheses only when needed (based on precedence)

## Error Handling

**Parser errors**:
- Insufficient operands: `"5 +"` -> "Operator '+' requires two operands"
- Error includes token position (line, column)

## I/O Contract Test Cases

**Addition tests:**

| Input | Expected Output | Category |
|-------|-----------------|----------|
| `5 3 +` | `$5 + 3$` | Basic binary addition |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Chained addition (left-associative) |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Precedence: addition < multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Precedence: multiplication > addition |

**Key behaviors**:
- Addition is commutative in LaTeX sense (same spacing/format as subtraction)
- Parenthesization follows precedence rules
- Left-associative chaining works naturally with RPN

## Rust Migration Notes

### Type Mappings

| Python | Rust | Notes |
|--------|------|-------|
| `TokenType.PLUS` | `TokenType::Plus` enum variant | Owned variant |
| `operator: str` | `operator: &'static str` or `String` | "+" is constant |
| `BinaryOp` struct | `BinaryOp` struct with Box<Expr> | Recursive type |

### Pattern Changes

1. **Recursive ownership**: BinaryOp children need Box for recursive types:
   ```rust
   pub struct BinaryOp {
       operator: String,
       left: Box<Expr>,
       right: Box<Expr>,
   }
   ```

2. **Enum dispatch**: Replace Python's singledispatchmethod with Rust enum match:
   ```rust
   match node {
       Expr::Number(n) => { ... }
       Expr::BinaryOp(op) => { ... }
   }
   ```

3. **String constants**: Consider using `&'static str` for operator values

### Special Handling

1. **Precedence tables**: Can be HashMap or match expressions (simple, few entries)
2. **Parenthesization logic**: Exact same algorithm, but with Rust ownership patterns
3. **Error handling**: Use Result<String, Error> instead of exceptions

---

# Feature 3: Subtraction

**Purpose**: Implement subtraction operator (-) for RPN expressions

## Files Touched

- `tokens.py` - MINUS token type
- `lexer.py` - Lexer scanning of `-` (with negative number distinction)
- `parser.py` - BinaryOp creation
- `ast_nodes.py` - BinaryOp reuse
- `latex_gen.py` - LaTeX generation for `-`
- `cli.py` - Pipeline integration

## Dependencies

- **numbers** - Must parse the operands
- **addition** - Same BinaryOp and precedence handling

## Token Definitions

**From `tokens.py` lines 37:**

```python
class TokenType(Enum):
    MINUS = auto()  # - (subtraction)
```

## AST Nodes

**Reuses BinaryOp from addition feature:**

```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str  # "-" for subtraction
    left: Expr
    right: Expr
```

## Lexer Logic

**From `lexer.py` lines 153-162:**

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

**Negative number vs operator distinction**:
- `-` immediately followed by digit (no whitespace) = negative number
- `-` followed by whitespace or other = MINUS operator
- Example: `"-5"` -> NUMBER("-5"), `"3 - 5"` -> MINUS

## Parser Logic

**Identical to addition, via same shared operator handling:**

```python
elif token.type in (..., TokenType.MINUS, ...):
    # Same stack-based algorithm
    right = stack.pop()
    left = stack.pop()
    operator = op_map[token.type]  # "-"
    op_node = BinaryOp(
        line=token.line,
        column=token.column,
        operator="-",
        left=left,
        right=right,
    )
    stack.append(op_node)
```

**RPN evaluation order**:
- In RPN `5 3 -`, evaluation is `5 - 3` (left - right)
- Stack: `[5, 3]`
- Pop 3 (right), pop 5 (left)
- Result: BinaryOp("-", Number(5), Number(3))

## Generator Logic

**From `latex_gen.py`:**

```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "-": "-",  # Direct mapping
    ...
}

PRECEDENCE: ClassVar[dict[str, int]] = {
    "-": 1,  # Same precedence as addition
    ...
}
```

**Parenthesization rule** (lines 176-180):

```python
return (
    child_precedence == parent_precedence
    and is_right
    and child.operator in ("-", "/")
)
```

**Special rule for subtraction**: When child has equal precedence and is on RIGHT side, add parentheses for `-` and `/` only (non-commutative operators).

Example: `5 3 2 - -` evaluates as `5 - (3 - 2) = 4`, but outputs as `$5 - 3 - 2$` because left-associative evaluation means `(5 - 3) - 2 = 0`.

**Output format**: Same as addition - single space around operator

## Error Handling

**Parser errors**:
- Insufficient operands: `"5 -"` -> "Operator '-' requires two operands"

## I/O Contract Test Cases

**Subtraction tests:**

| Input | Expected Output | Category |
|-------|-----------------|----------|
| `5 3 -` | `$5 - 3$` | Basic subtraction |
| `5 3 - 2 -` | `$5 - 3 - 2$` | Chained subtraction (left-associative) |
| `2 3 4 + *` | `$2 + 3 \times 4$` | Precedence: multiplication > subtraction |

**Key behaviors**:
- Left-associative: `5 - 3 - 2` evaluates as `(5 - 3) - 2 = 0`
- No extra parentheses for left-associative chains
- Non-commutative: order matters, unlike addition

## Rust Migration Notes

### Type Mappings

Same as addition - reuses BinaryOp and shared dispatch mechanisms.

### Pattern Changes

1. **Negative number lookahead**: Crucial to preserve the immediate-digit check:
   ```rust
   if char == '-' {
       self.advance()?;
       if !self.at_end() && self.peek().is_ascii_digit() {
           self.scan_number_with_prefix("-")
       } else {
           Token::Minus
       }
   }
   ```

2. **Operator precedence**: Subtraction and addition share precedence level (1)

3. **Non-commutativity**: The parenthesization logic must check `is_right` for minus

### Special Handling

1. **Lexer ambiguity**: The `-` character has context-dependent meaning - this is a fundamental design choice that must be preserved
2. **Left-associativity**: When chaining subtractions, no parentheses are added even though evaluation order matters
3. **Right-side parenthesization**: Only needed when child is also `-` or `/` and on right side

---

# Feature 4: Multiplication

**Purpose**: Implement multiplication operator (*) for RPN expressions

## Files Touched

- `tokens.py` - MULT token type
- `lexer.py` - Lexer scanning of `*`
- `parser.py` - BinaryOp creation
- `ast_nodes.py` - BinaryOp reuse
- `latex_gen.py` - LaTeX generation for `\times`
- `cli.py` - Pipeline integration

## Dependencies

- **numbers** - Must parse the operands
- **addition** - Same BinaryOp and operator handling

## Token Definitions

**From `tokens.py` lines 38:**

```python
class TokenType(Enum):
    MULT = auto()  # * (multiplication)
```

## AST Nodes

**Reuses BinaryOp:**

```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str  # "*" for multiplication
    left: Expr
    right: Expr
```

## Lexer Logic

**From `lexer.py` lines 163-165:**

```python
if char == "*":
    self._advance()
    return Token(TokenType.MULT, "*", start_line, start_column)
```

**Key logic**:
- Single character token
- No ambiguity
- No lookahead needed

## Parser Logic

**Identical to addition/subtraction via shared operator handling:**

```python
elif token.type in (..., TokenType.MULT, ...):
    # Same stack-based algorithm
    right = stack.pop()
    left = stack.pop()
    operator = op_map[token.type]  # "*"
    op_node = BinaryOp(
        line=token.line,
        column=token.column,
        operator="*",
        left=left,
        right=right,
    )
    stack.append(op_node)
```

## Generator Logic

**From `latex_gen.py`:**

```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "*": r"\times",  # LaTeX multiplication symbol
    ...
}

PRECEDENCE: ClassVar[dict[str, int]] = {
    "*": 2,  # Higher precedence than +/-
    ...
}
```

**Critical feature**: Multiplication is HIGHER precedence than addition/subtraction.

**Parenthesization behavior**:

```python
# Lower precedence always needs parens
if child_precedence < parent_precedence:
    return True
```

When multiplication appears as parent operator:
- Children with precedence < 2 (i.e., +/-) need parentheses
- Children with precedence >= 2 (* or /) don't need parentheses

**Output format**: `\times` (LaTeX command, not literal `*`)

## Error Handling

**Parser errors**:
- Insufficient operands: `"5 *"` -> "Operator '*' requires two operands"

## I/O Contract Test Cases

**Multiplication tests:**

| Input | Expected Output | Category |
|-------|-----------------|----------|
| `4 7 *` | `$4 \times 7$` | Basic multiplication |
| `3.14 2 *` | `$3.14 \times 2$` | Float multiplication |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Precedence: addition < multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Precedence: multiplication > addition |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | Precedence: multiplication > addition |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | Same precedence as division |

**Key behaviors**:
- Highest precedence among basic operators (level 2)
- Commutative (order doesn't affect semantics)
- Parenthesizes lower-precedence children
- No parentheses for equal-precedence siblings

## Rust Migration Notes

### Type Mappings

Same as previous operators - reuses BinaryOp.

### Pattern Changes

1. **LaTeX output**: The `\times` command must be handled correctly in string output
2. **Precedence constant**: Multiplication at level 2 (hardcoded in precedence table)

### Special Handling

1. **LaTeX escaping**: The backslash in `\times` must be properly escaped in Rust strings (raw strings recommended)
   ```rust
   "*" => r"\times"
   ```

---

# Feature 5: Division

**Purpose**: Implement division operator (/) for RPN expressions

## Files Touched

- `tokens.py` - DIV token type
- `lexer.py` - Lexer scanning of `/`
- `parser.py` - BinaryOp creation
- `ast_nodes.py` - BinaryOp reuse
- `latex_gen.py` - LaTeX generation for `\div`
- `cli.py` - Pipeline integration

## Dependencies

- **numbers** - Must parse the operands
- **addition** - Same BinaryOp and operator handling
- **multiplication** - Same precedence level

## Token Definitions

**From `tokens.py` lines 39:**

```python
class TokenType(Enum):
    DIV = auto()  # / (division)
```

## AST Nodes

**Reuses BinaryOp:**

```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str  # "/" for division
    left: Expr
    right: Expr
```

## Lexer Logic

**From `lexer.py` lines 166-168:**

```python
if char == "/":
    self._advance()
    return Token(TokenType.DIV, "/", start_line, start_column)
```

**Key logic**:
- Single character token
- No ambiguity
- No lookahead needed

## Parser Logic

**Identical to other binary operators:**

```python
elif token.type in (..., TokenType.DIV, ...):
    # Same stack-based algorithm
    right = stack.pop()
    left = stack.pop()
    operator = op_map[token.type]  # "/"
    op_node = BinaryOp(
        line=token.line,
        column=token.column,
        operator="/",
        left=left,
        right=right,
    )
    stack.append(op_node)
```

## Generator Logic

**From `latex_gen.py`:**

```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "/": r"\div",  # LaTeX division symbol
    ...
}

PRECEDENCE: ClassVar[dict[str, int]] = {
    "/": 2,  # Same as multiplication
    ...
}
```

**Parenthesization rule** (lines 176-180):

```python
return (
    child_precedence == parent_precedence
    and is_right
    and child.operator in ("-", "/")
)
```

**Special rule for division**: When child has equal precedence and is on RIGHT side, add parentheses for `/` (non-commutative).

Example: `100 10 5 / /` evaluates as `100 / (10 / 5) = 50`, but outputs as `$100 \div 10 \div 5$` because left-associative chains don't need explicit parentheses.

**Output format**: `\div` (LaTeX command)

## Error Handling

**Parser errors**:
- Insufficient operands: `"5 /"` -> "Operator '/' requires two operands"

## I/O Contract Test Cases

**Division tests:**

| Input | Expected Output | Category |
|-------|-----------------|----------|
| `10 2 /` | `$10 \div 2$` | Basic division |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Chained division (left-associative) |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | Same precedence as multiplication |

**Key behaviors**:
- Same precedence as multiplication (level 2)
- Non-commutative: order matters significantly
- Left-associative: `100 / 10 / 5 = (100 / 10) / 5 = 2`
- Right-side parenthesization rule applies

## Rust Migration Notes

### Type Mappings

Same as previous operators - reuses BinaryOp.

### Pattern Changes

1. **LaTeX output**: The `\div` command must be properly escaped
   ```rust
   "/" => r"\div"
   ```

2. **Non-commutativity**: Division is not commutative, so operand order is critical

### Special Handling

1. **Right-side parenthesization**: The special rule for `/` on the right side of another `/` must be preserved exactly
2. **Precedence equality**: Division has same precedence as multiplication (both level 2)

---

# Feature 6: Operator Precedence

**Purpose**: Implement correct parenthesization based on operator precedence levels

## Files Touched

- `latex_gen.py` - Precedence table and parenthesization logic
- `parser.py` - Stack-based parsing (indirectly creates correct AST structure)

## Dependencies

- **addition** - Provides first operator (precedence 1)
- **subtraction** - Shares precedence level with addition
- **multiplication** - Higher precedence (level 2)
- **division** - Shares precedence level with multiplication

## Precedence Definition

**From `latex_gen.py` lines 54-62:**

```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,  # Lower precedence level
    "-": 1,  # Same as addition
    "*": 2,  # Higher precedence level
    "/": 2,  # Same as multiplication
}
```

**Precedence levels** (higher number = tighter binding):
- Level 1: Addition (+), Subtraction (-)
- Level 2: Multiplication (*), Division (/)

## Parenthesization Algorithm

**From `latex_gen.py` lines 143-180:**

```python
def _needs_parens(
    self, child: Expr, parent_precedence: int, *, is_right: bool
) -> bool:
    """Determine if a child expression needs parentheses."""
    if not isinstance(child, BinaryOp):
        return False

    child_precedence = self.PRECEDENCE[child.operator]

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

**Three rules**:

1. **Lower precedence always needs parentheses**
   - Example: `5 3 + 2 *` -> `(5 + 3) * 2`
   - Addition (precedence 1) child under multiplication (precedence 2) parent

2. **Equal precedence on right side needs parentheses for non-commutative operators**
   - Example: `5 3 - 2 -` -> `5 - 3 - 2` (NO extra parentheses because left-associative)
   - BUT: `5 3 2 - -` -> `5 - (3 - 2)` (right side gets parens)
   - Only applies to `-` and `/`

3. **Higher precedence never needs parentheses**
   - Example: `2 3 4 * +` -> `2 + 3 * 4`
   - Multiplication (precedence 2) child under addition (precedence 1) parent

## Generator Integration

**From `latex_gen.py` lines 112-141:**

```python
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

**Asymmetric handling**: Left and right children are checked separately with `is_right` flag

## I/O Contract Test Cases

**Precedence tests:**

| Input | Expected Output | Category | Rule Applied |
|-------|-----------------|----------|---------------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Precedence | Rule 1: lower precedence |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Precedence | Rule 1: lower precedence |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Precedence | Rule 1: lower precedence on right |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Precedence | Rule 3: higher precedence |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | Precedence | Rule 3: higher precedence |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | Precedence | Rule 3: higher precedence |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | Precedence | Same level, commutative |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Complex | Rule 1 on both sides |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Complex | Mixed precedence |

**Key test cases that validate precedence**:

1. `5 3 + 2 *` - Addition must be parenthesized under multiplication
2. `2 3 4 * +` - Multiplication on right of addition needs NO parentheses
3. `1 2 + 3 4 + *` - Both sides need parentheses (both are lower precedence)

## Rust Migration Notes

### Type Mappings

| Python | Rust | Notes |
|--------|------|-------|
| `dict[str, int]` | `fn` match or `HashMap` | Hardcoded lookup |
| `bool` parameter `is_right` | `bool` parameter | Same semantic |
| `isinstance` check | Pattern match | Rust's type system |

### Pattern Changes

1. **Visitor dispatch**: Replace singledispatchmethod with match expression
   ```rust
   match child {
       Expr::Number(_) => false,
       Expr::BinaryOp(op) => { /* check rules */ }
   }
   ```

2. **Precedence table**: Can be a match expression instead of dictionary
   ```rust
   fn precedence(op: &str) -> u32 {
       match op {
           "+" | "-" => 1,
           "*" | "/" => 2,
           _ => panic!()
       }
   }
   ```

3. **Immutable reference checks**: Use pattern matching to check child type

### Special Handling

1. **Asymmetric left/right handling**: The `is_right` flag must be threaded through correctly
2. **String comparison**: Operator comparison via string lookup (can optimize with enums)
3. **Recursive formatting**: The _visit call must handle recursion correctly with proper formatting

---

# Consolidated Feature Dependencies

```
numbers
  ↓
addition
  ↓
subtraction
  ├→ addition (BinaryOp, operator handling)
  ↓
multiplication
  ├→ addition (BinaryOp, operator handling)
  ↓
division
  ├→ multiplication (precedence level)
  ├→ subtraction (non-commutativity)
  ↓
precedence
  ├→ all operators
```

**Implementation order** for Rust migration:
1. **numbers** - Foundation
2. **addition** - First operator, establishes patterns
3. **subtraction** - Adds negative number handling
4. **multiplication** - Introduces higher precedence level
5. **division** - Shares precedence with multiplication
6. **precedence** - Integrates all operators with correct parenthesization

---

# I/O Contract

From Phase 0 verification, the following test cases define the ground truth:

## Complete Test Results

**Total**: 21 tests
**Passed**: 18
**Failed**: 0
**Unsupported**: 3 (exponent operator)

### All Passing Test Cases

#### Basic Operations (6/6 passing)

1. Input: `5 3 +` → Output: `$5 + 3$`
2. Input: `5 3 -` → Output: `$5 - 3$`
3. Input: `4 7 *` → Output: `$4 \times 7$`
4. Input: `10 2 /` → Output: `$10 \div 2$`
5. Input: `3.14 2 *` → Output: `$3.14 \times 2$`
6. Input: `1.5 0.5 +` → Output: `$1.5 + 0.5$`

#### Chained Operations (3/3 passing)

7. Input: `1 2 + 3 + 4 +` → Output: `$1 + 2 + 3 + 4$`
8. Input: `5 3 - 2 -` → Output: `$5 - 3 - 2$`
9. Input: `100 10 / 5 / 2 /` → Output: `$100 \div 10 \div 5 \div 2$`

#### Operator Precedence (7/7 passing)

10. Input: `5 3 + 2 *` → Output: `$( 5 + 3 ) \times 2$`
11. Input: `2 3 + 4 *` → Output: `$( 2 + 3 ) \times 4$`
12. Input: `2 3 4 + *` → Output: `$2 \times ( 3 + 4 )$`
13. Input: `2 3 4 * +` → Output: `$2 + 3 \times 4$`
14. Input: `2 3 * 4 +` → Output: `$2 \times 3 + 4$`
15. Input: `5 3 * 2 +` → Output: `$5 \times 3 + 2$`
16. Input: `10 2 / 5 *` → Output: `$10 \div 2 \times 5$`

#### Complex Expressions (2/2 passing)

17. Input: `1 2 + 3 4 + *` → Output: `$( 1 + 2 ) \times ( 3 + 4 )$`
18. Input: `10 2 / 3 + 4 *` → Output: `$( 10 \div 2 + 3 ) \times 4$`

### Unsupported Features (3/3 correctly unsupported)

19. Input: `2 3 ^` → Status: UNSUPPORTED (Exponent operator not implemented)
20. Input: `2 3 ^ 4 *` → Status: UNSUPPORTED
21. Input: `2 3 4 ^ ^` → Status: UNSUPPORTED

## Output Format Rules

All outputs must follow these rules:

1. **Math mode delimiters**: `$...$` (LaTeX inline math mode)
2. **Spacing**: Single space around all operators
3. **Parentheses format**: `( expr )` with space after `(` and before `)`
4. **Number format**: Preserve exactly as input (e.g., `3.14` must output as `3.14`)
5. **Operator symbols**:
   - Addition: `+`
   - Subtraction: `-`
   - Multiplication: `\times` (LaTeX command)
   - Division: `\div` (LaTeX command)

## Validation Checklist for Rust Implementation

When migrating to Rust, verify:

- [ ] All 18 passing test cases produce identical output
- [ ] Output format exactly matches (spacing, parentheses, delimiters)
- [ ] LaTeX operators use correct commands (`\times`, `\div`)
- [ ] Precedence rules correctly parenthesize expressions
- [ ] Negative numbers handled in lexer
- [ ] Error messages for insufficient operands
- [ ] Floating-point numbers preserved as input
- [ ] Left-associativity of chained operations

---

# Summary Table: Feature to Module Mapping

| Feature | tokens.py | lexer.py | parser.py | ast_nodes.py | latex_gen.py | cli.py |
|---------|-----------|----------|-----------|--------------|--------------|--------|
| numbers | TokenType.NUMBER, Token | _scan_number | Number creation | Number | _visit_number | orchestration |
| addition | TokenType.PLUS | char=='+' scan | BinaryOp creation | BinaryOp | BINARY_OPS["+"] | orchestration |
| subtraction | TokenType.MINUS | char=='-' logic | BinaryOp creation | BinaryOp | BINARY_OPS["-"] | orchestration |
| multiplication | TokenType.MULT | char=='*' scan | BinaryOp creation | BinaryOp | BINARY_OPS["*"]="\times" | orchestration |
| division | TokenType.DIV | char=='/' scan | BinaryOp creation | BinaryOp | BINARY_OPS["/"]="\div" | orchestration |
| precedence | - | - | - | - | PRECEDENCE dict, _needs_parens | - |

---

# Rust Implementation Roadmap

This feature specification enables a systematic Rust implementation:

## Phase 1: Foundation
- Implement `tokens.rs` with Token struct and TokenType enum
- Implement `lexer.rs` with complete number scanning

## Phase 2: Basic Operators
- Implement `ast_nodes.rs` with Number and BinaryOp nodes
- Implement `parser.rs` with stack-based RPN parsing
- Implement `latex_gen.rs` with basic visitor pattern

## Phase 3: Precedence
- Add PRECEDENCE table to `latex_gen.rs`
- Implement `_needs_parens` logic
- Integrate parenthesization into BinaryOp visitor

## Phase 4: Integration
- Implement `cli.rs` with pipeline orchestration
- Implement `errors.rs` with ErrorFormatter
- Test against I/O contract

## Phase 5: Validation
- Run all 18 test cases
- Verify exact output format matching
- Validate error handling

---

**Document Status**: Complete
**Last Updated**: 2025-12-30
**Ready for**: Rust implementation (Phase 1 onwards)
