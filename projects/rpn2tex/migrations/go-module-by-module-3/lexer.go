package rpn2tex

import (
	"fmt"
	"strings"
	"unicode"
)

// LexerError represents an error that occurs during lexical analysis.
type LexerError struct {
	Message string
	Line    int
	Column  int
}

// Error implements the error interface for LexerError.
func (e *LexerError) Error() string {
	return fmt.Sprintf("Line %d, column %d: %s", e.Line, e.Column, e.Message)
}

// Lexer performs lexical analysis on RPN input text.
type Lexer struct {
	text   string // Source text to tokenize
	pos    int    // Current position in text
	line   int    // Current line number (1-based)
	column int    // Current column number (1-based)
}

// NewLexer creates a new Lexer for the given input text.
func NewLexer(text string) *Lexer {
	return &Lexer{
		text:   text,
		pos:    0,
		line:   1,
		column: 1,
	}
}

// Tokenize scans the input text and returns a slice of all tokens.
// Returns an error if an unexpected character is encountered.
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

	// Add EOF token at the end
	tokens = append(tokens, Token{
		Type:   EOF,
		Value:  "",
		Line:   l.line,
		Column: l.column,
	})

	return tokens, nil
}

// atEnd checks if we've reached the end of the input.
func (l *Lexer) atEnd() bool {
	return l.pos >= len(l.text)
}

// peek returns the current character without advancing.
func (l *Lexer) peek() rune {
	if l.atEnd() {
		return 0
	}
	return rune(l.text[l.pos])
}

// advance consumes the current character and returns it.
// Updates line and column tracking.
func (l *Lexer) advance() rune {
	if l.atEnd() {
		return 0
	}

	ch := rune(l.text[l.pos])
	l.pos++

	if ch == '\n' {
		l.line++
		l.column = 1
	} else {
		l.column++
	}

	return ch
}

// skipWhitespace skips over whitespace characters.
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

// scanToken scans a single token from the current position.
func (l *Lexer) scanToken() (Token, error) {
	startLine := l.line
	startColumn := l.column

	ch := l.advance()

	// Check for operators
	switch ch {
	case '+':
		return Token{Type: PLUS, Value: "+", Line: startLine, Column: startColumn}, nil
	case '*':
		return Token{Type: MULT, Value: "*", Line: startLine, Column: startColumn}, nil
	case '/':
		return Token{Type: DIV, Value: "/", Line: startLine, Column: startColumn}, nil
	case '-':
		// Check if this is a negative number or subtraction operator
		// Negative number: '-' followed immediately by a digit (no space)
		if !l.atEnd() && unicode.IsDigit(l.peek()) {
			return l.scanNumber("-", startLine, startColumn)
		}
		return Token{Type: MINUS, Value: "-", Line: startLine, Column: startColumn}, nil
	}

	// Check for numbers
	if unicode.IsDigit(ch) {
		return l.scanNumber(string(ch), startLine, startColumn)
	}

	// Unsupported character
	return Token{}, &LexerError{
		Message: fmt.Sprintf("Unexpected character '%c'", ch),
		Line:    startLine,
		Column:  startColumn,
	}
}

// scanNumber scans a numeric literal (integer or decimal).
// prefix contains the already-consumed first character(s).
func (l *Lexer) scanNumber(prefix string, startLine, startColumn int) (Token, error) {
	var sb strings.Builder
	sb.WriteString(prefix)

	// Scan digits
	for !l.atEnd() {
		ch := l.peek()
		if unicode.IsDigit(ch) {
			sb.WriteRune(ch)
			l.advance()
		} else if ch == '.' {
			// Check if we've already seen a decimal point
			if strings.Contains(sb.String(), ".") {
				break
			}
			sb.WriteRune(ch)
			l.advance()
		} else {
			break
		}
	}

	return Token{
		Type:   NUMBER,
		Value:  sb.String(),
		Line:   startLine,
		Column: startColumn,
	}, nil
}
