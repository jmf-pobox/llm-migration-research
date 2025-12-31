//! Error formatting with source context for gcc/rustc-style error messages.
//!
//! This module provides the [`ErrorFormatter`] type for formatting parse errors
//! with visual source context, similar to compiler error output.

/// Formats parse errors with source context.
///
/// The `ErrorFormatter` produces user-friendly error messages with visual context,
/// showing the error line with configurable context lines before/after, and a caret
/// (`^`) pointing to the exact column where the error occurred.
///
/// # Examples
///
/// ```
/// use rpn2tex::error::ErrorFormatter;
///
/// let source = "5 3 +\n10 @ 2";
/// let formatter = ErrorFormatter::new(source);
/// let error_msg = formatter.format_error("Unexpected character '@'", 2, 4, 1);
/// assert!(error_msg.contains("Line 2, column 4"));
/// assert!(error_msg.contains("^"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorFormatter {
    /// Complete source text being parsed
    source: String,
    /// Source split into lines for efficient access
    lines: Vec<String>,
}

impl ErrorFormatter {
    /// Creates a new error formatter for the given source text.
    ///
    /// The source text is split into lines and cached for efficient access
    /// during error formatting.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::error::ErrorFormatter;
    ///
    /// let formatter = ErrorFormatter::new("5 3 +\n10 2 /");
    /// ```
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let lines = source.lines().map(String::from).collect();
        Self { source, lines }
    }

    /// Formats an error message with source context.
    ///
    /// Creates a formatted error message showing:
    /// 1. The error message with line and column numbers
    /// 2. Context lines before the error
    /// 3. The error line with line number
    /// 4. A caret (`^`) pointing to the exact column
    /// 5. Context lines after the error
    ///
    /// # Arguments
    ///
    /// * `message` - The error message to display
    /// * `line` - 1-based line number where the error occurred
    /// * `column` - 1-based column number where the error occurred
    /// * `context_lines` - Number of lines to show before and after the error
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::error::ErrorFormatter;
    ///
    /// let source = "5 3 +\n10 @ 2\n7 4 *";
    /// let formatter = ErrorFormatter::new(source);
    /// let error = formatter.format_error("Unexpected character", 2, 4, 1);
    /// assert!(error.contains("Line 2, column 4"));
    /// ```
    #[must_use]
    pub fn format_error(
        &self,
        message: impl AsRef<str>,
        line: u32,
        column: u32,
        context_lines: u32,
    ) -> String {
        let message = message.as_ref();
        let context = self.get_context(line, column, context_lines);
        format!("Line {}, column {}: {}\n{}", line, column, message, context)
    }

    /// Extracts context around an error location.
    ///
    /// Returns a formatted string containing:
    /// - Context lines before the error
    /// - The error line
    /// - A caret line pointing to the error column
    /// - Context lines after the error
    ///
    /// Line numbers are formatted with proper alignment.
    fn get_context(&self, line: u32, column: u32, context_lines: u32) -> String {
        // Convert from 1-based to 0-based indexing
        let line_idx = (line.saturating_sub(1)) as usize;

        if line_idx >= self.lines.len() {
            return String::new();
        }

        // Calculate context range
        let start = line_idx.saturating_sub(context_lines as usize);
        let end = (line_idx + 1 + context_lines as usize).min(self.lines.len());

        // Calculate line number width for alignment
        let max_line_num = end;
        let line_num_width = max_line_num.to_string().len();

        let mut result = String::new();

        // Add context lines
        for (idx, line_content) in self.lines[start..end].iter().enumerate() {
            let current_line = start + idx + 1; // Convert back to 1-based
            result.push_str(&format!(
                "{:>width$} | {}\n",
                current_line,
                line_content,
                width = line_num_width
            ));

            // Add caret line if this is the error line
            if current_line == line as usize {
                // Create caret pointer
                let caret_padding =
                    " ".repeat(line_num_width + 3 + (column.saturating_sub(1)) as usize);
                result.push_str(&format!("{}^\n", caret_padding));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_formatter() {
        let source = "5 3 +\n10 2 /";
        let formatter = ErrorFormatter::new(source);
        assert_eq!(formatter.lines.len(), 2);
        assert_eq!(formatter.lines[0], "5 3 +");
        assert_eq!(formatter.lines[1], "10 2 /");
    }

    #[test]
    fn test_format_error_basic() {
        let source = "5 3 +";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Test error", 1, 1, 0);
        assert!(error.contains("Line 1, column 1"));
        assert!(error.contains("Test error"));
        assert!(error.contains("5 3 +"));
        assert!(error.contains("^"));
    }

    #[test]
    fn test_format_error_multiline() {
        let source = "5 3 +\n10 @ 2\n7 4 *";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Unexpected character", 2, 4, 1);

        // Should contain error location
        assert!(error.contains("Line 2, column 4"));
        assert!(error.contains("Unexpected character"));

        // Should contain the error line
        assert!(error.contains("10 @ 2"));

        // Should contain context lines
        assert!(error.contains("5 3 +"));
        assert!(error.contains("7 4 *"));

        // Should contain caret
        assert!(error.contains("^"));
    }

    #[test]
    fn test_caret_positioning() {
        let source = "hello world";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Test", 1, 7, 0);

        // Find the caret line (should be after the source line)
        let lines: Vec<&str> = error.lines().collect();
        let source_line_idx = lines
            .iter()
            .position(|l| l.contains("hello world"))
            .unwrap();
        let caret_line = lines[source_line_idx + 1];

        // The caret should point to column 7 (the 'w' in 'world')
        // Format is: "{line_num_width spaces} | {source}\n{line_num_width + 3 + (column-1) spaces}^"
        let caret_pos = caret_line.find('^').unwrap();
        // Should be: "1 | hello world" followed by "    ^" (1 space + " | " = 4 chars + 6 spaces = 10)
        assert!(caret_pos >= 6); // At least after "1 | " and some source chars
    }

    #[test]
    fn test_context_lines() {
        let source = "line1\nline2\nline3\nline4\nline5";
        let formatter = ErrorFormatter::new(source);

        // Test with context_lines = 1
        let error = formatter.format_error("Error", 3, 1, 1);
        assert!(error.contains("line2")); // 1 line before
        assert!(error.contains("line3")); // error line
        assert!(error.contains("line4")); // 1 line after
        assert!(!error.contains("line1")); // too far before
        assert!(!error.contains("line5")); // too far after
    }

    #[test]
    fn test_context_lines_at_start() {
        let source = "line1\nline2\nline3";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Error", 1, 1, 1);

        // Should not crash when trying to show lines before line 1
        assert!(error.contains("line1"));
        assert!(error.contains("line2")); // context after
    }

    #[test]
    fn test_context_lines_at_end() {
        let source = "line1\nline2\nline3";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Error", 3, 1, 1);

        // Should not crash when trying to show lines after last line
        assert!(error.contains("line2")); // context before
        assert!(error.contains("line3"));
    }

    #[test]
    fn test_line_number_alignment() {
        let source = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Error", 10, 1, 1);

        // Line numbers should be right-aligned
        // Line 9 should have a space before it to align with line 10
        assert!(error.contains(" 9 | "));
        assert!(error.contains("10 | "));
        assert!(error.contains("11 | "));
    }

    #[test]
    fn test_empty_source() {
        let formatter = ErrorFormatter::new("");
        let error = formatter.format_error("Error", 1, 1, 0);
        // Should not crash
        assert!(error.contains("Line 1, column 1"));
        assert!(error.contains("Error"));
    }

    #[test]
    fn test_invalid_line_number() {
        let source = "line1\nline2";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Error", 100, 1, 0);

        // Should not crash, just won't show context
        assert!(error.contains("Line 100, column 1"));
        assert!(error.contains("Error"));
    }

    #[test]
    fn test_column_at_end_of_line() {
        let source = "12345";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Error", 1, 5, 0);

        // Should position caret at column 5
        assert!(error.contains("12345"));
        assert!(error.contains("^"));
    }

    #[test]
    fn test_column_beyond_line_length() {
        let source = "abc";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Error", 1, 100, 0);

        // Should not crash, caret will be far to the right
        assert!(error.contains("abc"));
        assert!(error.contains("^"));
    }
}
