# Phase 1: Feature-by-Feature Migration Specification

**Document Purpose:** Complete specification for migrating rpn2tex from Python to Go, organized by features rather than modules. Each feature represents a cohesive unit of functionality that can be implemented independently (respecting dependencies).

**Specification Date:** 2025-12-29
**Target Language:** Go
**Reference Implementation:** Python (source/tokens.py, source/ast_nodes.py, source/errors.py, source/lexer.py, source/parser.py, source/latex_gen.py, source/cli.py)

---

## I/O Contract

[Verbatim from PHASE_0_IO_CONTRACT.md]

### Verification Summary

All test cases have been executed against the Python reference implementation.

**Verification Date:** 2025-12-29
**Implementation Tested:** Python rpn2tex (source/)
**Total Test Cases:** 21
**Passing Tests:** 18
**Failing Tests:** 3 (exponentiation operator not implemented)

### Test Cases - Basic Operations

#### Numbers
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5` | `$5$` | PASS | Single integer |
| `3.14` | `$3.14$` | PASS | Decimal number |

#### Addition
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 +` | `$5 + 3$` | PASS | Simple addition |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | PASS | Chained addition |

#### Subtraction
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 -` | `$5 - 3$` | PASS | Simple subtraction |
| `5 3 - 2 -` | `$5 - 3 - 2$` | PASS | Chained subtraction |

#### Multiplication
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `4 7 *` | `$4 \times 7$` | PASS | Simple multiplication |
| `2 3 4 * +` | `$2 + 3 \times 4$` | PASS | Multiplication has higher precedence |

#### Division
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `10 2 /` | `$10 \div 2$` | PASS | Simple division |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | PASS | Chained division |

#### Precedence and Parentheses
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | PASS | Addition wrapped in parentheses before multiplication |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | PASS | Addition wrapped before multiplication |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | PASS | Addition wrapped on right operand |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS | Both operands are wrapped expressions |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | PASS | Mixed division and addition in left operand |

#### Mixed Operations (Precedence)
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `5 3 * 2 +` | `$5 \times 3 + 2$` | PASS | Multiplication evaluated first, no parens needed |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | PASS | Same precedence, left to right |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | PASS | Multiplication before addition |

#### Decimal Numbers
| Input | Expected Output | Status | Notes |
|-------|-----------------|--------|-------|
| `3.14 2 *` | `$3.14 \times 2$` | PASS | Decimal number in multiplication |
| `1.5 0.5 +` | `$1.5 + 0.5$` | PASS | Decimal numbers in addition |

### Error Cases - Exponentiation Not Supported

| Input | Error Message | Status | Notes |
|-------|---------------|--------|-------|
| `2 3 ^` | `Error: Unexpected character '^'` | ERROR | Exponentiation operator not supported |
| `2 3 ^ 4 *` | `Error: Unexpected character '^'` | ERROR | Exponentiation operator not supported |
| `2 3 4 ^ ^` | `Error: Unexpected character '^'` | ERROR | Exponentiation operator not supported |

### Lexer Output Format

When the exponentiation operator (^) is encountered during lexing, the error message follows this format:

```
Error: Unexpected character '^'

1 | 2 3 ^
  |     ^
```

The error indicates:
- The line number (1-indexed)
- The line content
- A caret (^) pointing to the problematic character

### I/O Contract Validation Summary

#### Arithmetic Operators Supported
- Addition (+) → outputs `+` with space padding
- Subtraction (-) → outputs `-` with space padding
- Multiplication (*) → outputs `\times` with space padding
- Division (/) → outputs `\div` with space padding

#### Number Format
- Integers: output as-is (e.g., `5` → `5`)
- Decimals: output as-is (e.g., `3.14` → `3.14`)
- Wrapped in LaTeX math mode: `$...$`

#### Parentheses Rules
1. Parentheses are added when a lower-precedence operation is a child of a higher-precedence operation
2. Precedence: Multiplication/Division > Addition/Subtraction
3. Format: `( left ) operator right` or `left operator ( right )`
4. Spaces included: `( `, ` )`, ` operator `

#### Output Format
- All output is wrapped in `$...$` (LaTeX inline math mode)
- Binary operations output as: `left operator right`
- Spaces surround operators: ` + `, ` - `, ` \times `, ` \div `
- Parentheses include spaces: `( expression )`

---

# Feature 1: Numbers

## Feature Overview

**What it does:**
The Numbers feature handles parsing and outputting numeric literals, including both integers and decimal numbers. This is the foundational feature upon which all other operations depend.

**Dependencies:**
- No internal dependencies (foundational feature)

## Token Layer (tokens.py)

### Token Types Needed
- `TokenType.NUMBER` - numeric literals

### Token Structure
```python
@dataclass(frozen=True)
class Token:
    type: TokenType      # The token type
    value: str          # String representation of the number
    line: int           # 1-based line number
    column: int         # 1-based column number
```

### Key Attributes
- **type:** Must be `TokenType.NUMBER`
- **value:** String representation (can be "5", "3.14", "-2")
- **line/column:** Position tracking for error reporting

## AST Layer (ast_nodes.py)

### AST Node Types Needed
- `Number` - represents numeric literals

### Node Structure
```python
@dataclass(frozen=True)
class Number(ASTNode):
    value: str    # String representation of the number
    line: int     # Position tracking
    column: int   # Position tracking
