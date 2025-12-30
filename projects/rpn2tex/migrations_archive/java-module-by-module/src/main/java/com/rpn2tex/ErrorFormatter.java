package com.rpn2tex;

import java.util.Arrays;
import java.util.List;
import java.util.Objects;

/**
 * Utility class for formatting error messages with source context.
 *
 * <p>This formatter displays errors in a compiler-style format, showing the
 * source line where the error occurred with a caret (^) pointing to the
 * exact column.
 *
 * <p>Example output:
 * <pre>
 * Error: Unexpected character: ^
 *
 * 1 | 2 3 ^ 4
 *   |     ^
 * </pre>
 *
 * <p>Example usage:
 * <pre>{@code
 * String source = "2 3 ^ 4";
 * ErrorFormatter formatter = new ErrorFormatter(source);
 * String formatted = formatter.formatError("Unexpected character: ^", 1, 5);
 * System.err.println(formatted);
 * }</pre>
 */
public final class ErrorFormatter {
    private final String source;
    private final List<String> lines;

    /**
     * Constructs an ErrorFormatter for the given source text.
     *
     * @param source the source text to format errors for
     * @throws NullPointerException if source is null
     */
    public ErrorFormatter(String source) {
        this.source = Objects.requireNonNull(source, "source must not be null");
        this.lines = Arrays.asList(source.split("\n", -1));
    }

    /**
     * Formats an error message with source context.
     *
     * <p>This method displays the error message along with the source line
     * where the error occurred, with a caret pointing to the exact column.
     *
     * @param message the error message to display
     * @param line the 1-based line number where the error occurred
     * @param column the 1-based column number where the error occurred
     * @return the formatted error message with context
     * @throws IllegalArgumentException if line or column are less than 1
     */
    public String formatError(String message, int line, int column) {
        return formatError(message, line, column, 1);
    }

    /**
     * Formats an error message with source context and additional context lines.
     *
     * <p>This method displays the error message along with the source line
     * where the error occurred plus additional surrounding lines for context.
     *
     * @param message the error message to display
     * @param line the 1-based line number where the error occurred
     * @param column the 1-based column number where the error occurred
     * @param contextLines the number of lines to show before and after the error line
     * @return the formatted error message with context
     * @throws IllegalArgumentException if line or column are less than 1, or contextLines is negative
     */
    public String formatError(String message, int line, int column, int contextLines) {
        Objects.requireNonNull(message, "message must not be null");
        if (line < 1) {
            throw new IllegalArgumentException("line must be >= 1");
        }
        if (column < 1) {
            throw new IllegalArgumentException("column must be >= 1");
        }
        if (contextLines < 0) {
            throw new IllegalArgumentException("contextLines must be >= 0");
        }

        var sb = new StringBuilder();
        sb.append("Error: ").append(message).append("\n\n");
        sb.append(getContext(line, column, contextLines));
        return sb.toString();
    }

    /**
     * Gets the source context around the specified position.
     *
     * @param line the 1-based line number
     * @param column the 1-based column number
     * @param contextLines the number of lines to show before and after
     * @return the formatted context with line numbers and caret
     */
    private String getContext(int line, int column, int contextLines) {
        var sb = new StringBuilder();

        // Calculate the range of lines to display
        int startLine = Math.max(1, line - contextLines);
        int endLine = Math.min(lines.size(), line + contextLines);

        // Calculate the width needed for line numbers
        int maxLineNum = endLine;
        int lineNumWidth = String.valueOf(maxLineNum).length();

        // Display the lines
        for (int i = startLine; i <= endLine; i++) {
            String lineContent = getLine(i);
            sb.append(String.format("%" + lineNumWidth + "d | %s\n", i, lineContent));

            // Add the caret line if this is the error line
            if (i == line) {
                // Create the caret line with proper spacing
                sb.append(" ".repeat(lineNumWidth));
                sb.append(" | ");
                // Position the caret at the correct column (accounting for 1-based indexing)
                int caretPosition = Math.max(0, column - 1);
                sb.append(" ".repeat(caretPosition));
                sb.append("^\n");
            }
        }

        return sb.toString();
    }

    /**
     * Gets the content of a specific line (1-based).
     *
     * @param lineNum the 1-based line number
     * @return the line content, or an empty string if the line doesn't exist
     */
    private String getLine(int lineNum) {
        if (lineNum < 1 || lineNum > lines.size()) {
            return "";
        }
        return lines.get(lineNum - 1);
    }
}
