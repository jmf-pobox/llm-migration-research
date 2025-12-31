package com.rpn2tex;

/**
 * AST node types representing parsed expression structure.
 *
 * <p>This sealed interface defines the Abstract Syntax Tree (AST) node hierarchy
 * for representing mathematical expressions in RPN (Reverse Polish Notation).
 * All implementations are immutable and include position tracking for error reporting.
 *
 * <p>The expression hierarchy consists of:
 * <ul>
 *   <li>{@link Number} - Numeric literal nodes</li>
 *   <li>{@link BinaryOp} - Binary operation nodes (e.g., addition, multiplication)</li>
 * </ul>
 *
 * <p>All expressions track their source position (1-based line and column numbers)
 * to enable precise error reporting. The immutable design prevents accidental
 * AST corruption during traversal and transformation.
 *
 * <h2>Example Usage</h2>
 * <pre>{@code
 * // Create a simple expression: 5 + 3
 * Expr five = new Number(1, 1, "5");
 * Expr three = new Number(1, 3, "3");
 * Expr addition = new BinaryOp(1, 5, "+", five, three);
 * }</pre>
 *
 * <h2>Sealed Interface</h2>
 * <p>This interface is sealed to ensure that only {@link Number} and {@link BinaryOp}
 * can implement it. This provides exhaustive pattern matching capabilities and
 * ensures type safety throughout the compilation pipeline.
 *
 * @see Number
 * @see BinaryOp
 */
public sealed interface Expr permits Number, BinaryOp {
    /**
     * Returns the 1-based line number where this expression appears in source.
     *
     * @return the line number (1-based)
     */
    int getLine();

    /**
     * Returns the 1-based column number where this expression appears in source.
     *
     * @return the column number (1-based)
     */
    int getColumn();
}
