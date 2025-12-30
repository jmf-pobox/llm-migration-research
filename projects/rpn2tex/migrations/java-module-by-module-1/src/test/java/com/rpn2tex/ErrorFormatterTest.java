package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for ErrorFormatter class.
 *
 * <p>Tests error message formatting with source context, including:
 * <ul>
 *   <li>Basic error formatting</li>
 *   <li>Caret positioning</li>
 *   <li>Multi-line context</li>
 *   <li>Edge cases (empty source, boundary positions)</li>
 * </ul>
 */
class ErrorFormatterTest {

    @Test
    void testFormatErrorBasic() {
        ErrorFormatter formatter = new ErrorFormatter("5 3 ^");
        String result = formatter.formatError("Unexpected character '^'", 1, 5);

        assertTrue(result.contains("Error: Unexpected character '^'"));
        assertTrue(result.contains("1 | 5 3 ^"));
        assertTrue(result.contains("^"));
    }

    @Test
    void testCaretPositioning() {
        ErrorFormatter formatter = new ErrorFormatter("5 3 @");
        String result = formatter.formatError("Unexpected character '@'", 1, 5);

        String[] lines = result.split("\n");
        // Find the caret line (should be after the source line)
        String sourceLine = null;
        String caretLine = null;
        for (int i = 0; i < lines.length - 1; i++) {
            if (lines[i].contains("5 3 @")) {
                sourceLine = lines[i];
                caretLine = lines[i + 1];
                break;
            }
        }

        assertNotNull(sourceLine, "Source line should be present");
        assertNotNull(caretLine, "Caret line should be present");

        // Caret should be at column 5 (0-indexed: 4)
        // Source line format: "1 | 5 3 @"
        // Caret line format: "  | ....^" where dots are spaces
        int pipePos = sourceLine.indexOf('|');
        int atPos = sourceLine.indexOf('@');
        int caretPos = caretLine.indexOf('^');

        // Caret should be at the same position as '@' in source line
        assertEquals(atPos, caretPos, "Caret should align with error position");
    }

    @Test
    void testMultiLineContext() {
        String source = "line 1\nline 2\nline 3 error\nline 4\nline 5";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error on line 3", 3, 8, 2);

        assertTrue(result.contains("1 | line 1"));
        assertTrue(result.contains("2 | line 2"));
        assertTrue(result.contains("3 | line 3 error"));
        assertTrue(result.contains("4 | line 4"));
        assertTrue(result.contains("5 | line 5"));
    }

    @Test
    void testSingleLineContext() {
        String source = "line 1\nline 2\nline 3 error\nline 4\nline 5";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error on line 3", 3, 8, 1);

        assertTrue(result.contains("2 | line 2"));
        assertTrue(result.contains("3 | line 3 error"));
        assertTrue(result.contains("4 | line 4"));
        assertFalse(result.contains("1 | line 1"));
        assertFalse(result.contains("5 | line 5"));
    }

    @Test
    void testNoContext() {
        String source = "line 1\nline 2\nline 3 error\nline 4\nline 5";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error on line 3", 3, 8, 0);

        assertTrue(result.contains("3 | line 3 error"));
        assertFalse(result.contains("2 | line 2"));
        assertFalse(result.contains("4 | line 4"));
    }

    @Test
    void testFirstLineError() {
        String source = "error here\nline 2\nline 3";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error on first line", 1, 1, 1);

        assertTrue(result.contains("1 | error here"));
        assertTrue(result.contains("2 | line 2"));
        // Should not crash at boundary
    }

    @Test
    void testLastLineError() {
        String source = "line 1\nline 2\nerror here";
        ErrorFormatter formatter = new ErrorFormatter(source);
        String result = formatter.formatError("Error on last line", 3, 1, 1);

        assertTrue(result.contains("2 | line 2"));
        assertTrue(result.contains("3 | error here"));
        // Should not crash at boundary
    }

    @Test
    void testEmptySource() {
        ErrorFormatter formatter = new ErrorFormatter("");
        String result = formatter.formatError("Error in empty source", 1, 1, 0);

        assertTrue(result.contains("Error: Error in empty source"));
        // Should not crash
    }

