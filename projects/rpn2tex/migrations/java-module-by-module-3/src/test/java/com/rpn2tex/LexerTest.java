package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Test suite for the Lexer class.
 *
 * <p>This test suite validates:
 * <ul>
 *   <li>Basic token recognition (numbers, operators, EOF)</li>
 *   <li>Decimal number parsing</li>
 *   <li>Negative number handling</li>
 *   <li>Whitespace skipping</li>
 *   <li>Position tracking (1-based line and column)</li>
 *   <li>Error handling for invalid characters</li>
 *   <li>Edge cases (empty input, only whitespace)</li>
 * </ul>
 */
class LexerTest {

    @Test
    void testEmptyInput() throws RpnException {
        Lexer lexer = new Lexer("");
        List<Token> tokens = lexer.tokenize();

        assertEquals(1, tokens.size());
        assertEquals(TokenType.EOF, tokens.get(0).type());
        assertEquals("", tokens.get(0).value());
        assertEquals(1, tokens.get(0).line());
        assertEquals(1, tokens.get(0).column());
    }

    @Test
    void testSingleNumber() throws RpnException {
        Lexer lexer = new Lexer("5");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "5", 1, 1), tokens.get(0));
        assertEquals(new Token(TokenType.EOF, "", 1, 2), tokens.get(1));
    }

    @Test
    void testBasicAddition() throws RpnException {
        Lexer lexer = new Lexer("5 3 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "5", 1, 1), tokens.get(0));
        assertEquals(new Token(TokenType.NUMBER, "3", 1, 3), tokens.get(1));
        assertEquals(new Token(TokenType.PLUS, "+", 1, 5), tokens.get(2));
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    @Test
    void testAllOperators() throws RpnException {
        Lexer lexer = new Lexer("+ - * /");
        List<Token> tokens = lexer.tokenize();

        assertEquals(5, tokens.size());
        assertEquals(TokenType.PLUS, tokens.get(0).type());
        assertEquals(TokenType.MINUS, tokens.get(1).type());
        assertEquals(TokenType.MULT, tokens.get(2).type());
        assertEquals(TokenType.DIV, tokens.get(3).type());
        assertEquals(TokenType.EOF, tokens.get(4).type());
    }

    @Test
    void testDecimalNumbers() throws RpnException {
        Lexer lexer = new Lexer("3.14 2 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "3.14", 1, 1), tokens.get(0));
        assertEquals(new Token(TokenType.NUMBER, "2", 1, 6), tokens.get(1));
        assertEquals(new Token(TokenType.MULT, "*", 1, 8), tokens.get(2));
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    @Test
    void testDecimalOnlyFraction() {
        // Numbers starting with a decimal point like ".5" are not supported
        // The lexer will throw an error for the unexpected '.' character
        Lexer lexer = new Lexer(".5");
        RpnException exception = assertThrows(RpnException.class, () -> lexer.tokenize());

        assertEquals("Unexpected character '.'", exception.getErrorMessage());
        assertEquals(1, exception.getLine());
        assertEquals(1, exception.getColumn());
    }

    @Test
    void testNegativeNumber() throws RpnException {
        Lexer lexer = new Lexer("5 -3 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "5", 1, 1), tokens.get(0));
        assertEquals(new Token(TokenType.NUMBER, "-3", 1, 3), tokens.get(1));
        assertEquals(new Token(TokenType.PLUS, "+", 1, 6), tokens.get(2));
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    @Test
    void testMinusOperatorVsNegativeNumber() throws RpnException {
        // "5 - 3" should parse as three separate tokens
        Lexer lexer = new Lexer("5 - 3");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "5", 1, 1), tokens.get(0));
        assertEquals(new Token(TokenType.MINUS, "-", 1, 3), tokens.get(1));
        assertEquals(new Token(TokenType.NUMBER, "3", 1, 5), tokens.get(2));
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    @Test
    void testMultipleWhitespaces() throws RpnException {
        Lexer lexer = new Lexer("5    3  +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "5", 1, 1), tokens.get(0));
        assertEquals(new Token(TokenType.NUMBER, "3", 1, 6), tokens.get(1));
        assertEquals(new Token(TokenType.PLUS, "+", 1, 9), tokens.get(2));
    }

    @Test
    void testLeadingAndTrailingWhitespace() throws RpnException {
        Lexer lexer = new Lexer("  5 3 +  ");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "5", 1, 3), tokens.get(0));
        assertEquals(new Token(TokenType.NUMBER, "3", 1, 5), tokens.get(1));
        assertEquals(new Token(TokenType.PLUS, "+", 1, 7), tokens.get(2));
    }

    @Test
    void testOnlyWhitespace() throws RpnException {
        Lexer lexer = new Lexer("   \t\n  ");
        List<Token> tokens = lexer.tokenize();

        assertEquals(1, tokens.size());
        assertEquals(TokenType.EOF, tokens.get(0).type());
    }

    @Test
    void testMultilineInput() throws RpnException {
        Lexer lexer = new Lexer("5\n3\n+");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "5", 1, 1), tokens.get(0));
        assertEquals(new Token(TokenType.NUMBER, "3", 2, 1), tokens.get(1));
        assertEquals(new Token(TokenType.PLUS, "+", 3, 1), tokens.get(2));
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    @Test
    void testPositionTrackingAcrossLines() throws RpnException {
        Lexer lexer = new Lexer("10\n20 30\n+");
        List<Token> tokens = lexer.tokenize();

        assertEquals(5, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "10", 1, 1), tokens.get(0));
        assertEquals(new Token(TokenType.NUMBER, "20", 2, 1), tokens.get(1));
        assertEquals(new Token(TokenType.NUMBER, "30", 2, 4), tokens.get(2));
        assertEquals(new Token(TokenType.PLUS, "+", 3, 1), tokens.get(3));
    }

    @Test
    void testComplexExpression() throws RpnException {
        Lexer lexer = new Lexer("5 3 + 2 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(6, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "5", 1, 1), tokens.get(0));
        assertEquals(new Token(TokenType.NUMBER, "3", 1, 3), tokens.get(1));
        assertEquals(new Token(TokenType.PLUS, "+", 1, 5), tokens.get(2));
        assertEquals(new Token(TokenType.NUMBER, "2", 1, 7), tokens.get(3));
        assertEquals(new Token(TokenType.MULT, "*", 1, 9), tokens.get(4));
        assertEquals(TokenType.EOF, tokens.get(5).type());
    }

    @Test
    void testInvalidCharacterCaret() {
        Lexer lexer = new Lexer("2 3 ^");
        RpnException exception = assertThrows(RpnException.class, () -> lexer.tokenize());

        assertEquals("Unexpected character '^'", exception.getErrorMessage());
        assertEquals(1, exception.getLine());
        assertEquals(5, exception.getColumn());
    }

    @Test
    void testInvalidCharacterAtSign() {
        Lexer lexer = new Lexer("5 @");
        RpnException exception = assertThrows(RpnException.class, () -> lexer.tokenize());

        assertEquals("Unexpected character '@'", exception.getErrorMessage());
        assertEquals(1, exception.getLine());
        assertEquals(3, exception.getColumn());
    }

    @Test
    void testInvalidCharacterInMiddle() {
        Lexer lexer = new Lexer("5 3 # 2");
        RpnException exception = assertThrows(RpnException.class, () -> lexer.tokenize());

        assertEquals("Unexpected character '#'", exception.getErrorMessage());
        assertEquals(1, exception.getLine());
        assertEquals(5, exception.getColumn());
    }

    @Test
    void testMultipleOperatorsInSequence() throws RpnException {
        Lexer lexer = new Lexer("++--");
        List<Token> tokens = lexer.tokenize();

        assertEquals(5, tokens.size());
        assertEquals(TokenType.PLUS, tokens.get(0).type());
        assertEquals(TokenType.PLUS, tokens.get(1).type());
        assertEquals(TokenType.MINUS, tokens.get(2).type());
        assertEquals(TokenType.MINUS, tokens.get(3).type());
        assertEquals(TokenType.EOF, tokens.get(4).type());
    }

    @Test
    void testDecimalWithLeadingZero() throws RpnException {
        Lexer lexer = new Lexer("0.5");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "0.5", 1, 1), tokens.get(0));
        assertEquals(TokenType.EOF, tokens.get(1).type());
    }

    @Test
    void testLargeNumbers() throws RpnException {
        Lexer lexer = new Lexer("123456789 987654321");
        List<Token> tokens = lexer.tokenize();

        assertEquals(3, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "123456789", 1, 1), tokens.get(0));
        assertEquals(new Token(TokenType.NUMBER, "987654321", 1, 11), tokens.get(1));
        assertEquals(TokenType.EOF, tokens.get(2).type());
    }

    @Test
    void testTokenListIsUnmodifiable() throws RpnException {
        Lexer lexer = new Lexer("5 3 +");
        List<Token> tokens = lexer.tokenize();

        assertThrows(UnsupportedOperationException.class, () ->
            tokens.add(new Token(TokenType.NUMBER, "0", 1, 1))
        );
    }

    @Test
    void testNullInputThrowsException() {
        assertThrows(NullPointerException.class, () -> new Lexer(null));
    }

    /**
     * Test cases from the I/O contract that should cause lexer errors.
     */
    @ParameterizedTest
    @ValueSource(strings = {"2 3 ^", "2 3 ^ 4 *", "2 3 4 ^ ^"})
    void testIOContractErrorCases(String input) {
        Lexer lexer = new Lexer(input);
        RpnException exception = assertThrows(RpnException.class, () -> lexer.tokenize());

        assertEquals("Unexpected character '^'", exception.getErrorMessage());
        assertTrue(exception.getMessage().contains("Unexpected character '^'"));
    }

    /**
     * Test successful tokenization of I/O contract success cases.
     */
    @ParameterizedTest
    @ValueSource(strings = {
        "5 3 +",
        "5 3 -",
        "4 7 *",
        "10 2 /",
        "5 3 + 2 *",
        "5 3 * 2 +",
        "10 2 / 5 *",
        "5 3 - 2 -",
        "3.14 2 *",
        "1.5 0.5 +"
    })
    void testIOContractSuccessCases(String input) {
        Lexer lexer = new Lexer(input);
        assertDoesNotThrow(() -> {
            List<Token> tokens = lexer.tokenize();
            assertNotNull(tokens);
            assertFalse(tokens.isEmpty());
            assertEquals(TokenType.EOF, tokens.get(tokens.size() - 1).type());
        });
    }

    @Test
    void testTabsAsWhitespace() throws RpnException {
        Lexer lexer = new Lexer("5\t3\t+");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals(TokenType.PLUS, tokens.get(2).type());
    }

    @Test
    void testMixedWhitespace() throws RpnException {
        Lexer lexer = new Lexer("5 \t\n 3  +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "5", 1, 1), tokens.get(0));
        assertEquals(new Token(TokenType.NUMBER, "3", 2, 2), tokens.get(1));
        assertEquals(new Token(TokenType.PLUS, "+", 2, 5), tokens.get(2));
    }

    @Test
    void testNegativeDecimalNumber() throws RpnException {
        Lexer lexer = new Lexer("-3.14");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "-3.14", 1, 1), tokens.get(0));
        assertEquals(TokenType.EOF, tokens.get(1).type());
    }

    @Test
    void testZero() throws RpnException {
        Lexer lexer = new Lexer("0");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(new Token(TokenType.NUMBER, "0", 1, 1), tokens.get(0));
        assertEquals(TokenType.EOF, tokens.get(1).type());
    }

    @Test
    void testEOFPositionAfterNumber() throws RpnException {
        Lexer lexer = new Lexer("123");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        Token eofToken = tokens.get(1);
        assertEquals(TokenType.EOF, eofToken.type());
        assertEquals(1, eofToken.line());
        assertEquals(4, eofToken.column()); // After "123"
    }

    @Test
    void testEOFPositionAfterOperator() throws RpnException {
        Lexer lexer = new Lexer("+");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        Token eofToken = tokens.get(1);
        assertEquals(TokenType.EOF, eofToken.type());
        assertEquals(1, eofToken.line());
        assertEquals(2, eofToken.column()); // After "+"
    }

    @Test
    void testValuePreservation() throws RpnException {
        // Verify that number values are preserved exactly as input
        Lexer lexer = new Lexer("5 3.14 -2 0.5");
        List<Token> tokens = lexer.tokenize();

        assertEquals("5", tokens.get(0).value());
        assertEquals("3.14", tokens.get(1).value());
        assertEquals("-2", tokens.get(2).value());
        assertEquals("0.5", tokens.get(3).value());
    }
}
