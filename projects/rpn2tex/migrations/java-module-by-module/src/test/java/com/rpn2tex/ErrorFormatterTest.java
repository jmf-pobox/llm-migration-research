package com.rpn2tex;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for ErrorFormatter class.
 */
class ErrorFormatterTest {

    @Test
    void testConstructorWithValidSource() {
        String source = "5 3 +";
        ErrorFormatter formatter = new ErrorFormatter(source);
        assertNotNull(formatter);
    }

    @Test
    void testConstructorWithNullSource() {
        assertThrows(NullPointerException.class, () -> {
            new ErrorFormatter(null);
        });
    }

    @Test
    void testConstructorWithEmptySource() {
        ErrorFormatter formatter = new ErrorFormatter("");
        assertNotNull(formatter);
    }

    @Test
    void testFormatErrorBasic() {
        String source = "2 3 ^ 4";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Unexpected character: ^", 1, 5);

        assertTrue(result.contains("Error: Unexpected character: ^"));
        assertTrue(result.contains("1 | 2 3 ^ 4"));
        assertTrue(result.contains("^"));
    }

    @Test
    void testFormatErrorCaretPosition() {
        String source = "2 3 ^ 4";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Unexpected character: ^", 1, 5);

        // The caret should be at position 5 (1-based)
        // Format: "  | 2 3 ^ 4"
        // Format: "  |     ^"
        String[] lines = result.split("\n");
        boolean foundCaret = false;
        for (int i = 0; i < lines.length; i++) {
            if (lines[i].contains("2 3 ^ 4")) {
                // Next line should have the caret
                if (i + 1 < lines.length) {
                    String caretLine = lines[i + 1];
                    assertTrue(caretLine.contains("^"), "Caret line should contain ^");
                    foundCaret = true;
                }
            }
        }
        assertTrue(foundCaret, "Should find caret line");
    }

    @Test
    void testFormatErrorWithContextLines() {
        String source = "line 1\nline 2\nline 3\nline 4\nline 5";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error on line 3", 3, 1, 1);

        assertTrue(result.contains("line 2"));
        assertTrue(result.contains("line 3"));
        assertTrue(result.contains("line 4"));
    }

    @Test
    void testFormatErrorWithZeroContextLines() {
        String source = "line 1\nline 2\nline 3";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error", 2, 1, 0);

        assertTrue(result.contains("line 2"));
        assertFalse(result.contains("line 1"));
        assertFalse(result.contains("line 3"));
    }

    @Test
    void testFormatErrorAtBeginning() {
        String source = "2 3 + 4";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error at start", 1, 1);

        assertTrue(result.contains("Error: Error at start"));
        assertTrue(result.contains("2 3 + 4"));
    }

    @Test
    void testFormatErrorAtEnd() {
        String source = "2 3 + 4";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error at end", 1, 7);

        assertTrue(result.contains("Error: Error at end"));
        assertTrue(result.contains("2 3 + 4"));
    }

    @Test
    void testFormatErrorMultipleLines() {
        String source = "line 1\nline 2 error here\nline 3";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error message", 2, 8);

        assertTrue(result.contains("Error: Error message"));
        assertTrue(result.contains("2 | line 2 error here"));
        assertTrue(result.contains("^"));
    }

    @Test
    void testFormatErrorWithInvalidLine() {
        ErrorFormatter formatter = new ErrorFormatter("test");
        assertThrows(IllegalArgumentException.class, () -> {
            formatter.formatError("Error", 0, 1);
        });
    }

    @Test
    void testFormatErrorWithInvalidColumn() {
        ErrorFormatter formatter = new ErrorFormatter("test");
        assertThrows(IllegalArgumentException.class, () -> {
            formatter.formatError("Error", 1, 0);
        });
    }

    @Test
    void testFormatErrorWithNegativeContextLines() {
        ErrorFormatter formatter = new ErrorFormatter("test");
        assertThrows(IllegalArgumentException.class, () -> {
            formatter.formatError("Error", 1, 1, -1);
        });
    }

    @Test
    void testFormatErrorWithNullMessage() {
        ErrorFormatter formatter = new ErrorFormatter("test");
        assertThrows(NullPointerException.class, () -> {
            formatter.formatError(null, 1, 1);
        });
    }

    @Test
    void testFormatErrorLineNumbers() {
        String source = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error", 10, 1);

        // Line number should be right-aligned
        assertTrue(result.contains("10 | 10"));
    }

    @Test
    void testFormatErrorDefaultContextLines() {
        String source = "line 1\nline 2\nline 3\nline 4\nline 5";
        ErrorFormatter formatter = new ErrorFormatter(source);

        // Default should be 1 context line
        String result1 = formatter.formatError("Error", 3, 1);
        String result2 = formatter.formatError("Error", 3, 1, 1);

        // Both should include line 2, 3, and 4
        assertTrue(result1.contains("line 2"));
        assertTrue(result2.contains("line 2"));
    }

    @Test
    void testFormatErrorBeyondSourceBounds() {
        String source = "single line";
        ErrorFormatter formatter = new ErrorFormatter(source);

        // Requesting line beyond source should still work
        // (implementation should handle gracefully)
        String result = formatter.formatError("Error", 1, 20);
        assertTrue(result.contains("Error:"));
    }

    @Test
    void testFormatErrorEmptyLine() {
        String source = "line 1\n\nline 3";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error on empty line", 2, 1);

        assertTrue(result.contains("Error: Error on empty line"));
        assertTrue(result.contains("2 |"));
    }

    @Test
    void testFormatErrorCompilerStyleOutput() {
        // Test that output matches compiler-style format
        String source = "2 3 ^ 4";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Unexpected character: ^", 1, 5);

        // Should start with "Error: "
        assertTrue(result.startsWith("Error: "));

        // Should have blank line after message
        assertTrue(result.contains("Error: Unexpected character: ^\n\n"));

        // Should have line number with pipe separator
        assertTrue(result.contains("1 | "));

        // Should have caret on separate line
        assertTrue(result.contains("  | "));
    }

    @Test
    void testFormatErrorPreservesSpacing() {
        String source = "  indented line";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error", 1, 3);

        // Should preserve the indentation in the source line
        assertTrue(result.contains("  indented line"));
    }
}
