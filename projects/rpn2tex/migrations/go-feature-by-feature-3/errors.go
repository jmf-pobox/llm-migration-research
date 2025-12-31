package rpn2tex

import "fmt"

// LexerError represents an error during lexical analysis.
type LexerError struct {
	Message string
	Line    int
	Column  int
}

func (e *LexerError) Error() string {
	return fmt.Sprintf("lexer error at line %d, column %d: %s", e.Line, e.Column, e.Message)
}

// ParserError represents an error during parsing.
type ParserError struct {
	Message string
	Line    int
	Column  int
}

func (e *ParserError) Error() string {
	return fmt.Sprintf("parser error at line %d, column %d: %s", e.Line, e.Column, e.Message)
}
