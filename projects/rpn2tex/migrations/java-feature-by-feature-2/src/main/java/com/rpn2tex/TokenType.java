package com.rpn2tex;

/**
 * Types of tokens recognized by the lexer.
 * Supports numeric literals, operators, and EOF marker.
 */
public enum TokenType {
    /**
     * Numeric literal (integer or floating point).
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
    MULTIPLY,

    /**
     * Division operator (/).
     */
    DIVIDE,

    /**
     * End of file marker.
     */
    EOF
}
