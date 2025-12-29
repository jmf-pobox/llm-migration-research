package com.rpn2tex;

import java.util.ArrayList;
import java.util.List;

/**
 * Lexer for RPN expressions.
 *
 * <p>The lexer converts input text into a sequence of tokens. It handles:
 * <ul>
 *   <li>Operators: +, -, *, /</li>
 *   <li>Numbers: integers and decimals (e.g., 42, 3.14)</li>
 *   <li>Negative numbers: distinguishes "-5" from "-" operator</li>
 *   <li>Whitespace: automatically skipped</li>
 * </ul>
 *
 * <p>Example usage:
 * <pre>
 * Lexer lexer = new Lexer("5 3 +");
 * List&lt;Token&gt; tokens = lexer.tokenize();
 * // tokens: [NUMBER("5"), NUMBER("3"), PLUS("+"), EOF]
 * </pre>
 *
 * @throws RpnException if an unexpected character is encountered
 */
public class Lexer {
    private final String text;
    private int pos;
    private int line;
    private int column;

    /**
     * Creates a new lexer for the given input text.
     *
     * @param text the input text to tokenize
     */
    public Lexer(String text) {
        this.text = text;
        this.pos = 0;
        this.line = 1;
        this.column = 1;
    }

    /**
     * Tokenizes the input text into a list of tokens.
     *
     * <p>The returned list always ends with an EOF token.
     *
     * @return a list of tokens
     * @throws RpnException if an unexpected character is encountered
     */
    public List<Token> tokenize() throws RpnException {
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
