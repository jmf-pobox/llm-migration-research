package com.rpn2tex;

/**
 * Enumeration of all token types in the RPN expression language.
 *
 * <p>This enum defines the lexical elements that can appear in an RPN expression:
 * <ul>
 *   <li>{@link #NUMBER} - Numeric literals (integers or decimals)</li>
 *   <li>{@link #PLUS} - Addition operator (+)</li>
 *   <li>{@link #MINUS} - Subtraction operator (-)</li>
 *   <li>{@link #MULT} - Multiplication operator (*)</li>
 *   <li>{@link #DIV} - Division operator (/)</li>
 *   <li>{@link #EOF} - End of file marker</li>
 * </ul>
 *
 * @since 1.0.0
 */
public enum TokenType {
    /**
     * Numeric literal token (e.g., "5", "3.14", "-2").
     */
    NUMBER,

    /**
     * Addition operator token (+).
     */
    PLUS,

    /**
     * Subtraction operator token (-).
     */
    MINUS,

    /**
     * Multiplication operator token (*).
     */
    MULT,

    /**
     * Division operator token (/).
     */
    DIV,

    /**
     * End of file marker token.
     */
    EOF
}
