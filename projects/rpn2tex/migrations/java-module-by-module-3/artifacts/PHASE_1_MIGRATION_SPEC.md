# PHASE 1: Java Migration Specification for rpn2tex

**Document Version**: 1.0
**Generated**: 2025-12-30
**Target Language**: Java
**Source Implementation**: Python
**Status**: Comprehensive Module Analysis

---

## Table of Contents

1. [I/O Contract (Phase 0)](#io-contract-phase-0)
2. [Overview](#overview)
3. [Module-by-Module Specification](#module-by-module-specification)
4. [Dependency Graph](#dependency-graph)
5. [Package Structure](#package-structure)
6. [Python to Java Type Mappings](#python-to-java-type-mappings)
7. [Common Patterns](#common-patterns)
8. [Testing Strategy](#testing-strategy)
9. [Migration Order](#migration-order)

---

## I/O Contract (Phase 0)

### Test Cases Summary

**Successful Cases (Exit Code 0)**: 18 tests
- Basic arithmetic operations: addition, subtraction, multiplication, division
- Floating-point numbers
- Complex expressions with multiple operators
- Proper parenthesization when needed

**Error Cases (Exit Code 1)**: 3 tests
- Exponentiation operator (^) not supported in base implementation
- Character validation at lexer stage

### Detailed Test Cases

| # | Input | Output | Exit Code | Notes |
|---|-------|--------|-----------|-------|
| 1 | `5 3 +` | `$5 + 3$` | 0 | Basic addition |
| 2 | `5 3 -` | `$5 - 3$` | 0 | Basic subtraction |
| 3 | `4 7 *` | `$4 \times 7$` | 0 | Basic multiplication |
| 4 | `10 2 /` | `$10 \div 2$` | 0 | Basic division |
| 5 | `2 3 ^` | Error (lexer) | 1 | Unsupported operator |
| 6 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | 0 | Precedence with parens |
| 7 | `5 3 * 2 +` | `$5 \times 3 + 2$` | 0 | Precedence no parens |
| 8 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | 0 | Left-associative ops |
| 9 | `5 3 - 2 -` | `$5 - 3 - 2$` | 0 | Chained subtraction |
| 10 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | 0 | Chained division |
| 11 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | 0 | Chained addition |
| 12 | `2 3 4 * +` | `$2 + 3 \times 4$` | 0 | Mixed ops, no parens |
| 13 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | 0 | Addition parenthesized |
| 14 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | 0 | Right operand parens |
| 15 | `2 3 * 4 +` | `$2 \times 3 + 4$` | 0 | Mult then add |
| 16 | `2 3 ^ 4 *` | Error (lexer) | 1 | Unsupported operator |
| 17 | `2 3 4 ^ ^` | Error (lexer) | 1 | Unsupported operator |
| 18 | `3.14 2 *` | `$3.14 \times 2$` | 0 | Decimal numbers |
| 19 | `1.5 0.5 +` | `$1.5 + 0.5$` | 0 | Decimal addition |
| 20 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | 0 | Double parenthesized |
| 21 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | 0 | Complex mixed ops |

### LaTeX Output Format Details

**Operator Symbols**:
- Addition: `+`
- Subtraction: `-`
- Multiplication: `\times`
- Division: `\div`
- Exponentiation: Not supported (raises lexer error)

**Formatting Requirements**:
- All output wrapped in inline math mode: `$...$`
- Numbers preserve input format (integers as integers, decimals as decimals)
- Parentheses inserted only when needed for precedence
- Space-separated in LaTeX output for readability

**Parenthesization Rules**:
- Inserted when lower-precedence operation is operand to higher-precedence operation
- NOT inserted when operators have same precedence and properly ordered
- NOT inserted when higher-precedence operation is operand to lower-precedence operation

### Error Handling Requirements

**Lexer Error Format**:
```
Error: Unexpected character '<char>'
<line> | <input>
        | <pointer to error column>
```

**Supported Characters**:
- Digits: `0-9`
- Decimal point: `.`
- Whitespace: space, tab, newline
- Operators: `+`, `-`, `*`, `/`

**Validation Checklist**:
- [ ] All 18 success cases produce EXACT LaTeX output
- [ ] All 3 error cases with `^` operator produce lexer errors with exit code 1
- [ ] Error messages include character location information
- [ ] Decimal numbers preserved in output (not converted to integers)
- [ ] LaTeX symbols correct: `\times` for multiplication, `\div` for division
- [ ] Parentheses inserted only when necessary
- [ ] Output wrapped in `$...$` for inline math mode
- [ ] No extra whitespace at beginning/end of output
- [ ] Exit code 0 on success, 1 on error

---

## Overview

The rpn2tex project is a RPN (Reverse Polish Notation) expression parser that converts mathematical expressions to LaTeX format. It consists of 7 modules organized in a classic compiler pipeline:

```
Input → Lexer → Parser → AST → LaTeX Generator → Output
```

### Key Design Principles

1. **Immutability**: All data structures are immutable (frozen dataclasses in Python)
2. **Position Tracking**: Every token and AST node tracks source location for error reporting
3. **Visitor Pattern**: LaTeX generation uses single-dispatch visitor pattern
4. **Stack-Based Parsing**: RPN parser uses stack instead of recursive descent
5. **Error Context**: Errors include formatted source context with caret pointers

---

## Module-by-Module Specification

### Module 1: tokens.py

**Purpose**: Define token types and the Token data structure representing lexical elements.

**Responsibilities**:
- Define TokenType enum with all supported token types
- Provide immutable Token dataclass with position tracking
- Document token semantics

**Public API**:

```python
class TokenType(Enum):
    """Token type enumeration"""
    NUMBER = auto()      # Numeric literals
    PLUS = auto()        # Addition operator
    MINUS = auto()       # Subtraction operator
    MULT = auto()        # Multiplication operator
    DIV = auto()         # Division operator
    EOF = auto()         # End of file marker

@dataclass(frozen=True)
class Token:
    """Lexical token with position tracking"""
    type: TokenType       # Token type
    value: str           # String value
    line: int            # 1-based line number
    column: int          # 1-based column number

    def __repr__(self) -> str
        # Returns: "Token(<TYPE>, '<value>', <line>:<column>)"
```

**Data Structures**:
- `TokenType`: Enum with 6 members (NUMBER, PLUS, MINUS, MULT, DIV, EOF)
- `Token`: Immutable record with 4 fields (type, value, line, column)

**Dependencies**:
- Internal: None
- External: `dataclasses`, `enum`

**Python-Specific Patterns**:
- `@dataclass(frozen=True)`: Creates immutable class with auto-generated __init__, __repr__, etc.
- `Enum` with `auto()`: Creates auto-incremented enum values
- Type hints with forward references: `from __future__ import annotations`

**Java Migration Strategy**:
- `TokenType` → Java `enum TokenType` with explicit values
- `Token` → Java immutable class with private final fields and constructor
- Implement `Comparable` or `equals()` / `hashCode()` for comparison
- Use records if Java 16+ (sealed records preferred)
- Create builder pattern for construction if needed

**Critical Behaviors**:
- Position tracking must be accurate (1-based indexing in both line and column)
- Token value preserves exact input (including negative sign for numbers)
- Immutability prevents accidental mutation in collections
- EOF token has empty string value

**Java Implementation Notes**:
```java
// Preferred approach with record (Java 16+)
public record Token(
    TokenType type,
    String value,
    int line,
    int column
) {
    public Token {
        if (type == null) throw new NullPointerException("type");
        if (value == null) throw new NullPointerException("value");
    }

    @Override
    public String toString() {
        return "Token(" + type.name() + ", \"" + value + "\", " + line + ":" + column + ")";
    }
}

// Fallback for Java < 16
public final class Token {
    public final TokenType type;
    public final String value;
    public final int line;
    public final int column;

    public Token(TokenType type, String value, int line, int column) {
        this.type = Objects.requireNonNull(type);
        this.value = Objects.requireNonNull(value);
        this.line = line;
        this.column = column;
    }

    @Override
    public String toString() {
        return "Token(" + type.name() + ", \"" + value + "\", " + line + ":" + column + ")";
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Token)) return false;
        Token token = (Token) o;
        return line == token.line &&
               column == token.column &&
               type == token.type &&
               value.equals(token.value);
    }

    @Override
    public int hashCode() {
        return Objects.hash(type, value, line, column);
    }
}
```

---

### Module 2: ast_nodes.py

**Purpose**: Define AST node types representing parsed expression structure.

**Responsibilities**:
- Define base ASTNode class with position tracking
- Provide concrete node types for expressions
- Maintain immutability across AST tree
- Define Expr type alias for all valid expression types

**Public API**:

```python
@dataclass(frozen=True)
class ASTNode:
    """Base class for all AST nodes"""
    line: int        # 1-based line number
    column: int      # 1-based column number

@dataclass(frozen=True)
class Number(ASTNode):
    """Numeric literal node"""
    value: str       # String representation of number

@dataclass(frozen=True)
class BinaryOp(ASTNode):
    """Binary operation node"""
    operator: str    # Operator: "+", "-", "*", "/"
    left: Expr       # Left operand
    right: Expr      # Right operand

# Type alias for all expression types
Expr = Number | BinaryOp
```

**Data Structures**:
- `ASTNode`: Base class with position information
- `Number`: Literal node containing string value
- `BinaryOp`: Operation node with two child expressions
- `Expr`: Union type representing all valid expressions

**Dependencies**:
- Internal: None
- External: `dataclasses`

**Python-Specific Patterns**:
- Frozen dataclasses: Create immutable structures with automatic __init__, __repr__, __eq__
- Type alias using union operator `|` (Python 3.10+): `Expr = Number | BinaryOp`
- Field-based equality: Dataclasses auto-generate proper __eq__ based on fields
- Recursive data structure: BinaryOp contains Expr children

**Java Migration Strategy**:
- `ASTNode` → Abstract base class with protected fields
- `Number` → Concrete class extending ASTNode
- `BinaryOp` → Concrete class extending ASTNode
- `Expr` → Java sealed interface or abstract class (Java 16+)
- Use immutable pattern with final fields and constructor validation
- Implement visitor pattern interface for traversal

**Critical Behaviors**:
- Position tracking in every node enables precise error reporting
- Immutability prevents accidental AST corruption
- Type alias enables type-safe operations on all expression types
- Recursive structure naturally represents expression trees
- String values in Number preserve original input format

**Java Implementation Notes**:
```java
// Abstract base class (required)
public abstract class ASTNode {
    public final int line;
    public final int column;

    protected ASTNode(int line, int column) {
        this.line = line;
        this.column = column;
    }
}

// Concrete number node
public final class Number extends ASTNode {
    public final String value;

    public Number(int line, int column, String value) {
        super(line, column);
        this.value = Objects.requireNonNull(value);
    }

    @Override
    public String toString() {
        return "Number(" + line + ", " + column + ", \"" + value + "\")";
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Number)) return false;
        Number number = (Number) o;
        return line == number.line &&
               column == number.column &&
               value.equals(number.value);
    }

    @Override
    public int hashCode() {
        return Objects.hash(line, column, value);
    }
}

// Concrete binary operation node
public final class BinaryOp extends ASTNode {
    public final String operator;
    public final Expr left;
    public final Expr right;

    public BinaryOp(int line, int column, String operator, Expr left, Expr right) {
        super(line, column);
        this.operator = Objects.requireNonNull(operator);
        this.left = Objects.requireNonNull(left);
        this.right = Objects.requireNonNull(right);
    }

    @Override
    public String toString() {
        return "BinaryOp(" + line + ", " + column + ", \"" + operator + "\", " + left + ", " + right + ")";
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof BinaryOp)) return false;
        BinaryOp binaryOp = (BinaryOp) o;
        return line == binaryOp.line &&
               column == binaryOp.column &&
               operator.equals(binaryOp.operator) &&
               left.equals(binaryOp.left) &&
               right.equals(binaryOp.right);
    }

    @Override
    public int hashCode() {
        return Objects.hash(line, column, operator, left, right);
    }
}

// Type alias (as an interface for polymorphism)
public sealed interface Expr permits Number, BinaryOp {
    int getLine();
    int getColumn();
}
```

---

### Module 3: errors.py

**Purpose**: Format parse and lexer errors with source context for user-friendly output.

**Responsibilities**:
- Accept source text and track line breaks
- Format error messages with context and caret positioning
- Calculate line/column information for display
- Provide gcc/rustc-style error output

**Public API**:

```python
class ErrorFormatter:
    """Error formatter with source context"""

    def __init__(self, source: str) -> None:
        """Initialize with source text"""
        # Stores source and splits into lines

    def format_error(
        self,
        message: str,
        line: int,
        column: int,
        *,
        context_lines: int = 1,
    ) -> str:
        """Format error with source context

        Returns formatted error string with:
        - Error message header
        - Source line(s) with line numbers
        - Caret pointing to error column
        """

    def _get_context(
        self,
        line: int,
        column: int,
        context_lines: int,
    ) -> str:
        """Extract context around error position (private)"""
```

**Key Methods**:

1. `__init__(source: str)`:
   - Stores complete source text
   - Splits source into lines for quick access
   - Enables efficient context extraction

2. `format_error(message, line, column, context_lines=1)`:
   - Formats error message with "Error: " prefix
   - Adds blank line
   - Appends formatted context
   - Returns joined string

3. `_get_context(line, column, context_lines)`:
   - Converts 1-based indices to 0-based for list access
   - Clamps range to valid line indices
   - Calculates line number width for alignment
   - Formats each context line with number
   - Adds caret line at error position
   - Positions caret at column-1 (0-based offset)

**Dependencies**:
- Internal: None
- External: Standard library only (string operations)

**Python-Specific Patterns**:
- String formatting: f-strings with field width specifiers (e.g., `f"{line_num:>{num_width}}"`)
- List comprehensions: Not used; instead uses loop
- String joining: `"\n".join(list_of_strings)`
- Keyword-only arguments: `format_error(..., *, context_lines=1)`

**Java Migration Strategy**:
- Immutable class with final fields
- Cache lines array in constructor
- Use StringBuilder for efficient string building
- Implement String.format() or StringBuilder for line formatting
- Calculate layout with Integer.toString().length()

**Critical Behaviors**:
- 1-based indexing: User-facing line/column are 1-based
- 0-based internal indexing: Python lists use 0-based indices
- Caret positioning: Column adjusted to 0-based for spaces calculation
- Line clamping: Context range bounded by valid line indices
- Width calculation: Line number width determines prefix spacing
- Empty line handling: Missing lines treated as empty strings

**Error Format Example**:
```
Error: Unexpected character '^'
1 | 2 3 ^ 4 *
        ^
```

**Java Implementation Notes**:
```java
public final class ErrorFormatter {
    private final String source;
    private final String[] lines;

    public ErrorFormatter(String source) {
        this.source = Objects.requireNonNull(source);
        this.lines = source.split("\n", -1); // Keep trailing empty
    }

    public String formatError(String message, int line, int column) {
        return formatError(message, line, column, 1);
    }

    public String formatError(String message, int line, int column, int contextLines) {
        StringBuilder sb = new StringBuilder();
        sb.append("Error: ").append(message).append("\n\n");
        sb.append(getContext(line, column, contextLines));
        return sb.toString();
    }

    private String getContext(int line, int column, int contextLines) {
        // Convert 1-based to 0-based
        int errorIdx = line - 1;

        // Clamp range
        int startIdx = Math.max(0, errorIdx - contextLines);
        int endIdx = Math.min(lines.length, errorIdx + contextLines + 1);

        // Calculate width
        int numWidth = String.valueOf(endIdx).length();

        StringBuilder sb = new StringBuilder();
        for (int idx = startIdx; idx < endIdx; idx++) {
            int lineNum = idx + 1;
            String lineContent = idx < lines.length ? lines[idx] : "";

            // Format line with number
            String prefix = String.format("%" + numWidth + "d | ", lineNum);
            sb.append(prefix).append(lineContent).append("\n");

            // Add caret on error line
            if (idx == errorIdx) {
                String caretPrefix = " ".repeat(numWidth) + " | ";
                int caretPos = Math.max(0, column - 1);
                sb.append(caretPrefix).append(" ".repeat(caretPos)).append("^\n");
            }
        }

        // Remove trailing newline if present
        String result = sb.toString();
        if (result.endsWith("\n")) {
            result = result.substring(0, result.length() - 1);
        }
        return result;
    }
}
```

---

### Module 4: lexer.py

**Purpose**: Convert input text into token stream for parsing.

**Responsibilities**:
- Scan input character-by-character
- Recognize numbers (integers and decimals)
- Recognize operators (+, -, *, /)
- Track position (line, column) for error reporting
- Handle whitespace as delimiter
- Validate characters and report unknown characters

**Public API**:

```python
class LexerError(Exception):
    """Exception raised by lexer"""
    message: str      # Error message
    line: int         # 1-based line number
    column: int       # 1-based column number

    def __init__(self, message: str, line: int, column: int) -> None
        # Stores attributes and formats parent message

class Lexer:
    """Tokenizer for RPN expressions"""

    def __init__(self, text: str) -> None
        # Initialize with input text
        # Sets pos=0, line=1, column=1

    def tokenize(self) -> list[Token]:
        """Scan entire input and return token list

        Returns:
            List of tokens including EOF token

        Raises:
            LexerError: If unknown character encountered
        """

    # Private methods (implementation details)
    def _at_end(self) -> bool
    def _peek(self) -> str
    def _advance(self) -> str
    def _skip_whitespace(self) -> None
    def _scan_token(self) -> Token
    def _scan_number(self, prefix: str, start_line: int, start_column: int) -> Token
```

**Key Methods**:

1. `__init__(text)`:
   - Stores input text
   - Initializes position at 0
   - Initializes line/column tracking at 1

2. `tokenize()`:
   - Main entry point
   - Loops until EOF
   - Skips whitespace at start of each iteration
   - Calls _scan_token() to get next token
   - Appends EOF token at end
   - Raises LexerError on invalid character

3. `_scan_token()`:
   - Checks for operators: +, -, *, /
   - Handles negative numbers after -
   - Checks for digits at start
   - Raises LexerError for unknown character
   - Returns appropriate Token

4. `_scan_number(prefix, start_line, start_column)`:
   - Processes integer digits
   - Processes optional decimal point and fractional digits
   - Returns NUMBER token with accumulated value
   - Preserves prefix (e.g., "-" for negative numbers)

5. `_peek()`, `_advance()`, `_at_end()`:
   - Low-level character scanning
   - _advance() tracks line/column
   - Newlines reset column to 1 and increment line

**Dependencies**:
- Internal: tokens (Token, TokenType)
- External: None

**Python-Specific Patterns**:
- Exception class attributes: `message`, `line`, `column` stored directly on exception
- Duck typing in _scan_token: Checks type with isinstance() after value checked
- String methods: `isdigit()`, `in` operator for character sets
- String methods: implicit character-by-character iteration via indexing

**Java Migration Strategy**:
- `LexerError` extends `Exception` with custom constructor
- `Lexer` class with private state fields
- Use `charAt()` instead of direct indexing
- Explicit character checks instead of `isdigit()`
- CharacterTester class or static methods for character classification
- Immutable Token creation at each scan step

**Critical Behaviors**:
- Whitespace is delimiter; multiple whitespaces treated as single separator
- Negative numbers: "-" followed immediately by digit is number; "-" followed by space/operator is minus
- Decimal parsing: Allows ".5" but parser/LaTeX gen must handle correctly
- Position tracking: Column increments for each character; line resets on newline
- Error position: Reported at character that triggered error
- EOF token: Added even if input empty; has empty value and current position

**Edge Cases**:
- Empty input → Returns just EOF token
- Leading/trailing whitespace → Skipped appropriately
- Multiple operators in sequence → Each becomes separate token
- Decimal without integer: ".5" → Valid NUMBER token
- Decimal with multiple points: "1.2.3" → Parsed as "1.2" and ".3" (parser error)
- Negative after operator: "2 -3" → Parsed as NUMBER("-3") not MINUS and NUMBER("3")

**Java Implementation Notes**:
```java
public class LexerError extends Exception {
    public final String message;
    public final int line;
    public final int column;

    public LexerError(String message, int line, int column) {
        super("Line " + line + ", column " + column + ": " + message);
        this.message = Objects.requireNonNull(message);
        this.line = line;
        this.column = column;
    }
}

public final class Lexer {
    private final String text;
    private int pos;
    private int line;
    private int column;

    public Lexer(String text) {
        this.text = Objects.requireNonNull(text);
        this.pos = 0;
        this.line = 1;
        this.column = 1;
    }

    public List<Token> tokenize() throws LexerError {
        List<Token> tokens = new ArrayList<>();

        while (!isAtEnd()) {
            skipWhitespace();
            if (isAtEnd()) break;
            tokens.add(scanToken());
        }

        tokens.add(new Token(TokenType.EOF, "", line, column));
        return Collections.unmodifiableList(tokens);
    }

    private boolean isAtEnd() {
        return pos >= text.length();
    }

    private char peek() {
        if (isAtEnd()) return '\0';
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

    private void skipWhitespace() {
        while (!isAtEnd() && Character.isWhitespace(peek())) {
            advance();
        }
    }

    private Token scanToken() throws LexerError {
        int startLine = line;
        int startColumn = column;
        char c = peek();

        switch (c) {
            case '+':
                advance();
                return new Token(TokenType.PLUS, "+", startLine, startColumn);
            case '*':
                advance();
                return new Token(TokenType.MULT, "*", startLine, startColumn);
            case '/':
                advance();
                return new Token(TokenType.DIV, "/", startLine, startColumn);
            case '-':
                advance();
                if (!isAtEnd() && Character.isDigit(peek())) {
                    return scanNumber("-", startLine, startColumn);
                }
                return new Token(TokenType.MINUS, "-", startLine, startColumn);
            default:
                if (Character.isDigit(c)) {
                    return scanNumber("", startLine, startColumn);
                }
                throw new LexerError("Unexpected character '" + c + "'", startLine, startColumn);
        }
    }

    private Token scanNumber(String prefix, int startLine, int startColumn) {
        StringBuilder sb = new StringBuilder(prefix);

        // Integer part
        while (!isAtEnd() && Character.isDigit(peek())) {
            sb.append(advance());
        }

        // Decimal part
        if (!isAtEnd() && peek() == '.') {
            sb.append(advance()); // consume '.'
            while (!isAtEnd() && Character.isDigit(peek())) {
                sb.append(advance());
            }
        }

        return new Token(TokenType.NUMBER, sb.toString(), startLine, startColumn);
    }
}
```

---

### Module 5: parser.py

**Purpose**: Convert token stream into Abstract Syntax Tree (AST).

**Responsibilities**:
- Implement stack-based RPN parsing algorithm
- Accumulate operands on stack
- Build operation nodes when operators encountered
- Validate proper RPN structure
- Report parse errors with token context

**Public API**:

```python
class ParserError(Exception):
    """Exception raised by parser"""
    message: str      # Error message
    token: Token      # Token where error occurred

    def __init__(self, message: str, token: Token) -> None
        # Stores attributes and formats parent message

class Parser:
    """Stack-based RPN parser"""

    def __init__(self, tokens: list[Token]) -> None
        # Initialize with token list
        # Sets pos=0

    def parse(self) -> Expr:
        """Parse token stream into AST

        Returns:
            Root expression node

        Raises:
            ParserError: If RPN is invalid

        Algorithm:
            - For each NUMBER: push Number node onto stack
            - For each operator: pop 2 operands, create BinaryOp, push result
            - At EOF: stack must contain exactly 1 element
        """

    # Private methods (implementation details)
    def _current(self) -> Token
    def _at_end(self) -> bool
    def _advance(self) -> Token
```

**Key Methods**:

1. `__init__(tokens)`:
   - Stores token list
   - Initializes position at 0

2. `parse()`:
   - Initializes empty stack
   - Iterates through tokens
   - For NUMBER: creates Number node, pushes to stack
   - For operators: validates stack has 2+ items, pops right then left, creates BinaryOp, pushes result
   - At EOF: validates stack has exactly 1 element
   - Returns top of stack
   - Raises ParserError if:
     - Operator encountered with < 2 operands
     - Empty expression
     - Multiple values on stack at end

3. `_current()`, `_at_end()`, `_advance()`:
   - Low-level token stream handling
   - _at_end() checks if current token is EOF type
   - _advance() returns current and increments position

**Dependencies**:
- Internal: ast_nodes (BinaryOp, Expr, Number), tokens (Token, TokenType)
- External: None

**Python-Specific Patterns**:
- Stack implemented as Python list with pop/append
- Token type checking with `in` operator on tuple: `token.type in (PLUS, MINUS, ...)`
- Dictionary mapping: `op_map[token.type]` maps token to operator string
- Error with token context: Token stored in exception for error formatting

**Java Migration Strategy**:
- Parser class with private state fields
- Use `java.util.Stack<Expr>` or `java.util.Deque<Expr>` for stack
- Switch statement for token type matching
- Map<TokenType, String> for operator mapping
- Immutable AST node creation
- Clear exception hierarchy

**Critical Behaviors**:
- Stack-based algorithm: All operands accumulated before operators processed
- Right operand popped first: Correct left-to-right processing (right = stack.pop(), left = stack.pop())
- Position tracking: Operators create nodes at operator position, not operand positions
- Error validation: Three error conditions checked at end (empty, multiple operands, insufficient operands)
- EOF requirement: Input must end with EOF token for proper termination

**Error Conditions**:
1. Insufficient operands: Operator with < 2 items on stack
   - Message: "Operator '<op>' requires two operands"
   - Position: Current operator token
2. Empty expression: Stack empty at EOF
   - Message: "Empty expression"
   - Position: EOF token
3. Extra operands: Stack has > 1 element at EOF
   - Message: "Invalid RPN: <count> values remain on stack (missing operators?)"
   - Position: EOF token

**Java Implementation Notes**:
```java
public class ParserError extends Exception {
    public final String message;
    public final Token token;

    public ParserError(String message, Token token) {
        super(message + " at line " + token.line + ", column " + token.column);
        this.message = Objects.requireNonNull(message);
        this.token = Objects.requireNonNull(token);
    }
}

public final class Parser {
    private final List<Token> tokens;
    private int pos;

    public Parser(List<Token> tokens) {
        this.tokens = Objects.requireNonNull(tokens);
        this.pos = 0;
    }

    public Expr parse() throws ParserError {
        Deque<Expr> stack = new ArrayDeque<>();

        while (!isAtEnd()) {
            Token token = current();

            if (token.type() == TokenType.NUMBER) {
                Number numNode = new Number(token.line(), token.column(), token.value());
                stack.push(numNode);
                advance();
            } else if (token.type() == TokenType.PLUS ||
                       token.type() == TokenType.MINUS ||
                       token.type() == TokenType.MULT ||
                       token.type() == TokenType.DIV) {
                if (stack.size() < 2) {
                    throw new ParserError("Operator '" + token.value() + "' requires two operands", token);
                }

                Expr right = stack.pop();
                Expr left = stack.pop();

                String op = switch (token.type()) {
                    case PLUS -> "+";
                    case MINUS -> "-";
                    case MULT -> "*";
                    case DIV -> "/";
                    default -> throw new AssertionError();
                };

                Expr opNode = new BinaryOp(token.line(), token.column(), op, left, right);
                stack.push(opNode);
                advance();
            } else if (token.type() == TokenType.EOF) {
                break;
            } else {
                throw new ParserError("Unexpected token '" + token.value() + "'", token);
            }
        }

        if (stack.isEmpty()) {
            throw new ParserError("Empty expression", tokens.get(tokens.size() - 1));
        }

        if (stack.size() > 1) {
            throw new ParserError(
                "Invalid RPN: " + stack.size() + " values remain on stack (missing operators?)",
                tokens.get(tokens.size() - 1)
            );
        }

        return stack.pop();
    }

    private Token current() {
        return tokens.get(pos);
    }

    private boolean isAtEnd() {
        return current().type() == TokenType.EOF;
    }

    private Token advance() {
        Token token = current();
        if (!isAtEnd()) {
            pos++;
        }
        return token;
    }
}
```

---

### Module 6: latex_gen.py

**Purpose**: Convert AST into LaTeX mathematical notation with proper precedence handling.

**Responsibilities**:
- Traverse AST using visitor pattern
- Generate LaTeX strings for each node type
- Manage operator precedence
- Insert parentheses only when needed
- Handle left-associativity rules

**Public API**:

```python
class LaTeXGenerator:
    """AST to LaTeX converter"""

    # Class constants
    BINARY_OPS: ClassVar[dict[str, str]]  # Maps operators to LaTeX symbols
    PRECEDENCE: ClassVar[dict[str, int]]  # Operator precedence levels

    def generate(self, ast: Expr) -> str:
        """Convert AST to LaTeX

        Args:
            ast: Root expression node

        Returns:
            LaTeX string wrapped in $...$ math mode

        Example:
            >>> ast = BinaryOp(1, 1, "+", Number(1, 1, "5"), Number(1, 3, "3"))
            >>> LaTeXGenerator().generate(ast)
            '$5 + 3$'
        """

    @singledispatchmethod
    def _visit(self, node: Expr) -> str:
        """Visit AST node (dispatcher)

        Internal method implementing visitor pattern via @singledispatchmethod.
        Raises NotImplementedError for unhandled node types.
        """

    @_visit.register
    def _visit_number(self, node: Number) -> str:
        """Generate LaTeX for Number node"""

    @_visit.register
    def _visit_binary_op(self, node: BinaryOp) -> str:
        """Generate LaTeX for BinaryOp node

        Handles precedence and parenthesization logic.
        """

    def _needs_parens(
        self,
        child: Expr,
        parent_precedence: int,
        *,
        is_right: bool,
    ) -> bool:
        """Determine if child needs parentheses

        Private method implementing precedence rules:
        1. Lower precedence → parenthesize
        2. Equal precedence + right operand + non-associative → parenthesize
        """
```

**Class Constants**:

```python
BINARY_OPS = {
    "+": "+",           # Addition (no change)
    "-": "-",           # Subtraction (no change)
    "*": r"\times",     # Multiplication (LaTeX symbol)
    "/": r"\div",       # Division (LaTeX symbol)
}

PRECEDENCE = {
    "+": 1,             # Addition: level 1
    "-": 1,             # Subtraction: level 1
    "*": 2,             # Multiplication: level 2
    "/": 2,             # Division: level 2
}
```

**Key Methods**:

1. `generate(ast)`:
   - Calls _visit() to generate content
   - Wraps result in $...$ delimiters
   - Returns final LaTeX string

2. `_visit(node)` (dispatcher):
   - Uses @singledispatchmethod for type dispatch
   - Base implementation raises NotImplementedError

3. `_visit_number(node)`:
   - Returns node.value as-is
   - Preserves integer/decimal formatting

4. `_visit_binary_op(node)`:
   - Gets LaTeX operator symbol
   - Gets operator precedence
   - Generates left operand recursively
   - Adds parentheses if _needs_parens() returns True
   - Generates right operand recursively
   - Adds parentheses if _needs_parens() returns True
   - Joins: `{left} {op_latex} {right}`

5. `_needs_parens(child, parent_precedence, is_right)`:
   - Returns False if child is not BinaryOp
   - Returns True if child precedence < parent precedence
   - Returns True if equal precedence AND is_right AND operator in ("-", "/")
   - Returns False otherwise

**Dependencies**:
- Internal: ast_nodes (BinaryOp, Expr, Number)
- External: `functools.singledispatchmethod`, `typing.ClassVar`

**Python-Specific Patterns**:
- `@singledispatchmethod`: Decorator enabling type-based method dispatch
- `@_visit.register`: Registers implementations for specific types
- `isinstance()` type checking in _needs_parens
- f-strings and string formatting
- Type hints with forward references

**Java Migration Strategy**:
- Visitor pattern: Use traditional visitor interface or switch on type
- Class constants: Static final Map and Map for BINARY_OPS and PRECEDENCE
- Method dispatch: Use instanceof or visitor interface pattern
- String building: Use StringBuilder for efficiency
- Immutable maps: Use Collections.unmodifiableMap() or Map.of()

**Critical Behaviors**:
- Precedence levels: * and / (level 2) higher than + and - (level 1)
- Left-associativity: - and / are left-associative; need parens on right operand with equal precedence
- Parenthesization logic:
  - (a + b) * c needs parens (precedence: 1 < 2)
  - a * b + c no parens (precedence: 2 > 1)
  - a - (b - c) needs parens (left-associativity)
  - a - b - c no parens (left-associative, properly ordered)
- Number preservation: Numbers output exactly as parsed (no rounding/conversion)
- Spacing: Single space around operators and inside parentheses

**Precedence Examples**:
- `5 3 +` → `$5 + 3$` (no parens)
- `5 3 + 2 *` → `$( 5 + 3 ) \times 2$` (parens: 1 < 2)
- `5 3 * 2 +` → `$5 \times 3 + 2$` (no parens: 2 > 1)
- `5 3 - 2 -` → `$5 - 3 - 2$` (no parens: left-associative)
- `5 3 2 - -` → `$5 - ( 3 - 2 )$` (parens: right operand of -)

**Java Implementation Notes**:
```java
public final class LaTeXGenerator {
    static final Map<String, String> BINARY_OPS = Collections.unmodifiableMap(
        Map.ofEntries(
            Map.entry("+", "+"),
            Map.entry("-", "-"),
            Map.entry("*", "\\times"),
            Map.entry("/", "\\div")
        )
    );

    static final Map<String, Integer> PRECEDENCE = Collections.unmodifiableMap(
        Map.ofEntries(
            Map.entry("+", 1),
            Map.entry("-", 1),
            Map.entry("*", 2),
            Map.entry("/", 2)
        )
    );

    public String generate(Expr ast) {
        String content = visit(ast);
        return "$" + content + "$";
    }

    private String visit(Expr node) {
        if (node instanceof Number num) {
            return visitNumber(num);
        } else if (node instanceof BinaryOp binOp) {
            return visitBinaryOp(binOp);
        } else {
            throw new AssertionError("Unknown node type: " + node.getClass().getName());
        }
    }

    private String visitNumber(Number node) {
        return node.value;
    }

    private String visitBinaryOp(BinaryOp node) {
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

    private boolean needsParens(Expr child, int parentPrecedence, boolean isRight) {
        if (!(child instanceof BinaryOp binOp)) {
            return false;
        }

        int childPrecedence = PRECEDENCE.get(binOp.operator);

        if (childPrecedence < parentPrecedence) {
            return true;
        }

        return childPrecedence == parentPrecedence &&
               isRight &&
               (binOp.operator.equals("-") || binOp.operator.equals("/"));
    }
}
```

---

### Module 7: cli.py

**Purpose**: Command-line interface orchestrating the complete pipeline.

**Responsibilities**:
- Parse command-line arguments
- Read input from file or stdin
- Orchestrate lexer → parser → generator pipeline
- Format and output results
- Handle all error cases with appropriate exit codes
- Print errors to stderr, output to stdout

**Public API**:

```python
def main() -> int:
    """Main CLI entry point

    Returns:
        Exit code: 0 for success, 1 for any error

    Pipeline:
        1. Parse arguments
        2. Read input (file or stdin)
        3. Tokenize (Lexer)
        4. Parse (Parser)
        5. Generate (LaTeXGenerator)
        6. Format error if needed
        7. Write output (file or stdout)
    """

if __name__ == "__main__":
    sys.exit(main())
```

**Argument Parsing**:
```python
parser = argparse.ArgumentParser(
    description="Convert RPN expressions to LaTeX math mode",
    prog="rpn2tex",
    epilog="Example: rpn2tex input.rpn -o output.tex",
)
parser.add_argument(
    "input",
    type=str,
    help="Input RPN file (use '-' for stdin)",
)
parser.add_argument(
    "-o",
    "--output",
    type=Path,
    help="Output LaTeX file (default: stdout)",
)
```

**Error Handling**:

1. File reading errors:
   - FileNotFoundError → Exit 1, print "Error: Input file not found: ..."
   - PermissionError → Exit 1, print "Error: Permission denied reading: ..."
   - IsADirectoryError → Exit 1, print "Error: Expected a file, got a directory: ..."

2. Processing errors:
   - LexerError → Format with ErrorFormatter, print to stderr, exit 1
   - ParserError → Format with ErrorFormatter, print to stderr, exit 1

3. Output errors:
   - PermissionError → Exit 1, print "Error: Permission denied writing: ..."
   - IsADirectoryError → Exit 1, print "Error: Cannot write to directory: ..."

**Dependencies**:
- Internal: errors (ErrorFormatter), latex_gen (LaTeXGenerator), lexer (Lexer, LexerError), parser (Parser, ParserError)
- External: argparse, sys, pathlib

**Python-Specific Patterns**:
- argparse: Argument parser with subcommands and options
- Path.read_text(): File reading into string
- Path.write_text(): File writing from string
- sys.stdin.read(): Read all stdin into string
- sys.stderr: Error output stream
- Exception type matching: `except FileNotFoundError:` catches specific exception type
- Ternary/conditional in Python: Used for input selection

**Java Migration Strategy**:
- Static main() method with System.exit()
- Apache Commons CLI or picocli for argument parsing
- Files.readString() / Files.writeString() for file operations
- System.in.read() wrapped in scanner for stdin
- System.err for error output
- try-catch blocks for exception handling
- Enum or constants for exit codes

**Critical Behaviors**:
- Argument "-" means stdin, not a filename
- Input-less argument required (positional argument)
- -o/--output is optional; default is stdout
- Errors printed to stderr; output to stdout
- LaTeX output followed by newline when written to file
- LaTeX output with no newline when output to stdout
- Exit code 0 only on complete success
- All exceptions caught at top level with appropriate error message

**I/O Behaviors**:
1. File input:
   - Opens file for reading
   - Reads entire content into string
   - Closes file

2. Stdin input (when input == "-"):
   - Reads all available stdin
   - Blocks until EOF

3. File output:
   - Creates or overwrites output file
   - Appends newline after LaTeX
   - Prints "Generated: <path>" to stderr

4. Stdout output:
   - Prints LaTeX without trailing newline
   - Default behavior when no -o specified

**Java Implementation Notes**:
```java
public final class Rpn2tex {
    private static final int SUCCESS = 0;
    private static final int ERROR = 1;

    public static void main(String[] args) {
        System.exit(main(args));
    }

    static int main(String[] args) {
        // Parse arguments (using picocli or commons-cli)
        CommandLineParser parser = new DefaultParser();
        Options options = new Options();
        options.addOption(Option.builder("o")
            .longOpt("output")
            .hasArg()
            .argName("FILE")
            .desc("Output LaTeX file (default: stdout)")
            .build());

        CommandLine cmd;
        try {
            cmd = parser.parse(options, args);
        } catch (ParseException e) {
            System.err.println("Error: " + e.getMessage());
            return ERROR;
        }

        String[] remaining = cmd.getArgs();
        if (remaining.length != 1) {
            System.err.println("Error: Exactly one input file required");
            return ERROR;
        }

        String input = remaining[0];
        String output = cmd.getOptionValue("o");

        // Read input
        String text;
        try {
            text = readInput(input);
        } catch (IOException e) {
            System.err.println("Error: " + e.getMessage());
            return ERROR;
        }

        // Process pipeline
        ErrorFormatter formatter = new ErrorFormatter(text);
        try {
            Lexer lexer = new Lexer(text);
            List<Token> tokens = lexer.tokenize();

            Parser p = new Parser(tokens);
            Expr ast = p.parse();

            LaTeXGenerator generator = new LaTeXGenerator();
            String latex = generator.generate(ast);

            // Write output
            if (output != null) {
                try {
                    Files.writeString(Paths.get(output), latex + "\n");
                    System.err.println("Generated: " + output);
                } catch (IOException e) {
                    System.err.println("Error: " + e.getMessage());
                    return ERROR;
                }
            } else {
                System.out.println(latex);
            }

            return SUCCESS;
        } catch (LexerError e) {
            String formatted = formatter.formatError(e.message, e.line, e.column);
            System.err.println(formatted);
            return ERROR;
        } catch (ParserError e) {
            String formatted = formatter.formatError(e.message, e.token.line(), e.token.column());
            System.err.println(formatted);
            return ERROR;
        }
    }

    private static String readInput(String input) throws IOException {
        if ("-".equals(input)) {
            return new String(System.in.readAllBytes(), StandardCharsets.UTF_8);
        } else {
            return Files.readString(Paths.get(input));
        }
    }
}
```

---

## Dependency Graph

### Module Dependencies

```
tokens.py (no dependencies)
  ↓
errors.py (no internal dependencies)
  ↓
lexer.py (depends on tokens)
  ↓
ast_nodes.py (no dependencies)
  ↓
parser.py (depends on ast_nodes, tokens)
  ↓
latex_gen.py (depends on ast_nodes)
  ↓
cli.py (depends on errors, latex_gen, lexer, parser)
```

### Dependency Table

| Module | Imports | Depends On |
|--------|---------|-----------|
| tokens.py | dataclasses, enum | None |
| errors.py | None (string ops) | None |
| ast_nodes.py | dataclasses | None |
| lexer.py | rpn2tex.tokens | tokens |
| parser.py | rpn2tex.ast_nodes, rpn2tex.tokens | ast_nodes, tokens |
| latex_gen.py | rpn2tex.ast_nodes, functools, typing | ast_nodes |
| cli.py | argparse, sys, pathlib, rpn2tex.* | errors, latex_gen, lexer, parser |

### Migration Order

**Recommended order** (dependencies first):

1. **tokens.py** - Foundation; no dependencies
2. **errors.py** - Foundation; no dependencies
3. **ast_nodes.py** - Data structures; no dependencies
4. **lexer.py** - Depends on tokens only
5. **parser.py** - Depends on ast_nodes and tokens
6. **latex_gen.py** - Depends on ast_nodes only
7. **cli.py** - Main entry point; depends on all others

### Data Flow

```
Input Text
    ↓
  Lexer (tokens.py + lexer.py)
    ↓
Token Stream
    ↓
  Parser (parser.py)
    ↓
     AST (ast_nodes.py)
    ↓
LaTeXGenerator (latex_gen.py)
    ↓
LaTeX Output
    ↓
ErrorFormatter (errors.py) [on error]
    ↓
CLI (cli.py) [orchestration]
```

---

## Package Structure

### Proposed Java Package Organization

```
com.rpn2tex/
├── core/
│   ├── Token.java              (from tokens.py)
│   ├── TokenType.java          (from tokens.py)
│   ├── ast/
│   │   ├── Expr.java           (from ast_nodes.py - type alias)
│   │   ├── ASTNode.java        (from ast_nodes.py - base class)
│   │   ├── Number.java         (from ast_nodes.py)
│   │   └── BinaryOp.java       (from ast_nodes.py)
│   └── visitor/
│       └── ASTVisitor.java     (new - visitor interface)
├── error/
│   ├── LexerError.java         (from lexer.py)
│   ├── ParserError.java        (from parser.py)
│   └── ErrorFormatter.java     (from errors.py)
├── lexer/
│   └── Lexer.java              (from lexer.py)
├── parser/
│   └── Parser.java             (from parser.py)
├── codegen/
│   └── LaTeXGenerator.java     (from latex_gen.py)
├── cli/
│   └── Rpn2tex.java            (from cli.py)
└── Rpn2tex.java                (main entry point)
```

### Alternative Flat Package Organization

If simpler structure preferred:

```
com.rpn2tex/
├── Token.java
├── TokenType.java
├── ASTNode.java
├── Number.java
├── BinaryOp.java
├── Expr.java
├── LexerError.java
├── ParserError.java
├── ErrorFormatter.java
├── Lexer.java
├── Parser.java
├── LaTeXGenerator.java
└── Rpn2tex.java
```

**Recommendation**: Use hierarchical structure for scalability and maintainability.

---

## Python to Java Type Mappings

### Basic Types

| Python | Java | Notes |
|--------|------|-------|
| `str` | `String` | Immutable strings same in both |
| `int` | `int` | Primitive type for counters |
| `list[T]` | `List<T>` or `java.util.ArrayList<T>` | Use interface type in signatures |
| `dict[K, V]` | `Map<K, V>` | Use interface type in signatures |
| `Enum` | `enum` (Java keyword) | Direct equivalent |
| `bool` | `boolean` | Primitive type |
| `None` | `null` | Null reference; use Optional for safer handling |

### Collections Patterns

| Python | Java | Notes |
|--------|------|-------|
| `list.append(x)` | `list.add(x)` | Growing dynamic list |
| `list.pop()` | `stack.pop()` or `deque.pop()` | Use Stack or Deque interface |
| `list[i]` | `list.get(i)` | Bounds checking required |
| `tuple` | `record` (Java 16+) or immutable class | Fixed-size immutable |
| `dict[k]` | `map.get(k)` | May return null |
| `len(list)` | `list.size()` | Collection interface method |

### Function/Method Patterns

| Python | Java | Notes |
|--------|------|-------|
| `def func(x: Type)` | `returnType func(Type x)` | Type annotation mandatory |
| `def func() -> Type` | `Type func()` | Return type required |
| `*args` | `Type... args` | Varargs in Java |
| `**kwargs` | Map<String, Object> | Named parameters via map |
| `@classmethod` | `static` method | Class-level method |
| `@staticmethod` | `static` method | Package-level function |
| `@property` | getter method | Convention: `getX()` |
| `@singledispatchmethod` | `instanceof` + cast OR visitor pattern | Dispatch on runtime type |

### Exception Patterns

| Python | Java | Notes |
|--------|------|-------|
| `raise Error` | `throw new Error()` | Create and throw instance |
| `except ErrorType:` | `catch (ErrorType e)` | Type-specific catching |
| `except:` | `catch (Exception e)` | Catch-all (bad practice) |
| `finally:` | `finally` | Cleanup block same |
| Custom exception attrs | Exception fields | Store as instance fields |

### Class/Type Patterns

| Python | Java | Notes |
|--------|------|-------|
| `@dataclass(frozen=True)` | `record` or immutable final class | Immutable structure |
| `@dataclass` | Regular class or record | Mutable or frozen |
| Instance variables | `private final` fields | Encapsulation + immutability |
| Type hints | Type declarations | Required in Java |
| Union types `A \| B` | `sealed interface` or `Object` | Java 16+ sealed types |

---

## Common Patterns

### 1. Immutable Data Structures

**Python Pattern**:
```python
@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str
    line: int
    column: int
```

**Java Pattern** (Java 16+):
```java
public record Token(
    TokenType type,
    String value,
    int line,
    int column
) {}
```

**Java Pattern** (Java 8-15):
```java
public final class Token {
    private final TokenType type;
    private final String value;
    private final int line;
    private final int column;

    public Token(TokenType type, String value, int line, int column) {
        this.type = Objects.requireNonNull(type);
        this.value = Objects.requireNonNull(value);
        this.line = line;
        this.column = column;
    }

    // Generated equals, hashCode, toString
}
```

### 2. Position Tracking

**Python Pattern**:
```python
class ASTNode:
    line: int
    column: int

class Number(ASTNode):
    value: str

class BinaryOp(ASTNode):
    operator: str
    left: Expr
    right: Expr
```

**Java Pattern**:
```java
public abstract class ASTNode {
    public final int line;
    public final int column;

    protected ASTNode(int line, int column) {
        this.line = line;
        this.column = column;
    }
}

public final class Number extends ASTNode {
    public final String value;

    public Number(int line, int column, String value) {
        super(line, column);
        this.value = Objects.requireNonNull(value);
    }
}

public final class BinaryOp extends ASTNode {
    public final String operator;
    public final Expr left;
    public final Expr right;

    public BinaryOp(int line, int column, String operator, Expr left, Expr right) {
        super(line, column);
        this.operator = Objects.requireNonNull(operator);
        this.left = Objects.requireNonNull(left);
        this.right = Objects.requireNonNull(right);
    }
}
```

### 3. Visitor Pattern

**Python Pattern** (single-dispatch):
```python
class LaTeXGenerator:
    @singledispatchmethod
    def _visit(self, node: Expr) -> str:
        raise NotImplementedError(f"No visitor for {type(node).__name__}")

    @_visit.register
    def _visit_number(self, node: Number) -> str:
        return node.value

    @_visit.register
    def _visit_binary_op(self, node: BinaryOp) -> str:
        # implementation
        pass
```

**Java Pattern** (double dispatch):
```java
public interface Expr {
    String accept(ExprVisitor visitor);
}

public interface ExprVisitor {
    String visit(Number number);
    String visit(BinaryOp binaryOp);
}

public class LaTeXGenerator implements ExprVisitor {
    public String generate(Expr ast) {
        return "$" + ast.accept(this) + "$";
    }

    @Override
    public String visit(Number number) {
        return number.value;
    }

    @Override
    public String visit(BinaryOp binaryOp) {
        // implementation
        return "";
    }
}
```

**Java Pattern** (instanceof dispatch):
```java
public class LaTeXGenerator {
    public String generate(Expr ast) {
        return "$" + visit(ast) + "$";
    }

    private String visit(Expr node) {
        if (node instanceof Number num) {
            return num.value;
        } else if (node instanceof BinaryOp binOp) {
            // implementation
            return "";
        } else {
            throw new AssertionError("Unknown node type");
        }
    }
}
```

### 4. Stack-Based Algorithm

**Python Pattern**:
```python
def parse(self) -> Expr:
    stack: list[Expr] = []

    while not self._at_end():
        token = self._current()
        if token.type == TokenType.NUMBER:
            stack.append(Number(...))
        elif token.type in (TokenType.PLUS, ...):
            right = stack.pop()
            left = stack.pop()
            stack.append(BinaryOp(..., left, right))

    return stack[0]
```

**Java Pattern**:
```java
public Expr parse() {
    Deque<Expr> stack = new ArrayDeque<>();

    while (!isAtEnd()) {
        Token token = current();
        if (token.type() == TokenType.NUMBER) {
            stack.push(new Number(...));
        } else if (token.type() == TokenType.PLUS ||
                   token.type() == TokenType.MINUS ||
                   token.type() == TokenType.MULT ||
                   token.type() == TokenType.DIV) {
            Expr right = stack.pop();
            Expr left = stack.pop();
            stack.push(new BinaryOp(..., left, right));
        }
        advance();
    }

    return stack.pop();
}
```

### 5. Error Handling with Context

**Python Pattern**:
```python
class LexerError(Exception):
    message: str
    line: int
    column: int

    def __init__(self, message: str, line: int, column: int) -> None:
        super().__init__(f"Line {line}, column {column}: {message}")
        self.message = message
        self.line = line
        self.column = column

# Raise with context
raise LexerError("Unexpected character '@'", 1, 5)

# Catch and format
try:
    # processing
except LexerError as e:
    formatted = formatter.format_error(e.message, e.line, e.column)
```

**Java Pattern**:
```java
public class LexerError extends Exception {
    public final String message;
    public final int line;
    public final int column;

    public LexerError(String message, int line, int column) {
        super("Line " + line + ", column " + column + ": " + message);
        this.message = Objects.requireNonNull(message);
        this.line = line;
        this.column = column;
    }
}

// Raise with context
throw new LexerError("Unexpected character '@'", 1, 5);

// Catch and format
try {
    // processing
} catch (LexerError e) {
    String formatted = formatter.formatError(e.message, e.line, e.column);
}
```

### 6. String Building and Formatting

**Python Pattern**:
```python
parts: list[str] = []
parts.append(f"Error: {message}")
parts.append("")
parts.append(context)
return "\n".join(parts)

# Caret positioning
caret_prefix = " " * num_width + " | "
caret_line = caret_prefix + " " * (column - 1) + "^"
```

**Java Pattern**:
```java
StringBuilder sb = new StringBuilder();
sb.append("Error: ").append(message).append("\n\n");
sb.append(context);
return sb.toString();

// Caret positioning
String caretPrefix = " ".repeat(numWidth) + " | ";
String caretLine = caretPrefix + " ".repeat(column - 1) + "^";
```

### 7. Operator Precedence Mapping

**Python Pattern**:
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
```

**Java Pattern**:
```java
static final Map<String, String> BINARY_OPS = Collections.unmodifiableMap(
    Map.ofEntries(
        Map.entry("+", "+"),
        Map.entry("-", "-"),
        Map.entry("*", "\\times"),
        Map.entry("/", "\\div")
    )
);

static final Map<String, Integer> PRECEDENCE = Collections.unmodifiableMap(
    Map.ofEntries(
        Map.entry("+", 1),
        Map.entry("-", 1),
        Map.entry("*", 2),
        Map.entry("/", 2)
    )
);
```

### 8. Token Type Matching

**Python Pattern**:
```python
if token.type == TokenType.NUMBER:
    # handle number
elif token.type in (TokenType.PLUS, TokenType.MINUS, TokenType.MULT, TokenType.DIV):
    # handle operator
```

**Java Pattern**:
```java
if (token.type() == TokenType.NUMBER) {
    // handle number
} else if (token.type() == TokenType.PLUS ||
           token.type() == TokenType.MINUS ||
           token.type() == TokenType.MULT ||
           token.type() == TokenType.DIV) {
    // handle operator
}

// Alternative: switch expression (Java 16+)
switch (token.type()) {
    case NUMBER -> { /* handle number */ }
    case PLUS, MINUS, MULT, DIV -> { /* handle operator */ }
    default -> { /* unknown */ }
}
```

### 9. File I/O

**Python Pattern**:
```python
from pathlib import Path

# Read file
text = Path(args.input).read_text()

# Read stdin
text = sys.stdin.read()

# Write file
args.output.write_text(latex + "\n")

# Print to stdout
print(latex)

# Print to stderr
print(formatted, file=sys.stderr)
```

**Java Pattern**:
```java
import java.nio.file.*;

// Read file
String text = Files.readString(Paths.get(filename));

// Read stdin
String text = new String(System.in.readAllBytes(), StandardCharsets.UTF_8);

// Write file
Files.writeString(Paths.get(filename), latex + "\n");

// Print to stdout
System.out.println(latex);

// Print to stderr
System.err.println(formatted);
```

---

## Testing Strategy

### 1. Unit Testing by Module

#### tokens.py → TokenType / Token
**Test Cases**:
- Token equality: Two tokens with same fields should be equal
- Token immutability: Fields cannot be modified (use reflection check)
- TokenType enum: All 6 types exist with unique values
- Token string representation: Matches expected format

#### errors.py → ErrorFormatter
**Test Cases**:
- Format single-line error: Message, source, caret position correct
- Format multi-line context: Shows requested context lines
- Caret positioning: Correctly aligned to column number
- Line number width: Proper padding for alignment
- Edge cases: Error on first line, error on last line, error at column 1

#### lexer.py → Lexer / LexerError
**Test Cases**:
- Basic tokens: NUMBER, PLUS, MINUS, MULT, DIV
- Decimal numbers: "3.14", ".5", "0.0"
- Negative numbers: "-5" (number vs minus operator)
- Whitespace: Single space, multiple spaces, tabs, newlines
- Token positions: Line and column tracked correctly
- EOF token: Always added at end
- Invalid character: LexerError with correct position
- Edge cases: Empty input, only whitespace, numbers without operators

#### ast_nodes.py → ASTNode / Number / BinaryOp
**Test Cases**:
- Number node creation: Fields set correctly
- BinaryOp creation: Children stored correctly
- Immutability: Fields cannot be modified
- Type union: Expr type accepts both Number and BinaryOp
- Equality: Nodes with same fields should be equal
- String representation: Nodes format correctly

#### parser.py → Parser / ParserError
**Test Cases**:
- Single number: Returns Number node
- Basic operations: Addition, subtraction, multiplication, division
- Complex expressions: Multiple operators, nested operations
- Operator precedence: Stack-based evaluation correct
- Error cases: Insufficient operands, extra operands, empty input
- Position tracking: Operator nodes have correct position

#### latex_gen.py → LaTeXGenerator
**Test Cases**:
- Number generation: Output matches input
- Basic operations: Correct LaTeX symbols (+ - \times \div)
- Parenthesization: Added when needed for precedence
- Left-associativity: Correct parens for - and /
- Complex expressions: All test cases from I/O contract
- Math mode: Output wrapped in $...$

#### cli.py → main()
**Test Cases**:
- File input/output: Read from file, write to file
- Stdin/stdout: Read from stdin, write to stdout
- Arguments: Parse input and output arguments
- Error handling: LexerError and ParserError caught and formatted
- Exit codes: 0 on success, 1 on error
- Error output: Errors to stderr, output to stdout

### 2. Integration Testing

**Test Suite**: All 21 test cases from I/O contract

1. Basic operations (tests 1-4): Addition, subtraction, multiplication, division
2. Error cases (tests 5, 16, 17): Unsupported operators
3. Precedence cases (tests 6-7, 12-15): Parenthesization rules
4. Complex expressions (tests 8-11, 18-21): Mixed operators, decimals

**Validation Steps**:
- Exact output matching: LaTeX string must match exactly
- Exit codes: 0 or 1 as expected
- Error formatting: Error message includes position and context
- Decimal preservation: Input decimals not converted to integers

### 3. Test Implementation Structure

**Java Test Framework**: JUnit 5 (or JUnit 4)

```java
// Token tests
class TokenTest {
    @Test void testTokenEquality() { }
    @Test void testTokenRepresentation() { }
    @Test void testAllTokenTypes() { }
}

// Lexer tests
class LexerTest {
    @Test void testBasicTokens() { }
    @Test void testNumberParsing() { }
    @Test void testNegativeNumbers() { }
    @Test void testWhitespace() { }
    @Test void testInvalidCharacter() { }
}

// Parser tests
class ParserTest {
    @Test void testSingleNumber() { }
    @Test void testBasicOperations() { }
    @Test void testComplexExpressions() { }
    @Test void testErrorHandling() { }
}

// LaTeX generator tests
class LaTeXGeneratorTest {
    @Test void testNumberGeneration() { }
    @Test void testBasicOperations() { }
    @Test void testParenthesization() { }
    @Test void testComplexExpressions() { }
    @ParameterizedTest
    @ValueSource(strings = {
        "5 3 +",
        "5 3 -",
        "4 7 *",
        "10 2 /",
        // ... all test cases
    })
    void testIOContract(String input) { }
}

// CLI tests
class Rpn2texTest {
    @Test void testFileInput() { }
    @Test void testStdinInput() { }
    @Test void testFileOutput() { }
    @Test void testStdoutOutput() { }
    @Test void testErrorHandling() { }
    @Test void testExitCodes() { }
}
```

### 4. Test Data

**Success Cases**: 18 test inputs with expected LaTeX outputs
**Error Cases**: 3 test inputs expecting lexer errors with exit code 1

**Test Data File Format**:
```
# Token Tests (unit)
testTokenCreation:
    Input: Token(TokenType.PLUS, "+", 1, 5)
    Expected: Token equality and representation

# Lexer Tests (unit)
testLexerBasic:
    Input: "5 3 +"
    Expected: [Token(NUMBER, "5"), Token(NUMBER, "3"), Token(PLUS, "+"), Token(EOF, "")]

testLexerInvalidCharacter:
    Input: "5 3 ^"
    Expected: LexerError("Unexpected character '^'", 1, 5)

# Parser Tests (unit)
testParserAddition:
    Tokens: [Token(NUMBER, "5"), Token(NUMBER, "3"), Token(PLUS, "+"), Token(EOF)]
    Expected: BinaryOp(1, 3, "+", Number(1, 1, "5"), Number(1, 3, "3"))

# LaTeX Tests (integration)
testIOCase1:
    Input: "5 3 +"
    Expected: "$5 + 3$"

testIOCase5:
    Input: "2 3 ^"
    Expected: Error exit code 1

# CLI Tests (integration)
testCLISuccess:
    Args: ["5 3 +"]
    Expected: Stdout "$5 + 3$\n", Exit 0

testCLIError:
    Args: ["2 3 ^"]
    Expected: Stderr with formatted error, Exit 1
```

---

## Migration Order

### Phase 1: Foundation (Weeks 1-2)
**Goal**: Establish base data structures and error handling

1. **Module: tokens**
   - Java equivalents: TokenType enum, Token record/class
   - Tests: Token creation, equality, string representation
   - Duration: 2-3 days
   - Deliverable: Immutable Token/TokenType with position tracking

2. **Module: errors**
   - Java equivalent: ErrorFormatter class
   - Tests: Single-line error, multi-line context, caret positioning
   - Duration: 2-3 days
   - Deliverable: Compiler-style error formatting with source context

3. **Module: ast_nodes**
   - Java equivalents: ASTNode base, Number, BinaryOp classes
   - Tests: Node creation, immutability, type union
   - Duration: 2-3 days
   - Deliverable: Immutable AST node hierarchy

### Phase 2: Processing Pipeline (Weeks 3-4)
**Goal**: Implement lexer, parser, and code generation

4. **Module: lexer**
   - Java equivalent: Lexer class + LexerError exception
   - Tests: All lexer unit tests from lexer_test.py
   - Duration: 3-4 days
   - Deliverable: Tokenizer with position tracking and error handling

5. **Module: parser**
   - Java equivalent: Parser class + ParserError exception
   - Tests: All parser unit tests
   - Duration: 3-4 days
   - Deliverable: Stack-based RPN parser with error validation

6. **Module: latex_gen**
   - Java equivalent: LaTeXGenerator class
   - Tests: LaTeX generation, precedence, parenthesization
   - Duration: 3-4 days
   - Deliverable: AST to LaTeX converter with precedence handling

### Phase 3: CLI and Integration (Weeks 5)
**Goal**: Complete CLI and validate full pipeline

7. **Module: cli**
   - Java equivalent: Rpn2tex main class with argument parsing
   - Tests: All integration tests (21 I/O contract cases)
   - Duration: 2-3 days
   - Deliverable: Complete CLI with file/stdin I/O and error handling

8. **Integration Testing**
   - Run all 21 test cases from I/O contract
   - Verify exit codes, output format, error messages
   - Performance testing (if needed)
   - Duration: 1-2 days
   - Deliverable: Test report validating all contracts

### Timeline Summary

- **Foundation**: ~1 week (tokens, errors, ast_nodes)
- **Pipeline**: ~1.5 weeks (lexer, parser, latex_gen)
- **CLI & Testing**: ~1 week (cli, integration)
- **Buffer & Review**: ~1 week
- **Total**: ~4-5 weeks for complete Java migration

---

## Critical Implementation Notes

### 1. Position Tracking (1-based indexing)
- **Python**: All position tracking is 1-based (line 1, column 1)
- **Java**: Must maintain 1-based indexing in public APIs
- **Common Error**: Java arrays are 0-based internally; convert for calculations

### 2. Token Value Preservation
- **Requirement**: Token.value stores exact input string
- **Examples**:
  - "5" → "5" (not 5)
  - "-3" → "-3" (negative number)
  - "3.14" → "3.14" (decimal preserved)
  - "-" → "-" (subtraction operator)
- **Java Implementation**: Use String for value; parse in generator if needed

### 3. LaTeX Output Format
- **Must preserve**:
  - Input number format (no rounding to integers)
  - Space-separated operators and operands
  - Parentheses at specific precedence points only
  - Wrapping in $...$ delimiters
  - No trailing whitespace

### 4. Exception Hierarchy
- **LexerError**: Thrown during tokenization
  - Contains message, line, column
  - Used for invalid character detection
- **ParserError**: Thrown during parsing
  - Contains message, token (with position)
  - Used for insufficient operands, extra operands
- **Both**: Caught at CLI level and formatted with ErrorFormatter

### 5. Immutability Guarantee
- **All data structures**: Final fields, no setters
- **AST nodes**: Immutable tree; can be cached/shared safely
- **Token list**: Returned as immutable from Lexer
- **Maps/Constants**: Use Collections.unmodifiableMap() or Map.of()

### 6. Error Message Formatting
- **Format**: "Error: <message>\n\n<context>"
- **Context**: Line number, source line, caret on next line
- **Example**:
```
Error: Unexpected character '^'

1 | 2 3 ^ 4 *
        ^
```

### 7. Precedence Handling
- **Levels**: + - (1), * / (2)
- **Left-associativity**: - and / require parens on right operand with equal precedence
- **Rule**: Child needs parens if:
  1. Child precedence < Parent precedence, OR
  2. Child precedence == Parent precedence AND is_right AND operator in {-, /}

---

## Validation Checklist for Java Implementation

Before declaring migration complete, verify:

### Functional Correctness
- [ ] All 18 success test cases produce exact LaTeX output
- [ ] All 3 error cases with ^ produce lexer errors with exit code 1
- [ ] Error messages formatted correctly with location and context
- [ ] Decimal numbers preserved (not converted to integers)
- [ ] LaTeX symbols correct: \times for *, \div for /
- [ ] Parentheses inserted only when necessary
- [ ] Output wrapped in $...$ delimiters
- [ ] No extra whitespace in output

### Code Quality
- [ ] All data structures immutable (final fields)
- [ ] No null pointer exceptions possible (null checking)
- [ ] Proper exception hierarchy (custom exceptions)
- [ ] Clear error messages with position information
- [ ] Consistent naming conventions
- [ ] No dead code or unused imports

### API Contract
- [ ] TokenType enum with 6 types (NUMBER, PLUS, MINUS, MULT, DIV, EOF)
- [ ] Token record with (type, value, line, column) fields
- [ ] ASTNode base with position tracking
- [ ] Number and BinaryOp concrete types
- [ ] Lexer.tokenize() returns List<Token>
- [ ] Parser.parse() returns Expr
- [ ] LaTeXGenerator.generate() returns String
- [ ] ErrorFormatter.formatError() returns String

### I/O Contract
- [ ] Exit code 0 on success
- [ ] Exit code 1 on any error
- [ ] LaTeX output to stdout by default
- [ ] LaTeX output to file with -o option
- [ ] Error output to stderr
- [ ] Read from file or stdin (- for stdin)
- [ ] File I/O error handling

---

## Conclusion

This specification provides a comprehensive guide for migrating the Python rpn2tex codebase to Java. The modular approach ensures that each component can be migrated independently while maintaining the same external interface and behavior. The I/O contract serves as the validation criterion for all migrations.

Key success factors:
1. **Maintain immutability**: All data structures remain immutable
2. **Preserve positions**: 1-based line/column tracking throughout
3. **Follow dependency order**: Implement foundation modules first
4. **Validate with test cases**: Use all 21 I/O contract tests
5. **Attention to detail**: Exact output matching required

The estimated timeline is 4-5 weeks for a complete, well-tested Java implementation.

