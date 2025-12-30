package main

import (
	"fmt"
	"strconv"
	"strings"
)

// SyntaxError represents a parsing or lexing error with position information.
type SyntaxError struct {
	Message string
	Line    int
	Column  int
}

// Error implements the error interface for SyntaxError.
func (e *SyntaxError) Error() string {
	return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
}

// ErrorFormatter formats error messages with source code context.
type ErrorFormatter struct {
	Source string
	Lines  []string
}

// NewErrorFormatter creates a new ErrorFormatter from source text.
func NewErrorFormatter(source string) *ErrorFormatter {
	lines := strings.Split(source, "\n")
	return &ErrorFormatter{
		Source: source,
		Lines:  lines,
	}
}

// FormatError formats an error message with default context (1 line).
func (ef *ErrorFormatter) FormatError(message string, line, column int) string {
	return ef.FormatErrorWithContext(message, line, column, 1)
}

// FormatErrorWithContext formats an error message with surrounding context lines.
// The message includes the error description, source lines with line numbers,
// and a caret (^) pointing to the error position.
func (ef *ErrorFormatter) FormatErrorWithContext(message string, line, column, contextLines int) string {
	context := ef.getContext(line, column, contextLines)
	return fmt.Sprintf("%s\n%s", message, context)
}

// getContext extracts source lines around the error position with line numbers and a caret.
func (ef *ErrorFormatter) getContext(line, column, contextLines int) string {
	errorIdx := line - 1
	startIdx := max(0, errorIdx-contextLines)
	endIdx := min(len(ef.Lines), errorIdx+contextLines+1)
	maxLineNum := endIdx
	numWidth := len(strconv.Itoa(maxLineNum))

	var resultLines []string
	for idx := startIdx; idx < endIdx; idx++ {
		lineNum := idx + 1
		lineContent := ""
		if idx < len(ef.Lines) {
			lineContent = ef.Lines[idx]
		}
		prefix := fmt.Sprintf("%*d | ", numWidth, lineNum)
		resultLines = append(resultLines, prefix+lineContent)

		if idx == errorIdx {
			caretPrefix := strings.Repeat(" ", numWidth) + " | "
			caretPos := max(0, column-1)
			caretLine := caretPrefix + strings.Repeat(" ", caretPos) + "^"
			resultLines = append(resultLines, caretLine)
		}
	}

	return strings.Join(resultLines, "\n")
}

// max returns the maximum of two integers.
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// min returns the minimum of two integers.
func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
