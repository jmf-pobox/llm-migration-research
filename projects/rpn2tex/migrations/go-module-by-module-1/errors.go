// Package rpn2tex provides utilities for converting Reverse Polish Notation (RPN)
// expressions to LaTeX format.
package rpn2tex

import (
	"fmt"
	"strings"
)

// LexerError represents an error that occurs during lexical analysis.
// It implements the error interface and includes position information
// for detailed error reporting.
type LexerError struct {
	Message string
	Line    int
	Column  int
}

// Error returns the string representation of the LexerError.
// It implements the error interface.
func (e *LexerError) Error() string {
	return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
}

// ErrorFormatter formats error messages with source code context.
type ErrorFormatter struct {
	source string
	lines  []string
}

// NewErrorFormatter creates a new ErrorFormatter for the given source text.
func NewErrorFormatter(source string) *ErrorFormatter {
	lines := strings.Split(source, "\n")
	return &ErrorFormatter{
		source: source,
		lines:  lines,
	}
}

// FormatError formats an error message with source context showing the error location.
// The contextLines parameter controls how many lines of context to show before and after
// the error line (default behavior is 1).
func (ef *ErrorFormatter) FormatError(message string, line int, column int, contextLines int) string {
	var result strings.Builder

	// Write the error message header
	result.WriteString(message)
	result.WriteString("\n\n")

	// Get the context
	context := ef.getContext(line, column, contextLines)
	result.WriteString(context)

	return result.String()
}

// getContext generates the source context showing the error location.
// It displays the line number, source line, and a caret pointing to the error column.
func (ef *ErrorFormatter) getContext(line int, column int, contextLines int) string {
	var result strings.Builder

	// For now, we only show the single line with the error (contextLines is ignored)
	// This matches the Python implementation's actual behavior in the I/O contract

	// Calculate line number width for alignment
	numWidth := len(fmt.Sprintf("%d", line))

	// Get the source line (1-based indexing)
	if line < 1 || line > len(ef.lines) {
		return ""
	}
	sourceLine := ef.lines[line-1]

	// Format the line with line number
	linePrefix := fmt.Sprintf("%*d | ", numWidth, line)
	result.WriteString(linePrefix)
	result.WriteString(sourceLine)
	result.WriteString("\n")

	// Format the caret line
	// The caret prefix has spaces instead of the line number
	caretPrefix := strings.Repeat(" ", numWidth) + " | "
	result.WriteString(caretPrefix)

	// Position the caret (column is 1-based, so we need column-1 spaces)
	caretPos := column - 1
	result.WriteString(strings.Repeat(" ", caretPos))
	result.WriteString("^")

	return result.String()
}