```

### Methods Required
- Constructor accepting `line`, `column`, `value` parameters
- Immutable (frozen dataclass in Python)

## Lexer Layer (lexer.py)

### Token Recognition Pattern
Numbers are recognized by:
1. Checking if character is a digit (0-9)
2. Optionally preceded by "-" for negative numbers
3. Optionally containing a single decimal point "."
4. Followed by more digits

### Pattern Matching Logic
- **Positive numbers:** `[0-9]+(\.[0-9]+)?`
- **Negative numbers:** `-[0-9]+(\.[0-9]+)?`
- **Whitespace delimiter:** Numbers are separated by whitespace

### Lexer Implementation Details
```python
def _scan_number(self, prefix: str, start_line: int, start_column: int) -> Token:
    """
    Scan a numeric literal including:
    - Prefix (e.g., "-" for negative numbers)
    - Integer part (digits only)
    - Optional decimal point and fractional part
    """
    value = prefix
    # Scan integer part: [0-9]+
    while not self._at_end() and self._peek().isdigit():
        value += self._advance()
    # Scan decimal part: (\.[0-9]+)?
    if not self._at_end() and self._peek() == ".":
        value += self._advance()
        while not self._at_end() and self._peek().isdigit():
            value += self._advance()
    return Token(TokenType.NUMBER, value, start_line, start_column)
```

### State Requirements
- **pos:** Current position in text (0-based)
- **line:** Current line number (1-based)
- **column:** Current column number (1-based)
- **text:** The input being scanned

## Parser Layer (parser.py)

### AST Construction
When a NUMBER token is encountered:
1. Create a `Number` node with the token's value, line, and column
2. Push the node onto the evaluation stack
3. Advance to the next token

### Stack Operations
```python
if token.type == TokenType.NUMBER:
    num_node = Number(
        line=token.line,
        column=token.column,
        value=token.value
    )
    stack.append(num_node)
```

### Error Handling
- Numbers are always valid (no syntax errors at parse time)
- Position information is preserved for potential runtime errors

## Generator Layer (latex_gen.py)

### LaTeX Generation
For a `Number` node, simply output the number value as-is:

```python
@_visit.register
def _visit_number(self, node: Number) -> str:
    """Generate LaTeX for a number literal."""
    return node.value
```

### No Parenthesization
Numbers never require parentheses

## Cross-Cutting Concerns

### Error Handling
- **LexerError:** Raised for invalid numeric formats (handled by separator logic)
- **Position tracking:** All tokens must have accurate line and column numbers

### Edge Cases
1. **Negative numbers:** Must be distinguished from subtraction operator
   - Implementation: Check if "-" is followed immediately by a digit
   - If yes, treat as negative number; if no or whitespace, treat as operator

2. **Decimal numbers:** Single decimal point allowed
   - Leading digit required (e.g., "3.14" valid, ".14" not tested)
   - Trailing digit after decimal required (e.g., "3.14" valid, "3." not tested)

3. **Whitespace:** Numbers are delimited by whitespace
   - Supports spaces, tabs, newlines

### Validation Requirements
- Number must be complete (all digits/decimal point consumed)
- Position information must be accurately tracked

## Go Implementation Guide

### Recommended Go Idioms

```go
// Token type enum - Go has no native enum, use const with iota
const (
    NUMBER TokenType = iota
    // ... other types
)

// Struct for Token
type Token struct {
    Type   TokenType
    Value  string
    Line   int
    Column int
}

// Struct for Number AST node
type Number struct {
    Value  string
    Line   int
    Column int
}

// Implement Node interface if using interface-based approach
func (n *Number) astNode() {}
```

### Type Definitions Needed
- `TokenType` - Use `uint` with const definitions or custom type
- `Token` - Struct with Type (TokenType), Value (string), Line (int), Column (int)
- `Number` - Struct with Value (string), Line (int), Column (int)

### Interface Requirements
- If using visitor pattern: `Number` should implement a common `Expr` interface
- If using type assertion: `Number` must be a concrete type that can be asserted

### String Handling
- Go's `string` type is immutable (like Python)
- Use `strings.Builder` for constructing output
- Rune vs byte: Use `rune` for character classification

### Specific Implementation Notes
1. **Negative number detection:** Use `unicode.IsDigit()` or `rune.IsDigit()`
2. **Decimal point handling:** Check for '.' character with `ch == '.'`
3. **String concatenation:** Use `strings.Builder` for efficiency
4. **Position tracking:** Increment column for each character, reset to 1 on newline

## Test Cases (from verified I/O contract)

### Passing Tests
```
Input: "5"
Expected: "$5$"

