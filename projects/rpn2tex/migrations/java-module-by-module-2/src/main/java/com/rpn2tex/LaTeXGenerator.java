package com.rpn2tex;

import java.util.Map;

/**
 * Generates LaTeX math mode output from an Abstract Syntax Tree (AST).
 *
 * <p>This class converts an {@link Expr} tree into formatted LaTeX source code with:
 * <ul>
 *   <li>Proper operator symbols (e.g., {@code *} becomes {@code \times})</li>
 *   <li>Intelligent parenthesization based on operator precedence</li>
 *   <li>Math mode delimiters ({@code $ ... $})</li>
 *   <li>Proper spacing around operators and parentheses</li>
 * </ul>
 *
 * <h2>Operator Precedence</h2>
 * <p>The generator handles two precedence levels:
 * <ul>
 *   <li><b>Level 1 (Lower):</b> Addition (+), Subtraction (-)</li>
 *   <li><b>Level 2 (Higher):</b> Multiplication (*), Division (/)</li>
 * </ul>
 *
 * <h2>Parenthesization Rules</h2>
 * <p>Parentheses are added when:
 * <ol>
 *   <li>A lower-precedence operation appears as an operand of a higher-precedence operation</li>
 *   <li>A right operand has equal precedence to its parent for non-associative operations</li>
 * </ol>
 *
 * <h2>Example Usage</h2>
 * <pre>{@code
 * // Create AST for: (5 + 3) * 2
 * Expr ast = new BinaryOp(1, 7, "*",
 *     new BinaryOp(1, 3, "+",
 *         new Number(1, 1, "5"),
 *         new Number(1, 3, "3")
 *     ),
 *     new Number(1, 7, "2")
 * );
 *
 * LaTeXGenerator generator = new LaTeXGenerator();
 * String latex = generator.generate(ast);
 * // Result: "$( 5 + 3 ) \times 2$"
 * }</pre>
 *
 * @since 1.0
 */
public class LaTeXGenerator {

    /**
     * Maps RPN operators to their LaTeX equivalents.
     */
    private static final Map<String, String> BINARY_OPS = Map.of(
        "+", "+",
        "-", "-",
        "*", "\\times",
        "/", "\\div"
    );

    /**
     * Defines operator precedence levels.
     * Higher numbers indicate higher precedence (tighter binding).
     */
    private static final Map<String, Integer> PRECEDENCE = Map.of(
        "+", 1,
        "-", 1,
        "*", 2,
        "/", 2
    );

    /**
     * Generates LaTeX math mode output from an AST.
     *
     * <p>The output is wrapped in {@code $ ... $} delimiters and includes
     * proper spacing around operators and parentheses.
     *
     * @param ast the root expression node of the AST
     * @return LaTeX source code wrapped in math mode delimiters
     * @throws IllegalArgumentException if the AST contains unsupported node types
     */
    public String generate(Expr ast) {
        String content = visit(ast);
        return "$" + content + "$";
    }

    /**
     * Visitor method that dispatches to the appropriate handler based on node type.
     *
     * @param node the expression node to visit
     * @return the LaTeX representation of the node
     * @throws IllegalArgumentException if the node type is not recognized
     */
    private String visit(Expr node) {
        if (node instanceof Number) {
            return visitNumber((Number) node);
        } else if (node instanceof BinaryOp) {
            return visitBinaryOp((BinaryOp) node);
        } else {
            throw new IllegalArgumentException("Unknown node type: " + node.getClass().getName());
        }
    }

    /**
     * Visits a {@link Number} node and returns its string value.
     *
     * @param node the number node
     * @return the numeric value as a string
     */
    private String visitNumber(Number node) {
        return node.getValue();
    }

    /**
     * Visits a {@link BinaryOp} node and generates LaTeX with appropriate parenthesization.
     *
     * <p>This method:
     * <ol>
     *   <li>Looks up the LaTeX operator symbol</li>
     *   <li>Recursively generates left and right operands</li>
     *   <li>Adds parentheses to operands when needed based on precedence</li>
     *   <li>Formats the result with proper spacing</li>
     * </ol>
     *
     * @param node the binary operation node
     * @return the formatted LaTeX expression
     * @throws IllegalArgumentException if the operator is not recognized
     */
    private String visitBinaryOp(BinaryOp node) {
        String operator = node.getOperator();
        String opLatex = BINARY_OPS.get(operator);

        if (opLatex == null) {
            throw new IllegalArgumentException("Unknown operator: " + operator);
        }

        Integer precedence = PRECEDENCE.get(operator);
        if (precedence == null) {
            throw new IllegalArgumentException("No precedence defined for operator: " + operator);
        }

        int myPrecedence = precedence;

        // Generate left operand, adding parens if needed
        String left = visit(node.getLeft());
        if (needsParens(node.getLeft(), myPrecedence, false)) {
            left = "( " + left + " )";
        }

        // Generate right operand, adding parens if needed
        String right = visit(node.getRight());
        if (needsParens(node.getRight(), myPrecedence, true)) {
            right = "( " + right + " )";
        }

        return left + " " + opLatex + " " + right;
    }

    /**
     * Determines whether a child expression needs parentheses.
     *
     * <p>Parentheses are required when:
     * <ul>
     *   <li>The child has lower precedence than the parent operation</li>
     *   <li>The child has equal precedence and is the right operand of a non-commutative operation</li>
     * </ul>
     *
     * <p>Numbers never need parentheses as they are atomic values.
     *
     * @param child the child expression to check
     * @param parentPrecedence the precedence level of the parent operation
     * @param isRight true if the child is the right operand, false if left
     * @return true if parentheses are needed, false otherwise
     */
    private boolean needsParens(Expr child, int parentPrecedence, boolean isRight) {
        // Numbers never need parentheses
        if (!(child instanceof BinaryOp)) {
            return false;
        }

        BinaryOp binOp = (BinaryOp) child;
        String childOperator = binOp.getOperator();
        Integer childPrecVal = PRECEDENCE.get(childOperator);

        if (childPrecVal == null) {
            throw new IllegalArgumentException("No precedence defined for operator: " + childOperator);
        }

        int childPrecedence = childPrecVal;

        // Lower precedence always needs parens
        if (childPrecedence < parentPrecedence) {
            return true;
        }

        // Equal precedence on right side needs parens for non-associative operators
        // (subtraction and division are right-associative in our representation)
        if (childPrecedence == parentPrecedence && isRight) {
            // For subtraction and division, we need parens on the right
            // to avoid ambiguity (e.g., 5 - (3 - 2) vs 5 - 3 - 2)
            return childOperator.equals("-") || childOperator.equals("/");
        }

        return false;
    }
}
