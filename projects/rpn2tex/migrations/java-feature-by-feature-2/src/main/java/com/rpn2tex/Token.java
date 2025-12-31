package com.rpn2tex;

/**
 * Immutable token produced by the lexer.
 *
 * @param type   The type of this token
 * @param value  The string representation of the token
 * @param line   1-based line number in source
 * @param column 1-based column number in source
 */
public record Token(TokenType type, String value, int line, int column) {
}
