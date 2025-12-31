package com.rpn2tex;

import java.util.Objects;

/**
 * Custom exception class for rpn2tex errors with position tracking.
 *
 * <p>This class serves as a base for all rpn2tex errors and includes
 * functionality for formatting compiler-style error messages with
 * source context.
 *
 * <p>Error messages are formatted in GCC/Rustc style:
 * <pre>
 * Error: Unexpected character '^'
 *
 * 1 | 2 3 ^ 4 *
 *         ^
 * </pre>
 *
 * @author rpn2tex
 * @version 1.0
 */
public class RpnException extends Exception {
    private final String errorMessage;
    private final int line;
    private final int column;

    /**
     * Constructs a new RpnException with the specified message and position.
     *
     * @param message the error message describing what went wrong
     * @param line the 1-based line number where the error occurred
     * @param column the 1-based column number where the error occurred
     * @throws NullPointerException if message is null
     */
    public RpnException(String message, int line, int column) {
        super("Line " + line + ", column " + column + ": " + message);
        this.errorMessage = Objects.requireNonNull(message, "message cannot be null");
        this.line = line;
        this.column = column;
    }

    /**
     * Returns the error message without position information.
     *
     * @return the error message
     */
    public String getErrorMessage() {
        return errorMessage;
    }

    /**
     * Returns the 1-based line number where the error occurred.
     *
     * @return the line number
     */
    public int getLine() {
        return line;
    }

    /**
     * Returns the 1-based column number where the error occurred.
     *
     * @return the column number
     */
    public int getColumn() {
        return column;
    }

    /**
     * Error formatter with source context for user-friendly output.
     *
     * <p>This class provides compiler-style error formatting with source
     * context and caret positioning. It tracks line breaks and calculates
     * line/column information for display.
     *
     * <p>Example usage:
     * <pre>
     * ErrorFormatter formatter = new ErrorFormatter("5 3 ^ +");
     * String formatted = formatter.formatError("Unexpected character '^'", 1, 5);
     * System.err.println(formatted);
     * </pre>
     */
    public static final class ErrorFormatter {
        private final String source;
        private final String[] lines;

        /**
         * Constructs a new ErrorFormatter with the given source text.
         *
         * @param source the complete source text to be formatted
         * @throws NullPointerException if source is null
         */
        public ErrorFormatter(String source) {
            this.source = Objects.requireNonNull(source, "source cannot be null");
            // Split on newlines, keeping trailing empty strings
            this.lines = source.split("\n", -1);
        }

        /**
         * Formats an error with source context and default context lines (1).
         *
         * @param message the error message
         * @param line the 1-based line number where the error occurred
         * @param column the 1-based column number where the error occurred
         * @return the formatted error string with context
         */
        public String formatError(String message, int line, int column) {
            return formatError(message, line, column, 1);
        }

        /**
         * Formats an error with source context.
         *
         * <p>The formatted output includes:
         * <ul>
         *   <li>Error message header with "Error: " prefix</li>
         *   <li>Blank line for readability</li>
         *   <li>Source line(s) with line numbers</li>
         *   <li>Caret (^) pointing to the error column</li>
         * </ul>
         *
         * @param message the error message
         * @param line the 1-based line number where the error occurred
         * @param column the 1-based column number where the error occurred
         * @param contextLines the number of lines to show before and after the error
         * @return the formatted error string with context
         * @throws NullPointerException if message is null
         */
        public String formatError(String message, int line, int column, int contextLines) {
            Objects.requireNonNull(message, "message cannot be null");

            StringBuilder sb = new StringBuilder();
            sb.append("Error: ").append(message).append("\n\n");
            sb.append(getContext(line, column, contextLines));
            return sb.toString();
        }

        /**
         * Extracts context around the error position.
         *
         * <p>This method generates a formatted string showing the source lines
         * around the error position, with line numbers and a caret pointing to
         * the exact error location.
         *
         * @param line the 1-based line number where the error occurred
         * @param column the 1-based column number where the error occurred
         * @param contextLines the number of lines to show before and after the error
         * @return the formatted context string
         */
        private String getContext(int line, int column, int contextLines) {
            // Convert 1-based line to 0-based array index
            int errorIdx = line - 1;

            // Clamp range to valid line indices
            int startIdx = Math.max(0, errorIdx - contextLines);
            int endIdx = Math.min(lines.length, errorIdx + contextLines + 1);

            // Calculate line number width for alignment
            int numWidth = String.valueOf(endIdx).length();

            StringBuilder sb = new StringBuilder();
            for (int idx = startIdx; idx < endIdx; idx++) {
                int lineNum = idx + 1;
                String lineContent = idx < lines.length ? lines[idx] : "";

                // Format line with number: "  1 | content"
                String prefix = String.format("%" + numWidth + "d | ", lineNum);
                sb.append(prefix).append(lineContent).append("\n");

                // Add caret line on error line
                if (idx == errorIdx) {
                    // Create prefix with spaces: "    | "
                    String caretPrefix = " ".repeat(numWidth) + " | ";
                    // Position caret at column (adjusting to 0-based)
                    int caretPos = Math.max(0, column - 1);
                    sb.append(caretPrefix).append(" ".repeat(caretPos)).append("^\n");
                }
            }

            // Remove trailing newline if present
            String result = sb.toString();
            if (result.endsWith("\n")) {
                result = result.substring(0, result.length() - 1);
            }
            return result;
        }
    }
}
