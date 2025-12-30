//! Error formatting with source context.
//!
//! This module provides the `ErrorFormatter` struct for formatting parse and
//! lexer errors with source code context and position markers.

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
pub struct ErrorFormatter {
    #[allow(dead_code)] // Used in tests
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
    /// Returns formatted context showing the source line and a caret
    /// pointing to the error column.
    ///
    /// # Arguments
    ///
    /// * `line` - The line number (1-based) where the error occurred
    /// * `column` - The column number (1-based) where the error occurred
    /// * `context_lines` - Number of context lines to show (currently only 1 is used)
    fn get_context(&self, line: u32, column: u32, context_lines: u32) -> String {
        let _ = context_lines; // Reserved for future use

        // Convert to 0-based index
        let line_index = (line as usize).saturating_sub(1);

        if line_index >= self.lines.len() {
            return String::new();
        }

        let source_line = &self.lines[line_index];

        // Format: "<line_num> | <source_line>"
        let line_display = format!("{line} | {source_line}");

        // Calculate padding for the caret line
        // We need to match the width of the line number + " | " + (column - 1) spaces
        let line_num_width = line.to_string().len();
        let padding_before_caret = line_num_width + 3 + (column as usize).saturating_sub(1);
        let caret_line = format!("{}^", " ".repeat(padding_before_caret));

        format!("{line_display}\n{caret_line}")
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
        assert!(formatted.contains("^"));
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
        // The caret should be at column 3
        assert!(lines[3].trim_start().starts_with("^"));
    }

    #[test]
    fn test_get_context() {
        let source = "5 3 +".to_string();
        let formatter = ErrorFormatter::new(source);
        let context = formatter.get_context(1, 1, 1);

        assert!(context.contains("1 | 5 3 +"));
        assert!(context.contains("^"));
    }

    #[test]
    fn test_get_context_multiline() {
        let source = "5 3 +\n2 3 ^".to_string();
        let formatter = ErrorFormatter::new(source);
        let context = formatter.get_context(2, 5, 1);

        assert!(context.contains("2 | 2 3 ^"));
        assert!(context.contains("^"));
    }

    #[test]
    fn test_caret_position_column_1() {
        let source = "abc".to_string();
        let formatter = ErrorFormatter::new(source);
        let context = formatter.get_context(1, 1, 1);

        let lines: Vec<&str> = context.lines().collect();
        assert_eq!(lines[0], "1 | abc");
        // Caret should be at position 1 (under 'a')
        // "1 | abc" has "1 | " = 4 characters before content
        // Column 1 means 0 spaces before caret
        assert!(lines[1].starts_with("    ^"));
    }

    #[test]
    fn test_caret_position_column_5() {
        let source = "2 3 ^".to_string();
        let formatter = ErrorFormatter::new(source);
        let context = formatter.get_context(1, 5, 1);

        let lines: Vec<&str> = context.lines().collect();
        assert_eq!(lines[0], "1 | 2 3 ^");
        // "1 | " = 4 characters, then 4 more spaces for columns 1-4, then caret
        assert!(lines[1].starts_with("        ^"));
    }

    #[test]
    fn test_format_error_multiline() {
        let source = "5 3 +\n2 3 ^".to_string();
        let formatter = ErrorFormatter::new(source);
        let formatted = formatter.format_error("Unexpected character '^'", 2, 5);

        assert!(formatted.contains("Error: Unexpected character '^'"));
        assert!(formatted.contains("2 | 2 3 ^"));
        assert!(formatted.contains("^"));
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
}
