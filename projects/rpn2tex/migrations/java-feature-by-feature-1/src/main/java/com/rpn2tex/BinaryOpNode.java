package com.rpn2tex;

import java.util.Objects;

/**
 * AST node representing a binary operation.
 *
 * <p>Represents operations with two operands: +, -, *, /
 *
 * <p>The node is immutable and preserves the exact operator string
 * for accurate LaTeX generation.
 *
 * <p>Example:
 * <pre>
 *   // Represents "5 + 3"
 *   BinaryOpNode node = new BinaryOpNode(
 *       1, 3,
 *       "+",
 *       new NumberNode(1, 1, "5"),
 *       new NumberNode(1, 3, "3")
 *   );
 * </pre>
 *
 * @param line line number (1-based) where operator appears
 * @param column column number (1-based) where operator starts
 * @param operator the operator string ("+", "-", "*", "/")
 * @param left the left operand expression
 * @param right the right operand expression
 */
public record BinaryOpNode(
    int line,
    int column,
    String operator,
    ASTNode left,
    ASTNode right
) implements ASTNode {

    /**
     * Creates a new binary operation node.
     *
     * @param line line number (1-based) where operator appears
     * @param column column number (1-based) where operator starts
     * @param operator the operator string ("+", "-", "*", "/")
     * @param left the left operand expression
     * @param right the right operand expression
     * @throws NullPointerException if operator, left, or right is null
     */
    public BinaryOpNode {
        Objects.requireNonNull(operator, "operator must not be null");
        Objects.requireNonNull(left, "left must not be null");
        Objects.requireNonNull(right, "right must not be null");
    }
}
