package com.rpn2tex;

import java.util.Map;

/**
 * Generates LaTeX output from an AST with proper precedence and parenthesization.
 *
 * <p>This class uses a visitor pattern to traverse the AST and generate LaTeX strings.
 * It handles operator precedence and automatically inserts parentheses where needed
 * to preserve mathematical correctness.</p>
 *
 * <p>Example usage:</p>
 * <pre>{@code
 * LaTeXGenerator generator = new LaTeXGenerator();
 * Expr ast = parser.parse(); // Parse "5 3 +"
 * String latex = generator.generate(ast); // Returns "$5 + 3$"
 * }</pre>
 *
 * <h2>Operator Mappings</h2>
 * <ul>
 *   <li>+ maps to +</li>
 *   <li>- maps to -</li>
 *   <li>* maps to \times</li>
 *   <li>/ maps to \div</li>
 * </ul>
 *
 * <h2>Precedence Rules</h2>
 * <ul>
 *   <li>Level 1 (low): + and -</li>
 *   <li>Level 2 (high): * and /</li>
 * </ul>
 *
 * <h2>Parenthesization Rules</h2>
 * <ul>
 *   <li>Lower precedence child always needs parentheses</li>
 *   <li>Equal precedence on right side of - or / needs parentheses (left-associativity)</li>
 *   <li>Otherwise, no parentheses</li>
 * </ul>
 */
public class LaTeXGenerator {

    /**
     * Maps operators to their LaTeX representations.
     */
    private static final Map<String, String> BINARY_OPS = Map.of(
        "+", "+",
        "-", "-",
        "*", "\\times",
        "/", "\\div"
    );

    /**
     * Maps operators to their precedence levels.
     * Higher numbers indicate higher precedence.
     */
    private static final Map<String, Integer> PRECEDENCE = Map.of(
        "+", 1,
        "-", 1,
        "*", 2,
        "/", 2
    );

    /**
     * Generates LaTeX output from an AST.
     *
     * <p>The output is wrapped in LaTeX math mode delimiters ($...$) and includes
     * proper spacing and parenthesization.</p>
     *
     * @param ast the expression tree to convert
     * @return the LaTeX string representation (e.g., "$5 + 3$")
     * @throws NullPointerException if ast is null
     */
    public String generate(Expr ast) {
        if (ast == null) {
            throw new NullPointerException("ast must not be null");
        }
        String content = visit(ast);
        return "$" + content + "$";
    }

    /**
     * Visits an expression node and generates its LaTeX representation.
     *
     * @param expr the expression to visit
     * @return the LaTeX string for this expression (without $ delimiters)
     */
    private String visit(Expr expr) {
        return switch (expr) {
            case Number num -> num.value();
            case BinaryOp binOp -> visitBinaryOp(binOp);
        };
    }

    /**
     * Visits a binary operation and generates its LaTeX representation.
     *
     * @param binOp the binary operation to visit
     * @return the LaTeX string for this operation
     */
    private String visitBinaryOp(BinaryOp binOp) {
        String operator = binOp.operator();
        int myPrecedence = PRECEDENCE.get(operator);
        String latexOp = BINARY_OPS.get(operator);

        // Process left child
        String leftStr = visit(binOp.left());
        if (needsParens(binOp.left(), myPrecedence, false)) {
            leftStr = "( " + leftStr + " )";
        }

        // Process right child
        String rightStr = visit(binOp.right());
        if (needsParens(binOp.right(), myPrecedence, true)) {
            rightStr = "( " + rightStr + " )";
        }

        return leftStr + " " + latexOp + " " + rightStr;
    }

    /**
     * Determines if a child expression needs parentheses.
     *
     * <p>Parentheses are needed in the following cases:</p>
     * <ul>
     *   <li>Child has lower precedence than parent</li>
     *   <li>Child has equal precedence, is on the right, and is a - or / operation
     *       (to preserve left-associativity)</li>
     * </ul>
     *
     * @param child the child expression to check
     * @param parentPrecedence the precedence level of the parent operation
     * @param isRight true if this is the right child, false if left
     * @return true if parentheses are needed, false otherwise
     */
    private boolean needsParens(Expr child, int parentPrecedence, boolean isRight) {
        if (!(child instanceof BinaryOp childOp)) {
            return false;
        }

        String childOperator = childOp.operator();
        int childPrecedence = PRECEDENCE.get(childOperator);

        // Lower precedence always needs parens
        if (childPrecedence < parentPrecedence) {
            return true;
        }

        // Equal precedence on right of - or / needs parens
        if (childPrecedence == parentPrecedence && isRight) {
            return childOperator.equals("-") || childOperator.equals("/");
        }

        return false;
    }
}
