package com.rpn2tex;

import java.util.Objects;

/**
 * Represents a binary operation in the AST (e.g., addition, subtraction).
 *
 * <p>BinaryOp nodes are immutable and contain:
 * <ul>
 *   <li>An operator string ("+", "-", "*", "/")</li>
 *   <li>Left and right operand expressions</li>
 *   <li>Source position tracking (line and column)</li>
 * </ul>
 *
 * <p>Example:
 * <pre>
 *     Expr left = new Number("5", 1, 1);
 *     Expr right = new Number("3", 1, 3);
 *     BinaryOp addition = new BinaryOp("+", left, right, 1, 5);
 * </pre>
 */
public final class BinaryOp implements Expr {
    private final String operator;
    private final Expr left;
    private final Expr right;
    private final int line;
    private final int column;

    /**
     * Constructs a new BinaryOp node.
     *
     * @param operator the operator string (e.g., "+", "-", "*", "/")
     * @param left the left operand expression
     * @param right the right operand expression
     * @param line the 1-based line number in the source
     * @param column the 1-based column number in the source
     * @throws NullPointerException if any argument is null
     * @throws IllegalArgumentException if line or column is less than 1
     */
    public BinaryOp(String operator, Expr left, Expr right, int line, int column) {
        this.operator = Objects.requireNonNull(operator, "operator cannot be null");
        this.left = Objects.requireNonNull(left, "left cannot be null");
        this.right = Objects.requireNonNull(right, "right cannot be null");
        if (line < 1) {
            throw new IllegalArgumentException("line must be >= 1, got: " + line);
        }
        if (column < 1) {
            throw new IllegalArgumentException("column must be >= 1, got: " + column);
        }
        this.line = line;
        this.column = column;
    }

    /**
     * Gets the operator string.
     *
     * @return the operator (e.g., "+", "-", "*", "/")
     */
    public String operator() {
        return operator;
    }

    /**
     * Gets the left operand expression.
     *
     * @return the left operand
     */
    public Expr left() {
        return left;
    }

    /**
     * Gets the right operand expression.
     *
     * @return the right operand
     */
    public Expr right() {
        return right;
    }

    @Override
    public int line() {
        return line;
    }

    @Override
    public int column() {
        return column;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) {
            return true;
        }
        if (!(obj instanceof BinaryOp)) {
            return false;
        }
        BinaryOp other = (BinaryOp) obj;
        return operator.equals(other.operator)
            && left.equals(other.left)
            && right.equals(other.right)
            && line == other.line
            && column == other.column;
    }

    @Override
    public int hashCode() {
        return Objects.hash(operator, left, right, line, column);
    }

    @Override
    public String toString() {
        return String.format(
            "BinaryOp(operator='%s', left=%s, right=%s, line=%d, column=%d)",
            operator, left, right, line, column
        );
    }
}
