package com.rpn2tex;

/**
 * Base exception class for all RPN processing errors.
 * <p>
 * This serves as the parent class for more specific exception types
 * like LexerException and ParserException.
 * </p>
 */
public class RpnException extends Exception {
    private final int line;
    private final int column;

    /**
     * Constructs a new RpnException with the specified message and position.
     *
     * @param message The error message
     * @param line    The 1-based line number where the error occurred
     * @param column  The 1-based column number where the error occurred
     */
    public RpnException(String message, int line, int column) {
        super(message);
        this.line = line;
        this.column = column;
    }

    /**
     * Returns the line number where the error occurred.
     *
     * @return The 1-based line number
     */
    public int getLine() {
        return line;
    }

    /**
     * Returns the column number where the error occurred.
     *
     * @return The 1-based column number
     */
    public int getColumn() {
        return column;
    }

    @Override
    public String toString() {
        return String.format("%s at line %d, column %d: %s",
                getClass().getSimpleName(), line, column, getMessage());
    }
}
