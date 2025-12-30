package com.rpn2tex;

/**
 * Enumeration of token types recognized by the lexer.
 *
 * <p>This enum defines all possible token types that can appear in an RPN expression:
 * <ul>
 *   <li>NUMBER - numeric literals (integers and decimals)</li>
 *   <li>PLUS - addition operator (+)</li>
 *   <li>MINUS - subtraction operator (-)</li>
 *   <li>MULT - multiplication operator (*)</li>
 *   <li>DIV - division operator (/)</li>
 *   <li>EOF - end of input marker</li>
 * </ul>
 */
public enum TokenType {
    /**
     * Numeric literal token (e.g., "42", "3.14").
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
     * End of input marker token.
     */
    EOF
}
