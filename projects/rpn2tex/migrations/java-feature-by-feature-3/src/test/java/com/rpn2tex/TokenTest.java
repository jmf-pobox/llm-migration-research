package com.rpn2tex;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the Token class.
 */
class TokenTest {
    @Test
    void testTokenCreation() {
        Token token = new Token(TokenType.NUMBER, "42", 1, 5);
        assertEquals(TokenType.NUMBER, token.type());
        assertEquals("42", token.value());
        assertEquals(1, token.line());
        assertEquals(5, token.column());
    }

    @Test
    void testTokenEquality() {
        Token token1 = new Token(TokenType.NUMBER, "42", 1, 5);
        Token token2 = new Token(TokenType.NUMBER, "42", 1, 5);
        Token token3 = new Token(TokenType.NUMBER, "43", 1, 5);

        assertEquals(token1, token2);
        assertNotEquals(token1, token3);
    }

    @Test
    void testTokenNullValidation() {
        assertThrows(NullPointerException.class, () ->
                new Token(null, "42", 1, 5));
        assertThrows(NullPointerException.class, () ->
                new Token(TokenType.NUMBER, null, 1, 5));
    }

    @Test
    void testTokenToString() {
        Token token = new Token(TokenType.NUMBER, "42", 1, 5);
        String str = token.toString();
        assertTrue(str.contains("NUMBER"));
        assertTrue(str.contains("42"));
    }
}
