package main

import (
	"fmt"
	"unicode"
)

// Lexer tokenizes RPN expression input.
// It tracks position (line and column) for error reporting.
type Lexer struct {
	Text   string
	Pos    int // Current position in Text (0-based)
	Line   int // Current line number (1-based)
	Column int // Current column number (1-based)
}

// NewLexer creates a new Lexer for the given input text.
func NewLexer(text string) *Lexer {
	return &Lexer{
		Text:   text,
		Pos:    0,
		Line:   1,
		Column: 1,
	}
}

// Tokenize scans the entire input and returns a slice of tokens.
// It returns an error if an invalid character is encountered.
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
		Line:   l.Line,
		Column: l.Column,
	})

	return tokens, nil
}

// atEnd returns true if we've reached the end of input.
func (l *Lexer) atEnd() bool {
	return l.Pos >= len(l.Text)
}

// peek returns the current character without consuming it.
// Returns 0 if at end of input.
func (l *Lexer) peek() rune {
	if l.atEnd() {
		return 0
	}
	return rune(l.Text[l.Pos])
}

// peekNext returns the next character without consuming it.
// Returns 0 if at end or only one character remaining.
func (l *Lexer) peekNext() rune {
	if l.Pos+1 >= len(l.Text) {
		return 0
	}
	return rune(l.Text[l.Pos+1])
}

// advance consumes and returns the current character, updating position.
func (l *Lexer) advance() rune {
	if l.atEnd() {
		return 0
	}
	ch := rune(l.Text[l.Pos])
	l.Pos++
	if ch == '\n' {
		l.Line++
		l.Column = 1
	} else {
		l.Column++
	}
	return ch
}

// skipWhitespace consumes all whitespace characters.
func (l *Lexer) skipWhitespace() {
	for !l.atEnd() && unicode.IsSpace(l.peek()) {
		l.advance()
	}
}

// scanToken scans and returns the next token.
func (l *Lexer) scanToken() (Token, error) {
	startLine := l.Line
	startColumn := l.Column
	ch := l.advance()

	switch ch {
	case '+':
		return Token{Type: PLUS, Value: "+", Line: startLine, Column: startColumn}, nil
	case '*':
		return Token{Type: MULT, Value: "*", Line: startLine, Column: startColumn}, nil
	case '/':
		return Token{Type: DIV, Value: "/", Line: startLine, Column: startColumn}, nil
	case '-':
		// Check if this is a negative number or subtraction operator
		// Negative number: '-' followed immediately by a digit
		next := l.peek()
		if unicode.IsDigit(next) {
			return l.scanNumber("-", startLine, startColumn), nil
		}
		return Token{Type: MINUS, Value: "-", Line: startLine, Column: startColumn}, nil
	default:
		if unicode.IsDigit(ch) {
			return l.scanNumber(string(ch), startLine, startColumn), nil
		}
		// Invalid character
		return Token{}, &SyntaxError{
			Message: fmt.Sprintf("Unexpected character '%c'", ch),
			Line:    startLine,
			Column:  startColumn,
		}
	}
}

// scanNumber scans a numeric literal (integer or decimal).
// prefix is the initial character(s) already consumed (e.g., "-" or a digit).
func (l *Lexer) scanNumber(prefix string, startLine, startColumn int) Token {
	value := prefix

	// Scan integer part
	for !l.atEnd() && unicode.IsDigit(l.peek()) {
		value += string(l.advance())
	}

	// Check for decimal point
	if !l.atEnd() && l.peek() == '.' && !l.atEnd() && l.Pos+1 < len(l.Text) && unicode.IsDigit(l.peekNext()) {
		// Consume decimal point
		value += string(l.advance())
		// Scan fractional part
		for !l.atEnd() && unicode.IsDigit(l.peek()) {
			value += string(l.advance())
		}
	}

	return Token{
		Type:   NUMBER,
		Value:  value,
		Line:   startLine,
		Column: startColumn,
	}
}
