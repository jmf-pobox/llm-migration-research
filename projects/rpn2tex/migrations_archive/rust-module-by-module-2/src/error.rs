//! Error formatting utilities for rpn2tex.
//!
//! This module provides the `ErrorFormatter` struct which formats error messages
//! with source context and visual indicators showing exactly where an error occurred.

/// Formats error messages with source context and position indicators.
///
/// The formatter displays errors with line numbers, the source line where the error
/// occurred, and a caret (^) pointing to the exact column of the error.
///
/// # Examples
///
/// ```
/// use rpn2tex::ErrorFormatter;
///
/// let source = "2 3 ^";
/// let formatter = ErrorFormatter::new(source);
/// let error = formatter.format_error("Unexpected character '^'", 1, 5);
///
/// // Output format:
/// // Error: Unexpected character '^'
/// //
/// // 1 | 2 3 ^
/// //   |     ^
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorFormatter {
    source: String,
    lines: Vec<String>,
}

impl ErrorFormatter {
    /// Creates a new error formatter for the given source text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ErrorFormatter;
    ///
    /// let formatter = ErrorFormatter::new("5 3 +");
    /// ```
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let lines = source
            .lines()
            .map(std::string::ToString::to_string)
            .collect();
        Self { source, lines }
    }

    /// Formats an error message with source context.
    ///
    /// This is a convenience method that calls `format_error_with_context` with
    /// a default context of 1 line.
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
    /// let formatter = ErrorFormatter::new("2 3 ^");
    /// let error = formatter.format_error("Unexpected character '^'", 1, 5);
    /// assert!(error.contains("Error: Unexpected character '^'"));
    /// assert!(error.contains("1 | 2 3 ^"));
    /// ```
    #[must_use]
    pub fn format_error(&self, message: &str, line: usize, column: usize) -> String {
        self.format_error_with_context(message, line, column, 1)
    }

    /// Formats an error message with source context and a specified number of context lines.
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
    /// let source = "line1\nline2\nline3";
    /// let formatter = ErrorFormatter::new(source);
    /// let error = formatter.format_error_with_context("Test error", 2, 3, 1);
    /// ```
    #[must_use]
    pub fn format_error_with_context(
        &self,
        message: &str,
        line: usize,
        column: usize,
        context_lines: usize,
    ) -> String {
        let mut parts = Vec::new();

        // Error header
        parts.push(format!("Error: {message}"));
        parts.push(String::new()); // Blank line

        // Source context
        let context = self.get_context(line, column, context_lines);
        parts.push(context);

        parts.join("\n")
    }

    /// Extracts source context around an error position.
    ///
    /// Returns a formatted string with line numbers, source lines, and a caret
    /// pointing to the error column.
    fn get_context(&self, line: usize, column: usize, context_lines: usize) -> String {
        // Convert to 0-based index
        let error_idx = line.saturating_sub(1);

        // Calculate range (clamped to valid indices)
        let start_idx = error_idx.saturating_sub(context_lines);
        let end_idx = std::cmp::min(self.lines.len(), error_idx + context_lines + 1);

        // Calculate line number width for alignment
        let max_line_num = end_idx;
        let num_width = max_line_num.to_string().len();

        let mut result_lines = Vec::new();

        for idx in start_idx..end_idx {
            let line_num = idx + 1; // Convert back to 1-based
            let line_content = self.lines.get(idx).map_or("", String::as_str);

            // Format line with number: "1 | source code"
            let prefix = format!("{line_num:>width$} | ", width = num_width);
            result_lines.push(format!("{prefix}{line_content}"));

            // Add caret on error line
            if idx == error_idx {
                // Spaces for line number column, then position caret
                let caret_prefix = format!("{:>width$} | ", "", width = num_width);
                // Position caret at column (1-based, so column-1 spaces)
                let caret_pos = column.saturating_sub(1);
                let caret_line = format!("{caret_prefix}{:>width$}^", "", width = caret_pos);
                result_lines.push(caret_line);
            }
        }

        result_lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_formatter_creation() {
        let formatter = ErrorFormatter::new("5 3 ^");
        assert_eq!(formatter.source, "5 3 ^");
        assert_eq!(formatter.lines, vec!["5 3 ^"]);
    }

    #[test]
    fn test_format_error_basic() {
        let formatter = ErrorFormatter::new("5 3 ^");
        let error = formatter.format_error("Unexpected character '^'", 1, 5);

        assert!(error.contains("Error: Unexpected character '^'"));
        assert!(error.contains("1 | 5 3 ^"));
        assert!(error.contains("^"));
    }

    #[test]
    fn test_caret_position() {
        let formatter = ErrorFormatter::new("hello world");
        let error = formatter.format_error("Test", 1, 7);

        // Caret should be at position 6 (0-based)
        let lines: Vec<&str> = error.lines().collect();
        // Line 0: "Error: Test"
        // Line 1: "" (blank)
        // Line 2: "1 | hello world"
        // Line 3: "  |       ^" (caret line - 7 spaces from start of line)
        assert_eq!(lines.len(), 4);
        let caret_line = lines[3];
        // The caret line has format "  |       ^" where spaces position the caret
        // It should contain the pipe and end with the caret
        assert!(caret_line.contains('|'));
        assert!(caret_line.trim_end().ends_with('^'));
    }

    #[test]
    fn test_multiline_source() {
        let source = "line1\nline2\nline3";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Error on line 2", 2, 3);

        assert!(error.contains("2 | line2"));
        assert!(error.contains("^"));
    }

    #[test]
    fn test_error_at_first_column() {
        let formatter = ErrorFormatter::new("abc");
        let error = formatter.format_error("Error at start", 1, 1);

        let lines: Vec<&str> = error.lines().collect();
        let caret_line = lines[3];
        // Caret should be immediately after " | "
        assert!(caret_line.ends_with("^"));
    }

    #[test]
    fn test_error_with_context() {
        let source = "line1\nline2\nline3\nline4\nline5";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error_with_context("Error", 3, 2, 2);

        // Should show lines 1-5 (context of 2 means 2 before, 2 after)
        assert!(error.contains("1 | line1"));
        assert!(error.contains("2 | line2"));
        assert!(error.contains("3 | line3"));
        assert!(error.contains("4 | line4"));
        assert!(error.contains("5 | line5"));
    }

    #[test]
    fn test_line_number_alignment() {
        let source = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Error", 10, 1);

        // Line numbers should be aligned
        let lines: Vec<&str> = error.lines().collect();
        // Find the line with "10 |"
        let line_10 = lines.iter().find(|l| l.contains("10 |")).unwrap();
        assert!(line_10.starts_with("10 | "));
    }

    #[test]
    fn test_empty_line() {
        let source = "line1\n\nline3";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Error on empty line", 2, 1);

        assert!(error.contains("2 | "));
        // Empty line should still have caret
        assert!(error.contains("^"));
    }

    #[test]
    fn test_exact_output_format() {
        let formatter = ErrorFormatter::new("2 3 ^");
        let error = formatter.format_error("Unexpected character '^'", 1, 5);

        let expected_lines = vec![
            "Error: Unexpected character '^'",
            "",
            "1 | 2 3 ^",
            "  |     ^",
        ];

        let actual_lines: Vec<&str> = error.lines().collect();
        assert_eq!(actual_lines, expected_lines);
    }

    #[test]
    fn test_saturating_sub_edge_case() {
        let formatter = ErrorFormatter::new("test");
        // Line 0 or column 0 should not panic due to saturating_sub
        let error = formatter.format_error("Test", 0, 0);
        // Should handle gracefully
        assert!(error.contains("Error: Test"));
    }
}
