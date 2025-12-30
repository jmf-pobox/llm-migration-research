package com.rpn2tex;

import org.junit.jupiter.api.Test;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the Lexer class.
 */
public class LexerTest {
    @Test
    public void testTokenizeSingleNumber() throws RpnException {
        Lexer lexer = new Lexer("5");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("5", tokens.get(0).value);
        assertEquals(1, tokens.get(0).line);
        assertEquals(1, tokens.get(0).column);
        assertEquals(TokenType.EOF, tokens.get(1).type);
    }

    @Test
    public void testTokenizeDecimal() throws RpnException {
        Lexer lexer = new Lexer("3.14");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("3.14", tokens.get(0).value);
    }

    @Test
    public void testTokenizeNegativeNumber() throws RpnException {
        Lexer lexer = new Lexer("-42");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("-42", tokens.get(0).value);
    }

    @Test
    public void testTokenizeMultipleNumbers() throws RpnException {
        Lexer lexer = new Lexer("5 3");
        List<Token> tokens = lexer.tokenize();

        assertEquals(3, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("5", tokens.get(0).value);
        assertEquals(TokenType.NUMBER, tokens.get(1).type);
        assertEquals("3", tokens.get(1).value);
        assertEquals(TokenType.EOF, tokens.get(2).type);
    }

    @Test
    public void testInvalidCharacter() {
        Lexer lexer = new Lexer("5 # 3");
        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);
        assertTrue(exception.message.contains("Unexpected character"));
        assertEquals(1, exception.line);
        assertEquals(3, exception.column);
    }

    @Test
    public void testWhitespaceHandling() throws RpnException {
        Lexer lexer = new Lexer("  5  \n  3  ");
        List<Token> tokens = lexer.tokenize();

        assertEquals(3, tokens.size());
        assertEquals("5", tokens.get(0).value);
        assertEquals(1, tokens.get(0).line);
        assertEquals(3, tokens.get(0).column);
        assertEquals("3", tokens.get(1).value);
        assertEquals(2, tokens.get(1).line);
        assertEquals(3, tokens.get(1).column);
    }
}
