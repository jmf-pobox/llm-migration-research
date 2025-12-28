package com.rpn2tex;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Tests to verify Token implementation matches the migration specification.
 *
 * <p>From Module 1 spec:
 * - TokenType: 6 variants for NUMBER, PLUS, MINUS, MULT, DIV, EOF
 * - Token: Immutable struct with position tracking
 * - Line/column are 1-based values
 * - Implement toString for debugging
 */
class TokenSpecificationTest {

    @Test
    void testTokenTypeEnum() {
        // Verify all 6 token types exist
        assertEquals(6, TokenType.values().length);
        assertNotNull(TokenType.NUMBER);
        assertNotNull(TokenType.PLUS);
        assertNotNull(TokenType.MINUS);
        assertNotNull(TokenType.MULT);
        assertNotNull(TokenType.DIV);
        assertNotNull(TokenType.EOF);
    }

    @Test
    void testTokenImmutability() {
        Token token = new Token(TokenType.NUMBER, "42", 1, 5);

        // Verify all fields are accessible (getters exist)
        assertEquals(TokenType.NUMBER, token.type());
        assertEquals("42", token.value());
        assertEquals(1, token.line());
        assertEquals(5, token.column());

        // Token should be immutable - verify fields can't be changed
        // (This is enforced by Java's final keyword at compile time)
    }

    @Test
    void testPositionTracking1Based() {
        // Line and column should be 1-based
        Token token = new Token(TokenType.PLUS, "+", 1, 1);
        assertEquals(1, token.line());
        assertEquals(1, token.column());

        // Verify 0 is rejected (not 1-based)
        assertThrows(IllegalArgumentException.class, () -> {
            new Token(TokenType.PLUS, "+", 0, 1);
        });
        assertThrows(IllegalArgumentException.class, () -> {
            new Token(TokenType.PLUS, "+", 1, 0);
        });
    }

    @Test
    void testToStringImplementation() {
        Token token = new Token(TokenType.NUMBER, "3.14", 2, 7);
        String str = token.toString();

        // Verify toString includes all important information
        assertTrue(str.contains("NUMBER"), "toString should include token type");
        assertTrue(str.contains("3.14"), "toString should include value");
        assertTrue(str.contains("2"), "toString should include line");
        assertTrue(str.contains("7"), "toString should include column");

        // Verify format matches Python's repr style: Token(type=..., value='...', line=..., column=...)
        assertTrue(str.startsWith("Token("), "toString should start with 'Token('");
        assertTrue(str.contains("type="), "toString should have 'type=' label");
        assertTrue(str.contains("value="), "toString should have 'value=' label");
        assertTrue(str.contains("line="), "toString should have 'line=' label");
        assertTrue(str.contains("column="), "toString should have 'column=' label");
    }

    @Test
    void testAllOperatorTokens() {
        // Verify all operator tokens can be created
        Token plus = new Token(TokenType.PLUS, "+", 1, 1);
        Token minus = new Token(TokenType.MINUS, "-", 1, 2);
        Token mult = new Token(TokenType.MULT, "*", 1, 3);
        Token div = new Token(TokenType.DIV, "/", 1, 4);
        Token eof = new Token(TokenType.EOF, "", 1, 5);

        assertEquals("+", plus.value());
        assertEquals("-", minus.value());
        assertEquals("*", mult.value());
        assertEquals("/", div.value());
        assertEquals("", eof.value());
    }

    @Test
    void testNumberTokens() {
        // Verify number tokens with various formats
        Token integer = new Token(TokenType.NUMBER, "42", 1, 1);
        Token decimal = new Token(TokenType.NUMBER, "3.14", 1, 5);
        Token negative = new Token(TokenType.NUMBER, "-5", 1, 10);

        assertEquals("42", integer.value());
        assertEquals("3.14", decimal.value());
        assertEquals("-5", negative.value());
    }

    @Test
    void testNullSafety() {
        // Verify null checks as per Java idiom requirements
        assertThrows(NullPointerException.class, () -> {
            new Token(null, "value", 1, 1);
        });

        assertThrows(NullPointerException.class, () -> {
            new Token(TokenType.NUMBER, null, 1, 1);
        });
    }
}
