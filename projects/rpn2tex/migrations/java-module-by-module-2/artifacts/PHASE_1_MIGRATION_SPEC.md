# Phase 1: Java Migration Specification for rpn2tex

## Document Overview

This document provides a comprehensive analysis of the Python rpn2tex codebase and specifies the migration strategy to Java. The specification is organized by module in dependency order, with detailed type mappings, class structures, and Java-specific considerations.

**Target Package Structure**: `com.rpn2tex`

**Modules to Migrate** (in order):
1. `tokens` → Foundation for token definitions
2. `ast_nodes` → AST node definitions
3. `errors` → Error formatting utilities
4. `lexer` → Tokenization
5. `parser` → Token-to-AST conversion
6. `latex_gen` → AST-to-LaTeX generation
7. `cli` → Command-line interface

---

## I/O Contract

**Source**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/java-module-by-module-2/artifacts/PHASE_0_IO_CONTRACT.md`

This contract defines the exact input/output behavior that the Java implementation must preserve.

### Overview
This document captures the exact input/output behavior of the Python rpn2tex implementation. It serves as the baseline for validating behavioral equivalence of Java, Go, and Rust migrations.

Generated from: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source`

### Test Cases

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| `5 3 +` | `$5 + 3$` | Basic addition |
| `5 3 -` | `$5 - 3$` | Basic subtraction |
| `4 7 *` | `$4 \times 7$` | Basic multiplication with \times |
| `10 2 /` | `$10 \div 2$` | Basic division with \div |
| `2 3 ^` | ERROR: Line 1, column 5: Unexpected character '^' | Exponentiation not supported |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | Parentheses added for operator precedence |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | No parentheses when precedence is natural |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | Left-to-right evaluation for same precedence |
| `5 3 - 2 -` | `$5 - 3 - 2$` | Multiple subtractions |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | Multiple divisions |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | Multiple additions |
| `2 3 4 * +` | `$2 + 3 \times 4$` | Multiplication has higher precedence |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | Parentheses for lower precedence operation |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | Parentheses around addition in multiplication |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | Multiplication before addition |
| `2 3 ^ 4 *` | ERROR: Line 1, column 5: Unexpected character '^' | Exponentiation not supported |
| `2 3 4 ^ ^` | ERROR: Line 1, column 7: Unexpected character '^' | Exponentiation not supported |
| `3.14 2 *` | `$3.14 \times 2$` | Floating point numbers supported |
| `1.5 0.5 +` | `$1.5 + 0.5$` | Floating point addition |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | Multiple complex subexpressions |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | Mixed operations with precedence |

### Error Cases

#### Unsupported Operators
The Python implementation does NOT support the exponentiation operator `^`. Inputs containing this operator will fail at the lexer stage:
- **Error Type**: `LexerError`
- **Error Pattern**: `Line <line>, column <column>: Unexpected character '^'`

#### Valid Operators
- `+` Addition
- `-` Subtraction
- `*` Multiplication (renders as `\times` in LaTeX)
- `/` Division (renders as `\div` in LaTeX)

### LaTeX Output Format

#### Operator Mapping
- Addition `+`: Rendered as ` + ` in LaTeX
- Subtraction `-`: Rendered as ` - ` in LaTeX
- Multiplication `*`: Rendered as ` \times ` in LaTeX
- Division `/`: Rendered as ` \div ` in LaTeX

#### Parentheses Rules
Parentheses are automatically added based on operator precedence:
1. **Multiplication and Division** have higher precedence than Addition and Subtraction
2. **Parentheses are added** when a lower-precedence operation appears as an operand of a higher-precedence operation
3. **No parentheses** when operations have the same precedence (natural left-to-right reading)

#### Output Format
All outputs are wrapped in LaTeX math mode delimiters: `$ ... $`

### Implementation Details

#### Python Entry Point
- **Module**: `rpn2tex.cli`
- **Function**: `main()`
- **Stdin Input**: Use input argument as `-`

#### Processing Pipeline
1. **Lexer** (`rpn2tex.lexer.Lexer`): Tokenizes input string
2. **Parser** (`rpn2tex.parser.Parser`): Builds AST from tokens
3. **Generator** (`rpn2tex.latex_gen.LaTeXGenerator`): Generates LaTeX from AST

#### Number Format Support
- Integer numbers: `5`, `10`, `100`
- Floating-point numbers: `3.14`, `1.5`, `0.5`

### Behavioral Notes

1. **Operator Precedence**: Multiplication and division have higher precedence than addition and subtraction
2. **Associativity**: All operators are left-associative
3. **RPN Evaluation**: Proper RPN evaluation respects mathematical precedence in output
4. **Whitespace**: Tokens are separated by spaces in input
5. **Error Handling**: Any lexer or parser error terminates processing and outputs to stderr

---

## Module Specifications

### Module 1: tokens.py

**File**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/tokens.py`

#### Purpose and Responsibilities

The `tokens` module defines the lexical units (tokens) recognized by the rpn2tex lexer. It establishes:
- Token types enumeration (`TokenType`)
- Token data structure with position information
- Foundation for all lexical analysis

This is a foundational module with no dependencies on other rpn2tex modules.

#### Public API

**Enums**:
- `TokenType`: Enumeration with values:
  - `NUMBER` - Numeric literals (integers and decimals)
  - `PLUS` - Addition operator (+)
  - `MINUS` - Subtraction operator (-)
  - `MULT` - Multiplication operator (*)
  - `DIV` - Division operator (/)
  - `EOF` - End of file marker

**Classes**:
- `Token(frozen=True)` - Immutable lexical token
  - **Constructor**: `__init__(type: TokenType, value: str, line: int, column: int)`
  - **Fields**:
    - `type: TokenType` - The token type
    - `value: str` - String value of the token
    - `line: int` - Line number (1-based)
    - `column: int` - Column number (1-based)
  - **Methods**:
    - `__repr__() -> str` - Returns string representation like `Token(NUMBER, '42', 1:5)`

#### Key Implementation Details

- Uses Python dataclasses with `frozen=True` to create immutable value types
- Custom `__repr__` method formats token position as `line:column`
- Token positions are 1-based (human-readable), not 0-based
- Token value is stored as string representation of the actual value

#### Java Migration Notes

**Package**: `com.rpn2tex.tokens`

**Type Mappings**:
- `TokenType` enum → Java `enum TokenType`
- `Token` dataclass → Java immutable `final class Token`
- Python frozen dataclass → Java record or immutable class with `final` fields
- `str` → `String`
- `int` → `int`

**Recommended Java Approach**:

```java
package com.rpn2tex.tokens;

public enum TokenType {
    NUMBER,
    PLUS,
    MINUS,
    MULT,
    DIV,
    EOF
}

public final class Token {
    private final TokenType type;
    private final String value;
    private final int line;
    private final int column;

