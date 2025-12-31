package rpn2tex

import "fmt"

// LexerError represents an error during lexical analysis.
type LexerError struct {
	Message string
	Line    int
	Column  int
}

// Error implements the error interface.
func (e *LexerError) Error() string {
	return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
}

// ParserError represents an error during parsing.
type ParserError struct {
	Message string
	Token   Token
}

// Error implements the error interface.
func (e *ParserError) Error() string {
	return fmt.Sprintf("Line %d, column %d: %s", e.Token.Line, e.Token.Column, e.Message)
}
