package com.rpn2tex;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for RpnException class.
 */
class RpnExceptionTest {

    @Test
    void testConstructorWithValidValues() {
        RpnException exception = new RpnException("Test error", 5, 10);
        assertEquals("Test error", exception.getMessage());
        assertEquals(5, exception.getLine());
        assertEquals(10, exception.getColumn());
    }

    @Test
    void testConstructorWithMinimumValidValues() {
        RpnException exception = new RpnException("Error", 1, 1);
        assertEquals(1, exception.getLine());
        assertEquals(1, exception.getColumn());
    }

    @Test
    void testConstructorWithInvalidLine() {
        assertThrows(IllegalArgumentException.class, () -> {
            new RpnException("Error", 0, 1);
        });
    }

    @Test
    void testConstructorWithNegativeLine() {
        assertThrows(IllegalArgumentException.class, () -> {
            new RpnException("Error", -1, 1);
        });
    }

    @Test
    void testConstructorWithInvalidColumn() {
        assertThrows(IllegalArgumentException.class, () -> {
            new RpnException("Error", 1, 0);
        });
    }

    @Test
    void testConstructorWithNegativeColumn() {
        assertThrows(IllegalArgumentException.class, () -> {
            new RpnException("Error", 1, -1);
        });
    }

    @Test
    void testToStringContainsAllInformation() {
        RpnException exception = new RpnException("Unexpected character", 3, 7);
        String result = exception.toString();
        assertTrue(result.contains("RpnException"));
        assertTrue(result.contains("Unexpected character"));
        assertTrue(result.contains("3"));
        assertTrue(result.contains("7"));
    }

    @Test
    void testToStringFormat() {
        RpnException exception = new RpnException("Test message", 1, 5);
        String result = exception.toString();
        assertEquals("RpnException: Test message at line 1, column 5", result);
    }

    @Test
    void testExceptionIsThrowable() {
        RpnException exception = new RpnException("Error", 1, 1);
        assertThrows(RpnException.class, () -> {
            throw exception;
        });
    }

    @Test
    void testExceptionMessage() {
        String message = "Unexpected character: ^";
        RpnException exception = new RpnException(message, 1, 5);
        assertEquals(message, exception.getMessage());
    }

    @Test
    void testLargePositionValues() {
        RpnException exception = new RpnException("Error", 999, 888);
        assertEquals(999, exception.getLine());
        assertEquals(888, exception.getColumn());
    }

    @Test
    void testEmptyMessage() {
        RpnException exception = new RpnException("", 1, 1);
        assertEquals("", exception.getMessage());
    }

    @Test
    void testNullMessage() {
        RpnException exception = new RpnException(null, 1, 1);
        assertNull(exception.getMessage());
    }
}
