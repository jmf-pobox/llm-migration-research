package com.rpn2tex;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

/**
 * Tokenizes RPN input text.
 *
 * <p>The lexer scans input character by character, producing tokens for:
 * <ul>
 *   <li>Numbers (integers and decimals, including negative numbers)</li>
 *   <li>Operators (+, -, *, /)</li>
 *   <li>EOF marker</li>
 * </ul>
 *
 * <p>Whitespace is used as a delimiter and is otherwise ignored.
 *
 * <p>The '-' character is disambiguated using lookahead:
 * <ul>
 *   <li>If followed immediately by a digit: negative number (NUMBER token)</li>
 *   <li>Otherwise: subtraction operator (MINUS token)</li>
 * </ul>
 *
 * <p>Example usage:
 * <pre>
 *   Lexer lexer = new Lexer("5 3 -");
 *   List&lt;Token&gt; tokens = lexer.tokenize();
 *   // Returns: [Token(NUMBER, '5', 1:1), Token(NUMBER, '3', 1:3), Token(MINUS, '-', 1:5), Token(EOF, '', 1:6)]
 * </pre>
 */
public class Lexer {
    private final String text;
    private int pos;
    private int line;
    private int column;

    /**
     * Creates a new lexer for the given input text.
     *
     * @param text the RPN expression to tokenize
     * @throws NullPointerException if text is null
     */
    public Lexer(String text) {
        this.text = Objects.requireNonNull(text, "text must not be null");
        this.pos = 0;
        this.line = 1;
        this.column = 1;
    }

    /**
     * Tokenizes the entire input text.
     *
     * @return list of tokens, ending with EOF token
     * @throws LexerException if an invalid character is encountered
     */
    public List<Token> tokenize() throws LexerException {
        List<Token> tokens = new ArrayList<>();

        while (!atEnd()) {
            skipWhitespace();
            if (atEnd()) {
                break;
            }
            tokens.add(scanToken());
        }

        // Add EOF token
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
        char ch = text.charAt(pos);
        pos++;
        if (ch == '\n') {
            line++;
            column = 1;
        } else {
            column++;
        }
        return ch;
    }

    private void skipWhitespace() {
        while (!atEnd() && isWhitespace(peek())) {
            advance();
        }
    }

    private boolean isWhitespace(char ch) {
        return ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r';
    }

    private Token scanToken() throws LexerException {
        int startLine = line;
        int startColumn = column;
        char ch = peek();

        // Single-character operators
        if (ch == '+') {
            advance();
            return new Token(TokenType.PLUS, "+", startLine, startColumn);
        }

        if (ch == '*') {
            advance();
            return new Token(TokenType.MULT, "*", startLine, startColumn);
        }

        if (ch == '/') {
            advance();
            return new Token(TokenType.DIV, "/", startLine, startColumn);
        }

        // Handle minus: distinguish between negative number and subtraction operator
        if (ch == '-') {
            advance();
            // If followed immediately by a digit, it's a negative number
            if (!atEnd() && Character.isDigit(peek())) {
                return scanNumber("-", startLine, startColumn);
            }
            // Otherwise, it's a subtraction operator
            return new Token(TokenType.MINUS, "-", startLine, startColumn);
        }

        // Numbers
        if (Character.isDigit(ch)) {
            return scanNumber("", startLine, startColumn);
        }

        // Unknown character
        throw new LexerException(
            String.format("Unexpected character '%c'", ch),
            startLine,
            startColumn
        );
    }

    private Token scanNumber(String prefix, int startLine, int startColumn) {
        StringBuilder value = new StringBuilder(prefix);

        // Integer part
        while (!atEnd() && Character.isDigit(peek())) {
            value.append(advance());
        }

        // Decimal part (optional)
        if (!atEnd() && peek() == '.') {
            value.append(advance()); // consume '.'
            while (!atEnd() && Character.isDigit(peek())) {
                value.append(advance());
            }
        }

        return new Token(TokenType.NUMBER, value.toString(), startLine, startColumn);
    }
}
