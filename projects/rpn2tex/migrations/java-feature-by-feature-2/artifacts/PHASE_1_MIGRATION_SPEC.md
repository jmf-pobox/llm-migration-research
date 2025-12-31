# PHASE 1 MIGRATION SPECIFICATION: Feature-by-Feature Analysis

**Project**: rpn2tex (RPN to LaTeX Converter)
**Target Language**: Java
**Migration Approach**: Feature-based decomposition
**Date**: 2025-12-30
**Status**: Specification Phase

---

## Executive Summary

This specification documents the complete decomposition of the rpn2tex Python codebase into independent features, ordered by dependency. Each feature is analyzed across all five components (tokens, AST, lexer, parser, LaTeX generator) to enable parallel, feature-based migration to Java.

The implementation consists of:
- **7 Python source modules** (tokens, ast_nodes, errors, lexer, parser, latex_gen, cli)
- **6 core features** organized by dependency
- **21 test cases** from the I/O contract (18 passing, 3 failing due to unimplemented exponentiation)

---

## I/O Contract

This contract specifies the exact input/output behavior that must be preserved during migration.

### Test Execution Summary

| Metric | Value |
|--------|-------|
| Date | 2025-12-30 |
| Implementation | Python |
| Test Command | `echo "<input>" | python -m source.cli -` |
| Total Tests | 21 |
| Passed | 18 |
| Failed | 3 (exponentiation operator not supported) |

### Test Cases by Feature

#### Numbers

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `5` | `$5$` | `$5$` | PASS |
| `3.14` | `$3.14$` | `$3.14$` | PASS |

#### Addition

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `5 3 +` | `$5 + 3$` | `$5 + 3$` | PASS |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | `$1 + 2 + 3 + 4$` | PASS |

#### Subtraction

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `5 3 -` | `$5 - 3$` | `$5 - 3$` | PASS |
| `5 3 - 2 -` | `$5 - 3 - 2$` | `$5 - 3 - 2$` | PASS |

#### Multiplication

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `4 7 *` | `$4 \times 7$` | `$4 \times 7$` | PASS |
| `2 3 4 * +` | `$2 + 3 \times 4$` | `$2 + 3 \times 4$` | PASS |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | `$5 \times 3 + 2$` | PASS |

#### Division

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `10 2 /` | `$10 \div 2$` | `$10 \div 2$` | PASS |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | `$100 \div 10 \div 5 \div 2$` | PASS |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | `$10 \div 2 \times 5$` | PASS |

#### Operator Precedence

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | `$( 5 + 3 ) \times 2$` | PASS |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | `$( 2 + 3 ) \times 4$` | PASS |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | `$2 \times ( 3 + 4 )$` | PASS |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | `$2 \times 3 + 4$` | PASS |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | `$( 10 \div 2 + 3 ) \times 4$` | PASS |

#### Floating Point Numbers

| Input | Expected Output | Actual Output | Status |
|-------|-----------------|---------------|--------|
| `3.14 2 *` | `$3.14 \times 2$` | `$3.14 \times 2$` | PASS |
| `1.5 0.5 +` | `$1.5 + 0.5$` | `$1.5 + 0.5$` | PASS |

#### Exponentiation (Not Supported in Phase 0)

| Input | Expected Output | Actual Output | Status | Error |
|-------|-----------------|---------------|--------|-------|
| `2 3 ^` | `$2 ^ {3}$` | (empty) | FAIL | Unexpected character '^' |
| `2 3 ^ 4 *` | `$( 2 ^ {3} ) \times 4$` | (empty) | FAIL | Unexpected character '^' |
| `2 3 4 ^ ^` | `$2 ^ {3 ^ {4}}$` | (empty) | FAIL | Unexpected character '^' |

---

## Feature Dependency Graph

```
FEATURE HIERARCHY
├── Feature 1: numbers (no dependencies)
│   ├── Token: NUMBER
│   ├── AST Node: Number
│   └── Components: lexer, parser, latex_gen
│
├── Feature 2: addition (depends on: numbers)
│   ├── Token: PLUS
│   ├── AST Node: BinaryOp (operator="+")
│   └── Components: tokens, lexer, parser, latex_gen
│
├── Feature 3: subtraction (depends on: numbers)
│   ├── Token: MINUS
│   ├── AST Node: BinaryOp (operator="-")
│   └── Components: tokens, lexer, parser, latex_gen
│
├── Feature 4: multiplication (depends on: numbers)
│   ├── Token: MULT
│   ├── AST Node: BinaryOp (operator="*")
│   └── Components: tokens, lexer, parser, latex_gen
│
├── Feature 5: division (depends on: numbers)
│   ├── Token: DIV
│   ├── AST Node: BinaryOp (operator="/")
│   └── Components: tokens, lexer, parser, latex_gen
│
└── Feature 6: precedence (depends on: addition, subtraction, multiplication, division)
    ├── No new tokens or AST nodes
    ├── Core algorithm: _needs_parens() with precedence map
    └── Components: latex_gen only
```

---

# Feature 1: Numbers

## Overview

Numeric literal parsing and output. Foundation feature that all operators depend on.

### Token Definitions

**File**: `tokens.py`

```python
class TokenType(Enum):
    NUMBER = auto()  # Numeric values: 5, 3.14, -2
```

**Token Structure**:
```python
@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str          # String representation of the number
    line: int          # 1-based line number
    column: int        # 1-based column number
```

### AST Node Definitions