    public Token(TokenType type, String value, int line, int column) {
        this.type = type;
        this.value = value;
        this.line = line;
        this.column = column;
    }

    // Getters
    public TokenType getType() { return type; }
    public String getValue() { return value; }
    public int getLine() { return line; }
    public int getColumn() { return column; }

    // Immutability: no setters

    @Override
    public String toString() {
        return String.format("Token(%s, '%s', %d:%d)",
            type.name(), value, line, column);
    }
}
```

**Alternatively, use Java 16+ Records** (more concise, automatically immutable):

```java
public record Token(TokenType type, String value, int line, int column) {
    @Override
    public String toString() {
        return String.format("Token(%s, '%s', %d:%d)",
            type.name(), value, line, column);
    }
}
```

**Java-Specific Considerations**:
- Use `enum` for `TokenType` - standard Java idiom
- Make `Token` immutable with `final` keyword and final fields
- Provide getters (following JavaBean convention) or use Records
- Implement custom `toString()` to match Python `__repr__` output
- Java enums auto-provide `name()` method (equivalent to Python `.name`)
- No need for `frozen=True` equivalent; Java's access control and `final` handle immutability

**I/O Contract Relevance**:
- Directly supports test cases by providing token structure
- Position tracking (line/column) enables error formatting in `ErrorFormatter`
- Token type enumeration prevents invalid token types at compile-time

---

### Module 2: ast_nodes.py

**File**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/ast_nodes.py`

#### Purpose and Responsibilities

The `ast_nodes` module defines the Abstract Syntax Tree (AST) node types used to represent parsed RPN expressions. It provides:
- Base class for all AST nodes
- Concrete node types: `Number`, `BinaryOp`
- Type alias for expression union type
- Position tracking for error reporting

#### Dependencies
- **Internal**: None
- **External**: `dataclasses` module (Python stdlib)

#### Public API

**Classes**:
- `ASTNode` - Base class for all AST nodes
  - **Immutable dataclass** (`frozen=True`)
  - **Constructor**: `__init__(line: int, column: int)`
  - **Fields**:
    - `line: int` - Line number (1-based)
    - `column: int` - Column number (1-based)

- `Number(ASTNode)` - Numeric literal node
  - **Constructor**: `__init__(line: int, column: int, value: str)`
  - **Fields**:
    - Inherited: `line: int`, `column: int`
    - Own: `value: str` - String representation of the number

- `BinaryOp(ASTNode)` - Binary operation node
  - **Constructor**: `__init__(line: int, column: int, operator: str, left: Expr, right: Expr)`
  - **Fields**:
    - Inherited: `line: int`, `column: int`
    - Own:
      - `operator: str` - The operator ("+", "-", "*", "/")
      - `left: Expr` - Left operand expression
      - `right: Expr` - Right operand expression

**Type Aliases**:
- `Expr = Number | BinaryOp` - Union type for all expression nodes

#### Key Implementation Details

- All nodes are immutable (frozen dataclasses)
- Recursive structure: `BinaryOp` contains `Expr` children, allowing arbitrary nesting
- Position tracking inherited from base class enables error context
- Operator stored as string (not enum) for flexibility in matching LaTeX generation

#### Java Migration Notes

**Package**: `com.rpn2tex.ast`

**Type Mappings**:
- `ASTNode` dataclass → Abstract Java class with position fields
- `Number` dataclass → Concrete Java class extending `ASTNode`
- `BinaryOp` dataclass → Concrete Java class extending `ASTNode`
- `Expr` union type → Use sealed interfaces or abstract base (Java 16+ sealed classes, or abstract superclass)
- `str` → `String`
- `int` → `int`

**Recommended Java Approach**:

```java
package com.rpn2tex.ast;

// Base class
public abstract class ASTNode {
    protected final int line;
    protected final int column;

    public ASTNode(int line, int column) {
        this.line = line;
        this.column = column;
    }

    public int getLine() { return line; }
    public int getColumn() { return column; }
}

// Concrete node types
public final class Number extends ASTNode {
    private final String value;

    public Number(int line, int column, String value) {
        super(line, column);
        this.value = value;
    }

    public String getValue() { return value; }
}

public final class BinaryOp extends ASTNode {
    private final String operator;
    private final Expr left;
    private final Expr right;

    public BinaryOp(int line, int column, String operator, Expr left, Expr right) {
        super(line, column);
        this.operator = operator;
        this.left = left;
        this.right = right;
    }

    public String getOperator() { return operator; }
    public Expr getLeft() { return left; }
    public Expr getRight() { return right; }
}

// Union type using sealed classes (Java 16+)
public sealed interface Expr permits Number, BinaryOp {}
```

**Alternative approach without sealed interfaces** (Java 8+):

```java
public abstract class Expr extends ASTNode {
    public Expr(int line, int column) {
        super(line, column);
    }
}

public final class Number extends Expr {
    // ... implementation
}

public final class BinaryOp extends Expr {
    // ... implementation
}
```

**Java-Specific Considerations**:
- Use abstract base class for `ASTNode` rather than standalone dataclass
- Make concrete node types (`Number`, `BinaryOp`) `final` to prevent further subclassing
- Provide getters following JavaBean convention
- Use sealed interfaces (Java 16+) to restrict `Expr` implementations for type safety
- Consider using builder pattern for `BinaryOp` which has multiple parameters
- Null safety: Mark fields with `@NonNull` if using Optional or Nullable annotations
- Immutability: No setters; all fields initialized in constructor

**I/O Contract Relevance**:
- `Number` directly represents numeric literals in test cases (e.g., `5`, `3.14`)
- `BinaryOp` represents operations evaluated in RPN (e.g., `5 3 +` becomes `BinaryOp("+", Number("5"), Number("3"))`)
- Position tracking enables error formatting with line/column information
- Structure enables LaTeX generation via visitor pattern

---

### Module 3: errors.py

