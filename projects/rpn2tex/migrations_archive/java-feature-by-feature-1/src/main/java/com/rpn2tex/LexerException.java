package com.rpn2tex;

/**
 * Exception thrown when the lexer encounters invalid input.
 *
 * <p>Examples:
 * <ul>
 *   <li>Unexpected characters</li>
 *   <li>Malformed tokens</li>
 * </ul>
 */
public class LexerException extends RpnException {

    /**
     * Creates a new lexer exception.
     *
     * @param message description of the error
     * @param line line number where error occurred (1-based)
     * @param column column number where error occurred (1-based)
     */
    public LexerException(String message, int line, int column) {
        super(message, line, column);
    }
}
