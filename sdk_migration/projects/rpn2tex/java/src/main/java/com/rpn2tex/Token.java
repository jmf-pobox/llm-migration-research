package com.rpn2tex;

import java.util.Objects;

/**
 * Represents a token in an RPN expression with position tracking.
 *
 * <p>A token is an immutable value object that contains:
 * <ul>
 *   <li>type - the kind of token (NUMBER, PLUS, MINUS, etc.)</li>
 *   <li>value - the string representation (e.g., "42", "+", "")</li>
 *   <li>line - 1-based line number in source</li>
 *   <li>column - 1-based column number in source</li>
 * </ul>
 *
 * <p>Example usage:
 * <pre>{@code
 * Token numberToken = new Token(TokenType.NUMBER, "42", 1, 1);
 * Token plusToken = new Token(TokenType.PLUS, "+", 1, 4);
 * Token eofToken = new Token(TokenType.EOF, "", 1, 5);
 * }</pre>
 */
public final class Token {
    private final TokenType type;
    private final String value;
    private final int line;
    private final int column;

    /**
     * Constructs a new Token.
     *
     * @param type the type of token
     * @param value the string value of the token
     * @param line the 1-based line number where the token appears
     * @param column the 1-based column number where the token appears
     * @throws NullPointerException if type or value is null
     * @throws IllegalArgumentException if line or column is less than 1
     */
    public Token(TokenType type, String value, int line, int column) {
        this.type = Objects.requireNonNull(type, "type must not be null");
        this.value = Objects.requireNonNull(value, "value must not be null");

        if (line < 1) {
            throw new IllegalArgumentException("line must be >= 1, got: " + line);
        }
        if (column < 1) {
            throw new IllegalArgumentException("column must be >= 1, got: " + column);
        }

        this.line = line;
        this.column = column;
    }

    /**
     * Returns the type of this token.
     *
     * @return the token type
     */
    public TokenType type() {
        return type;
    }

    /**
     * Returns the string value of this token.
     *
     * @return the token value
     */
    public String value() {
        return value;
    }

    /**
     * Returns the 1-based line number where this token appears.
     *
     * @return the line number
     */
    public int line() {
        return line;
    }

    /**
     * Returns the 1-based column number where this token appears.
     *
     * @return the column number
     */
    public int column() {
        return column;
    }

    /**
     * Returns a string representation of this token.
     *
     * <p>Format: {@code Token(type=TYPE, value='VALUE', line=N, column=M)}
     *
     * @return string representation of this token
     */
    @Override
    public String toString() {
        return String.format("Token(type=%s, value='%s', line=%d, column=%d)",
                type, value, line, column);
    }

    /**
     * Checks equality based on all fields.
     *
     * @param obj the object to compare
     * @return true if equal, false otherwise
     */
    @Override
    public boolean equals(Object obj) {
        if (this == obj) {
            return true;
        }
        if (obj == null || getClass() != obj.getClass()) {
            return false;
        }
        Token other = (Token) obj;
        return type == other.type
                && value.equals(other.value)
                && line == other.line
                && column == other.column;
    }

    /**
     * Returns hash code based on all fields.
     *
     * @return the hash code
     */
    @Override
    public int hashCode() {
        return Objects.hash(type, value, line, column);
    }
}
