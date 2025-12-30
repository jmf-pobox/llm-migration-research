package com.rpn2tex;

import java.util.Objects;

/**
 * AST node representing a numeric literal.
 *
 * <p>Stores the number as a string to preserve exact formatting
 * (e.g., "3.14", "5", "-2"). This allows faithful reproduction
 * in the generated LaTeX output.
 *
 * <p>Example:
 * <pre>
 *   NumberNode node = new NumberNode(1, 1, "42");
 * </pre>
 *
 * @param line line number (1-based) where number appears
 * @param column column number (1-based) where number starts
 * @param value the string representation of the number
 */
public record NumberNode(int line, int column, String value) implements ASTNode {

    /**
     * Creates a new number node.
     *
     * @param line line number (1-based) where number appears
     * @param column column number (1-based) where number starts
     * @param value the string representation of the number
     * @throws NullPointerException if value is null
     */
    public NumberNode {
        Objects.requireNonNull(value, "value must not be null");
    }
}
