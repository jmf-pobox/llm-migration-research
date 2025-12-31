package com.rpn2tex;

/**
 * AST node representing a numeric literal.
 *
 * @param line   1-based line number in source
 * @param column 1-based column number in source
 * @param value  String representation of the number (e.g., "42", "3.14", "-2")
 */
public record Number(int line, int column, String value) implements Expr {
}