**File**: `ast_nodes.py`

```python
@dataclass(frozen=True)
class Number(ASTNode):
    """Numeric literal node"""
    value: str         # String representation (e.g., "42", "3.14", "-2")
    # inherited: line, column
```

### Lexer Logic

**File**: `lexer.py` (lines 136-200)

**Method**: `_scan_number(prefix: str, start_line: int, start_column: int) -> Token`

**Algorithm**:
1. Accept optional prefix (empty string or "-" for negatives)
2. Consume one or more digits for integer part
3. If next character is ".", consume it and one or more digits for decimal part
4. Return NUMBER token with accumulated value

**Key Features**:
- Handles both integers (5) and decimals (3.14)
- Handles negative numbers (prefix "-" when digit immediately follows "-" operator)
- Position tracking (line, column)

**Special Cases**:
- Distinction between minus operator and negative number prefix:
  - Lines 154-162: If "-" is followed by digit, treat as negative number
  - Otherwise, treat "-" as subtraction operator

### Parser Logic

**File**: `parser.py` (lines 88-168)

**Method**: `parse() -> Expr` (stack-based RPN parser)

**Algorithm**:
1. Maintain expression stack
2. When NUMBER token encountered:
   - Create Number AST node with value, line, column
   - Push onto stack
   - Advance to next token
3. Continue processing operators

**Key Implementation**:
```python
if token.type == TokenType.NUMBER:
    num_node = Number(
        line=token.line,
        column=token.column,
        value=token.value
    )
    stack.append(num_node)
    self._advance()
```

### Generator Logic

**File**: `latex_gen.py` (lines 100-109)

**Method**: `_visit_number(node: Number) -> str`

**Algorithm**:
1. Return the number value as-is (no transformation)

**Implementation**:
```python
@_visit.register
def _visit_number(self, node: Number) -> str:
    return node.value
```

### Key Algorithms

**Numeric Lexing** (in Lexer._scan_number):
- Character-by-character accumulation
- Two-phase parsing: integer part, then optional decimal part
- Position tracking with line/column updates on newlines

### Edge Cases

1. **Decimal numbers**: 3.14, 0.5
2. **Negative numbers**: Lexer must distinguish "-" operator from negative prefix
3. **Floating-point precision**: Value stored as string, not parsed to float (preserves original representation)
4. **Large numbers**: No upper limit (string-based)
5. **Leading zeros**: Preserved as written (e.g., "007" remains "007")

### Java Mapping

| Python | Java |
|--------|------|
| `str` value field | `String` value field |
| `@dataclass(frozen=True)` | Immutable record class or `final class` with immutable fields |
| `TokenType.NUMBER` enum | `TokenType.NUMBER` enum |
| Position tracking (line, column) | Same approach with int fields |
| Method `_scan_number()` | Same algorithm, return `Token` object |
| Visitor pattern via `@singledispatchmethod` | Method overloading or visitor pattern implementation |
| String concatenation | `StringBuilder` or string concatenation (Java will optimize) |

### Implementation Checklist

- [ ] TokenType.NUMBER enum value
- [ ] Token class with type, value, line, column fields
- [ ] ASTNode base class with line, column
- [ ] Number class extending ASTNode with value field
- [ ] Lexer._scan_number() method
- [ ] Parser handling NUMBER tokens
- [ ] LaTeXGenerator._visit_number() method
- [ ] Integration test: "5" -> "$5$"
- [ ] Integration test: "3.14" -> "$3.14$"

---

# Feature 2: Addition

## Overview

Addition operator (+) combining two numeric operands.

### Token Definitions

**File**: `tokens.py` (lines 26-45)

```python
class TokenType(Enum):
    PLUS = auto()  # + (addition)
```

### AST Node Definitions

**File**: `ast_nodes.py` (lines 58-82, 90)

```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str      # "+" for addition
    left: Expr        # Left operand
    right: Expr       # Right operand
    # inherited: line, column

Expr = Number | BinaryOp  # Type alias
```

### Lexer Logic

**File**: `lexer.py` (lines 136-175)

**Method**: `_scan_token() -> Token`

**Algorithm**:
1. When current character is "+", advance and return PLUS token

**Implementation** (lines 150-152):
```python
if char == "+":
    self._advance()
    return Token(TokenType.PLUS, "+", start_line, start_column)
```

### Parser Logic

**File**: `parser.py` (lines 88-168)

**Method**: `parse() -> Expr` (RPN stack-based)

**Algorithm**:
1. When PLUS token encountered:
   - Check stack has at least 2 operands (raise ParserError if not)
   - Pop right operand, then left operand (RPN order)
   - Create BinaryOp node with operator="+"
   - Push result back onto stack

**Implementation** (lines 115-147):
```python
elif token.type in (
    TokenType.PLUS,
    TokenType.MINUS,
    TokenType.MULT,
    TokenType.DIV,
):
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

### Generator Logic

**File**: `latex_gen.py` (lines 47-62, 111-141)

**Class variables**:
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "+": "+",
    # ... other operators
}

PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,  # Lowest precedence
    # ... other operators
}
```

**Method**: `_visit_binary_op(node: BinaryOp) -> str`

**Algorithm**:
1. Get LaTeX representation of operator ("+")
2. Get precedence level (1 for addition)
3. Visit left operand, add parentheses if needed
4. Visit right operand, add parentheses if needed
5. Return formatted string

**Implementation** (lines 111-141):
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

