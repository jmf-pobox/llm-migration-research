package com.rpn2tex;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for Token class.
 */
class TokenTest {

    @Test
    void testConstructorWithValidValues() {
        Token token = new Token(TokenType.NUMBER, "42", 1, 1);
        assertEquals(TokenType.NUMBER, token.type());
        assertEquals("42", token.value());
        assertEquals(1, token.line());
        assertEquals(1, token.column());
    }

    @Test
    void testConstructorWithNullType() {
        assertThrows(NullPointerException.class, () -> {
            new Token(null, "42", 1, 1);
        });
    }

    @Test
    void testConstructorWithNullValue() {
        assertThrows(NullPointerException.class, () -> {
            new Token(TokenType.NUMBER, null, 1, 1);
        });
    }

    @Test
    void testConstructorWithInvalidLine() {
        assertThrows(IllegalArgumentException.class, () -> {
            new Token(TokenType.NUMBER, "42", 0, 1);
        });
    }

    @Test
    void testConstructorWithInvalidColumn() {
        assertThrows(IllegalArgumentException.class, () -> {
            new Token(TokenType.NUMBER, "42", 1, 0);
        });
    }

    @Test
    void testToString() {
        Token token = new Token(TokenType.PLUS, "+", 1, 5);
        String result = token.toString();
        assertTrue(result.contains("PLUS"));
        assertTrue(result.contains("+"));
        assertTrue(result.contains("1"));
        assertTrue(result.contains("5"));
    }

    @Test
    void testEqualsReflexive() {
        Token token = new Token(TokenType.NUMBER, "42", 1, 1);
        assertEquals(token, token);
    }

    @Test
    void testEqualsSymmetric() {
        Token token1 = new Token(TokenType.NUMBER, "42", 1, 1);
        Token token2 = new Token(TokenType.NUMBER, "42", 1, 1);
        assertEquals(token1, token2);
        assertEquals(token2, token1);
    }

    @Test
    void testNotEqualsWithDifferentType() {
        Token token1 = new Token(TokenType.NUMBER, "42", 1, 1);
        Token token2 = new Token(TokenType.PLUS, "42", 1, 1);
        assertNotEquals(token1, token2);
    }

    @Test
    void testNotEqualsWithDifferentValue() {
        Token token1 = new Token(TokenType.NUMBER, "42", 1, 1);
        Token token2 = new Token(TokenType.NUMBER, "43", 1, 1);
        assertNotEquals(token1, token2);
    }

    @Test
    void testNotEqualsWithDifferentPosition() {
        Token token1 = new Token(TokenType.NUMBER, "42", 1, 1);
        Token token2 = new Token(TokenType.NUMBER, "42", 1, 2);
        assertNotEquals(token1, token2);
    }

    @Test
    void testHashCodeConsistency() {
        Token token1 = new Token(TokenType.NUMBER, "42", 1, 1);
        Token token2 = new Token(TokenType.NUMBER, "42", 1, 1);
        assertEquals(token1.hashCode(), token2.hashCode());
    }

    @Test
    void testAllTokenTypes() {
        assertNotNull(new Token(TokenType.NUMBER, "42", 1, 1));
        assertNotNull(new Token(TokenType.PLUS, "+", 1, 1));
        assertNotNull(new Token(TokenType.MINUS, "-", 1, 1));
        assertNotNull(new Token(TokenType.MULT, "*", 1, 1));
        assertNotNull(new Token(TokenType.DIV, "/", 1, 1));
        assertNotNull(new Token(TokenType.EOF, "", 1, 1));
    }

    @Test
    void testEmptyValue() {
        Token token = new Token(TokenType.EOF, "", 1, 10);
        assertEquals("", token.value());
    }

    @Test
    void testLargePositionValues() {
        Token token = new Token(TokenType.NUMBER, "123", 999, 888);
        assertEquals(999, token.line());
        assertEquals(888, token.column());
    }
}
