# PHASE 1: Comprehensive Feature-by-Feature Analysis

**Analysis Date**: 2025-12-30
**Source Directory**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
**Target Language**: Java
**Analysis Scope**: 7 Python modules analyzed for feature extraction

---

## I/O Contract

This section includes the I/O contract from Phase 0. All test cases below must pass in the Java implementation.

### Test Cases Summary
- **Total Tests**: 26
- **Passed in Python**: 24
- **Failed/Unsupported**: 2 (exponentiation "^" not implemented)

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

### Operator Precedence and Parentheses

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | PASS |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | PASS |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | PASS |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | PASS |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | PASS |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | PASS |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | PASS |

### Floating Point Support

| Input | Expected Output | Status |
|-------|-----------------|--------|
| `3.14 2 *` | `$3.14 \times 2$` | PASS |
| `1.5 0.5 +` | `$1.5 + 0.5$` | PASS |

---

## Feature 1: Numbers

### Overview
The **Numbers** feature handles parsing and output of numeric literals. This is the foundational feature that all other operators depend on.

### Token Types Required

**TokenType enum values**:
- `NUMBER`: Represents numeric literals (integers and decimals)
  - Token value: String representation of the number (e.g., "42", "3.14", "-2")
  - Position tracking: line and column attributes

**Python Code Reference** (tokens.py, lines 26-45):
```python
class TokenType(Enum):
    NUMBER = auto()  # Numeric values: 5, 3.14, -2
    # ... other types
    EOF = auto()
```

**Token Class** (tokens.py, lines 48-71):
```python
@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str
    line: int
    column: int
```

### AST Nodes Required

**Number Node** (ast_nodes.py, lines 41-56):
```python
@dataclass(frozen=True)
class Number(ASTNode):
    """Numeric literal node."""
    value: str  # String representation of the number
```

**Base ASTNode** (ast_nodes.py, lines 26-38):
```python
@dataclass(frozen=True)
class ASTNode:
    line: int      # 1-based line number
    column: int    # 1-based column number
```

### Lexer Logic

**Lexer Class** (lexer.py, lines 46-201):

The lexer handles number tokenization with the following approach:

1. **Character Detection** (lines 171-172):
   - Identifies digit characters using `char.isdigit()`
   - Positive integers start here

2. **Number Scanning** (lines 177-200):
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

3. **Negative Number Handling** (lines 153-162):
   - When "-" is encountered, lexer checks if digit follows immediately
   - If yes, treats as negative number prefix (passed to `_scan_number`)
   - If no, treats as subtraction operator

4. **Decimal Support**:
   - Accepts decimal point followed by digits
   - Preserves exact string representation in token value

### Parser Logic

**Parser Class** (parser.py, lines 57-184):

Number parsing (lines 107-113):
```python
if token.type == TokenType.NUMBER:
    # Push number onto stack
    num_node = Number(
        line=token.line, column=token.column, value=token.value
    )
    stack.append(num_node)
    self._advance()
```

**Stack-Based Processing**:
- When a NUMBER token is encountered, create a Number AST node
- Push immediately onto the evaluation stack
- This is the foundational operation for RPN evaluation

### Code Generation

**LaTeX Generator** (latex_gen.py, lines 99-109):

Number rendering (lines 100-109):
```python
@_visit.register
def _visit_number(self, node: Number) -> str:
    """Generate LaTeX for a number literal."""
    return node.value
```

**Key Behavior**:
- Numbers are output as-is (preserving exact string from source)
- No formatting or transformation applied
- Floating-point numbers maintain their decimal representation

### Error Handling

**Lexer Errors** (lexer.py, lines 21-43):
- LexerError is raised if unexpected character encountered
- Not directly applicable to numbers since digit recognition is straightforward

