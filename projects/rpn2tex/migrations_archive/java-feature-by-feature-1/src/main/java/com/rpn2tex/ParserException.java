package com.rpn2tex;

/**
 * Exception thrown when the parser encounters invalid input.
 *
 * <p>Examples:
 * <ul>
 *   <li>Insufficient operands for an operator</li>
 *   <li>Extra operands remaining after parsing</li>
 *   <li>Empty expressions</li>
 * </ul>
 */
public class ParserException extends RpnException {
    private final Token token;

    /**
     * Creates a new parser exception.
     *
     * @param message description of the error
     * @param token the token where error occurred
     */
    public ParserException(String message, Token token) {
        super(message, token.line(), token.column());
        this.token = token;
    }

    /**
     * Returns the token where the error occurred.
     *
     * @return the error token
     */
    public Token getToken() {
        return token;
    }
}
