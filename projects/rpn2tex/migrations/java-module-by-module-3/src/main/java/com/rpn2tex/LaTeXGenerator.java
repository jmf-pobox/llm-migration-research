package com.rpn2tex;

import java.util.Collections;
import java.util.Map;

/**
 * AST to LaTeX converter with proper precedence handling.
 *
 * <p>Generates LaTeX mathematical notation from an Abstract Syntax Tree (AST),
 * automatically handling operator precedence and inserting parentheses only
 * when necessary. The output is formatted for inline math mode with proper
 * LaTeX symbols for mathematical operators.
 *
 * <h2>Operator Mapping</h2>
 * <ul>
 *   <li>{@code +} → {@code +} (addition)</li>
 *   <li>{@code -} → {@code -} (subtraction)</li>
 *   <li>{@code *} → {@code \times} (multiplication)</li>
 *   <li>{@code /} → {@code \div} (division)</li>
 * </ul>
 *
 * <h2>Precedence Rules</h2>
 * <p>Multiplication and division have higher precedence (level 2) than
 * addition and subtraction (level 1). Parentheses are inserted when:
 * <ul>
 *   <li>A lower-precedence operation is an operand to a higher-precedence operation</li>
 *   <li>A right operand has equal precedence and the operator is non-associative (-, /)</li>
 * </ul>
 *
 * <h2>Examples</h2>
 * <pre>{@code
 * // Input RPN: "5 3 +"
 * // AST: BinaryOp("+", Number("5"), Number("3"))
 * // Output: "$5 + 3$"
 *
 * // Input RPN: "2 3 + 4 *"
 * // AST: BinaryOp("*", BinaryOp("+", Number("2"), Number("3")), Number("4"))
 * // Output: "$( 2 + 3 ) \times 4$"
 *
 * // Input RPN: "5 3 * 2 +"
 * // AST: BinaryOp("+", BinaryOp("*", Number("5"), Number("3")), Number("2"))
 * // Output: "$5 \times 3 + 2$"
 * }</pre>
 *
 * <h2>Thread Safety</h2>
 * <p>This class is thread-safe and can be reused across multiple invocations.
 * All methods are stateless except for the static configuration maps.
 *
 * @see Expr
 * @see Number
 * @see BinaryOp
 */
public final class LaTeXGenerator {
    /**
     * Mapping of operators to their LaTeX symbol equivalents.
     * Immutable configuration defining the visual representation of each operator.
     */
    static final Map<String, String> BINARY_OPS = Collections.unmodifiableMap(
        Map.ofEntries(
            Map.entry("+", "+"),
            Map.entry("-", "-"),
            Map.entry("*", "\\times"),
            Map.entry("/", "\\div")
        )
    );

    /**
     * Operator precedence levels.
     * Higher numbers indicate higher precedence.
     * <ul>
     *   <li>Level 1: Addition (+), Subtraction (-)</li>
     *   <li>Level 2: Multiplication (*), Division (/)</li>
     * </ul>
     */
    static final Map<String, Integer> PRECEDENCE = Collections.unmodifiableMap(
        Map.ofEntries(
            Map.entry("+", 1),
            Map.entry("-", 1),
            Map.entry("*", 2),
            Map.entry("/", 2)
        )
    );

    /**
     * Generates LaTeX notation from an AST expression.
     *
     * <p>Converts the provided expression tree into LaTeX format, wrapped in
     * inline math mode delimiters ({@code $...$}). The method handles operator
     * precedence automatically and inserts parentheses only when needed.
     *
     * @param ast the root expression node to convert
     * @return LaTeX string wrapped in {@code $...$} delimiters
     * @throws NullPointerException if ast is null
     *
     * @example
     * <pre>{@code
     * LaTeXGenerator generator = new LaTeXGenerator();
     * Expr ast = new BinaryOp(1, 3, "+", new Number(1, 1, "5"), new Number(1, 3, "3"));
     * String latex = generator.generate(ast);
     * // Returns: "$5 + 3$"
     * }</pre>
     */
    public String generate(Expr ast) {
        String content = visit(ast);
        return "$" + content + "$";
    }

