package com.rpn2tex;

import java.util.Objects;

/**
 * AST node representing a binary operation (placeholder for future features).
 * <p>
 * This class is currently a stub to satisfy the sealed interface requirement.
 * It will be fully implemented in Feature 2 (Addition) and beyond.
 * </p>
 *
 * @param operator The operator string (e.g., "+", "-", "*", "/")
 * @param left     The left operand expression
 * @param right    The right operand expression
 * @param line     The 1-based line number where the operation appears
 * @param column   The 1-based column number where the operation appears
 */
public record BinaryOpExpr(String operator, Expr left, Expr right, int line, int column) implements Expr {
    /**
     * Constructs a new BinaryOpExpr with validation.
     *
     * @param operator The operator string
     * @param left     The left operand expression
     * @param right    The right operand expression
     * @param line     The 1-based line number
     * @param column   The 1-based column number
     * @throws NullPointerException if any parameter is null
     */
    public BinaryOpExpr {
        Objects.requireNonNull(operator, "Operator cannot be null");
        Objects.requireNonNull(left, "Left operand cannot be null");
        Objects.requireNonNull(right, "Right operand cannot be null");
    }

    @Override
    public String toString() {
        return String.format("BinaryOp(%s, %s, %s)", operator, left, right);
    }
}