Input: "3.14"
Expected: "$3.14$"
```

### Edge Cases to Verify
- Single digit: `5` → `$5$`
- Multi-digit: `123` → `$123$`
- Decimal: `3.14` → `$3.14$`
- Long decimal: `1.5` → `$1.5$`
- Zero: `0` → `$0$` (not explicitly tested, but should work)

---

# Feature 2: Addition

## Feature Overview

**What it does:**
The Addition feature implements the binary addition operator (+). It handles tokenization, parsing, and LaTeX generation of addition expressions.

**Dependencies:**
- **Must be implemented after:** Feature 1 (Numbers)
- No other feature dependencies

## Token Layer (tokens.py)

### Token Types Needed
- `TokenType.PLUS` - addition operator

### Token Structure
```python
# Inherits from Token class defined in Feature 1
# type: TokenType.PLUS
# value: "+"
# line/column: Position of the "+" character
```

## AST Layer (ast_nodes.py)

### AST Node Types Needed
- `BinaryOp` - represents binary operations including addition

### Node Structure
```python
@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str    # The operator string ("+" for addition)
    left: Expr       # Left operand (can be Number or BinaryOp)
    right: Expr      # Right operand (can be Number or BinaryOp)
    line: int        # Position of operator
    column: int      # Position of operator
```

### Methods Required
- Constructor accepting `line`, `column`, `operator`, `left`, `right`
- Immutable (frozen dataclass)

## Lexer Layer (lexer.py)

### Token Recognition Pattern
Single character "+" recognized as PLUS token:

```python
if char == "+":
    self._advance()
    return Token(TokenType.PLUS, "+", start_line, start_column)
```

### Pattern Matching Logic
- Simple single-character match: `ch == '+' && !isdigit(next_char)`
- No ambiguity with other operators

### Lexer State Requirements
- Same as Feature 1 (pos, line, column, text)

## Parser Layer (parser.py)

### AST Construction for Addition
When PLUS token encountered:
1. Pop two operands from stack (must exist)
2. Create `BinaryOp` node with operator "+"
3. Push result back onto stack

### Stack Operations
```python
elif token.type == TokenType.PLUS:
    if len(stack) < 2:
        raise ParserError(f"Operator '{token.value}' requires two operands", token)
    right = stack.pop()
    left = stack.pop()
    op_node = BinaryOp(
        line=token.line,
        column=token.column,
        operator="+",
        left=left,
        right=right
    )
    stack.append(op_node)
    self._advance()
```

### RPN Semantics
- Addition is **commutative** but operates in RPN order
- Example: `5 3 +` produces `BinaryOp("+", Number("5"), Number("3"))`
- Output should be: `5 + 3` (left operand first)

### Error Handling
- **InsufficientOperands:** Raise ParserError if fewer than 2 elements on stack
- Error message format: `"Operator '+' requires two operands"`

## Generator Layer (latex_gen.py)

### LaTeX Generation for Addition
```python
# For BinaryOp with operator "+"
left_latex = self._visit(node.left)
right_latex = self._visit(node.right)
result = f"{left_latex} + {right_latex}"
```

### Operator Mapping
- Python operator: `"+"`
- LaTeX output: `"+"`
- Spacing: ` + ` (spaces on both sides)

### Precedence Rules
- **Precedence level:** 1 (lowest among implemented operators)
- **Associativity:** Left-associative (addition is naturally left-associative)
- Parenthesization rules covered in Feature 6

### No Special Parenthesization for Addition Alone
- Addition never needs parentheses when both operands are numbers
- Addition needs parentheses only when it's a child of multiplication/division

## Cross-Cutting Concerns

### Error Handling
- **ParserError:** Insufficient operands (< 2 on stack)
- Position information preserved from operator token

### Edge Cases
1. **Chained addition:** Multiple additions in sequence
   - Example: `1 2 + 3 + 4 +` produces left-associative tree
   - Output: `1 + 2 + 3 + 4`

2. **Addition with different operand types:**
   - Left can be Number or BinaryOp
   - Right can be Number or BinaryOp
   - Both cases must be handled

3. **Decimal operands:** Addition works with decimals
   - Example: `1.5 0.5 +` produces `1.5 + 0.5`

## Go Implementation Guide

### Type Definitions Needed
- `BinaryOp` struct with fields: operator (string), left (Expr), right (Expr), line (int), column (int)
- Update `Expr` type to be `Number | BinaryOp` (using interface or union type)

### Interface Requirements
```go
// If using interface-based approach
type Expr interface {
    exprNode()
}

// Both Number and BinaryOp must implement Expr
func (n *Number) exprNode() {}
func (b *BinaryOp) exprNode() {}
```

### Pattern Matching
- Use type switch or type assertion to differentiate between Number and BinaryOp
- Go example: `switch node := node.(type) { case *Number: ... case *BinaryOp: ... }`

### String Operations
- Operator field is simply `"+"`
- Concatenation uses `fmt.Sprintf()` or `strings.Builder`

## Test Cases (from verified I/O contract)

### Passing Tests
```
Input: "5 3 +"
Expected: "$5 + 3$"

