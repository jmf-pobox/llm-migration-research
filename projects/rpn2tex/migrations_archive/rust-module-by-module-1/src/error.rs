//! Error formatting with source context.
//!
//! This module provides the `ErrorFormatter` struct for formatting parse and
//! lexer errors with source code context and position markers, similar to error
//! output from compilers like gcc and rustc.

/// Formatter for parse and lexer errors with source context.
///
/// Stores the source text and provides formatted error messages with
/// line numbers, source context, and a caret pointing to the error position.
///
/// # Examples
///
/// ```
/// use rpn2tex::error::ErrorFormatter;
///
/// let source = "2 3 ^".to_string();
/// let formatter = ErrorFormatter::new(source);
/// let formatted = formatter.format_error("Unexpected character '^'", 1, 5);
/// assert!(formatted.contains("Error:"));
/// assert!(formatted.contains("2 3 ^"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorFormatter {
    source: String,
    lines: Vec<String>,
}

impl ErrorFormatter {
    /// Creates a new error formatter for the given source text.
    ///
    /// The source is split into lines for context display.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::error::ErrorFormatter;
    ///
    /// let source = "5 3 +\n2 3 ^".to_string();
    /// let formatter = ErrorFormatter::new(source);
    /// ```
    #[must_use]
    pub fn new(source: String) -> Self {
        let lines = source.lines().map(String::from).collect();
        Self { source, lines }
    }

    /// Formats an error message with source context.
    ///
    /// Generates a formatted error message including:
    /// - The error message
    /// - A blank line
    /// - The source line where the error occurred
    /// - A caret (^) pointing to the error column
    ///
    /// # Arguments
    ///
    /// * `message` - The error message to display
    /// * `line` - The line number (1-based) where the error occurred
    /// * `column` - The column number (1-based) where the error occurred
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::error::ErrorFormatter;
    ///
    /// let source = "2 3 ^".to_string();
    /// let formatter = ErrorFormatter::new(source);
    /// let formatted = formatter.format_error("Unexpected character '^'", 1, 5);
    /// assert!(formatted.contains("Error: Unexpected character '^'"));
    /// ```
    #[must_use]
    pub fn format_error(&self, message: &str, line: u32, column: u32) -> String {
        let context = self.get_context(line, column, 1);
        format!("Error: {message}\n\n{context}")
    }

