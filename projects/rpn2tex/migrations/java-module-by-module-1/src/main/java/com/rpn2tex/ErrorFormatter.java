package com.rpn2tex;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

/**
 * Formats parse errors with source context and helpful visual indicators.
 *
 * <p>Provides compiler-style error output similar to gcc/rustc with:
 * <ul>
 *   <li>Line numbers and source context</li>
 *   <li>Caret (^) pointing to error column</li>
 *   <li>Clear, readable error messages</li>
 * </ul>
 *
 * <p>Example output:
 * <pre>
 * Error: Unexpected character '^'
 *
 * 1 | 5 3 ^
 *         ^
 * </pre>
 *
 * <p>Example usage:
 * <pre>
 * ErrorFormatter formatter = new ErrorFormatter("5 3 ^");
 * String errorMsg = formatter.formatError("Unexpected character '^'", 1, 5);
 * System.err.println(errorMsg);
 * </pre>
 *
 * <p>This class is thread-safe and immutable after construction.
 *
 * @see RpnException
 */
public final class ErrorFormatter {
    /** The complete source text being parsed. */
    private final String source;

    /** Source text split into lines for easy access. */
    private final List<String> lines;

    /**
     * Initializes formatter with source text.
     *
     * <p>The source text is split into lines for context extraction.
     * Empty sources are supported but will produce minimal context output.
     *
     * @param source the complete source text being parsed
     * @throws NullPointerException if source is null
     */
    public ErrorFormatter(final String source) {
        this.source = Objects.requireNonNull(source, "source must not be null");
        this.lines = splitLines(source);
    }

    /**
     * Formats an error with source context.
     *
     * <p>Uses default context of 1 line before and after the error line.
     *
     * @param message the error message
     * @param line the 1-based line number
     * @param column the 1-based column number
     * @return formatted error string with context
     * @throws NullPointerException if message is null
     * @throws IllegalArgumentException if line or column is less than 1
     */
    public String formatError(final String message, final int line, final int column) {
        return formatError(message, line, column, 1);
    }

    /**
     * Formats an error with source context and custom context lines.
     *
     * <p>The formatted error includes:
     * <ul>
     *   <li>Error header: "Error: {message}"</li>
     *   <li>Blank line</li>
     *   <li>Source context with line numbers</li>
     *   <li>Caret line pointing to error location</li>
     * </ul>
     *
     * @param message the error message
     * @param line the 1-based line number
     * @param column the 1-based column number
     * @param contextLines number of lines to show before/after error line
     * @return formatted error string with context
     * @throws NullPointerException if message is null
     * @throws IllegalArgumentException if line, column, or contextLines is invalid
     *
     * @see #getContext(int, int, int)
     */
    public String formatError(final String message, final int line, final int column,
                             final int contextLines) {
        Objects.requireNonNull(message, "message must not be null");
        validatePosition(line, column);
        if (contextLines < 0) {
            throw new IllegalArgumentException("contextLines must be >= 0, got: " + contextLines);
        }

        StringBuilder result = new StringBuilder();

        // Error header
        result.append("Error: ").append(message).append("\n");
        result.append("\n");

        // Source context
        String context = getContext(line, column, contextLines);
        result.append(context);

        return result.toString();
    }

    /**
     * Extracts source context around error position.
     *
     * <p>Shows the specified number of lines before and after the error line,
     * with line numbers aligned and a caret pointing to the error column.
     *
     * <p>Line numbers are right-aligned to handle varying digit widths.
     * The caret is positioned at the exact column of the error (1-based).
     *
     * @param line the 1-based error line number
     * @param column the 1-based error column number
     * @param contextLines number of lines to show before/after
     * @return formatted context string with line numbers and caret
     */
    String getContext(final int line, final int column, final int contextLines) {
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
                // Spaces for line number column, then position caret
                String caretPrefix = " ".repeat(numWidth) + " | ";
                // Position caret at column (1-based, so column-1 spaces)
                int caretPos = Math.max(0, column - 1);
                String caretLine = caretPrefix + " ".repeat(caretPos) + "^";
                resultLines.add(caretLine);
            }
        }

        return String.join("\n", resultLines);
    }

    /**
     * Splits source text into lines.
     *
     * <p>Handles various line ending styles (LF, CRLF) and preserves
     * empty lines in the source.
     *
     * @param text the source text to split
     * @return list of lines
     */
    private static List<String> splitLines(final String text) {
        // Use splitlines-style behavior: split on \n but preserve structure
        String[] parts = text.split("\n", -1);
        List<String> result = new ArrayList<>(parts.length);
        for (String part : parts) {
            // Remove trailing \r if present (handles CRLF)
            if (part.endsWith("\r")) {
                result.add(part.substring(0, part.length() - 1));
            } else {
                result.add(part);
            }
        }
        return result;
    }

    /**
     * Validates that line and column are valid 1-based positions.
     *
     * @param line the line number to validate
     * @param column the column number to validate
     * @throws IllegalArgumentException if line or column is less than 1
     */
    private static void validatePosition(final int line, final int column) {
        if (line < 1) {
            throw new IllegalArgumentException("line must be >= 1, got: " + line);
        }
        if (column < 1) {
            throw new IllegalArgumentException("column must be >= 1, got: " + column);
        }
    }
}
