package com.rpn2tex;

/**
 * Token types recognized by the RPN lexer.
 *
 * <p>Each token type represents a distinct lexical element in RPN expressions.
 *
 * <p>Currently supports:
 * <ul>
 *   <li>NUMBER - Numeric literals (integers and decimals)</li>
 *   <li>PLUS - Addition operator (+)</li>
 *   <li>MINUS - Subtraction operator (-)</li>
 *   <li>MULT - Multiplication operator (*)</li>
 *   <li>DIV - Division operator (/)</li>
 *   <li>EOF - End of input marker</li>
 * </ul>
 */
public enum TokenType {
    /** Numeric values: 5, 3.14, -2. */
    NUMBER,

    /** Addition operator: +. */
    PLUS,

    /** Subtraction operator: -. */
    MINUS,

    /** Multiplication operator: *. */
    MULT,

    /** Division operator: /. */
    DIV,

    /** End of input. */
    EOF
}
