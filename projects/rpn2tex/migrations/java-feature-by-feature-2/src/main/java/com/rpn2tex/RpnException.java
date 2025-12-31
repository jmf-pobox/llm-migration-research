package com.rpn2tex;

/**
 * Base exception for all RPN-to-LaTeX conversion errors.
 */
public class RpnException extends Exception {
    private final int line;
    private final int column;

    /**
     * Creates a new RPN exception.
     *
     * @param message Error message
     * @param line    1-based line number where error occurred
     * @param column  1-based column number where error occurred
     */
    public RpnException(String message, int line, int column) {
        super(message);
        this.line = line;
        this.column = column;
    }

    /**
     * Creates a new RPN exception from a token.
     *
     * @param message Error message
     * @param token   Token where error occurred
     */
    public RpnException(String message, Token token) {
        this(message, token.line(), token.column());
    }

    /**
     * Gets the line number where the error occurred.
     *
     * @return 1-based line number
     */
    public int getLine() {
        return line;
    }

    /**
     * Gets the column number where the error occurred.
     *
     * @return 1-based column number
     */
    public int getColumn() {
        return column;
    }
}