Input: "1 2 + 3 + 4 +"
Expected: "$1 + 2 + 3 + 4$"
```

### Stack Operations Verification
```
"1 2 + 3 +"
Stack: [] → [1] → [1,2] → [BinaryOp(+,1,2)] → [BinaryOp(+,1,2),3] → [BinaryOp(+,BinaryOp(+,1,2),3)]
Output: "1 + 2 + 3"
```

---

# Feature 3: Subtraction

## Feature Overview

**What it does:**
The Subtraction feature implements the binary subtraction operator (-). Unlike addition, subtraction is non-commutative and left-associative, affecting both parsing and output generation.

**Dependencies:**
- **Must be implemented after:** Feature 1 (Numbers), Feature 2 (Addition)
- No other feature dependencies

## Token Layer (tokens.py)

### Token Types Needed
- `TokenType.MINUS` - subtraction operator

### Token Structure
```python
# type: TokenType.MINUS
# value: "-"
# line/column: Position of the "-" character
```

### Disambiguation Note
- Single "-" can represent either subtraction operator OR negative number
- Lexer must disambiguate: if "-" followed immediately by digit (no whitespace), it's a negative number
- Otherwise, it's the subtraction operator

## AST Layer (ast_nodes.py)

### AST Node Types Needed
- `BinaryOp` (reused from Feature 2)
- operator field will contain "-"

## Lexer Layer (lexer.py)

### Token Recognition Pattern
The "-" character requires special handling:

```python
if char == "-":
    self._advance()
    # Check if this is a negative number (digit follows immediately)
    if not self._at_end() and self._peek().isdigit():
        # It's a negative number
        return self._scan_number("-", start_line, start_column)
    # Otherwise it's subtraction operator
    return Token(TokenType.MINUS, "-", start_line, start_column)
```

### Pattern Matching Logic
1. If "-" is followed immediately by digit → negative number
2. If "-" is followed by non-digit or whitespace → subtraction operator

### Lexer State Requirements
- Same as previous features

## Parser Layer (parser.py)

### AST Construction for Subtraction
Identical to addition but with operator "-":

```python
elif token.type == TokenType.MINUS:
    if len(stack) < 2:
        raise ParserError(f"Operator '{token.value}' requires two operands", token)
    right = stack.pop()
    left = stack.pop()
    op_node = BinaryOp(
        line=token.line,
        column=token.column,
        operator="-",
        left=left,
        right=right
    )
    stack.append(op_node)
    self._advance()
```

### RPN Semantics
- Subtraction is **NOT commutative**: `5 - 3 ≠ 3 - 5`
- Order matters: left operand is minuend, right operand is subtrahend
- Example: `5 3 -` produces `BinaryOp("-", Number("5"), Number("3"))` → `5 - 3`

### Error Handling
- **InsufficientOperands:** ParserError if fewer than 2 elements on stack
- **OperandOrder:** Stack order determines operand positions

## Generator Layer (latex_gen.py)

### LaTeX Generation for Subtraction
```python
# For BinaryOp with operator "-"
left_latex = self._visit(node.left)
right_latex = self._visit(node.right)
result = f"{left_latex} - {right_latex}"
```

### Operator Mapping
- Python operator: `"-"`
- LaTeX output: `"-"`
- Spacing: ` - ` (spaces on both sides)

### Precedence Rules
- **Precedence level:** 1 (same as addition)
- **Associativity:** Left-associative
  - `5 - 3 - 2` means `(5 - 3) - 2` = 0, NOT `5 - (3 - 2)` = 4

### Parenthesization for Subtraction
Special case: When subtraction is on the **right side** of another subtraction or division, parentheses ARE needed:

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
        and child.operator in ("-", "/")  # Non-commutative operators
    )
```

Example: `5 3 - 2 -`
- Parses to: `BinaryOp("-", BinaryOp("-", 5, 3), 2)`
- Output: `5 - 3 - 2` (no parens needed because subtraction is on the left)
- The right operand is just a number, so no parens needed

## Cross-Cutting Concerns

### Error Handling
- **ParserError:** Insufficient operands
- **Position tracking:** From operator token

### Edge Cases
1. **Chained subtraction:** Left-associative
   - `5 3 - 2 -` → `(5 - 3) - 2` = 0
   - Output: `5 - 3 - 2`

2. **Right subtraction in context:** Parentheses needed when on right
   - For Feature 3 alone: Not possible (subtraction operator has same precedence)
   - Relevant for Feature 6 (Precedence) when combined with higher-precedence operators

3. **Negative numbers vs subtraction:**
   - Lexer must disambiguate
   - `-5` is a negative number (single token)
   - `5 -3 +` is subtraction operator followed by negative number

## Go Implementation Guide

### Type Definitions
- Reuse `BinaryOp` struct from Feature 2
- operator field will contain `"-"`

### Pattern Matching for Disambiguation
```go
// In lexer
if char == '-' {
    // Peek at next character
    if peekNext() != EOF && unicode.IsDigit(rune(peekNext())) {
        // Negative number
        return scanNumber("-", startLine, startColumn)
    } else {
        // Subtraction operator
        return Token{Type: MINUS, Value: "-", Line: startLine, Column: startColumn}
    }
}
```

### Precedence Handling
- Store precedence map: `map[string]int{"+": 1, "-": 1, "*": 2, "/": 2}`
- Non-commutative operator set: `map[string]bool{"-": true, "/": true}`

## Test Cases (from verified I/O contract)

### Passing Tests
```
Input: "5 3 -"
Expected: "$5 - 3$"

Input: "5 3 - 2 -"
Expected: "$5 - 3 - 2$"
```

### Stack Operations Verification
```
"5 3 - 2 -"
Stack: [] → [5] → [5,3] → [BinaryOp(-,5,3)] → [BinaryOp(-,5,3),2] → [BinaryOp(-,BinaryOp(-,5,3),2)]
Output: "5 - 3 - 2"
```

