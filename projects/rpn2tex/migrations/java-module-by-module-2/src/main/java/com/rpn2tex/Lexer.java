package com.rpn2tex;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

/**
 * Tokenizes RPN expression input into a stream of tokens.
 *
 * <p>The lexer performs character-by-character scanning and recognizes:
 * <ul>
 *   <li>Numbers (integers and decimals, including negative)</li>
 *   <li>Operators: +, -, *, /</li>
 *   <li>Whitespace (used as delimiter, skipped)</li>
 * </ul>
 *
 * <p>Position tracking (line/column) is maintained for error reporting.
 * Both line and column numbers are 1-based (human-readable).
 *
 * <p>Example usage:
 * <pre>{@code
 * Lexer lexer = new Lexer("5 3 +");
 * List<Token> tokens = lexer.tokenize();
 * // tokens: [NUMBER("5"), NUMBER("3"), PLUS("+"), EOF("")]
 * }</pre>
 *
 * <p>Error handling:
 * <pre>{@code
 * try {
 *     Lexer lexer = new Lexer("2 3 ^");
 *     List<Token> tokens = lexer.tokenize();
 * } catch (RpnException e) {
 *     // e.getMessage() returns "Line 1, column 5: Unexpected character '^'"
 * }
 * }</pre>
 *
 * @since 1.0
 */
public class Lexer {
    private final String text;
    private int pos;
    private int line;
    private int column;

    /**
     * Constructs a new lexer for the given input text.
     *
     * @param text the RPN expression to tokenize
     * @throws NullPointerException if text is null
     */
    public Lexer(String text) {
        Objects.requireNonNull(text, "Input text cannot be null");
        this.text = text;
        this.pos = 0;
        this.line = 1;
        this.column = 1;
    }

    /**
     * Tokenizes the input text into a list of tokens.
     *
     * <p>The token list always ends with an EOF token.
     * Position information is captured for each token for error reporting.
     *
     * @return list of tokens ending with EOF token
     * @throws RpnException if an invalid character is encountered
     */
    public List<Token> tokenize() throws RpnException {
        List<Token> tokens = new ArrayList<>();

        while (!atEnd()) {
            skipWhitespace();
            if (atEnd()) {
                break;
            }
            tokens.add(scanToken());
        }

        tokens.add(new Token(TokenType.EOF, "", line, column));
        return tokens;
    }

    /**
     * Checks if we've reached the end of input.
     *
     * @return true if at end of input
     */
    private boolean atEnd() {
        return pos >= text.length();
    }

    /**
     * Peeks at the current character without consuming it.
     *
     * @return current character, or '\0' if at end
     */
    private char peek() {
        if (atEnd()) {
            return '\0';
        }
        return text.charAt(pos);
    }

    /**
     * Consumes and returns the current character, updating position tracking.
     *
     * <p>Position tracking:
     * <ul>
     *   <li>Newline: increments line, resets column to 1</li>
     *   <li>Other characters: increments column</li>
     * </ul>
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
     * Skips whitespace characters (space, tab, newline, carriage return).
     */
    private void skipWhitespace() {
        while (!atEnd() && Character.isWhitespace(peek())) {
            advance();
        }
    }

    /**
     * Scans and returns the next token from the input.
     *
     * <p>Token recognition:
     * <ul>
     *   <li>'+' → PLUS token</li>
     *   <li>'-' → MINUS token (or part of negative number if followed by digit)</li>
     *   <li>'*' → MULT token</li>
     *   <li>'/' → DIV token</li>
     *   <li>Digit → NUMBER token (integer or decimal)</li>
     *   <li>Other → RpnException</li>
     * </ul>
     *
     * @return the scanned token
     * @throws RpnException if an unexpected character is encountered
     */
    private Token scanToken() throws RpnException {
        int startLine = line;
        int startColumn = column;
        char c = peek();

        // Single-character operators
        if (c == '+') {
            advance();
            return new Token(TokenType.PLUS, "+", startLine, startColumn);
        }
        if (c == '-') {
            advance();
            // Check if this is a negative number (minus followed immediately by digit)
            if (!atEnd() && Character.isDigit(peek())) {
                return scanNumber("-", startLine, startColumn);
            }
            return new Token(TokenType.MINUS, "-", startLine, startColumn);
        }
        if (c == '*') {
            advance();
            return new Token(TokenType.MULT, "*", startLine, startColumn);
        }
        if (c == '/') {
            advance();
            return new Token(TokenType.DIV, "/", startLine, startColumn);
        }

        // Numbers
        if (Character.isDigit(c)) {
            return scanNumber("", startLine, startColumn);
        }

        // Unknown character
        throw new RpnException(String.format("Unexpected character '%c'", c), startLine, startColumn);
    }

    /**
     * Scans a numeric literal (integer or decimal).
     *
     * <p>Number format:
     * <ul>
     *   <li>Optional prefix (e.g., "-" for negative numbers)</li>
     *   <li>Integer part: sequence of digits</li>
     *   <li>Optional decimal part: "." followed by digits</li>
     * </ul>
     *
     * <p>Examples: 5, 3.14, -2, 0.5
     *
     * @param prefix      optional prefix (e.g., "-" for negative numbers)
     * @param startLine   line number where the token starts
     * @param startColumn column number where the token starts
     * @return NUMBER token with the numeric value as a string
     */
    private Token scanNumber(String prefix, int startLine, int startColumn) {
        StringBuilder value = new StringBuilder(prefix);

        // Integer part
        while (!atEnd() && Character.isDigit(peek())) {
            value.append(advance());
        }

        // Decimal part (optional)
        if (!atEnd() && peek() == '.') {
            value.append(advance());  // consume '.'
            while (!atEnd() && Character.isDigit(peek())) {
                value.append(advance());
            }
        }

        return new Token(TokenType.NUMBER, value.toString(), startLine, startColumn);
    }
}
