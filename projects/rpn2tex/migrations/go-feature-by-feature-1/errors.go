package rpn2tex

import (
	"fmt"
	"strings"
)

// ErrorFormatter provides formatted error messages with source context.
type ErrorFormatter struct {
	Source string
	Lines  []string
}

// NewErrorFormatter creates a new error formatter for the given source.
func NewErrorFormatter(source string) *ErrorFormatter {
	return &ErrorFormatter{
		Source: source,
		Lines:  strings.Split(source, "\n"),
	}
}

// FormatError formats an error message with source context.
func (ef *ErrorFormatter) FormatError(message string, line, column int) string {
	var builder strings.Builder

	builder.WriteString(fmt.Sprintf("Error: %s\n", message))
	builder.WriteString(ef.getContext(line, column, 1))

	return builder.String()
}

// getContext returns the source context with a caret pointing to the error location.
func (ef *ErrorFormatter) getContext(line, column, contextLines int) string {
	var builder strings.Builder

	// Lines are 1-based
	lineIdx := line - 1
	if lineIdx < 0 || lineIdx >= len(ef.Lines) {
		return ""
	}

	// Add the line with the error
	builder.WriteString(fmt.Sprintf("  %d | %s\n", line, ef.Lines[lineIdx]))

	// Add caret pointing to the column
	// Account for line number prefix: "  N | "
	prefix := fmt.Sprintf("  %d | ", line)
	spaces := strings.Repeat(" ", len(prefix)+column-1)
	builder.WriteString(fmt.Sprintf("%s^\n", spaces))

	return builder.String()
}