**Parser Errors** (parser.py, lines 34-54):
- Not applicable to numbers (they don't have operand requirements)

### Dependencies
- **No dependencies**: Numbers are the foundational feature
- All other features depend on this one

### Key Design Patterns

1. **Immutable Token/AST Design**:
   - Token uses `@dataclass(frozen=True)` for immutability
   - Number node is frozen dataclass
   - Thread-safe, no mutation concerns

2. **Position Tracking**:
   - Every token and AST node carries line/column
   - Enables precise error reporting

3. **String Representation**:
   - Numbers stored as strings, not native numeric types
   - Preserves exact input formatting (e.g., "3.14" stays "3.14")
   - Allows for arbitrary precision in LaTeX output

### Java Migration Notes

**Type Mappings**:
```
Python                      Java
---------------------------------------------
dataclass with frozen=True  final class with immutable fields
str                         String
Enum                        enum or int constants
list[Token]                 List<Token>
int                         int
```

**Pattern Changes**:
1. Replace `@dataclass(frozen=True)` with final Java classes
2. Replace Python Enum with Java enum or integer constants
3. Use StringBuilder for string building in lexer
4. String immutability is native in Java (no frozen needed)

**Special Handling**:
- Position tracking: Create a Position class or use separate int fields
- Token equality: Implement equals() and hashCode()
- Visitor pattern: Use double-dispatch or instanceof checks instead of singledispatchmethod

### Edge Cases

1. **Negative Numbers**:
   - "-5" is lexed as single NUMBER token "−5"
   - Only when "-" is immediately followed by digit
   - "5 -2" would be tokenized as NUMBER "5", MINUS "-", NUMBER "2"

2. **Decimal Numbers**:
   - "3.14" is valid
   - ".5" is NOT valid (must have digit before decimal)
   - "5." is valid (represents 5.0 equivalent)

3. **Leading Zeros**:
   - "007" is valid and output as "007"
   - No automatic normalization

4. **No Scientific Notation**:
   - "1e5" would be tokenized as NUMBER "1", followed by unknown "e"
   - Not supported in current implementation

---

## Feature 2: Addition

### Overview
The **Addition** feature adds two numbers or sub-expressions together using the `+` operator. This is the first binary operator and demonstrates the core RPN evaluation pattern.

### Token Types Required

**TokenType enum values**:
- `PLUS`: Represents the addition operator
  - Token value: "+"
  - No additional attributes

**Python Code Reference** (tokens.py, lines 26-45):
```python
class TokenType(Enum):
    PLUS = auto()  # + (addition)
```

### AST Nodes Required

**BinaryOp Node** (ast_nodes.py, lines 58-82):
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    """Binary operation node."""
    operator: str  # The operator string ("+", "-", "*", "/")
    left: Expr     # The left operand expression
    right: Expr    # The right operand expression
```

**Expr Type Union** (ast_nodes.py, line 90):
```python
Expr = Number | BinaryOp
```

### Lexer Logic

**Lexer Character Recognition** (lexer.py, lines 150-152):
```python
if char == "+":
    self._advance()
    return Token(TokenType.PLUS, "+", start_line, start_column)
```

**Behavior**:
- Single character match for "+"
- No state required (simple operator)
- Position tracking automatic via _advance()

### Parser Logic

**RPN Evaluation** (parser.py, lines 104-147):

Addition parsing (lines 115-147):
```python
elif token.type in (TokenType.PLUS, TokenType.MINUS, TokenType.MULT, TokenType.DIV):
    # Pop two operands and create binary operation
    if len(stack) < 2:
        raise ParserError(
            f"Operator '{token.value}' requires two operands", token
        )

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

**RPN Stack Evolution**:
```
Input:  "5 3 +"
Tokens: [NUMBER("5"), NUMBER("3"), PLUS("+"), EOF]

Step 1 (NUMBER "5"):  stack = [Number("5")]
Step 2 (NUMBER "3"):  stack = [Number("5"), Number("3")]
Step 3 (PLUS "+"):
    - pop right = Number("3")
    - pop left = Number("5")
    - create BinaryOp("+", left, right)
    - stack = [BinaryOp("+", 5, 3)]
Step 4 (EOF):         return BinaryOp("+", 5, 3)
```

**Key Algorithm**:
1. Right operand is popped first (top of stack)
2. Left operand is popped second (beneath right)
3. BinaryOp created with left as first operand, right as second
4. Result pushed back onto stack

### Code Generation

**LaTeX Generator** (latex_gen.py, lines 111-141):

Addition rendering:
```python
@_visit.register
def _visit_binary_op(self, node: BinaryOp) -> str:
    """Generate LaTeX for a binary operation."""
    op_latex = self.BINARY_OPS[node.operator]  # "+" for addition
    my_precedence = self.PRECEDENCE[node.operator]  # 1 for "+"

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

**Operator Mapping** (latex_gen.py, lines 47-52):
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "+": "+",  # Addition outputs as-is
    # ...
}
```

**Precedence** (latex_gen.py, lines 57-62):
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,  # Level 1: lowest precedence
    # ...
}
```

**Output Examples**:
- Input RPN: `5 3 +` → Output: `$5 + 3$`
- Input RPN: `1 2 + 3 + 4 +` → Output: `$1 + 2 + 3 + 4$` (no parens needed, left-associative)
- Input RPN: `5 3 + 2 *` → Output: `$( 5 + 3 ) \times 2$` (parens needed, + has lower precedence than *)

### Error Handling

**Parser Validation** (parser.py, lines 122-125):
```python
if len(stack) < 2:
    raise ParserError(
        f"Operator '{token.value}' requires two operands", token
    )
```

**Error Cases**:
- `+` with insufficient operands raises ParserError
- Error includes token position for precise error reporting

### Dependencies
- **Depends on**: Numbers (Feature 1)
- **Depended on by**: Precedence (Feature 6)

### Key Design Patterns

1. **Stack-Based RPN Evaluation**:
   - Operands pushed onto stack
   - Operator consumes two operands from top of stack
   - Result pushed back
   - Implicit tree building during parsing

2. **Lazy AST Building**:
   - AST not fully constructed during parsing
   - Structure emerges from RPN evaluation order
   - Matches mathematical evaluation order

3. **Operator Map Pattern**:
   - TokenType → operator string mapping
   - Allows easy addition of new operators

### Java Migration Notes

**Type Mappings**:
```
Python              Java
---------------------------------------------
str (operator)      String (final constant)
list[Expr]          Deque<Expr> or List<Expr>
ParserError         custom exception class
BinaryOp instance   new BinaryOp(...)
```

**Pattern Changes**:
1. Use Deque<ASTNode> instead of list for stack operations
2. Pre-define operator constants as final Strings
3. Custom exception class for ParserError
4. Stack.pop() in Python corresponds to Deque.pop() or stack.pop() in Java

**Special Handling**:
- Java stack order is same (LIFO)
- No surprises in addition implementation

### Test Cases from I/O Contract

From PHASE_0_IO_CONTRACT.md:

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 +` | `$5 + 3$` | Simple addition, no parens needed |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Left-associative chaining, no parens |

---

## Feature 3: Subtraction

### Overview
The **Subtraction** feature handles the `-` operator. Subtraction is left-associative and has the same precedence as addition. It is more complex than addition due to associativity rules: `5 - 3 - 2` means `(5 - 3) - 2`, not `5 - (3 - 2)`.

### Token Types Required

**TokenType enum values**:
- `MINUS`: Represents the subtraction operator
  - Token value: "-"
  - Overlaps with negative number prefix

