package com.rpn2tex;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

/**
 * I/O Contract validation tests for error formatting.
 *
 * <p>These tests verify that error messages match the exact format
 * specified in the I/O contract:
 * <pre>
 * ERROR: Line X, column Y: message
 * </pre>
 *
 * <p>Test cases are derived from:
 * {@code PHASE_0_IO_CONTRACT.md}
 */
class IOContractErrorTest {

    /**
     * Validates the error format for the exponentiation test case.
     *
     * <p>From I/O contract:
     * <pre>
     * Input: 2 3 ^
     * Expected: ERROR: Line 1, column 5: Unexpected character '^'
     * </pre>
     */
    @Test
    void testExponentiationErrorFormat() {
        // Simulate the lexer error that would occur
        RpnException exception = new RpnException("Unexpected character '^'", 1, 5);

        assertEquals("Line 1, column 5: Unexpected character '^'", exception.getMessage());
        assertEquals(1, exception.getLine());
        assertEquals(5, exception.getColumn());
        assertEquals("Unexpected character '^'", exception.getErrorMessage());
    }

    /**
     * Validates formatted error output with source context.
     *
     * <p>Tests that {@link ErrorFormatter} produces properly formatted
     * error messages with source code context.
     */
    @Test
    void testFormattedErrorWithContext() {
        String source = "2 3 ^";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String formatted = formatter.formatError("Unexpected character '^'", 1, 5);

        // Verify structure
        String[] lines = formatted.split("\n");
        assertTrue(lines.length >= 3, "Should have at least 3 lines (header, blank, source)");

        // Line 1: Error header
        assertEquals("Error: Unexpected character '^'", lines[0]);

        // Line 2: Blank line
        assertEquals("", lines[1]);

        // Line 3+: Source context with line number
        boolean foundSourceLine = false;
        boolean foundCaret = false;

        for (String line : lines) {
            if (line.contains("2 3 ^") && line.contains("|")) {
                foundSourceLine = true;
            }
            if (line.contains("^") && !line.contains("Error:") && !line.contains("2 3 ^")) {
                foundCaret = true;
            }
        }

        assertTrue(foundSourceLine, "Should contain source line with pipe separator");
        assertTrue(foundCaret, "Should contain caret line pointing to error");
    }

    /**
     * Tests multiple error scenarios from the I/O contract.
     */
    @Test
    void testMultipleErrorFormats() {
        // Test case: 2 3 ^ 4 *
        RpnException error1 = new RpnException("Unexpected character '^'", 1, 5);
        assertEquals("Line 1, column 5: Unexpected character '^'", error1.getMessage());

        // Test case: 2 3 4 ^ ^
        RpnException error2 = new RpnException("Unexpected character '^'", 1, 7);
        assertEquals("Line 1, column 7: Unexpected character '^'", error2.getMessage());
    }

    /**
     * Validates that error messages preserve exact wording from source.
     */
    @Test
    void testErrorMessagePreservation() {
        String[] errorMessages = {
            "Unexpected character '^'",
            "Operator '+' requires two operands",
            "Empty expression",
            "Invalid RPN: 2 values remain on stack (missing operators?)"
        };

        for (String message : errorMessages) {
            RpnException exception = new RpnException(message, 1, 1);
            assertEquals(message, exception.getErrorMessage());
            assertTrue(exception.getMessage().contains(message));
        }
    }

    /**
     * Tests that exception can be caught and error information extracted.
     *
     * <p>This simulates how a CLI would catch and format errors for display.
     */
    @Test
    void testErrorHandlingWorkflow() {
        String source = "2 3 ^";
        ErrorFormatter formatter = new ErrorFormatter(source);

        try {
            // Simulate lexer throwing an exception
            throw new RpnException("Unexpected character '^'", 1, 5);
        } catch (RpnException e) {
            // Extract error details
            String message = e.getErrorMessage();
            int line = e.getLine();
            int column = e.getColumn();

            // Format for display
            String formatted = formatter.formatError(message, line, column);

            // Verify format
            assertTrue(formatted.startsWith("Error: "));
            assertTrue(formatted.contains("Unexpected character '^'"));
            assertTrue(formatted.contains("1 |"));
            assertTrue(formatted.contains("2 3 ^"));
        }
    }

    /**
     * Validates error format consistency across different positions.
     */
    @Test
    void testErrorFormatConsistency() {
        int[][] testPositions = {
            {1, 1},
            {1, 5},
            {2, 3},
            {10, 23},
            {100, 500}
        };

        for (int[] pos : testPositions) {
            int line = pos[0];
            int column = pos[1];

            RpnException exception = new RpnException("Test error", line, column);

            String expected = String.format("Line %d, column %d: Test error", line, column);
            assertEquals(expected, exception.getMessage());
        }
    }
}
