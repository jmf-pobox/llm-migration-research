package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the Lexer class.
 */
class LexerTest {
    @Test
    void testEmptyInput() throws LexerException {
        Lexer lexer = new Lexer("");
        List<Token> tokens = lexer.tokenize();
        assertEquals(1, tokens.size());
        assertEquals(TokenType.EOF, tokens.get(0).type());
    }

    @Test
    void testSingleInteger() throws LexerException {
        Lexer lexer = new Lexer("5");
        List<Token> tokens = lexer.tokenize();
        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.EOF, tokens.get(1).type());
    }

    @Test
    void testDecimalNumber() throws LexerException {
        Lexer lexer = new Lexer("3.14");
        List<Token> tokens = lexer.tokenize();
        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("3.14", tokens.get(0).value());
    }

    @Test
    void testMultipleNumbers() throws LexerException {
        Lexer lexer = new Lexer("5 3.14");
        List<Token> tokens = lexer.tokenize();
        assertEquals(3, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("3.14", tokens.get(1).value());
        assertEquals(TokenType.EOF, tokens.get(2).type());
    }

    @Test
    void testNegativeNumber() throws LexerException {
        Lexer lexer = new Lexer("-5");
        List<Token> tokens = lexer.tokenize();
        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("-5", tokens.get(0).value());
    }

    @Test
    void testNegativeDecimal() throws LexerException {
        Lexer lexer = new Lexer("-3.14");
        List<Token> tokens = lexer.tokenize();
        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("-3.14", tokens.get(0).value());
    }

    @ParameterizedTest
    @ValueSource(strings = {"007", "0", "100", "0.5", "5.0", "5."})
    void testVariousNumberFormats(String input) throws LexerException {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals(input, tokens.get(0).value());
    }

    @Test
    void testWhitespaceHandling() throws LexerException {
        Lexer lexer = new Lexer("  5   3.14  ");
        List<Token> tokens = lexer.tokenize();
        assertEquals(3, tokens.size());
        assertEquals("5", tokens.get(0).value());
        assertEquals("3.14", tokens.get(1).value());
    }

    @Test
    void testPositionTracking() throws LexerException {
        Lexer lexer = new Lexer("5 3.14");
        List<Token> tokens = lexer.tokenize();

        // First token "5" at position (1, 1)
        assertEquals(1, tokens.get(0).line());
        assertEquals(1, tokens.get(0).column());

        // Second token "3.14" at position (1, 3)
        assertEquals(1, tokens.get(1).line());
        assertEquals(3, tokens.get(1).column());
    }

    @Test
    void testPlusOperator() throws LexerException {
        Lexer lexer = new Lexer("+");
        List<Token> tokens = lexer.tokenize();
        assertEquals(2, tokens.size());
        assertEquals(TokenType.PLUS, tokens.get(0).type());
        assertEquals("+", tokens.get(0).value());
    }

    @Test
    void testAdditionExpression() throws LexerException {
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
    void testMinusOperator() throws LexerException {
        Lexer lexer = new Lexer("-");
        List<Token> tokens = lexer.tokenize();
        assertEquals(2, tokens.size());
        assertEquals(TokenType.MINUS, tokens.get(0).type());
        assertEquals("-", tokens.get(0).value());
    }

    @Test
    void testSubtractionExpression() throws LexerException {
        Lexer lexer = new Lexer("5 3 -");
        List<Token> tokens = lexer.tokenize();
        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("3", tokens.get(1).value());
        assertEquals(TokenType.MINUS, tokens.get(2).type());
        assertEquals("-", tokens.get(2).value());
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    @Test
    void testMinusFollowedByWhitespaceAndNumber() throws LexerException {
        // "- 5" should tokenize as MINUS operator followed by NUMBER
        Lexer lexer = new Lexer("- 5");
        List<Token> tokens = lexer.tokenize();
        assertEquals(3, tokens.size());
        assertEquals(TokenType.MINUS, tokens.get(0).type());
        assertEquals("-", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("5", tokens.get(1).value());
    }

    @Test
    void testNullInput() throws LexerException {
        Lexer lexer = new Lexer(null);
        List<Token> tokens = lexer.tokenize();
        assertEquals(1, tokens.size());
        assertEquals(TokenType.EOF, tokens.get(0).type());
    }

    @Test
    void testTimesOperator() throws LexerException {
        Lexer lexer = new Lexer("*");
        List<Token> tokens = lexer.tokenize();
        assertEquals(2, tokens.size());
        assertEquals(TokenType.TIMES, tokens.get(0).type());
        assertEquals("*", tokens.get(0).value());
    }

    @Test
    void testMultiplicationExpression() throws LexerException {
        Lexer lexer = new Lexer("4 7 *");
        List<Token> tokens = lexer.tokenize();
        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("4", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("7", tokens.get(1).value());
        assertEquals(TokenType.TIMES, tokens.get(2).type());
        assertEquals("*", tokens.get(2).value());
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    @Test
    void testMixedOperators() throws LexerException {
        Lexer lexer = new Lexer("2 3 4 * +");
        List<Token> tokens = lexer.tokenize();
        assertEquals(6, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("2", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("3", tokens.get(1).value());
        assertEquals(TokenType.NUMBER, tokens.get(2).type());
        assertEquals("4", tokens.get(2).value());
        assertEquals(TokenType.TIMES, tokens.get(3).type());
        assertEquals("*", tokens.get(3).value());
        assertEquals(TokenType.PLUS, tokens.get(4).type());
        assertEquals("+", tokens.get(4).value());
        assertEquals(TokenType.EOF, tokens.get(5).type());
    }
}