### Key Algorithms

**RPN Operator Processing**:
- Stack-based: when operator token encountered, pop operands and build tree
- Order critical: `left = stack.pop()` then `right = stack.pop()` (after first pop, top of stack is left operand)

### Edge Cases

1. **Insufficient operands**: Error handling for "3 +"
2. **Left-associativity**: "1 2 + 3 +" produces `( 1 + 2 ) + 3` not `1 + ( 2 + 3 )`
3. **Parenthesization**: No parens needed for addition operands since it's lowest precedence

### Java Mapping

| Python | Java |
|--------|------|
| `operator: str` | `String operator` field |
| `BinaryOp` with left, right | Java object with Expr left, right fields |
| Token type mapping | Enum value lookup |
| Stack operations | `Stack<Expr>` or `List<Expr>` with pop/push |
| Error handling | Custom exception class ParserError |
| LaTeX operator string | Direct string constant |

### Implementation Checklist

- [ ] TokenType.PLUS enum value
- [ ] Lexer._scan_token() recognizes "+" character
- [ ] Parser._needs_parens() considers addition precedence
- [ ] LaTeXGenerator.BINARY_OPS maps "+" to "+"
- [ ] LaTeXGenerator.PRECEDENCE maps "+" to 1
- [ ] BinaryOp creation with operator="+"
- [ ] Integration test: "5 3 +" -> "$5 + 3$"
- [ ] Integration test: "1 2 + 3 + 4 +" -> "$1 + 2 + 3 + 4$"

---

# Feature 3: Subtraction

## Overview

Subtraction operator (-) with special handling for negative number detection.

### Token Definitions

**File**: `tokens.py` (lines 26-45)

```python
class TokenType(Enum):
    MINUS = auto()  # - (subtraction)
```

### AST Node Definitions

**File**: `ast_nodes.py` (lines 58-82, 90)

```python
# Same BinaryOp as addition
# operator field will be "-"
Expr = Number | BinaryOp
```

### Lexer Logic

**File**: `lexer.py` (lines 136-175)

**Method**: `_scan_token() -> Token` (lines 153-162)

**Algorithm**:
1. When current character is "-", advance
2. Check if next character is a digit
   - If YES: this is a negative number, call _scan_number("-", ...)
   - If NO: this is subtraction operator, return MINUS token

**Implementation**:
```python
if char == "-":
    self._advance()
    # Check if this is a negative number (digit follows immediately)
    if not self._at_end() and self._peek().isdigit():
        # It's a negative number
        return self._scan_number("-", start_line, start_column)
    return Token(TokenType.MINUS, "-", start_line, start_column)
```

**Critical Design Decision**:
RPN doesn't use negative literals in the standard way. Negative numbers are represented as "0 5 -" (compute 0 - 5). However, this implementation allows "-5" as a negative literal for convenience.

### Parser Logic

**File**: `parser.py` (lines 115-147)

**Algorithm**:
Identical to addition processing:
1. Check stack has >= 2 operands
2. Pop right, then left
3. Create BinaryOp with operator="-"
4. Push result

### Generator Logic

**File**: `latex_gen.py` (lines 47-62, 111-141)

**LaTeX representation**: "-" (no special escaping needed)

**Precedence**: 1 (same as addition, left-associative)

**Algorithm**:
Identical to addition, but with special handling for right-associativity:

```python
def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
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

**Key Rule** (lines 174-180):
If child has same precedence as parent AND is on right side AND operator is "-" (or "/"), add parens to preserve left-associativity.

### Key Algorithms

**Negative Number Detection** (Lexer):
- Lookahead after "-" character
- If next is digit, treat as negative number prefix
- Otherwise, treat as operator

**Left-Associativity Preservation** (LaTeX Generator):
- "5 - 3 - 2" should output "5 - 3 - 2" (not "5 - ( 3 - 2 )" which would be wrong)
- Implementation: when right child has same precedence and is "-", add parens to left operand

### Edge Cases

1. **Negative numbers**: "-5 3 +" must parse as Number(-5) + Number(3)
2. **Right-associativity**: "5 3 - 2 -" should be "5 - 3 - 2" not "5 - ( 3 - 2 )"
3. **Subtract from subtraction**: "10 5 - 2 -" -> "10 - 5 - 2" (left-associative)

### Java Mapping

| Python | Java |
|--------|------|
| Character lookahead | `charAt(pos + 1)` after advancing |
| Negative detection | Same conditional logic |
| Operator mapping | Token type enum |
| Right-side parenthesization | Conditional in _needs_parens() method |

### Implementation Checklist

- [ ] TokenType.MINUS enum value
- [ ] Lexer negative number detection (lookahead after "-")
- [ ] Parser handles MINUS tokens same as PLUS
- [ ] LaTeXGenerator.PRECEDENCE maps "-" to 1
- [ ] LaTeXGenerator._needs_parens() handles right-side subtraction
- [ ] Integration test: "5 3 -" -> "$5 - 3$"
- [ ] Integration test: "5 3 - 2 -" -> "$5 - 3 - 2$" (no extra parens)
- [ ] Edge case test: "-5 3 +" -> "$-5 + 3$"

---

# Feature 4: Multiplication

## Overview

Multiplication operator (*) with higher precedence than addition/subtraction.

### Token Definitions

**File**: `tokens.py` (lines 26-45)

```python
class TokenType(Enum):
    MULT = auto()  # * (multiplication)
