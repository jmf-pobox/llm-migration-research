package com.rpn2tex;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests to verify ErrorFormatter produces I/O contract-compliant output.
 */
class ErrorFormatterIntegrationTest {

    @Test
    void testIOContract_ErrorCase5_Format() {
        // I/O Contract Case 5: "2 3 ^"
        // Expected: Error: Unexpected character '^'
        // Location: Line 1, Column 5
        String source = "2 3 ^";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Unexpected character '^'", 1, 5);

        // Verify the format matches compiler-style output
        assertTrue(result.startsWith("Error: Unexpected character '^'"));
        assertTrue(result.contains("\n\n")); // Blank line after error message
        assertTrue(result.contains("1 | 2 3 ^"));
        assertTrue(result.contains("  |     ^")); // Caret at column 5

        System.out.println("Test Case 5 - Output:");
        System.out.println(result);
    }

    @Test
    void testIOContract_ErrorCase17_Format() {
        // I/O Contract Case 17: "2 3 4 ^ ^"
        // Expected: Error: Unexpected character '^'
        // Location: Line 1, Column 7
        String source = "2 3 4 ^ ^";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Unexpected character '^'", 1, 7);

        assertTrue(result.startsWith("Error: Unexpected character '^'"));
        assertTrue(result.contains("1 | 2 3 4 ^ ^"));
        assertTrue(result.contains("  |       ^")); // Caret at column 7

        System.out.println("\nTest Case 17 - Output:");
        System.out.println(result);
    }

    @Test
    void testExceptionWithFormatter() {
        // Test using RpnException with ErrorFormatter
        String source = "2 3 ^ 4";
        RpnException exception = new RpnException("Unexpected character '^'", 1, 5);
        ErrorFormatter formatter = new ErrorFormatter(source);

        String formattedError = formatter.formatError(
            exception.getMessage(),
            exception.getLine(),
            exception.getColumn()
        );

        assertTrue(formattedError.contains("Error: Unexpected character '^'"));
        assertTrue(formattedError.contains("1 | 2 3 ^ 4"));
        assertTrue(formattedError.contains("^"));
    }

    @Test
    void testMultilineSourceError() {
        String source = "line 1\nline 2 error\nline 3";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Parse error", 2, 8);

        // Should show line 2 with error
        assertTrue(result.contains("2 | line 2 error"));
        // Should have caret at column 8
        String[] lines = result.split("\n");
        boolean foundCaretLine = false;
        for (int i = 0; i < lines.length; i++) {
            if (lines[i].contains("line 2 error")) {
                if (i + 1 < lines.length) {
                    String caretLine = lines[i + 1];
                    assertTrue(caretLine.contains("^"));
                    foundCaretLine = true;
                }
            }
        }
        assertTrue(foundCaretLine);

        System.out.println("\nMultiline Source Test - Output:");
        System.out.println(result);
    }

    @Test
    void testCaretPositioning() {
        // Test various column positions
        String source = "0123456789";
        ErrorFormatter formatter = new ErrorFormatter(source);

        // Test column 1 (first character)
        String result1 = formatter.formatError("Error at position 1", 1, 1);
        assertTrue(result1.contains("1 | 0123456789"));
        assertTrue(result1.contains("  | ^")); // Caret at start

        // Test column 5 (middle)
        String result5 = formatter.formatError("Error at position 5", 1, 5);
        assertTrue(result5.contains("  |     ^")); // Caret at column 5

        // Test column 10 (end)
        String result10 = formatter.formatError("Error at position 10", 1, 10);
        assertTrue(result10.contains("  |          ^")); // Caret at column 10
    }

    @Test
    void testErrorMessagePreservation() {
        String source = "test";
        ErrorFormatter formatter = new ErrorFormatter(source);

        // Test various error messages
        String[] messages = {
            "Unexpected character '^'",
            "Not enough operands for operator",
            "Empty expression",
            "Multiple values remain on stack"
        };

        for (String message : messages) {
            String result = formatter.formatError(message, 1, 1);
            assertTrue(result.startsWith("Error: " + message),
                "Error message should be preserved: " + message);
        }
    }
}
