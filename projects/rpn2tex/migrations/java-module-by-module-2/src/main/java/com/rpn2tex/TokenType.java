package com.rpn2tex;

/**
 * Enumeration of token types recognized by the rpn2tex lexer.
 *
 * <p>This enum defines all possible lexical units (tokens) that can appear
 * in an RPN expression input.</p>
 *
 * @since 1.0
 */
public enum TokenType {
    /** Numeric literal (integers and decimals). */
    NUMBER,

    /** Addition operator (+). */
    PLUS,

    /** Subtraction operator (-). */
    MINUS,

    /** Multiplication operator (*). */
    MULT,

    /** Division operator (/). */
    DIV,

    /** End of file marker. */
    EOF
}
