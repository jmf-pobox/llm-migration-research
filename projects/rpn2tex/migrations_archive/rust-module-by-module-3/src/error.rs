//! Error formatting for rpn2tex - provides context-aware error messages.
//!
//! This module formats parse and lexer errors with source context,
//! similar to error output from compilers like gcc and rustc.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::ErrorFormatter;
//!
//! let formatter = ErrorFormatter::new("5 3 @".to_string());
//! let error = formatter.format_error("Unexpected character '@'", 1, 5, 1);
//! println!("{}", error);
//! // Output:
//! // Error: Unexpected character '@'
//! //
//! // 1 | 5 3 @
//! //       ^
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
/// use rpn2tex::ErrorFormatter;
///
/// let formatter = ErrorFormatter::new("5 3 + @ 2".to_string());
/// let error = formatter.format_error("Unexpected '@'", 1, 7, 1);
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
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ErrorFormatter;
    ///
    /// let formatter = ErrorFormatter::new("5 3 +".to_string());
    /// ```
    #[must_use]
    pub fn new(source: String) -> Self {
        let lines = source
            .lines()
            .map(std::string::ToString::to_string)
            .collect();
        Self { source, lines }
    }

    /// Format an error with source context.
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
    /// use rpn2tex::ErrorFormatter;
    ///
    /// let formatter = ErrorFormatter::new("5 3 + @ 2".to_string());
    /// let err = formatter.format_error("Unexpected '@'", 1, 7, 1);
    /// assert!(err.contains("^"));
    /// assert!(err.contains("Error: Unexpected '@'"));
    /// ```
    #[must_use]
    pub fn format_error(
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
        let num_width = Self::calculate_width(max_line_num);

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
                let caret_pos = column.saturating_sub(1);
                let caret_line = format!("{caret_prefix}{:caret_pos$}^", "");
                result_lines.push(caret_line);
            }
        }

        result_lines.join("\n")
    }

    /// Calculate the width needed to display a line number.
    ///
    /// # Arguments
    ///
    /// * `line_num` - The line number to calculate width for
    ///
    /// # Returns
    ///
    /// The number of digits needed to display the line number
    fn calculate_width(line_num: usize) -> usize {
        if line_num == 0 {
            1
        } else {
            (line_num as f64).log10().floor() as usize + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_formatter() {
        let formatter = ErrorFormatter::new("5 3 +".to_string());
        assert_eq!(formatter.source, "5 3 +");
        assert_eq!(formatter.lines, vec!["5 3 +"]);
    }

    #[test]
    fn test_new_formatter_multiline() {
        let formatter = ErrorFormatter::new("5 3 +\n2 4 *".to_string());
        assert_eq!(formatter.lines, vec!["5 3 +", "2 4 *"]);
    }

    #[test]
    fn test_format_error_simple() {
        let formatter = ErrorFormatter::new("5 3 @".to_string());
        let error = formatter.format_error("Unexpected character '@'", 1, 5, 1);

        assert!(error.contains("Error: Unexpected character '@'"));
        assert!(error.contains("1 | 5 3 @"));
        assert!(error.contains("^"));
    }

    #[test]
    fn test_format_error_with_context() {
        let source = "5 3 +\n2 4 @\n1 2 *".to_string();
        let formatter = ErrorFormatter::new(source);
        let error = formatter.format_error("Unexpected '@'", 2, 5, 1);

        assert!(error.contains("Error: Unexpected '@'"));
        assert!(error.contains("1 | 5 3 +"));
        assert!(error.contains("2 | 2 4 @"));
        assert!(error.contains("3 | 1 2 *"));
        assert!(error.contains("^"));
    }

    #[test]
    fn test_caret_positioning() {
        let formatter = ErrorFormatter::new("5 3 + @ 2".to_string());
        let error = formatter.format_error("Unexpected '@'", 1, 7, 0);

        let lines: Vec<&str> = error.lines().collect();
        // Find the caret line (should be after the source line)
        let caret_line = lines.iter().find(|line| line.contains('^')).unwrap();

        // Count spaces before caret - should be at column 7 (6 spaces + caret)
        let caret_pos = caret_line.find('^').unwrap();
        let prefix_len = "1 | ".len();
        let spaces_before_caret = caret_pos - prefix_len;
        assert_eq!(spaces_before_caret, 6); // column 7 - 1 = 6 spaces
    }

    #[test]
    fn test_line_number_width() {
        let source = (1..=100)
            .map(|i| format!("line {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        let formatter = ErrorFormatter::new(source);

        // With context_lines=51, we show lines up to 100+, requiring 3-digit width
        let error = formatter.format_error("Error at line 50", 50, 1, 51);

        // Line numbers should be right-aligned with 3-digit width
        assert!(error.contains(" 50 | line 50"));
        assert!(error.contains("100 | line 100"));
    }

    #[test]
    fn test_edge_case_empty_source() {
        let formatter = ErrorFormatter::new(String::new());
        let error = formatter.format_error("Error", 1, 1, 1);

        assert!(error.contains("Error: Error"));
    }

    #[test]
    fn test_edge_case_error_beyond_file() {
        let formatter = ErrorFormatter::new("5 3 +".to_string());
        let error = formatter.format_error("Error beyond file", 10, 1, 1);

        assert!(error.contains("Error: Error beyond file"));
        // Should handle gracefully without panic
    }

    #[test]
    fn test_edge_case_column_beyond_line() {
        let formatter = ErrorFormatter::new("5 3".to_string());
        let error = formatter.format_error("Error beyond line", 1, 100, 0);

        assert!(error.contains("Error: Error beyond line"));
        assert!(error.contains("^"));
        // Should handle gracefully without panic
    }

    #[test]
    fn test_calculate_width() {
        assert_eq!(ErrorFormatter::calculate_width(0), 1);
        assert_eq!(ErrorFormatter::calculate_width(1), 1);
        assert_eq!(ErrorFormatter::calculate_width(9), 1);
        assert_eq!(ErrorFormatter::calculate_width(10), 2);
        assert_eq!(ErrorFormatter::calculate_width(99), 2);
        assert_eq!(ErrorFormatter::calculate_width(100), 3);
        assert_eq!(ErrorFormatter::calculate_width(999), 3);
        assert_eq!(ErrorFormatter::calculate_width(1000), 4);
    }

    #[test]
    fn test_context_lines_boundary() {
        let source = "line1\nline2\nline3\nline4\nline5".to_string();
        let formatter = ErrorFormatter::new(source);

        // Error at line 3 with 1 context line should show lines 2-4
        let error = formatter.format_error("Error", 3, 1, 1);
        assert!(error.contains("2 | line2"));
        assert!(error.contains("3 | line3"));
        assert!(error.contains("4 | line4"));
        assert!(!error.contains("1 | line1"));
        assert!(!error.contains("5 | line5"));
    }

    #[test]
    fn test_context_lines_at_start() {
        let source = "line1\nline2\nline3".to_string();
        let formatter = ErrorFormatter::new(source);

        // Error at line 1 with context shouldn't go before start
        let error = formatter.format_error("Error", 1, 1, 2);
        assert!(error.contains("1 | line1"));
        assert!(error.contains("2 | line2"));
        assert!(error.contains("3 | line3"));
    }

    #[test]
    fn test_context_lines_at_end() {
        let source = "line1\nline2\nline3".to_string();
        let formatter = ErrorFormatter::new(source);

        // Error at line 3 with context shouldn't go past end
        let error = formatter.format_error("Error", 3, 1, 2);
        assert!(error.contains("1 | line1"));
        assert!(error.contains("2 | line2"));
        assert!(error.contains("3 | line3"));
    }
}

#[cfg(test)]
mod python_comparison {
    use super::*;

    #[test]
    fn test_python_case_1() {
        let formatter = ErrorFormatter::new("2 3 ^".to_string());
        let error = formatter.format_error("Unexpected character '^'", 1, 5, 0);
        let expected = "Error: Unexpected character '^'\n\n1 | 2 3 ^\n  |     ^";
        assert_eq!(error, expected, "Test 1 mismatch!\nGot:\n{:?}\n\nExpected:\n{:?}", error, expected);
    }

    #[test]
    fn test_python_case_3() {
        let formatter = ErrorFormatter::new("5 3 + @ 2".to_string());
        let error = formatter.format_error("Unexpected '@'", 1, 7, 0);
        let expected = "Error: Unexpected '@'\n\n1 | 5 3 + @ 2\n  |       ^";
        assert_eq!(error, expected, "Test 3 mismatch!\nGot:\n{:?}\n\nExpected:\n{:?}", error, expected);
    }

    #[test]
    fn test_python_case_multiline() {
        let formatter = ErrorFormatter::new("5 3 +\n2 4 @\n1 2 *".to_string());
        let error = formatter.format_error("Unexpected '@'", 2, 5, 1);
        let expected = "Error: Unexpected '@'\n\n1 | 5 3 +\n2 | 2 4 @\n  |     ^\n3 | 1 2 *";
        assert_eq!(error, expected, "Multiline test mismatch!\nGot:\n{:?}\n\nExpected:\n{:?}", error, expected);
    }
}

#[cfg(test)]
mod context_zero_test {
    use super::*;

    #[test]
    fn test_context_zero_matches_python() {
        let formatter = ErrorFormatter::new("5 3 +\nline 2\nline 3".to_string());
        let error = formatter.format_error("Error", 2, 1, 0);
        let expected = "Error: Error\n\n2 | line 2\n  | ^";
        assert_eq!(error, expected, "Context 0 should match Python behavior");
    }
}
