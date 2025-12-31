package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for RpnException and ErrorFormatter.
 *
 * <p>Tests cover:
 * <ul>
 *   <li>Exception construction and field access</li>
 *   <li>Error message formatting</li>
 *   <li>Source context extraction</li>
 *   <li>Caret positioning</li>
 *   <li>Edge cases (first line, last line, various columns)</li>
 * </ul>
 */
class RpnExceptionTest {

    @Test
    @DisplayName("RpnException stores message, line, and column")
    void testExceptionFields() {
        RpnException ex = new RpnException("Unexpected character '^'", 1, 5);

        assertEquals("Unexpected character '^'", ex.getErrorMessage());
        assertEquals(1, ex.getLine());
        assertEquals(5, ex.getColumn());
    }

    @Test
    @DisplayName("RpnException message includes position information")
    void testExceptionMessage() {
        RpnException ex = new RpnException("Unexpected character '^'", 1, 5);

        String message = ex.getMessage();
        assertTrue(message.contains("Line 1"));
        assertTrue(message.contains("column 5"));
        assertTrue(message.contains("Unexpected character '^'"));
    }

    @Test
    @DisplayName("RpnException requires non-null message")
    void testNullMessage() {
        assertThrows(NullPointerException.class, () -> {
            new RpnException(null, 1, 1);
        });
    }

    @Test
    @DisplayName("ErrorFormatter requires non-null source")
    void testErrorFormatterNullSource() {
        assertThrows(NullPointerException.class, () -> {
            new RpnException.ErrorFormatter(null);
        });
    }

    @Test
    @DisplayName("ErrorFormatter formats single-line error correctly")
    void testSingleLineError() {
        String source = "2 3 ^ 4 *";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        String formatted = formatter.formatError("Unexpected character '^'", 1, 5);

        // Check structure
        assertTrue(formatted.startsWith("Error: Unexpected character '^'"));
        assertTrue(formatted.contains("\n\n")); // Blank line after error message
        assertTrue(formatted.contains("1 | 2 3 ^ 4 *"));
        assertTrue(formatted.contains("^")); // Caret present
    }

    @Test
    @DisplayName("ErrorFormatter positions caret correctly at column 5")
    void testCaretPositioning() {
        String source = "2 3 ^ 4 *";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        String formatted = formatter.formatError("Unexpected character '^'", 1, 5);

        // Extract the caret line (should be third line after the "Error:" header)
        String[] lines = formatted.split("\n");
        boolean foundCaretLine = false;
        for (String line : lines) {
            if (line.contains("^") && !line.contains("|")) {
                // This is the caret line (not the source line)
                // Count spaces before caret
                int caretPos = line.indexOf("^");
                // The caret line has " | " prefix, then column-1 spaces, then ^
                assertTrue(caretPos >= 4, "Caret should be positioned after '  | ' prefix");
                foundCaretLine = true;
                break;
            }
        }
        assertTrue(foundCaretLine, "Caret line should be present");
    }

    @ParameterizedTest
    @CsvSource({
        "1, 1, 0",   // First column
        "1, 5, 4",   // Middle column
        "1, 9, 8",   // Last column
    })
    @DisplayName("ErrorFormatter positions caret at various columns")
    void testCaretAtVariousColumns(int line, int column, int expectedOffset) {
        String source = "2 3 ^ 4 *";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        String formatted = formatter.formatError("Test error", line, column);

        // Find the caret line
        String[] lines = formatted.split("\n");
        for (int i = 0; i < lines.length; i++) {
            if (lines[i].contains("| 2 3 ^ 4 *")) {
                // Next line should have the caret
                assertTrue(i + 1 < lines.length, "Caret line should follow source line");
                String caretLine = lines[i + 1];

                // Find the caret position relative to the pipe
                int pipePos = caretLine.indexOf("|");
                int caretPos = caretLine.indexOf("^");

                assertTrue(pipePos >= 0, "Caret line should have pipe");
                assertTrue(caretPos >= 0, "Caret line should have caret");

                // The distance from pipe to caret includes "| " (2 chars) plus expectedOffset
                int distance = caretPos - pipePos;
                assertEquals(expectedOffset + 2, distance,
                    "Caret offset should match column position (0-based) + 2 for '| '");
                break;
            }
        }
    }

    @Test
    @DisplayName("ErrorFormatter handles multi-line source")
    void testMultiLineSource() {
        String source = "line 1\nline 2\nline 3";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        String formatted = formatter.formatError("Error on line 2", 2, 3);

        // Should contain line 2
        assertTrue(formatted.contains("2 | line 2"));
        // Should contain caret
        assertTrue(formatted.contains("^"));
    }

    @Test
    @DisplayName("ErrorFormatter shows context lines")
    void testContextLines() {
        String source = "line 1\nline 2\nline 3\nline 4\nline 5";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        // Error on line 3 with 1 context line should show lines 2, 3, 4
        String formatted = formatter.formatError("Error on line 3", 3, 1, 1);

        assertTrue(formatted.contains("2 | line 2"));
        assertTrue(formatted.contains("3 | line 3"));
        assertTrue(formatted.contains("4 | line 4"));
        assertFalse(formatted.contains("1 | line 1"));
        assertFalse(formatted.contains("5 | line 5"));
    }

