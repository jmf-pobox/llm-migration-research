# PHASE 1: FEATURE-BASED MIGRATION SPECIFICATION
## rpn2tex Python to Rust Migration

**Document Version**: 1.0
**Target Language**: Rust
**Source Language**: Python 3.x
**Date**: 2025-12-29
**Status**: Ready for Implementation

---

## I/O Contract (Baseline from Phase 0)

### Overview
This migration specification is built upon the I/O contract established in Phase 0. All test cases below have been verified against the Python implementation and must pass with the Rust implementation.

### Test Results Summary
- **Total Test Cases**: 36
- **Passed**: 33
- **Failed**: 3 (exponentiation operator - NOT IMPLEMENTED)
- **Success Rate**: 91.7%

### Core Features (All Implemented)
1. Numbers: Parse and output numeric literals
2. Addition: Basic binary addition with chaining
3. Subtraction: Basic binary subtraction with chaining
4. Multiplication: Binary multiplication with higher precedence than addition
5. Division: Binary division with same precedence as multiplication
6. Precedence & Parentheses: Complex precedence handling

### Verified Test Cases

#### Numbers Feature
```
Input: "5"              → Output: "$5$"
Input: "3.14"           → Output: "$3.14$"
```

#### Addition Feature
```
Input: "5 3 +"          → Output: "$5 + 3$"
Input: "1 2 + 3 + 4 +"  → Output: "$1 + 2 + 3 + 4$"
```

#### Subtraction Feature
```
Input: "5 3 -"          → Output: "$5 - 3$"
Input: "5 3 - 2 -"      → Output: "$5 - 3 - 2$"
```

#### Multiplication Feature
```
Input: "4 7 *"          → Output: "$4 \times 7$"
Input: "2 3 4 * +"      → Output: "$2 + 3 \times 4$"
```

#### Division Feature
```
Input: "10 2 /"         → Output: "$10 \div 2$"
Input: "100 10 / 5 / 2 /"  → Output: "$100 \div 10 \div 5 \div 2$"
```

#### Precedence & Parentheses Feature
```
Input: "5 3 + 2 *"                    → Output: "$( 5 + 3 ) \times 2$"
Input: "2 3 + 4 *"                    → Output: "$( 2 + 3 ) \times 4$"
Input: "2 3 4 + *"                    → Output: "$2 \times ( 3 + 4 )$"
Input: "1 2 + 3 4 + *"                → Output: "$( 1 + 2 ) \times ( 3 + 4 )$"
Input: "10 2 / 3 + 4 *"               → Output: "$( 10 \div 2 + 3 ) \times 4$"
```

#### Mixed Operators Feature
```
Input: "5 3 * 2 +"     → Output: "$5 \times 3 + 2$"
Input: "10 2 / 5 *"    → Output: "$10 \div 2 \times 5$"
Input: "2 3 * 4 +"     → Output: "$2 \times 3 + 4$"
```

#### Floating-Point Operations
```
Input: "3.14 2 *"      → Output: "$3.14 \times 2$"
Input: "1.5 0.5 +"     → Output: "$1.5 + 0.5$"
```

### Error Cases
When the lexer encounters an unsupported character (like `^`), it produces:
```
Error: Unexpected character '^'

1 | 2 3 ^
  |     ^
```

---

# FEATURE SPECIFICATIONS

## Feature 1: Numbers

### Feature Overview
Parsing and rendering of numeric literals (both integers and floating-point numbers). Numbers are the foundational building blocks of all expressions and must be implemented first.

### Dependency Order
- **Position in Migration**: 1 (no dependencies)
- **Required Before**: All other features
- **Blocks**: Addition, Subtraction, Multiplication, Division, Precedence

### Python Implementation Details

#### tokens.py
```python
@dataclass(frozen=True)
class Token:
    """A lexical token with type, value, and position."""
    type: TokenType
    value: str
    line: int
    column: int

class TokenType(Enum):
    NUMBER = auto()  # Numeric values: 5, 3.14, -2
    EOF = auto()     # End of input
```

**Key Points**:
- Tokens are immutable dataclasses (frozen)
- Each token carries position information (line, column) for error reporting
- `value` field stores the string representation of the number

#### lexer.py - Number Scanning
```python
class Lexer:
    def _scan_number(self, prefix: str, start_line: int, start_column: int) -> Token:
        """Scan a numeric literal.

        Args:
            prefix: Any prefix already consumed (e.g., "-" for negatives)
            start_line: Line where number started
            start_column: Column where number started

        Returns:
            A NUMBER token.
        """
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

**Algorithm**:
1. Start with optional prefix (empty string or "-")
2. Consume all consecutive digits for integer part
3. Optionally consume decimal point and following digits
4. Return token with complete numeric string

**Number Recognition in _scan_token**:
```python
# Numbers
if char.isdigit():
    return self._scan_number("", start_line, start_column)

# Negative numbers
if char == "-":
    self._advance()
    if not self._at_end() and self._peek().isdigit():
        return self._scan_number("-", start_line, start_column)
    return Token(TokenType.MINUS, "-", start_line, start_column)
```

#### ast_nodes.py
```python
@dataclass(frozen=True)
class ASTNode:
    """Base class for all AST nodes."""
    line: int
    column: int

@dataclass(frozen=True)
class Number(ASTNode):
    """Numeric literal node."""
    value: str
```

**Key Points**:
- Numbers are immutable dataclasses
- Store numeric value as string (preserves original representation)
- Include position information for error reporting

#### parser.py - Number Handling
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

**RPN Parsing Logic for Numbers**:
- When a NUMBER token is encountered, create a Number AST node
- Push it onto the evaluation stack
- Move to the next token

#### latex_gen.py - Number Output
```python
@_visit.register
def _visit_number(self, node: Number) -> str:
    """Generate LaTeX for a number literal."""
    return node.value
```

**LaTeX Generation**:
- Simple pass-through: return the number value as-is
- No LaTeX escaping needed for numeric literals
- Wrapping in `$...$` happens at the top level

### Rust Translation Strategy

#### Type Mappings
| Python | Rust |
|--------|------|
| `Token` (dataclass) | `struct Token { type_: TokenType, value: String, line: usize, column: usize }` |
| `Enum` TokenType | `enum TokenType { Number, Eof, ... }` |
| `Number` (dataclass) | `struct Number { line: usize, column: usize, value: String }` |
| `str` (value field) | `String` (owned) or `&str` (borrowed) |
| `list[Expr]` (stack) | `Vec<Expr>` |

#### Pattern Changes

**Immutability**:
- Python uses `@dataclass(frozen=True)` for immutable dataclasses
- Rust's structs are immutable by default
- Use `#[derive(Clone)]` on structs for copy semantics where needed

**Position Tracking**:
- Python: `line: int`, `column: int` (0-based in many systems, 1-based here)
- Rust: Use `usize` for indices (consistent with 1-based line/column numbers)
- Consider `NonZeroUsize` if always >= 1

**String Handling**:
- Python stores `value: str` (string view)
- Rust should use `String` for owned values (created during lexing)
- Use `&str` only when borrowing from larger owned string

**Stack-based Parsing**:
- Python: `stack: list[Expr] = []`
- Rust: `let mut stack: Vec<Expr> = Vec::new();`
- Pop/push operations are similar

#### Special Handling

