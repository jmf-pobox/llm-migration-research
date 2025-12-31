// Package rpn2tex provides error formatting utilities for the rpn2tex converter.
package rpn2tex

import (
	"fmt"
	"strconv"
	"strings"
)

// ErrorFormatter formats errors with source context, displaying line numbers
// and a caret pointing to the error location.
type ErrorFormatter struct {
	Source string
	Lines  []string
}

// NewErrorFormatter creates a new ErrorFormatter for the given source code.
// The source is split into lines for context display.
func NewErrorFormatter(source string) *ErrorFormatter {
	return &ErrorFormatter{
		Source: source,
		Lines:  strings.Split(source, "\n"),
	}
}

// FormatError formats an error message with source context.
// It displays the error message, the source line where the error occurred,
// and a caret (^) pointing to the specific column.
//
// Parameters:
//   - message: The error message to display
//   - line: The line number (1-based) where the error occurred
//   - column: The column number (1-based) where the error occurred
//   - contextLines: Number of context lines to show (default: 1)
//
// Returns a formatted error string with context.
func (f *ErrorFormatter) FormatError(message string, line, column, contextLines int) string {
	// Default contextLines to 1 if 0 is passed
	if contextLines == 0 {
		contextLines = 1
	}

	context := f.getContext(line, column, contextLines)
	return fmt.Sprintf("Error: %s\n\n%s", message, context)
}

// getContext builds the context display with line numbers and caret.
// This is the private implementation that handles the actual formatting.
func (f *ErrorFormatter) getContext(line, column, contextLines int) string {
	// Clamp context range to valid line indices
	// Context lines are shown before the error line, not after
	startLine := max(1, line-contextLines)
	endLine := line

	// Calculate width for line number column alignment
	numWidth := len(strconv.Itoa(endLine))

	var result strings.Builder

	for i := startLine; i <= endLine; i++ {
		lineNum := i
		lineContent := ""
		if i-1 < len(f.Lines) {
			lineContent = f.Lines[i-1]
		}

		// Format line number and content
		prefix := fmt.Sprintf("%*d | ", numWidth, lineNum)
		result.WriteString(prefix)
		result.WriteString(lineContent)
		result.WriteString("\n")

		// Add caret on the error line
		if i == line {
			// Calculate spaces needed: width of line number + " | " + (column - 1)
			spaces := numWidth + 3 + (column - 1)
			caret := strings.Repeat(" ", spaces) + "^"
			result.WriteString(caret)
		}
	}

	return result.String()
}

// max returns the larger of two integers.
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// min returns the smaller of two integers.
func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
