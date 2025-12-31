package com.rpn2tex;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the TokenType enum.
 */
class TokenTypeTest {

    @Test
    void testAllTokenTypesExist() {
        // Verify all 6 token types exist
        TokenType[] types = TokenType.values();
        assertEquals(6, types.length, "TokenType should have exactly 6 values");
    }

    @Test
    void testTokenTypeNames() {
        // Verify each token type has the expected name
        assertEquals("NUMBER", TokenType.NUMBER.name());
        assertEquals("PLUS", TokenType.PLUS.name());
        assertEquals("MINUS", TokenType.MINUS.name());
        assertEquals("MULT", TokenType.MULT.name());
        assertEquals("DIV", TokenType.DIV.name());
        assertEquals("EOF", TokenType.EOF.name());
    }

    @Test
    void testTokenTypeValueOf() {
        // Verify valueOf works for each token type
        assertEquals(TokenType.NUMBER, TokenType.valueOf("NUMBER"));
        assertEquals(TokenType.PLUS, TokenType.valueOf("PLUS"));
        assertEquals(TokenType.MINUS, TokenType.valueOf("MINUS"));
        assertEquals(TokenType.MULT, TokenType.valueOf("MULT"));
        assertEquals(TokenType.DIV, TokenType.valueOf("DIV"));
        assertEquals(TokenType.EOF, TokenType.valueOf("EOF"));
    }

    @Test
    void testTokenTypeUniqueness() {
        // Verify all token types are unique
        TokenType[] types = TokenType.values();
        for (int i = 0; i < types.length; i++) {
            for (int j = i + 1; j < types.length; j++) {
                assertNotEquals(types[i], types[j],
                    "Token types should be unique: " + types[i] + " vs " + types[j]);
            }
        }
    }
}
