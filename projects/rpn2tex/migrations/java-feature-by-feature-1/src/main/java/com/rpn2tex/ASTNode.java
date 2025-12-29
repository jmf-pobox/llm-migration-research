package com.rpn2tex;

/**
 * Base interface for all Abstract Syntax Tree nodes.
 *
 * <p>All AST nodes track their position in source for error reporting.
 * Nodes are immutable value objects that represent the structure of
 * mathematical expressions.
 *
 * <p>Current implementations:
 * <ul>
 *   <li>{@link NumberNode} - Numeric literals</li>
 *   <li>{@link BinaryOpNode} - Binary operations (+, -, *, /)</li>
 * </ul>
 */
public sealed interface ASTNode permits NumberNode, BinaryOpNode {

    /**
     * Returns the line number where this node appears in source.
     *
     * @return line number (1-based)
     */
    int line();

    /**
     * Returns the column number where this node starts in source.
     *
     * @return column number (1-based)
     */
    int column();
}
