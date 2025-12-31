package com.rpn2tex;

/**
 * Marker interface for expression AST nodes.
 * Implemented by all nodes that represent expressions (numbers, operations).
 */
public sealed interface Expr permits Number, BinaryOp {
    /**
     * Gets the line number where this expression starts.
     *
     * @return 1-based line number
     */
    int line();

    /**
     * Gets the column number where this expression starts.
     *
     * @return 1-based column number
     */
    int column();
}
