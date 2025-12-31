package com.rpn2tex;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

/**
 * Stack-based parser that converts a stream of tokens into an Abstract Syntax Tree (AST).
 *
 * <p>This parser implements the standard Reverse Polish Notation (RPN) parsing algorithm:
 * <ol>
 *   <li>Iterate through each token in sequence</li>
 *   <li>If token is a NUMBER: push it onto the stack as a Number node</li>
 *   <li>If token is an OPERATOR: pop two operands, create a BinaryOp node, and push result</li>
 *   <li>At the end: the stack must contain exactly one element (the root AST node)</li>
 * </ol>
 *
 * <p>RPN parsing ensures that mathematical expressions are properly structured. For example:
 * <ul>
 *   <li>{@code "5 3 +"} becomes {@code BinaryOp("+", Number("5"), Number("3"))}</li>
 *   <li>{@code "5 3 + 2 *"} becomes {@code BinaryOp("*", BinaryOp("+", ...), Number("2"))}</li>
 * </ul>
 *
 * <p>The parser validates RPN structure and provides detailed error messages for:
 * <ul>
 *   <li>Empty expressions (no tokens)</li>
 *   <li>Insufficient operands for operators</li>
 *   <li>Extra operands left on stack (missing operators)</li>
 * </ul>
 *
 * <p>Example usage:
 * <pre>{@code
 * List<Token> tokens = lexer.tokenize();
 * Parser parser = new Parser(tokens);
 * Expr ast = parser.parse();
 * }</pre>
 *
 * @since 1.0
 */
public class Parser {
    private final List<Token> tokens;
    private int pos;

    /**
     * Constructs a new parser with the given token list.
     *
     * <p>The token list must end with an EOF token (as produced by the Lexer).
     *
     * @param tokens the list of tokens to parse
     * @throws NullPointerException if tokens is null
     */
    public Parser(List<Token> tokens) {
        Objects.requireNonNull(tokens, "Token list cannot be null");
        this.tokens = tokens;
        this.pos = 0;
    }

    /**
     * Parses the token stream into an Abstract Syntax Tree (AST).
     *
     * <p>This method implements the RPN stack algorithm:
     * <ol>
     *   <li>Initialize an empty stack</li>
     *   <li>For each token until EOF:
     *     <ul>
     *       <li>NUMBER token: Create Number node, push to stack</li>
     *       <li>OPERATOR token: Pop 2 operands, create BinaryOp node, push result</li>
     *     </ul>
     *   </li>
     *   <li>Validate: Stack must contain exactly one element</li>
     * </ol>
     *
     * @return the root expression node of the AST
     * @throws RpnException if the RPN expression is malformed:
     *                      <ul>
     *                        <li>Empty expression (no tokens)</li>
     *                        <li>Operator without sufficient operands</li>
     *                        <li>Extra operands remaining on stack</li>
     *                      </ul>
     */
    public Expr parse() throws RpnException {
        List<Expr> stack = new ArrayList<>();

        while (!atEnd()) {
            Token token = current();

            if (token.type() == TokenType.NUMBER) {
                // Create a Number node and push to stack
                Number numNode = new Number(
                    token.line(),
                    token.column(),
                    token.value()
                );
                stack.add(numNode);
                advance();

            } else if (isOperator(token.type())) {
                // Validate: Need at least 2 operands on stack
                if (stack.size() < 2) {
                    throw new RpnException(
                        String.format("Operator '%s' requires two operands", token.value()),
                        token.line(),
                        token.column()
                    );
                }

                // Pop right operand first (RPN order)
                Expr right = stack.remove(stack.size() - 1);
                // Pop left operand second
                Expr left = stack.remove(stack.size() - 1);

                // Map token type to operator string
                String operator = tokenTypeToOperator(token.type());

                // Create BinaryOp node and push to stack
                BinaryOp opNode = new BinaryOp(
                    token.line(),
                    token.column(),
                    operator,
                    left,
                    right
                );
                stack.add(opNode);
                advance();

            } else if (token.type() == TokenType.EOF) {
                break;

            } else {
                // Unexpected token type
                throw new RpnException(
                    String.format("Unexpected token '%s'", token.value()),
                    token.line(),
                    token.column()
                );
            }
        }

        // Validate final state: stack must have exactly one element
        if (stack.isEmpty()) {
            Token eofToken = tokens.get(tokens.size() - 1);
            throw new RpnException(
                "Empty expression",
                eofToken.line(),
                eofToken.column()
            );
        }

        if (stack.size() > 1) {
            Token eofToken = tokens.get(tokens.size() - 1);
            throw new RpnException(
                String.format("Invalid RPN: %d values remain on stack (missing operators?)",
                    stack.size()),
                eofToken.line(),
                eofToken.column()
            );
        }

        return stack.get(0);
    }

    /**
     * Returns the current token at the current position.
     *
     * @return the current token
     */
    private Token current() {
        return tokens.get(pos);
    }

    /**
     * Checks if the parser has reached the end of the token stream.
     *
     * <p>The end is identified by the EOF token type.
     *
     * @return true if at EOF, false otherwise
     */
    private boolean atEnd() {
        return tokens.get(pos).type() == TokenType.EOF;
    }

    /**
     * Advances to the next token in the stream.
     *
     * <p>This method does not advance past the EOF token.
     *
     * @return the token that was current before advancing
     */
    private Token advance() {
        Token token = tokens.get(pos);
        if (!atEnd()) {
            pos++;
        }
        return token;
    }

    /**
     * Checks if a token type represents an operator.
     *
     * @param type the token type to check
     * @return true if the type is PLUS, MINUS, MULT, or DIV
     */
    private boolean isOperator(TokenType type) {
        return type == TokenType.PLUS || type == TokenType.MINUS ||
               type == TokenType.MULT || type == TokenType.DIV;
    }

    /**
     * Maps a token type to its corresponding operator string.
     *
     * <p>This mapping is used to create BinaryOp nodes with the correct operator.
     *
     * @param type the token type (must be an operator type)
     * @return the operator string ("+", "-", "*", or "/")
     * @throws IllegalArgumentException if the type is not an operator
     */
    private String tokenTypeToOperator(TokenType type) {
        return switch (type) {
            case PLUS -> "+";
            case MINUS -> "-";
            case MULT -> "*";
            case DIV -> "/";
            default -> throw new IllegalArgumentException("Not an operator: " + type);
        };
    }
}