---

# Feature 4: Multiplication

## Feature Overview

**What it does:**
The Multiplication feature implements the binary multiplication operator (*). Multiplication has **higher precedence** than addition and subtraction, which affects parenthesization in the output. This is the first feature to introduce precedence-based parenthesis insertion.

**Dependencies:**
- **Must be implemented after:** Feature 1 (Numbers), Feature 2 (Addition)
- Feature 3 (Subtraction) recommended but not strictly required for basic cases
- Feature 5 (Division) not required (independent operator)

## Token Layer (tokens.py)

### Token Types Needed
- `TokenType.MULT` - multiplication operator

### Token Structure
```python
# type: TokenType.MULT
# value: "*"
# line/column: Position of the "*" character
```

## AST Layer (ast_nodes.py)

### AST Node Types Needed
- `BinaryOp` (reused from Feature 2)
- operator field will contain "*"

## Lexer Layer (lexer.py)

### Token Recognition Pattern
Single character "*" recognized as MULT token:

```python
if char == "*":
    self._advance()
    return Token(TokenType.MULT, "*", start_line, start_column)
```

### Pattern Matching Logic
- Simple single-character match: `ch == '*'`
- No ambiguity with other operators

## Parser Layer (parser.py)

### AST Construction for Multiplication
Identical to addition and subtraction but with operator "*":

```python
elif token.type == TokenType.MULT:
    if len(stack) < 2:
        raise ParserError(f"Operator '{token.value}' requires two operands", token)
    right = stack.pop()
    left = stack.pop()
    op_node = BinaryOp(
        line=token.line,
        column=token.column,
        operator="*",
        left=left,
        right=right
    )
    stack.append(op_node)
    self._advance()
```

### RPN Semantics
- Multiplication is commutative: `5 * 3 = 3 * 5`
- But order in RPN determines operand positions in output
- Example: `2 3 4 * +` produces `BinaryOp("+", Number("2"), BinaryOp("*", Number("3"), Number("4")))` → `2 + 3 * 4`

### Error Handling
- **InsufficientOperands:** ParserError if fewer than 2 elements on stack

## Generator Layer (latex_gen.py)

### LaTeX Generation for Multiplication
```python
# For BinaryOp with operator "*"
left_latex = self._visit(node.left)
right_latex = self._visit(node.right)
result = f"{left_latex} \\times {right_latex}"
```

### Operator Mapping
- Python operator: `"*"`
- LaTeX output: `r"\times"` (backslash times)
- Spacing: ` \times ` (spaces on both sides)
- Important: Single backslash in Python raw string becomes `\times` in output

### Precedence Rules
- **Precedence level:** 2 (higher than addition/subtraction which are level 1)
- **Associativity:** Left-associative

### Critical Parenthesization for Multiplication
This is where precedence becomes essential. When an **addition or subtraction** is an operand of multiplication, it must be wrapped in parentheses:

```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2,
}

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

Examples:
- `2 3 4 * +` → `2 + 3 * 4` (no parens: multiplication has higher precedence)
- `5 3 + 2 *` → `( 5 + 3 ) * 2` (parens needed: addition is lower precedence)
- `2 3 * 4 *` → `2 * 3 * 4` (no parens: same precedence, left-associative)

## Cross-Cutting Concerns

### Error Handling
- **ParserError:** Insufficient operands
- **Position tracking:** From operator token

### Edge Cases
1. **Multiplication with added expressions:**
   - `5 3 + 2 *` → `( 5 + 3 ) * 2` (both left and right parens)
   - `2 3 4 + *` → `2 * ( 3 + 4 )` (right parens only)
   - `1 2 + 3 4 + *` → `( 1 + 2 ) * ( 3 + 4 )` (both sides)

2. **Multiplication with divided expressions:**
   - `10 2 / 3 +` → `10 / 2 + 3` (no parens: division same precedence)
   - But if this becomes operand to multiplication, division needs parens

3. **Chained multiplication:**
   - `2 3 * 4 *` → `2 * 3 * 4` (no parens, left-associative)

## Go Implementation Guide

### Type Definitions
- Reuse `BinaryOp` struct from Feature 2
- operator field will contain `"*"`

### Precedence Map
```go
var precedence = map[string]int{
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2,
}

var nonCommutative = map[string]bool{
    "-": true,
    "/": true,
}
```

### LaTeX Operator Mapping
```go
var binaryOps = map[string]string{
    "+": "+",
    "-": "-",
    "*": `\times`,
    "/": `\div`,
}
```

## Test Cases (from verified I/O contract)

### Passing Tests
```
Input: "4 7 *"
Expected: "$4 \times 7$"

Input: "2 3 4 * +"
Expected: "$2 + 3 \times 4$"

Input: "5 3 * 2 +"
Expected: "$5 \times 3 + 2$"
```

### Parenthesization Verification
```
Input: "5 3 + 2 *"
Expected: "$( 5 + 3 ) \times 2$"

Input: "2 3 + 4 *"
Expected: "$( 2 + 3 ) \times 4$"

Input: "2 3 4 + *"
Expected: "$2 \times ( 3 + 4 )$"