**Python Code Reference** (tokens.py, lines 26-45):
```python
class TokenType(Enum):
    MINUS = auto()  # - (subtraction)
```

### AST Nodes Required

**BinaryOp Node** (same as addition):
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str  # "-" for subtraction
    left: Expr
    right: Expr
```

### Lexer Logic

**Lexer Character Recognition** (lexer.py, lines 153-162):
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

**Behavior**:
- "-" followed immediately by digit: treated as negative number
- "-" followed by whitespace or operator: treated as MINUS operator
- This matches RPN semantics where negative literals must be formed inline

**Example Tokenizations**:
- `-5` (start of input) → NUMBER("-5")
- `5 -3` → NUMBER("5"), NUMBER("-3")
- `5 - 3` → NUMBER("5"), MINUS("-"), NUMBER("3")

### Parser Logic

**RPN Evaluation** (parser.py, lines 115-147):

Subtraction parsing (identical code path to addition):
```python
elif token.type in (TokenType.PLUS, TokenType.MINUS, TokenType.MULT, TokenType.DIV):
    if len(stack) < 2:
        raise ParserError(...)

    right = stack.pop()
    left = stack.pop()

    op_map = {
        TokenType.MINUS: "-",
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

**RPN Stack Evolution**:
```
Input:  "5 3 - 2 -"
Tokens: [NUMBER("5"), NUMBER("3"), MINUS, NUMBER("2"), MINUS, EOF]

Step 1 (NUMBER "5"):  stack = [5]
Step 2 (NUMBER "3"):  stack = [5, 3]
Step 3 (MINUS "-"):   stack = [5 - 3]
Step 4 (NUMBER "2"):  stack = [5 - 3, 2]
Step 5 (MINUS "-"):   stack = [(5 - 3) - 2]
Step 6 (EOF):         return (5 - 3) - 2
```

### Code Generation

**LaTeX Generator** (latex_gen.py, lines 111-180):

Subtraction rendering with left-associativity handling:
```python
@_visit.register
def _visit_binary_op(self, node: BinaryOp) -> str:
    """Generate LaTeX for a binary operation."""
    op_latex = self.BINARY_OPS[node.operator]  # "-" for subtraction
    my_precedence = self.PRECEDENCE[node.operator]  # 1 for "-"

    left = self._visit(node.left)
    if self._needs_parens(node.left, my_precedence, is_right=False):
        left = f"( {left} )"

    right = self._visit(node.right)
    if self._needs_parens(node.right, my_precedence, is_right=True):
        right = f"( {right} )"  # Parens on right for non-commutative ops

    return f"{left} {op_latex} {right}"
```

**Parenthesization Rules** (latex_gen.py, lines 143-180):
```python
def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
    """Determine if a child expression needs parentheses."""
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

**Critical Rule for Subtraction**:
- When subtraction is the right operand of subtraction, it needs parentheses
- This preserves left-associativity: `5 - (3 - 2)` is different from `(5 - 3) - 2`
- Example: `5 3 - 2 - 1 -` outputs `$5 - 3 - 2 - 1$` (no parens because of left-associativity)

**Operator Mapping** (latex_gen.py, lines 47-52):
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "-": "-",  # Subtraction outputs as-is
}
```

**Precedence** (latex_gen.py, lines 57-62):
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "-": 1,  # Same level as addition
}
```

**Output Examples**:
- Input RPN: `5 3 -` → Output: `$5 - 3$`
- Input RPN: `5 3 - 2 -` → Output: `$5 - 3 - 2$` (left-associative chain)
- Input RPN: `5 3 - 2 *` → Output: `$( 5 - 3 ) \times 2$` (parens needed)

### Error Handling

**Parser Validation** (same as addition):
```python
if len(stack) < 2:
    raise ParserError(
        f"Operator '{token.value}' requires two operands", token
    )
```

### Dependencies
- **Depends on**: Numbers (Feature 1), Addition (Feature 2)
- **Depended on by**: Precedence (Feature 6)

### Key Design Patterns

1. **Left-Associativity Handling**:
   - Same precedence as addition (+)
   - Right side gets parentheses if it's also subtraction
   - Subtraction is explicitly non-commutative

2. **Unified Binary Operator Handling**:
   - Same code path as addition in parser
   - Only distinguished by operator string in AST

3. **Negative Number vs Subtraction Operator**:
   - Lookahead required in lexer
   - Context-dependent token classification

### Java Migration Notes

**Type Mappings**:
Same as Addition (Feature 2)

**Pattern Changes**:
Same as Addition (Feature 2)

**Special Handling**:
- Lexer lookahead for negative numbers:
  - Java equivalent of `self._peek().isdigit()` using `Character.isDigit()`
  - Need to check bounds before peeking

### Test Cases from I/O Contract

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 -` | `$5 - 3$` | Simple subtraction |
| `5 3 - 2 -` | `$5 - 3 - 2$` | Left-associative, no parens |

---

## Feature 4: Multiplication

### Overview
The **Multiplication** feature handles the `*` operator. Multiplication has higher precedence than addition/subtraction, so it binds tighter. This means `2 + 3 * 4` groups as `2 + (3 * 4)`.

### Token Types Required

**TokenType enum values**:
- `MULT`: Represents the multiplication operator
  - Token value: "*"

**Python Code Reference** (tokens.py, lines 26-45):
```python
class TokenType(Enum):
    MULT = auto()  # * (multiplication)
```

### AST Nodes Required

**BinaryOp Node** (same as addition/subtraction):
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str  # "*" for multiplication
    left: Expr
    right: Expr
```

### Lexer Logic

**Lexer Character Recognition** (lexer.py, lines 163-165):
```python
if char == "*":
    self._advance()
    return Token(TokenType.MULT, "*", start_line, start_column)
