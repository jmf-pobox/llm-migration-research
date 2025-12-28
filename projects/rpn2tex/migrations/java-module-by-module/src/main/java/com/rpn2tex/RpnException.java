package com.rpn2tex;

/**
 * Base exception class for RPN parsing and lexing errors.
 *
 * <p>This exception includes position information (line and column) to help
 * users locate the source of errors in their RPN expressions.
 *
 * <p>Example usage:
 * <pre>{@code
 * throw new RpnException("Unexpected character: ^", 1, 5);
 * }</pre>
 */
public class RpnException extends Exception {
    private final int line;
    private final int column;

    /**
     * Constructs a new RPN exception with the specified message and position.
     *
     * @param message the detail message describing the error
     * @param line the 1-based line number where the error occurred
     * @param column the 1-based column number where the error occurred
     */
    public RpnException(String message, int line, int column) {
        super(message);
        if (line < 1) {
            throw new IllegalArgumentException("Line must be >= 1");
        }
        if (column < 1) {
            throw new IllegalArgumentException("Column must be >= 1");
        }
        this.line = line;
        this.column = column;
    }

    /**
     * Returns the 1-based line number where the error occurred.
     *
     * @return the line number
     */
    public int getLine() {
        return line;
    }

    /**
     * Returns the 1-based column number where the error occurred.
     *
     * @return the column number
     */
    public int getColumn() {
        return column;
    }

    @Override
    public String toString() {
        return String.format("RpnException: %s at line %d, column %d",
            getMessage(), line, column);
    }
}
