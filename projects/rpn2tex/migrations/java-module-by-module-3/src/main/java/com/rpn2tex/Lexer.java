package com.rpn2tex;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Tokenizer for RPN (Reverse Polish Notation) expressions.
 *
 * <p>The Lexer scans input text character-by-character and produces a stream
 * of {@link Token} objects representing the lexical elements of the expression.
 * It handles:
 * <ul>
 *   <li>Numeric literals (integers and decimals like "5", "3.14", "-2")</li>
 *   <li>Operators (+, -, *, /)</li>
 *   <li>Whitespace as token delimiters (spaces, tabs, newlines)</li>
 *   <li>Position tracking (1-based line and column numbers)</li>
 * </ul>
 *
 * <p>The lexer validates characters and reports errors for any unexpected input.
 * Unsupported characters (like '^') will cause a {@link RpnException} to be thrown.
 *
 * <p><b>Examples:</b>
 * <pre>{@code
 * // Basic tokenization
 * Lexer lexer = new Lexer("5 3 +");
 * List<Token> tokens = lexer.tokenize();
 * // Returns: [NUMBER("5"), NUMBER("3"), PLUS("+"), EOF]
 *
 * // Decimal numbers
 * Lexer lexer2 = new Lexer("3.14 2 *");
 * List<Token> tokens2 = lexer2.tokenize();
 * // Returns: [NUMBER("3.14"), NUMBER("2"), MULT("*"), EOF]
 *
 * // Error case
 * Lexer lexer3 = new Lexer("2 3 ^");
 * lexer3.tokenize(); // Throws RpnException for unexpected character '^'
 * }</pre>
 *
 * <p><b>Position Tracking:</b> The lexer maintains 1-based line and column
 * numbers for all tokens. Lines start at 1, columns start at 1. Newline
 * characters increment the line and reset the column to 1.
 *
 * <p><b>Negative Numbers:</b> The minus sign '-' is treated as a number prefix
 * if immediately followed by a digit (no whitespace). Otherwise, it's treated
 * as the subtraction operator.
 * <ul>
 *   <li>"5 -3" → NUMBER("5"), NUMBER("-3")</li>
 *   <li>"5 - 3" → NUMBER("5"), MINUS("-"), NUMBER("3")</li>
 * </ul>
 *
 * @since 1.0.0
 */
public final class Lexer {
    private final String text;
    private int pos;
    private int line;
    private int column;

    /**
     * Constructs a new Lexer for the given input text.
     *
     * @param text the input text to tokenize (must not be null)
     * @throws NullPointerException if text is null
     */
    public Lexer(String text) {
        this.text = Objects.requireNonNull(text, "text must not be null");
        this.pos = 0;
        this.line = 1;
        this.column = 1;
    }

    /**
     * Tokenizes the entire input and returns a list of tokens.
     *
     * <p>The returned list always ends with an EOF token. The list is unmodifiable
     * to preserve immutability guarantees.
     *
     * @return an unmodifiable list of tokens including EOF token
     * @throws RpnException if an unexpected character is encountered
     */
    public List<Token> tokenize() throws RpnException {
        List<Token> tokens = new ArrayList<>();

        while (!isAtEnd()) {
            skipWhitespace();
            if (isAtEnd()) {
                break;
            }
            tokens.add(scanToken());
        }

        // Always add EOF token at the end
        tokens.add(new Token(TokenType.EOF, "", line, column));
        return Collections.unmodifiableList(tokens);
    }

    /**
     * Checks if the lexer has reached the end of input.
     *
     * @return true if at end of input, false otherwise
     */
    private boolean isAtEnd() {
        return pos >= text.length();
    }

    /**
     * Peeks at the current character without consuming it.
     *
     * @return the current character, or '\0' if at end of input
     */
    private char peek() {
        if (isAtEnd()) {
            return '\0';
        }
        return text.charAt(pos);
    }

    /**
     * Consumes and returns the current character, advancing position.
     *
     * <p>This method updates line and column tracking. Newlines increment
     * the line number and reset the column to 1.
     *
     * @return the consumed character
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
     * Skips whitespace characters (space, tab, newline).
     *
     * <p>Multiple consecutive whitespace characters are skipped as a group.
     */
    private void skipWhitespace() {
        while (!isAtEnd() && Character.isWhitespace(peek())) {
            advance();
        }
    }

    /**
     * Scans a single token from the current position.
     *
     * <p>This method dispatches to the appropriate scanning function based
     * on the current character. It handles:
     * <ul>
     *   <li>Operators: +, -, *, /</li>
     *   <li>Numbers: digits and decimal points</li>
     *   <li>Negative numbers: minus followed by digit</li>
     * </ul>
     *
     * @return the scanned token
     * @throws RpnException if an unexpected character is encountered
     */
    private Token scanToken() throws RpnException {
        int startLine = line;
        int startColumn = column;
        char c = peek();

        switch (c) {
            case '+':
                advance();
                return new Token(TokenType.PLUS, "+", startLine, startColumn);
            case '*':
                advance();
                return new Token(TokenType.MULT, "*", startLine, startColumn);
            case '/':
                advance();
                return new Token(TokenType.DIV, "/", startLine, startColumn);
            case '-':
                advance();
                // Check if this is a negative number (minus followed by digit)
                if (!isAtEnd() && Character.isDigit(peek())) {
                    return scanNumber("-", startLine, startColumn);
                }
                // Otherwise, it's a subtraction operator
                return new Token(TokenType.MINUS, "-", startLine, startColumn);
            default:
                // Check if it's a digit (start of a number)
                if (Character.isDigit(c)) {
                    return scanNumber("", startLine, startColumn);
                }
                // Unexpected character - throw error
                throw new RpnException("Unexpected character '" + c + "'", startLine, startColumn);
        }
    }

    /**
     * Scans a numeric literal from the current position.
     *
     * <p>Numbers can be:
     * <ul>
     *   <li>Integers: "5", "123"</li>
     *   <li>Decimals: "3.14", "0.5"</li>
     *   <li>Negative: "-5", "-3.14"</li>
     * </ul>
     *
     * <p>The number is accumulated as a string to preserve the exact input format
     * (no conversion to numeric types).
     *
     * @param prefix the prefix to prepend (e.g., "-" for negative numbers)
     * @param startLine the 1-based line number where the number starts
     * @param startColumn the 1-based column number where the number starts
     * @return a NUMBER token containing the complete numeric string
     */
    private Token scanNumber(String prefix, int startLine, int startColumn) {
        StringBuilder sb = new StringBuilder(prefix);

        // Scan integer part (sequence of digits)
        while (!isAtEnd() && Character.isDigit(peek())) {
            sb.append(advance());
        }

        // Check for decimal part
        if (!isAtEnd() && peek() == '.') {
            sb.append(advance()); // Consume the '.'

            // Scan fractional part (sequence of digits after decimal point)
            while (!isAtEnd() && Character.isDigit(peek())) {
                sb.append(advance());
            }
        }

        return new Token(TokenType.NUMBER, sb.toString(), startLine, startColumn);
    }
}
