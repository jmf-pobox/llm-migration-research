package com.rpn2tex;

public final class Number implements Expr {
    public final int line;
    public final int column;
    public final String value;

    public Number(int line, int column, String value) {
        this.line = line;
        this.column = column;
        this.value = value;
    }

    @Override public int line() { return line; }
    @Override public int column() { return column; }
}
