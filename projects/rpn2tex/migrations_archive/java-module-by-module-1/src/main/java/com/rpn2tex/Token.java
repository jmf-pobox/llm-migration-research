package com.rpn2tex;

/**
 * Represents a token in an RPN expression.
 *
 * <p>Tokens are immutable objects that represent the smallest units of an RPN expression.
 * Each token has a type, optional value, and source location (line and column).
 *
 * <p>Example usage:
 * <pre>
 * Token numberToken = new Token(TokenType.NUMBER, "42", 1, 1);
 * Token plusToken = new Token(TokenType.PLUS, "+", 1, 4);
 * </pre>
 */
public final class Token {
    /** The type of this token. */
    public final TokenType type;

    /** The string value of this token (e.g., "42", "+"). */
    public final String value;

    /** The 1-based line number where this token appears. */
    public final int line;

    /** The 1-based column number where this token appears. */
    public final int column;

    /**
     * Creates a new token.
     *
     * @param type the type of token
     * @param value the string value of the token
     * @param line the 1-based line number
     * @param column the 1-based column number
     */
    public Token(TokenType type, String value, int line, int column) {
        this.type = type;
        this.value = value;
        this.line = line;
        this.column = column;
    }

    /**
     * Returns a string representation of this token for debugging.
     *
     * @return a string in the format "Token(TYPE, 'value', line:column)"
     */
    @Override
    public String toString() {
        return String.format("Token(%s, '%s', %d:%d)",
            type.name(), value, line, column);
    }
}
