package com.rpn2tex;

/**
 * Exception thrown during lexical analysis (tokenization).
 * <p>
 * This exception is thrown when the lexer encounters invalid
 * characters or malformed tokens in the input.
 * </p>
 */
public class LexerException extends RpnException {
    /**
     * Constructs a new LexerException with the specified message and position.
     *
     * @param message The error message
     * @param line    The 1-based line number where the error occurred
     * @param column  The 1-based column number where the error occurred
     */
    public LexerException(String message, int line, int column) {
        super(message, line, column);
    }
}