**File**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/errors.py`

#### Purpose and Responsibilities

The `errors` module provides error formatting functionality for displaying user-friendly error messages with source context. It:
- Formats errors with source code context (gcc/rustc style)
- Displays line numbers and content
- Positions caret (^) under error location
- Supports configurable context lines

#### Dependencies
- **Internal**: None
- **External**: None (pure Python stdlib features)

#### Public API

**Classes**:
- `ErrorFormatter` - Formats parse/lexer errors with context
  - **Constructor**: `__init__(source: str)`
    - **Parameters**: `source` - Complete source text being parsed
  - **Instance Variables**:
    - `source: str` - The original source text
    - `lines: list[str]` - Source split into lines
  - **Public Methods**:
    - `format_error(message: str, line: int, column: int, *, context_lines: int = 1) -> str`
      - **Parameters**:
        - `message: str` - The error message to display
        - `line: int` - Line number (1-based)
        - `column: int` - Column number (1-based)
        - `context_lines: int` - Lines to show before/after (default: 1)
      - **Returns**: Formatted error string with context
  - **Private Methods**:
    - `_get_context(line: int, column: int, context_lines: int) -> str`
      - Extracts and formats source context around error position
      - Returns formatted lines with caret indicator

#### Key Implementation Details

- **Line number indexing**: Converts between 1-based (user-facing) and 0-based (Python list) indices
- **Caret positioning**: Calculates exact column position for error caret
- **Context clamping**: Prevents seeking beyond file boundaries with `max()` and `min()`
- **Alignment**: Aligns caret under error by calculating line number width
- **Output format**:
  ```
  Error: <message>

  <line_num> | <source_line>
  <spaces>   | <caret_line>
  ```

#### Java Migration Notes

**Package**: `com.rpn2tex.errors`

**Type Mappings**:
- `ErrorFormatter` class → Java class with same structure
- `str` → `String`
- `list[str]` → `String[]` or `List<String>`
- `int` → `int`

**Recommended Java Approach**:

```java
package com.rpn2tex.errors;

import java.util.ArrayList;
import java.util.List;

public class ErrorFormatter {
    private final String source;
    private final List<String> lines;

    public ErrorFormatter(String source) {
        this.source = source;
        this.lines = new ArrayList<>();
        for (String line : source.split("\\n", -1)) {
            this.lines.add(line);
        }
    }

    public String formatError(String message, int line, int column) {
        return formatError(message, line, column, 1);
    }

    public String formatError(String message, int line, int column, int contextLines) {
        List<String> parts = new ArrayList<>();

        parts.add("Error: " + message);
        parts.add("");

        String context = getContext(line, column, contextLines);
        parts.add(context);

        return String.join("\n", parts);
    }

    private String getContext(int line, int column, int contextLines) {
        // Convert to 0-based index
        int errorIdx = line - 1;

        // Calculate range (clamped to valid indices)
        int startIdx = Math.max(0, errorIdx - contextLines);
        int endIdx = Math.min(lines.size(), errorIdx + contextLines + 1);

        // Calculate line number width for alignment
        int maxLineNum = endIdx;
        int numWidth = String.valueOf(maxLineNum).length();

        List<String> resultLines = new ArrayList<>();

        for (int idx = startIdx; idx < endIdx; idx++) {
            int lineNum = idx + 1;  // Convert back to 1-based
            String lineContent = idx < lines.size() ? lines.get(idx) : "";

            // Format line with number
            String prefix = String.format("%" + numWidth + "d | ", lineNum);
            resultLines.add(prefix + lineContent);

            // Add caret on error line
            if (idx == errorIdx) {
                String caretPrefix = " ".repeat(numWidth) + " | ";
                int caretPos = Math.max(0, column - 1);
                String caretLine = caretPrefix + " ".repeat(caretPos) + "^";
                resultLines.add(caretLine);
            }
        }

        return String.join("\n", resultLines);
    }
}
```

**Java-Specific Considerations**:
- Java 8 regex: `split("\\n", -1)` to preserve empty trailing lines
- Java 11+: Use `String.repeat()` for clean string repetition
- Java 8: Use Apache Commons Lang or implement helper for string repetition
- Use `ArrayList<String>` for dynamic line building, convert to `String[]` if needed
- Provide overload of `formatError()` with default `contextLines = 1`
- Use `String.format()` for formatted output (similar to Python's `f-strings`)
- Line number alignment: Calculate padding dynamically based on max line number width
- Clamping: Use `Math.max()` and `Math.min()` instead of Python's built-in `max()` and `min()`

**I/O Contract Relevance**:
- Direct relevance to error test cases (e.g., `2 3 ^` produces error with line/column context)
- Formats `LexerError` and `ParserError` for display to users
- Error pattern must match exactly: `Line <line>, column <column>: <message>`
- Example error output test case:
  ```
  Error: Unexpected character '^'

  1 | 2 3 ^
        ^
  ```

---

### Module 4: lexer.py

**File**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/lexer.py`

#### Purpose and Responsibilities

The `lexer` module tokenizes RPN expression input into a stream of tokens. It:
- Scans input character-by-character
- Recognizes numbers (integers and decimals), operators, and whitespace
- Tracks position (line/column) for error reporting
- Handles negative numbers as prefix to positive integer
- Produces token stream ending with EOF marker

#### Dependencies
- **Internal**: `tokens` module (`Token`, `TokenType`)
- **External**: None

#### Public API

**Exception Classes**:
- `LexerError(Exception)` - Raised when lexer encounters invalid input
  - **Constructor**: `__init__(message: str, line: int, column: int)`
  - **Instance Variables**:
    - `message: str` - Description of the error
    - `line: int` - Line number (1-based)
    - `column: int` - Column number (1-based)
  - **Inherited from Exception**: Standard exception behavior

**Classes**:
- `Lexer` - Tokenizes RPN input text
  - **Constructor**: `__init__(text: str)`
    - **Parameters**: `text` - The RPN expression to tokenize
  - **Instance Variables** (managed internally):
    - `text: str` - Input text
    - `pos: int` - Current position in text (0-based)
    - `line: int` - Current line number (1-based)
    - `column: int` - Current column number (1-based)
  - **Public Methods**:
    - `tokenize() -> list[Token]`
      - **Returns**: List of tokens ending with EOF token
      - **Raises**: `LexerError` for invalid characters
  - **Private Methods**:
    - `_at_end() -> bool` - Check if reached end of input
    - `_peek() -> str` - Look at current character without consuming
    - `_advance() -> str` - Consume and return current character
    - `_skip_whitespace() -> None` - Skip whitespace characters
    - `_scan_token() -> Token` - Scan and return next token
    - `_scan_number(prefix: str, start_line: int, start_column: int) -> Token` - Scan numeric literal

#### Key Implementation Details

**Token Recognition**:
- **Numbers**: Sequence of digits, optionally followed by decimal point and more digits
  - Handles negative numbers: `-` followed immediately by digit becomes part of number
  - Examples: `5`, `3.14`, `-2`, `0.5`
- **Operators**: Single-character tokens `+`, `-`, `*`, `/`
  - Special case: `-` can be operator or prefix (disambiguated by following character)
- **Whitespace**: Space, tab, newline, carriage return - used as delimiter
- **EOF**: Synthetic token at end

**Position Tracking**:
- Maintains `line` and `column` counters (1-based)
- Updates on each character consumption:
  - Newline increments `line`, resets `column` to 1
  - Other characters increment `column`

**Error Handling**:
- Raises `LexerError` for unrecognized characters (e.g., `^`, `@`)
- Includes position information in error

#### Java Migration Notes

**Package**: `com.rpn2tex.lexer`

