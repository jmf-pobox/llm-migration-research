package com.rpn2tex;

/**
 * Sealed interface representing an expression in the AST.
 * This is the base type for all expression nodes in the RPN to LaTeX converter.
 *
 * <p>The AST has two concrete implementations:</p>
 * <ul>
 *   <li>{@link Number} - represents numeric literals</li>
 *   <li>{@link BinaryOp} - represents binary operations (+, -, *, /)</li>
 * </ul>
 *
 * <p>All expressions track their source position (line and column) for error reporting.</p>
 */
public sealed interface Expr permits Number, BinaryOp {

    /**
     * Returns the line number where this expression appears in the source.
     * Line numbers are 1-based.
     *
     * @return the line number (1-based)
     */
    int line();

    /**
     * Returns the column number where this expression appears in the source.
     * Column numbers are 1-based.
     *
     * @return the column number (1-based)
     */
    int column();
}