```

### AST Node Definitions

**File**: `ast_nodes.py` (lines 58-82, 90)

```python
# Same BinaryOp as other operators
# operator field will be "*"
Expr = Number | BinaryOp
```

### Lexer Logic

**File**: `lexer.py` (lines 163-165)

**Method**: `_scan_token() -> Token`

**Algorithm**:
1. When current character is "*", advance and return MULT token

**Implementation**:
```python
if char == "*":
    self._advance()
    return Token(TokenType.MULT, "*", start_line, start_column)
```

### Parser Logic

**File**: `parser.py` (lines 115-147)

**Algorithm**:
Identical to addition and subtraction:
1. Check stack >= 2 operands
2. Pop right, then left
3. Create BinaryOp with operator="*"
4. Push result

### Generator Logic

**File**: `latex_gen.py` (lines 47-62, 111-141)

**LaTeX representation**: `\times` (requires backslash escaping)

**Precedence**: 2 (higher than addition/subtraction, binds tighter)

**Algorithm**:
1. Get LaTeX operator: `\times`
2. Visit left operand
3. Add parentheses to left if its precedence < 2
4. Visit right operand
5. Add parentheses to right if:
   - Its precedence < 2, OR
   - Its precedence == 2 AND is_right=True AND operator in ("*", "/")
6. Format output

**Key Examples**:
- "5 3 +" (precedence 1) as left child of "*" needs parens: "( 5 + 3 ) * 2"
- "5 3 *" (precedence 2) as left child of "*" doesn't need parens: "5 * 3 * 2"
- "5 3 /" (precedence 2) as right child of "*" doesn't need parens: "5 * 10 / 2" (still left-to-right)

### Key Algorithms

**Precedence Comparison**:
- Multiplication precedence (2) > Addition precedence (1)
- When visiting operands, check if child precedence < parent precedence
- For equal precedence with multiplication, right-associativity not special (unlike subtraction)

### Edge Cases

1. **Mixed operators**: "2 3 4 * +" -> "2 + 3 * 4" (multiplication has higher precedence)
2. **Parenthesization**: "5 3 + 2 *" -> "( 5 + 3 ) * 2" (addition needs parens)
3. **Right associativity of division**: "10 2 / 5 /" -> "10 / 2 / 5" (left-to-right, no extra parens)
4. **Commutative but higher precedence**: "2 3 4 * +" vs "2 3 + 4 *" produce different output

### Java Mapping

| Python | Java |
|--------|------|
| `\times` string literal | Raw string or escape sequence for backslash |
| Precedence integer comparison | Same int constants and comparison |
| Token.MULT enum | Enum value |
| LaTeX string concatenation | String builder or concatenation |

### Implementation Checklist

- [ ] TokenType.MULT enum value
- [ ] Lexer._scan_token() recognizes "*" character
- [ ] Parser handles MULT tokens
- [ ] LaTeXGenerator.BINARY_OPS maps "*" to `\times`
- [ ] LaTeXGenerator.PRECEDENCE maps "*" to 2
- [ ] LaTeXGenerator._needs_parens() handles precedence 2
- [ ] Integration test: "4 7 *" -> "$4 \times 7$"
- [ ] Integration test: "2 3 4 * +" -> "$2 + 3 \times 4$"
- [ ] Integration test: "5 3 * 2 +" -> "$5 \times 3 + 2$"
- [ ] Integration test: "5 3 + 2 *" -> "$( 5 + 3 ) \times 2$"

---

# Feature 5: Division

## Overview

Division operator (/) with higher precedence than addition/subtraction, and special left-associativity handling.

### Token Definitions

**File**: `tokens.py` (lines 26-45)

```python
class TokenType(Enum):
    DIV = auto()  # / (division)
```

### AST Node Definitions

**File**: `ast_nodes.py` (lines 58-82, 90)

```python
# Same BinaryOp as other operators
# operator field will be "/"
Expr = Number | BinaryOp
```

### Lexer Logic

**File**: `lexer.py` (lines 166-168)

**Method**: `_scan_token() -> Token`

**Algorithm**:
1. When current character is "/", advance and return DIV token

**Implementation**:
```python
if char == "/":
    self._advance()
    return Token(TokenType.DIV, "/", start_line, start_column)
