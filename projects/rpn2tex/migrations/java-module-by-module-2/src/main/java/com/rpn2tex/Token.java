package com.rpn2tex;

import java.util.Objects;

/**
 * Represents an immutable lexical token in an RPN expression.
 *
 * <p>A token consists of:
 * <ul>
 *   <li>A {@link TokenType} indicating the kind of token</li>
 *   <li>A string value containing the actual text</li>
 *   <li>Position information (1-based line and column numbers) for error reporting</li>
 * </ul>
 *
 * <p>Example usage:
 * <pre>{@code
 * Token numberToken = new Token(TokenType.NUMBER, "42", 1, 5);
 * Token plusToken = new Token(TokenType.PLUS, "+", 1, 8);
 * }</pre>
 *
 * @param type   the token type
 * @param value  the string value of the token
 * @param line   the line number (1-based)
 * @param column the column number (1-based)
 * @since 1.0
 */
public record Token(TokenType type, String value, int line, int column) {

    /**
     * Compact constructor with validation.
     *
     * @throws NullPointerException if type or value is null
     * @throws IllegalArgumentException if line or column is less than 1
     */
    public Token {
        Objects.requireNonNull(type, "Token type cannot be null");
        Objects.requireNonNull(value, "Token value cannot be null");
        if (line < 1) {
            throw new IllegalArgumentException("Line number must be >= 1, got: " + line);
        }
        if (column < 1) {
            throw new IllegalArgumentException("Column number must be >= 1, got: " + column);
        }
    }

    /**
     * Returns a string representation of this token.
     *
     * <p>Format: {@code Token(TYPE, 'value', line:column)}
     *
     * @return a formatted string representation
     */
    @Override
    public String toString() {
        return String.format("Token(%s, '%s', %d:%d)", type.name(), value, line, column);
    }
}
