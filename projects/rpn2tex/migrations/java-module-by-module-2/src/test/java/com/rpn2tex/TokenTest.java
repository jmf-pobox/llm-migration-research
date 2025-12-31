package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;
import org.junit.jupiter.params.provider.ValueSource;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;

/**
 * Unit tests for the {@link Token} record.
 *
 * <p>Tests cover:
 * <ul>
 *   <li>Token creation with valid parameters</li>
 *   <li>Immutability guarantees</li>
 *   <li>String representation formatting</li>
 *   <li>Position information (1-based indexing)</li>
 *   <li>Validation of null and invalid parameters</li>
 * </ul>
 */
class TokenTest {

    @Test
    void testTokenCreation() {
        Token token = new Token(TokenType.NUMBER, "42", 1, 5);

        assertNotNull(token);
        assertEquals(TokenType.NUMBER, token.type());
        assertEquals("42", token.value());
        assertEquals(1, token.line());
        assertEquals(5, token.column());
    }

    @ParameterizedTest
    @CsvSource({
        "NUMBER, '5', 1, 1",
        "NUMBER, '3.14', 1, 5",
        "PLUS, '+', 1, 3",
        "MINUS, '-', 2, 10",
        "MULT, '*', 3, 7",
        "DIV, '/', 1, 9",
        "EOF, '', 5, 20"
    })
    void testTokenCreationWithVariousValues(String typeStr, String value, int line, int column) {
        TokenType type = TokenType.valueOf(typeStr);
        Token token = new Token(type, value, line, column);

        assertEquals(type, token.type());
        assertEquals(value, token.value());
        assertEquals(line, token.line());
        assertEquals(column, token.column());
    }

    @Test
    void testToStringFormat() {
        Token token = new Token(TokenType.NUMBER, "42", 1, 5);
        String result = token.toString();

        // Expected format: Token(TYPE, 'value', line:column)
        assertEquals("Token(NUMBER, '42', 1:5)", result);
    }

    @ParameterizedTest
    @CsvSource({
        "NUMBER, '5', 1, 1, 'Token(NUMBER, ''5'', 1:1)'",
        "PLUS, '+', 2, 3, 'Token(PLUS, ''+'', 2:3)'",
        "EOF, '', 10, 25, 'Token(EOF, '''', 10:25)'"
    })
    void testToStringFormatVariousTokens(String typeStr, String value, int line, int column,
                                          String expected) {
        TokenType type = TokenType.valueOf(typeStr);
        Token token = new Token(type, value, line, column);

        assertEquals(expected, token.toString());
    }

    @Test
    void testNullTypeThrowsException() {
        assertThrows(NullPointerException.class, () -> {
            new Token(null, "42", 1, 5);
        });
    }

    @Test
    void testNullValueThrowsException() {
        assertThrows(NullPointerException.class, () -> {
            new Token(TokenType.NUMBER, null, 1, 5);
        });
    }

    @ParameterizedTest
    @ValueSource(ints = {0, -1, -10, Integer.MIN_VALUE})
    void testInvalidLineNumberThrowsException(int invalidLine) {
        assertThrows(IllegalArgumentException.class, () -> {
            new Token(TokenType.NUMBER, "42", invalidLine, 5);
        });
    }

    @ParameterizedTest
    @ValueSource(ints = {0, -1, -10, Integer.MIN_VALUE})
    void testInvalidColumnNumberThrowsException(int invalidColumn) {
        assertThrows(IllegalArgumentException.class, () -> {
            new Token(TokenType.NUMBER, "42", 1, invalidColumn);
        });
    }

    @Test
    void testMinimumValidPosition() {
        // Line 1, Column 1 is the minimum valid position
        Token token = new Token(TokenType.NUMBER, "42", 1, 1);

        assertEquals(1, token.line());
        assertEquals(1, token.column());
    }

    @Test
    void testLargeLineAndColumnNumbers() {
        // Test with large but valid position numbers
        Token token = new Token(TokenType.NUMBER, "42", 1000, 5000);

        assertEquals(1000, token.line());
        assertEquals(5000, token.column());
    }

    @Test
    void testRecordEquality() {
        // Records automatically implement equals() based on all fields
        Token token1 = new Token(TokenType.NUMBER, "42", 1, 5);
        Token token2 = new Token(TokenType.NUMBER, "42", 1, 5);
        Token token3 = new Token(TokenType.NUMBER, "43", 1, 5);

        assertEquals(token1, token2, "Identical tokens should be equal");
        assertEquals(token1.hashCode(), token2.hashCode(),
                     "Identical tokens should have same hash code");

        // Different value should make tokens unequal
        assertEquals(false, token1.equals(token3),
                     "Tokens with different values should not be equal");
    }

    @Test
    void testRecordImmutability() {
        // Records are immutable - this test verifies the accessor methods work
        Token token = new Token(TokenType.PLUS, "+", 3, 7);

        // Multiple calls should return the same values
        assertEquals(TokenType.PLUS, token.type());
        assertEquals(TokenType.PLUS, token.type());
        assertEquals("+", token.value());
        assertEquals("+", token.value());
        assertEquals(3, token.line());
        assertEquals(3, token.line());
        assertEquals(7, token.column());
        assertEquals(7, token.column());
    }

    @Test
    void testEmptyValueAllowed() {
        // EOF tokens often have empty values
        Token token = new Token(TokenType.EOF, "", 1, 10);

        assertEquals("", token.value());
        assertEquals("Token(EOF, '', 1:10)", token.toString());
    }

    @Test
    void testNumberTokenWithDecimal() {
        Token token = new Token(TokenType.NUMBER, "3.14", 1, 1);

        assertEquals(TokenType.NUMBER, token.type());
        assertEquals("3.14", token.value());
    }

    @Test
    void testNumberTokenWithNegative() {
        Token token = new Token(TokenType.NUMBER, "-42", 1, 1);

        assertEquals(TokenType.NUMBER, token.type());
        assertEquals("-42", token.value());
    }

    @Test
    void testOperatorTokens() {
        // Test all operator types
        Token plusToken = new Token(TokenType.PLUS, "+", 1, 1);
        Token minusToken = new Token(TokenType.MINUS, "-", 1, 2);
        Token multToken = new Token(TokenType.MULT, "*", 1, 3);
        Token divToken = new Token(TokenType.DIV, "/", 1, 4);

        assertEquals("+", plusToken.value());
        assertEquals("-", minusToken.value());
        assertEquals("*", multToken.value());
        assertEquals("/", divToken.value());
    }
}
