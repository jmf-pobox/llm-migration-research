# rpn2tex Python-to-Java Migration Specification

## Document Information
- **Target Language:** Java
- **Source Language:** Python
- **Analyzer:** Phase 1 Analyst Agent
- **Date Generated:** 2025-12-28
- **Status:** Complete and Ready for Migration

---

## I/O Contract (from Phase 0)

This I/O contract was generated from the Python reference implementation. It is the SINGLE SOURCE OF TRUTH for behavioral validation.

### Complete Test Case Matrix

| # | Input | Expected Output | Status | Notes |
|---|-------|-----------------|--------|-------|
| 1 | `5 3 +` | `$5 + 3$` | SUCCESS | Basic addition |
| 2 | `5 3 -` | `$5 - 3$` | SUCCESS | Basic subtraction |
| 3 | `4 7 *` | `$4 \times 7$` | SUCCESS | Basic multiplication with \times |
| 4 | `10 2 /` | `$10 \div 2$` | SUCCESS | Basic division with \div |
| 5 | `2 3 ^` | ERROR: Unexpected character '^' | ERROR | Exponentiation not supported |
| 6 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | SUCCESS | Parenthesizes lower precedence operations |
| 7 | `5 3 * 2 +` | `$5 \times 3 + 2$` | SUCCESS | Respects operator precedence |
| 8 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | SUCCESS | Left-to-right evaluation of same precedence |
| 9 | `5 3 - 2 -` | `$5 - 3 - 2$` | SUCCESS | Chained subtraction |
| 10 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | SUCCESS | Multiple divisions |
| 11 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | SUCCESS | Chained additions |
| 12 | `2 3 4 * +` | `$2 + 3 \times 4$` | SUCCESS | Multiplication has higher precedence than addition |
| 13 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | SUCCESS | Parenthesizes addition when multiplied |
| 14 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | SUCCESS | Parenthesizes addition in multiplication context |
| 15 | `2 3 * 4 +` | `$2 \times 3 + 4$` | SUCCESS | Multiplication before addition without parentheses |
| 16 | `2 3 ^ 4 *` | ERROR: Unexpected character '^' | ERROR | Exponentiation not supported |
| 17 | `2 3 4 ^ ^` | ERROR: Unexpected character '^' | ERROR | Exponentiation not supported |
| 18 | `3.14 2 *` | `$3.14 \times 2$` | SUCCESS | Handles decimal numbers |
| 19 | `1.5 0.5 +` | `$1.5 + 0.5$` | SUCCESS | Decimal arithmetic |
| 20 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | SUCCESS | Multiple parenthesized groups |
| 21 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | SUCCESS | Complex precedence handling |

See io_contract.md for complete specification details.

---

## Architecture Overview

The rpn2tex system is a pipeline that converts RPN mathematical expressions to LaTeX format:

```
Input Text → [LEXER] → Tokens → [PARSER] → AST → [LaTeX Generator] → LaTeX String
```

---

## Module Dependencies & Migration Order

**Foundation (Phase 2A - Core):**
1. `tokens.py` → `Token.java` (no dependencies)
2. `ast_nodes.py` → `Expr.java` (no dependencies)
3. `errors.py` → `RpnException.java` (no dependencies)

**Pipeline (Phase 2B):**
4. `lexer.py` → `Lexer.java` (depends on Token.java)
5. `parser.py` → `Parser.java` (depends on Token.java, Expr.java)
6. `latex_gen.py` → `LaTeXGenerator.java` (depends on Expr.java)

**CLI (Phase 2C):**
7. `cli.py` → `Main.java` (depends on all above)

---

## Module Specifications

### Module 1: tokens.py → Token.java

**Python Structure:**
- `TokenType` enum with values: NUMBER, PLUS, MINUS, MULT, DIV, EOF
- `Token` dataclass with fields: type, value, line, column (all immutable)

**Java Implementation:**
```java
package com.rpn2tex;

public enum TokenType {
    NUMBER, PLUS, MINUS, MULT, DIV, EOF
}

public final class Token {
    public final TokenType type;
    public final String value;
    public final int line;    // 1-based
    public final int column;  // 1-based

    public Token(TokenType type, String value, int line, int column) {
        this.type = type;
        this.value = value;
        this.line = line;
        this.column = column;
    }

    @Override
    public String toString() {
        return String.format("Token(%s, '%s', %d:%d)",
            type.name(), value, line, column);
    }
}
```

