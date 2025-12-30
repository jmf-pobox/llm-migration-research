package main

import "fmt"

// LexerError represents an error during lexical analysis
type LexerError struct {
	Message string
	Line    int
	Column  int
}

func (e *LexerError) Error() string {
	return fmt.Sprintf("LexerError at line %d, column %d: %s", e.Line, e.Column, e.Message)
}

// ParserError represents an error during parsing
type ParserError struct {
	Message string
	Token   Token
}

func (e *ParserError) Error() string {
	return fmt.Sprintf("ParserError at line %d, column %d: %s", e.Token.Line, e.Token.Column, e.Message)
}
