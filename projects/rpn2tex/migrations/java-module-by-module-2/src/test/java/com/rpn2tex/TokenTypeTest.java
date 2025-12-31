package com.rpn2tex;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;

/**
 * Unit tests for the {@link TokenType} enum.
 */
class TokenTypeTest {

    @Test
    void testEnumValues() {
        TokenType[] values = TokenType.values();
        assertEquals(6, values.length, "Should have exactly 6 token types");
    }

    @Test
    void testEnumContainsAllExpectedTypes() {
        assertNotNull(TokenType.valueOf("NUMBER"));
        assertNotNull(TokenType.valueOf("PLUS"));
        assertNotNull(TokenType.valueOf("MINUS"));
        assertNotNull(TokenType.valueOf("MULT"));
        assertNotNull(TokenType.valueOf("DIV"));
        assertNotNull(TokenType.valueOf("EOF"));
    }

    @Test
    void testEnumName() {
        assertEquals("NUMBER", TokenType.NUMBER.name());
        assertEquals("PLUS", TokenType.PLUS.name());
        assertEquals("MINUS", TokenType.MINUS.name());
        assertEquals("MULT", TokenType.MULT.name());
        assertEquals("DIV", TokenType.DIV.name());
        assertEquals("EOF", TokenType.EOF.name());
    }

    @Test
    void testEnumOrdinals() {
        // Verify ordinals match expected order from specification
        assertEquals(0, TokenType.NUMBER.ordinal());
        assertEquals(1, TokenType.PLUS.ordinal());
        assertEquals(2, TokenType.MINUS.ordinal());
        assertEquals(3, TokenType.MULT.ordinal());
        assertEquals(4, TokenType.DIV.ordinal());
        assertEquals(5, TokenType.EOF.ordinal());
    }
}
