package com.rpn2tex;

import java.util.Objects;

/**
 * AST node representing a numeric literal.
 * <p>
 * The value is stored as a string to preserve the exact representation
 * from the source (e.g., "3.14", "007", "-5"). This allows for precise
 * LaTeX output without floating-point conversion issues.
 * </p>
 *
 * @param value  The string representation of the number
 * @param line   The 1-based line number where the number appears
 * @param column The 1-based column number where the number appears
 */
public record NumberExpr(String value, int line, int column) implements Expr {
    /**
     * Constructs a new NumberExpr with validation.
     *
     * @param value  The string representation of the number
     * @param line   The 1-based line number where the number appears
     * @param column The 1-based column number where the number appears
     * @throws NullPointerException if value is null
     */
    public NumberExpr {
        Objects.requireNonNull(value, "Number value cannot be null");
    }

    @Override
    public String toString() {
        return String.format("Number(%s)", value);
    }
}
