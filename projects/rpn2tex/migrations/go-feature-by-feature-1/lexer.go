package main

import "unicode"

// Lexer tokenizes input text for RPN expressions
type Lexer struct {
	text   string
	pos    int
	line   int
	column int
}

// NewLexer creates a new lexer for the given input text
func NewLexer(text string) *Lexer {
	return &Lexer{
		text:   text,
		pos:    0,
		line:   1,
		column: 1,
	}
}

// Tokenize returns all tokens from the input
func (l *Lexer) Tokenize() ([]Token, error) {
	var tokens []Token
	for !l.atEnd() {
		token, err := l.nextToken()
		if err != nil {
			return nil, err
		}
		if token != nil {
			tokens = append(tokens, *token)
		}
	}
	return tokens, nil
}

// nextToken returns the next token from the input
func (l *Lexer) nextToken() (*Token, error) {
	// Skip whitespace
	l.skipWhitespace()

	if l.atEnd() {
		return nil, nil
	}

	ch := l.peek()
	startLine := l.line
	startColumn := l.column

	// Check for numbers
	if unicode.IsDigit(ch) {
		return l.scanNumber("", startLine, startColumn), nil
	}

	// Check for minus sign (could be negative number or subtraction operator)
	if ch == '-' {
		l.advance()
		// Check if next character is a digit (negative number)
		if !l.atEnd() && unicode.IsDigit(l.peek()) {
			return l.scanNumber("-", startLine, startColumn), nil
		}
		// Otherwise it's subtraction operator
		return &Token{
			Type:   MINUS,
			Value:  "-",
			Line:   startLine,
			Column: startColumn,
		}, nil
	}

	// Check for operators
	if ch == '+' {
		l.advance()
		return &Token{
			Type:   PLUS,
			Value:  "+",
			Line:   startLine,
			Column: startColumn,
		}, nil
	}

	if ch == '*' {
		l.advance()
		return &Token{
			Type:   MULTIPLY,
			Value:  "*",
			Line:   startLine,
			Column: startColumn,
		}, nil
	}

	if ch == '/' {
		l.advance()
		return &Token{
			Type:   DIVIDE,
			Value:  "/",
			Line:   startLine,
			Column: startColumn,
		}, nil
	}

	// Unknown character
	return nil, &LexerError{
		Message: "Unexpected character '" + string(ch) + "'",
		Line:    startLine,
		Column:  startColumn,
	}
}

// scanNumber scans a numeric literal
func (l *Lexer) scanNumber(prefix string, startLine, startColumn int) *Token {
	value := prefix

	// Scan integer part
	for !l.atEnd() && unicode.IsDigit(l.peek()) {
		value += string(l.advance())
	}

	// Scan decimal part if present
	if !l.atEnd() && l.peek() == '.' {
		value += string(l.advance())
		for !l.atEnd() && unicode.IsDigit(l.peek()) {
			value += string(l.advance())
		}
	}

	return &Token{
		Type:   NUMBER,
		Value:  value,
		Line:   startLine,
		Column: startColumn,
	}
}

// skipWhitespace skips whitespace characters
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

// atEnd checks if we're at the end of input
func (l *Lexer) atEnd() bool {
	return l.pos >= len(l.text)
}

// peek returns the current character without consuming it
func (l *Lexer) peek() rune {
	if l.atEnd() {
		return 0
	}
	return rune(l.text[l.pos])
}

// advance consumes and returns the current character
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
