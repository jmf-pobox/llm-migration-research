package com.rpn2tex;

import java.util.Map;

/**
 * Generates LaTeX representations of mathematical expressions from AST nodes.
 *
 * <p>Converts the AST into LaTeX math notation, wrapping the result in
 * math mode delimiters ($...$). Handles operator precedence and automatic
 * parenthesization.
 *
 * <p>Operator precedence levels:
 * <ul>
 *   <li>Addition (+) and Subtraction (-): precedence 1 (lowest)</li>
 *   <li>Multiplication (*) and Division (/): precedence 2 (highest)</li>
 * </ul>
 *
 * <p>Parenthesization rules:
 * <ul>
 *   <li>Lower precedence operations need parentheses when children of higher precedence</li>
 *   <li>Right operands of subtraction/division need parentheses if they're also subtraction/division (left-associativity)</li>
 * </ul>
 *
 * <p>Example usage:
 * <pre>{@code
 * LaTeXGenerator generator = new LaTeXGenerator();
 * Expr ast = new Number("5", 1, 1);
 * String latex = generator.generate(ast);
 * // Returns: "$5$"
 *
 * // Subtraction example: "5 3 -" -> "$5 - 3$"
 * Expr sub = new BinaryOp("-", new Number("5", 1, 1), new Number("3", 1, 3), 1, 5);
 * String latexSub = generator.generate(sub);
 * // Returns: "$5 - 3$"
 *
 * // Left-associativity example: "5 3 - 2 -" -> "$5 - 3 - 2$"
 * }</pre>
 */
public class LaTeXGenerator {
    /**
     * Operator precedence levels. Higher value = higher precedence.
     */
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
     * @return LaTeX string wrapped in dollar signs (e.g., "$5$", "$5 - 3$")
     */
    public String generate(Expr ast) {
        return "$" + visit(ast) + "$";
    }

    /**
     * Maps operators to their LaTeX representations.
     */
    private static final Map<String, String> OPERATOR_LATEX = Map.of(
        "+", "+",
        "-", "-",
        "*", "\\times",
        "/", "\\div"
    );

    /**
     * Recursively visits AST nodes to generate LaTeX.
     *
     * @param node the expression node to visit
     * @return LaTeX representation of the node
     */
    private String visit(Expr node) {
        if (node instanceof Number) {
            return ((Number) node).value();
        } else if (node instanceof BinaryOp) {
            BinaryOp binOp = (BinaryOp) node;
            int myPrecedence = PRECEDENCE.getOrDefault(binOp.operator(), 0);

            // Visit left operand
            String left = visit(binOp.left());
            if (needsParens(binOp.left(), myPrecedence, false)) {
                left = "( " + left + " )";
            }

            // Visit right operand
            String right = visit(binOp.right());
            if (needsParens(binOp.right(), myPrecedence, true)) {
                right = "( " + right + " )";
            }

            // Get LaTeX operator symbol
            String opLatex = OPERATOR_LATEX.getOrDefault(binOp.operator(), binOp.operator());
            return left + " " + opLatex + " " + right;
        }
        throw new AssertionError("Unknown node type: " + node.getClass().getName());
    }

    /**
     * Determines if a child expression needs parentheses.
     *
     * <p>Parentheses are needed when:
     * <ol>
     *   <li>Child has lower precedence than parent</li>
     *   <li>Child has equal precedence, is on the right side, and is subtraction or division
     *       (to preserve left-associativity)</li>
     * </ol>
     *
     * @param child the child expression to check
     * @param parentPrecedence the precedence level of the parent operator
     * @param isRight true if this is the right operand, false if left
     * @return true if parentheses are needed
     */
    private boolean needsParens(Expr child, int parentPrecedence, boolean isRight) {
        if (!(child instanceof BinaryOp)) {
            return false; // Numbers never need parentheses
        }

        BinaryOp childOp = (BinaryOp) child;
        int childPrecedence = PRECEDENCE.getOrDefault(childOp.operator(), 0);

        // Lower precedence always needs parentheses
        if (childPrecedence < parentPrecedence) {
            return true;
        }

        // Equal precedence on right side needs parentheses for non-commutative operators
        // This handles left-associativity of subtraction and division
        // Example: 5 - (3 - 2) needs parens, but (5 - 3) - 2 doesn't
        if (childPrecedence == parentPrecedence && isRight) {
            String childOperator = childOp.operator();
            return childOperator.equals("-") || childOperator.equals("/");
        }

        return false;
    }
}
