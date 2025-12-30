package com.rpn2tex;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

/**
 * Tokenizes RPN (Reverse Polish Notation) input text into a stream of tokens.
 *
 * <p>The lexer recognizes:
 * <ul>
 *   <li>Numbers (integers and decimals, including negative numbers)</li>
 *   <li>Addition operator (+)</li>
 *   <li>Subtraction operator (-)</li>
 *   <li>Multiplication operator (*)</li>
 *   <li>Division operator (/)</li>
 *   <li>EOF marker</li>
 * </ul>
 *
 * <p>Whitespace (spaces, tabs, newlines, carriage returns) is used as a delimiter
 * and is otherwise ignored. Position tracking is maintained throughout for accurate
 * error reporting with 1-based line and column numbers.
 *
 * <p>The minus sign (-) is recognized as either a negative number prefix or a
 * subtraction operator based on lookahead: if immediately followed by a digit,
 * it's a negative number; otherwise, it's a subtraction operator.
 *
 * <p>Example usage:
 * <pre>
 * Lexer lexer = new Lexer("5 3 -");
 * List&lt;Token&gt; tokens = lexer.tokenize();
 * // tokens = [Token(NUMBER, '5', 1:1), Token(NUMBER, '3', 1:3), Token(MINUS, '-', 1:5), Token(EOF, '', 1:6)]
 * </pre>
 *
 * @see Token
 * @see TokenType
 * @see RpnException
 */
public final class Lexer {
    /** The input text to tokenize. */
    private final String text;

    /** Current position in text (0-based index). */
    private int pos;

    /** Current line number (1-based for user-friendly error messages). */
    private int line;

    /** Current column number (1-based for user-friendly error messages). */
    private int column;

    /**
     * Creates a new lexer for the given input text.
     *
     * <p>Initializes position to 0 and line/column to 1.
     *
     * @param text the RPN expression to tokenize
     * @throws NullPointerException if text is null
     */
    public Lexer(String text) {
        this.text = Objects.requireNonNull(text, "text must not be null");
        this.pos = 0;
        this.line = 1;
        this.column = 1;
    }

    /**
     * Tokenizes the entire input text.
     *
     * <p>Scans the input from beginning to end, producing a list of tokens.
     * The list always ends with an EOF token at the final position.
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

        // Add EOF token at final position
        tokens.add(new Token(TokenType.EOF, "", line, column));
        return tokens;
    }

    /**
     * Checks if we've reached the end of input.
     *
     * @return true if position is at or beyond text length
     */
    private boolean atEnd() {
        return pos >= text.length();
    }

    /**
     * Peeks at the current character without consuming it.
     *
     * @return current character, or null character (\0) if at end
     */
    private char peek() {
        return atEnd() ? '\0' : text.charAt(pos);
    }

    /**
     * Consumes and returns the current character, advancing position.
     *
     * <p>Updates line and column tracking:
     * <ul>
     *   <li>Newline (\n): increments line, resets column to 1</li>
     *   <li>Other characters: increments column</li>
     * </ul>
     *
     * @return the character that was consumed
     */
    private char advance() {
        char c = text.charAt(pos++);
        if (c == '\n') {
            line++;
            column = 1;
        } else {
            column++;
        }
        return c;
    }

    /**
     * Skips over whitespace characters.
     *
     * <p>Recognizes space, tab, newline, and carriage return as whitespace.
     * Uses Character.isWhitespace() for comprehensive whitespace detection.
     */
    private void skipWhitespace() {
        while (!atEnd() && Character.isWhitespace(peek())) {
            advance();
        }
    }

    /**
     * Scans and returns the next token from the input.
     *
     * <p>Recognizes:
     * <ul>
     *   <li>Numbers: integers and decimals (including negative numbers)</li>
     *   <li>Addition operator: +</li>
     *   <li>Subtraction operator: -</li>
     *   <li>Multiplication operator: *</li>
     *   <li>Division operator: /</li>
     * </ul>
     *
     * @return the next token
     * @throws RpnException if an invalid character is encountered
     */
    private Token scanToken() throws RpnException {
        int startLine = line;
        int startColumn = column;
        char c = peek();

        if (c == '+') {
            advance();
            return new Token(TokenType.PLUS, "+", startLine, startColumn);
        }

        if (c == '-') {
            // Check if this is a negative number or subtraction operator
            advance();
            if (!atEnd() && Character.isDigit(peek())) {
                // It's a negative number
                return scanNumber("-", startLine, startColumn);
            }
            // It's a subtraction operator
            return new Token(TokenType.MINUS, "-", startLine, startColumn);
        }

        if (c == '*') {
            advance();
            return new Token(TokenType.STAR, "*", startLine, startColumn);
        }

        if (c == '/') {
            advance();
            return new Token(TokenType.SLASH, "/", startLine, startColumn);
        }

        if (Character.isDigit(c)) {
            return scanNumber("", startLine, startColumn);
        }

        // Unknown character - throw error
        throw new RpnException("Unexpected character '" + c + "'", startLine, startColumn);
    }

    /**
     * Scans a numeric literal (integer or decimal).
     *
     * <p>Supports:
     * <ul>
     *   <li>Integers: 5, 42, 100</li>
     *   <li>Decimals: 3.14, 1.5, 0.5</li>
     *   <li>Negative numbers: -5, -3.14 (when prefix is "-")</li>
     * </ul>
     *
     * @param prefix any prefix already consumed (e.g., "-" for negative numbers)
     * @param startLine line number where the number started
     * @param startColumn column number where the number started
     * @return a NUMBER token with the complete numeric value
     */
    private Token scanNumber(String prefix, int startLine, int startColumn) {
        StringBuilder value = new StringBuilder(prefix);

        // Integer part
        while (!atEnd() && Character.isDigit(peek())) {
            value.append(advance());
        }

        // Decimal part (optional)
        if (!atEnd() && peek() == '.') {
            value.append(advance()); // consume '.'
            while (!atEnd() && Character.isDigit(peek())) {
                value.append(advance());
            }
        }

        return new Token(TokenType.NUMBER, value.toString(), startLine, startColumn);
    }
}