Input: "1 2 + 3 4 + *"
Expected: "$( 1 + 2 ) \times ( 3 + 4 )$"
```

---

# Feature 5: Division

## Feature Overview

**What it does:**
The Division feature implements the binary division operator (/). Like multiplication, division has **higher precedence** than addition and subtraction. Additionally, division is **non-commutative** and **left-associative**, meaning parentheses are needed on the right side of division when the right operand is another division.

**Dependencies:**
- **Must be implemented after:** Feature 1 (Numbers), Feature 2 (Addition)
- Feature 3 (Subtraction) recommended but not required for basic cases
- Feature 4 (Multiplication) not required (independent operator)

## Token Layer (tokens.py)

### Token Types Needed
- `TokenType.DIV` - division operator

### Token Structure
```python
# type: TokenType.DIV
# value: "/"
# line/column: Position of the "/" character
```

## AST Layer (ast_nodes.py)

### AST Node Types Needed
- `BinaryOp` (reused from Feature 2)
- operator field will contain "/"

## Lexer Layer (lexer.py)

### Token Recognition Pattern
Single character "/" recognized as DIV token:

```python
if char == "/":
    self._advance()
    return Token(TokenType.DIV, "/", start_line, start_column)
```

### Pattern Matching Logic
- Simple single-character match: `ch == '/'`
- No ambiguity with other operators

## Parser Layer (parser.py)

### AST Construction for Division
Identical to multiplication but with operator "/":

```python
elif token.type == TokenType.DIV:
    if len(stack) < 2:
        raise ParserError(f"Operator '{token.value}' requires two operands", token)
    right = stack.pop()
    left = stack.pop()
    op_node = BinaryOp(
        line=token.line,
        column=token.column,
        operator="/",
        left=left,
        right=right
    )
    stack.append(op_node)
    self._advance()
```

### RPN Semantics
- Division is **NOT commutative**: `10 / 2 ≠ 2 / 10`
- Order matters: left operand is dividend, right operand is divisor
- Example: `10 2 /` produces `BinaryOp("/", Number("10"), Number("2"))` → `10 / 2`

### Error Handling
- **InsufficientOperands:** ParserError if fewer than 2 elements on stack

## Generator Layer (latex_gen.py)

### LaTeX Generation for Division
```python
# For BinaryOp with operator "/"
left_latex = self._visit(node.left)
right_latex = self._visit(node.right)
result = f"{left_latex} \\div {right_latex}"
```

### Operator Mapping
- Python operator: `"/"`
- LaTeX output: `r"\div"` (backslash div)
- Spacing: ` \div ` (spaces on both sides)

### Precedence Rules
- **Precedence level:** 2 (same as multiplication, higher than addition/subtraction)
- **Associativity:** Left-associative
  - `100 / 10 / 5 / 2` means `(((100 / 10) / 5) / 2)` = 1, NOT `100 / (10 / (5 / 2))`

### Parenthesization for Division
Division follows the same precedence rules as multiplication but with an important distinction:
- Lower precedence operands (+ and -) need parentheses
- Same precedence on the RIGHT side needs parentheses (because division is left-associative and non-commutative)

```python
# Using the same _needs_parens logic:
# Division is treated as non-commutative, so right-side same-precedence needs parens
return (
    child_precedence == parent_precedence
    and is_right
    and child.operator in ("-", "/")
)
```

Examples:
- `10 2 /` → `10 / 2` (simple division, no parens)
- `100 10 / 5 / 2 /` → `100 / 10 / 5 / 2` (chained left-associative, no parens)
- `10 2 / 3 +` → `10 / 2 + 3` (different precedence, no parens)
- `10 2 / 5 *` → `10 / 2 * 5` (same precedence, left-to-right, no parens)

## Cross-Cutting Concerns

### Error Handling
- **ParserError:** Insufficient operands
- **Position tracking:** From operator token
- Note: Division by zero is NOT caught (output is still valid LaTeX; runtime behavior is up to user)

### Edge Cases
1. **Chained division:**
   - `100 10 / 5 / 2 /` → `100 / 10 / 5 / 2`
   - Left-associative: each operator groups with accumulated result on left

2. **Division with different operands:**
   - Decimal operands: `3.14 2 /` works fine
   - Integer operands: `10 2 /` works fine

3. **Division in mixed expressions (Feature 6):**
   - `10 2 / 3 + 4 *` → `( 10 / 2 + 3 ) * 4`
   - Division same precedence as multiplication, both higher than addition

## Go Implementation Guide

### Type Definitions
- Reuse `BinaryOp` struct from Feature 2
- operator field will contain `"/"`

### Precedence Considerations
- Division shares precedence level with multiplication (level 2)
- Both are non-commutative in terms of parenthesization

## Test Cases (from verified I/O contract)

### Passing Tests
```
Input: "10 2 /"
Expected: "$10 \div 2$"

Input: "100 10 / 5 / 2 /"
Expected: "$100 \div 10 \div 5 \div 2$"

