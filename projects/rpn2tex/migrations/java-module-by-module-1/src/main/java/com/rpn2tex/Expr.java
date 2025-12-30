package com.rpn2tex;

/**
 * Base interface for all AST expression nodes in the RPN2TeX compiler.
 *
 * <p>This sealed interface represents the union type of all expression nodes
 * that can appear in the Abstract Syntax Tree. It ensures type safety by
 * restricting implementations to only Number and BinaryOp.
 *
 * <p>All expression nodes are immutable and include source position tracking
 * (line and column) for error reporting.
 *
 * <p>Example usage:
 * <pre>
 *     Expr num = new Number("42", 1, 1);
 *     Expr add = new BinaryOp("+", num, num, 1, 3);
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
