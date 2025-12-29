package rpn2tex

import (
	"fmt"
	"unicode"
)

// Lexer performs lexical analysis on the input source.
type Lexer struct {
	source  string
	pos     int
	line    int
	column  int
	linePos int // Position in the current line (for column tracking)
}

// NewLexer creates a new lexer for the given source.
func NewLexer(source string) *Lexer {
	return &Lexer{
		source: source,
		pos:    0,
		line:   1,
		column: 1,
	}
}

// NextToken returns the next token from the input.
func (l *Lexer) NextToken() (Token, error) {
	l.skipWhitespace()

	if l.atEnd() {
		return Token{
			Type:   TokenEOF,
			Value:  "",
			Line:   l.line,
			Column: l.column,
		}, nil
	}

	startLine := l.line
	startColumn := l.column
	char := l.peek()

	// Number
	if unicode.IsDigit(rune(char)) {
		return l.scanNumber("", startLine, startColumn)
	}

	// Plus operator
	if char == '+' {
		l.advance()
		return Token{
			Type:   TokenPlus,
			Value:  "+",
			Line:   startLine,
			Column: startColumn,
		}, nil
	}

	// Minus operator (with negative number handling)
	if char == '-' {
		l.advance()
		// Check if this is a negative number (digit follows immediately)
		if !l.atEnd() && unicode.IsDigit(rune(l.peek())) {
			// It's a negative number
			return l.scanNumber("-", startLine, startColumn)
		}
		// It's a subtraction operator
		return Token{
			Type:   TokenMinus,
			Value:  "-",
			Line:   startLine,
			Column: startColumn,
		}, nil
	}

	// Multiplication operator
	if char == '*' {
		l.advance()
		return Token{
			Type:   TokenTimes,
			Value:  "*",
			Line:   startLine,
			Column: startColumn,
		}, nil
	}

	// Division operator
	if char == '/' {
		l.advance()
		return Token{
			Type:   TokenDivide,
			Value:  "/",
			Line:   startLine,
			Column: startColumn,
		}, nil
	}

	// Unknown character
	l.advance()
	return Token{}, fmt.Errorf("Unexpected character '%c'", char)
}

// scanNumber scans a numeric literal (integer or float).
func (l *Lexer) scanNumber(prefix string, startLine, startColumn int) (Token, error) {
	value := prefix

	// Scan digits before decimal point
	for !l.atEnd() && unicode.IsDigit(rune(l.peek())) {
		value += string(l.advance())
	}

	// Optional decimal point
	if !l.atEnd() && l.peek() == '.' {
		value += string(l.advance())

		// Scan digits after decimal point
		for !l.atEnd() && unicode.IsDigit(rune(l.peek())) {
			value += string(l.advance())
		}
	}

	return Token{
		Type:   TokenNumber,
		Value:  value,
		Line:   startLine,
		Column: startColumn,
	}, nil
}

// skipWhitespace skips whitespace characters.
func (l *Lexer) skipWhitespace() {
	for !l.atEnd() {
		char := l.peek()
		if char == ' ' || char == '\t' || char == '\r' || char == '\n' {
			l.advance()
		} else {
			break
		}
	}
}

// peek returns the current character without advancing.
func (l *Lexer) peek() byte {
	if l.atEnd() {
		return 0
	}
	return l.source[l.pos]
}

// advance moves to the next character and returns the current one.
func (l *Lexer) advance() byte {
	if l.atEnd() {
		return 0
	}

	char := l.source[l.pos]
	l.pos++

	if char == '\n' {
		l.line++
		l.column = 1
	} else {
		l.column++
	}

	return char
}

// atEnd returns true if the lexer has reached the end of the source.
func (l *Lexer) atEnd() bool {
	return l.pos >= len(l.source)
}