Input: "10 2 / 5 *"
Expected: "$10 \div 2 \times 5$"
```

---

# Feature 6: Precedence and Parenthesization

## Feature Overview

**What it does:**
The Precedence feature handles operator precedence rules and automatic parenthesization of sub-expressions in the LaTeX output. This feature ensures that the mathematical meaning is preserved in infix notation when converting from RPN.

**Key Insight:** In RPN, precedence is implicit in the structure (stack operations determine evaluation order). When converting to infix notation, precedence must be made explicit through parentheses.

**Dependencies:**
- **Must be implemented after:** Features 1-5 (all operators and their precedence levels)
- Assumes all previous features fully implemented
- This is a cross-cutting feature that affects all operator output

## Precedence Rules

### Precedence Levels
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1,  # Lowest precedence
    "-": 1,
    "*": 2,  # Highest precedence
    "/": 2,
}
```

**Interpretation:**
- Higher number = tighter binding / higher precedence
- Operations with higher precedence are evaluated first
- When a lower-precedence operation is a child of higher-precedence operation, parentheses are needed

### Parenthesization Decision Logic

A child expression needs parentheses if:
1. **Lower precedence:** Child's precedence < parent's precedence
2. **Equal precedence, right side, non-commutative:** Child precedence == parent precedence AND is_right=True AND operator in {"-", "/"}

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
    return (
        child_precedence == parent_precedence
        and is_right
        and child.operator in ("-", "/")
    )
```

### Why Right-Side Special Handling?

For non-commutative, left-associative operators (- and /):
- `5 - (3 - 2)` = 4 (parens change meaning)
- `(5 - 3) - 2` = 0 (implicit left-association)
- `5 - 3 - 2` is interpreted as left-associative by default, so no parens needed

BUT in RPN parsing, if we have `5 3 2 - -`, this creates:
- Stack: [5] → [5, 3] → [5, 3, 2]
- On first `-`: pop 2 and 3, create BinaryOp(-, 3, 2), stack: [5, BinaryOp(-, 3, 2)]
- On second `-`: pop BinaryOp(-, 3, 2) and 5, create BinaryOp(-, 5, BinaryOp(-, 3, 2))
- This represents: 5 - (3 - 2) = 4, which NEEDS parentheses in output!

So when the right operand of a subtraction/division is itself a subtraction/division, parentheses are needed to preserve the RPN semantics.

## Token Layer

No changes needed. Uses existing token types from Features 1-5.

## AST Layer

No changes needed. Uses existing AST nodes from Features 1-5.

## Lexer Layer

No changes needed. Uses existing lexer from Features 1-5.

## Parser Layer

No changes needed. Uses existing parser from Features 1-5.

## Generator Layer (latex_gen.py)

### Updated _visit_binary_op Implementation
```python
def _visit_binary_op(self, node: BinaryOp) -> str:
    """Generate LaTeX for a binary operation with precedence handling."""
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

### Parentheses Formatting
- Format: `( expression )`
- Spacing: space after `(` and before `)`
- Quotes: no quotes in output, just the characters

## Cross-Cutting Concerns

### Interaction with All Operators
This feature affects how all binary operations are rendered. The precedence map must include all operators implemented.

### Edge Cases

1. **Nested same-precedence on left vs right:**
   - Left: `5 3 - 2 -` → `5 - 3 - 2` (left-associative, no parens)
   - Right: `5 3 2 - -` → `5 - ( 3 - 2 )` (right operand is subtraction, needs parens)

2. **Mixed operators:**
   - `10 2 / 3 + 4 *` → `( 10 / 2 + 3 ) * 4` (division same precedence as multiplication, but mixed with addition)
   - Evaluation: `(10 / 2) + 3` = 8, then `8 * 4` = 32

3. **Multiple levels:**
   - `1 2 + 3 4 + *` → `( 1 + 2 ) * ( 3 + 4 )` (both operands need parens)

4. **No parens needed when not required:**
   - `5 3 * 2 +` → `5 * 3 + 2` (multiplication higher precedence, no parens)
   - `2 3 4 * +` → `2 + 3 * 4` (multiplication already higher, no parens)

## Go Implementation Guide

### Data Structures
```go
// Precedence map
var precedence = map[string]int{
    "+": 1,
    "-": 1,
    "*": 2,
    "/": 2,
}

// Non-commutative operator set
var nonCommutative = map[string]bool{
    "-": true,
    "/": true,
}

// LaTeX operator mapping
var binaryOps = map[string]string{
    "+": "+",
    "-": "-",
    "*": `\times`,
    "/": `\div`,
}
```

### Type Assertions and Pattern Matching
In Go, to check if a node is BinaryOp:
```go
func needsParens(child Expr, parentPrecedence int, isRight bool) bool {
    binOp, ok := child.(*BinaryOp)
    if !ok {
        return false
    }

    childPrec := precedence[binOp.Operator]

    if childPrec < parentPrecedence {
        return true
    }

    if childPrec == parentPrecedence && isRight && nonCommutative[binOp.Operator] {
        return true
    }

    return false
}
```

### Visitor Pattern Implementation
Use type assertion with switch statement:
```go
func (lg *LaTeXGenerator) visit(node Expr) string {
    switch n := node.(type) {
    case *Number:
        return visitNumber(n)
    case *BinaryOp:
        return visitBinaryOp(n)
    default:
        return ""
    }
}
```

## Test Cases (from verified I/O contract)

### Precedence Test Cases

