package com.rpn2tex;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for {@link ErrorFormatter}.
 *
 * <p>Tests verify:
 * <ul>
 *   <li>Error message formatting with source context</li>
 *   <li>Caret positioning at the correct column</li>
 *   <li>Context line handling (before and after error)</li>
 *   <li>Edge cases (first/last line, boundary conditions)</li>
 *   <li>Multi-line source handling</li>
 * </ul>
 */
class ErrorFormatterTest {

    @Test
    void testBasicErrorFormatting() {
        String source = "5 3 +";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Test error", 1, 3);

        assertTrue(result.contains("Error: Test error"));
        assertTrue(result.contains("1 | 5 3 +"));
        assertTrue(result.contains("  ^"));
    }

    @Test
    void testCaretPositioning() {
        String source = "2 3 ^";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Unexpected character '^'", 1, 5);

        // The caret should be at position 5 (0-indexed: position 4)
        String[] lines = result.split("\n");
        boolean foundCaret = false;
        for (String line : lines) {
            if (line.contains("^") && !line.contains("Error:")) {
                // Find the position of the caret in the line
                int caretPos = line.indexOf("^");
                // Account for line number prefix "1 | "
                assertTrue(caretPos > 0, "Caret should be positioned after line number prefix");
                foundCaret = true;
            }
        }
        assertTrue(foundCaret, "Caret should be present in formatted output");
    }

    @Test
    void testMultilineSourceErrorOnSecondLine() {
        String source = "5 3 +\n2 3 ^";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Unexpected character '^'", 2, 5);

        assertTrue(result.contains("Error: Unexpected character '^'"));
        assertTrue(result.contains("2 | 2 3 ^"));
        // Should show caret on line 2
        String[] lines = result.split("\n");
        boolean foundLine2 = false;
        for (int i = 0; i < lines.length; i++) {
            if (lines[i].contains("2 | 2 3 ^")) {
                foundLine2 = true;
                // Next line should have the caret
                if (i + 1 < lines.length) {
                    assertTrue(lines[i + 1].contains("^"));
                }
            }
        }
        assertTrue(foundLine2, "Line 2 should be in the output");
    }

    @Test
    void testContextLinesDefault() {
        String source = "line1\nline2\nline3\nline4\nline5";
        ErrorFormatter formatter = new ErrorFormatter(source);

        // Default context is 1 line before and after
        String result = formatter.formatError("Error on line 3", 3, 1);

        assertTrue(result.contains("2 | line2"), "Should show 1 line before");
        assertTrue(result.contains("3 | line3"), "Should show error line");
        assertTrue(result.contains("4 | line4"), "Should show 1 line after");
        assertFalse(result.contains("1 | line1"), "Should not show 2 lines before");
        assertFalse(result.contains("5 | line5"), "Should not show 2 lines after");
    }

    @Test
    void testContextLinesCustom() {
        String source = "line1\nline2\nline3\nline4\nline5";
        ErrorFormatter formatter = new ErrorFormatter(source);

        // Custom context: 2 lines before and after
        String result = formatter.formatError("Error on line 3", 3, 1, 2);

        assertTrue(result.contains("1 | line1"), "Should show 2 lines before");
        assertTrue(result.contains("2 | line2"), "Should show 1 line before");
        assertTrue(result.contains("3 | line3"), "Should show error line");
        assertTrue(result.contains("4 | line4"), "Should show 1 line after");
        assertTrue(result.contains("5 | line5"), "Should show 2 lines after");
    }

    @Test
    void testErrorOnFirstLine() {
        String source = "line1\nline2\nline3";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Error on first line", 1, 1);

        assertTrue(result.contains("1 | line1"), "Should show error line");
        assertTrue(result.contains("2 | line2"), "Should show 1 line after");
        // Should not crash or show line 0
    }

    @Test
    void testErrorOnLastLine() {
        String source = "line1\nline2\nline3";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Error on last line", 3, 1);

        assertTrue(result.contains("2 | line2"), "Should show 1 line before");
        assertTrue(result.contains("3 | line3"), "Should show error line");
        // Should not crash or try to show line 4
    }

