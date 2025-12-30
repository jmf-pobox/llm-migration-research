package com.rpn2tex;

import java.util.Objects;

/**
 * Represents a numeric literal in the AST.
 *
 * <p>Numbers are immutable leaf nodes containing a string representation
 * of the numeric value. The value is stored as a string to preserve the
 * exact format (including decimal places) from the source.
 *
 * <p>Example:
 * <pre>
 *     Number num = new Number("3.14", 1, 5);
 *     System.out.println(num.value());  // "3.14"
 * </pre>
 */
public final class Number implements Expr {
    private final String value;
    private final int line;
    private final int column;

    /**
     * Constructs a new Number node.
     *
     * @param value the string representation of the number
     * @param line the 1-based line number in the source
     * @param column the 1-based column number in the source
     * @throws NullPointerException if value is null
     * @throws IllegalArgumentException if line or column is less than 1
     */
    public Number(String value, int line, int column) {
        this.value = Objects.requireNonNull(value, "value cannot be null");
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
     * Gets the string representation of this number.
     *
     * @return the numeric value as a string
     */
    public String value() {
        return value;
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
        if (!(obj instanceof Number)) {
            return false;
        }
        Number other = (Number) obj;
        return value.equals(other.value) && line == other.line && column == other.column;
    }

    @Override
    public int hashCode() {
        return Objects.hash(value, line, column);
    }

    @Override
    public String toString() {
        return String.format("Number(value='%s', line=%d, column=%d)", value, line, column);
    }
}
