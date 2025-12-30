package com.rpn2tex;

/**
 * Token types for RPN expression lexer.
 *
 * <p>Supported token types:
 * <ul>
 *   <li>NUMBER - numeric literals (integers and decimals)</li>
 *   <li>PLUS - addition operator (+)</li>
 *   <li>MINUS - subtraction operator (-)</li>
 *   <li>STAR - multiplication operator (*)</li>
 *   <li>SLASH - division operator (/)</li>
 *   <li>EOF - end of file marker</li>
 * </ul>
 */
public enum TokenType {
    NUMBER,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    EOF
}
