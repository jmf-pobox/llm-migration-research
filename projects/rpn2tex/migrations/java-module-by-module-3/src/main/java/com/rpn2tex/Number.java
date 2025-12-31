package com.rpn2tex;

import java.util.Objects;

/**
 * Numeric literal node representing a number in the expression.
 *
 * <p>Numbers preserve their exact string representation from the source input,
 * including decimal points and negative signs. This ensures that formatting
 * is maintained through the compilation pipeline (e.g., "3.14" stays "3.14",
 * not "3.140000" or simplified to an integer).
 *
 * <h2>Examples</h2>
 * <pre>{@code
 * Number integer = new Number(1, 1, "42");
 * Number decimal = new Number(1, 1, "3.14");
 * Number negative = new Number(1, 1, "-5");
 * }</pre>
 *
 * @see Expr
 * @see BinaryOp
 */
public final class Number implements Expr {
    private final int line;
    private final int column;
    private final String value;

    /**
     * Creates a new Number node.
     *
     * @param line   the 1-based line number
     * @param column the 1-based column number
     * @param value  the string value of the number
     * @throws NullPointerException if value is null
     */
    public Number(int line, int column, String value) {
        this.line = line;
        this.column = column;
        this.value = Objects.requireNonNull(value, "value cannot be null");
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
     * Returns the string representation of this number.
     *
     * @return the number value as a string
     */
    public String getValue() {
        return value;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Number number)) return false;
        return line == number.line &&
               column == number.column &&
               value.equals(number.value);
    }

    @Override
    public int hashCode() {
        return Objects.hash(line, column, value);
    }

    @Override
    public String toString() {
        return "Number(" + line + ", " + column + ", \"" + value + "\")";
    }
}