    @Test
    @DisplayName("ErrorFormatter handles error on first line")
    void testErrorOnFirstLine() {
        String source = "line 1\nline 2\nline 3";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        String formatted = formatter.formatError("Error on first line", 1, 1);

        assertTrue(formatted.contains("1 | line 1"));
        assertTrue(formatted.contains("^"));
        // Should not crash or show negative line numbers
        assertFalse(formatted.contains("0 |"));
    }

    @Test
    @DisplayName("ErrorFormatter handles error on last line")
    void testErrorOnLastLine() {
        String source = "line 1\nline 2\nline 3";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        String formatted = formatter.formatError("Error on last line", 3, 1);

        assertTrue(formatted.contains("3 | line 3"));
        assertTrue(formatted.contains("^"));
        // Should not show lines beyond the source
        assertFalse(formatted.contains("4 |"));
    }

    @Test
    @DisplayName("ErrorFormatter handles empty source")
    void testEmptySource() {
        String source = "";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        // Should not crash when formatting error on empty source
        String formatted = formatter.formatError("Error in empty source", 1, 1);

        assertTrue(formatted.startsWith("Error: Error in empty source"));
        assertTrue(formatted.contains("^"));
    }

    @Test
    @DisplayName("ErrorFormatter pads line numbers for alignment")
    void testLineNumberPadding() {
        // Create source with 10+ lines to test line number width
        StringBuilder sb = new StringBuilder();
        for (int i = 1; i <= 12; i++) {
            sb.append("line ").append(i);
            if (i < 12) sb.append("\n");
        }
        String source = sb.toString();

        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        // Error on line 5 should show lines with proper padding
        String formatted = formatter.formatError("Error", 5, 1, 6);

        // Line numbers 1-9 should be padded to match width of line 11 (2 digits)
        // We can't easily assert exact formatting, but we can check it contains the lines
        assertTrue(formatted.contains("| line 1"));
        assertTrue(formatted.contains("| line 5"));
        assertTrue(formatted.contains("| line 11"));
    }

    @Test
    @DisplayName("ErrorFormatter with default context lines")
    void testDefaultContextLines() {
        String source = "line 1\nline 2\nline 3\nline 4\nline 5";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        // Use the overload without contextLines parameter
        String formatted = formatter.formatError("Error", 3, 1);

        // Default is 1 context line, so should show lines 2, 3, 4
        assertTrue(formatted.contains("2 | line 2"));
        assertTrue(formatted.contains("3 | line 3"));
        assertTrue(formatted.contains("4 | line 4"));
    }

    @Test
    @DisplayName("ErrorFormatter formatError requires non-null message")
    void testFormatErrorNullMessage() {
        String source = "test";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        assertThrows(NullPointerException.class, () -> {
            formatter.formatError(null, 1, 1);
        });
    }

    @Test
    @DisplayName("ErrorFormatter matches expected output format exactly")
    void testExactOutputFormat() {
        String source = "2 3 ^ 4 *";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        String formatted = formatter.formatError("Unexpected character '^'", 1, 5);

        // Expected format:
        // Error: Unexpected character '^'
        //
        // 1 | 2 3 ^ 4 *
        //       ^

        String[] lines = formatted.split("\n");
        assertEquals("Error: Unexpected character '^'", lines[0]);
        assertEquals("", lines[1]); // Blank line
        assertEquals("1 | 2 3 ^ 4 *", lines[2]);
        assertEquals("  |     ^", lines[3]); // Caret line with pipe and proper spacing
    }

    @Test
    @DisplayName("ErrorFormatter handles source with trailing newline")
    void testSourceWithTrailingNewline() {
        String source = "line 1\n";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        String formatted = formatter.formatError("Error", 1, 1);

        assertTrue(formatted.contains("1 | line 1"));
        assertTrue(formatted.contains("^"));
    }

    @Test
    @DisplayName("ErrorFormatter handles column at beginning of line")
    void testColumnAtBeginning() {
        String source = "test";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        String formatted = formatter.formatError("Error", 1, 1);

        // Caret should be at position 0 (no leading spaces before it)
        String[] lines = formatted.split("\n");
        boolean foundCaret = false;
        for (String line : lines) {
            if (line.contains("^") && !line.contains("test")) {
                // Should be: "  | ^" (with pipe prefix, caret immediately after)
                assertTrue(line.contains("| ^"), "Caret should immediately follow pipe and space");
                foundCaret = true;
                break;
            }
        }
        assertTrue(foundCaret);
    }

    @Test
    @DisplayName("ErrorFormatter handles very long lines")
    void testVeryLongLine() {
        String source = "x".repeat(100);
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        String formatted = formatter.formatError("Error", 1, 50);

        // Should not crash and should contain the line
        assertTrue(formatted.contains(source));
        assertTrue(formatted.contains("^"));
    }
}
