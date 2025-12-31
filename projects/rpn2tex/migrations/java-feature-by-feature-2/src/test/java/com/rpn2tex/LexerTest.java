package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class LexerTest {

    @Test
    void testTokenizeSingleInteger() throws RpnException {
        Lexer lexer = new Lexer("5");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.EOF, tokens.get(1).type());
    }

    @Test
    void testTokenizeSingleDecimal() throws RpnException {
        Lexer lexer = new Lexer("3.14");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("3.14", tokens.get(0).value());
        assertEquals(TokenType.EOF, tokens.get(1).type());
    }

    @ParameterizedTest
    @CsvSource({
        "42, 42",
        "0, 0",
        "123, 123",
        "3.14, 3.14",
        "0.5, 0.5",
        "10.0, 10.0"
    })
    void testTokenizeNumbers(String input, String expectedValue) throws RpnException {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals(expectedValue, tokens.get(0).value());
    }

    @Test
    void testTokenizeMultipleNumbers() throws RpnException {
        Lexer lexer = new Lexer("5 3");
        List<Token> tokens = lexer.tokenize();

        assertEquals(3, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("3", tokens.get(1).value());
        assertEquals(TokenType.EOF, tokens.get(2).type());
    }

    @Test
    void testTokenizeWithVariousWhitespace() throws RpnException {
        Lexer lexer = new Lexer("  5   3.14  \n  42  ");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals("5", tokens.get(0).value());
        assertEquals("3.14", tokens.get(1).value());
        assertEquals("42", tokens.get(2).value());
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    @Test
    void testTokenizeEmptyString() throws RpnException {
        Lexer lexer = new Lexer("");
        List<Token> tokens = lexer.tokenize();

        assertEquals(1, tokens.size());
        assertEquals(TokenType.EOF, tokens.get(0).type());
    }

    @Test
    void testTokenizeWhitespaceOnly() throws RpnException {
        Lexer lexer = new Lexer("   \n  \t  ");
        List<Token> tokens = lexer.tokenize();

        assertEquals(1, tokens.size());
        assertEquals(TokenType.EOF, tokens.get(0).type());
    }

    @Test
    void testTokenizePlusOperator() throws RpnException {
        Lexer lexer = new Lexer("5 3 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("3", tokens.get(1).value());
        assertEquals(TokenType.PLUS, tokens.get(2).type());
        assertEquals("+", tokens.get(2).value());
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    @Test
    void testTokenizeMultiplePlusOperators() throws RpnException {
        Lexer lexer = new Lexer("1 2 + 3 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(6, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals(TokenType.PLUS, tokens.get(2).type());
        assertEquals(TokenType.NUMBER, tokens.get(3).type());
        assertEquals(TokenType.PLUS, tokens.get(4).type());
        assertEquals(TokenType.EOF, tokens.get(5).type());
    }

    @Test
    void testTokenizeMultiplyOperator() throws RpnException {
        Lexer lexer = new Lexer("4 7 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("4", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("7", tokens.get(1).value());
        assertEquals(TokenType.MULTIPLY, tokens.get(2).type());
        assertEquals("*", tokens.get(2).value());
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    @Test
    void testInvalidCharacterThrowsException() {
        Lexer lexer = new Lexer("5 @ 3");

        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);
        assertTrue(exception.getMessage().contains("Unexpected character"));
        assertTrue(exception.getMessage().contains("@"));
    }

    @Test
    void testLineAndColumnTracking() throws RpnException {
        Lexer lexer = new Lexer("5\n3.14");
        List<Token> tokens = lexer.tokenize();

        assertEquals(1, tokens.get(0).line());
        assertEquals(1, tokens.get(0).column());
        assertEquals(2, tokens.get(1).line());
        assertEquals(1, tokens.get(1).column());
    }
}
