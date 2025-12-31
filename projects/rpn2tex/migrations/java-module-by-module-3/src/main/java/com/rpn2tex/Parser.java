package com.rpn2tex;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.List;
import java.util.Objects;

/**
 * Stack-based RPN (Reverse Polish Notation) parser.
 *
 * <p>This parser implements a classic stack-based algorithm for parsing RPN expressions
 * into Abstract Syntax Trees (AST). The algorithm processes tokens from left to right:
 * <ul>
 *   <li>Numbers are pushed onto the stack as Number nodes</li>
 *   <li>Operators pop two operands, create a BinaryOp node, and push the result</li>
 *   <li>At the end, the stack must contain exactly one expression</li>
 * </ul>
 *
 * <h2>Algorithm</h2>
 * <pre>
 * For each token:
 *   If NUMBER:
 *     - Create Number node with token value and position
 *     - Push onto stack
 *   If OPERATOR (+, -, *, /):
 *     - Validate stack has at least 2 operands
 *     - Pop right operand (top of stack)
 *     - Pop left operand (second from top)
 *     - Create BinaryOp node with operator and operands
 *     - Push result onto stack
 *   At EOF:
 *     - Validate stack has exactly 1 element
 *     - Return top of stack as root expression
 * </pre>
 *
 * <h2>Example Usage</h2>
 * <pre>{@code
 * // Parse "5 3 +" to produce an addition AST
 * List<Token> tokens = List.of(
 *     new Token(TokenType.NUMBER, "5", 1, 1),
 *     new Token(TokenType.NUMBER, "3", 1, 3),
 *     new Token(TokenType.PLUS, "+", 1, 5),
 *     new Token(TokenType.EOF, "", 1, 6)
 * );
 *
 * Parser parser = new Parser(tokens);
 * Expr ast = parser.parse(); // Returns BinaryOp(+, Number(5), Number(3))
 * }</pre>
 *
 * <h2>Error Conditions</h2>
 * <p>The parser validates proper RPN structure and throws RpnException for:
 * <ul>
 *   <li>Insufficient operands: Operator encountered with fewer than 2 operands on stack</li>
 *   <li>Empty expression: No tokens before EOF</li>
 *   <li>Extra operands: Multiple values remain on stack after processing all tokens</li>
 * </ul>
 *
 * @see Expr
 * @see Number
 * @see BinaryOp
 * @see RpnException
 * @since 1.0.0
 */
public final class Parser {
    private final List<Token> tokens;
    private int pos;

    /**
     * Creates a new Parser with the given token list.
     *
     * @param tokens the list of tokens to parse (must not be null)
     * @throws NullPointerException if tokens is null
     */
    public Parser(List<Token> tokens) {
        this.tokens = Objects.requireNonNull(tokens, "tokens cannot be null");
        this.pos = 0;
    }

    /**
     * Parses the token stream into an Abstract Syntax Tree.
     *
     * <p>This method implements the stack-based RPN parsing algorithm. It processes
     * each token sequentially, building up an expression tree on the stack. The final
     * result is a single expression representing the entire input.
     *
     * <p><b>Stack Algorithm:</b>
     * <ol>
     *   <li>Initialize empty stack</li>
     *   <li>For each token until EOF:
     *     <ul>
     *       <li>NUMBER: Push Number node onto stack</li>
     *       <li>OPERATOR: Pop 2 operands, create BinaryOp, push result</li>
     *     </ul>
     *   </li>
     *   <li>Validate exactly 1 element remains on stack</li>
     *   <li>Return top of stack as root expression</li>
     * </ol>
     *
     * @return the root expression node of the parsed AST
     * @throws RpnException if the RPN expression is invalid:
     *     <ul>
     *       <li>Operator requires two operands but stack has fewer</li>
     *       <li>Expression is empty (no tokens before EOF)</li>
     *       <li>Multiple values remain on stack (missing operators)</li>
     *       <li>Unexpected token type encountered</li>
     *     </ul>
     */
    public Expr parse() throws RpnException {
        Deque<Expr> stack = new ArrayDeque<>();

        while (!isAtEnd()) {
            Token token = current();

            if (token.type() == TokenType.NUMBER) {
                // Create Number node and push onto stack
                Number numNode = new Number(token.line(), token.column(), token.value());
                stack.push(numNode);
                advance();
            } else if (token.type() == TokenType.PLUS ||
                       token.type() == TokenType.MINUS ||
                       token.type() == TokenType.MULT ||
                       token.type() == TokenType.DIV) {
                // Validate sufficient operands
                if (stack.size() < 2) {
                    throw new RpnException(
                        "Operator '" + token.value() + "' requires two operands",
                        token.line(),
                        token.column()
                    );
                }

                // Pop operands (note: right before left due to stack order)
                Expr right = stack.pop();
                Expr left = stack.pop();

                // Map token type to operator string
                String op = switch (token.type()) {
                    case PLUS -> "+";
                    case MINUS -> "-";
                    case MULT -> "*";
                    case DIV -> "/";
                    default -> throw new AssertionError("Unreachable: validated above");
                };

                // Create BinaryOp node and push result
                BinaryOp opNode = new BinaryOp(token.line(), token.column(), op, left, right);
                stack.push(opNode);
                advance();
            } else if (token.type() == TokenType.EOF) {
                // End of input, break to validate stack
                break;
            } else {
                // Unknown token type
                throw new RpnException(
                    "Unexpected token '" + token.value() + "'",
                    token.line(),
                    token.column()
                );
            }
        }

        // Validate exactly one expression remains on stack
        if (stack.isEmpty()) {
            // Get EOF token for position info
            Token eofToken = tokens.get(tokens.size() - 1);
            throw new RpnException("Empty expression", eofToken.line(), eofToken.column());
        }

        if (stack.size() > 1) {
            // Get EOF token for position info
            Token eofToken = tokens.get(tokens.size() - 1);
            throw new RpnException(
                "Invalid RPN: " + stack.size() + " values remain on stack (missing operators?)",
                eofToken.line(),
                eofToken.column()
            );
        }

        // Return the single expression from the stack
        return stack.pop();
    }

    /**
     * Returns the current token without advancing.
     *
     * @return the current token
     */
    private Token current() {
        return tokens.get(pos);
    }

    /**
     * Checks if we've reached the end of the token stream.
     *
     * @return true if current token is EOF, false otherwise
     */
    private boolean isAtEnd() {
        return current().type() == TokenType.EOF;
    }

    /**
     * Advances to the next token.
     *
     * @return the token before advancing
     */
    private Token advance() {
        Token token = current();
        if (!isAtEnd()) {
            pos++;
        }
        return token;
    }
}
