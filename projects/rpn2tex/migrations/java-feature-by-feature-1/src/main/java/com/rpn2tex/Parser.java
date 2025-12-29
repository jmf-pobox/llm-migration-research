package com.rpn2tex;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.List;
import java.util.Objects;

/**
 * Stack-based RPN parser.
 *
 * <p>Converts a token stream into an Abstract Syntax Tree.
 * Uses a stack to accumulate operands and build expression trees
 * when operators are encountered.
 *
 * <p>RPN Parsing Algorithm:
 * <ol>
 *   <li>When you see a number, push it onto the stack</li>
 *   <li>When you see an operator, pop operands, create a node, push result</li>
 *   <li>At EOF, the stack should contain exactly one element: the AST root</li>
 * </ol>
 *
 * <p>Example usage:
 * <pre>
 *   Lexer lexer = new Lexer("5 3 +");
 *   List&lt;Token&gt; tokens = lexer.tokenize();
 *   ASTNode ast = new Parser(tokens).parse();
 *   // Result: BinaryOpNode("+", NumberNode("5"), NumberNode("3"))
 * </pre>
 */
public class Parser {
    private final List<Token> tokens;
    private int pos;

    /**
     * Creates a new parser for the given token list.
     *
     * @param tokens list of tokens from lexer (must end with EOF)
     * @throws NullPointerException if tokens is null
     */
    public Parser(List<Token> tokens) {
        this.tokens = Objects.requireNonNull(tokens, "tokens must not be null");
        this.pos = 0;
    }

    /**
     * Parses tokens into an AST.
     *
     * @return the root expression node of the AST
     * @throws ParserException if the input is invalid RPN
     */
    public ASTNode parse() throws ParserException {
        Deque<ASTNode> stack = new ArrayDeque<>();

        while (!atEnd()) {
            Token token = current();

            if (token.type() == TokenType.NUMBER) {
                // Push number onto stack
                NumberNode numNode = new NumberNode(
                    token.line(),
                    token.column(),
                    token.value()
                );
                stack.push(numNode);
                advance();
            } else if (token.type() == TokenType.PLUS
                       || token.type() == TokenType.MINUS
                       || token.type() == TokenType.MULT
                       || token.type() == TokenType.DIV) {
                // Binary operator: pop two operands and create BinaryOpNode
                if (stack.size() < 2) {
                    throw new ParserException(
                        String.format("Operator '%s' requires two operands", token.value()),
                        token
                    );
                }

                ASTNode right = stack.pop();
                ASTNode left = stack.pop();

                BinaryOpNode opNode = new BinaryOpNode(
                    token.line(),
                    token.column(),
                    token.value(),  // Use token value (+, -, or *)
                    left,
                    right
                );
                stack.push(opNode);
                advance();
            } else if (token.type() == TokenType.EOF) {
                break;
            } else {
                throw new ParserException(
                    String.format("Unexpected token '%s'", token.value()),
                    token
                );
            }
        }

        // Validate final state
        if (stack.isEmpty()) {
            Token eofToken = tokens.get(tokens.size() - 1);
            throw new ParserException("Empty expression", eofToken);
        }

        if (stack.size() > 1) {
            Token eofToken = tokens.get(tokens.size() - 1);
            throw new ParserException(
                String.format(
                    "Invalid RPN: %d values remain on stack (missing operators?)",
                    stack.size()
                ),
                eofToken
            );
        }

        return stack.pop();
    }

    private Token current() {
        return tokens.get(pos);
    }

    private boolean atEnd() {
        return tokens.get(pos).type() == TokenType.EOF;
    }

    private void advance() {
        if (!atEnd()) {
            pos++;
        }
    }
}
