package com.rpn2tex;

import java.util.Objects;

/**
 * Binary operation node representing an operation with two operands.
 *
 * <p>Supports the four basic arithmetic operators:
 * <ul>
 *   <li>{@code "+"} - Addition</li>
 *   <li>{@code "-"} - Subtraction</li>
 *   <li>{@code "*"} - Multiplication</li>
 *   <li>{@code "/"} - Division</li>
 * </ul>
 *
 * <p>The recursive structure naturally represents expression trees. Each
 * BinaryOp contains two child expressions (left and right), which can
 * themselves be Number nodes or other BinaryOp nodes.
 *
 * <h2>Example</h2>
 * <pre>{@code
 * // Represents: (5 + 3) * 2
 * Expr five = new Number(1, 1, "5");
 * Expr three = new Number(1, 3, "3");
 * Expr addition = new BinaryOp(1, 5, "+", five, three);
 * Expr two = new Number(1, 8, "2");
 * Expr multiplication = new BinaryOp(1, 10, "*", addition, two);
 * }</pre>
 *
 * @see Expr
 * @see Number
 */
public final class BinaryOp implements Expr {
    private final int line;
    private final int column;
    private final String operator;
    private final Expr left;
    private final Expr right;

    /**
     * Creates a new BinaryOp node.
     *
     * @param line     the 1-based line number
     * @param column   the 1-based column number
     * @param operator the operator string
     * @param left     the left operand
     * @param right    the right operand
     * @throws NullPointerException if operator, left, or right is null
     */
    public BinaryOp(int line, int column, String operator, Expr left, Expr right) {
        this.line = line;
        this.column = column;
        this.operator = Objects.requireNonNull(operator, "operator cannot be null");
        this.left = Objects.requireNonNull(left, "left operand cannot be null");
        this.right = Objects.requireNonNull(right, "right operand cannot be null");
    }

    @Override
    public int getLine() {
        return line;
    }

    @Override
    public int getColumn() {
        return column;
    }

    /**
     * Returns the operator string for this binary operation.
     *
     * @return the operator ("+", "-", "*", or "/")
     */
    public String getOperator() {
        return operator;
    }

    /**
     * Returns the left operand expression.
     *
     * @return the left child expression
     */
    public Expr getLeft() {
        return left;
    }

    /**
     * Returns the right operand expression.
     *
     * @return the right child expression
     */
    public Expr getRight() {
        return right;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof BinaryOp binaryOp)) return false;
        return line == binaryOp.line &&
               column == binaryOp.column &&
               operator.equals(binaryOp.operator) &&
               left.equals(binaryOp.left) &&
               right.equals(binaryOp.right);
    }

    @Override
    public int hashCode() {
        return Objects.hash(line, column, operator, left, right);
    }

    @Override
    public String toString() {
        return "BinaryOp(" + line + ", " + column + ", \"" + operator + "\", " +
               left + ", " + right + ")";
    }
}
