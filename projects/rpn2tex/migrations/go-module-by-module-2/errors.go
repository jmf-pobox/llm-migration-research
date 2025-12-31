package rpn2tex

import (
	"fmt"
	"strings"
)

// CompileError represents a compilation error with source context.
// It implements the error interface and provides formatted error messages
// with source line extraction and caret positioning.
type CompileError struct {
	Message string
	Source  string
	Line    int
	Column  int
}

// NewCompileError creates a new CompileError with the given parameters.
func NewCompileError(message, source string, line, column int) *CompileError {
	return &CompileError{
		Message: message,
		Source:  source,
		Line:    line,
		Column:  column,
	}
}

// Error implements the error interface.
// It returns a formatted error message with source context and caret positioning.
func (e *CompileError) Error() string {
	var sb strings.Builder

	// Write the error message with "Error: " prefix
	sb.WriteString("Error: ")
	sb.WriteString(e.Message)
	sb.WriteString("\n\n")

	// Extract and write the source line if available
	// Don't show source context if source is empty
	if e.Source != "" {
		lines := strings.Split(e.Source, "\n")
		if e.Line > 0 && e.Line <= len(lines) {
			lineIdx := e.Line - 1 // Convert 1-based to 0-based
			lineContent := lines[lineIdx]

			// Write the line number and content
			sb.WriteString(fmt.Sprintf("%d | %s\n", e.Line, lineContent))

			// Write the caret pointer
			// The caret should align under the character at the column position
			numWidth := len(fmt.Sprintf("%d", e.Line))
			caretPrefix := strings.Repeat(" ", numWidth) + " | "
			sb.WriteString(caretPrefix)

			// Add spaces before the caret to align with the column
			// Column is 1-based, so we need (column - 1) spaces
			if e.Column > 0 {
				sb.WriteString(strings.Repeat(" ", e.Column-1))
			}
			sb.WriteString("^")
		}
	}

	return sb.String()
}