**Key Points:**
- Use separate files: TokenType.java and Token.java
- All fields are public final for immutability
- Line/column are 1-based (critical for error messages)

---

### Module 2: ast_nodes.py → Expr.java

**Python Structure:**
- `ASTNode` base class with line, column
- `Number` extends ASTNode with value field
- `BinaryOp` extends ASTNode with operator, left, right fields
- `Expr` type alias for Number | BinaryOp union

**Java Implementation:**
```java
package com.rpn2tex;

public interface Expr {
    int line();
    int column();
}

public final class Number implements Expr {
    public final int line;
    public final int column;
    public final String value;

    public Number(int line, int column, String value) {
        this.line = line;
        this.column = column;
        this.value = value;
    }

    @Override public int line() { return line; }
    @Override public int column() { return column; }
}

public final class BinaryOp implements Expr {
    public final int line;
    public final int column;
    public final String operator;  // "+", "-", "*", "/"
    public final Expr left;
    public final Expr right;

    public BinaryOp(int line, int column, String operator, Expr left, Expr right) {
        this.line = line;
        this.column = column;
        this.operator = operator;
        this.left = left;
        this.right = right;
    }

    @Override public int line() { return line; }
    @Override public int column() { return column; }
}
```

**Key Points:**
- Use interface Expr for polymorphism (simpler than sealed interface)
- Number.value is String to preserve input format (e.g., "3.14")
- Operators are strings, not enums
- All fields are public final

---

### Module 3: errors.py → RpnException.java

**Python Structure:**
- `ErrorFormatter` class with source text
- `format_error()` method creates error messages with source context

**Java Implementation:**
```java
package com.rpn2tex;

import java.util.*;

public class RpnException extends Exception {
    public final String message;
    public final int line;
    public final int column;

    public RpnException(String message, int line, int column) {
        super(message);
        this.message = message;
        this.line = line;
        this.column = column;
    }

    public String format(String source) {
        String[] lines = source.split("\n", -1);
        StringBuilder sb = new StringBuilder();

        sb.append("Error: ").append(message).append("\n\n");

        // Show source line with line number
        if (line >= 1 && line <= lines.length) {
            sb.append(line).append(" | ").append(lines[line - 1]).append("\n");

            // Add caret pointer
            sb.append("  | ");
            for (int i = 1; i < column; i++) {
                sb.append(" ");
            }
            sb.append("^");
        }

        return sb.toString();
    }
}
```

**Key Points:**
- Single exception class instead of separate Lexer/ParserError
- `format()` method generates error message with source context
- Line numbers are 1-based; convert to 0-based for array access
- Caret positioned at column (1-based)

---

### Module 4: lexer.py → Lexer.java

**Python Behavior:**
- Tokenizes input character by character
- Handles operators: +, -, *, /
- Handles numbers: integers and decimals
- Handles negative numbers: "-5" vs "-" operator
- Raises LexerError for unknown characters (like ^)

**Java Implementation:**
```java
package com.rpn2tex;

import java.util.*;

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

    public List<Token> tokenize() throws RpnException {
        List<Token> tokens = new ArrayList<>();

        while (!atEnd()) {
            skipWhitespace();
            if (atEnd()) break;
            tokens.add(scanToken());
        }

        tokens.add(new Token(TokenType.EOF, "", line, column));
        return tokens;
    }

    private boolean atEnd() {
        return pos >= text.length();
    }

    private char peek() {
        return atEnd() ? '\0' : text.charAt(pos);
    }

    private char advance() {
        char c = text.charAt(pos++);
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

    private Token scanToken() throws RpnException {
        int startLine = line;
        int startColumn = column;
        char c = peek();

        if (c == '+') {
            advance();
            return new Token(TokenType.PLUS, "+", startLine, startColumn);
        }
        if (c == '-') {
            advance();
            if (!atEnd() && Character.isDigit(peek())) {
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
        if (Character.isDigit(c)) {
            return scanNumber("", startLine, startColumn);
        }

        throw new RpnException("Unexpected character '" + c + "'", startLine, startColumn);
    }

    private Token scanNumber(String prefix, int startLine, int startColumn) {
        StringBuilder value = new StringBuilder(prefix);

        while (!atEnd() && Character.isDigit(peek())) {
            value.append(advance());
        }

        if (!atEnd() && peek() == '.') {
            value.append(advance());
            while (!atEnd() && Character.isDigit(peek())) {
                value.append(advance());
            }
        }

        return new Token(TokenType.NUMBER, value.toString(), startLine, startColumn);
    }
}
```

