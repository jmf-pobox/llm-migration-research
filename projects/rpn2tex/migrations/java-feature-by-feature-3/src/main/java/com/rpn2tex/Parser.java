package com.rpn2tex;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.List;

/**
 * Parser for RPN expressions.
 * <p>
 * The parser uses a stack-based approach to evaluate RPN notation
 * and build an abstract syntax tree (AST). It processes tokens from
 * left to right:
 * </p>
 * <ul>
 *   <li>Numbers are pushed onto the stack</li>
 *   <li>Operators pop operands from the stack and push results</li>
 * </ul>
 * <p>
 * Example usage:
 * </p>
 * <pre>
 * List&lt;Token&gt; tokens = lexer.tokenize();
 * Parser parser = new Parser(tokens);
 * Expr ast = parser.parse();
 * </pre>
 */
public class Parser {
    private final List<Token> tokens;
    private int position;

    /**
     * Constructs a new Parser for the given token list.
     *
     * @param tokens The list of tokens to parse
     */
    public Parser(List<Token> tokens) {
        this.tokens = tokens;
        this.position = 0;
    }

    /**
     * Parses the tokens and returns the root AST node.
     *
     * @return The root expression node
     * @throws ParserException if the token sequence is invalid
     */
    public Expr parse() throws ParserException {
        Deque<Expr> stack = new ArrayDeque<>();

        while (!atEnd()) {
            Token token = current();

            if (token.type() == TokenType.EOF) {
                break;
            }

            if (token.type() == TokenType.NUMBER) {
                // Push number onto stack
                NumberExpr numberExpr = new NumberExpr(
                        token.value(),
                        token.line(),
                        token.column()
                );
                stack.push(numberExpr);
                advance();
            } else if (token.type() == TokenType.PLUS || token.type() == TokenType.MINUS || token.type() == TokenType.TIMES || token.type() == TokenType.DIVIDE) {
                // Binary operator: pop two operands, push result
                if (stack.size() < 2) {
                    throw new ParserException(
                            "Operator '" + token.value() + "' requires two operands",
                            token
                    );
                }

                Expr right = stack.pop();
                Expr left = stack.pop();

                // Map token type to operator string
                String operator;
                if (token.type() == TokenType.PLUS) {
                    operator = "+";
                } else if (token.type() == TokenType.MINUS) {
                    operator = "-";
                } else if (token.type() == TokenType.TIMES) {
                    operator = "*";
                } else {
                    operator = "/";
                }

                BinaryOpExpr binaryOpExpr = new BinaryOpExpr(
                        operator,
                        left,
                        right,
                        token.line(),
                        token.column()
                );
                stack.push(binaryOpExpr);
                advance();
            } else {
                // Unknown token type
                throw new ParserException("Unexpected token: " + token.type(), token);
            }
        }

        // Validate stack state
        if (stack.isEmpty()) {
            Token eofToken = tokens.get(tokens.size() - 1);
            throw new ParserException("Empty expression", eofToken);
        }

        if (stack.size() > 1) {
            throw new ParserException(
                    "Expression has too many operands (expected 1, got " + stack.size() + ")",
                    tokens.get(tokens.size() - 1)
            );
        }

        return stack.pop();
    }

    /**
     * Returns the current token without consuming it.
     *
     * @return The current token
     */
    private Token current() {
        if (atEnd()) {
            return tokens.get(tokens.size() - 1); // Should be EOF
        }
        return tokens.get(position);
    }

    /**
     * Advances to the next token.
     */
    private void advance() {
        if (!atEnd()) {
            position++;
        }
    }

    /**
     * Checks if we've reached the end of the token list.
     *
     * @return true if at end, false otherwise
     */
    private boolean atEnd() {
        return position >= tokens.size() || tokens.get(position).type() == TokenType.EOF;
    }
}
