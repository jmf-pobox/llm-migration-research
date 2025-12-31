package com.rpn2tex;

import java.util.ArrayList;
import java.util.List;

/**
 * Lexical analyzer for RPN expressions.
 * <p>
 * The lexer scans the input text character-by-character and produces
 * a sequence of tokens. It handles:
 * </p>
 * <ul>
 *   <li>Numeric literals (integers and decimals)</li>
 *   <li>Whitespace (ignored)</li>
 *   <li>Position tracking for error reporting</li>
 * </ul>
 * <p>
 * Example usage:
 * </p>
 * <pre>
 * Lexer lexer = new Lexer("5 3.14");
 * List&lt;Token&gt; tokens = lexer.tokenize();
 * </pre>
 */
public class Lexer {
    private final String text;
    private int position;
    private int line;
    private int column;

    /**
     * Constructs a new Lexer for the given input text.
     *
     * @param text The input text to tokenize
     */
    public Lexer(String text) {
        this.text = text != null ? text : "";
        this.position = 0;
        this.line = 1;
        this.column = 1;
    }

    /**
     * Tokenizes the entire input text.
     *
     * @return A list of tokens, ending with EOF
     * @throws LexerException if an invalid character is encountered
     */
    public List<Token> tokenize() throws LexerException {
        List<Token> tokens = new ArrayList<>();

        while (!atEnd()) {
            Token token = nextToken();
            tokens.add(token);
            if (token.type() == TokenType.EOF) {
                break;
            }
        }

        // Ensure EOF token is present
        if (tokens.isEmpty() || tokens.get(tokens.size() - 1).type() != TokenType.EOF) {
            tokens.add(new Token(TokenType.EOF, "", line, column));
        }

        return tokens;
    }

    /**
     * Scans and returns the next token from the input.
     *
     * @return The next token
     * @throws LexerException if an invalid character is encountered
     */
    private Token nextToken() throws LexerException {
        skipWhitespace();

        if (atEnd()) {
            return new Token(TokenType.EOF, "", line, column);
        }

        char current = peek();
        int startLine = line;
        int startColumn = column;

        // Check for numbers
        if (Character.isDigit(current)) {
            return scanNumber("", startLine, startColumn);
        }

        // Check for addition operator
        if (current == '+') {
            advance();
            return new Token(TokenType.PLUS, "+", startLine, startColumn);
        }

        // Check for minus sign (could be negative number or subtraction operator)
        if (current == '-') {
            advance();
            if (!atEnd() && Character.isDigit(peek())) {
                // It's a negative number (- followed immediately by digit)
                return scanNumber("-", startLine, startColumn);
            }
            // It's a subtraction operator
            return new Token(TokenType.MINUS, "-", startLine, startColumn);
        }

        // Check for multiplication operator
        if (current == '*') {
            advance();
            return new Token(TokenType.TIMES, "*", startLine, startColumn);
        }

        // Check for division operator
        if (current == '/') {
            advance();
            return new Token(TokenType.DIVIDE, "/", startLine, startColumn);
        }

        // Unknown character
        throw new LexerException("Unexpected character: '" + current + "'", startLine, startColumn);
    }

    /**
     * Scans a numeric literal (integer or decimal).
     *
     * @param prefix      The prefix already consumed (e.g., "-" for negative numbers)
     * @param startLine   The line where the number starts
     * @param startColumn The column where the number starts
     * @return A NUMBER token
     */
    private Token scanNumber(String prefix, int startLine, int startColumn) {
        StringBuilder value = new StringBuilder(prefix);

        // Scan integer part
        while (!atEnd() && Character.isDigit(peek())) {
            value.append(advance());
        }

        // Scan decimal part (if present)
        if (!atEnd() && peek() == '.') {
            value.append(advance()); // consume '.'
            while (!atEnd() && Character.isDigit(peek())) {
                value.append(advance());
            }
        }

        return new Token(TokenType.NUMBER, value.toString(), startLine, startColumn);
    }

    /**
     * Skips whitespace characters.
     */
    private void skipWhitespace() {
        while (!atEnd() && Character.isWhitespace(peek())) {
            advance();
        }
    }

    /**
     * Returns the current character without consuming it.
     *
     * @return The current character
     */
    private char peek() {
        if (atEnd()) {
            return '\0';
        }
        return text.charAt(position);
    }

    /**
     * Consumes and returns the current character, advancing position.
     *
     * @return The consumed character
     */
    private char advance() {
        char current = text.charAt(position);
        position++;

        if (current == '\n') {
            line++;
            column = 1;
        } else {
            column++;
        }

        return current;
    }

    /**
     * Checks if we've reached the end of the input.
     *
     * @return true if at end, false otherwise
     */
    private boolean atEnd() {
        return position >= text.length();
    }
}