```

**Behavior**:
- Single character match for "*"
- No special handling or lookahead needed
- Simple operator

### Parser Logic

**RPN Evaluation** (parser.py, lines 115-147):

Multiplication parsing (same code path):
```python
elif token.type in (TokenType.PLUS, TokenType.MINUS, TokenType.MULT, TokenType.DIV):
    if len(stack) < 2:
        raise ParserError(...)

    right = stack.pop()
    left = stack.pop()

    op_map = {
        TokenType.MULT: "*",
        # ...
    }
    operator = op_map[token.type]

    op_node = BinaryOp(...)
    stack.append(op_node)
    self._advance()
```

**RPN Stack Evolution**:
```
Input:  "5 3 + 2 *"
Tokens: [NUMBER("5"), NUMBER("3"), PLUS, NUMBER("2"), MULT, EOF]

Step 1 (NUMBER "5"):  stack = [5]
Step 2 (NUMBER "3"):  stack = [5, 3]
Step 3 (PLUS "+"):    stack = [5 + 3]
Step 4 (NUMBER "2"):  stack = [5 + 3, 2]
Step 5 (MULT "*"):    stack = [(5 + 3) * 2]
Step 6 (EOF):         return (5 + 3) * 2
```

### Code Generation

**LaTeX Generator** (latex_gen.py, lines 111-141):

Multiplication rendering:
```python
@_visit.register
def _visit_binary_op(self, node: BinaryOp) -> str:
    """Generate LaTeX for a binary operation."""
    op_latex = self.BINARY_OPS[node.operator]  # "\times" for multiplication
    my_precedence = self.PRECEDENCE[node.operator]  # 2 for "*"

    left = self._visit(node.left)
    if self._needs_parens(node.left, my_precedence, is_right=False):
        left = f"( {left} )"

    right = self._visit(node.right)
    if self._needs_parens(node.right, my_precedence, is_right=True):
        right = f"( {right} )"

    return f"{left} {op_latex} {right}"
```

**Operator Mapping** (latex_gen.py, lines 47-52):
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "*": r"\times",  # Multiplication uses LaTeX \times symbol
}
```

**Precedence** (latex_gen.py, lines 57-62):
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "*": 2,  # Higher precedence than + and -
}
```

**Parenthesization Rules**:
- Left operand of multiplication: parentheses only if it's addition/subtraction (lower precedence)
- Right operand of multiplication: parentheses only if it's addition/subtraction (lower precedence)
- Multiplication is left-associative, but only on same precedence level

**Output Examples**:
- Input RPN: `4 7 *` → Output: `$4 \times 7$`
- Input RPN: `2 3 4 * +` → Output: `$2 + 3 \times 4$` (no parens on * because higher precedence)
- Input RPN: `5 3 + 2 *` → Output: `$( 5 + 3 ) \times 2$` (parens needed because + lower precedence)
- Input RPN: `5 3 * 2 +` → Output: `$5 \times 3 + 2$` (no parens on *; lower-precedence + is outer operation)

### Error Handling

**Parser Validation** (same as addition/subtraction):
```python
if len(stack) < 2:
    raise ParserError(
        f"Operator '{token.value}' requires two operands", token
    )
```

### Dependencies
- **Depends on**: Numbers (Feature 1), Addition (Feature 2), Subtraction (Feature 3)
- **Depended on by**: Precedence (Feature 6)

### Key Design Patterns

1. **Precedence Hierarchy**:
   - Multiplication at level 2 (higher than level 1 for +/-)
   - Causes automatic parenthesization of lower-precedence operations
   - No special handling needed in parser (RPN naturally respects this)

2. **LaTeX Special Characters**:
   - Raw string prefix `r"\times"` for LaTeX backslash
   - Important for Java migration: use raw string or escape backslashes

### Java Migration Notes

**Type Mappings**:
```
Python                  Java
---------------------------------------------
r"\times"              "\\times" (double backslash)
dict[str, str]         Map<String, String>
ClassVar                static final
```

**Pattern Changes**:
1. LaTeX escaping: Python raw strings become Java escaped strings
2. Map constants can be static final fields initialized in class body or static initializer

**Special Handling**:
- Ensure LaTeX backslashes are properly escaped in Java strings
- Test that `$4 \times 7$` output is correct (not `$4 \\times 7$`)

### Test Cases from I/O Contract

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `4 7 *` | `$4 \times 7$` | Simple multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | * has higher precedence than +, no parens on * |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | + is lower precedence, needs parens |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | * binds tighter, naturally on left side |

---

## Feature 5: Division

### Overview
The **Division** feature handles the `/` operator. Division has the same precedence as multiplication and is also left-associative and non-commutative. Like subtraction, the order of operands matters critically: `10 / 2 / 5` means `(10 / 2) / 5 = 1`, not `10 / (2 / 5) = 25`.

### Token Types Required

**TokenType enum values**:
- `DIV`: Represents the division operator
  - Token value: "/"

**Python Code Reference** (tokens.py, lines 26-45):
```python
class TokenType(Enum):
    DIV = auto()  # / (division)
```

### AST Nodes Required

**BinaryOp Node** (same as other binary operators):
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str  # "/" for division
    left: Expr
    right: Expr
```

### Lexer Logic

**Lexer Character Recognition** (lexer.py, lines 166-168):
```python
if char == "/":
    self._advance()
    return Token(TokenType.DIV, "/", start_line, start_column)
```

**Behavior**:
- Single character match for "/"
- No special handling
- Simple operator

### Parser Logic

**RPN Evaluation** (parser.py, lines 115-147):

