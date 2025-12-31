package com.rpn2tex;

/**
 * Base interface for all expression AST nodes.
 * <p>
 * This sealed interface represents all possible expression types
 * in the RPN language. All expression nodes carry position information
 * for error reporting.
 * </p>
 * <p>
 * Currently, the only permitted implementation is {@link NumberExpr}.
 * Future features will add {@link BinaryOpExpr}.
 * </p>
 */
public sealed interface Expr permits NumberExpr, BinaryOpExpr {
    /**
     * Returns the line number where this expression appears in the source.
     *
     * @return The 1-based line number
     */
    int line();

    /**
     * Returns the column number where this expression appears in the source.
     *
     * @return The 1-based column number
     */
    int column();
}