#### Parentheses Needed (Lower precedence child)
```
Input: "5 3 + 2 *"
Expected: "$( 5 + 3 ) \times 2$"
Explanation: Addition (precedence 1) is child of multiplication (precedence 2)

Input: "2 3 + 4 *"
Expected: "$( 2 + 3 ) \times 4$"

Input: "2 3 4 + *"
Expected: "$2 \times ( 3 + 4 )$"
Explanation: Right operand is addition, needs parens

Input: "1 2 + 3 4 + *"
Expected: "$( 1 + 2 ) \times ( 3 + 4 )$"
Explanation: Both operands are additions

Input: "10 2 / 3 + 4 *"
Expected: "$( 10 \div 2 + 3 ) \times 4$"
Explanation: Mixed division and addition become operand to multiplication
```

#### No Parentheses Needed (Higher or equal precedence)
```
Input: "5 3 * 2 +"
Expected: "$5 \times 3 + 2$"
Explanation: Multiplication (precedence 2) is operand of addition (precedence 1) - no issue

Input: "2 3 * 4 +"
Expected: "$2 \times 3 + 4$"

Input: "10 2 / 5 *"
Expected: "$10 \div 2 \times 5$"
Explanation: Same precedence, left-to-right, no parens

Input: "100 10 / 5 / 2 /"
Expected: "$100 \div 10 \div 5 \div 2$"
Explanation: Chained same precedence, left-associative, no parens
```

#### Decimal Numbers with Precedence
```
Input: "3.14 2 *"
Expected: "$3.14 \times 2$"

Input: "1.5 0.5 +"
Expected: "$1.5 + 0.5$"
```

---

# Integration Summary

## Feature Implementation Order

**Recommended sequence** (respecting all dependencies):

1. **Feature 1: Numbers** - Foundation for all other features
2. **Feature 2: Addition** - First operator, introduces BinaryOp
3. **Feature 3: Subtraction** - Introduces non-commutative operators, "-" disambiguation
4. **Feature 4: Multiplication** - Introduces higher precedence, first parenthesization
5. **Feature 5: Division** - Similar to multiplication, completes operator set
6. **Feature 6: Precedence** - Unifies parenthesization logic across all operators

## Module-to-Feature Mapping

Each Python module supports multiple features:

| Module | Features Implemented |
|--------|----------------------|
| tokens.py | All features (token types) |
| ast_nodes.py | All features (AST nodes) |
| errors.py | All features (error formatting) |
| lexer.py | All features (tokenization) |
| parser.py | All features (RPN parsing) |
| latex_gen.py | All features (LaTeX generation with precedence) |
| cli.py | All features (CLI orchestration) |

## Implementation Checklist for Each Feature

For each feature, ensure:
- [ ] Token types defined
- [ ] AST nodes defined
- [ ] Lexer recognizes tokens
- [ ] Parser constructs AST nodes
- [ ] Generator outputs LaTeX
- [ ] All test cases pass
- [ ] Error handling correct
- [ ] Position tracking accurate
- [ ] Precedence rules correct (Features 4-6)

## Go Structural Recommendations

### Type Hierarchy
```go
// Interface for all expressions
type Expr interface {
    exprNode()
}

// Number expression
type Number struct {
    Value  string
    Line   int
    Column int
}

func (n *Number) exprNode() {}

// Binary operation expression
type BinaryOp struct {
    Operator string
    Left     Expr
    Right    Expr
    Line     int
    Column   int
}

func (b *BinaryOp) exprNode() {}
```

### Package Organization
Recommended Go package structure:

```
rpn2tex/
├── token.go          # TokenType, Token
├── ast.go            # Expr, Number, BinaryOp
├── errors.go         # ErrorFormatter
├── lexer.go          # Lexer, LexerError
├── parser.go         # Parser, ParserError
├── latex.go          # LaTeXGenerator
├── main.go           # CLI entry point
└── _test.go files    # Tests for each module
```

### Key Implementation Patterns

1. **Token matching:** Character classification (isDigit, isPunct, etc.)
2. **Number scanning:** Accumulate digits, optional decimal point
3. **Stack-based parsing:** Push numbers, pop operands for operators
4. **Visitor pattern:** Type assertion + switch for node traversal
5. **Precedence recursion:** Check precedence before adding parentheses

---

# Appendix: Python-to-Go Type Mappings

| Python | Go | Notes |
|--------|-----|-------|
| `Enum` | `const` with `iota` | Use named constants instead |
| `@dataclass(frozen=True)` | `struct` | No direct equivalent; use const receiver methods |
| `\|` (union type) | `interface{}` or explicit type switch | Go uses type assertions |
| `list[T]` | `[]T` | Slice type |
| `dict[K, V]` | `map[K]V` | Map type |
| `str` | `string` | Immutable by default |
| `@singledispatchmethod` | Type switch or type assertion | Go doesn't have runtime dispatch like Python |
| `isinstance()` | Type assertion with `,ok` | `v, ok := x.(*Type)` |
| `raise Exception` | `error` return value | Go's error handling idiom |

---

*This specification was generated as part of Phase 1 of the rpn2tex migration process. It serves as the definitive guide for implementing all features in Go while maintaining complete behavioral compatibility with the Python reference implementation.*