```

### Parser Logic

**File**: `parser.py` (lines 115-147)

**Algorithm**:
Identical to other binary operators:
1. Check stack >= 2 operands
2. Pop right, then left (RPN order critical for division)
3. Create BinaryOp with operator="/"
4. Push result

**Critical Design Note**:
In RPN, "10 2 /" means "push 10, push 2, divide" which is "10 / 2 = 5". The order matters because division is non-commutative. The parser correctly implements this by popping right-operand first, then left-operand.

### Generator Logic

**File**: `latex_gen.py` (lines 47-62, 111-141)

**LaTeX representation**: `\div` (division symbol)

**Precedence**: 2 (same as multiplication)

**Algorithm**:
Same as multiplication, but with special left-associativity handling for right operand:

```python
# Equal precedence on right side needs parens for non-commutative operators
# (handles left-associativity of - and /)
return (
    child_precedence == parent_precedence
    and is_right
    and child.operator in ("-", "/")
)
```

**Key Rule**:
When division appears on the right side of another division with equal precedence, add parentheses to preserve left-associativity.

**Example**:
- "100 10 / 5 /" should output "100 / 10 / 5" (not "100 / ( 10 / 5 )")
- First division creates BinaryOp("/", 100, 10)
- Second division creates BinaryOp("/", BinaryOp("/", 100, 10), 5)
- When generating right operand (5) of outer "/", no parens needed since 5 is Number
- LaTeX generator correctly handles this

### Key Algorithms

**Non-Commutative Operator Handling**:
- Unlike addition (commutative), order matters: a / b ≠ b / a
- RPN parser correctly respects this by maintaining operand order
- LaTeX generator must preserve left-to-right evaluation through parenthesization

**Left-Associativity of Division**:
- "10 / 5 / 2" = (10 / 5) / 2 = 2 / 2 = 1
- NOT "10 / (5 / 2)" = 10 / 2.5 = 4
- Generator adds parens when division appears on right of division to force correct grouping

### Edge Cases

1. **Multiple divisions**: "100 10 / 5 / 2 /" -> "100 / 10 / 5 / 2" (all left-associative)
2. **Division with multiplication**: "10 2 / 5 *" -> "10 / 2 * 5" (left-to-right, both precedence 2)
3. **Division in numerator vs denominator**: Different RPN orderings produce different results
4. **Zero division**: Not caught by parser (mathematical error, not syntactic)

### Java Mapping

| Python | Java |
|--------|------|
| `\div` string literal | Raw string or escape sequence |
| Right-side division handling | Conditional in _needs_parens() |
| Precedence level 2 | Same int constant |
| Non-commutative check | String comparison with "/" operator |

### Implementation Checklist

- [ ] TokenType.DIV enum value
- [ ] Lexer._scan_token() recognizes "/" character
- [ ] Parser handles DIV tokens (order critical: right pop before left pop)
- [ ] LaTeXGenerator.BINARY_OPS maps "/" to `\div`
- [ ] LaTeXGenerator.PRECEDENCE maps "/" to 2
- [ ] LaTeXGenerator._needs_parens() checks is_right and child.operator in ("-", "/")
- [ ] Integration test: "10 2 /" -> "$10 \div 2$"
- [ ] Integration test: "100 10 / 5 / 2 /" -> "$100 \div 10 \div 5 \div 2$"
- [ ] Integration test: "10 2 / 5 *" -> "$10 \div 2 \times 5$"
- [ ] Integration test: "10 2 / 3 + 4 *" -> "$( 10 \div 2 + 3 ) \times 4$"

---

# Feature 6: Precedence

## Overview

Operator precedence and parenthesization logic that ensures LaTeX output correctly represents evaluation order.

### Token Definitions

**Files**: None (no new tokens)

### AST Node Definitions

**Files**: None (no new AST nodes)

### Lexer Logic

**Files**: None (no new lexer logic)

### Parser Logic

**Files**: None (no parser changes needed)

The parser already produces correct AST structure due to RPN being unambiguous about precedence. The parser doesn't need to "know" about precedence—it just builds trees as operators are encountered.

### Generator Logic

**File**: `latex_gen.py` (lines 47-62, 143-180)

**Core Algorithm**: Precedence-based parenthesization

**Class Data**:
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2,
}
```

**Method**: `_needs_parens(child: Expr, parent_precedence: int, *, is_right: bool) -> bool`

**Algorithm**:
```python
def _needs_parens(
    self, child: Expr, parent_precedence: int, *, is_right: bool
) -> bool:
    # Numbers never need parens
    if not isinstance(child, BinaryOp):
        return False

    child_precedence = self.PRECEDENCE[child.operator]

    # Rule 1: Lower precedence child always needs parens
    # Example: (5 + 3) when used as operand of multiplication
    if child_precedence < parent_precedence:
        return True

    # Rule 2: Equal precedence on right side needs parens
    #         for non-commutative operators (- and /)
    # Example: 5 - (3 - 2) to avoid changing to 5 - 3 - 2
    return (
        child_precedence == parent_precedence
        and is_right
        and child.operator in ("-", "/")
    )
```

