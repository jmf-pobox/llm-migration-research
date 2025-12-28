package com.rpn2tex;

import java.util.Objects;

/**
 * Represents a binary operation in the AST.
 *
 * <p>Binary operations have an operator and two child expressions (left and right).
 * The operator is represented as a string: "+", "-", "*", or "/".</p>
 *
 * <p>This class creates a recursive tree structure where children can be either
 * {@link Number} nodes or other {@link BinaryOp} nodes.</p>
 *
 * <p>Example usage:</p>
 * <pre>{@code
 * // Represents "5 + 3"
 * Expr left = new Number(1, 1, "5");
 * Expr right = new Number(1, 3, "3");
 * BinaryOp add = new BinaryOp(1, 3, "+", left, right);
 * }</pre>
 *
 * @param line the line number where this operation appears (1-based)
 * @param column the column number where this operation appears (1-based)
 * @param operator the operator string ("+", "-", "*", or "/")
 * @param left the left operand expression
 * @param right the right operand expression
 */
public record BinaryOp(int line, int column, String operator, Expr left, Expr right) implements Expr {

    /**
     * Constructs a BinaryOp with validation.
     *
     * @param line the line number (must be positive)
     * @param column the column number (must be positive)
     * @param operator the operator (must not be null)
     * @param left the left operand (must not be null)
     * @param right the right operand (must not be null)
     * @throws NullPointerException if operator, left, or right is null
     * @throws IllegalArgumentException if line or column is not positive
     */
    public BinaryOp {
        Objects.requireNonNull(operator, "operator must not be null");
        Objects.requireNonNull(left, "left operand must not be null");
        Objects.requireNonNull(right, "right operand must not be null");
        if (line <= 0) {
            throw new IllegalArgumentException("line must be positive, got: " + line);
        }
        if (column <= 0) {
            throw new IllegalArgumentException("column must be positive, got: " + column);
        }
    }

    @Override
    public String toString() {
        return String.format("BinaryOp{line=%d, column=%d, operator='%s', left=%s, right=%s}",
            line, column, operator, left, right);
    }
}
