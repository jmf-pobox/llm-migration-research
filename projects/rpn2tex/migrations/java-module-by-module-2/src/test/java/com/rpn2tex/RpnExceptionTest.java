package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for {@link RpnException}.
 *
 * <p>Tests verify:
 * <ul>
 *   <li>Proper exception message formatting</li>
 *   <li>Error message, line, and column accessors</li>
 *   <li>Input validation (null checks, range checks)</li>
 *   <li>I/O contract compliance for error formatting</li>
 * </ul>
 */
class RpnExceptionTest {

    @Test
    void testBasicExceptionCreation() {
        RpnException exception = new RpnException("Unexpected character '^'", 1, 5);

        assertEquals("Unexpected character '^'", exception.getErrorMessage());
        assertEquals(1, exception.getLine());
        assertEquals(5, exception.getColumn());
        assertEquals("Line 1, column 5: Unexpected character '^'", exception.getMessage());
    }

    @Test
    void testExceptionWithMultilinePosition() {
        RpnException exception = new RpnException("Invalid token", 10, 23);

        assertEquals("Invalid token", exception.getErrorMessage());
        assertEquals(10, exception.getLine());
        assertEquals(23, exception.getColumn());
        assertEquals("Line 10, column 23: Invalid token", exception.getMessage());
    }

    @ParameterizedTest
    @CsvSource({
        "Unexpected character '^', 1, 5, 'Line 1, column 5: Unexpected character ''^'''",
        "Operator '+' requires two operands, 2, 3, 'Line 2, column 3: Operator ''+'' requires two operands'",
        "Empty expression, 1, 1, 'Line 1, column 1: Empty expression'"
    })
    void testErrorMessageFormatting(String message, int line, int column, String expected) {
        RpnException exception = new RpnException(message, line, column);
        assertEquals(expected, exception.getMessage());
    }

    @Test
    void testNullMessageThrowsException() {
        assertThrows(NullPointerException.class, () -> {
            new RpnException(null, 1, 1);
        });
    }

    @Test
    void testInvalidLineNumberThrowsException() {
        assertThrows(IllegalArgumentException.class, () -> {
            new RpnException("Error", 0, 1);
        });

        assertThrows(IllegalArgumentException.class, () -> {
            new RpnException("Error", -1, 1);
        });
    }

    @Test
    void testInvalidColumnNumberThrowsException() {
        assertThrows(IllegalArgumentException.class, () -> {
            new RpnException("Error", 1, 0);
        });

        assertThrows(IllegalArgumentException.class, () -> {
            new RpnException("Error", 1, -1);
        });
    }

    @Test
    void testMinimumValidPosition() {
        RpnException exception = new RpnException("Error at start", 1, 1);
        assertEquals(1, exception.getLine());
        assertEquals(1, exception.getColumn());
    }

    @Test
    void testLargeLineAndColumnNumbers() {
        RpnException exception = new RpnException("Error in large file", 999, 500);
        assertEquals(999, exception.getLine());
        assertEquals(500, exception.getColumn());
        assertEquals("Line 999, column 500: Error in large file", exception.getMessage());
    }

    @Test
    void testExceptionIsCheckedException() {
        // Verify that RpnException is a checked exception (extends Exception)
        assertTrue(Exception.class.isAssignableFrom(RpnException.class));
        assertFalse(RuntimeException.class.isAssignableFrom(RpnException.class));
    }

    @Test
    void testExceptionCanBeCaught() {
        try {
            throw new RpnException("Test error", 1, 1);
        } catch (RpnException e) {
            assertEquals("Test error", e.getErrorMessage());
            // Test passes if exception is caught
        }
    }

    /**
     * Tests that verify I/O contract compliance.
     * Error messages must follow the exact format: "Line X, column Y: message"
     */
    @Test
    void testIOContractErrorFormat() {
        // From I/O contract: ERROR: Line 1, column 5: Unexpected character '^'
        RpnException exception = new RpnException("Unexpected character '^'", 1, 5);
        assertEquals("Line 1, column 5: Unexpected character '^'", exception.getMessage());
    }

    @Test
    void testIOContractFormatWithDifferentMessages() {
        // Test various error messages that might appear in the system
        String[] messages = {
            "Unexpected character '^'",
            "Unexpected token 'xyz'",
            "Operator '+' requires two operands",
            "Invalid RPN: 2 values remain on stack (missing operators?)",
            "Empty expression"
        };

        for (String message : messages) {
            RpnException exception = new RpnException(message, 1, 1);
            assertTrue(exception.getMessage().startsWith("Line 1, column 1: "));
            assertTrue(exception.getMessage().endsWith(message));
        }
    }
}