**Invocation** (in _visit_binary_op):
```python
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

### Key Algorithms

**Precedence-Based Parenthesization**:

1. **Lower Precedence Rule**:
   - If child has lower precedence than parent, needs parens
   - Example: "5 3 + 2 *" creates BinaryOp(*, BinaryOp(+, 5, 3), 2)
   - When generating *, left child is BinaryOp(+, ...) with precedence 1
   - Parent precedence is 2, so left child needs parens: "( 5 + 3 ) * 2"

2. **Right-Side Non-Commutative Rule**:
   - If child has equal precedence, is on right side, and operator is "-" or "/", needs parens
   - Example: "5 3 - 2 -" creates BinaryOp(-, BinaryOp(-, 5, 3), 2)
   - When generating *, right child is Number(2), which doesn't need parens
   - But if we had "5 3 - 2 - 1 -", the innermost "3 - 2" would be on right of outer "-"
   - Actually, in RPN this wouldn't happen: the AST structure already enforces left-associativity

3. **Commutative Operators**:
   - Multiplication and addition are commutative: can reorder operands
   - But LaTeX generator doesn't reorder; it preserves structure
   - Same precedence addition as child of addition doesn't need parens: "1 + 2 + 3" not "1 + ( 2 + 3 )"

### Edge Cases

1. **Deeply nested expressions**: "1 2 + 3 4 + *"
   - Creates BinaryOp(*, BinaryOp(+, 1, 2), BinaryOp(+, 3, 4))
   - Both children of * have precedence 1 < 2, so both need parens
   - Output: "( 1 + 2 ) * ( 3 + 4 )"

2. **Chained subtraction**: "5 3 - 2 -"
   - RPN parser creates: BinaryOp(-, BinaryOp(-, 5, 3), 2)
   - Left child of outer - has precedence 1 (same as parent)
   - But left side doesn't trigger right-side rule, so no parens
   - Output: "5 - 3 - 2" (correct left-to-right evaluation)

3. **Division of addition**: "10 2 / 3 + 4 *"
   - Creates complex tree with multiple precedence levels
   - Each level adds parens as needed

4. **No-parenthesis cases**: "2 3 * 4 +"
   - Creates BinaryOp(+, BinaryOp(*, 2, 3), 4)
   - Left child (* with precedence 2) is child of + (precedence 1)
   - Higher precedence child of lower precedence parent doesn't need parens
   - Output: "2 * 3 + 4" (correct, multiplication already binds tighter)

### Java Mapping

| Python | Java |
|--------|------|
| `dict[str, int]` for precedence | `Map<String, Integer>` or array |
| `isinstance(child, BinaryOp)` | `child instanceof BinaryOp` |
| Method parameter `is_right: bool` | Boolean parameter |
| String operator in ("*", "/") check | `equals()` or `switch` statement |
| Conditional logic | Same if-else structure |
| Format string f"( {left} )" | String concatenation or builder |

### Implementation Checklist

- [ ] LaTeXGenerator.PRECEDENCE map initialized with all operators
- [ ] _needs_parens() method with two rules implemented
- [ ] Rule 1: Lower precedence check (child_precedence < parent_precedence)
- [ ] Rule 2: Right-side non-commutative check (child_precedence == parent_precedence && is_right && operator in {"-", "/"})
- [ ] _visit_binary_op() calls _needs_parens() for both left and right operands
- [ ] Parenthesis formatting: "( <operand> )"
- [ ] Integration test: "5 3 + 2 *" -> "$( 5 + 3 ) \times 2$"
- [ ] Integration test: "2 3 * 4 +" -> "$2 \times 3 + 4$"
- [ ] Integration test: "1 2 + 3 4 + *" -> "$( 1 + 2 ) \times ( 3 + 4 )$"
- [ ] Integration test: "10 2 / 3 + 4 *" -> "$( 10 \div 2 + 3 ) \times 4$"

---

## Cross-Feature Dependencies

### Import Chains

**tokens.py** (no imports from project)
- Only uses Python standard library (dataclass, Enum, auto)

**ast_nodes.py** (no imports from project)
- Only uses Python standard library (dataclass)
- Uses forward reference: `Expr = Number | BinaryOp` (Python 3.10+ union syntax)

**errors.py** (no imports from project)
- Only uses Python standard library (string operations)

**lexer.py** (imports from project)
- `from rpn2tex.tokens import Token, TokenType`
- Depends on: tokens.py

**parser.py** (imports from project)
- `from rpn2tex.tokens import Token, TokenType`
- `from rpn2tex.ast_nodes import BinaryOp, Expr, Number`
- Depends on: tokens.py, ast_nodes.py

**latex_gen.py** (imports from project)
- `from rpn2tex.ast_nodes import BinaryOp, Expr, Number`
- `from functools import singledispatchmethod` (Python standard library)
- Depends on: ast_nodes.py

**cli.py** (imports from project)
- `from rpn2tex.errors import ErrorFormatter`
- `from rpn2tex.latex_gen import LaTeXGenerator`
- `from rpn2tex.lexer import Lexer, LexerError`
- `from rpn2tex.parser import Parser, ParserError`
- Depends on: all other modules

### Recommended Java Package Structure

```
com.rpn2tex
├── tokens
│   ├── TokenType.java (enum)
│   └── Token.java (immutable record)
├── ast
│   ├── ASTNode.java (abstract base)
│   ├── Number.java
│   ├── BinaryOp.java
│   └── Expr.java (marker interface or union type)
├── errors
│   ├── LexerError.java
│   ├── ParserError.java
│   └── ErrorFormatter.java
├── lexer
│   └── Lexer.java
├── parser
│   └── Parser.java
├── codegen
│   └── LaTeXGenerator.java
└── cli
    └── Main.java
```

### Dependency Graph for Implementation Order

```
1. tokens (TokenType enum, Token class)
2. ast_nodes (ASTNode, Number, BinaryOp classes)
3. errors (ErrorFormatter, custom exceptions)
4. lexer (Lexer class, depends on tokens)
5. parser (Parser class, depends on tokens + ast_nodes)
6. latex_gen (LaTeXGenerator class, depends on ast_nodes)
7. cli (Main class, depends on all)
```

---

## Python-to-Java Type System Mappings

### Fundamental Types

| Python | Java | Notes |
|--------|------|-------|
| `str` | `String` | Immutable in both |
| `int` | `int` | Primitive for counters |
| `bool` | `boolean` | Primitive |
| `list[T]` | `List<T>` or `ArrayList<T>` | Use interface for declarations |
| `dict[K, V]` | `Map<K, V>` or `HashMap<K, V>` | Use interface for declarations |

### Data Structures

| Python | Java | Notes |
|--------|------|-------|
| `@dataclass(frozen=True)` | Record class (Java 16+) or immutable `final class` | Immutability important for AST |
| `Enum` with `auto()` | `enum` with ordinals | Same concept |
| Union type `A \| B` | Interface or sealed class | Java 16+ sealed classes recommended |
| Type alias `Expr = ...` | Interface `Expr` | Create marker interface |

### Patterns

| Python | Java | Notes |
|--------|------|-------|
| `@singledispatchmethod` | Visitor pattern or method overloading | Java lacks singledispatch; use overload |
| `ClassVar[dict]` | `static final Map` | Class-level constants |
| `dict.get()` | `map.get()` or `map.getOrDefault()` | Similar API |
| List comprehension | Streams or loops | Java lambdas with streams for complex logic |
| Exception with fields | Custom exception class | Store line, column, message fields |

---

## Common Pitfalls and Patterns

### 1. Immutability in AST Nodes

**Python Approach**:
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str
    left: Expr
    right: Expr
```

