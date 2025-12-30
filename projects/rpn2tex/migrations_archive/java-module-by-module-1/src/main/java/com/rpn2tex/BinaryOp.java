package com.rpn2tex;

public final class BinaryOp implements Expr {
    public final int line;
    public final int column;
    public final String operator;  // "+", "-", "*", "/"
    public final Expr left;
    public final Expr right;

    public BinaryOp(int line, int column, String operator, Expr left, Expr right) {
        this.line = line;
        this.column = column;
        this.operator = operator;
        this.left = left;
        this.right = right;
    }

    @Override public int line() { return line; }
    @Override public int column() { return column; }
}
