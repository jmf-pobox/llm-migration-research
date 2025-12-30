package com.rpn2tex;

/**
 * Token types for RPN expression lexer.
 *
 * <p>Represents the different types of tokens that can appear in an RPN expression:
 * <ul>
 *   <li>NUMBER - numeric literals (integers and decimals)</li>
 *   <li>PLUS - addition operator (+)</li>
 *   <li>MINUS - subtraction operator (-)</li>
 *   <li>MULT - multiplication operator (*)</li>
 *   <li>DIV - division operator (/)</li>
 *   <li>EOF - end of file marker</li>
 * </ul>
 */
public enum TokenType {
    NUMBER, PLUS, MINUS, MULT, DIV, EOF
}
