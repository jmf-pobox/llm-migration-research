package com.rpn2tex;

import java.util.Objects;

/**
 * Represents an expression node in the Abstract Syntax Tree (AST).
 *
 * <p>This is a sealed interface that permits only two implementations:
 * {@link Number} for numeric literals and {@link BinaryOp} for binary operations.
 * This design ensures exhaustive pattern matching at compile-time and prevents
 * unauthorized extensions.</p>
 *
 * <p>All AST nodes track their position in the source code (line and column)
 * to enable accurate error reporting.</p>
 *
 * <p>Example usage:
 * <pre>{@code
 * // Create a number node: 5
 * Expr five = new Number(1, 1, "5");
 *
 * // Create a binary operation node: 5 + 3
 * Expr addition = new BinaryOp(1, 3, "+",
 *     new Number(1, 1, "5"),
 *     new Number(1, 3, "3")
 * );
 * }</pre>
 *
 * @since 1.0
 */
public sealed interface Expr permits Number, BinaryOp {
    /**
     * Returns the 1-based line number where this expression appears in source code.
     *
     * @return the line number (1-based)
     */
    int line();

    /**
     * Returns the 1-based column number where this expression appears in source code.
     *
     * @return the column number (1-based)
     */
    int column();
}

/**
 * Represents a numeric literal in the AST.
 *
 * <p>Numbers are stored as strings to preserve the exact representation
 * from the source code (e.g., "3.14" remains "3.14", not converted to a
 * floating-point type which might lose precision or formatting).</p>
 *
 * <p>This record is immutable and validates all inputs in its compact constructor.</p>
 *
 * @param line   the line number (1-based) where this number appears
 * @param column the column number (1-based) where this number appears
 * @param value  the string representation of the numeric value
 * @since 1.0
 */
record Number(int line, int column, String value) implements Expr {
    /**
     * Compact constructor with validation.
     *
     * @throws NullPointerException if value is null
     * @throws IllegalArgumentException if line or column is less than 1
     */
    public Number {
        if (line < 1) {
            throw new IllegalArgumentException("Line number must be >= 1, got: " + line);
        }
        if (column < 1) {
            throw new IllegalArgumentException("Column number must be >= 1, got: " + column);
        }
        Objects.requireNonNull(value, "Number value cannot be null");
    }

    /**
     * Returns the string representation of this number.
     *
     * @return the numeric value as a string
     */
    public String getValue() {
        return value;
    }
}

/**
 * Represents a binary operation in the AST.
 *
 * <p>A binary operation consists of:
 * <ul>
 *   <li>An operator ("+", "-", "*", "/")</li>
 *   <li>A left operand (any {@link Expr})</li>
 *   <li>A right operand (any {@link Expr})</li>
 * </ul>
 *
 * <p>This recursive structure allows arbitrary nesting of expressions,
 * enabling complex mathematical expressions to be represented as a tree.</p>
 *
 * <p>Example:
 * <pre>{@code
 * // Represents: (5 + 3) * 2
 * Expr expr = new BinaryOp(1, 7, "*",
 *     new BinaryOp(1, 3, "+",
 *         new Number(1, 1, "5"),
 *         new Number(1, 3, "3")
 *     ),
 *     new Number(1, 7, "2")
 * );
 * }</pre>
 *
 * @param line     the line number (1-based) where this operation appears
 * @param column   the column number (1-based) where this operation appears
 * @param operator the operator string ("+", "-", "*", "/")
 * @param left     the left operand expression
 * @param right    the right operand expression
 * @since 1.0
 */
record BinaryOp(int line, int column, String operator, Expr left, Expr right) implements Expr {
    /**
     * Compact constructor with validation.
     *
     * @throws NullPointerException if operator, left, or right is null
     * @throws IllegalArgumentException if line or column is less than 1
     */
    public BinaryOp {
        if (line < 1) {
            throw new IllegalArgumentException("Line number must be >= 1, got: " + line);
        }
        if (column < 1) {
            throw new IllegalArgumentException("Column number must be >= 1, got: " + column);
        }
        Objects.requireNonNull(operator, "Operator cannot be null");
        Objects.requireNonNull(left, "Left operand cannot be null");
        Objects.requireNonNull(right, "Right operand cannot be null");
    }

    /**
     * Returns the operator of this binary operation.
     *
     * @return the operator string
     */
    public String getOperator() {
        return operator;
    }

    /**
     * Returns the left operand of this binary operation.
     *
     * @return the left operand expression
     */
    public Expr getLeft() {
        return left;
    }

    /**
     * Returns the right operand of this binary operation.
     *
     * @return the right operand expression
     */
    public Expr getRight() {
        return right;
    }
}