Division parsing (same code path):
```python
elif token.type in (TokenType.PLUS, TokenType.MINUS, TokenType.MULT, TokenType.DIV):
    if len(stack) < 2:
        raise ParserError(...)

    right = stack.pop()
    left = stack.pop()

    op_map = {
        TokenType.DIV: "/",
        # ...
    }
    operator = op_map[token.type]

    op_node = BinaryOp(...)
    stack.append(op_node)
    self._advance()
```

**RPN Stack Evolution**:
```
Input:  "10 2 /"
Tokens: [NUMBER("10"), NUMBER("2"), DIV, EOF]

Step 1 (NUMBER "10"):  stack = [10]
Step 2 (NUMBER "2"):   stack = [10, 2]
Step 3 (DIV "/"):      stack = [10 / 2]
Step 4 (EOF):          return 10 / 2

Input:  "100 10 / 5 / 2 /"
Tokens: [NUMBER("100"), NUMBER("10"), DIV, NUMBER("5"), DIV, NUMBER("2"), DIV, EOF]

Step 1 (100):   stack = [100]
Step 2 (10):    stack = [100, 10]
Step 3 (/):     stack = [100 / 10]
Step 4 (5):     stack = [100 / 10, 5]
Step 5 (/):     stack = [(100 / 10) / 5]
Step 6 (2):     stack = [(100 / 10) / 5, 2]
Step 7 (/):     stack = [((100 / 10) / 5) / 2]
Step 8 (EOF):   return ((100 / 10) / 5) / 2
```

### Code Generation

**LaTeX Generator** (latex_gen.py, lines 111-141):

Division rendering with left-associativity handling:
```python
@_visit.register
def _visit_binary_op(self, node: BinaryOp) -> str:
    """Generate LaTeX for a binary operation."""
    op_latex = self.BINARY_OPS[node.operator]  # "\div" for division
    my_precedence = self.PRECEDENCE[node.operator]  # 2 for "/"

    left = self._visit(node.left)
    if self._needs_parens(node.left, my_precedence, is_right=False):
        left = f"( {left} )"

    right = self._visit(node.right)
    if self._needs_parens(node.right, my_precedence, is_right=True):
        right = f"( {right} )"  # Parens on right for non-commutative operators

    return f"{left} {op_latex} {right}"
```

**Operator Mapping** (latex_gen.py, lines 47-52):
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "/": r"\div",  # Division uses LaTeX \div symbol
}
```

**Precedence** (latex_gen.py, lines 57-62):
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "/": 2,  # Same precedence as multiplication
}
```

**Parenthesization Rules** (latex_gen.py, lines 143-180):
```python
def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
    # ...
    return (
        child_precedence == parent_precedence
        and is_right
        and child.operator in ("-", "/")  # Division is non-commutative
    )
```

**Critical Rule for Division**:
- When division is the right operand of division, it needs parentheses
- This preserves left-associativity: `10 / (2 / 5)` is different from `(10 / 2) / 5`
- Example: `100 10 / 5 / 2 /` outputs `$100 \div 10 \div 5 \div 2$` (no parens because of left-associativity)

**Output Examples**:
- Input RPN: `10 2 /` → Output: `$10 \div 2$`
- Input RPN: `100 10 / 5 / 2 /` → Output: `$100 \div 10 \div 5 \div 2$` (left-associative chain)
- Input RPN: `10 2 / 3 +` → Output: `$10 \div 2 + 3$` (/ is higher precedence, no parens needed)
- Input RPN: `10 2 / 3 + 4 *` → Output: `$( 10 \div 2 + 3 ) \times 4$` (addition lower precedence, needs parens)

### Error Handling

**Parser Validation** (same as other binary operators):
```python
if len(stack) < 2:
    raise ParserError(
        f"Operator '{token.value}' requires two operands", token
    )
```

### Dependencies
- **Depends on**: Numbers, Addition, Subtraction, Multiplication (Features 1-4)
- **Depended on by**: Precedence (Feature 6)

### Key Design Patterns

1. **Same-Precedence Operator Handling**:
   - Division at same level as multiplication (level 2)
   - Left-associativity rule (right side needs parens if same operator)
   - Same code structure as multiplication

2. **Non-Commutative Operator Tracking**:
   - Division explicitly in list `("-", "/")` for right-associativity check
   - Ensures `a / b / c` doesn't get incorrectly parenthesized

3. **LaTeX Rendering**:
   - Similar to multiplication: uses LaTeX command `\div`
   - Requires proper backslash escaping in Java

### Java Migration Notes

**Type Mappings**:
Same as Multiplication (Feature 4)

**Pattern Changes**:
Same as Multiplication (Feature 4)

**Special Handling**:
- Same LaTeX escaping concerns as multiplication
- Ensure `\div` is properly rendered

### Test Cases from I/O Contract

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `10 2 /` | `$10 \div 2$` | Simple division |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Left-associative chain, no parens |
| `10 2 / 3 +` | Covered in precedence tests | Higher precedence than addition |

---

## Feature 6: Precedence and Parenthesization

### Overview
The **Precedence** feature is the most complex: it handles operator precedence levels and automatic parenthesization of sub-expressions. Precedence affects which operations are performed first in the mathematical expression. RPN parsing naturally respects precedence by evaluation order, but generating LaTeX output requires careful parenthesization.

### Token Types Required
- Uses all existing operator token types: PLUS, MINUS, MULT, DIV
- No new token types needed

### AST Nodes Required
- Uses existing BinaryOp and Number nodes
- Precedence is entirely a code generation concern

### Lexer Logic
- No special lexer logic for precedence
- Lexer tokenizes all operators uniformly

### Parser Logic

**RPN Parsing Naturally Respects Precedence**:

