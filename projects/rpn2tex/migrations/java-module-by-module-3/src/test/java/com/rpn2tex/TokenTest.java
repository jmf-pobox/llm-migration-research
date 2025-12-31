package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the Token record.
 */
class TokenTest {

    @Test
    void testTokenCreation() {
        // Test basic token creation
        Token token = new Token(TokenType.NUMBER, "5", 1, 1);

        assertEquals(TokenType.NUMBER, token.type());
        assertEquals("5", token.value());
        assertEquals(1, token.line());
        assertEquals(1, token.column());
    }

    @Test
    void testTokenEquality() {
        // Two tokens with same fields should be equal
        Token token1 = new Token(TokenType.NUMBER, "5", 1, 1);
        Token token2 = new Token(TokenType.NUMBER, "5", 1, 1);

        assertEquals(token1, token2);
        assertEquals(token1.hashCode(), token2.hashCode());
    }

    @Test
    void testTokenInequality() {
        // Tokens with different fields should not be equal
        Token token1 = new Token(TokenType.NUMBER, "5", 1, 1);
        Token token2 = new Token(TokenType.NUMBER, "3", 1, 1);
        Token token3 = new Token(TokenType.PLUS, "5", 1, 1);
        Token token4 = new Token(TokenType.NUMBER, "5", 2, 1);
        Token token5 = new Token(TokenType.NUMBER, "5", 1, 2);

        assertNotEquals(token1, token2, "Different values should not be equal");
        assertNotEquals(token1, token3, "Different types should not be equal");
        assertNotEquals(token1, token4, "Different lines should not be equal");
        assertNotEquals(token1, token5, "Different columns should not be equal");
    }

    @Test
    void testTokenStringRepresentation() {
        // Test toString format: Token(TYPE, "value", line:column)
        Token numberToken = new Token(TokenType.NUMBER, "5", 1, 1);
        assertEquals("Token(NUMBER, \"5\", 1:1)", numberToken.toString());

        Token plusToken = new Token(TokenType.PLUS, "+", 1, 3);
        assertEquals("Token(PLUS, \"+\", 1:3)", plusToken.toString());

        Token eofToken = new Token(TokenType.EOF, "", 1, 10);
        assertEquals("Token(EOF, \"\", 1:10)", eofToken.toString());
    }

    @ParameterizedTest
    @CsvSource({
        "NUMBER, '5', 1, 1",
        "NUMBER, '3.14', 1, 5",
        "NUMBER, '-2', 2, 1",
        "PLUS, '+', 1, 3",
        "MINUS, '-', 1, 3",
        "MULT, '*', 1, 5",
        "DIV, '/', 1, 7",
        "EOF, '', 1, 10"
    })
    void testVariousTokenTypes(String typeStr, String value, int line, int column) {
        // Test creating tokens of different types
        TokenType type = TokenType.valueOf(typeStr);
        Token token = new Token(type, value, line, column);

        assertEquals(type, token.type());
        assertEquals(value, token.value());
        assertEquals(line, token.line());
        assertEquals(column, token.column());
    }

    @Test
    void testNullTypeThrowsException() {
        // Null type should throw NullPointerException
        assertThrows(NullPointerException.class, () -> {
            new Token(null, "5", 1, 1);
        });
    }

    @Test
    void testNullValueThrowsException() {
        // Null value should throw NullPointerException
        assertThrows(NullPointerException.class, () -> {
            new Token(TokenType.NUMBER, null, 1, 1);
        });
    }

    @Test
    void testTokenWithDecimalValue() {
        // Test token with decimal number
        Token token = new Token(TokenType.NUMBER, "3.14", 1, 1);

        assertEquals(TokenType.NUMBER, token.type());
        assertEquals("3.14", token.value());
    }

    @Test
    void testTokenWithNegativeValue() {
        // Test token with negative number
        Token token = new Token(TokenType.NUMBER, "-5", 1, 1);

        assertEquals(TokenType.NUMBER, token.type());
        assertEquals("-5", token.value());
    }

    @Test
    void testEOFToken() {
        // Test EOF token with empty value
        Token token = new Token(TokenType.EOF, "", 1, 10);

        assertEquals(TokenType.EOF, token.type());
        assertEquals("", token.value());
        assertTrue(token.value().isEmpty());
    }

    @Test
    void testTokenPositionTracking() {
        // Test that position is tracked correctly across different locations
        Token token1 = new Token(TokenType.NUMBER, "5", 1, 1);
        Token token2 = new Token(TokenType.PLUS, "+", 1, 3);
        Token token3 = new Token(TokenType.NUMBER, "3", 2, 1);

        assertEquals(1, token1.line());
        assertEquals(1, token1.column());

        assertEquals(1, token2.line());
        assertEquals(3, token2.column());

        assertEquals(2, token3.line());
        assertEquals(1, token3.column());
    }

    @Test
    void testTokenImmutability() {
        // Verify that Token is immutable (record automatically provides this)
        Token token = new Token(TokenType.NUMBER, "5", 1, 1);

        // Store original values
        TokenType originalType = token.type();
        String originalValue = token.value();
        int originalLine = token.line();
        int originalColumn = token.column();

        // Create a "modified" token - must create new instance
        Token modifiedToken = new Token(TokenType.PLUS, "6", 2, 2);

        // Original token should be unchanged
        assertEquals(originalType, token.type());
        assertEquals(originalValue, token.value());
        assertEquals(originalLine, token.line());
        assertEquals(originalColumn, token.column());
    }

    @Test
    void testTokenRecordComponentAccessors() {
        // Test that record component accessors work correctly
        Token token = new Token(TokenType.MULT, "*", 3, 5);

        // All accessor methods should be available and work correctly
        assertNotNull(token.type());
        assertNotNull(token.value());
        assertTrue(token.line() > 0);
        assertTrue(token.column() > 0);
    }
}
