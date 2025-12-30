package com.rpn2tex;

import java.util.*;

/**
 * RPN Parser that converts a list of tokens into an abstract syntax tree (AST).
 *
 * <p>Uses stack-based evaluation to parse Reverse Polish Notation expressions:
 * - Numbers are pushed onto the stack
 * - Operators pop two operands (right then left) and push a BinaryOp node
 * - Final validation ensures exactly one value remains on the stack
 *
 * <p>Example: "5 3 +" becomes BinaryOp("+", Number(5), Number(3))
 *
 * @see Expr
 * @see Token
 */
public class Parser {
    private final List<Token> tokens;
    private int pos;

    /**
     * Creates a new parser for the given token list.
     *
     * @param tokens the list of tokens to parse (must include EOF token)
     */
    public Parser(List<Token> tokens) {
        this.tokens = tokens;
        this.pos = 0;
    }

    /**
     * Parses the token list into an expression tree.
     *
     * @return the root expression node
     * @throws RpnException if the expression is invalid (empty, insufficient operands, or extra values)
     */
    public Expr parse() throws RpnException {
        Stack<Expr> stack = new Stack<>();

        while (!atEnd()) {
            Token token = current();

            if (token.type == TokenType.NUMBER) {
                stack.push(new Number(token.value, token.line, token.column));
                advance();

            } else if (token.type == TokenType.PLUS || token.type == TokenType.MINUS ||
                       token.type == TokenType.MULT || token.type == TokenType.DIV) {

                if (stack.size() < 2) {
                    throw new RpnException(
                        "Operator '" + token.value + "' requires two operands",
                        token.line, token.column
                    );
                }

                Expr right = stack.pop();
                Expr left = stack.pop();
                String operator = tokenTypeToOperator(token.type);

                stack.push(new BinaryOp(operator, left, right, token.line, token.column));
                advance();

            } else if (token.type == TokenType.EOF) {
                break;
            }
        }

        if (stack.isEmpty()) {
            Token eofToken = tokens.get(tokens.size() - 1);
            throw new RpnException("Empty expression", eofToken.line, eofToken.column);
        }

        if (stack.size() > 1) {
            Token eofToken = tokens.get(tokens.size() - 1);
            throw new RpnException(
                "Invalid RPN: " + stack.size() + " values remain on stack (missing operators?)",
                eofToken.line, eofToken.column
            );
        }

        return stack.pop();
    }

    private Token current() {
        return tokens.get(pos);
    }

    private boolean atEnd() {
        return tokens.get(pos).type == TokenType.EOF;
    }

    private void advance() {
        if (!atEnd()) {
            pos++;
        }
    }

    private String tokenTypeToOperator(TokenType type) {
        switch (type) {
            case PLUS: return "+";
            case MINUS: return "-";
            case MULT: return "*";
            case DIV: return "/";
            default: throw new AssertionError("Invalid operator type");
        }
    }
}