**Type Mappings**:
- `LexerError` exception → Java exception class extending `Exception`
- `Lexer` class → Java class with same structure
- `str` → `String`
- `list[Token]` → `List<Token>` or `Token[]`
- `int` → `int`
- `bool` → `boolean`

**Recommended Java Approach**:

```java
package com.rpn2tex.lexer;

import com.rpn2tex.tokens.Token;
import com.rpn2tex.tokens.TokenType;
import java.util.ArrayList;
import java.util.List;

public class LexerError extends Exception {
    private final String message;
    private final int line;
    private final int column;

    public LexerError(String message, int line, int column) {
        super(String.format("Line %d, column %d: %s", line, column, message));
        this.message = message;
        this.line = line;
        this.column = column;
    }

    public String getMessage() { return message; }
    public int getLine() { return line; }
    public int getColumn() { return column; }
}

public class Lexer {
    private final String text;
    private int pos;
    private int line;
    private int column;

    public Lexer(String text) {
        this.text = text;
        this.pos = 0;
        this.line = 1;
        this.column = 1;
    }

    public List<Token> tokenize() throws LexerError {
        List<Token> tokens = new ArrayList<>();

        while (!atEnd()) {
            skipWhitespace();
            if (atEnd()) {
                break;
            }
            tokens.add(scanToken());
        }

        tokens.add(new Token(TokenType.EOF, "", line, column));
        return tokens;
    }

    private boolean atEnd() {
        return pos >= text.length();
    }

    private char peek() {
        if (atEnd()) {
            return '\0';
        }
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
        while (!atEnd() && Character.isWhitespace(peek())) {
            advance();
        }
    }

    private Token scanToken() throws LexerError {
        int startLine = line;
        int startColumn = column;
        char c = peek();

        // Single-character operators
        if (c == '+') {
            advance();
            return new Token(TokenType.PLUS, "+", startLine, startColumn);
        }
        if (c == '-') {
            advance();
            if (!atEnd() && Character.isDigit(peek())) {
                // Negative number
                return scanNumber("-", startLine, startColumn);
            }
            return new Token(TokenType.MINUS, "-", startLine, startColumn);
        }
        if (c == '*') {
            advance();
            return new Token(TokenType.MULT, "*", startLine, startColumn);
        }
        if (c == '/') {
            advance();
            return new Token(TokenType.DIV, "/", startLine, startColumn);
        }

        // Numbers
        if (Character.isDigit(c)) {
            return scanNumber("", startLine, startColumn);
        }

        // Unknown character
        throw new LexerError(String.format("Unexpected character '%c'", c), startLine, startColumn);
    }

    private Token scanNumber(String prefix, int startLine, int startColumn) {
        String value = prefix;

        // Integer part
        while (!atEnd() && Character.isDigit(peek())) {
            value += advance();
        }

        // Decimal part (optional)
        if (!atEnd() && peek() == '.') {
            value += advance();  // consume '.'
            while (!atEnd() && Character.isDigit(peek())) {
                value += advance();
            }
        }

        return new Token(TokenType.NUMBER, value, startLine, startColumn);
    }
}
```

**Java-Specific Considerations**:
- Java uses checked exceptions; make `LexerError extends Exception`
- `Character.isWhitespace()` is equivalent to Python's string method
- `Character.isDigit()` for checking digits
- Use `StringBuilder` for building token values (more efficient than string concatenation in loops)
- `char` type for single characters (use `'\0'` as null character sentinel)
- Explicit null character check instead of Python's empty string `""`
- Method naming: Java convention uses `camelCase` with prefix patterns (e.g., `_` → private methods should NOT have underscore, just be private)
- Exception must be checked exception (extends `Exception`), not unchecked

**I/O Contract Relevance**:
- Core component in processing pipeline
- Tokenizes all test inputs correctly
- Error case examples:
  - `2 3 ^` → `LexerError` at column 5 (position of `^`)
  - `5 3 +` → Tokens: `NUMBER("5")`, `NUMBER("3")`, `PLUS("+")`, `EOF("")`
- Position tracking enables error messages with line/column context
- Number parsing supports integers and floating-point (e.g., `3.14`)

---

### Module 5: parser.py

**File**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/parser.py`

#### Purpose and Responsibilities

The `parser` module implements a stack-based RPN parser that converts a token stream into an Abstract Syntax Tree (AST). It:
- Implements stack-based RPN parsing algorithm
- Consumes tokens and builds expression tree
- Validates RPN structure (correct operand/operator counts)
- Provides detailed error messages with token context

#### Dependencies
- **Internal**: `ast_nodes` module (`ASTNode`, `Number`, `BinaryOp`, `Expr`), `tokens` module (`Token`, `TokenType`)
- **External**: None

#### Public API

**Exception Classes**:
- `ParserError(Exception)` - Raised when parser encounters invalid input
  - **Constructor**: `__init__(message: str, token: Token)`
  - **Instance Variables**:
    - `message: str` - Description of the error
    - `token: Token` - The token where error occurred
  - **Exception message format**: `"{message} at line {line}, column {column}"`

**Classes**:
- `Parser` - Stack-based RPN parser
  - **Constructor**: `__init__(tokens: list[Token])`
    - **Parameters**: `tokens` - List of tokens from lexer (must end with EOF)
  - **Instance Variables**:
    - `tokens: list[Token]` - Token list
    - `pos: int` - Current position in token list
  - **Public Methods**:
    - `parse() -> Expr`
      - **Returns**: Root expression node of the AST
      - **Raises**: `ParserError` for invalid RPN
  - **Private Methods**:
    - `_current() -> Token` - Get current token
    - `_at_end() -> bool` - Check if at EOF
    - `_advance() -> Token` - Consume and advance to next token

#### Key Implementation Details

**RPN Parsing Algorithm**:
1. Initialize empty stack
2. For each token:
   - If NUMBER: Create `Number` node, push to stack
   - If OPERATOR: Pop 2 operands, create `BinaryOp` node, push result
   - If EOF: Break loop
3. Validate: Stack must contain exactly one element (the AST root)

**Error Cases**:
- **Insufficient operands**: Operator without 2 operands on stack
  - Example: `+` with only 1 number → `ParserError: Operator '+' requires two operands`
- **Empty expression**: No tokens (only EOF)
  - Example: Empty input → `ParserError: Empty expression`
- **Extra operands**: Multiple values remain after parsing
  - Example: `5 3` without operator → `ParserError: Invalid RPN: 2 values remain on stack (missing operators?)`

**Operator Mapping**:
- `TokenType.PLUS` → `"+"`
- `TokenType.MINUS` → `"-"`
- `TokenType.MULT` → `"*"`
- `TokenType.DIV` → `"/"`

#### Java Migration Notes

**Package**: `com.rpn2tex.parser`

**Type Mappings**:
- `ParserError` exception → Java exception class extending `Exception`
- `Parser` class → Java class with same structure
- `list[Token]` → `List<Token>` or `Token[]`
- `int` → `int`
- `bool` → `boolean`

**Recommended Java Approach**:

```java
package com.rpn2tex.parser;

