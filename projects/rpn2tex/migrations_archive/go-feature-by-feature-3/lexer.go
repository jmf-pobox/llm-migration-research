package rpn2tex

import "unicode"

// Lexer tokenizes RPN mathematical expressions.
type Lexer struct {
	source string
	pos    int
	line   int
	column int
	tokens []Token
}

// NewLexer creates a new lexer for the given source text.
func NewLexer(source string) *Lexer {
	return &Lexer{
		source: source,
		pos:    0,
		line:   1,
		column: 1,
		tokens: []Token{},
	}
}

// Tokenize scans the source text and returns a list of tokens.
func (l *Lexer) Tokenize() ([]Token, error) {
	for !l.atEnd() {
		l.skipWhitespace()
		if l.atEnd() {
			break
		}

		startLine := l.line
		startColumn := l.column
		char := l.peek()

		if unicode.IsDigit(char) {
			token := l.scanNumber("", startLine, startColumn)
			l.tokens = append(l.tokens, token)
		} else if char == '+' {
			l.advance()
			l.tokens = append(l.tokens, Token{
				Type:   TokenPlus,
				Value:  "+",
				Line:   startLine,
				Column: startColumn,
			})
		} else if char == '-' {
			l.advance()
			// Check if this is a negative number (digit follows immediately)
			if !l.atEnd() && unicode.IsDigit(l.peek()) {
				// It's a negative number
				token := l.scanNumber("-", startLine, startColumn)
				l.tokens = append(l.tokens, token)
			} else {
				// It's a subtraction operator
				l.tokens = append(l.tokens, Token{
					Type:   TokenMinus,
					Value:  "-",
					Line:   startLine,
					Column: startColumn,
				})
			}
		} else if char == '*' {
			l.advance()
			l.tokens = append(l.tokens, Token{
				Type:   TokenMul,
				Value:  "*",
				Line:   startLine,
				Column: startColumn,
			})
		} else if char == '/' {
			l.advance()
			l.tokens = append(l.tokens, Token{
				Type:   TokenDiv,
				Value:  "/",
				Line:   startLine,
				Column: startColumn,
			})
		} else {
			return nil, &LexerError{
				Message: "Unexpected character: " + string(char),
				Line:    startLine,
				Column:  startColumn,
			}
		}
	}

	// Add EOF token
	l.tokens = append(l.tokens, Token{
		Type:   TokenEOF,
		Value:  "",
		Line:   l.line,
		Column: l.column,
	})

	return l.tokens, nil
}

// scanNumber scans a numeric literal (integer or float).
func (l *Lexer) scanNumber(prefix string, startLine, startColumn int) Token {
	value := prefix

	// Integer part
	for !l.atEnd() && unicode.IsDigit(l.peek()) {
		value += string(l.advance())
	}

	// Decimal part (optional)
	if !l.atEnd() && l.peek() == '.' {
		value += string(l.advance()) // consume '.'
		for !l.atEnd() && unicode.IsDigit(l.peek()) {
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

// skipWhitespace advances past whitespace characters.
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
func (l *Lexer) peek() rune {
	if l.atEnd() {
		return 0
	}
	return rune(l.source[l.pos])
}

// advance consumes the current character and returns it.
func (l *Lexer) advance() rune {
	if l.atEnd() {
		return 0
	}
	char := rune(l.source[l.pos])
	l.pos++
	if char == '\n' {
		l.line++
		l.column = 1
	} else {
		l.column++
	}
	return char
}

// atEnd returns true if we've reached the end of the source.
func (l *Lexer) atEnd() bool {
	return l.pos >= len(l.source)
}
