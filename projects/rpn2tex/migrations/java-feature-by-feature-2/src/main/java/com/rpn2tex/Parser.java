package com.rpn2tex;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.List;

/**
 * Parser for RPN expressions.
 * Converts a stream of tokens into an AST using a stack-based algorithm.
 */
public class Parser {
    private final List<Token> tokens;
    private int current;

    /**
     * Creates a new parser for the given token stream.
     *
     * @param tokens List of tokens to parse
     */
    public Parser(List<Token> tokens) {
        this.tokens = tokens;
        this.current = 0;
    }

    /**
     * Parses the token stream into an expression AST.
     *
     * @return Root expression node
     * @throws RpnException If the expression is invalid
     */
    public Expr parse() throws RpnException {
        Deque<Expr> stack = new ArrayDeque<>();

        while (currentToken().type() != TokenType.EOF) {
            Token token = currentToken();

            if (token.type() == TokenType.NUMBER) {
                Number numNode = new Number(
                    token.line(),
                    token.column(),
                    token.value()
                );
                stack.push(numNode);
                advance();
            } else if (token.type() == TokenType.PLUS
                    || token.type() == TokenType.MINUS
                    || token.type() == TokenType.MULTIPLY
                    || token.type() == TokenType.DIVIDE) {
                if (stack.size() < 2) {
                    throw new RpnException(
                        "Operator '" + token.value() + "' requires two operands",
                        token
                    );
                }

                Expr right = stack.pop();
                Expr left = stack.pop();

                String operator = switch (token.type()) {
                    case PLUS -> "+";
                    case MINUS -> "-";
                    case MULTIPLY -> "*";
                    case DIVIDE -> "/";
                    default -> throw new RpnException("Unknown operator: " + token.type(), token);
                };

                BinaryOp opNode = new BinaryOp(
                    token.line(),
                    token.column(),
                    operator,
                    left,
                    right
                );
                stack.push(opNode);
                advance();
            } else {
                throw new RpnException("Unexpected token: " + token.type(), token);
            }
        }

        if (stack.isEmpty()) {
            throw new RpnException("Empty expression", currentToken());
        }

        if (stack.size() > 1) {
            throw new RpnException("Too many operands", currentToken());
        }

        return stack.pop();
    }

    private Token currentToken() {
        return tokens.get(current);
    }

    private void advance() {
        if (current < tokens.size() - 1) {
            current++;
        }
    }
}
