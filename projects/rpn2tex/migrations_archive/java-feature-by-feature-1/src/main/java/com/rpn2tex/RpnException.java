package com.rpn2tex;

/**
 * Base exception for all rpn2tex errors.
 *
 * <p>Provides position tracking (line and column) for error reporting.
 * Subclasses handle specific error categories like lexer and parser errors.
 */
public class RpnException extends Exception {
    private final int line;
    private final int column;

    /**
     * Creates a new exception with message and position.
     *
     * @param message description of the error
     * @param line line number where error occurred (1-based)
     * @param column column number where error occurred (1-based)
     */
    public RpnException(String message, int line, int column) {
        super(String.format("Line %d, column %d: %s", line, column, message));
        this.line = line;
        this.column = column;
    }

    /**
     * Returns the line number where the error occurred.
     *
     * @return line number (1-based)
     */
    public int getLine() {
        return line;
    }

    /**
     * Returns the column number where the error occurred.
     *
     * @return column number (1-based)
     */
    public int getColumn() {
        return column;
    }
}