import com.rpn2tex.ast.*;
import com.rpn2tex.tokens.Token;
import com.rpn2tex.tokens.TokenType;
import java.util.ArrayList;
import java.util.List;

public class ParserError extends Exception {
    private final String message;
    private final Token token;

    public ParserError(String message, Token token) {
        super(String.format("%s at line %d, column %d",
            message, token.getLine(), token.getColumn()));
        this.message = message;
        this.token = token;
    }

    public String getMessage() { return message; }
    public Token getToken() { return token; }
}

public class Parser {
    private final List<Token> tokens;
    private int pos;

    public Parser(List<Token> tokens) {
        this.tokens = tokens;
        this.pos = 0;
    }

    public Expr parse() throws ParserError {
        List<Expr> stack = new ArrayList<>();

        while (!atEnd()) {
            Token token = current();

            if (token.getType() == TokenType.NUMBER) {
                Number numNode = new Number(
                    token.getLine(),
                    token.getColumn(),
                    token.getValue()
                );
                stack.add(numNode);
                advance();

            } else if (isOperator(token.getType())) {
                if (stack.size() < 2) {
                    throw new ParserError(
                        String.format("Operator '%s' requires two operands", token.getValue()),
                        token
                    );
                }

                Expr right = stack.remove(stack.size() - 1);
                Expr left = stack.remove(stack.size() - 1);

                String operator = tokenTypeToOperator(token.getType());

                BinaryOp opNode = new BinaryOp(
                    token.getLine(),
                    token.getColumn(),
                    operator,
                    left,
                    right
                );
                stack.add(opNode);
                advance();

            } else if (token.getType() == TokenType.EOF) {
                break;

            } else {
                throw new ParserError(
                    String.format("Unexpected token '%s'", token.getValue()),
                    token
                );
            }
        }

        // Validate final state
        if (stack.isEmpty()) {
            Token eofToken = tokens.get(tokens.size() - 1);
            throw new ParserError("Empty expression", eofToken);
        }

        if (stack.size() > 1) {
            throw new ParserError(
                String.format("Invalid RPN: %d values remain on stack (missing operators?)",
                    stack.size()),
                tokens.get(tokens.size() - 1)
            );
        }

        return stack.get(0);
    }

    private Token current() {
        return tokens.get(pos);
    }

    private boolean atEnd() {
        return tokens.get(pos).getType() == TokenType.EOF;
    }

    private Token advance() {
        Token token = tokens.get(pos);
        if (!atEnd()) {
            pos++;
        }
        return token;
    }

    private boolean isOperator(TokenType type) {
        return type == TokenType.PLUS || type == TokenType.MINUS ||
               type == TokenType.MULT || type == TokenType.DIV;
    }

    private String tokenTypeToOperator(TokenType type) {
        return switch (type) {
            case PLUS -> "+";
            case MINUS -> "-";
            case MULT -> "*";
            case DIV -> "/";
            default -> throw new IllegalArgumentException("Not an operator: " + type);
        };
    }
}
```

**Java-Specific Considerations**:
- Java switch expressions (Java 14+) provide cleaner operator mapping
- `List.remove(index)` removes element at index
- Stack implemented as `List` (not Java's `Stack` class) to match Python's list-based approach
- Use `List<Expr>` (generic type) for type safety at compile-time
- Exception messages use `String.format()` for formatting
- Method names follow Java convention: no underscore prefix for private methods
- Consider using enum for operator types instead of string mapping
- Checked exception: `ParserError extends Exception` (not `RuntimeException`)

**I/O Contract Relevance**:
- Core component in RPN processing pipeline
- Converts token stream to AST that drives LaTeX generation
- Test cases validate:
  - Basic operations: `5 3 +` → `BinaryOp("+", Number("5"), Number("3"))`
  - Complex expressions: `5 3 + 2 *` → `BinaryOp("*", BinaryOp("+", 5, 3), 2)`
  - Error cases: Insufficient operands, extra operands, empty expressions
- Operator precedence determined by AST structure (not by parser)
- LaTeX generation depends on correct AST construction

---

### Module 6: latex_gen.py

**File**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/latex_gen.py`

#### Purpose and Responsibilities

The `latex_gen` module generates LaTeX source code from the AST. It:
- Implements visitor pattern using singledispatchmethod
- Converts AST nodes to infix notation with LaTeX formatting
- Manages operator precedence for intelligent parenthesization
- Wraps output in LaTeX math mode delimiters
- Outputs LaTeX-formatted mathematical expressions

#### Dependencies
- **Internal**: `ast_nodes` module (`ASTNode`, `Number`, `BinaryOp`, `Expr`)
- **External**: `functools.singledispatchmethod`, `typing`

#### Public API

**Classes**:
- `LaTeXGenerator` - Converts AST to LaTeX source
  - **Class Variables**:
    - `BINARY_OPS: dict[str, str]` - Maps operators to LaTeX strings:
      - `"+"` → `"+"`
      - `"-"` → `"-"`
      - `"*"` → `r"\times"`
      - `"/"` → `r"\div"`
    - `PRECEDENCE: dict[str, int]` - Operator precedence levels:
      - `"+"` → 1
      - `"-"` → 1
      - `"*"` → 2
      - `"/"` → 2
  - **Constructor**: `__init__()` - No arguments
  - **Public Methods**:
    - `generate(ast: Expr) -> str`
      - **Parameters**: `ast` - Root expression node
      - **Returns**: LaTeX string wrapped in `$...$` delimiters
  - **Private Methods** (Visitor Pattern):
    - `_visit(node: Expr) -> str`
      - Dispatcher method using `@singledispatchmethod`
      - Raises `NotImplementedError` for unknown node types
    - `_visit_number(node: Number) -> str`
      - Registered handler for `Number` nodes
      - Returns the numeric value as string
    - `_visit_binary_op(node: BinaryOp) -> str`
      - Registered handler for `BinaryOp` nodes
      - Manages parenthesization based on precedence
      - Returns formatted LaTeX expression
    - `_needs_parens(child: Expr, parent_precedence: int, *, is_right: bool) -> bool`
      - Determines if parentheses needed for child expression
      - Considers operator precedence and associativity

#### Key Implementation Details

**Visitor Pattern**:
- Uses Python's `@singledispatchmethod` decorator
- Dispatches on node type to appropriate handler method
- Extensible for new node types (exercises)

**Precedence-Based Parenthesization**:
- **Precedence Levels**:
  - Level 1: `+`, `-` (lower binding)
  - Level 2: `*`, `/` (higher binding)
