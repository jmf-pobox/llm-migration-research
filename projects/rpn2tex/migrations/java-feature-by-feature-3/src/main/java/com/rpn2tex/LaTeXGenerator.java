package com.rpn2tex;

import java.util.Map;

/**
 * Generates LaTeX output from an AST.
 * <p>
 * The generator traverses the AST and produces LaTeX mathematical
 * expressions suitable for rendering. The output is wrapped in
 * dollar signs ($...$) for inline math mode.
 * </p>
 * <p>
 * Example usage:
 * </p>
 * <pre>
 * Expr ast = parser.parse();
 * LaTeXGenerator generator = new LaTeXGenerator();
 * String latex = generator.generate(ast);
 * </pre>
 */
public class LaTeXGenerator {
    /**
     * Mapping from operator strings to their LaTeX representations.
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
     * Generates LaTeX output for the given expression.
     *
     * @param expr The root expression to generate LaTeX for
     * @return The LaTeX string wrapped in dollar signs
     */
    public String generate(Expr expr) {
        String content = visit(expr);
        return "$" + content + "$";
    }

    /**
     * Visits an expression node and generates its LaTeX representation.
     *
     * @param expr The expression to visit
     * @return The LaTeX string for this expression
     */
    private String visit(Expr expr) {
        if (expr instanceof NumberExpr numberExpr) {
            return visitNumber(numberExpr);
        } else if (expr instanceof BinaryOpExpr binaryOpExpr) {
            return visitBinaryOp(binaryOpExpr);
        }
        throw new IllegalArgumentException("Unknown expression type: " + expr.getClass());
    }

    /**
     * Generates LaTeX for a number literal.
     *
     * @param numberExpr The number expression
     * @return The number value as-is
     */
    private String visitNumber(NumberExpr numberExpr) {
        return numberExpr.value();
    }

    /**
     * Generates LaTeX for a binary operation.
     *
     * @param binaryOpExpr The binary operation expression
     * @return The LaTeX representation
     */
    private String visitBinaryOp(BinaryOpExpr binaryOpExpr) {
        String opLatex = BINARY_OPS.get(binaryOpExpr.operator());
        int myPrecedence = PRECEDENCE.get(binaryOpExpr.operator());

        // Generate left operand, adding parentheses if needed
        String left = visit(binaryOpExpr.left());
        if (needsParens(binaryOpExpr.left(), myPrecedence, false)) {
            left = "( " + left + " )";
        }

        // Generate right operand, adding parentheses if needed
        String right = visit(binaryOpExpr.right());
        if (needsParens(binaryOpExpr.right(), myPrecedence, true)) {
            right = "( " + right + " )";
        }

        return left + " " + opLatex + " " + right;
    }

    /**
     * Determines if a child expression needs parentheses.
     *
     * @param child           The child expression
     * @param parentPrecedence The precedence of the parent operator
     * @param isRight         Whether the child is the right operand
     * @return true if parentheses are needed, false otherwise
     */
    private boolean needsParens(Expr child, int parentPrecedence, boolean isRight) {
        if (!(child instanceof BinaryOpExpr binaryChild)) {
            return false;
        }

        int childPrecedence = PRECEDENCE.get(binaryChild.operator());

        // Lower precedence always needs parentheses
        if (childPrecedence < parentPrecedence) {
            return true;
        }

        // Equal precedence on right side needs parentheses for non-commutative operators
        // Subtraction is non-commutative: 5 - (3 - 2) != (5 - 3) - 2
        // Division is non-commutative: 10 / (2 / 5) != (10 / 2) / 5
        return childPrecedence == parentPrecedence
                && isRight
                && (binaryChild.operator().equals("-") || binaryChild.operator().equals("/"));
    }
}