**Negative Numbers**:
- The lexer handles negative numbers specially (prefix "-" when followed by digit)
- This must be replicated in Rust with same logic
- Alternative: handle as unary negation (future enhancement)

**Floating-Point Representation**:
- Python stores numbers as strings (e.g., "3.14")
- Rust should do the same to preserve original formatting
- Don't parse as f64 unless calculation is needed (future feature)
- This maintains output fidelity: input "3.14" outputs "3.14" (not "3.1400000")

**Column Number Format**:
- Python uses 1-based columns (first column is 1, not 0)
- Rust should match this for consistency with error reporting
- Be careful with conversions between 0-based string indices and 1-based columns

### Key Implementation Details

**Token Lifecycle**:
1. Lexer scans input character-by-character
2. Recognizes number pattern: digit+ ('.' digit+)?
3. Creates Token with type=NUMBER, value="5", position info
4. Parser receives tokens and creates AST nodes
5. Generator visits Number nodes and outputs their value

**Example: Input "5"**
1. Lexer: scan_number("", 1, 1) → Token(NUMBER, "5", 1, 1)
2. Parser: Create Number(line=1, column=1, value="5"), push to stack
3. Generator: Visit Number node → returns "5"
4. Final output: "$5$"

**Example: Input "3.14"**
1. Lexer:
   - Consume '3' → value = "3"
   - See '.' → consume it → value = "3."
   - Consume '1', '4' → value = "3.14"
   - Return Token(NUMBER, "3.14", 1, 1)
2. Parser: Create Number(line=1, column=1, value="3.14")
3. Generator: Returns "3.14"
4. Final output: "$3.14$"

**Whitespace Handling**:
- Whitespace delimiters tokens but is otherwise ignored
- Numbers cannot contain whitespace (e.g., "3 . 14" is three tokens)
- Lexer skips whitespace before calling _scan_token

### Test Cases for Numbers Feature

#### Integration Test Scenario
```python
# Test: Integer parsing
input_text = "5"
lexer = Lexer(input_text)
tokens = lexer.tokenize()
assert len(tokens) == 2  # NUMBER, EOF
assert tokens[0].type == TokenType.NUMBER
assert tokens[0].value == "5"

parser = Parser(tokens)
ast = parser.parse()
assert isinstance(ast, Number)
assert ast.value == "5"

generator = LaTeXGenerator()
output = generator.generate(ast)
assert output == "$5$"
```

#### Edge Cases

1. **Single digit**: "0" → "$0$"
2. **Multi-digit**: "12345" → "$12345$"
3. **Decimal point**: "3.14" → "$3.14$"
4. **Leading zero**: "01" → "$01$" (preserved as-is)
5. **Trailing decimal**: "5." → "$5.$" (mathematically odd but preserved)
6. **Negative number**: "-5" → "$-5$"
7. **Very long decimal**: "3.14159265358979" → "$3.14159265358979$"

#### Invalid Cases (should error)
- Start with decimal: ".5" → LexerError (. not recognized at start)
- Multiple decimals: "3.14.15" → Token(NUMBER, "3.14"), then error on second "."
- Letter in number: "3a14" → Token(NUMBER, "3"), then error on 'a'

### Rust Implementation Checklist
- [ ] Define `Token` struct with type, value, line, column
- [ ] Define `TokenType` enum with at least `Number` and `Eof` variants
- [ ] Implement `Lexer` struct with position tracking
- [ ] Implement `Lexer::_peek()` - look at current char without consuming
- [ ] Implement `Lexer::_advance()` - consume char and update position
- [ ] Implement `Lexer::_scan_number()` - scan numeric literal
- [ ] Implement `Lexer::tokenize()` - main entry point
- [ ] Define `ASTNode` trait or base struct
- [ ] Define `Number` struct with value, line, column
- [ ] Implement `Parser` struct with stack-based parsing
- [ ] Implement `Parser::parse()` - handles NUMBER tokens
- [ ] Define `LaTeXGenerator` struct
- [ ] Implement `LaTeXGenerator::generate()` - entry point with $ wrapping
- [ ] Implement visitor for Number nodes
- [ ] Unit tests for each component
- [ ] Integration test: "5" → "$5$"
- [ ] Integration test: "3.14" → "$3.14$"

---

## Feature 2: Addition

### Feature Overview
Parse and render the binary addition operator (+). This is the first binary operator and introduces precedence concepts. Addition is left-associative and has lower precedence than multiplication/division.

### Dependency Order
- **Position in Migration**: 2
- **Depends On**: Numbers (Feature 1)
- **Required Before**: Precedence handling
- **Blocks**: Complex expressions

### Python Implementation Details

#### tokens.py - Token Definition
```python
class TokenType(Enum):
    NUMBER = auto()
    PLUS = auto()   # + (addition)
    EOF = auto()
```

#### lexer.py - Addition Recognition
```python
def _scan_token(self) -> Token:
    """Scan and return the next token."""
    start_line = self.line
    start_column = self.column
    char = self._peek()

    # Single-character operators
    if char == "+":
        self._advance()
        return Token(TokenType.PLUS, "+", start_line, start_column)
```

**Algorithm**:
1. See character '+'
2. Advance past it
3. Return Token(PLUS, "+", position)

#### ast_nodes.py - BinaryOp Definition
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    """Binary operation node.

    Represents operations with two operands: +, -, *, /
    """
    operator: str      # The operator string ("+", "-", "*", "/")
    left: Expr         # The left operand expression
    right: Expr        # The right operand expression
```

**Key Points**:
- `operator` is a string (not an enum)
- Both operands are Expr (recursive type allowing nested operations)
- Immutable structure

#### parser.py - Addition Parsing
```python
def parse(self) -> Expr:
    """Parse tokens into an AST."""
    stack: list[Expr] = []

    while not self._at_end():
        token: Token = self._current()

        if token.type == TokenType.NUMBER:
            # Push number onto stack
            num_node = Number(...)
            stack.append(num_node)
            self._advance()

        elif token.type in (TokenType.PLUS, ...):
            # Pop two operands and create binary operation
            if len(stack) < 2:
                raise ParserError(...)

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

**RPN Parsing Algorithm for Addition**:
1. Push operands to stack as they're encountered
2. When PLUS token appears, pop two operands
3. Create BinaryOp node with operator="+"
4. Push result back to stack
5. Continue until EOF
6. Stack should have exactly one element (the root AST)

**Example: "5 3 +"**
```
Token: NUMBER(5) → push Number(5) → stack = [5]
Token: NUMBER(3) → push Number(3) → stack = [5, 3]
Token: PLUS(+)   → pop 3, pop 5 → create BinaryOp("+", 5, 3) → stack = [BinaryOp(+, 5, 3)]
Token: EOF       → return stack[0]
```

