package rpn2tex

import "unicode"

// Lexer performs lexical analysis on input text.
type Lexer struct {
	text   string
	pos    int
	line   int
	column int
}

// NewLexer creates a new lexer for the given input text.
func NewLexer(text string) *Lexer {
	return &Lexer{
		text:   text,
		pos:    0,
		line:   1,
		column: 1,
	}
}

// Tokenize converts the input text into a slice of tokens.
func (l *Lexer) Tokenize() ([]Token, error) {
	var tokens []Token

	for !l.isAtEnd() {
		l.skipWhitespace()
		if l.isAtEnd() {
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
		Type:   TokenEOF,
		Value:  "",
		Line:   l.line,
		Column: l.column,
	})

	return tokens, nil
}

func (l *Lexer) isAtEnd() bool {
	return l.pos >= len(l.text)
}

func (l *Lexer) peek() rune {
	if l.isAtEnd() {
		return 0
	}
	return rune(l.text[l.pos])
}

func (l *Lexer) advance() rune {
	if l.isAtEnd() {
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

func (l *Lexer) skipWhitespace() {
	for !l.isAtEnd() && unicode.IsSpace(l.peek()) {
		l.advance()
	}
}

func (l *Lexer) scanToken() (Token, error) {
	startLine := l.line
	startColumn := l.column
	ch := l.peek()

	if unicode.IsDigit(ch) {
		return l.scanNumber("", startLine, startColumn), nil
	}

	if ch == '+' {
		l.advance()
		return Token{
			Type:   TokenPlus,
			Value:  "+",
			Line:   startLine,
			Column: startColumn,
		}, nil
	}

	if ch == '-' {
		l.advance()
		// Check if this is a negative number (minus followed by digit with no whitespace)
		if !l.isAtEnd() && unicode.IsDigit(l.peek()) {
			return l.scanNumber("-", startLine, startColumn), nil
		}
		return Token{
			Type:   TokenMinus,
			Value:  "-",
			Line:   startLine,
			Column: startColumn,
		}, nil
	}

	if ch == '*' {
		l.advance()
		return Token{
			Type:   TokenStar,
			Value:  "*",
			Line:   startLine,
			Column: startColumn,
		}, nil
	}

	if ch == '/' {
		l.advance()
		return Token{
			Type:   TokenSlash,
			Value:  "/",
			Line:   startLine,
			Column: startColumn,
		}, nil
	}

	return Token{}, &LexerError{
		Message: "unexpected character",
		Line:    startLine,
		Column:  startColumn,
	}
}

func (l *Lexer) scanNumber(prefix string, startLine, startColumn int) Token {
	value := prefix

	// Scan integer part
	for !l.isAtEnd() && unicode.IsDigit(l.peek()) {
		value += string(l.advance())
	}

	// Check for decimal point
	if !l.isAtEnd() && l.peek() == '.' {
		value += string(l.advance())

		// Scan fractional part
		for !l.isAtEnd() && unicode.IsDigit(l.peek()) {
			value += string(l.advance())
		}
	}

	return Token{
		Type:   TokenNumber,
		Value:  value,
		Line:   startLine,
		Column: startColumn,
	}
}
