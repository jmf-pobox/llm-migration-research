package com.rpn2tex;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Demonstration tests showing the complete error formatting functionality.
 *
 * <p>These tests illustrate how {@link RpnException} and {@link ErrorFormatter}
 * work together to provide user-friendly error messages with source context.
 */
class ErrorFormattingDemoTest {

    /**
     * Demonstrates the complete error formatting workflow.
     *
     * <p>This test shows how a typical error would be caught and formatted
     * for display to the user.
     */
    @Test
    void demonstrateCompleteErrorWorkflow() {
        // Simulate input with an error
        String source = "2 3 ^";

        // Create formatter
        ErrorFormatter formatter = new ErrorFormatter(source);

        // Simulate an error being thrown (e.g., from lexer)
        try {
            throw new RpnException("Unexpected character '^'", 1, 5);
        } catch (RpnException e) {
            // Format the error for display
            String formatted = formatter.formatError(
                e.getErrorMessage(),
                e.getLine(),
                e.getColumn()
            );

            // Verify the output format
            String[] lines = formatted.split("\n");

            // Expected format:
            // Line 1: Error: Unexpected character '^'
            // Line 2: (blank)
            // Line 3: 1 | 2 3 ^
            // Line 4:   | ^

            assertTrue(lines.length >= 4);
            assertEquals("Error: Unexpected character '^'", lines[0]);
            assertEquals("", lines[1]);
            assertTrue(lines[2].contains("1 | 2 3 ^"));
            assertTrue(lines[3].contains("^"));

            // Print to show the actual formatted output (for documentation)
            System.out.println("Formatted Error Output:");
            System.out.println(formatted);
        }
    }

    /**
     * Demonstrates multi-line error context.
     */
    @Test
    void demonstrateMultilineContext() {
        String source = "5 3 +\n2 3 *\n1 2 /\n7 8 ^";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String formatted = formatter.formatError("Unexpected character '^'", 4, 5);

        System.out.println("\nMulti-line Error Context:");
        System.out.println(formatted);

        // Verify it shows context lines
        assertTrue(formatted.contains("3 | 1 2 /"));
        assertTrue(formatted.contains("4 | 7 8 ^"));
    }

    /**
     * Demonstrates various error types that might occur.
     */
    @Test
    void demonstrateVariousErrorTypes() {
        String source = "5 3 + 2";

        System.out.println("\nVarious Error Types:");

        // Lexer error
        RpnException lexerError = new RpnException("Unexpected character '^'", 1, 7);
        System.out.println("Lexer Error: " + lexerError.getMessage());

        // Parser error - insufficient operands
        RpnException parserError1 = new RpnException(
            "Operator '+' requires two operands", 1, 5);
        System.out.println("Parser Error 1: " + parserError1.getMessage());

        // Parser error - extra operands
        RpnException parserError2 = new RpnException(
            "Invalid RPN: 2 values remain on stack (missing operators?)", 1, 8);
        System.out.println("Parser Error 2: " + parserError2.getMessage());

        // Empty expression
        RpnException parserError3 = new RpnException("Empty expression", 1, 1);
        System.out.println("Parser Error 3: " + parserError3.getMessage());

        // All should follow the format
        assertTrue(lexerError.getMessage().startsWith("Line 1, column 7:"));
        assertTrue(parserError1.getMessage().startsWith("Line 1, column 5:"));
        assertTrue(parserError2.getMessage().startsWith("Line 1, column 8:"));
        assertTrue(parserError3.getMessage().startsWith("Line 1, column 1:"));
    }

    /**
     * Demonstrates error formatting at different positions.
     */
    @Test
    void demonstrateCaretPositioning() {
        String source = "abcdefghij";
        ErrorFormatter formatter = new ErrorFormatter(source);

        System.out.println("\nCaret Positioning at Different Columns:");

        for (int col = 1; col <= 10; col += 3) {
            String formatted = formatter.formatError("Error at column " + col, 1, col);
            System.out.println("\nColumn " + col + ":");
            System.out.println(formatted);
        }

        // Test passes if no exceptions thrown
        assertTrue(true);
    }

    /**
     * Demonstrates error handling with proper line number alignment.
     */
    @Test
    void demonstrateLineNumberAlignment() {
        // Create a source with many lines
        StringBuilder source = new StringBuilder();
        for (int i = 1; i <= 12; i++) {
            source.append("line").append(i).append("\n");
        }

        ErrorFormatter formatter = new ErrorFormatter(source.toString());

        // Error on line 10 (requires 2-digit line numbers)
        String formatted = formatter.formatError("Error on line 10", 10, 3, 2);

        System.out.println("\nLine Number Alignment (2 digits):");
        System.out.println(formatted);

        // Verify alignment
        assertTrue(formatted.contains(" 8 |"));  // Single digit padded
        assertTrue(formatted.contains("10 |"));  // Double digit not padded
    }

    /**
     * Verifies I/O contract compliance with actual formatted output.
     */
    @Test
    void verifyIOContractFormat() {
        // From I/O contract: 2 3 ^ → ERROR: Line 1, column 5: Unexpected character '^'
        String source = "2 3 ^";
        ErrorFormatter formatter = new ErrorFormatter(source);

        RpnException exception = new RpnException("Unexpected character '^'", 1, 5);

        // The exception message should match the I/O contract format (minus "ERROR:" prefix)
        assertEquals("Line 1, column 5: Unexpected character '^'", exception.getMessage());

        // The formatted output should provide context
        String formatted = formatter.formatError(
            exception.getErrorMessage(),
            exception.getLine(),
            exception.getColumn()
        );

        // Expected format:
        // Error: Unexpected character '^'
        //
        // 1 | 2 3 ^
        //       ^

        String[] lines = formatted.split("\n");
        assertEquals("Error: Unexpected character '^'", lines[0]);
        assertTrue(lines[2].contains("1 | 2 3 ^"));

        System.out.println("\nI/O Contract Format:");
        System.out.println(formatted);
    }
}
