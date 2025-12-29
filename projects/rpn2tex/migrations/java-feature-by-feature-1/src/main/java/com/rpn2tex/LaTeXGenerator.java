package com.rpn2tex;

import java.util.Objects;

/**
 * Converts rpn2tex AST to LaTeX source code.
 *
 * <p>Uses the visitor pattern to handle different node types.
 * The generator produces LaTeX math mode output wrapped in $...$ delimiters.
 *
 * <p>Example usage:
 * <pre>
 *   NumberNode node = new NumberNode(1, 1, "42");
 *   LaTeXGenerator generator = new LaTeXGenerator();
 *   String latex = generator.generate(node);
 *   // Returns: "$42$"
 * </pre>
 */
public class LaTeXGenerator {

    // Operator precedence (higher = binds tighter)
    private static final int PRECEDENCE_ADDITION = 1;

    /**
     * Generates LaTeX from an AST.
     *
     * @param ast the root expression node
     * @return LaTeX string wrapped in math delimiters ($...$)
     * @throws NullPointerException if ast is null
     */
    public String generate(ASTNode ast) {
        Objects.requireNonNull(ast, "ast must not be null");
        String content = visit(ast);
        return "$" + content + "$";
    }

    /**
     * Visits an AST node and generates LaTeX.
     *
     * <p>This method dispatches to specific handlers based on the node type.
     *
     * @param node the AST node to visit
     * @return LaTeX string for the node
     */
    private String visit(ASTNode node) {
        if (node instanceof NumberNode numberNode) {
            return visitNumber(numberNode);
        } else if (node instanceof BinaryOpNode binaryOpNode) {
            return visitBinaryOp(binaryOpNode);
        }

        throw new IllegalArgumentException(
            "Unsupported node type: " + node.getClass().getName()
        );
    }

    /**
     * Generates LaTeX for a number literal.
     *
     * @param node the Number node
     * @return the number value as a string
     */
    private String visitNumber(NumberNode node) {
        return node.value();
    }

    /**
     * Generates LaTeX for a binary operation.
     *
     * <p>Handles operator precedence by adding parentheses around
     * lower-precedence sub-expressions.
     *
     * @param node the BinaryOp node
     * @return LaTeX string with appropriate parentheses
     */
    private String visitBinaryOp(BinaryOpNode node) {
        String opLatex = getOperatorLatex(node.operator());
        int myPrecedence = getPrecedence(node.operator());

        // Generate left operand, adding parens if needed
        String left = visit(node.left());
        if (needsParens(node.left(), myPrecedence, false)) {
            left = "( " + left + " )";
        }

        // Generate right operand, adding parens if needed
        String right = visit(node.right());
        if (needsParens(node.right(), myPrecedence, true)) {
            right = "( " + right + " )";
        }

        return left + " " + opLatex + " " + right;
    }

    /**
     * Maps an operator string to its LaTeX representation.
     *
     * @param operator the operator string ("+", "-", "*", "/")
     * @return the LaTeX representation
     */
    private String getOperatorLatex(String operator) {
        return switch (operator) {
            case "+" -> "+";
            case "-" -> "-";
            case "*" -> "\\times";
            case "/" -> "\\div";
            default -> throw new IllegalArgumentException("Unknown operator: " + operator);
        };
    }

    /**
     * Returns the precedence level for an operator.
     *
     * @param operator the operator string
     * @return precedence level (higher = tighter binding)
     */
    private int getPrecedence(String operator) {
        return switch (operator) {
            case "+", "-" -> PRECEDENCE_ADDITION;
            case "*", "/" -> 2;  // Higher precedence for multiplication/division
            default -> throw new IllegalArgumentException("Unknown operator: " + operator);
        };
    }

    /**
     * Determines if a child expression needs parentheses.
     *
     * <p>Parentheses are needed when:
     * <ol>
     *   <li>Child has lower precedence than parent</li>
     *   <li>Child has equal precedence and is on the right side
     *       (for left-associative operators like - and /)</li>
     * </ol>
     *
     * @param child the child expression
     * @param parentPrecedence precedence of the parent operator
     * @param isRight true if child is the right operand
     * @return true if parentheses are needed
     */
    private boolean needsParens(ASTNode child, int parentPrecedence, boolean isRight) {
        if (!(child instanceof BinaryOpNode childOp)) {
            return false;
        }

        int childPrecedence = getPrecedence(childOp.operator());

        // Lower precedence always needs parens
        if (childPrecedence < parentPrecedence) {
            return true;
        }

        // Equal precedence on right side needs parens for non-commutative operators
        // (handles left-associativity of - and /)
        return childPrecedence == parentPrecedence
            && isRight
            && (childOp.operator().equals("-") || childOp.operator().equals("/"));
    }
}