**Java Approach**:
```java
public record BinaryOp(int line, int column, String operator, Expr left, Expr right)
    implements Expr { }
```

Or with explicit final fields:
```java
public final class BinaryOp implements Expr {
    private final int line;
    private final int column;
    private final String operator;
    private final Expr left;
    private final Expr right;

    public BinaryOp(int line, int column, String operator, Expr left, Expr right) {
        this.line = line;
        this.column = column;
        this.operator = operator;
        this.left = left;
        this.right = right;
    }

    // getters, equals(), hashCode(), toString()
}
```

### 2. Operator Mapping

**Python**:
```python
op_map = {
    TokenType.PLUS: "+",
    TokenType.MINUS: "-",
    TokenType.MULT: "*",
    TokenType.DIV: "/",
}
operator = op_map[token.type]
```

**Java**:
```java
String operator = switch(token.type()) {
    case PLUS -> "+";
    case MINUS -> "-";
    case MULT -> "*";
    case DIV -> "/";
    default -> throw new IllegalArgumentException("Unknown operator: " + token.type());
};
```

Or with Map:
```java
private static final Map<TokenType, String> OP_MAP = Map.of(
    TokenType.PLUS, "+",
    TokenType.MINUS, "-",
    TokenType.MULT, "*",
    TokenType.DIV, "/"
);
String operator = OP_MAP.get(token.type());
```

### 3. Stack Operations in Parser

**Python**:
```python
stack: list[Expr] = []
stack.append(node)
right = stack.pop()
left = stack.pop()
```

**Java**:
```java
Deque<Expr> stack = new ArrayDeque<>();
stack.push(node);
Expr right = stack.pop();
Expr left = stack.pop();
```

Or with List:
```java
List<Expr> stack = new ArrayList<>();
stack.add(node);
Expr right = stack.remove(stack.size() - 1);
Expr left = stack.remove(stack.size() - 1);
```

### 4. Character-by-Character Lexing

**Python**:
```python
def _peek(self) -> str:
    if self._at_end():
        return ""
    return self.text[self.pos]

def _advance(self) -> str:
    char = self.text[self.pos]
    self.pos += 1
    if char == "\n":
        self.line += 1
        self.column = 1
    else:
        self.column += 1
    return char
```

**Java**:
```java
private char peek() {
    if (atEnd()) return '\0';
    return text.charAt(pos);
}

private char advance() {
    char c = text.charAt(pos);
    pos++;
    if (c == '\n') {
        line++;
        column = 1;
    } else {
        column++;
    }
    return c;
}
```

### 5. Visitor Pattern for AST Traversal

**Python** (using singledispatchmethod):
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
```

**Java** (using instanceof + method overloading):
```java
public class LaTeXGenerator {
    public String visit(Expr node) {
        if (node instanceof Number n) {
            return visitNumber(n);
        } else if (node instanceof BinaryOp op) {
            return visitBinaryOp(op);
        } else {
            throw new IllegalArgumentException("Unknown node type: " + node.getClass());
        }
    }

    private String visitNumber(Number node) {
        return node.value();
    }

    private String visitBinaryOp(BinaryOp node) {
        // ...
    }
}
```

Or with sealed classes:
```java
public sealed interface Expr permits Number, BinaryOp { }

public final class LaTeXGenerator {
    public String visit(Expr node) {
        return switch(node) {
            case Number n -> visitNumber(n);
            case BinaryOp op -> visitBinaryOp(op);
        };
    }

    private String visitNumber(Number node) { ... }
    private String visitBinaryOp(BinaryOp node) { ... }
}
```

### 6. String Escaping for LaTeX

**Python**:
```python
BINARY_OPS: ClassVar[dict[str, str]] = {
    "*": r"\times",
    "/": r"\div",
}
```

**Java**:
```java
private static final Map<String, String> BINARY_OPS = Map.of(
    "*", "\\times",
    "/", "\\div"
);
```

Note: Java string literals require double backslash, Python raw strings avoid this.

---

## Integration Testing Strategy

### Test Framework Considerations

**Python Implementation**:
- Uses simple CLI-based testing
- Command: `echo "<input>" | python -m source.cli -`
- Validates stdout against expected LaTeX

**Java Implementation**:
- JUnit 5 recommended
- Create test cases for each feature
- Mock or parametrized tests for multiple inputs

### Sample Java Test Structure

```java
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

public class IntegrationTests {

    @Test
    void testSimpleNumber() throws LexerError, ParserError {
        String input = "5";
        String expected = "$5$";
        String actual = parse(input);
        assertEquals(expected, actual);
    }

    @Test
    void testAddition() throws LexerError, ParserError {
        String input = "5 3 +";
        String expected = "$5 + 3$";
        String actual = parse(input);
        assertEquals(expected, actual);
    }

