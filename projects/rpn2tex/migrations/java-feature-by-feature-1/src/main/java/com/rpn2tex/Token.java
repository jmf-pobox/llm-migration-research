package com.rpn2tex;

import java.util.Objects;

/**
 * A lexical token with type, value, and position.
 *
 * <p>Tokens are immutable value objects that represent lexical elements
 * scanned from the input text. Each token tracks its position for
 * error reporting.
 *
 * <p>Example:
 * <pre>
 *   Token token = new Token(TokenType.NUMBER, "42", 1, 5);
 * </pre>
 *
 * @param type the token type (from TokenType enum)
 * @param value the string value of the token
 * @param line line number (1-based) where token appears
 * @param column column number (1-based) where token starts
 */
public record Token(TokenType type, String value, int line, int column) {

    /**
     * Creates a new token.
     *
     * @param type the token type (from TokenType enum)
     * @param value the string value of the token
     * @param line line number (1-based) where token appears
     * @param column column number (1-based) where token starts
     * @throws NullPointerException if type or value is null
     */
    public Token {
        Objects.requireNonNull(type, "type must not be null");
        Objects.requireNonNull(value, "value must not be null");
    }

    @Override
    public String toString() {
        return String.format("Token(%s, '%s', %d:%d)", type, value, line, column);
    }
}
