package com.rpn2tex;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

/**
 * Tokenizes RPN expressions into a stream of tokens.
 *
 * <p>The lexer performs character-by-character scanning of the input text,
 * recognizing the following token types:
 * <ul>
 *   <li>Numbers: integers and decimals (e.g., "42", "3.14", "-5")</li>
 *   <li>Operators: +, -, *, /</li>
 *   <li>Whitespace: space, tab, newline, carriage return (skipped)</li>
 * </ul>
 *
 * <p>Special handling for negative numbers:
 * <ul>
 *   <li>If '-' is followed immediately by a digit, it's part of a negative number</li>
 *   <li>Otherwise, '-' is treated as the subtraction operator</li>
 * </ul>
 *
 * <p>Position tracking:
 * <ul>
 *   <li>Line numbers are 1-based</li>
 *   <li>Column numbers are 1-based</li>
 *   <li>Newlines increment line and reset column to 1</li>
 * </ul>
 *
 * <p>Example usage:
 * <pre>{@code
 * Lexer lexer = new Lexer("5 3 +");
 * List<Token> tokens = lexer.tokenize();
 * // tokens: [NUMBER("5"), NUMBER("3"), PLUS("+"), EOF("")]
 * }</pre>
 *
 * @throws RpnException if an unexpected character is encountered (e.g., '^')
 */
public final class Lexer {
    private final String text;
    private int pos;
    private int line;
    private int column;

    /**
     * Constructs a new Lexer for the given input text.
     *
     * @param text the input text to tokenize
     * @throws NullPointerException if text is null
     */
    public Lexer(String text) {
        this.text = Objects.requireNonNull(text, "text must not be null");
        this.pos = 0;
        this.line = 1;
        this.column = 1;
    }

    /**
     * Tokenizes the input text into a list of tokens.
     *
     * <p>The returned list always ends with an EOF token. Position information
     * is tracked throughout the tokenization process for error reporting.
     *
     * @return list of tokens including a trailing EOF token
     * @throws RpnException if an unexpected character is encountered
     */
    public List<Token> tokenize() throws RpnException {
        List<Token> tokens = new ArrayList<>();

        while (!isAtEnd()) {
            skipWhitespace();
            if (isAtEnd()) {
                break;
            }
            Token token = scanToken();
            tokens.add(token);
        }

        // Add EOF token at final position
        tokens.add(new Token(TokenType.EOF, "", line, column));
        return tokens;
    }

    /**
     * Scans and returns the next token from the input.
     *
     * @return the next token
     * @throws RpnException if an unexpected character is encountered
     */
    private Token scanToken() throws RpnException {
        char c = peek();
        int tokenLine = line;
        int tokenColumn = column;

        switch (c) {
            case '+':
                advance();
                return new Token(TokenType.PLUS, "+", tokenLine, tokenColumn);

            case '-':
                // Check for negative number: '-' followed immediately by digit
                advance();
                if (!isAtEnd() && Character.isDigit(peek())) {
                    return scanNumber("-", tokenLine, tokenColumn);
                }
                return new Token(TokenType.MINUS, "-", tokenLine, tokenColumn);

            case '*':
                advance();
                return new Token(TokenType.MULT, "*", tokenLine, tokenColumn);

            case '/':
                advance();
                return new Token(TokenType.DIV, "/", tokenLine, tokenColumn);

            default:
                if (Character.isDigit(c)) {
                    return scanNumber("", tokenLine, tokenColumn);
                } else {
                    // Unexpected character
                    throw new RpnException(
                        String.format("Unexpected character '%c'", c),
                        line,
                        column
                    );
                }
        }
    }

    /**
     * Scans a numeric token (integer or decimal).
     *
     * <p>Numbers can have an optional prefix (e.g., "-" for negative numbers).
     * Decimals are recognized by the presence of a decimal point followed by digits.
     *
     * @param prefix the prefix string (e.g., "-" for negative numbers)
     * @param tokenLine the line number where the token starts
     * @param tokenColumn the column number where the token starts
     * @return a NUMBER token
     */
    private Token scanNumber(String prefix, int tokenLine, int tokenColumn) {
        StringBuilder value = new StringBuilder(prefix);

        // Scan integer part
        while (!isAtEnd() && Character.isDigit(peek())) {
            value.append(advance());
        }

        // Check for decimal point
        if (!isAtEnd() && peek() == '.') {
            value.append(advance());

            // Scan fractional part
            while (!isAtEnd() && Character.isDigit(peek())) {
                value.append(advance());
            }
        }

        return new Token(TokenType.NUMBER, value.toString(), tokenLine, tokenColumn);
    }

    /**
     * Skips whitespace characters (space, tab, newline, carriage return).
     *
     * <p>Updates position tracking appropriately: newlines increment line
     * and reset column to 1.
     */
    private void skipWhitespace() {
        while (!isAtEnd()) {
            char c = peek();
            if (c == ' ' || c == '\t' || c == '\n' || c == '\r') {
                advance();
            } else {
                break;
            }
        }
    }

    /**
     * Returns the current character without advancing.
     *
     * @return the current character
     */
    private char peek() {
        if (isAtEnd()) {
            return '\0';
        }
        return text.charAt(pos);
    }

    /**
     * Advances to the next character and returns the current one.
     *
     * <p>Updates line and column tracking:
     * <ul>
     *   <li>Newline: increment line, reset column to 1</li>
     *   <li>Other: increment column</li>
     * </ul>
     *
     * @return the current character before advancing
     */
    private char advance() {
        char c = text.charAt(pos);
        pos++;

        if (c == '\n') {
            line++;
            column = 1;
        } else {
            column++;
        }

        return c;
    }

    /**
     * Checks if we've reached the end of the input.
     *
     * @return true if at end of input, false otherwise
     */
    private boolean isAtEnd() {
        return pos >= text.length();
    }
}