#### latex_gen.py - Addition Output
```python
class LaTeXGenerator:
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

**LaTeX Generation for Addition**:
- Map "+" → "+" (plus sign in LaTeX)
- Precedence level 1 (lowest)
- No parentheses needed when adding two numbers
- Parentheses needed when addition is operand of higher-precedence operation

**Example: "5 3 +"**
1. BinaryOp("+", Number(5), Number(3))
2. Visit: left = "5", right = "3", no parens needed
3. Result: "5 + 3"
4. Final: "$5 + 3$"

### Rust Translation Strategy

#### Type Mappings
| Python | Rust |
|--------|------|
| `TokenType.PLUS` | `TokenType::Plus` |
| `BinaryOp` (dataclass) | `struct BinaryOp { operator: String, left: Box<Expr>, right: Box<Expr> }` |
| `operator: str` | `operator: String` (or `&str` with lifetime) |
| `left: Expr` | `left: Box<Expr>` (recursive type needs box) |
| `dict[str, str]` | `HashMap<String, String>` or pattern matching |

#### Pattern Changes

**Recursive Types**:
- Python: `left: Expr` works because Python uses GC
- Rust: Cannot have recursive types without indirection
- Solution: Use `Box<Expr>` for owned, heap-allocated expressions
- Or: Use enum with variants instead of dataclass composition

**Pattern Matching vs. Dict Lookup**:
- Python: `op_map[token.type]` - dictionary lookup
- Rust: Match expressions on `token.type` or use HashMap

**Operator Strings**:
- Python stores operator as string: `operator: str`
- Rust: Use `String` for owned or `&'static str` for literals
- Consider enum for operators instead: `enum Op { Add, Sub, Mul, Div }`

**Recursion in Visitor**:
- Python: `self._visit(node.left)` - recursive method call
- Rust: Need to handle ownership carefully
- Use `&` to borrow when visiting children

#### Special Handling

**Stack Safety**:
- Python: `stack: list[Expr] = []` with dynamic growth
- Rust: `Vec<Expr>` works similarly, but be careful with moves
- After `pop()`, ownership transfers to variable (no borrow issues)

**Parentheses Logic**:
- Python: `_needs_parens()` method checks precedence
- Rust: Same logic, but need to match on operator type
- BinaryOp is only variant that can have lower precedence

**Error Handling**:
- Python: `ParserError` exception with token context
- Rust: Return `Result<Expr, ParserError>` or use custom error type
- Consider anyhow/thiserror crates for ergonomics

### Key Implementation Details

**Left-Associativity**:
- Addition is left-associative: "1 + 2 + 3" = "(1 + 2) + 3"
- In RPN: "1 2 + 3 +"
- Stack evolution:
  - "1" → [1]
  - "2" → [1, 2]
  - "+" → [1+2]
  - "3" → [1+2, 3]
  - "+" → [(1+2)+3]
- AST structure reflects this: BinaryOp("+", BinaryOp("+", 1, 2), 3)
- This happens naturally with stack-based parsing

**Addition Does Not Parenthesize**:
- Addition has lowest precedence (along with subtraction)
- Never needs parentheses as left operand (precedence not lower)
- Never needs parentheses as right operand of another addition
- Example: "5 + 3 + 2" (no internal parentheses needed)

**Addition Parenthesizes Lower**:
- When addition is operand of multiplication: "(5 + 3) * 2"
- Happens in precedence check: my_precedence=1 > child_precedence=n/a (Number)
- Binary addition is child: child_precedence=1 < parent_precedence=2 (mult) → needs parens

### Test Cases for Addition Feature

#### Basic Addition
```
Input: "5 3 +"
Tokens: [NUMBER(5), NUMBER(3), PLUS(+), EOF]
AST: BinaryOp("+", Number(5), Number(3))
Output: "$5 + 3$"
```

#### Chained Addition
```
Input: "1 2 + 3 + 4 +"
Tokens: [NUMBER(1), NUMBER(2), PLUS, NUMBER(3), PLUS, NUMBER(4), PLUS, EOF]
AST: BinaryOp("+", BinaryOp("+", BinaryOp("+", 1, 2), 3), 4)
Output: "$1 + 2 + 3 + 4$"
```

#### With Floats
```
Input: "1.5 0.5 +"
Output: "$1.5 + 0.5$"
```

#### Error Cases

**Missing Operand**:
```
Input: "5 +"
Tokens: [NUMBER(5), PLUS(+), EOF]
Parsing:
  - NUMBER(5) → push 5 → stack = [5]
  - PLUS → need 2 operands, only have 1 → ParserError
```

**Extra Operand**:
```
Input: "5 3 2 +"
Tokens: [NUMBER(5), NUMBER(3), NUMBER(2), PLUS, EOF]
Parsing:
  - NUMBER(5) → stack = [5]
  - NUMBER(3) → stack = [5, 3]
  - NUMBER(2) → stack = [5, 3, 2]
  - PLUS → pop 2, pop 3 → create BinaryOp("+", 3, 2) → stack = [5, 3+2]
  - EOF → ERROR: 2 values on stack (missing operator)
```

### Rust Implementation Checklist
- [ ] Add `Plus` variant to `TokenType` enum
- [ ] Update lexer to recognize '+' character
- [ ] Define `BinaryOp` struct (with Box for recursive types)
- [ ] Update `Expr` type alias/enum to include BinaryOp
- [ ] Add precedence map/match logic
- [ ] Implement addition parsing in `Parser::parse()`
- [ ] Add visitor for BinaryOp in LaTeX generator
- [ ] Implement `_needs_parens()` logic
- [ ] Handle operator string mapping (+, -, *, /)
- [ ] Unit tests for lexer with PLUS token
- [ ] Unit tests for parser with addition
- [ ] Integration test: "5 3 +" → "$5 + 3$"
- [ ] Integration test: "1 2 + 3 + 4 +" → "$1 + 2 + 3 + 4$"
- [ ] Error test: "5 +" should fail
- [ ] Error test: "5 3 2 +" should fail

---

## Feature 3: Subtraction

### Feature Overview
Parse and render the binary subtraction operator (-). Subtraction is similar to addition but introduces important considerations for associativity and precedence when mixed with other operators.

### Dependency Order
- **Position in Migration**: 3
- **Depends On**: Numbers (Feature 1)
- **Related To**: Addition (Feature 2) - same precedence
- **Required Before**: Precedence handling with mixed operators

### Python Implementation Details

#### tokens.py - Token Definition
```python
class TokenType(Enum):
    MINUS = auto()  # - (subtraction)
```

#### lexer.py - Subtraction Recognition
```python
def _scan_token(self) -> Token:
    """Scan and return the next token."""
    start_line = self.line
    start_column = self.column
    char = self._peek()

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

**Algorithm**:
1. See character '-'
2. Advance past it
3. Look ahead: if next char is digit, it's a negative number → scan_number
4. Otherwise, it's the subtraction operator → return MINUS token

**Key Distinction**:
- "-5" (minus followed by digit) → NUMBER token with value "-5"
- "5 -" (minus standalone) → MINUS operator token
- "5 - 3" → MINUS operator token (with whitespace)

#### ast_nodes.py
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str      # "-" for subtraction
    left: Expr
    right: Expr
```

#### parser.py - Subtraction Parsing
```python
elif token.type in (TokenType.PLUS, TokenType.MINUS, TokenType.MULT, TokenType.DIV):
    if len(stack) < 2:
        raise ParserError(...)

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

**RPN Parsing for Subtraction**:
- Same algorithm as addition
- TokenType.MINUS maps to operator string "-"
- Push and pop work identically

**Example: "5 3 -"**
```
[5] [3] [-] → pop 3 (right), pop 5 (left) → BinaryOp("-", 5, 3) → [BinaryOp]
```

#### latex_gen.py - Subtraction Output
```python
BINARY_OPS = {
    "+": "+",
    "-": "-",      # minus sign
    "*": r"\times",
    "/": r"\div",
}

