package com.rpn2tex;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.List;
import java.util.Objects;

/**
 * Parses a stream of tokens into an Abstract Syntax Tree (AST) using a stack-based RPN algorithm.
 *
 * <p>The parser implements the standard Reverse Polish Notation (RPN) algorithm:
 * <ul>
 *   <li>Numbers are pushed onto a stack</li>
 *   <li>Operators pop two operands, create a BinaryOp node, and push the result</li>
 *   <li>At EOF, the stack must contain exactly one expression (the root of the AST)</li>
 * </ul>
 *
 * <p>Example usage:
 * <pre>{@code
 * List<Token> tokens = lexer.tokenize();
 * Parser parser = new Parser(tokens);
 * Expr ast = parser.parse();  // Returns the root of the AST
 * }</pre>
 *
 * <p>Error cases:
 * <ul>
 *   <li>Empty expression (no tokens before EOF)</li>
 *   <li>Insufficient operands for an operator (stack has fewer than 2 items)</li>
 *   <li>Too many operands (stack has more than 1 item at EOF)</li>
 * </ul>
 */
public final class Parser {
    private final List<Token> tokens;
    private int pos;

    /**
     * Constructs a new Parser with the given token stream.
     *
     * @param tokens the list of tokens to parse (must include EOF token at end)
     * @throws NullPointerException if tokens is null
     */
    public Parser(List<Token> tokens) {
        this.tokens = Objects.requireNonNull(tokens, "tokens must not be null");
        this.pos = 0;
    }

    /**
     * Parses the token stream into an AST.
     *
     * <p>This method implements the stack-based RPN parsing algorithm:
     * <ol>
     *   <li>Initialize an empty stack</li>
     *   <li>For each token:
     *     <ul>
     *       <li>NUMBER: create a Number node and push onto stack</li>
     *       <li>OPERATOR (+, -, *, /): pop right operand, pop left operand,
     *           create BinaryOp node, push onto stack</li>
     *       <li>EOF: stop processing</li>
     *     </ul>
     *   </li>
     *   <li>Validate that exactly one expression remains on the stack</li>
     * </ol>
     *
     * @return the root expression of the AST
     * @throws RpnException if parsing fails due to:
     *         <ul>
     *           <li>Empty expression</li>
     *           <li>Insufficient operands for an operator</li>
     *           <li>Too many operands remaining</li>
     *         </ul>
     */
    public Expr parse() throws RpnException {
        Deque<Expr> stack = new ArrayDeque<>();

        while (!isAtEnd()) {
            Token token = current();

            switch (token.type()) {
                case NUMBER -> {
                    Expr node = new Number(token.line(), token.column(), token.value());
                    stack.push(node);
                    advance();
                }
                case PLUS, MINUS, MULT, DIV -> {
                    if (stack.size() < 2) {
                        throw new RpnException(
                            String.format("Not enough operands for operator '%s'", token.value()),
                            token.line(),
                            token.column()
                        );
                    }
                    // Pop right operand first (LIFO order)
                    Expr right = stack.pop();
                    // Then pop left operand
                    Expr left = stack.pop();

                    // Map TokenType to operator string
                    String operator = switch (token.type()) {
                        case PLUS -> "+";
                        case MINUS -> "-";
                        case MULT -> "*";
                        case DIV -> "/";
                        default -> throw new IllegalStateException("Unexpected operator: " + token.type());
                    };

                    Expr node = new BinaryOp(token.line(), token.column(), operator, left, right);
                    stack.push(node);
                    advance();
                }
                case EOF -> {
                    // Stop processing at EOF
                    break;
                }
                default -> throw new RpnException(
                    String.format("Unexpected token type: %s", token.type()),
                    token.line(),
                    token.column()
                );
            }

            // Check if we hit EOF
            if (token.type() == TokenType.EOF) {
                break;
            }
        }

        // Validate final stack state
        if (stack.isEmpty()) {
            // Empty expression - no tokens before EOF
            Token eofToken = tokens.isEmpty() ?
                new Token(TokenType.EOF, "", 1, 1) :
                tokens.get(tokens.size() - 1);
            throw new RpnException("Empty expression", eofToken.line(), eofToken.column());
        }

        if (stack.size() > 1) {
            // Too many operands - more than one value remains on stack
            Token lastToken = pos > 0 ? tokens.get(pos - 1) : tokens.get(0);
            throw new RpnException(
                String.format("%d values remain on stack", stack.size()),
                lastToken.line(),
                lastToken.column()
            );
        }

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
     * Advances to the next token.
     */
    private void advance() {
        if (!isAtEnd()) {
            pos++;
        }
    }

    /**
     * Checks if we've reached the end of the token stream.
     *
     * @return true if at end, false otherwise
     */
    private boolean isAtEnd() {
        return pos >= tokens.size() || current().type() == TokenType.EOF;
    }
}
