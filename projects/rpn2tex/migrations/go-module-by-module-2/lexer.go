package rpn2tex

import (
	"fmt"
	"unicode"
)

// Lexer tokenizes RPN expression text into a stream of tokens.
// It maintains position tracking (line, column) during scanning and
// handles numbers (integers and decimals), operators, and whitespace.
type Lexer struct {
	source []rune // Source text as runes for Unicode-safe handling
	pos    int    // Current position in source (0-based)
	line   int    // Current line number (1-based)
	column int    // Current column number (1-based)
}

// NewLexer creates a new lexer for the given source text.
func NewLexer(source string) *Lexer {
	return &Lexer{
		source: []rune(source),
		pos:    0,
		line:   1,
		column: 1,
	}
}

// Tokenize scans the source text and returns a slice of tokens.
// Returns an error (CompileError) if an unsupported character is encountered.
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

	// Add EOF token at current position
	tokens = append(tokens, Token{
		Type:   EOF,
		Value:  "",
		Line:   l.line,
		Column: l.column,
	})

	return tokens, nil
}

// atEnd returns true if the lexer has reached the end of the source.
func (l *Lexer) atEnd() bool {
	return l.pos >= len(l.source)
}

// peek returns the current character without advancing the position.
// Returns 0 if at end of source.
func (l *Lexer) peek() rune {
	if l.atEnd() {
		return 0
	}
	return l.source[l.pos]
}

// advance returns the current character and advances the position.
// Updates line and column tracking.
func (l *Lexer) advance() rune {
	if l.atEnd() {
		return 0
	}

	ch := l.source[l.pos]
	l.pos++

	if ch == '\n' {
		l.line++
		l.column = 1
	} else {
		l.column++
	}

	return ch
}

// skipWhitespace advances past whitespace characters (space, tab, newline).
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

// scanToken scans and returns the next token.
func (l *Lexer) scanToken() (Token, error) {
	startLine := l.line
	startColumn := l.column

	ch := l.advance()

	switch ch {
	case '+':
		return Token{
			Type:   PLUS,
			Value:  "+",
			Line:   startLine,
			Column: startColumn,
		}, nil

	case '-':
		// Check if this is a negative number or minus operator
		// It's a negative number if followed by a digit
		if !l.atEnd() && unicode.IsDigit(l.peek()) {
			return l.scanNumber("-", startLine, startColumn)
		}
		return Token{
			Type:   MINUS,
			Value:  "-",
			Line:   startLine,
			Column: startColumn,
		}, nil

	case '*':
		return Token{
			Type:   MULT,
			Value:  "*",
			Line:   startLine,
			Column: startColumn,
		}, nil

	case '/':
		return Token{
			Type:   DIV,
			Value:  "/",
			Line:   startLine,
			Column: startColumn,
		}, nil

	default:
		// Check if it's a digit (start of number)
		if unicode.IsDigit(ch) {
			return l.scanNumber(string(ch), startLine, startColumn)
		}

		// Unsupported character - return error
		return Token{}, NewCompileError(
			fmt.Sprintf("Unexpected character '%c'", ch),
			string(l.source),
			startLine,
			startColumn,
		)
	}
}

// scanNumber scans a number (integer or decimal) starting with the given prefix.
// The prefix contains the characters already consumed (e.g., "-" or first digit).
func (l *Lexer) scanNumber(prefix string, startLine, startColumn int) (Token, error) {
	value := prefix

	// Scan integer part
	for !l.atEnd() && unicode.IsDigit(l.peek()) {
		value += string(l.advance())
	}

	// Check for decimal point
	if !l.atEnd() && l.peek() == '.' {
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
	}, nil
}
