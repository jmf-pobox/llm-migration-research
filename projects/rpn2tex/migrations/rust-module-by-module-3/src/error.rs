//! Error formatting for rpn2tex - provides context-aware error messages.
//!
//! This module formats parse and lexer errors with source context,
//! similar to error output from compilers like gcc and rustc.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::error::ErrorFormatter;
//!
//! let formatter = ErrorFormatter::new("5 3 @");
//! let error = formatter.format_error("Unexpected character '@'", 1, 5);
//! assert!(error.contains("Error: Unexpected character '@'"));
//! assert!(error.contains("^"));
//! ```

/// Formats parse errors with source context and helpful hints.
///
/// Provides gcc/rustc-style error output with:
/// - Line numbers and source context
/// - Caret (^) pointing to error column
/// - Clear error messages
///
/// # Examples
///
/// ```
/// use rpn2tex::error::ErrorFormatter;
///
/// let formatter = ErrorFormatter::new("5 3 + @ 2");
/// let error = formatter.format_error("Unexpected '@'", 1, 7);
/// assert!(error.contains("^"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorFormatter {
    /// The complete source text being parsed
    source: String,
    /// Source text split into lines
    lines: Vec<String>,
}

impl ErrorFormatter {
    /// Initialize formatter with source text.
    ///
    /// # Arguments
    ///
    /// * `source` - The complete source text being parsed
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::error::ErrorFormatter;
    ///
    /// let formatter = ErrorFormatter::new("5 3 +");
    /// ```
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let lines = source.lines().map(String::from).collect();
        Self { source, lines }
    }

    /// Format an error with source context.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    ///
    /// # Returns
    ///
    /// Formatted error string with context
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::error::ErrorFormatter;
    ///
    /// let formatter = ErrorFormatter::new("5 3 @");
    /// let error = formatter.format_error("Unexpected character '@'", 1, 5);
    /// assert!(error.contains("Error: Unexpected character '@'"));
    /// assert!(error.contains("1 | 5 3 @"));
    /// assert!(error.contains("^"));
    /// ```
    #[must_use]
    pub fn format_error(&self, message: &str, line: usize, column: usize) -> String {
        self.format_error_with_context(message, line, column, 1)
    }

    /// Format an error with source context and configurable context lines.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    /// * `context_lines` - Number of lines to show before/after error
    ///
    /// # Returns
    ///
    /// Formatted error string with context
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::error::ErrorFormatter;
    ///
    /// let formatter = ErrorFormatter::new("line1\nline2\nline3");
    /// let error = formatter.format_error_with_context("Error on line 2", 2, 1, 1);
    /// assert!(error.contains("line1"));
    /// assert!(error.contains("line2"));
    /// assert!(error.contains("line3"));
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
        parts.push(String::new());

        // Source context
        let context = self.get_context(line, column, context_lines);
        parts.push(context);

        parts.join("\n")
    }

    /// Extract source context around error position.
    ///
    /// # Arguments
    ///
    /// * `line` - Error line number (1-based)
    /// * `column` - Error column number (1-based)
    /// * `context_lines` - Lines to show before/after
    ///
    /// # Returns
    ///
    /// Formatted context with line numbers and caret
    fn get_context(&self, line: usize, column: usize, context_lines: usize) -> String {
        // Convert to 0-based index
        let error_idx = line.saturating_sub(1);

        // Calculate range (clamped to valid indices)
        let start_idx = error_idx.saturating_sub(context_lines);
        let end_idx = (error_idx + context_lines + 1).min(self.lines.len());

        // Calculate line number width for alignment
        let max_line_num = end_idx;
        let num_width = max_line_num.to_string().len();

        let mut result_lines = Vec::new();

        for idx in start_idx..end_idx {
            let line_num = idx + 1; // Convert back to 1-based
            let line_content = self.lines.get(idx).map_or("", String::as_str);

            // Format line with number
            let prefix = format!("{:>width$} | ", line_num, width = num_width);
            result_lines.push(format!("{prefix}{line_content}"));

            // Add caret on error line
            if idx == error_idx {
                // Spaces for line number column, then position caret
                let caret_prefix = format!("{:>width$} | ", "", width = num_width);
                // Position caret at column (1-based, so column-1 spaces)
                let caret_pos = column.saturating_sub(1);
                let caret_line = format!("{}{:width$}^", caret_prefix, "", width = caret_pos);
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
    fn test_new() {
        let formatter = ErrorFormatter::new("5 3 +");
        assert_eq!(formatter.source, "5 3 +");
        assert_eq!(formatter.lines, vec!["5 3 +"]);
    }

    #[test]
    fn test_new_multiline() {
        let formatter = ErrorFormatter::new("line1\nline2\nline3");
        assert_eq!(formatter.lines.len(), 3);
        assert_eq!(formatter.lines[0], "line1");
        assert_eq!(formatter.lines[1], "line2");
        assert_eq!(formatter.lines[2], "line3");
    }

    #[test]
    fn test_format_error_basic() {
        let formatter = ErrorFormatter::new("5 3 @");
        let error = formatter.format_error("Unexpected character '@'", 1, 5);

        assert!(error.contains("Error: Unexpected character '@'"));
        assert!(error.contains("1 | 5 3 @"));
        assert!(error.contains("^"));
    }

    #[test]
    fn test_format_error_caret_position() {
        let formatter = ErrorFormatter::new("5 3 @");
        let error = formatter.format_error("Unexpected character '@'", 1, 5);

        // The caret should be at column 5 (1-based)
        // Line format: "1 | 5 3 @"
        // Caret line: "  |     ^"
        let lines: Vec<&str> = error.lines().collect();
        assert!(lines.len() >= 3);
        assert_eq!(lines[0], "Error: Unexpected character '@'");
        assert_eq!(lines[1], "");
        assert_eq!(lines[2], "1 | 5 3 @");
        assert_eq!(lines[3], "  |     ^");
    }

    #[test]
    fn test_format_error_multiline_context() {
        let source = "line1\nline2 error\nline3";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error_with_context("Test error", 2, 7, 1);

        assert!(error.contains("line1"));
        assert!(error.contains("line2 error"));
        assert!(error.contains("line3"));
        assert!(error.contains("^"));
    }

    #[test]
    fn test_format_error_first_line() {
        let source = "error\nline2\nline3";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Test error", 1, 1);

        assert!(error.contains("1 | error"));
        assert!(error.contains("2 | line2"));
        assert!(error.contains("^"));
    }

    #[test]
    fn test_format_error_last_line() {
        let source = "line1\nline2\nerror";
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Test error", 3, 1);

        assert!(error.contains("2 | line2"));
        assert!(error.contains("3 | error"));
        assert!(error.contains("^"));
    }

    #[test]
    fn test_format_error_column_at_beginning() {
        let formatter = ErrorFormatter::new("@bc");
        let error = formatter.format_error("Error at start", 1, 1);

        let lines: Vec<&str> = error.lines().collect();
        assert_eq!(lines[2], "1 | @bc");
        assert_eq!(lines[3], "  | ^");
    }

    #[test]
    fn test_format_error_column_in_middle() {
        let formatter = ErrorFormatter::new("abc@def");
        let error = formatter.format_error("Error in middle", 1, 4);

        let lines: Vec<&str> = error.lines().collect();
        assert_eq!(lines[2], "1 | abc@def");
        assert_eq!(lines[3], "  |    ^");
    }

    #[test]
    fn test_format_error_large_line_numbers() {
        let mut lines = Vec::new();
        for i in 1..=100 {
            lines.push(format!("line{i}"));
        }
        let source = lines.join("\n");
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Error", 100, 1);

        // Line number 100 should be right-aligned with width 3
        assert!(error.contains(" 99 | line99"));
        assert!(error.contains("100 | line100"));
    }

    #[test]
    fn test_get_context_clamps_boundaries() {
        let formatter = ErrorFormatter::new("single line");
        let context = formatter.get_context(1, 1, 10);

        // Should not panic and should handle large context_lines gracefully
        assert!(context.contains("1 | single line"));
    }

    #[test]
    fn test_format_error_empty_source() {
        let formatter = ErrorFormatter::new("");
        let error = formatter.format_error("Error", 1, 1);

        // Should handle gracefully without panicking
        assert!(error.contains("Error: Error"));
    }

    #[test]
    fn test_format_error_preserves_exact_spacing() {
        let formatter = ErrorFormatter::new("5   3   @");
        let error = formatter.format_error("Error", 1, 9);

        // Should preserve the exact spacing from the source
        assert!(error.contains("1 | 5   3   @"));
    }

    #[test]
    fn test_format_error_with_tabs() {
        let formatter = ErrorFormatter::new("5\t3\t@");
        let error = formatter.format_error("Error", 1, 5);

        // Should preserve tabs as-is
        assert!(error.contains("5\t3\t@"));
    }

    #[test]
    fn test_context_lines_parameter() {
        let source = "line1\nline2\nline3\nline4\nline5";
        let formatter = ErrorFormatter::new(source);

        // Test with 0 context lines
        let error = formatter.format_error_with_context("Error", 3, 1, 0);
        assert!(error.contains("3 | line3"));
        assert!(!error.contains("line2"));
        assert!(!error.contains("line4"));

        // Test with 2 context lines
        let error = formatter.format_error_with_context("Error", 3, 1, 2);
        assert!(error.contains("1 | line1"));
        assert!(error.contains("2 | line2"));
        assert!(error.contains("3 | line3"));
        assert!(error.contains("4 | line4"));
        assert!(error.contains("5 | line5"));
    }
}
