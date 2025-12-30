package com.rpn2tex;

import java.util.Objects;

/**
 * Represents a numeric literal in the AST.
 *
 * <p>Numbers are stored as strings to preserve their exact representation
 * from the source code (e.g., "3.14", "42"). No numeric parsing or rounding
 * is performed.</p>
 *
 * <p>Example usage:</p>
 * <pre>{@code
 * Number num = new Number(1, 1, "42");
 * System.out.println(num.value()); // "42"
 * }</pre>
 *
 * @param line the line number where this number appears (1-based)
 * @param column the column number where this number appears (1-based)
 * @param value the string representation of the number (e.g., "3.14", "42")
 */
public record Number(int line, int column, String value) implements Expr {

    /**
     * Constructs a Number with validation.
     *
     * @param line the line number (must be positive)
     * @param column the column number (must be positive)
     * @param value the string value (must not be null)
     * @throws NullPointerException if value is null
     * @throws IllegalArgumentException if line or column is not positive
     */
    public Number {
        Objects.requireNonNull(value, "value must not be null");
        if (line <= 0) {
            throw new IllegalArgumentException("line must be positive, got: " + line);
        }
        if (column <= 0) {
            throw new IllegalArgumentException("column must be positive, got: " + column);
        }
    }

    @Override
    public String toString() {
        return String.format("Number{line=%d, column=%d, value='%s'}", line, column, value);
    }
}
