package com.rpn2tex;

import java.util.ArrayList;
import java.util.List;

/**
 * Lexer for RPN expressions.
 * Converts input text into a stream of tokens.
 */
public class Lexer {
    private final String text;
    private int pos;
    private int line;
    private int column;

    /**
     * Creates a new lexer for the given input text.
     *
     * @param text Input text to tokenize
     */
    public Lexer(String text) {
        this.text = text;
        this.pos = 0;
        this.line = 1;
        this.column = 1;
    }

    /**
     * Tokenizes the entire input text.
     *
     * @return List of tokens, ending with EOF token
     * @throws RpnException If an invalid character is encountered
     */
    public List<Token> tokenize() throws RpnException {
        List<Token> tokens = new ArrayList<>();

        while (!atEnd()) {
            skipWhitespace();
            if (atEnd()) {
                break;
            }

            Token token = scanToken();
            tokens.add(token);
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

    private Token scanToken() throws RpnException {
        int startLine = line;
        int startColumn = column;
        char c = peek();

        if (Character.isDigit(c)) {
            return scanNumber("", startLine, startColumn);
        }

        if (c == '+') {
            advance();
            return new Token(TokenType.PLUS, "+", startLine, startColumn);
        }

        if (c == '-') {
            advance();
            // Check if this is a negative number (digit follows immediately)
            if (!atEnd() && Character.isDigit(peek())) {
                // It's a negative number
                return scanNumber("-", startLine, startColumn);
            }
            // It's a subtraction operator
            return new Token(TokenType.MINUS, "-", startLine, startColumn);
        }

        if (c == '*') {
            advance();
            return new Token(TokenType.MULTIPLY, "*", startLine, startColumn);
        }

        if (c == '/') {
            advance();
            return new Token(TokenType.DIVIDE, "/", startLine, startColumn);
        }

        throw new RpnException("Unexpected character '" + c + "'", startLine, startColumn);
    }

    private Token scanNumber(String prefix, int startLine, int startColumn) {
        StringBuilder value = new StringBuilder(prefix);

        // Consume integer part
        while (!atEnd() && Character.isDigit(peek())) {
            value.append(advance());
        }

        // Check for decimal part
        if (!atEnd() && peek() == '.') {
            value.append(advance());

            // Consume decimal digits
            while (!atEnd() && Character.isDigit(peek())) {
                value.append(advance());
            }
        }

        return new Token(TokenType.NUMBER, value.toString(), startLine, startColumn);
    }
}
