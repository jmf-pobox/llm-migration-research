package com.rpn2tex;

/**
 * Exception thrown during RPN expression parsing and evaluation.
 *
 * <p>This exception captures error information including the error message,
 * line number, and column number where the error occurred. It provides
 * a formatted error message with source context.
 *
 * <p>Example usage:
 * <pre>
 * throw new RpnException("Unexpected character '^'", 1, 5);
 * </pre>
 */
public class RpnException extends Exception {
    /** The error message describing what went wrong. */
    public final String message;

    /** The 1-based line number where the error occurred. */
    public final int line;

    /** The 1-based column number where the error occurred. */
    public final int column;

    /**
     * Creates a new RPN exception.
     *
     * @param message the error message
     * @param line the 1-based line number where the error occurred
     * @param column the 1-based column number where the error occurred
     */
    public RpnException(String message, int line, int column) {
        super(message);
        this.message = message;
        this.line = line;
        this.column = column;
    }

    /**
     * Formats the error message with source context.
     *
     * <p>The formatted message includes:
     * <ul>
     *   <li>The error message</li>
     *   <li>The source line where the error occurred</li>
     *   <li>A caret (^) pointing to the error location</li>
     * </ul>
     *
     * @param source the complete source text
     * @return a formatted error message with context
     */
    public String format(String source) {
        String[] lines = source.split("\n", -1);
        StringBuilder sb = new StringBuilder();

        sb.append("Error: ").append(message).append("\n\n");

        // Show source line with line number
        if (line >= 1 && line <= lines.length) {
            sb.append(line).append(" | ").append(lines[line - 1]).append("\n");

            // Add caret pointer
            sb.append("  | ");
            for (int i = 1; i < column; i++) {
                sb.append(" ");
            }
            sb.append("^");
        }

        return sb.toString();
    }
}
