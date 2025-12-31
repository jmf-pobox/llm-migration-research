package com.rpn2tex;

import java.util.Objects;

/**
 * Immutable lexical token representing a single element in an RPN expression.
 *
 * <p>A token consists of:
 * <ul>
 *   <li>A {@link TokenType} indicating the category of the token</li>
 *   <li>A string value containing the exact lexeme from the source</li>
 *   <li>A 1-based line number for error reporting</li>
 *   <li>A 1-based column number for error reporting</li>
 * </ul>
 *
 * <p>Position tracking enables precise error messages that point to the exact
 * location of issues in the source text. All position values use 1-based indexing
 * to match common editor conventions.
 *
 * <p><b>Examples:</b>
 * <pre>{@code
 * // Number token at position (1, 1) with value "5"
 * Token number = new Token(TokenType.NUMBER, "5", 1, 1);
 *
 * // Addition operator at position (1, 3)
 * Token plus = new Token(TokenType.PLUS, "+", 1, 3);
 *
 * // EOF token at end of input
 * Token eof = new Token(TokenType.EOF, "", 1, 10);
 * }</pre>
 *
 * @param type   the token type (must not be null)
 * @param value  the string value from the source (must not be null)
 * @param line   the 1-based line number where the token appears
 * @param column the 1-based column number where the token starts
 * @since 1.0.0
 */
public record Token(
    TokenType type,
    String value,
    int line,
    int column
) {
    /**
     * Compact constructor with validation.
     *
     * @throws NullPointerException if type or value is null
     */
    public Token {
        Objects.requireNonNull(type, "type must not be null");
        Objects.requireNonNull(value, "value must not be null");
    }

    /**
     * Returns a string representation of this token.
     *
     * <p>The format is: {@code Token(TYPE, "value", line:column)}
     *
     * <p><b>Example:</b> {@code Token(NUMBER, "5", 1:1)}
     *
     * @return a string representation of this token
     */
    @Override
    public String toString() {
        return String.format("Token(%s, \"%s\", %d:%d)", type, value, line, column);
    }
}