PRECEDENCE = {
    "+": 1,
    "-": 1,        # same level as addition
    "*": 2,
    "/": 2,
}
```

**LaTeX Output**:
- Operator "-" maps to "-" (simple minus sign)
- Precedence 1 (same as addition)
- Parentheses logic is critical here

### Rust Translation Strategy

#### Type Mappings
| Python | Rust |
|--------|------|
| `TokenType.MINUS` | `TokenType::Minus` |
| operator "-" | `"-".to_string()` or `&"static str` |

#### Pattern Changes

**Negative Number vs. Operator Ambiguity**:
- Python resolves this in lexer with lookahead
- Rust should replicate: check if '-' is followed by digit
- Important for correct tokenization

**Associativity Handling**:
- Subtraction is left-associative: "5 - 3 - 2" = "(5 - 3) - 2" = 0
- Right-associative would be: "5 - (3 - 2)" = 4
- Stack-based parser naturally gives left-associativity

#### Special Handling

**Right-Associativity Not Needed**:
- Stack-based RPN parser handles associativity naturally
- "5 3 - 2 -" automatically creates left-associative tree
- No special logic needed beyond what addition uses

**Parenthesization on Right Side**:
- When subtraction is RIGHT operand of subtraction, needs parens
- Example: "5 - (3 - 2)" in infix
- In RPN: "5 3 2 - -" (not valid) vs "5 3 2 - -" (wait, this is confusing)
- Actually in RPN notation, you can't directly express "5 - (3 - 2)"
- You'd need "5 3 2 - -" which is "5 - (3 - 2)" = 5 - 1 = 4
- AST from parser: BinaryOp("-", 5, BinaryOp("-", 3, 2))
- When generating LaTeX: inner BinaryOp has child_precedence=1, parent_precedence=1, is_right=True
- Subtraction is in the exclusion list: `child.operator in ("-", "/")`
- So it DOES get parentheses: "5 - ( 3 - 2 )"

**Precedence Check Logic**:
```python
def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
    # Equal precedence on right side needs parens for non-commutative operators
    return (
        child_precedence == parent_precedence
        and is_right
        and child.operator in ("-", "/")
    )
```

This ensures:
- "5 + 3 + 2" → no parens (addition is commutative)
- "5 - 3 - 2" → no parens (left operand of subtraction)
- "5 - (3 - 2)" → parens on right operand of subtraction

### Key Implementation Details

**Left-Associativity in RPN**:
- "5 3 - 2 -" evaluates as ((5 - 3) - 2) = (2 - 2) = 0
- Stack evolution:
  1. "5" → [5]
  2. "3" → [5, 3]
  3. "-" → pop 3, pop 5 → BinaryOp("-", 5, 3) → [5-3]
  4. "2" → [5-3, 2]
  5. "-" → pop 2, pop (5-3) → BinaryOp("-", 5-3, 2) → [(5-3)-2]

**Parenthesization Rules**:
1. Subtraction as left operand: no parens needed
   - "(5 - 3) - 2" → "5 - 3 - 2" (no parens)
2. Subtraction as right operand of addition: no parens needed
   - "5 + (3 - 2)" → "5 + 3 - 2" (no parens, same precedence)
3. Subtraction as right operand of subtraction: NEEDS parens
   - "5 - (3 - 2)" → "5 - ( 3 - 2 )" (parens required)
4. Subtraction as operand of multiplication: NEEDS parens
   - "(5 - 3) * 2" → "( 5 - 3 ) \times 2" (parens required)

### Test Cases for Subtraction Feature

#### Basic Subtraction
```
Input: "5 3 -"
Output: "$5 - 3$"
```

#### Chained Subtraction
```
Input: "5 3 - 2 -"
AST: BinaryOp("-", BinaryOp("-", 5, 3), 2)
Output: "$5 - 3 - 2$"
Expected Evaluation: (5-3)-2 = 2-2 = 0
```

#### Subtraction with Floats
```
Input: "5.5 2.3 -"
Output: "$5.5 - 2.3$"
```

#### Mixed with Multiplication
```
Input: "5 3 - 2 *"
AST: BinaryOp("*", BinaryOp("-", 5, 3), 2)
Output: "$( 5 - 3 ) \times 2$"
```

#### Right-Side Parenthesization
```
Input: "5 3 2 - -"
Evaluation: 5 - (3 - 2) = 5 - 1 = 4
AST: BinaryOp("-", 5, BinaryOp("-", 3, 2))
Output: "$5 - ( 3 - 2 )$"
```

### Rust Implementation Checklist
- [ ] Add `Minus` variant to `TokenType` enum
- [ ] Update lexer to handle '-' with lookahead for numbers
- [ ] Implement negative number detection in lexer
- [ ] Test lexer distinguishes "-5" (NUMBER) from "- " (MINUS)
- [ ] Update parser to handle TokenType::Minus
- [ ] Map MINUS to operator string "-"
- [ ] Update operator match in LaTeX generator
- [ ] Verify `_needs_parens()` handles right-associativity case
- [ ] Unit tests for lexer with MINUS vs. negative numbers
- [ ] Integration test: "5 3 -" → "$5 - 3$"
- [ ] Integration test: "5 3 - 2 -" → "$5 - 3 - 2$"
- [ ] Integration test: "5 3 2 - -" → "$5 - ( 3 - 2 )$"
- [ ] Error test: "5 -" (missing operand)
- [ ] Error test: "- 5" (negative number vs operator ambiguity)

---

## Feature 4: Multiplication

### Feature Overview
Parse and render the binary multiplication operator (*) with higher precedence than addition and subtraction. Multiplication is left-associative like addition but has distinct precedence implications.

### Dependency Order
- **Position in Migration**: 4
- **Depends On**: Numbers (Feature 1)
- **Interacts With**: Addition, Subtraction (Features 2, 3)
- **Enables**: Precedence handling (Feature 6)

### Python Implementation Details

#### tokens.py - Token Definition
```python
class TokenType(Enum):
    MULT = auto()  # * (multiplication)
```

#### lexer.py - Multiplication Recognition
```python
if char == "*":
    self._advance()
    return Token(TokenType.MULT, "*", start_line, start_column)
```

**Algorithm**: Simple single-character token recognition.

#### ast_nodes.py
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str      # "*" for multiplication
    left: Expr
    right: Expr
