package com.rpn2tex;

import java.util.Objects;

/**
 * Represents a token in the RPN expression language.
 * <p>
 * Tokens are immutable and carry position information for error reporting.
 * </p>
 *
 * @param type   The type of the token
 * @param value  The string value of the token
 * @param line   The 1-based line number where the token appears
 * @param column The 1-based column number where the token appears
 */
public record Token(TokenType type, String value, int line, int column) {
    /**
     * Constructs a new Token with validation.
     *
     * @param type   The type of the token
     * @param value  The string value of the token
     * @param line   The 1-based line number where the token appears
     * @param column The 1-based column number where the token appears
     * @throws NullPointerException if type or value is null
     */
    public Token {
        Objects.requireNonNull(type, "Token type cannot be null");
        Objects.requireNonNull(value, "Token value cannot be null");
    }

    @Override
    public String toString() {
        return String.format("Token(%s, '%s', %d:%d)", type, value, line, column);
    }
}
