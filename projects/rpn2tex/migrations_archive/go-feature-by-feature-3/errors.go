package rpn2tex

import "fmt"

// LexerError represents an error that occurred during lexing.
type LexerError struct {
	Message string
	Line    int
	Column  int
}

// Error implements the error interface for LexerError.
func (e *LexerError) Error() string {
	return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
}

// ParserError represents an error that occurred during parsing.
type ParserError struct {
	Message string
	Line    int
	Column  int
}

// Error implements the error interface for ParserError.
func (e *ParserError) Error() string {
	return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
}
