package com.rpn2tex;

import java.util.Map;

/**
 * Generates LaTeX code from an expression AST.
 */
public class LaTeXGenerator {

    /**
     * Mapping from operators to their LaTeX representations.
     */
    private static final Map<String, String> BINARY_OPS = Map.of(
        "+", "+",
        "-", "-",
        "*", "\\times",
        "/", "\\div"
    );

    /**
     * Operator precedence levels (higher number = higher precedence).
     */
    private static final Map<String, Integer> PRECEDENCE = Map.of(
        "+", 1,
        "-", 1,
        "*", 2,
        "/", 2
    );

    /**
     * Generates LaTeX code for an expression.
     *
     * @param expr Expression to generate code for
     * @return LaTeX code wrapped in math delimiters
     */
    public String generate(Expr expr) {
        String latex = visit(expr);
        return "$" + latex + "$";
    }

    private String visit(Expr node) {
        if (node instanceof Number n) {
            return visitNumber(n);
        } else if (node instanceof BinaryOp op) {
            return visitBinaryOp(op);
        } else {
            throw new IllegalArgumentException("Unknown node type: " + node.getClass());
        }
    }

    private String visitNumber(Number node) {
        return node.value();
    }

    private String visitBinaryOp(BinaryOp node) {
        String opLatex = BINARY_OPS.get(node.operator());
        int myPrecedence = PRECEDENCE.get(node.operator());

        String left = visit(node.left());
        if (needsParens(node.left(), myPrecedence, false)) {
            left = "( " + left + " )";
        }

        String right = visit(node.right());
        if (needsParens(node.right(), myPrecedence, true)) {
            right = "( " + right + " )";
        }

        return left + " " + opLatex + " " + right;
    }

    /**
     * Determines if a child expression needs parentheses.
     *
     * @param child           The child expression
     * @param parentPrecedence The parent's precedence level
     * @param isRight         True if the child is the right operand
     * @return True if parentheses are needed
     */
    private boolean needsParens(Expr child, int parentPrecedence, boolean isRight) {
        // Numbers never need parentheses
        if (!(child instanceof BinaryOp childOp)) {
            return false;
        }

        int childPrecedence = PRECEDENCE.get(childOp.operator());

        // Lower precedence child always needs parentheses
        if (childPrecedence < parentPrecedence) {
            return true;
        }

        // Equal precedence on right side needs parentheses for non-commutative operators
        // (handles left-associativity of - and /)
        return childPrecedence == parentPrecedence
            && isRight
            && (childOp.operator().equals("-") || childOp.operator().equals("/"));
    }
}
