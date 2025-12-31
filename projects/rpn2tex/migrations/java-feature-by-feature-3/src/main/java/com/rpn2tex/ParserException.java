package com.rpn2tex;

/**
 * Exception thrown during parsing.
 * <p>
 * This exception is thrown when the parser encounters syntax errors
 * or invalid token sequences in the input.
 * </p>
 */
public class ParserException extends RpnException {
    private final Token token;

    /**
     * Constructs a new ParserException with the specified message and token.
     *
     * @param message The error message
     * @param token   The token where the error occurred
     */
    public ParserException(String message, Token token) {
        super(message, token.line(), token.column());
        this.token = token;
    }

    /**
     * Returns the token where the error occurred.
     *
     * @return The error token
     */
    public Token getToken() {
        return token;
    }
}