The stack-based RPN parser automatically handles precedence through the order of operations. No special precedence rules are needed in the parser.

Example 1: `5 3 + 2 *` (higher precedence * applied to lower precedence +)
```
Stack evolution:
5        -> [5]
3        -> [5, 3]
+        -> [BinaryOp("+", 5, 3)]
2        -> [BinaryOp("+", 5, 3), 2]
*        -> [BinaryOp("*", BinaryOp("+", 5, 3), 2)]

The * operator receives the addition as its left operand because
of the order operations are performed. No parser precedence rules needed!
```

Example 2: `5 3 * 2 +` (lower precedence + applied to higher precedence *)
```
Stack evolution:
5        -> [5]
3        -> [5, 3]
*        -> [BinaryOp("*", 5, 3)]
2        -> [BinaryOp("*", 5, 3), 2]
+        -> [BinaryOp("+", BinaryOp("*", 5, 3), 2)]

The + operator receives the multiplication as its left operand.
Again, no special precedence handling needed!
```

### Code Generation

**Precedence-Based Parenthesization** (latex_gen.py, lines 111-180):

This is the critical feature. The generator must determine when to add parentheses.

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

def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
    """Determine if a child expression needs parentheses."""
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

**Precedence Levels** (latex_gen.py, lines 54-62):
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,  # Addition: level 1 (lowest)
    "-": 1,  # Subtraction: level 1
    "*": 2,  # Multiplication: level 2 (highest)
    "/": 2,  # Division: level 2
}
```

**Parenthesization Algorithm**:

1. **Child has lower precedence than parent**: Always parenthesize
   - Example: `5 + 3` as left operand of `* 2` → `( 5 + 3 ) * 2`

2. **Child has same precedence as parent on left side**: Never parenthesize (except for atoms)
   - Example: `5 - 3` as left operand of `- 2` → `5 - 3 - 2` (left-associative)

3. **Child has same precedence as parent on right side**:
   - If operator is non-commutative (`-` or `/`): parenthesize
     - Example: `3 - 2` as right operand of `5 -` → `5 - ( 3 - 2 )`
   - If operator is commutative (`+` or `*`): don't parenthesize (but this case doesn't occur in practice)

**Output Format Rules** (from latex_gen.py, lines 134-141):
```python
left = f"( {left} )"  # Spaces inside parens
right = f"( {right} )"
return f"{left} {op_latex} {right}"  # Space around operators
```

**LaTeX Operator Mapping** (latex_gen.py, lines 47-52):
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "+": "+",        # Addition: plain +
    "-": "-",        # Subtraction: plain -
    "*": r"\times",  # Multiplication: LaTeX \times
    "/": r"\div",    # Division: LaTeX \div
}
```

### Error Handling
- No error handling specific to precedence
- All errors handled by token/parser validation

### Dependencies
- **Depends on**: Numbers, Addition, Subtraction, Multiplication, Division (Features 1-5)
- **No dependents**: This is the final feature

### Key Design Patterns

1. **Visitor Pattern with Dispatch**:
   - Uses `@singledispatchmethod` to dispatch by type
   - Elegant for extensibility (adding new node types)
   - Java equivalent: virtual methods or instanceof checks

2. **Lazy Parenthesization**:
   - Parens added only when needed (minimal output)
   - Determined by comparison of precedence levels
   - Sub-expressions visited recursively

3. **Context-Aware Rendering**:
   - Same node type rendered differently based on context
   - Parent precedence and position (left/right) matter
   - Demonstrates recursive tree visiting pattern

### Detailed Parenthesization Examples

**Example 1: `5 3 + 2 *` → `$( 5 + 3 ) \times 2$`**
```
AST: BinaryOp("*", BinaryOp("+", 5, 3), 2)

Visiting BinaryOp("*", ...):
  op_latex = "\times"
  my_precedence = 2

  Visit left = BinaryOp("+", 5, 3):
    child_precedence = 1
    1 < 2? YES → needs parens
    Result: "( 5 + 3 )"

  Visit right = Number("2"):
    Not BinaryOp → no parens
    Result: "2"

  Return: "( 5 + 3 ) \times 2"
```

**Example 2: `5 3 * 2 +` → `$5 \times 3 + 2$`**
```
AST: BinaryOp("+", BinaryOp("*", 5, 3), 2)

Visiting BinaryOp("+", ...):
  op_latex = "+"
  my_precedence = 1

  Visit left = BinaryOp("*", 5, 3):
    child_precedence = 2
    2 < 1? NO → no parens
    Result: "5 \times 3"

  Visit right = Number("2"):
    Not BinaryOp → no parens
    Result: "2"

  Return: "5 \times 3 + 2"
```

**Example 3: `10 2 / 3 + 4 *` → `$( 10 \div 2 + 3 ) \times 4$`**
```
AST: BinaryOp("*", BinaryOp("+", BinaryOp("/", 10, 2), 3), 4)

Visiting BinaryOp("*", ...):
  my_precedence = 2

  Visit left = BinaryOp("+", ...):
    child_precedence = 1
    1 < 2? YES → needs parens

    Visiting BinaryOp("+", ...):
      Visit left = BinaryOp("/", ...):
        child_precedence = 2
        2 < 1? NO → no parens
        Result: "10 \div 2"

      Visit right = Number("3"):
        Result: "3"

      Return: "10 \div 2 + 3"

    Result with parens: "( 10 \div 2 + 3 )"

  Visit right = Number("4"):
    Result: "4"

  Return: "( 10 \div 2 + 3 ) \times 4"
```

