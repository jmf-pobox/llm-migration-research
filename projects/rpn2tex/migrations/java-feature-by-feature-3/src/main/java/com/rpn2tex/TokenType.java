package com.rpn2tex;

/**
 * Enumeration of token types in the RPN expression language.
 * <p>
 * This enum represents all possible token types that can be
 * recognized by the lexer during tokenization.
 * </p>
 */
public enum TokenType {
    /**
     * Numeric literal token (integers and decimals).
     */
    NUMBER,

    /**
     * Addition operator (+).
     */
    PLUS,

    /**
     * Subtraction operator (-).
     */
    MINUS,

    /**
     * Multiplication operator (*).
     */
    TIMES,

    /**
     * Division operator (/).
     */
    DIVIDE,

    /**
     * End of file/input marker.
     */
    EOF
}
