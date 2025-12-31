package com.rpn2tex;

/**
 * AST node representing a binary operation (e.g., addition, subtraction).
 *
 * @param line     1-based line number in source
 * @param column   1-based column number in source
 * @param operator The operator symbol ("+", "-", "*", "/")
 * @param left     Left operand expression
 * @param right    Right operand expression
 */
public record BinaryOp(int line, int column, String operator, Expr left, Expr right) implements Expr {
}
