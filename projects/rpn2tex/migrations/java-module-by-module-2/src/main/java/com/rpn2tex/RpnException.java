package com.rpn2tex;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

/**
 * Base exception class for RPN parsing and lexing errors.
 *
 * <p>This exception captures detailed error information including:
 * <ul>
 *   <li>A descriptive error message</li>
 *   <li>The line number (1-based) where the error occurred</li>
 *   <li>The column number (1-based) where the error occurred</li>
 * </ul>
 *
 * <p>The exception message is automatically formatted as:
 * {@code Line <line>, column <column>: <message>}
 *
 * <p>Example usage:
 * <pre>{@code
 * throw new RpnException("Unexpected character '^'", 1, 5);
 * // Message: "Line 1, column 5: Unexpected character '^'"
 * }</pre>
 *
 * @since 1.0
 */
public class RpnException extends Exception {
    private final String errorMessage;
    private final int line;
    private final int column;

    /**
     * Constructs a new RPN exception with the specified error details.
     *
     * @param message the error message describing what went wrong
     * @param line    the line number (1-based) where the error occurred
     * @param column  the column number (1-based) where the error occurred
     * @throws NullPointerException     if message is null
     * @throws IllegalArgumentException if line or column is less than 1
     */
    public RpnException(String message, int line, int column) {
        super(formatMessage(message, line, column));
        Objects.requireNonNull(message, "Error message cannot be null");
        if (line < 1) {
            throw new IllegalArgumentException("Line number must be >= 1, got: " + line);
        }
        if (column < 1) {
            throw new IllegalArgumentException("Column number must be >= 1, got: " + column);
        }
        this.errorMessage = message;
        this.line = line;
        this.column = column;
    }

    /**
     * Returns the error message without the line/column prefix.
     *
     * @return the descriptive error message
     */
    public String getErrorMessage() {
        return errorMessage;
    }

    /**
     * Returns the line number where the error occurred.
     *
     * @return the line number (1-based)
     */
    public int getLine() {
        return line;
    }

    /**
     * Returns the column number where the error occurred.
     *
     * @return the column number (1-based)
     */
    public int getColumn() {
        return column;
    }

    /**
     * Formats an error message with line and column information.
     *
     * @param message the error message
     * @param line    the line number (1-based)
     * @param column  the column number (1-based)
     * @return formatted message: "Line X, column Y: message"
     */
    private static String formatMessage(String message, int line, int column) {
        return String.format("Line %d, column %d: %s", line, column, message);
    }
}

/**
 * Utility class for formatting error messages with source code context.
 *
 * <p>This class provides gcc/rustc-style error formatting with:
 * <ul>
 *   <li>Line numbers and source line content</li>
 *   <li>A caret (^) pointing to the exact error location</li>
 *   <li>Configurable context lines before and after the error</li>
 * </ul>
 *
 * <p>Example output:
 * <pre>{@code
 * Error: Unexpected character '^'
 *
 * 1 | 2 3 ^
 *       ^
 * }</pre>
 *
 * <p>Example usage:
 * <pre>{@code
 * String source = "5 3 +\n2 3 ^";
 * ErrorFormatter formatter = new ErrorFormatter(source);
 * String formatted = formatter.formatError("Unexpected character '^'", 2, 5);
 * System.err.println(formatted);
 * }</pre>
 *
 * @since 1.0
 */
class ErrorFormatter {
    private final String source;
    private final List<String> lines;

    /**
     * Creates a new error formatter for the given source text.
     *
     * @param source the complete source text being parsed
     * @throws NullPointerException if source is null
     */
    public ErrorFormatter(String source) {
        Objects.requireNonNull(source, "Source cannot be null");
        this.source = source;
        this.lines = new ArrayList<>();
        // Split on newlines, preserving empty lines
        for (String line : source.split("\\n", -1)) {
            this.lines.add(line);
        }
    }

    /**
     * Formats an error message with source context (default: 1 context line).
     *
     * @param message the error message
     * @param line    the line number (1-based) where the error occurred
     * @param column  the column number (1-based) where the error occurred
     * @return formatted error string with source context
     */
    public String formatError(String message, int line, int column) {
        return formatError(message, line, column, 1);
    }

    /**
     * Formats an error message with source context.
     *
     * <p>The formatted output includes:
     * <ul>
     *   <li>Error header: "Error: message"</li>
     *   <li>Blank line</li>
     *   <li>Source lines with line numbers (context_lines before and after)</li>
     *   <li>A caret line pointing to the error column</li>
     * </ul>
     *
     * @param message      the error message
     * @param line         the line number (1-based) where the error occurred
     * @param column       the column number (1-based) where the error occurred
     * @param contextLines number of lines to show before and after the error
     * @return formatted error string with source context
     */
    public String formatError(String message, int line, int column, int contextLines) {
        List<String> parts = new ArrayList<>();

        parts.add("Error: " + message);
        parts.add("");

        String context = getContext(line, column, contextLines);
        parts.add(context);

        return String.join("\n", parts);
    }

    /**
     * Extracts and formats source context around the error position.
     *
     * @param line         the error line number (1-based)
     * @param column       the error column number (1-based)
     * @param contextLines number of context lines before and after
     * @return formatted source context with line numbers and caret
     */
    private String getContext(int line, int column, int contextLines) {
        // Convert to 0-based index
        int errorIdx = line - 1;

        // Calculate range (clamped to valid indices)
        int startIdx = Math.max(0, errorIdx - contextLines);
        int endIdx = Math.min(lines.size(), errorIdx + contextLines + 1);

        // Calculate line number width for alignment
        int maxLineNum = endIdx;
        int numWidth = String.valueOf(maxLineNum).length();

        List<String> resultLines = new ArrayList<>();

        for (int idx = startIdx; idx < endIdx; idx++) {
            int lineNum = idx + 1;  // Convert back to 1-based
            String lineContent = idx < lines.size() ? lines.get(idx) : "";

            // Format line with number
            String prefix = String.format("%" + numWidth + "d | ", lineNum);
            resultLines.add(prefix + lineContent);

            // Add caret on error line
            if (idx == errorIdx) {
                String caretPrefix = " ".repeat(numWidth) + " | ";
                int caretPos = Math.max(0, column - 1);
                String caretLine = caretPrefix + " ".repeat(caretPos) + "^";
                resultLines.add(caretLine);
            }
        }

        return String.join("\n", resultLines);
    }
}