**Key Points:**
- Negative number detection: check if digit follows `-`
- Line/column tracking on every advance()
- Use Character.isDigit() and Character.isWhitespace()
- Throw RpnException for unknown characters

---

### Module 5: parser.py → Parser.java

**Python Behavior:**
- Stack-based RPN parsing
- Push numbers onto stack
- Pop two operands for operators (right then left)
- Final validation: exactly one value on stack

**Java Implementation:**
```java
package com.rpn2tex;

import java.util.*;

public class Parser {
    private final List<Token> tokens;
    private int pos;

    public Parser(List<Token> tokens) {
        this.tokens = tokens;
        this.pos = 0;
    }

    public Expr parse() throws RpnException {
        Stack<Expr> stack = new Stack<>();

        while (!atEnd()) {
            Token token = current();

            if (token.type == TokenType.NUMBER) {
                stack.push(new Number(token.line, token.column, token.value));
                advance();

            } else if (token.type == TokenType.PLUS || token.type == TokenType.MINUS ||
                       token.type == TokenType.MULT || token.type == TokenType.DIV) {

                if (stack.size() < 2) {
                    throw new RpnException(
                        "Operator '" + token.value + "' requires two operands",
                        token.line, token.column
                    );
                }

                Expr right = stack.pop();
                Expr left = stack.pop();
                String operator = tokenTypeToOperator(token.type);

                stack.push(new BinaryOp(token.line, token.column, operator, left, right));
                advance();

            } else if (token.type == TokenType.EOF) {
                break;
            }
        }

        if (stack.isEmpty()) {
            Token eofToken = tokens.get(tokens.size() - 1);
            throw new RpnException("Empty expression", eofToken.line, eofToken.column);
        }

        if (stack.size() > 1) {
            Token eofToken = tokens.get(tokens.size() - 1);
            throw new RpnException(
                "Invalid RPN: " + stack.size() + " values remain on stack (missing operators?)",
                eofToken.line, eofToken.column
            );
        }

        return stack.pop();
    }

    private Token current() {
        return tokens.get(pos);
    }

    private boolean atEnd() {
        return tokens.get(pos).type == TokenType.EOF;
    }

    private void advance() {
        if (!atEnd()) pos++;
    }

    private String tokenTypeToOperator(TokenType type) {
        switch (type) {
            case PLUS: return "+";
            case MINUS: return "-";
            case MULT: return "*";
            case DIV: return "/";
            default: throw new AssertionError("Invalid operator type");
        }
    }
}
```

**Key Points:**
- Use Stack<Expr> for RPN evaluation
- Pop order: right first, then left (critical for - and /)
- Validate stack size at end
- Map TokenType to operator string

---

### Module 6: latex_gen.py → LaTeXGenerator.java

**Python Behavior:**
- Visitor pattern for AST traversal
- Operator precedence: +/- = 1, */ = 2
- Parenthesization rules:
  - Lower precedence needs parens
  - Equal precedence on right needs parens for - and /

**Java Implementation:**
```java
package com.rpn2tex;

import java.util.*;

public class LaTeXGenerator {
    private static final Map<String, String> BINARY_OPS = Map.of(
        "+", "+",
        "-", "-",
        "*", "\\times",
        "/", "\\div"
    );

    private static final Map<String, Integer> PRECEDENCE = Map.of(
        "+", 1,
        "-", 1,
        "*", 2,
        "/", 2
    );

    public String generate(Expr ast) {
        return "$" + visit(ast) + "$";
    }

    private String visit(Expr node) {
        if (node instanceof Number) {
            return ((Number) node).value;
        } else if (node instanceof BinaryOp) {
            return visitBinaryOp((BinaryOp) node);
        }
        throw new AssertionError("Unknown node type");
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
        if (!(child instanceof BinaryOp)) {
            return false;
        }

        BinaryOp childOp = (BinaryOp) child;
        int childPrecedence = PRECEDENCE.get(childOp.operator);

        if (childPrecedence < parentPrecedence) {
            return true;
        }

        return childPrecedence == parentPrecedence && isRight &&
               (childOp.operator.equals("-") || childOp.operator.equals("/"));
    }
}
```

