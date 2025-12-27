//! Error formatting utilities for displaying parse errors with context.
//!
//! This module provides the `ErrorFormatter` struct for creating user-friendly
//! error messages with source code context and visual indicators.

/// Formats error messages with source code context.
///
/// The `ErrorFormatter` takes source code and provides methods to format
/// error messages with contextual lines and visual indicators (caret) pointing
/// to the exact location of the error.
///
/// # Examples
///
/// ```
/// use rpn2tex::ErrorFormatter;
///
/// let source = "3 4 +\n5 6 *";
/// let formatter = ErrorFormatter::new(source);
/// let error = formatter.format_error("Unexpected token", 1, 5);
/// assert!(error.contains("Error: Unexpected token"));
/// assert!(error.contains("^"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorFormatter {
    source: String,
    lines: Vec<String>,
}

impl ErrorFormatter {
    /// Creates a new error formatter with the given source code.
    ///
    /// The source code is split into lines for efficient context extraction.
    ///
    /// # Arguments
    ///
    /// * `source` - The complete source code text
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ErrorFormatter;
    ///
    /// let formatter = ErrorFormatter::new("3 4 + 5 *");
    /// ```
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let lines = source.lines().map(String::from).collect();
        Self { source, lines }
    }

    /// Formats an error message with one line of context.
    ///
    /// This is a convenience method that calls `format_error_with_context`
    /// with `context_lines = 1`.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message to display
    /// * `line` - The line number where the error occurred (1-based)
    /// * `column` - The column number where the error occurred (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ErrorFormatter;
    ///
    /// let source = "3 4 + !";
    /// let formatter = ErrorFormatter::new(source);
    /// let error = formatter.format_error("Unexpected character '!'", 1, 7);
    /// assert!(error.contains("Error: Unexpected character '!'"));
    /// assert!(error.contains("3 4 + !"));
    /// assert!(error.contains("^"));
    /// ```
    #[must_use]
    pub fn format_error(&self, message: &str, line: u32, column: u32) -> String {
        self.format_error_with_context(message, line, column, 1)
    }

    /// Formats an error message with the specified number of context lines.
    ///
    /// Displays the error message along with surrounding source lines for context.
    /// A caret (^) is placed under the error location to indicate the exact position.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message to display
    /// * `line` - The line number where the error occurred (1-based)
    /// * `column` - The column number where the error occurred (1-based)
    /// * `context_lines` - Number of lines to show before and after the error line
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ErrorFormatter;
    ///
    /// let source = "line1\nline2\nline3\nline4\nline5";
    /// let formatter = ErrorFormatter::new(source);
    /// let error = formatter.format_error_with_context("Error here", 3, 2, 2);
    /// assert!(error.contains("Error: Error here"));
    /// assert!(error.contains("line1"));
    /// assert!(error.contains("line3"));
    /// assert!(error.contains("line5"));
    /// ```
    #[must_use]
    pub fn format_error_with_context(
        &self,
        message: &str,
        line: u32,
        column: u32,
        context_lines: usize,
    ) -> String {
        let mut result = format!("Error: {message}\n");
        result.push_str(&self.get_context(line, column, context_lines));
        result
    }

    /// Gets the context lines around an error location (private helper).
    ///
    /// Extracts and formats the source lines surrounding the error,
    /// with line numbers and a caret indicator.
    fn get_context(&self, line: u32, column: u32, context_lines: usize) -> String {
        use std::fmt::Write;

        if self.lines.is_empty() {
            return String::new();
        }

        // Convert 1-based line to 0-based index
        let error_idx = line.saturating_sub(1) as usize;

        // Calculate context range with boundary checking
        let start_idx = error_idx.saturating_sub(context_lines);
        let end_idx = (error_idx + context_lines + 1).min(self.lines.len());

        // If the line is completely out of bounds, return empty
        if error_idx >= self.lines.len() {
            return String::new();
        }

        // Calculate the width needed for line numbers (based on the max line number in range)
        let max_line_num = end_idx;
        let width = max_line_num.to_string().len();

        let mut context = String::new();

        for (idx, line_content) in self.lines[start_idx..end_idx].iter().enumerate() {
            let line_num = start_idx + idx + 1; // Convert back to 1-based
            let _ = writeln!(context, "{line_num:>width$} | {line_content}");

            // Add caret line if this is the error line
            if start_idx + idx == error_idx {
                let spaces = " ".repeat(width);
                let caret_offset = (column as usize).saturating_sub(1);
                let caret_spacing = " ".repeat(caret_offset);
                let _ = writeln!(context, "{spaces} | {caret_spacing}^");
            }
        }

        context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_formatter_new() {
        let formatter = ErrorFormatter::new("line1\nline2\nline3");
        assert_eq!(formatter.lines.len(), 3);
        assert_eq!(formatter.lines[0], "line1");
        assert_eq!(formatter.lines[1], "line2");
        assert_eq!(formatter.lines[2], "line3");
    }

    #[test]
    fn test_error_formatter_new_with_string() {
        let source = String::from("test");
        let formatter = ErrorFormatter::new(source);
        assert_eq!(formatter.lines.len(), 1);
        assert_eq!(formatter.lines[0], "test");
    }

    #[test]
    fn test_format_error_single_line() {
        let source = "3 4 + !";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Unexpected character '!'", 1, 7);

        assert!(error.contains("Error: Unexpected character '!'"));
        assert!(error.contains("1 | 3 4 + !"));
        assert!(error.contains("  |       ^"));
    }

    #[test]
    fn test_format_error_with_context_multiple_lines() {
        let source = "line1\nline2\nline3\nline4\nline5";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error_with_context("Error here", 3, 2, 1);

        assert!(error.contains("Error: Error here"));
        assert!(error.contains("2 | line2"));
        assert!(error.contains("3 | line3"));
        assert!(error.contains("4 | line4"));
        assert!(error.contains("  |  ^"));
    }

    #[test]
    fn test_format_error_at_start_of_file() {
        let source = "line1\nline2\nline3";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Error at start", 1, 1);

        assert!(error.contains("Error: Error at start"));
        assert!(error.contains("1 | line1"));
        assert!(error.contains("2 | line2"));
        assert!(error.contains("  | ^"));
    }

    #[test]
    fn test_format_error_at_end_of_file() {
        let source = "line1\nline2\nline3";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Error at end", 3, 5);

        assert!(error.contains("Error: Error at end"));
        assert!(error.contains("2 | line2"));
        assert!(error.contains("3 | line3"));
        assert!(error.contains("  |     ^"));
    }

    #[test]
    fn test_format_error_out_of_bounds_line() {
        let source = "line1\nline2";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Out of bounds", 10, 1);

        // Should handle gracefully - just show the header
        assert!(error.contains("Error: Out of bounds"));
        // Context should be empty for out-of-bounds line
        assert!(!error.contains("line1"));
        assert!(!error.contains("line2"));
    }

    #[test]
    fn test_format_error_empty_source() {
        let formatter = ErrorFormatter::new("");
        let error = formatter.format_error("Error in empty source", 1, 1);

        assert!(error.contains("Error: Error in empty source"));
        // No context lines should be present
        assert!(!error.contains("|"));
    }

    #[test]
    fn test_column_positioning() {
        let source = "0123456789";
        let formatter = ErrorFormatter::new(source);

        // Test various column positions
        let error1 = formatter.format_error("At column 1", 1, 1);
        assert!(error1.contains("  | ^"));

        let error5 = formatter.format_error("At column 5", 1, 5);
        assert!(error5.contains("  |     ^"));

        let error10 = formatter.format_error("At column 10", 1, 10);
        assert!(error10.contains("  |          ^"));
    }

    #[test]
    fn test_line_number_width_alignment() {
        // Test with single-digit line numbers
        let source = "line1\nline2\nline3";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Test", 2, 1);
        assert!(error.contains("1 | line1"));
        assert!(error.contains("2 | line2"));
        assert!(error.contains("3 | line3"));

        // Test with double-digit line numbers
        let mut long_source = String::new();
        for i in 1..=15 {
            if i > 1 {
                long_source.push('\n');
            }
            long_source.push_str(&format!("line{i}"));
        }
        let formatter = ErrorFormatter::new(&long_source);
        let error = formatter.format_error_with_context("Test", 10, 1, 2);

        // Line numbers should be right-aligned with width 2
        assert!(error.contains(" 8 | line8"));
        assert!(error.contains(" 9 | line9"));
        assert!(error.contains("10 | line10"));
        assert!(error.contains("11 | line11"));
        assert!(error.contains("12 | line12"));
    }

    #[test]
    fn test_context_lines_zero() {
        let source = "line1\nline2\nline3";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error_with_context("Only error line", 2, 3, 0);

        assert!(error.contains("Error: Only error line"));
        assert!(error.contains("2 | line2"));
        assert!(!error.contains("line1"));
        assert!(!error.contains("line3"));
    }

    #[test]
    fn test_context_lines_large() {
        let source = "line1\nline2\nline3";
        let formatter = ErrorFormatter::new(source);
        // Request 100 lines of context, should safely clamp to available lines
        let error = formatter.format_error_with_context("Large context", 2, 1, 100);

        assert!(error.contains("1 | line1"));
        assert!(error.contains("2 | line2"));
        assert!(error.contains("3 | line3"));
    }

    #[test]
    fn test_error_formatter_clone() {
        let formatter = ErrorFormatter::new("test source");
        let cloned = formatter.clone();
        assert_eq!(formatter, cloned);
        assert_eq!(formatter.source, cloned.source);
        assert_eq!(formatter.lines, cloned.lines);
    }

    #[test]
    fn test_multiline_formatting() {
        let source = "def foo():\n    return bar\n    + baz";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error_with_context("Invalid syntax", 2, 5, 1);

        assert!(error.contains("Error: Invalid syntax"));
        assert!(error.contains("1 | def foo():"));
        assert!(error.contains("2 |     return bar"));
        assert!(error.contains("3 |     + baz"));
        assert!(error.contains("  |     ^"));
    }

    #[test]
    fn test_single_line_source() {
        let formatter = ErrorFormatter::new("single line");
        let error = formatter.format_error("Error", 1, 8);

        assert!(error.contains("Error: Error"));
        assert!(error.contains("1 | single line"));
        assert!(error.contains("  |        ^"));
    }

    #[test]
    fn test_rpn_example() {
        let source = "3 4 + !";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Unexpected character '!'", 1, 7);

        let lines: Vec<&str> = error.lines().collect();
        assert_eq!(lines[0], "Error: Unexpected character '!'");
        assert_eq!(lines[1], "1 | 3 4 + !");
        assert_eq!(lines[2], "  |       ^");
    }
}