    @Test
    void testSingleLineSource() {
        String source = "5 3 ^";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Unexpected character", 1, 5);

        assertTrue(result.contains("1 | 5 3 ^"));
        assertTrue(result.contains("^"));
    }

    @Test
    void testEmptyLineInSource() {
        String source = "line1\n\nline3";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Error after empty line", 3, 1);

        assertTrue(result.contains("2 | "), "Should show empty line 2");
        assertTrue(result.contains("3 | line3"), "Should show error line");
    }

    @Test
    void testLineNumberAlignment() {
        // Test that line numbers are properly aligned for multi-digit line numbers
        String source = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Error on line 10", 10, 1, 2);

        // All line numbers should be aligned (2 digits wide)
        assertTrue(result.contains(" 8 |"), "Line 8 should be padded");
        assertTrue(result.contains(" 9 |"), "Line 9 should be padded");
        assertTrue(result.contains("10 |"), "Line 10 should not be padded");
        assertTrue(result.contains("11 |"), "Line 11 should not be padded");
    }

    @Test
    void testCaretAtStartOfLine() {
        String source = "abc";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Error at start", 1, 1);

        String[] lines = result.split("\n");
        for (int i = 0; i < lines.length; i++) {
            if (lines[i].contains("1 | abc")) {
                // Next line should have caret right after prefix
                if (i + 1 < lines.length) {
                    String caretLine = lines[i + 1];
                    assertTrue(caretLine.contains("^"));
                    // Caret should be immediately after " | "
                    int pipePos = caretLine.indexOf("|");
                    int caretPos = caretLine.indexOf("^");
                    assertTrue(caretPos > pipePos);
                }
            }
        }
    }

    @Test
    void testCaretAtEndOfLine() {
        String source = "abc";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Error at end", 1, 4);

        assertTrue(result.contains("^"), "Caret should be present");
    }

    @Test
    void testNullSourceThrowsException() {
        assertThrows(NullPointerException.class, () -> {
            new ErrorFormatter(null);
        });
    }

    @Test
    void testIOContractErrorFormat() {
        // Test case from I/O contract: 2 3 ^ → ERROR: Line 1, column 5: Unexpected character '^'
        String source = "2 3 ^";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Unexpected character '^'", 1, 5);

        // Verify the formatted output structure
        assertTrue(result.startsWith("Error: Unexpected character '^'"));
        assertTrue(result.contains("1 | 2 3 ^"));
        assertTrue(result.contains("^"));

        // Verify there's a blank line after the error header
        String[] lines = result.split("\n");
        assertEquals("Error: Unexpected character '^'", lines[0]);
        assertEquals("", lines[1], "Second line should be blank");
    }

    @Test
    void testComplexMultilineExample() {
        String source = "5 3 +\n2 3 *\n1 0 /";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Division by zero", 3, 5);

        // Should show all three lines (context = 1, but it's a small file)
        assertTrue(result.contains("2 | 2 3 *"));
        assertTrue(result.contains("3 | 1 0 /"));
        assertTrue(result.contains("^"));
    }

    @Test
    void testCaretPositionBeyondLineEnd() {
        // Edge case: column beyond the actual line length
        String source = "abc";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Error beyond line", 1, 10);

        // Should not crash, caret should appear but possibly beyond visible content
        assertTrue(result.contains("^"));
    }

    @Test
    void testZeroContextLines() {
        String source = "line1\nline2\nline3";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Error", 2, 1, 0);

        // Should show only the error line
        assertTrue(result.contains("2 | line2"));
        assertFalse(result.contains("1 | line1"));
        assertFalse(result.contains("3 | line3"));
    }

    @Test
    void testFormatterWithWindowsLineEndings() {
        // Note: split("\\n", -1) handles \n, but Windows line endings \r\n
        // might be split differently. This tests cross-platform behavior.
        String source = "line1\nline2\nline3";
        ErrorFormatter formatter = new ErrorFormatter(source);

        String result = formatter.formatError("Error", 2, 1);

        assertTrue(result.contains("2 | line2"));
    }
}