```

#### parser.py - Multiplication Parsing
```python
elif token.type in (TokenType.PLUS, TokenType.MINUS, TokenType.MULT, TokenType.DIV):
    if len(stack) < 2:
        raise ParserError(...)

    right = stack.pop()
    left = stack.pop()

    op_map = {
        TokenType.MULT: "*",
        ...
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

**RPN Parsing for Multiplication**:
- Identical to addition/subtraction in parsing logic
- Operator string "*" maps from TokenType.MULT
- Precedence determines parenthesization later

**Example: "4 7 *"**
```
[4] [7] [*] → pop 7, pop 4 → BinaryOp("*", 4, 7)
```

#### latex_gen.py - Multiplication Output
```python
BINARY_OPS = {
    "*": r"\times",    # LaTeX multiplication symbol
    ...
}

PRECEDENCE = {
    "*": 2,            # HIGHER precedence than addition (level 1)
    ...
}
```

**LaTeX Output**:
- Operator "*" maps to `\times` (multiplication symbol in LaTeX)
- Precedence 2 (higher than addition/subtraction at level 1)
- This causes addition operands to be parenthesized

**Example: "2 3 4 * +"**
```
AST: BinaryOp("+", Number(2), BinaryOp("*", Number(3), Number(4)))
LaTeX generation:
  - Visit BinaryOp("+", ...)
  - Left: visit Number(2) → "2"
  - Right: visit BinaryOp("*", ...)
    - Left: "3" (no parens)
    - Right: "4" (no parens)
    - Result: "3 \times 4"
  - Check _needs_parens(BinaryOp("*", ...), parent_precedence=1, is_right=True):
    - child_precedence=2 > parent_precedence=1 → False (no parens)
  - Result: "2 + 3 \times 4"
Output: "$2 + 3 \times 4$"
```

**Example: "5 3 + 2 *"**
```
AST: BinaryOp("*", BinaryOp("+", Number(5), Number(3)), Number(2))
LaTeX generation:
  - Visit BinaryOp("*", ...)
  - Left: visit BinaryOp("+", ...)
    - Result: "5 + 3"
  - Check _needs_parens(BinaryOp("+", ...), parent_precedence=2, is_right=False):
    - child_precedence=1 < parent_precedence=2 → True (NEEDS parens)
  - Left with parens: "( 5 + 3 )"
  - Right: visit Number(2) → "2"
  - Result: "( 5 + 3 ) \times 2"
Output: "$( 5 + 3 ) \times 2$"
```

### Rust Translation Strategy

#### Type Mappings
| Python | Rust |
|--------|------|
| `TokenType.MULT` | `TokenType::Mult` |
| `r"\times"` | `r#"\times"#` (raw string in Rust) |

#### Pattern Changes

**LaTeX Escaping**:
- Python: `r"\times"` (raw string prefix)
- Rust: `r#"\times"#` or `"\\times"` (double backslash in normal string)
- Be careful with string literal escaping

**Precedence Levels**:
- No change from Python
- Still 2 for multiplication/division
- Still 1 for addition/subtraction

#### Special Handling

**Commutative Operator**:
- Multiplication is commutative: 3 * 4 = 4 * 3
- No special parenthesization needed on right side
- Unlike subtraction/division, never needs right-side parens for same precedence

**Precedence Dominates**:
- Multiplication always binds tighter than addition
- Parenthesization logic determined entirely by precedence values

### Key Implementation Details

**Precedence-Driven Parenthesization**:
- Multiplication (level 2) > Addition (level 1)
- When addition is operand of multiplication, it gets parenthesized
- When multiplication is operand of addition, it doesn't
- Example comparison:
  - "5 + 3 * 4" (no parens) = 5 + (3*4) = 5 + 12 = 17
  - "(5 + 3) * 4" (with parens) = 8 * 4 = 32

**No Special Right-Side Rules**:
- Multiplication is left-associative: "2 * 3 * 4" = "(2 * 3) * 4" = 24
- But right-side of multiplication doesn't need special parenthesization
- The `_needs_parens` check excludes "*" from the right-side exception:
  ```python
  return (child_precedence == parent_precedence and is_right
          and child.operator in ("-", "/"))
  ```
- "*" not in list, so "2 * 3 * 4" outputs as "2 \times 3 \times 4"

### Test Cases for Multiplication Feature

#### Basic Multiplication
```
Input: "4 7 *"
Output: "$4 \times 7$"
```

#### Multiplication with Floats
```
Input: "3.14 2 *"
Output: "$3.14 \times 2$"
```

#### Multiplication Precedence Over Addition
```
Input: "2 3 4 * +"
AST: BinaryOp("+", 2, BinaryOp("*", 3, 4))
Output: "$2 + 3 \times 4$"
No parentheses: multiplication has higher precedence
```

#### Addition Parenthesized Under Multiplication
```
Input: "5 3 + 2 *"
AST: BinaryOp("*", BinaryOp("+", 5, 3), 2)
Output: "$( 5 + 3 ) \times 2$"
Parentheses required: lower precedence operand
```

#### Chained Multiplication
```
Input: "2 3 4 * *"
AST: BinaryOp("*", BinaryOp("*", 2, 3), 4)
Output: "$2 \times 3 \times 4$"
Evaluation: (2*3)*4 = 6*4 = 24
```

#### Complex Precedence
```
Input: "1 2 + 3 4 + *"
AST: BinaryOp("*", BinaryOp("+", 1, 2), BinaryOp("+", 3, 4))
Output: "$( 1 + 2 ) \times ( 3 + 4 )$"
Both operands need parens
```

### Rust Implementation Checklist
- [ ] Add `Mult` variant to `TokenType` enum
- [ ] Update lexer to recognize '*' character
- [ ] Update parser TokenType match to include MULT
- [ ] Update operator mapping to include "*": "*"
- [ ] Add "*" to BINARY_OPS mapping with r#"\times"#
- [ ] Add "*": 2 to PRECEDENCE map
- [ ] Test precedence comparisons with addition
- [ ] Unit tests: "4 7 *" → "$4 \\times 7$"
- [ ] Integration test: "2 3 4 * +" → "$2 + 3 \\times 4$"
- [ ] Integration test: "5 3 + 2 *" → "$( 5 + 3 ) \\times 2$"
- [ ] Integration test: "2 3 4 * *" → "$2 \\times 3 \\times 4$"
- [ ] Integration test: "1 2 + 3 4 + *" → "$( 1 + 2 ) \\times ( 3 + 4 )$"
- [ ] Error test: "5 *" (missing operand)
- [ ] Error test: "5 3 2 *" (missing operator)

---

## Feature 5: Division

### Feature Overview
Parse and render the binary division operator (/) with same precedence as multiplication but requiring special handling for right-associativity in output parenthesization.

### Dependency Order
- **Position in Migration**: 5
- **Depends On**: Numbers (Feature 1)
- **Interacts With**: Addition, Subtraction, Multiplication (Features 2-4)
- **Enables**: Complete precedence system (Feature 6)

### Python Implementation Details

#### tokens.py - Token Definition
```python
class TokenType(Enum):
    DIV = auto()  # / (division)
```

#### lexer.py - Division Recognition
```python
if char == "/":
    self._advance()
    return Token(TokenType.DIV, "/", start_line, start_column)
```

**Algorithm**: Simple single-character token recognition.

#### ast_nodes.py
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str      # "/" for division
    left: Expr
    right: Expr
```

#### parser.py - Division Parsing
```python
elif token.type in (TokenType.PLUS, TokenType.MINUS, TokenType.MULT, TokenType.DIV):
    if len(stack) < 2:
        raise ParserError(...)

    right = stack.pop()
    left = stack.pop()

    op_map = {
        TokenType.DIV: "/",
        ...
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

**RPN Parsing for Division**:
- Identical to multiplication in parsing logic
- Operator string "/" maps from TokenType.DIV
- Precedence and parenthesization differ from multiplication

**Example: "10 2 /"**
```
[10] [2] [/] → pop 2, pop 10 → BinaryOp("/", 10, 2)
```

#### latex_gen.py - Division Output
```python
BINARY_OPS = {
    "/": r"\div",      # LaTeX division symbol
    ...
}

PRECEDENCE = {
    "/": 2,            # SAME precedence as multiplication
    ...
}
```

**LaTeX Output**:
- Operator "/" maps to `\div` (division symbol in LaTeX)
- Precedence 2 (same as multiplication, higher than addition)
- Right-side associativity rule applies (unlike multiplication)

**Special Parenthesization Rule for Division**:
```python
def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
    # Equal precedence on right side needs parens for non-commutative operators
    return (
        child_precedence == parent_precedence
        and is_right
        and child.operator in ("-", "/")
    )
```

**Why Division Needs Right-Side Parens**:
- Division is NOT associative: (10 / 2) / 5 ≠ 10 / (2 / 5)
- Left side: (10 / 2) / 5 = 5 / 5 = 1
- Right side: 10 / (2 / 5) = 10 / 0.4 = 25
- In RPN: "10 2 / 5 /" gives left-associative tree naturally
- When division appears on right side of division, needs parens
- Example: "10 2 5 / /" → "10 / ( 2 / 5 )" in output

**Example: "100 10 / 5 / 2 /"**
```
AST: BinaryOp("/", BinaryOp("/", BinaryOp("/", 100, 10), 5), 2)
       = ((100 / 10) / 5) / 2
       = (10 / 5) / 2
       = 2 / 2
       = 1

LaTeX generation:
  Visiting outermost BinaryOp("/", ..., 2):
    Left = result of visiting BinaryOp("/", BinaryOp("/", 100, 10), 5)
           = "( 100 \div 10 ) \div 5" (left side never parenthesized in same way)
           Actually, let me trace this more carefully...

  Innermost: BinaryOp("/", 100, 10)
    - Left: "100", Right: "10"
    - _needs_parens(100, precedence=2, is_right=False): False
    - _needs_parens(10, precedence=2, is_right=True): False (Number, not BinaryOp)
    - Result: "100 \div 10"

  Middle: BinaryOp("/", BinaryOp("/", 100, 10), 5)
    - Left: visit inner = "100 \div 10"
    - _needs_parens(BinaryOp("/", 100, 10), precedence=2, is_right=False): False
      (left side, so False)
    - Right: visit 5 = "5"
    - Result: "100 \div 10 \div 5"

  Outer: BinaryOp("/", BinaryOp("/", BinaryOp("/", 100, 10), 5), 2)
    - Left: visit middle = "100 \div 10 \div 5"
    - _needs_parens(..., precedence=2, is_right=False): False
    - Right: visit 2 = "2"
    - Result: "100 \div 10 \div 5 \div 2"

Output: "$100 \div 10 \div 5 \div 2$"
```

**Example: Hypothetical Right-Side Division**
```
If we had RPN: "10 2 5 / /"
AST: BinaryOp("/", 10, BinaryOp("/", 2, 5))
       = 10 / (2 / 5)
       = 10 / 0.4
       = 25

LaTeX generation:
  Inner: BinaryOp("/", 2, 5) → "2 \div 5"
  Outer: BinaryOp("/", 10, BinaryOp("/", 2, 5))
    - Left: visit 10 = "10"
    - Right: visit inner = "2 \div 5"
    - _needs_parens(BinaryOp("/", 2, 5), precedence=2, is_right=True):
      - child_precedence=2, parent_precedence=2, is_right=True
      - child.operator="/" is in ("-", "/") → True (NEEDS PARENS)
    - Right with parens: "( 2 \div 5 )"
    - Result: "10 \div ( 2 \div 5 )"

Output: "$10 \div ( 2 \div 5 )$"
```

### Rust Translation Strategy

#### Type Mappings
| Python | Rust |
|--------|------|
| `TokenType.DIV` | `TokenType::Div` |
| `r"\div"` | `r#"\div"#` (raw string) |

#### Pattern Changes

**LaTeX Escape Sequences**:
- Python: `r"\div"` is a raw string (backslash not interpreted)
- Rust: `r#"\div"#` or `"\\div"` (normal string with double backslash)
- Important: Rust string escape rules differ from Python

**Associativity and Parenthesization**:
- No changes to parsing logic
- Parenthesization determined by precedence and operator type
- "*" excluded from right-side rule, "/" included

#### Special Handling

**Non-Commutative and Non-Associative**:
- Unlike addition and multiplication, division order matters
- Parentheses critical for correct mathematical interpretation
- Parenthesization logic already handles this via operator check

**Precedence Ties with Multiplication**:
- Division and multiplication have same precedence level (2)
- This reflects mathematical convention
- Mixed operations: "10 / 2 * 5" and "10 * 2 / 5" are distinct

### Key Implementation Details

**Left-Associativity Naturally Enforced**:
- RPN parser creates left-associative tree by construction
- "100 10 / 5 /" creates BinaryOp("/", BinaryOp("/", 100, 10), 5)
- This evaluates as (100/10)/5, not 100/(10/5)
- Correct! This is left-associative behavior

**Right-Side Parenthesization**:
- Division on right side of division gets parenthesized in output
- This is correct because 10/(2/5) ≠ (10/2)/5
- Without parentheses, infix notation would be ambiguous

**Mixed with Multiplication**:
- Division and multiplication share precedence
- Left-associativity applies to both
- Example: "10 2 / 5 *" → "10 \div 2 \times 5" (no parens between)
- Example: "10 5 * 2 /" → "10 \times 5 \div 2" (no parens between)

### Test Cases for Division Feature

#### Basic Division
```
Input: "10 2 /"
Output: "$10 \div 2$"
Evaluation: 10 / 2 = 5
```

#### Chained Division
```
Input: "100 10 / 5 / 2 /"
AST: BinaryOp("/", BinaryOp("/", BinaryOp("/", 100, 10), 5), 2)
Output: "$100 \div 10 \div 5 \div 2$"
Evaluation: (((100/10)/5)/2) = ((10/5)/2) = (2/2) = 1
```

#### Division with Floats
```
Input: "1.5 0.5 /"
Output: "$1.5 \div 0.5$"
Evaluation: 1.5 / 0.5 = 3
```

#### Division with Multiplication
```
Input: "10 2 / 5 *"
AST: BinaryOp("*", BinaryOp("/", 10, 2), 5)
Output: "$10 \div 2 \times 5$"
Evaluation: (10/2)*5 = 5*5 = 25
```

#### Division with Addition
```
Input: "10 2 / 3 + 4 *"
AST: BinaryOp("*", BinaryOp("+", BinaryOp("/", 10, 2), 3), 4)
Output: "$( 10 \div 2 + 3 ) \times 4$"
Evaluation: ((10/2)+3)*4 = (5+3)*4 = 8*4 = 32
```

### Rust Implementation Checklist
- [ ] Add `Div` variant to `TokenType` enum
- [ ] Update lexer to recognize '/' character
- [ ] Update parser TokenType match to include DIV
- [ ] Update operator mapping to include "/": "/"
- [ ] Add "/" to BINARY_OPS mapping with r#"\div"#
- [ ] Add "/": 2 to PRECEDENCE map (same as multiplication)
- [ ] Verify "/" is in right-side parenthesization exclusion list
- [ ] Unit tests: "10 2 /" → "$10 \\div 2$"
- [ ] Integration test: "100 10 / 5 / 2 /" → "$100 \\div 10 \\div 5 \\div 2$"
- [ ] Integration test: "10 2 / 5 *" → "$10 \\div 2 \\times 5$"
- [ ] Integration test: "10 2 / 3 + 4 *" → "$( 10 \\div 2 + 3 ) \\times 4$"
- [ ] Error test: "10 /" (missing operand)
- [ ] Error test: "10 2 5 /" (extra operand)

---

## Feature 6: Precedence and Parenthesization

### Feature Overview
Correct handling of operator precedence and automatic parenthesization in LaTeX output. This is primarily a concern of the LaTeX generator, not the parser. The parser (via RPN evaluation) already respects precedence implicitly.

### Dependency Order
- **Position in Migration**: 6 (last)
- **Depends On**: All operators (Features 1-5)
- **Refines**: LaTeX output quality
- **Complete Feature Set**: All 6 core features

### Python Implementation Details

#### latex_gen.py - Precedence System

```python
class LaTeXGenerator:
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
```

**Precedence Levels**:
- Level 1 (lowest): "+" and "-" (addition, subtraction)
- Level 2 (higher): "*" and "/" (multiplication, division)
- Higher number = tighter binding = evaluated first

#### Parenthesization Logic

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
    return (
        child_precedence == parent_precedence
        and is_right
        and child.operator in ("-", "/")
    )
```

**Parenthesization Rules**:

1. **Lower Precedence Always Gets Parens**:
   - Child precedence < parent precedence → needs parens
   - Example: Addition under multiplication: "(5+3)*2"
   - This preserves correct evaluation order

2. **Same Precedence Left Side**:
   - Child precedence == parent precedence and is_right=False → no parens
   - Example: "5 + 3 + 2" (left associativity honored)
   - Example: "5 - 3 - 2" (left associativity honored)

3. **Same Precedence Right Side - Non-Commutative**:
   - Child precedence == parent precedence and is_right=True and operator in ("-", "/")
   - → needs parens
   - Example: "5 - (3 - 2)" requires parens
   - Example: "10 / (2 / 5)" requires parens
   - Addition and multiplication don't need this (commutative enough)

4. **Numbers Never Need Parens**:
   - Numbers are not BinaryOp → _needs_parens returns False
   - Simple operands never parenthesized

### Rust Translation Strategy

#### Type Mappings
| Python | Rust |
|--------|------|
| `ClassVar[dict[str, str]]` | `const BINARY_OPS: &[(&str, &str)]` or `HashMap` |
| `ClassVar[dict[str, int]]` | `const PRECEDENCE: &[(&str, int)]` or `HashMap` |
| `isinstance(child, BinaryOp)` | Pattern match on enum or type check |
| `child.operator in ("-", "/")` | `matches!(op, "-" or "/")` |

#### Pattern Changes

**Static Data**:
- Python: Class variables with dictionaries
- Rust: Const items or lazy_static!
- Performance: Const is preferred (compile-time, zero runtime cost)
- Consider: `const BINARY_OPS: &[(&str, &str)]` for simple lookup

**Enum vs. String Operators**:
- Python stores operator as string "+" for flexibility
- Rust could: Keep strings (match Python), or use enum (more type-safe)
- For Phase 1, keep strings to match Python exactly

**Pattern Matching**:
- Python: `isinstance()` check followed by dot access
- Rust: Use enum pattern matching or type-based dispatch
- Consider: `match child { Expr::BinaryOp(op) => ... _ => false }`

**String Membership Test**:
- Python: `child.operator in ("-", "/")`
- Rust: `matches!(child.operator, "-" | "/")`
- Or: Direct string comparison with `||` operator

#### Special Handling

**Immutable Data Structures**:
- Python: Dictionaries are mutable but used as constants
- Rust: Use `const` for compile-time constants
- Alternative: `lazy_static!` for runtime-initialized statics

**Visitor Pattern**:
- Python: `@singledispatchmethod` decorator
- Rust: Trait-based dispatch or `match` on enum
- For Phase 1, direct enum matching might be simpler

**String Lookups**:
- Python: `dict[key]` automatically looks up
- Rust: Need to search (Vec) or use HashMap/BTreeMap
- For small fixed set (4 operators), Vec with linear search is fine

### Key Implementation Details

**Precedence Determines Parenthesization**:
- The parser has already structured the AST correctly
- RPN evaluation naturally respects precedence through stack operations
- LaTeX generation just needs to add parens for clarity
- Without parentheses, mathematical meaning could be ambiguous

**Why RPN Naturally Respects Precedence**:
- In RPN: "2 3 + 4 *" is written with operations in post-order
- Stack evaluation forces left-to-right: compute (2+3) first, then multiply by 4
- This is inherently how precedence works
- AST structure reflects this ordering

**Example: "2 3 + 4 *"**
1. Lexer: NUMBER(2), NUMBER(3), PLUS(+), NUMBER(4), MULT(*), EOF
2. Parser builds AST:
   - Push 2 → stack = [2]
   - Push 3 → stack = [2, 3]
   - Pop 3, 2, create BinaryOp("+", 2, 3) → stack = [2+3]
   - Push 4 → stack = [2+3, 4]
   - Pop 4, 2+3, create BinaryOp("*", 2+3, 4) → stack = [(*)]
3. AST: BinaryOp("*", BinaryOp("+", 2, 3), 4)
4. LaTeX generation:
   - Visit BinaryOp("*", ...)
   - Left child is BinaryOp("+", ...) with precedence 1
   - Parent has precedence 2
   - 1 < 2, so left needs parens
   - Result: "( 2 + 3 ) \times 4"

**Parenthesization is Strictly Cosmetic**:
- Doesn't change semantics
- Makes output readable and mathematically unambiguous
- Preserves evaluation order that was already implicit in AST

### Test Cases for Precedence Feature

All test cases from previous features serve as precedence tests. Here are the most critical:

#### Precedence: Mult Over Add (Left)
```
Input: "5 3 * 2 +"
AST: BinaryOp("+", BinaryOp("*", 5, 3), 2)
Output: "$5 \times 3 + 2$"
Note: No parens. Multiplication has higher precedence.
```

#### Precedence: Mult Over Add (Right)
```
Input: "5 3 + 2 *"
AST: BinaryOp("*", BinaryOp("+", 5, 3), 2)
Output: "$( 5 + 3 ) \times 2$"
Note: Parens on left (addition under multiplication).
```

#### Precedence: Mult Over Add (Both)
```
Input: "1 2 + 3 4 + *"
AST: BinaryOp("*", BinaryOp("+", 1, 2), BinaryOp("+", 3, 4))
Output: "$( 1 + 2 ) \times ( 3 + 4 )$"
Note: Parens on both operands.
```

#### Precedence: Div Over Add
```
Input: "10 2 / 3 + 4 *"
AST: BinaryOp("*", BinaryOp("+", BinaryOp("/", 10, 2), 3), 4)
Output: "$( 10 \div 2 + 3 ) \times 4$"
Note: Complex nesting with correct parenthesization at each level.
```

#### Same Precedence: Left-Associativity (Add)
```
Input: "1 2 + 3 + 4 +"
Output: "$1 + 2 + 3 + 4$"
Note: No internal parens. Left-associativity implicit.
```

#### Same Precedence: Right-Side Rule (Sub)
```
Input: "5 3 2 - -"
AST: BinaryOp("-", 5, BinaryOp("-", 3, 2))
Output: "$5 - ( 3 - 2 )$"
Note: Parens on right operand (subtraction of subtraction).
```

#### Same Precedence: Right-Side Rule (Div)
```
Input: "10 2 5 / /" (hypothetical - RPN would normally be different)
AST: BinaryOp("/", 10, BinaryOp("/", 2, 5))
Output: "$10 \div ( 2 \div 5 )$"
Note: Parens on right operand (division of division).
```

#### Mixed: Mult and Div (Same Level)
```
Input: "10 2 / 5 *"
Output: "$10 \div 2 \times 5$"
Note: No parens between operators of same level.
```

### Rust Implementation Checklist
- [ ] Define const/static for BINARY_OPS mapping
- [ ] Define const/static for PRECEDENCE mapping
- [ ] Implement operator lookup (search Vec or use HashMap)
- [ ] Implement `_needs_parens()` method
- [ ] Handle BinaryOp vs. Number type checking
- [ ] Implement precedence comparison logic
- [ ] Implement right-side special case for ("-", "/")
- [ ] Test: "5 3 * 2 +" → "$5 \\times 3 + 2$"
- [ ] Test: "5 3 + 2 *" → "$( 5 + 3 ) \\times 2$"
- [ ] Test: "1 2 + 3 4 + *" → "$( 1 + 2 ) \\times ( 3 + 4 )$"
- [ ] Test: "10 2 / 3 + 4 *" → "$( 10 \\div 2 + 3 ) \\times 4$"
- [ ] Test: "1 2 + 3 + 4 +" → "$1 + 2 + 3 + 4$"
- [ ] Test: "5 3 2 - -" → "$5 - ( 3 - 2 )$"
- [ ] Test: "10 2 / 5 *" → "$10 \\div 2 \\times 5$"
- [ ] Integration: All precedence test cases pass

---

# MIGRATION ARCHITECTURE NOTES

## Module Dependency Graph

```
cli.py
├── lexer.py
│   └── tokens.py
├── parser.py
│   ├── tokens.py
│   └── ast_nodes.py
└── latex_gen.py
    └── ast_nodes.py

errors.py (used by cli.py)
```

## Feature Implementation Order

1. **Numbers** - Parse and render numeric literals
   - Core building blocks, all other features depend on this
   - Implement: tokens, lexer, parser, ast_nodes, latex_gen

2. **Addition** - Binary +operator
   - Introduces BinaryOp and operator handling
   - Extends: tokens, parser, latex_gen

3. **Subtraction** - Binary - operator
   - Introduces left-associativity and operator precedence
   - Extends: tokens, parser, latex_gen

4. **Multiplication** - Binary * operator
   - Introduces precedence differences
   - Extends: tokens, parser, latex_gen (with precedence level 2)

5. **Division** - Binary / operator
   - Introduces non-associativity in output (right-side parens)
   - Extends: tokens, parser, latex_gen

6. **Precedence** - Complete parenthesization system
   - Refinement of LaTeX generator
   - All operators working together

## Rust Project Structure

```
src/
├── main.rs         # CLI entry point
├── lib.rs          # Library exports
├── tokens.rs       # Feature 1: Token types
├── lexer.rs        # Feature 1-5: Lexical analysis
├── ast_nodes.rs    # Feature 1-5: AST definitions
├── parser.rs       # Feature 1-5: RPN parsing
├── latex_gen.rs    # Feature 1-6: LaTeX generation
└── errors.rs       # Error formatting (cross-cutting)

tests/
├── feature_1_numbers.rs
├── feature_2_addition.rs
├── feature_3_subtraction.rs
├── feature_4_multiplication.rs
├── feature_5_division.rs
├── feature_6_precedence.rs
└── integration.rs    # Full pipeline tests

examples/
└── basic_example.rs  # Demo of full pipeline
```

## Type System Considerations

### Python to Rust Type Mapping (Summary)

| Concept | Python | Rust |
|---------|--------|------|
| Token Stream | `list[Token]` | `Vec<Token>` |
| Stack | `list[Expr]` | `Vec<Expr>` |
| AST | `Number \| BinaryOp` | `enum Expr { Number(...), BinaryOp(...) }` |
| Immutable Data | `@dataclass(frozen=True)` | `struct T { ... }` |
| Operator Map | `dict[str, str]` | `const [...(&str, &str)]` |
| String Values | `str` | `String` or `&'static str` |

### Key Differences

1. **Recursive Types**: Need Box indirection in Rust
2. **String Handling**: String vs &str ownership semantics
3. **Immutability**: Default in Rust, explicit in Python
4. **Error Handling**: Exceptions vs Result types
5. **Pattern Matching**: Better in Rust, required for enums

## Testing Strategy

### Unit Tests (per feature)
- Lexer: Token generation for each token type
- Parser: AST construction for basic expressions
- LaTeX: Output generation for simple nodes

### Integration Tests (per feature)
- End-to-end: input → tokens → AST → LaTeX
- Verify all test cases from Phase 0 I/O contract
- Check error messages and formatting

### Full Regression Tests
- All 33 passing test cases from Python implementation
- Error cases with proper formatting
- Edge cases and corner cases

---

# IMPLEMENTATION NOTES FOR RUST MIGRATORS

## Error Handling Strategy

### Python Approach
```python
class LexerError(Exception):
    def __init__(self, message: str, line: int, column: int):
        self.message = message
        self.line = line
        self.column = column

class ParserError(Exception):
    def __init__(self, message: str, token: Token):
        self.message = message
        self.token = token
```

### Recommended Rust Approach
```rust
#[derive(Debug)]
struct LexerError {
    message: String,
    line: usize,
    column: usize,
}

#[derive(Debug)]
struct ParserError {
    message: String,
    token: Token,
}

// Use Result<T, LexerError> or anyhow::Result<T>
```

## String Handling

### Position Tracking
- Python: 1-based line/column (user-friendly)
- Rust: Be consistent with Python (also 1-based)
- Avoid confusion: 0-based is typical in many languages but Python uses 1-based here

### LaTeX Special Characters
- Python: `r"\times"` and `r"\div"` (raw strings)
- Rust: `r#"\times"#` or `"\\times"` (watch escaping)
- Test: Verify backslashes correctly rendered

## Performance Considerations

### Stack Allocations
- Python uses GC, no worry about allocations
- Rust: Vec operations are efficient, no major concerns for this codebase size
- Avoid: Unnecessary cloning of large structures

### String Operations
- Python: Strings are immutable, lots of implicit copying
- Rust: String vs &str trade-offs
- For tokens: Owned String is fine (small in number)
- For AST: Could use &str with lifetime parameters (but not necessary for simplicity)

## Regex Comparison

### Lexer Implementation
- Python: Character-by-character scanning (no regex)
- Rust: Same approach recommended (nom library not necessary for simple RPN)
- Keep it simple: while loops and char checks

### Number Scanning
- Handle integer part: while char.isdigit()
- Handle decimal: if char == '.' then scan more digits
- Negative: lookahead after '-'

---

# CONCLUSION

This specification provides a complete roadmap for migrating the rpn2tex Python implementation to Rust. The features are organized by dependency order, with each feature including:

1. Complete Python implementation details (code snippets)
2. Rust translation strategy with specific recommendations
3. Key implementation details and algorithms
4. Test cases from the Phase 0 I/O contract
5. Rust implementation checklist

All 33 passing test cases from the Python implementation must pass in the Rust version. The 3 failing test cases (exponentiation) are intentionally excluded from Phase 1.

The modular feature-based approach allows for incremental implementation and testing, with each feature depending only on its predecessors.