    private String parse(String input) throws LexerError, ParserError {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator gen = new LaTeXGenerator();
        return gen.generate(ast);
    }
}
```

### Test Coverage Checklist

For each feature:
- [ ] Basic single operation
- [ ] Chained operations (left-associativity)
- [ ] Mixed precedence operations
- [ ] Floating-point numbers
- [ ] Error cases (insufficient operands, etc.)
- [ ] Parenthesization correctness

---

## Summary of Key Invariants

### Parser Invariants

1. **Stack Completeness**: At EOF, exactly one element on stack (the root AST)
2. **Operand Count**: Each operator requires exactly 2 operands
3. **Type Safety**: All stack elements are Expr subtypes
4. **Position Tracking**: Every token and node has line/column information

### LaTeX Generation Invariants

1. **Precedence Preservation**: Output LaTeX evaluates in correct order
2. **Parenthesis Minimality**: No unnecessary parentheses
3. **Format Consistency**: All math wrapped in `$...$`
4. **Operator Escaping**: Special characters properly escaped
5. **Spacing**: Consistent spacing around operators (space before and after)

### Token Invariants

1. **EOF Marker**: Every token stream ends with EOF token
2. **Whitespace Handling**: Whitespace delimits tokens but isn't tokenized
3. **Position Accuracy**: Line and column numbers match source
4. **Unique Values**: Same logical token always has same string value

---

## Glossary

- **RPN (Reverse Polish Notation)**: Postfix notation where operators follow operands
- **Precedence**: Binding tightness of operators; higher precedence binds first
- **Left-associative**: `a - b - c` means `(a - b) - c`, not `a - (b - c)`
- **Non-commutative**: Order of operands matters; `a - b ≠ b - a`
- **LaTeX**: Document preparation system with math mode
- **Visitor Pattern**: Design pattern for operations on complex object structures
- **Immutable**: Cannot be modified after creation
- **Type Union**: Can be one of multiple types (e.g., `Number | BinaryOp`)

---

## File Modification Summary

### Files Modified/Created

| Module | Python File | Lines | Key Changes |
|--------|-------------|-------|------------|
| Tokens | tokens.py | 71 | 5 token types (NUMBER, PLUS, MINUS, MULT, DIV), Token class |
| AST | ast_nodes.py | 91 | ASTNode base, Number, BinaryOp classes, Expr union |
| Errors | errors.py | 128 | ErrorFormatter, LexerError, ParserError classes |
| Lexer | lexer.py | 201 | Lexer class, character scanning, token generation |
| Parser | parser.py | 184 | Parser class, stack-based RPN algorithm |
| LaTeX Generator | latex_gen.py | 185 | LaTeXGenerator class, visitor pattern, precedence logic |
| CLI | cli.py | 115 | Main entry point, pipeline orchestration |

### Total LOC (Production Code): ~955 lines

---

## Appendix: Python Syntax to Java Equivalents

### Function/Method Definitions

| Python | Java |
|--------|------|
| `def method(self, param: str) -> int:` | `public int method(String param) {` |
| `def _private_method():` | `private <returnType> privateMethod() {` |
| Default parameters: `def f(x=5):` | Java: must use overloading or Optional |

### Docstrings

| Python | Java |
|--------|------|
| Triple-quoted strings with examples | JavaDoc with `@param`, `@return`, `@throws` |
| Integrated with code through `__doc__` | Extracted by IDE and javadoc tool |

### Class Variables/Constants

| Python | Java |
|--------|------|
| `PRECEDENCE: ClassVar[dict[str, int]]` | `private static final Map<String, Integer> PRECEDENCE` |

### Iteration

| Python | Java |
|--------|------|
| `for idx in range(start, end):` | `for (int idx = start; idx < end; idx++)` |
| `for item in list:` | `for (Type item : list)` or `.forEach()` |
| `while not condition:` | `while (!condition)` |

### String Operations

| Python | Java |
|--------|------|
| `f"{var}"` f-strings | `"" + var` or `String.format()` or `StringBuilder` |
| `str.isdigit()` | `Character.isDigit(ch)` |
| `str.splitlines()` | `str.split("\n")` or `.lines()` (Java 11+) |
| String slicing `s[0:5]` | `s.substring(0, 5)` |

### Exception Handling

| Python | Java |
|--------|------|
| `raise CustomError("msg", line, col)` | `throw new CustomError("msg", line, col)` |
| `except SpecificError as e:` | `catch (SpecificError e)` |
| Custom exception with fields | Extend Exception, store fields, provide getters |

### Collections

| Python | Java |
|--------|------|
| `list[T]` | `List<T>` (interface), `ArrayList<T>` (impl) |
| `dict[K, V]` | `Map<K, V>` (interface), `HashMap<K, V>` (impl) |
| `tuple` | `record` (Java 16+) or immutable class |
| Set operations | `Set<T>`, `HashSet<T>` |

---

## Next Steps (Phase 2+)

After successful Java implementation of all 6 features:

1. **Feature Testing**: Run all 18 test cases against Java implementation
2. **Error Messages**: Implement detailed error formatting (ErrorFormatter)
3. **File I/O**: Implement CLI for reading/writing files
4. **Performance**: Benchmark against Python reference
5. **Documentation**: Generate Javadoc
6. **Future Extensions**: Implement exponentiation (Feature 7) if needed
