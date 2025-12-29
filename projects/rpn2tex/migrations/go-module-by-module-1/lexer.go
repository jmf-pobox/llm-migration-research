// Package rpn2tex provides tools for converting Reverse Polish Notation (RPN)
// expressions to LaTeX math mode output.
package rpn2tex

import (
	"fmt"
	"unicode"
)

// Lexer performs lexical analysis on RPN input text, converting it into tokens.
// It tracks position information (line and column) for error reporting.
type Lexer struct {
	text   string
	pos    int
	line   int
	column int
}

// NewLexer creates a new Lexer for the given input text.
// Position tracking starts at line 1, column 1.
func NewLexer(text string) *Lexer {
	return &Lexer{
		text:   text,
		pos:    0,
		line:   1,
		column: 1,
	}
}

// Tokenize converts the input text into a slice of tokens.
// It returns an error if any invalid characters are encountered.
func (l *Lexer) Tokenize() ([]Token, error) {
	var tokens []Token

	for !l.atEnd() {
		l.skipWhitespace()
		if l.atEnd() {
			break
		}

		token, err := l.scanToken()
		if err != nil {
			return nil, err
		}

		tokens = append(tokens, token)
	}

	// Add EOF token
	tokens = append(tokens, Token{
		Type:   EOF,
		Value:  "",
		Line:   l.line,
		Column: l.column,
	})

	return tokens, nil
}

// atEnd returns true if the lexer has reached the end of the input.
func (l *Lexer) atEnd() bool {
	return l.pos >= len(l.text)
}

// peek returns the current character without consuming it.
// Returns 0 if at end of input.
func (l *Lexer) peek() rune {
	if l.atEnd() {
		return 0
	}
	return rune(l.text[l.pos])
}

// advance consumes and returns the current character, updating position tracking.
func (l *Lexer) advance() rune {
	if l.atEnd() {
		return 0
	}

	ch := rune(l.text[l.pos])
	l.pos++

	// Update line and column tracking
	if ch == '\n' {
		l.line++
		l.column = 1
	} else {
		l.column++
	}

	return ch
}

// skipWhitespace consumes all whitespace characters (space, tab, newline, carriage return).
func (l *Lexer) skipWhitespace() {
	for !l.atEnd() {
		ch := l.peek()
		if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
			l.advance()
		} else {
			break
		}
	}
}

// scanToken scans and returns the next token from the input.
func (l *Lexer) scanToken() (Token, error) {
	startLine := l.line
	startColumn := l.column
	ch := l.advance()

	switch ch {
	case '+':
		return Token{Type: PLUS, Value: "+", Line: startLine, Column: startColumn}, nil
	case '*':
		return Token{Type: MULT, Value: "*", Line: startLine, Column: startColumn}, nil
	case '/':
		return Token{Type: DIV, Value: "/", Line: startLine, Column: startColumn}, nil
	case '-':
		// Check if this is a negative number or a subtraction operator
		// If next character is a digit, it's a negative number
		if !l.atEnd() && unicode.IsDigit(l.peek()) {
			return l.scanNumber("-", startLine, startColumn)
		}
		return Token{Type: MINUS, Value: "-", Line: startLine, Column: startColumn}, nil
	default:
		if unicode.IsDigit(ch) {
			return l.scanNumber(string(ch), startLine, startColumn)
		}
		// Unexpected character - return LexerError
		return Token{}, &LexerError{
			Message: fmt.Sprintf("Unexpected character '%c'", ch),
			Line:    startLine,
			Column:  startColumn,
		}
	}
}

// scanNumber scans a numeric token (integer or decimal).
// The prefix parameter contains any already-consumed characters (e.g., '-' for negative numbers).
func (l *Lexer) scanNumber(prefix string, startLine int, startColumn int) (Token, error) {
	value := prefix

	// Consume remaining integer digits
	for !l.atEnd() && unicode.IsDigit(l.peek()) {
		value += string(l.advance())
	}

	// Check for decimal point
	if !l.atEnd() && l.peek() == '.' {
		value += string(l.advance())

		// Consume decimal digits
		for !l.atEnd() && unicode.IsDigit(l.peek()) {
			value += string(l.advance())
		}
	}

	return Token{
		Type:   NUMBER,
		Value:  value,
		Line:   startLine,
		Column: startColumn,
	}, nil
}