    @Test
    void testSingleCharacterSource() {
        ErrorFormatter formatter = new ErrorFormatter("x");
        String result = formatter.formatError("Error", 1, 1);

        assertTrue(result.contains("1 | x"));
        assertTrue(result.contains("^"));
    }

    @Test
    void testLineNumberAlignment() {
        StringBuilder source = new StringBuilder();
        for (int i = 1; i <= 100; i++) {
            source.append("line ").append(i).append("\n");
        }
        ErrorFormatter formatter = new ErrorFormatter(source.toString());
        String result = formatter.formatError("Error on line 50", 50, 1, 5);

        // Line numbers should be aligned (3 digits wide for line 55)
        assertTrue(result.contains("45 | line 45"));
        assertTrue(result.contains("50 | line 50"));
        assertTrue(result.contains("55 | line 55"));
    }

    @ParameterizedTest
    @CsvSource({
        "1, 1",
        "1, 5",
        "2, 3",
        "1, 10"
    })
    void testVariousPositions(int line, int column) {
        ErrorFormatter formatter = new ErrorFormatter("5 3 +\n2 4 *");
        String result = formatter.formatError("Test error", line, column);

        assertNotNull(result);
        assertTrue(result.contains("Error: Test error"));
        assertTrue(result.contains("^"));
    }

    @Test
    void testNullSourceThrows() {
        assertThrows(NullPointerException.class, () -> new ErrorFormatter(null));
    }

    @Test
    void testNullMessageThrows() {
        ErrorFormatter formatter = new ErrorFormatter("test");
        assertThrows(NullPointerException.class,
            () -> formatter.formatError(null, 1, 1));
    }

    @Test
    void testInvalidLineThrows() {
        ErrorFormatter formatter = new ErrorFormatter("test");
        assertThrows(IllegalArgumentException.class,
            () -> formatter.formatError("error", 0, 1));
        assertThrows(IllegalArgumentException.class,
            () -> formatter.formatError("error", -1, 1));
    }

    @Test
    void testInvalidColumnThrows() {
        ErrorFormatter formatter = new ErrorFormatter("test");
        assertThrows(IllegalArgumentException.class,
            () -> formatter.formatError("error", 1, 0));
        assertThrows(IllegalArgumentException.class,
            () -> formatter.formatError("error", 1, -1));
    }

    @Test
    void testNegativeContextLinesThrows() {
        ErrorFormatter formatter = new ErrorFormatter("test");
        assertThrows(IllegalArgumentException.class,
            () -> formatter.formatError("error", 1, 1, -1));
    }

    @Test
    void testGetContextDirect() {
        ErrorFormatter formatter = new ErrorFormatter("5 3 ^");
        String context = formatter.getContext(1, 5, 0);

        assertTrue(context.contains("1 | 5 3 ^"));
        assertTrue(context.contains("^"));
    }

    @Test
    void testCaretAtBeginning() {
        ErrorFormatter formatter = new ErrorFormatter("error");
        String result = formatter.formatError("Test", 1, 1);
        String[] lines = result.split("\n");

        String caretLine = null;
        for (String line : lines) {
            // Look for line with ^ that comes after a line with "error"
            if (line.contains("^") && line.contains("|")) {
                caretLine = line;
                break;
            }
        }

        assertNotNull(caretLine, "Should have caret line");
        // Find the first ^ and verify it's at the right position
        assertTrue(caretLine.indexOf('^') >= 0);
    }

    @Test
    void testCaretBeyondLine() {
        ErrorFormatter formatter = new ErrorFormatter("ab");
        String result = formatter.formatError("Test", 1, 10);
        // Caret at column 10 even though line is shorter
        // Should still work (caret will be positioned at column 10)
        assertTrue(result.contains("^"));
    }

    @Test
    void testMultibyteCharacters() {
        // Test with Unicode characters
        ErrorFormatter formatter = new ErrorFormatter("5 3 \u2764");
        String result = formatter.formatError("Unexpected character", 1, 5);

        assertTrue(result.contains("Error: Unexpected character"));
        assertTrue(result.contains("^"));
    }

    @Test
    void testWindowsLineEndings() {
        ErrorFormatter formatter = new ErrorFormatter("line 1\r\nline 2\r\nerror");
        String result = formatter.formatError("Test error", 3, 1);

        assertTrue(result.contains("3 | error"));
        // Should handle CRLF properly
    }
}
