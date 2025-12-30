package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the Lexer class.
 */
class LexerTest {

    /**
     * Tests basic tokenization of simple expressions.
     */
    @Test
    void testSimpleTokenization() throws RpnException {
        Lexer lexer = new Lexer("5 3 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("5", tokens.get(0).value);
        assertEquals(TokenType.NUMBER, tokens.get(1).type);
        assertEquals("3", tokens.get(1).value);
        assertEquals(TokenType.PLUS, tokens.get(2).type);
        assertEquals("+", tokens.get(2).value);
        assertEquals(TokenType.EOF, tokens.get(3).type);
    }

    /**
     * Tests tokenization of all operator types.
     */
    @Test
    void testAllOperators() throws RpnException {
        Lexer lexer = new Lexer("+ - * /");
        List<Token> tokens = lexer.tokenize();

        assertEquals(5, tokens.size());
        assertEquals(TokenType.PLUS, tokens.get(0).type);
        assertEquals(TokenType.MINUS, tokens.get(1).type);
        assertEquals(TokenType.MULT, tokens.get(2).type);
        assertEquals(TokenType.DIV, tokens.get(3).type);
        assertEquals(TokenType.EOF, tokens.get(4).type);
    }

    /**
     * Tests tokenization of integer numbers.
     */
    @Test
    void testIntegerNumbers() throws RpnException {
        Lexer lexer = new Lexer("5 42 100");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals("5", tokens.get(0).value);
        assertEquals("42", tokens.get(1).value);
        assertEquals("100", tokens.get(2).value);
        assertEquals(TokenType.EOF, tokens.get(3).type);
    }

    /**
     * Tests tokenization of decimal numbers.
     */
    @Test
    void testDecimalNumbers() throws RpnException {
        Lexer lexer = new Lexer("3.14 1.5 0.5");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals("3.14", tokens.get(0).value);
        assertEquals("1.5", tokens.get(1).value);
        assertEquals("0.5", tokens.get(2).value);
        assertEquals(TokenType.EOF, tokens.get(3).type);
    }

    /**
     * Tests tokenization of negative numbers.
     */
    @Test
    void testNegativeNumbers() throws RpnException {
        Lexer lexer = new Lexer("-5 -3.14");
        List<Token> tokens = lexer.tokenize();

        assertEquals(3, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("-5", tokens.get(0).value);
        assertEquals(TokenType.NUMBER, tokens.get(1).type);
        assertEquals("-3.14", tokens.get(1).value);
        assertEquals(TokenType.EOF, tokens.get(2).type);
    }

    /**
     * Tests that minus is distinguished from negative numbers correctly.
     */
    @Test
    void testMinusOperatorVsNegativeNumber() throws RpnException {
        Lexer lexer = new Lexer("5 - 3");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("5", tokens.get(0).value);
        assertEquals(TokenType.MINUS, tokens.get(1).type);
        assertEquals("-", tokens.get(1).value);
        assertEquals(TokenType.NUMBER, tokens.get(2).type);
        assertEquals("3", tokens.get(2).value);
    }

    /**
     * Tests whitespace handling.
     */
    @Test
    void testWhitespaceHandling() throws RpnException {
        Lexer lexer = new Lexer("  5   3  +  ");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals(TokenType.NUMBER, tokens.get(1).type);
        assertEquals(TokenType.PLUS, tokens.get(2).type);
        assertEquals(TokenType.EOF, tokens.get(3).type);
    }

    /**
     * Tests tokenization of empty string.
     */
    @Test
    void testEmptyInput() throws RpnException {
        Lexer lexer = new Lexer("");
        List<Token> tokens = lexer.tokenize();

        assertEquals(1, tokens.size());
        assertEquals(TokenType.EOF, tokens.get(0).type);
    }

    /**
     * Tests tokenization of whitespace-only input.
     */
    @Test
    void testWhitespaceOnlyInput() throws RpnException {
        Lexer lexer = new Lexer("   \t\n  ");
        List<Token> tokens = lexer.tokenize();

        assertEquals(1, tokens.size());
        assertEquals(TokenType.EOF, tokens.get(0).type);
    }

    /**
     * Tests that invalid characters throw RpnException.
     */
    @Test
    void testInvalidCharacter() {
        Lexer lexer = new Lexer("5 3 ^");
        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);
        assertTrue(exception.getMessage().contains("Unexpected character '^'"));
        assertEquals(1, exception.line);
        assertEquals(5, exception.column);
    }

    /**
     * Tests position tracking for error reporting.
     */
    @Test
    void testPositionTracking() throws RpnException {
        Lexer lexer = new Lexer("5 3 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(1, tokens.get(0).line);
        assertEquals(1, tokens.get(0).column);
        assertEquals(1, tokens.get(1).line);
        assertEquals(3, tokens.get(1).column);
        assertEquals(1, tokens.get(2).line);
        assertEquals(5, tokens.get(2).column);
    }

    /**
     * Tests multi-line position tracking.
     */
    @Test
    void testMultiLinePositionTracking() throws RpnException {
        Lexer lexer = new Lexer("5 3\n+ 2");
        List<Token> tokens = lexer.tokenize();

        assertEquals(1, tokens.get(0).line);
        assertEquals(1, tokens.get(0).column);
        assertEquals(1, tokens.get(1).line);
        assertEquals(3, tokens.get(1).column);
        assertEquals(2, tokens.get(2).line);
        assertEquals(1, tokens.get(2).column);
        assertEquals(2, tokens.get(3).line);
        assertEquals(3, tokens.get(3).column);
    }

    /**
     * Tests null input throws NullPointerException.
     */
    @Test
    void testNullInput() {
        assertThrows(NullPointerException.class, () -> new Lexer(null));
    }

    /**
     * Parameterized tests for I/O contract validation.
     * Tests various valid RPN expressions to ensure proper tokenization.
     */
    @ParameterizedTest
    @CsvSource({
        "'5 3 +', 4",
        "'5 3 -', 4",
        "'4 7 *', 4",
        "'10 2 /', 4",
        "'5 3 + 2 *', 6",
        "'5 3 * 2 +', 6",
        "'10 2 / 5 *', 6",
        "'5 3 - 2 -', 6",
        "'100 10 / 5 / 2 /', 8",
        "'1 2 + 3 + 4 +', 8",
        "'2 3 4 * +', 6",
        "'2 3 + 4 *', 6",
        "'2 3 4 + *', 6",
        "'2 3 * 4 +', 6",
        "'3.14 2 *', 4",
        "'1.5 0.5 +', 4",
        "'1 2 + 3 4 + *', 8",
        "'10 2 / 3 + 4 *', 8"
    })
    void testIOContractValidInputs(String input, int expectedTokenCount) throws RpnException {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

        // Verify token count (including EOF)
        assertEquals(expectedTokenCount, tokens.size());

        // Verify last token is EOF
        assertEquals(TokenType.EOF, tokens.get(tokens.size() - 1).type);
    }

    /**
     * Parameterized tests for error cases from I/O contract.
     */
    @ParameterizedTest
    @CsvSource({
        "'2 3 ^', '^'",
        "'2 3 ^ 4 *', '^'",
        "'2 3 4 ^ ^', '^'"
    })
    void testIOContractErrorCases(String input, String expectedChar) {
        Lexer lexer = new Lexer(input);
        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);
        assertTrue(exception.getMessage().contains("Unexpected character '" + expectedChar + "'"));
    }

    /**
     * Tests that complex expressions tokenize correctly.
     */
    @Test
    void testComplexExpression() throws RpnException {
        Lexer lexer = new Lexer("10 2 / 3 + 4 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(8, tokens.size());
        assertEquals("10", tokens.get(0).value);
        assertEquals("2", tokens.get(1).value);
        assertEquals("/", tokens.get(2).value);
        assertEquals("3", tokens.get(3).value);
        assertEquals("+", tokens.get(4).value);
        assertEquals("4", tokens.get(5).value);
        assertEquals("*", tokens.get(6).value);
        assertEquals(TokenType.EOF, tokens.get(7).type);
    }

    /**
     * Tests that tab and carriage return are handled as whitespace.
     */
    @Test
    void testVariousWhitespace() throws RpnException {
        Lexer lexer = new Lexer("5\t3\r\n+");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals("5", tokens.get(0).value);
        assertEquals("3", tokens.get(1).value);
        assertEquals("+", tokens.get(2).value);
        assertEquals(TokenType.EOF, tokens.get(3).type);
    }
}