**Key Points:**
- Use `\\times` and `\\div` (double backslash in Java strings)
- Parentheses format: `"( " + expr + " )"` (spaces matter!)
- Only - and / need parens on right for equal precedence
- instanceof checks for visitor pattern

---

### Module 7: cli.py → Main.java

**Python Behavior:**
- Parse args: input file (or - for stdin), -o output file
- Read input, tokenize, parse, generate LaTeX
- Write output to file or stdout
- Format and print errors to stderr
- Return exit code: 0 success, 1 error

**Java Implementation:**
```java
package com.rpn2tex;

import java.io.*;
import java.nio.file.*;
import java.util.*;

public class Main {
    public static void main(String[] args) {
        System.exit(run(args));
    }

    public static int run(String[] args) {
        String input = null;
        String output = null;

        for (int i = 0; i < args.length; i++) {
            if (args[i].equals("-o") || args[i].equals("--output")) {
                if (i + 1 < args.length) {
                    output = args[++i];
                }
            } else if (!args[i].startsWith("-")) {
                input = args[i];
            }
        }

        if (input == null) {
            System.err.println("Error: Input file required");
            return 1;
        }

        String text;
        try {
            if (input.equals("-")) {
                text = readStdin();
            } else {
                text = Files.readString(Paths.get(input));
            }
        } catch (NoSuchFileException e) {
            System.err.println("Error: Input file not found: " + input);
            return 1;
        } catch (IOException e) {
            System.err.println("Error: " + e.getMessage());
            return 1;
        }

        String latex;
        try {
            Lexer lexer = new Lexer(text);
            List<Token> tokens = lexer.tokenize();

            Parser parser = new Parser(tokens);
            Expr ast = parser.parse();

            LaTeXGenerator generator = new LaTeXGenerator();
            latex = generator.generate(ast);

        } catch (RpnException e) {
            System.err.println(e.format(text));
            return 1;
        }

        if (output != null) {
            try {
                Files.writeString(Paths.get(output), latex + "\n");
                System.err.println("Generated: " + output);
            } catch (IOException e) {
                System.err.println("Error: " + e.getMessage());
                return 1;
            }
        } else {
            System.out.println(latex);
        }

        return 0;
    }

    private static String readStdin() throws IOException {
        StringBuilder sb = new StringBuilder();
        try (Scanner scanner = new Scanner(System.in)) {
            while (scanner.hasNextLine()) {
                sb.append(scanner.nextLine()).append("\n");
            }
        }
        String result = sb.toString();
        if (result.endsWith("\n")) {
            result = result.substring(0, result.length() - 1);
        }
        return result;
    }
}
```

**Key Points:**
- Use Files.readString() and Files.writeString() (Java 11+)
- Use Scanner for stdin reading
- Call e.format(text) for error messages
- Add newline when writing to file
- Print "Generated: ..." message to stderr

---

## Quality Gates

After each module migration:
1. `./gradlew compileJava` - Must compile without errors
2. `./gradlew checkstyleMain || true` - Check style (non-blocking)
3. `./gradlew test` - Run tests if available
4. **I/O Contract Validation** - Test outputs must match Python exactly

---

## Validation Strategy

For each test case in I/O contract:
```bash
echo "INPUT" | java -cp build/classes/java/main com.rpn2tex.Main -
```

Compare output to expected values. All 21 tests must pass with exact output matching.

---

## Critical Implementation Details

1. **Line/Column:** Always 1-based (never 0-based)
2. **Stack Pop Order:** Right first, then left (for - and /)
3. **Negative Numbers:** Check if digit follows `-` in lexer
4. **Parentheses:** Format is `"( " + expr + " )"` with spaces
5. **LaTeX Escaping:** Use `\\times` and `\\div` in Java strings
6. **Error Format:** "Error: message\n\nline | source\n  | ^^^^"
7. **Exit Codes:** 0 for success, 1 for any error
8. **Output Newline:** Add `\n` when writing to file

---

**End of Migration Specification**