**Example 4: `5 3 - 2 -` → `$5 - 3 - 2$`**
```
AST: BinaryOp("-", BinaryOp("-", 5, 3), 2)

Visiting BinaryOp("-", ...):
  my_precedence = 1

  Visit left = BinaryOp("-", 5, 3):
    child_precedence = 1
    1 < 1? NO
    same precedence on LEFT side? Never parenthesize (is_right=False)
    Result: "5 - 3"

  Visit right = Number("2"):
    Result: "2"

  Return: "5 - 3 - 2"
```

**Example 5: Hypothetical `5 - (3 - 2)` (if it existed)**
```
Note: RPN "5 3 2 - -" would create this:
AST: BinaryOp("-", 5, BinaryOp("-", 3, 2))

Visiting BinaryOp("-", ...):
  my_precedence = 1

  Visit left = Number("5"):
    Result: "5"

  Visit right = BinaryOp("-", 3, 2):
    child_precedence = 1
    Same precedence on RIGHT side
    child.operator = "-" (in non-commutative list)
    is_right = True
    → needs parens
    Result: "( 3 - 2 )"

  Return: "5 - ( 3 - 2 )"
```

### Java Migration Notes

**Type Mappings**:
```
Python                              Java
---------------------------------------------
dict[str, int]                      Map<String, Integer>
singledispatchmethod                visitor pattern or instanceof checks
ClassVar[dict[str, str]]            static final Map<String, String>
isinstance(child, BinaryOp)         child instanceof BinaryOp
```

**Pattern Changes**:
1. Replace `@singledispatchmethod` with:
   - Option A: Virtual methods (if using inheritance)
   - Option B: instanceof checks with casting
   - Option C: Visitor pattern (double-dispatch)

2. Static maps for constants:
```java
static final Map<String, Integer> PRECEDENCE =
    Map.ofEntries(
        Map.entry("+", 1),
        Map.entry("-", 1),
        Map.entry("*", 2),
        Map.entry("/", 2)
    );
```

3. Recursive visiting:
```java
String visitBinaryOp(BinaryOp node) {
    String opLatex = BINARY_OPS.get(node.operator);
    int myPrecedence = PRECEDENCE.get(node.operator);

    String left = visit(node.left);
    if (needsParens(node.left, myPrecedence, false)) {
        left = "( " + left + " )";
    }

    String right = visit(node.right);
    if (needsParens(node.right, myPrecedence, true)) {
        right = "( " + right + " )";
    }

    return left + " " + opLatex + " " + right;
}
```

**Special Handling**:
- LaTeX string constants: use double backslashes in Java `"\\times"`
- Collections: Java collections are less ergonomic than Python dicts
- String building: StringBuilder for efficiency in Java

### Test Cases from I/O Contract

| Input | Expected Output | Precedence Pattern |
|-------|-----------------|-------------------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Lower-prec child of higher-prec parent |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Same pattern |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Same precedence, different operand positions |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Both operands need parens |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Nested precedence levels |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | Higher-prec child of lower-prec parent (no parens) |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | Same precedence level, left-associative (no parens) |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | Higher-prec child as left operand (no parens) |

---

## Cross-Feature Analysis

### Feature Dependency Graph

```
Numbers (Feature 1)
    ↓
Addition (Feature 2) ──┐
    ↓                  │
Subtraction (Feature 3)├─→ Precedence (Feature 6)
    ↓                  │
Multiplication (Feature 4)┤
    ↓                  │
Division (Feature 5) ──┘
```

**Migration Order**: Features must be implemented in order: 1 → 2 → 3 → 4 → 5 → 6

### Unified Binary Operator Pattern

All binary operators follow an identical pattern:

**Lexer**:
1. Match single character
2. Create token with operator string
3. Return token

**Parser**:
1. Check for two operands on stack
2. Pop right, then left
3. Create BinaryOp node
4. Push result

**Code Generator**:
1. Look up LaTeX operator string
2. Look up precedence level
3. Visit left operand with precedence context
4. Visit right operand with precedence context
5. Render with appropriate parentheses

### Common Data Structures

**Tokens** (immutable):
- Used throughout pipeline
- Passed from lexer to parser
- Never modified

**AST Nodes** (immutable):
- Number: carries value string
- BinaryOp: carries operator string and child nodes
- All nodes carry position info (line, column)

**Collections**:
- List<Token> from lexer
- Stack<Expr> (implemented as list) in parser
- Map<String, String> and Map<String, Integer> for operator lookup

### Error Handling Pattern

All errors follow same pattern:
1. Create custom exception class (LexerError, ParserError)
2. Include position information (line, column, token)
3. Propagate to CLI layer
4. Format with ErrorFormatter for output

### Python Idioms to Java Equivalents

| Python | Java Equivalent |
|--------|-----------------|
| `@dataclass(frozen=True)` | `final class` with immutable fields |
| `Enum` | `enum` or `static final int` constants |
| `\|` union type | `extends` or interface |
| `list[T]` | `List<T>` (ArrayList, LinkedList) |
| `dict[K, V]` | `Map<K, V>` (HashMap, LinkedHashMap) |
| `isinstance(x, Type)` | `x instanceof Type` |
| `@singledispatchmethod` | visitor pattern or instanceof dispatch |
| String f-strings | String.format() or StringBuilder |
| `str.isdigit()` | `Character.isDigit(ch)` |
| `.append()` | `.add()` |
| `.pop()` | `.pop()` (Stack) or `.removeLast()` (Deque) |
| `__init__` | constructor |
| `self` | implicit in Java |

---

## CLI and Integration

### Overview (cli.py)

The CLI module orchestrates the entire pipeline:

**Main Function** (cli.py, lines 30-110):
```python
def main() -> int:
    # 1. Parse command-line arguments
    parser = argparse.ArgumentParser(...)
    args = parser.parse_args()

    # 2. Read input (file or stdin)
    if args.input == "-":
        text = sys.stdin.read()
    else:
        text = Path(args.input).read_text()

    # 3. Create error formatter for this input
    formatter = ErrorFormatter(text)

    # 4. Process: Lexer → Parser → LaTeXGenerator
    lexer = Lexer(text)
    tokens = lexer.tokenize()

    parser_obj = Parser(tokens)
    ast = parser_obj.parse()

    generator = LaTeXGenerator()
    latex = generator.generate(ast)

    # 5. Write output (file or stdout)
    if args.output is not None:
        args.output.write_text(latex + "\n")
    else:
        print(latex)

    return 0
```

**Error Handling** (cli.py, lines 87-94):
```python
try:
    # ... processing ...
except LexerError as e:
    formatted = formatter.format_error(e.message, e.line, e.column)
    print(formatted, file=sys.stderr)
    return 1
except ParserError as e:
    formatted = formatter.format_error(e.message, e.token.line, e.token.column)
    print(formatted, file=sys.stderr)
    return 1
```

### Module Imports and Dependencies

```python
# cli.py imports:
- argparse (stdlib)
- sys (stdlib)
- pathlib.Path (stdlib)
- rpn2tex.errors.ErrorFormatter (internal)
- rpn2tex.latex_gen.LaTeXGenerator (internal)
- rpn2tex.lexer.Lexer, LexerError (internal)
- rpn2tex.parser.Parser, ParserError (internal)
```

### Java Equivalents

**argparse**:
- Java: Apache Commons CLI, PICOCLI, or custom argument parsing

**sys.stdin/stdout**:
- Java: System.in, System.out, System.err

**pathlib.Path**:
- Java: java.nio.file.Path, java.nio.file.Files

**Exception hierarchy**:
- Java: Create custom exception classes extending Exception

---

## Summary of Implementation Scope

### What Must Be Implemented

1. **Token System**:
   - TokenType enum: NUMBER, PLUS, MINUS, MULT, DIV, EOF
   - Token class: immutable, with type, value, line, column

2. **AST Nodes**:
   - ASTNode base class: line, column
   - Number: value string
   - BinaryOp: operator string, left/right children
   - Expr type: union of expression types

3. **Lexer**:
   - Character-by-character scanning
   - Number parsing (integers and decimals)
   - Operator recognition
   - Whitespace handling
   - Line/column tracking
   - Error reporting

4. **Parser**:
   - Stack-based RPN evaluation
   - Operand validation (two operands required)
   - AST construction
   - Error reporting with token context

5. **Code Generator**:
   - Visitor pattern for tree traversal
   - Precedence-based parenthesization
   - LaTeX operator mapping
   - Output formatting

6. **Error Handling**:
   - ErrorFormatter for context-aware error messages
   - LexerError for lexical errors
   - ParserError for syntax errors

7. **CLI**:
   - Argument parsing
   - File I/O
   - Pipeline orchestration
   - Error formatting and display

### What Is NOT Implemented

- Exponentiation operator (^)
- Square root function (sqrt)
- Nth root function (root)
- Advanced precedence rules
- Infix notation parsing (RPN only)

---

## Notes for Java Migration

### Immutability

Python's frozen dataclasses are equivalent to Java final classes with immutable fields. Use:
- `private final` for all fields
- No setters
- Initialize in constructor
- Implement equals() and hashCode() if needed for comparisons

### Collections

Python's built-in collections are more convenient. Java requires explicit typing:
- `List<Token>` from `list[Token]`
- `Deque<ASTNode>` for stack operations
- `Map<String, Integer>` for precedence lookup
- Use Collections.unmodifiableMap() for constants if desired

### Visitor Pattern

Python's `@singledispatchmethod` is elegant but Java has no direct equivalent. Options:
1. **instanceof checks**: Simple, readable
   ```java
   if (node instanceof Number) {
       return visitNumber((Number) node);
   } else if (node instanceof BinaryOp) {
       return visitBinaryOp((BinaryOp) node);
   }
   ```

2. **Double-dispatch visitor**: Classic OOP pattern
   ```java
   public abstract String accept(Visitor visitor);
   ```

3. **Method references**: More functional approach

### String Building

Python f-strings and string concatenation are convenient. Java alternatives:
- `String.format()`
- `StringBuilder` for efficiency
- `String.join()` for list joining

### Error Classes

Create custom exception classes:
```java
class LexerError extends Exception {
    public final String message;
    public final int line;
    public final int column;
    // constructor and methods
}
```

### Testing

Key test categories based on I/O contract:
1. Number parsing and output
2. Single binary operations
3. Chained operations (associativity)
4. Parenthesization rules
5. Floating-point numbers
6. Error cases

All 24 passing test cases from Python must pass in Java.

---

## Conclusion

The rpn2tex application is a well-structured compiler-like pipeline with clear separation of concerns:

1. **Lexing**: Character-level tokenization
2. **Parsing**: Stack-based RPN evaluation with AST construction
3. **Code Generation**: Visitor-based tree traversal with precedence-aware parenthesization
4. **CLI Integration**: Orchestration and I/O management

The implementation uses immutable data structures throughout, making it thread-safe and predictable. The visitor pattern enables clean separation of code generation logic from AST representation.

Java migration requires translating Python idioms to Java equivalents but preserves the overall architecture and logic. The main challenges are:
1. Implementing the visitor pattern (singledispatchmethod → instanceof/visitor)
2. Managing collections with explicit generics
3. Proper LaTeX string escaping
4. Error formatting and display

All features are ordered by dependency, allowing incremental migration and testing following the feature-by-feature approach.