- **Parentheses Rules**:
  1. Child with lower precedence than parent → parentheses needed
  2. Child with equal precedence on right side of left-associative operator → parentheses needed
     - Example: `5 - (3 - 2)` needs parens (right side of `-`)
     - Example: `(5 - 3) - 2` doesn't need parens (left side of `-`)

**LaTeX Formatting**:
- Numbers: Output as-is (e.g., `5`, `3.14`)
- Operations: `{left} {operator} {right}` with spaces
- Parentheses: `( {expression} )` with spaces

**Output Format**:
- Wrapped in LaTeX math mode: `${content}$`

#### Java Migration Notes

**Package**: `com.rpn2tex.latex`

**Type Mappings**:
- `LaTeXGenerator` class → Java class with same structure
- `@singledispatchmethod` → Use visitor pattern or method overloading
- `dict[str, str]` → `Map<String, String>`
- `dict[str, int]` → `Map<String, Integer>`
- `str` → `String`
- `int` → `int`
- `bool` → `boolean`

**Recommended Java Approach (Using Visitor Pattern)**:

```java
package com.rpn2tex.latex;

import com.rpn2tex.ast.*;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

public class LaTeXGenerator {
    private static final Map<String, String> BINARY_OPS;
    private static final Map<String, Integer> PRECEDENCE;

    static {
        Map<String, String> ops = new HashMap<>();
        ops.put("+", "+");
        ops.put("-", "-");
        ops.put("*", "\\times");
        ops.put("/", "\\div");
        BINARY_OPS = Collections.unmodifiableMap(ops);

        Map<String, Integer> prec = new HashMap<>();
        prec.put("+", 1);
        prec.put("-", 1);
        prec.put("*", 2);
        prec.put("/", 2);
        PRECEDENCE = Collections.unmodifiableMap(prec);
    }

    public String generate(Expr ast) {
        String content = visit(ast);
        return "$" + content + "$";
    }

    private String visit(Expr node) {
        if (node instanceof Number) {
            return visitNumber((Number) node);
        } else if (node instanceof BinaryOp) {
            return visitBinaryOp((BinaryOp) node);
        } else {
            throw new IllegalArgumentException("Unknown node type: " + node.getClass().getName());
        }
    }

    private String visitNumber(Number node) {
        return node.getValue();
    }

    private String visitBinaryOp(BinaryOp node) {
        String opLatex = BINARY_OPS.get(node.getOperator());
        int myPrecedence = PRECEDENCE.get(node.getOperator());

        // Generate left operand, adding parens if needed
        String left = visit(node.getLeft());
        if (needsParens(node.getLeft(), myPrecedence, false)) {
            left = "( " + left + " )";
        }

        // Generate right operand, adding parens if needed
        String right = visit(node.getRight());
        if (needsParens(node.getRight(), myPrecedence, true)) {
            right = "( " + right + " )";
        }

        return left + " " + opLatex + " " + right;
    }

    private boolean needsParens(Expr child, int parentPrecedence, boolean isRight) {
        if (!(child instanceof BinaryOp)) {
            return false;
        }

        BinaryOp binOp = (BinaryOp) child;
        int childPrecedence = PRECEDENCE.get(binOp.getOperator());

        // Lower precedence always needs parens
        if (childPrecedence < parentPrecedence) {
            return true;
        }

        // Equal precedence on right side needs parens for non-commutative operators
        return childPrecedence == parentPrecedence && isRight &&
               (binOp.getOperator().equals("-") || binOp.getOperator().equals("/"));
    }
}
```

**Alternative with Sealed Interface** (Java 16+):

```java
// Use sealed interface Expr to enable instanceof pattern matching
private String visit(Expr node) {
    return switch (node) {
        case Number num -> visitNumber(num);
        case BinaryOp binOp -> visitBinaryOp(binOp);
    };
}
```