    /// Gets source context for an error position.
    ///
    /// Returns formatted context showing the source line(s) and a caret
    /// pointing to the error column.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number (1-based) where the error occurred
    /// * `column` - The column number (1-based) where the error occurred
    /// * `context_lines` - Number of context lines to show before/after the error line
    fn get_context(&self, line: u32, column: u32, context_lines: u32) -> String {
        // Convert to 0-based index
        let error_idx = line.saturating_sub(1) as usize;

        // Calculate range (clamped to valid indices)
        let start_idx = error_idx.saturating_sub(context_lines as usize);
        let end_idx = (error_idx + context_lines as usize + 1).min(self.lines.len());

        // Calculate line number width for alignment
        let max_line_num = end_idx;
        let num_width = max_line_num.to_string().len();

        let mut result_lines = Vec::new();

        for idx in start_idx..end_idx {
            let line_num = idx + 1; // Convert back to 1-based
            let line_content = self.lines.get(idx).map_or("", String::as_str);

            // Format line with number
            let prefix = format!("{line_num:>num_width$} | ");
            result_lines.push(format!("{prefix}{line_content}"));

            // Add caret on error line
            if idx == error_idx {
                // Spaces for line number column, then position caret
                let caret_prefix = format!("{:num_width$} | ", "");
                // Position caret at column (1-based, so column-1 spaces)
                let caret_pos = column.saturating_sub(1) as usize;
                let caret_line = format!("{caret_prefix}{:caret_pos$}^", "");
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
    fn test_formatter_creation() {
        let source = "5 3 +".to_string();
        let formatter = ErrorFormatter::new(source.clone());
        assert_eq!(formatter.source, source);
        assert_eq!(formatter.lines.len(), 1);
    }

    #[test]
    fn test_multiline_source() {
        let source = "5 3 +\n2 3 ^".to_string();
        let formatter = ErrorFormatter::new(source);
        assert_eq!(formatter.lines.len(), 2);
        assert_eq!(formatter.lines[0], "5 3 +");
        assert_eq!(formatter.lines[1], "2 3 ^");
    }

    #[test]
    fn test_format_error_basic() {
        let source = "2 3 ^".to_string();
        let formatter = ErrorFormatter::new(source);
        let formatted = formatter.format_error("Unexpected character '^'", 1, 5);

        assert!(formatted.contains("Error: Unexpected character '^'"));
        assert!(formatted.contains("1 | 2 3 ^"));
        assert!(formatted.contains('^'));
    }

    #[test]
    fn test_format_error_with_position() {
        let source = "5 3 +".to_string();
        let formatter = ErrorFormatter::new(source);
        let formatted = formatter.format_error("Test error", 1, 3);

        // Check that the caret is at the correct position
        let lines: Vec<&str> = formatted.lines().collect();
        assert!(lines[0].contains("Error: Test error"));
        assert!(lines[2].contains("1 | 5 3 +"));
        // The caret line should be "  |   ^" (2 spaces, |, space, 2 spaces, ^)
        assert_eq!(lines[3], "  |   ^");
    }

    #[test]
    fn test_get_context() {
        let source = "5 3 +".to_string();
        let formatter = ErrorFormatter::new(source);
        let context = formatter.get_context(1, 1, 1);

        assert!(context.contains("1 | 5 3 +"));
        assert!(context.contains('^'));
    }

    #[test]
    fn test_get_context_multiline() {
        let source = "5 3 +\n2 3 ^".to_string();
        let formatter = ErrorFormatter::new(source);
        let context = formatter.get_context(2, 5, 1);

        assert!(context.contains("2 | 2 3 ^"));
        assert!(context.contains('^'));
    }

    #[test]
    fn test_caret_position_column_1() {
        let source = "abc".to_string();
        let formatter = ErrorFormatter::new(source);
        let context = formatter.get_context(1, 1, 1);

        let lines: Vec<&str> = context.lines().collect();
        assert_eq!(lines[0], "1 | abc");
        // Caret line format: "  | ^" (spaces for line num width, " | ", then column-1 spaces, then ^)
        assert_eq!(lines[1], "  | ^");
    }

    #[test]
    fn test_caret_position_column_5() {
        let source = "2 3 ^".to_string();
        let formatter = ErrorFormatter::new(source);
        let context = formatter.get_context(1, 5, 1);

        let lines: Vec<&str> = context.lines().collect();
        assert_eq!(lines[0], "1 | 2 3 ^");
        // Caret line format: "  |     ^" (spaces for line num width, " | ", then column-1 spaces, then ^)
        assert_eq!(lines[1], "  |     ^");
    }

    #[test]
    fn test_format_error_multiline() {
        let source = "5 3 +\n2 3 ^".to_string();
        let formatter = ErrorFormatter::new(source);
        let formatted = formatter.format_error("Unexpected character '^'", 2, 5);

        assert!(formatted.contains("Error: Unexpected character '^'"));
        assert!(formatted.contains("2 | 2 3 ^"));
        assert!(formatted.contains('^'));
    }

    #[test]
    fn test_empty_source() {
        let source = String::new();
        let formatter = ErrorFormatter::new(source);
        let formatted = formatter.format_error("Test error", 1, 1);

        assert!(formatted.contains("Error: Test error"));
    }

    #[test]
    fn test_out_of_bounds_line() {
        let source = "5 3 +".to_string();
        let formatter = ErrorFormatter::new(source);
        let context = formatter.get_context(10, 1, 1);

        // Should return empty string for out-of-bounds line
        assert_eq!(context, "");
    }

    #[test]
    fn test_exact_output_format() {
        // Test the exact output format from the spec
        let source = "2 3 ^".to_string();
        let formatter = ErrorFormatter::new(source);
        let formatted = formatter.format_error("Unexpected character '^'", 1, 5);

        let expected_lines = [
            "Error: Unexpected character '^'",
            "",
            "1 | 2 3 ^",
            "  |     ^",
        ];

        let actual_lines: Vec<&str> = formatted.lines().collect();
        assert_eq!(actual_lines.len(), expected_lines.len());
        for (actual, expected) in actual_lines.iter().zip(expected_lines.iter()) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_multiline_with_context() {
        let source = "line 1\nline 2\nline 3".to_string();
        let formatter = ErrorFormatter::new(source);
        let context = formatter.get_context(2, 3, 1);

        // Should show lines 1, 2, and 3 with caret on line 2
        assert!(context.contains("1 | line 1"));
        assert!(context.contains("2 | line 2"));
        assert!(context.contains("3 | line 3"));
    }

    #[test]
    fn test_line_number_padding() {
        // Test with double-digit line numbers
        let source = (0..10)
            .map(|i| format!("line {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        let formatter = ErrorFormatter::new(source);
        let context = formatter.get_context(10, 1, 1);

        // Line 10 should have proper padding
        assert!(context.contains("10 | line 9"));
    }
}
