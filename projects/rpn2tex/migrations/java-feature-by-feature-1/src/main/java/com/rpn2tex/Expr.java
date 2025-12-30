package com.rpn2tex;

/**
 * Base interface for all AST expression nodes in the RPN2TeX compiler.
 *
 * <p>Supported expression types:
 * <ul>
 *   <li>{@link Number} - numeric literals</li>
 *   <li>{@link BinaryOp} - binary operations (addition, subtraction, etc.)</li>
 * </ul>
 *
 * <p>All expression nodes are immutable and include source position tracking
 * (line and column) for error reporting.
 *
 * <p>Example usage:
 * <pre>
 *     Expr num = new Number("42", 1, 1);
 *     Expr sum = new BinaryOp("+", num, num, 1, 5);
 * </pre>
 *
 * @see Number
 * @see BinaryOp
 */
public sealed interface Expr permits Number, BinaryOp {
    /**
     * Gets the line number where this expression appears in the source.
     *
     * @return 1-based line number
     */
    int line();

    /**
     * Gets the column number where this expression appears in the source.
     *
     * @return 1-based column number
     */
    int column();
}