**Java-Specific Considerations**:
- Java doesn't have built-in `@singledispatchmethod` (Python runtime dispatch)
- Use `instanceof` checks and casting, or sealed interfaces (Java 16+) with pattern matching
- Use `Collections.unmodifiableMap()` for immutable class constants
- Static initializer block for complex constant initialization
- Map values: `map.get(key)` returns null if key not found; use `map.getOrDefault(key, default)`
- Operator lookup: Validate key existence before accessing (avoid NPE)
- String concatenation: Consider `StringBuilder` for complex formatting, or use `String.format()`
- `instanceof` operator for type checking (don't use reflection unless necessary)
- Cast carefully: `(BinaryOp) expr` after confirming type

**I/O Contract Relevance**:
- Final component in RPN processing pipeline
- Generates LaTeX output that must match test cases exactly:
  - Basic: `5 3 +` → `$5 + 3$`
  - With precedence: `5 3 + 2 *` → `$( 5 + 3 ) \times 2$`
  - Operator mapping: `*` → `\times`, `/` → `\div`
  - Spacing: Consistent spaces around operators and parentheses
- Parenthesization algorithm critical for correct test case outputs
- Must handle floating-point numbers (e.g., `3.14`)

---

### Module 7: cli.py

**File**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/cli.py`

#### Purpose and Responsibilities

The `cli` module provides the command-line interface that orchestrates the entire processing pipeline. It:
- Parses command-line arguments
- Reads input from file or stdin
- Executes the pipeline: tokenize → parse → generate
- Handles errors with formatted output
- Writes output to file or stdout
- Returns appropriate exit codes

#### Dependencies
- **Internal**: All other modules (`tokens`, `ast_nodes`, `errors`, `lexer`, `parser`, `latex_gen`)
- **External**: `argparse`, `sys`, `pathlib.Path`

#### Public API

**Functions**:
- `main() -> int`
  - **Purpose**: Entry point for CLI
  - **Parameters**: None (reads from `sys.argv` via argparse)
  - **Returns**: Exit code (0 for success, 1 for error)
  - **Side Effects**:
    - Reads from stdin or file
    - Writes to stdout or file
    - Writes error messages to stderr

**Command-Line Interface**:
```
usage: rpn2tex [-h] [-o OUTPUT] input

Convert RPN expressions to LaTeX math mode

positional arguments:
  input              Input RPN file (use '-' for stdin)

optional arguments:
  -h, --help         show this help message and exit
  -o OUTPUT, --output OUTPUT
                     Output LaTeX file (default: stdout)
```

#### Key Implementation Details

**Argument Parsing**:
- `input` (positional): Input file path or `-` for stdin
- `-o`, `--output` (optional): Output file path (default: stdout)

**Input Handling**:
- If input is `-`: Read from `sys.stdin`
- Otherwise: Read from file path
- Error handling: `FileNotFoundError`, `PermissionError`, `IsADirectoryError`

**Processing Pipeline**:
1. Create `ErrorFormatter` with source text
2. Create `Lexer`, tokenize input
3. Create `Parser`, parse tokens to AST
4. Create `LaTeXGenerator`, generate LaTeX
5. Handle exceptions: `LexerError`, `ParserError`

**Error Handling**:
- Format errors using `ErrorFormatter.format_error()`
- Output to `sys.stderr` with line/column context
- Exit with code 1

**Output Handling**:
- If output file specified: Write to file, print confirmation to stderr
- Otherwise: Print to `sys.stdout`
- Error handling: `PermissionError`, `IsADirectoryError`

#### Java Migration Notes

**Package**: `com.rpn2tex.cli`

**Type Mappings**:
- `argparse.ArgumentParser` → Java argument parsing library (Commons CLI, JCommander, or custom)
- `sys.stdin`, `sys.stdout`, `sys.stderr` → Java I/O streams
- `sys.argv` → `args` parameter to `main(String[])`
- `Path` (pathlib) → Java `java.nio.file.Path` or `File`
- `str` → `String`
- `int` → `int`
- `Path.read_text()` → `Files.readString()`
- `Path.write_text()` → `Files.writeString()`

**Recommended Java Approach**:

```java
package com.rpn2tex.cli;

import com.rpn2tex.errors.ErrorFormatter;
import com.rpn2tex.latex.LaTeXGenerator;
import com.rpn2tex.lexer.Lexer;
import com.rpn2tex.lexer.LexerError;
import com.rpn2tex.parser.Parser;
import com.rpn2tex.parser.ParserError;
import com.rpn2tex.tokens.Token;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.util.Scanner;

public class Rpn2Tex {

    public static void main(String[] args) {
        System.exit(run(args));
    }

    public static int run(String[] args) {
        // Parse command-line arguments
        Arguments parsedArgs = parseArguments(args);
        if (parsedArgs == null) {
            return 1;
        }

        // Read input
        String text;
        try {
            if ("-".equals(parsedArgs.input)) {
                text = readFromStdin();
            } else {
                text = Files.readString(Paths.get(parsedArgs.input));
            }
        } catch (FileNotFoundException | java.nio.file.NoSuchFileException e) {
            System.err.println("Error: Input file not found: " + parsedArgs.input);
            return 1;
        } catch (PermissionDeniedException | java.nio.file.AccessDeniedException e) {
            System.err.println("Error: Permission denied reading: " + parsedArgs.input);
            return 1;
        } catch (IsADirectoryException | java.nio.file.NotDirectoryException e) {
            System.err.println("Error: Expected a file, got a directory: " + parsedArgs.input);
            return 1;
        } catch (IOException e) {
            System.err.println("Error reading input: " + e.getMessage());
            return 1;
        }

        // Process: tokenize → parse → generate
        ErrorFormatter formatter = new ErrorFormatter(text);
        try {
            // Tokenize
            Lexer lexer = new Lexer(text);
            List<Token> tokens = lexer.tokenize();

            // Parse
            Parser parser = new Parser(tokens);
            com.rpn2tex.ast.Expr ast = parser.parse();

            // Generate LaTeX
            LaTeXGenerator generator = new LaTeXGenerator();
            String latex = generator.generate(ast);

            // Write output
            if (parsedArgs.output != null) {
                try {
                    Files.writeString(Paths.get(parsedArgs.output), latex + "\n");
                    System.err.println("Generated: " + parsedArgs.output);
                } catch (PermissionDeniedException | java.nio.file.AccessDeniedException e) {
                    System.err.println("Error: Permission denied writing: " + parsedArgs.output);
                    return 1;
                } catch (IsADirectoryException e) {
                    System.err.println("Error: Cannot write to directory: " + parsedArgs.output);
                    return 1;
                } catch (IOException e) {
                    System.err.println("Error writing output: " + e.getMessage());
                    return 1;
                }
            } else {
                System.out.println(latex);
            }

        } catch (LexerError e) {
            String formatted = formatter.formatError(e.getMessage(), e.getLine(), e.getColumn());
            System.err.println(formatted);
            return 1;
        } catch (ParserError e) {
            String formatted = formatter.formatError(
                e.getMessage(),
                e.getToken().getLine(),
                e.getToken().getColumn()
            );
            System.err.println(formatted);
            return 1;
        }

        return 0;
    }

    private static String readFromStdin() throws IOException {
        StringBuilder sb = new StringBuilder();
        try (Scanner scanner = new Scanner(System.in)) {
            while (scanner.hasNextLine()) {
                sb.append(scanner.nextLine()).append("\n");
            }
        }
        return sb.toString();
    }

    private static Arguments parseArguments(String[] args) {
        // Simple argument parsing (can use external library)
        if (args.length == 0) {
            System.err.println("usage: rpn2tex [-h] [-o OUTPUT] input");
            return null;
        }

        String input = null;
        String output = null;

        for (int i = 0; i < args.length; i++) {
            if ("-h".equals(args[i]) || "--help".equals(args[i])) {
                System.out.println("Convert RPN expressions to LaTeX math mode");
                return null;
            } else if ("-o".equals(args[i]) || "--output".equals(args[i])) {
                if (i + 1 < args.length) {
                    output = args[++i];
                }
            } else if (!args[i].startsWith("-")) {
                input = args[i];
            }
        }

        if (input == null) {
            System.err.println("Error: input argument required");
            return null;
        }

        return new Arguments(input, output);
    }

    static class Arguments {
        String input;
        String output;

        Arguments(String input, String output) {
            this.input = input;
            this.output = output;
        }
    }
}
```

**Alternative with Apache Commons CLI**:

```java
import org.apache.commons.cli.*;

public class Rpn2Tex {
    public static void main(String[] args) {
        System.exit(run(args));
    }

    public static int run(String[] args) {
        Options options = new Options();
        options.addOption("o", "output", true, "Output LaTeX file (default: stdout)");
        options.addOption("h", "help", false, "Show help");

        CommandLineParser parser = new DefaultParser();
        try {
            CommandLine cmd = parser.parse(options, args);

            if (cmd.hasOption("h")) {
                HelpFormatter formatter = new HelpFormatter();
                formatter.printHelp("rpn2tex [OPTIONS] input", options);
                return 0;
            }

            String[] remaining = cmd.getArgs();
            if (remaining.length == 0) {
                System.err.println("Error: input argument required");
                return 1;
            }

            String input = remaining[0];
            String output = cmd.getOptionValue("o");

            // ... rest of processing

        } catch (ParseException e) {
            System.err.println("Parse error: " + e.getMessage());
            return 1;
        }

        return 0;
    }
}
```

**Java-Specific Considerations**:
- Java uses `String[]` args, not automatic `sys.argv` parsing
- Use external library (`apache.commons.cli`, JCommander, Picocli) for better argument parsing
- Java NIO: `Files.readString()` and `Files.writeString()` (Java 11+)
- For Java 8-10: Use `new String(Files.readAllBytes(...))` or `Files.lines()`
- Exception handling: Java checked exceptions must be caught and handled
- `Scanner` is convenient for reading stdin
- Use `System.err` for error messages, `System.out` for normal output
- `System.exit()` in `main()`, but return int from separate `run()` method for testability
- File path handling: `java.nio.file.Paths` and `java.nio.file.Files`
- Consider null checks for optional arguments
- Entry point class name: `Rpn2Tex` or `Rpn2TexCli` (camelCase, not snake_case)

**I/O Contract Relevance**:
- Entry point for all test cases
- Orchestrates pipeline that must produce exact outputs matching contract
- Error handling must format errors with correct line/column information
- Output format verification: All LaTeX must be wrapped in `$ ... $`
- Example test cases:
  - Success: `rpn2tex <<< "5 3 +"` → outputs `$5 + 3$`
  - Error: `rpn2tex <<< "2 3 ^"` → error message with line/column to stderr
  - File I/O: `rpn2tex input.rpn -o output.tex`

---

## Architecture Overview

### Module Dependencies

```
tokens.py (no dependencies)
    ↓
lexer.py (depends on tokens)
    ↓
parser.py (depends on tokens, ast_nodes)
    ↓
ast_nodes.py (no dependencies) - parallel to lexer/parser
    ↓
latex_gen.py (depends on ast_nodes)
    ↓
cli.py (depends on all modules)
```

### Processing Pipeline

```
Input Text
    ↓
[Lexer] → tokens
    ↓
[Parser] → AST
    ↓
[LaTeXGenerator] → LaTeX
    ↓
Output (file or stdout)
```

### Exception Hierarchy

```
Exception
├── LexerError (lexer.py)
│   └── Attributes: message, line, column
└── ParserError (parser.py)
    └── Attributes: message, token
```

---

## Type System Summary

### Numeric Representation

- **Python**: Numeric values stored as strings (`value: str`)
- **Java**: Same approach - store as `String` for faithful representation
  - Do NOT convert to `Double` or `BigDecimal` (preserves exact user input)
  - Example: `3.14` → `String("3.14")` → LaTeX `3.14`

### Position Information

- **Python**: 1-based line/column numbers
- **Java**: Must maintain 1-based indexing
  - Lexer: position tracking increments correctly
  - ErrorFormatter: position calculations use 1-based coordinates
  - Display: Column 5 means 5th character (not 4th, like 0-based)

### Collections

- **Python**: `list[T]` with type hints
- **Java**: `List<T>` from `java.util` with generics
  - `ArrayList<T>` for mutable lists
  - `Collections.unmodifiableList()` for immutable views

---

## Migration Checklist

### Phase 1 Implementation Order

1. **Tokens Module**
   - [ ] Enum `TokenType`
   - [ ] Record/Class `Token` (immutable)
   - [ ] Unit tests for token creation

2. **AST Nodes Module**
   - [ ] Abstract class `ASTNode`
   - [ ] Classes: `Number`, `BinaryOp`
   - [ ] Type definition for `Expr`
   - [ ] Unit tests for AST construction

3. **Errors Module**
   - [ ] Class `ErrorFormatter`
   - [ ] Method `formatError()`
   - [ ] Context extraction and caret positioning
   - [ ] Unit tests with sample errors

4. **Lexer Module**
   - [ ] Exception `LexerError`
   - [ ] Class `Lexer` with complete state machine
   - [ ] Token scanning methods
   - [ ] Number parsing (integers, decimals, negatives)
   - [ ] Integration tests with all token types

5. **Parser Module**
   - [ ] Exception `ParserError`
   - [ ] Class `Parser` with RPN stack algorithm
   - [ ] Stack-based parsing implementation
   - [ ] Error handling for malformed input
   - [ ] Integration tests with complex expressions

6. **LaTeX Generator Module**
   - [ ] Class `LaTeXGenerator` with visitor pattern
   - [ ] Operator precedence table and mapping
   - [ ] Parenthesization algorithm
   - [ ] Output formatting with `$ ... $` delimiters
   - [ ] Integration tests for all operators

7. **CLI Module**
   - [ ] `main()` / `run()` entry point
   - [ ] Argument parsing
   - [ ] Input/output handling
   - [ ] Error formatting and display
   - [ ] Integration tests with files and stdin

### Testing Strategy

- **Unit Tests**: Each module tested independently with mocked dependencies
- **Integration Tests**: Full pipeline with all test cases from I/O contract
- **Error Tests**: All error cases from contract (lexer errors, parser errors)
- **Edge Cases**: Floating-point numbers, negative numbers, multiple operators

---

## Java Package Structure

```
com.rpn2tex/
├── tokens/
│   ├── TokenType.java (enum)
│   └── Token.java (record or final class)
├── ast/
│   ├── ASTNode.java (abstract class)
│   ├── Number.java (final class)
│   ├── BinaryOp.java (final class)
│   └── Expr.java (sealed interface - Java 16+, or type alias)
├── errors/
│   └── ErrorFormatter.java
├── lexer/
│   ├── LexerError.java (exception)
│   └── Lexer.java
├── parser/
│   ├── ParserError.java (exception)
│   └── Parser.java
├── latex/
│   └── LaTeXGenerator.java
└── cli/
    └── Rpn2Tex.java (or Rpn2TexCli.java)
```

---

## Key Behavioral Guarantees

1. **Exact Output Matching**: LaTeX output must match test cases byte-for-byte
2. **Error Messages**: Must match error pattern `Line <line>, column <column>: <message>`
3. **Operator Precedence**: Parentheses must follow the precedence rules exactly
4. **Number Preservation**: Numeric strings preserved as-is (3.14 → 3.14, not 3.1400)
5. **Position Tracking**: All positions 1-based, consistent throughout
6. **Exception Handling**: Lexer/Parser errors terminate gracefully with proper exit codes

---

## Notes for Migrators

1. **Immutability**: Python frozen dataclasses map to Java `final` fields and immutable classes
2. **String Formatting**: Use `String.format()` or `StringBuilder` for complex string building
3. **Lambda/Functional**: Python's `singledispatchmethod` → Use visitor pattern with `instanceof` or sealed types
4. **Static Members**: Python class variables → Java `static` fields with static initializers
5. **Exception Messages**: Must preserve exact format for error matching in tests
6. **File I/O**: Use `java.nio.file` APIs for modern Java (8+)
7. **Testing**: Create JUnit 4/5 tests for each module before integration testing
8. **Compilation**: Target Java 11+ for modern features, Java 8+ for compatibility

---

## Conclusion

This specification provides detailed guidance for migrating the rpn2tex codebase from Python to Java. The modular structure and clear dependencies enable phased migration with early validation via the I/O contract. Each module includes concrete Java code examples, type mappings, and Java-specific considerations to ensure high-fidelity translation while maintaining behavioral equivalence.

The I/O contract serves as the ultimate validation criterion: all Java implementations must produce identical output for all test cases within the contract.
