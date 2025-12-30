package com.rpn2tex;

import java.util.*;

/**
 * Generates LaTeX representations of mathematical expressions from AST nodes.
 *
 * <p>This generator converts an abstract syntax tree (AST) into LaTeX-formatted
 * mathematical expressions, properly handling operator precedence and parenthesization.
 *
 * <p>Example usage:
 * <pre>{@code
 * LaTeXGenerator generator = new LaTeXGenerator();
 * Expr ast = ...; // from Parser
 * String latex = generator.generate(ast);
 * // Returns: "$5 + 3$"
 * }</pre>
 */
public class LaTeXGenerator {
    private static final Map<String, String> BINARY_OPS = Map.of(
        "+", "+",
        "-", "-",
        "*", "\\times",
        "/", "\\div"
    );

    private static final Map<String, Integer> PRECEDENCE = Map.of(
        "+", 1,
        "-", 1,
        "*", 2,
        "/", 2
    );

    /**
     * Generates LaTeX representation of an AST.
     *
     * @param ast the expression AST to convert
     * @return LaTeX string wrapped in dollar signs (e.g., "$5 + 3$")
     */
    public String generate(Expr ast) {
        return "$" + visit(ast) + "$";
    }

    private String visit(Expr node) {
        if (node instanceof Number) {
            return ((Number) node).value();
        } else if (node instanceof BinaryOp) {
            return visitBinaryOp((BinaryOp) node);
        }
        throw new AssertionError("Unknown node type");
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

    private boolean needsParens(Expr child, int parentPrecedence, boolean isRight) {
        if (!(child instanceof BinaryOp)) {
            return false;
        }

        BinaryOp childOp = (BinaryOp) child;
        int childPrecedence = PRECEDENCE.get(childOp.operator());

        if (childPrecedence < parentPrecedence) {
            return true;
        }

        return childPrecedence == parentPrecedence && isRight &&
               (childOp.operator().equals("-") || childOp.operator().equals("/"));
    }
}