    /**
     * Visits an AST node and generates its LaTeX representation.
     *
     * <p>Uses pattern matching to dispatch to the appropriate visitor method
     * based on the runtime type of the node. This implements a visitor pattern
     * without requiring the AST nodes to implement an accept method.
     *
     * @param node the expression node to visit
     * @return the LaTeX representation of the node
     * @throws AssertionError if an unknown node type is encountered
     */
    private String visit(Expr node) {
        if (node instanceof Number num) {
            return visitNumber(num);
        } else if (node instanceof BinaryOp binOp) {
            return visitBinaryOp(binOp);
        } else {
            throw new AssertionError("Unknown node type: " + node.getClass().getName());
        }
    }

    /**
     * Generates LaTeX for a Number node.
     *
     * <p>Simply returns the string value of the number, preserving its exact
     * format from the source input (including decimal points and negative signs).
     *
     * @param node the number node to convert
     * @return the number's string value
     */
    private String visitNumber(Number node) {
        return node.getValue();
    }

    /**
     * Generates LaTeX for a BinaryOp node.
     *
     * <p>Recursively generates LaTeX for both operands, adding parentheses
     * around child operations when required by precedence rules. The operator
     * is converted to its LaTeX equivalent symbol.
     *
     * <p>Parenthesization rules:
     * <ul>
     *   <li>Left operand: Add parentheses if child precedence is lower</li>
     *   <li>Right operand: Add parentheses if child precedence is lower, OR
     *       if precedence is equal and operator is non-associative (-, /)</li>
     * </ul>
     *
     * @param node the binary operation node to convert
     * @return the LaTeX representation with proper spacing and parentheses
     */
    private String visitBinaryOp(BinaryOp node) {
        String opLatex = BINARY_OPS.get(node.getOperator());
        int myPrecedence = PRECEDENCE.get(node.getOperator());

        // Generate left operand with parentheses if needed
        String left = visit(node.getLeft());
        if (needsParens(node.getLeft(), myPrecedence, false)) {
            left = "( " + left + " )";
        }

        // Generate right operand with parentheses if needed
        String right = visit(node.getRight());
        if (needsParens(node.getRight(), myPrecedence, true)) {
            right = "( " + right + " )";
        }

        // Combine with operator and proper spacing
        return left + " " + opLatex + " " + right;
    }

    /**
     * Determines if a child expression needs parentheses.
     *
     * <p>Parentheses are required when:
     * <ol>
     *   <li>The child is a binary operation with lower precedence than the parent</li>
     *   <li>The child is a binary operation with equal precedence, is the right
     *       operand, and the operator is non-associative (- or /)</li>
     * </ol>
     *
     * <p>Number nodes never need parentheses.
     *
     * <h3>Examples:</h3>
     * <ul>
     *   <li>{@code (2 + 3) * 4} - Addition (prec 1) needs parens in multiplication (prec 2)</li>
     *   <li>{@code 5 - (3 - 2)} - Right subtraction needs parens due to non-associativity</li>
     *   <li>{@code 5 * 3 + 2} - Multiplication (prec 2) doesn't need parens in addition (prec 1)</li>
     * </ul>
     *
     * @param child the child expression to check
     * @param parentPrecedence the precedence level of the parent operation
     * @param isRight true if this is the right operand, false if left
     * @return true if parentheses are needed, false otherwise
     */
    private boolean needsParens(Expr child, int parentPrecedence, boolean isRight) {
        // Numbers never need parentheses
        if (!(child instanceof BinaryOp binOp)) {
            return false;
        }

        int childPrecedence = PRECEDENCE.get(binOp.getOperator());

        // Lower precedence always needs parentheses
        if (childPrecedence < parentPrecedence) {
            return true;
        }

        // Equal precedence: check for non-associative operators on the right
        // Subtraction and division are left-associative, so:
        // a - (b - c) needs parens, but (a - b) - c doesn't
        // a / (b / c) needs parens, but (a / b) / c doesn't
        return childPrecedence == parentPrecedence &&
               isRight &&
               (binOp.getOperator().equals("-") || binOp.getOperator().equals("/"));
    }
}
