package rpn2tex

import (
	"unicode"
)

// Lexer tokenizes input text into a stream of tokens.
type Lexer struct {
	text   []rune
	pos    int
	line   int
	column int
}

// NewLexer creates a new Lexer for the given input text.
func NewLexer(text string) *Lexer {
	return &Lexer{
		text:   []rune(text),
		pos:    0,
		line:   1,
		column: 1,
	}
}

// Tokenize scans the input and returns a list of tokens.
func (l *Lexer) Tokenize() ([]Token, error) {
	var tokens []Token

	for !l.atEnd() {
		l.skipWhitespace()
		if l.atEnd() {
			break
		}

		startLine := l.line
		startColumn := l.column
		ch := l.peek()

		if unicode.IsDigit(ch) {
			token, err := l.scanNumber("", startLine, startColumn)
			if err != nil {
				return nil, err
			}
			tokens = append(tokens, token)
		} else if ch == '+' {
			l.advance()
			tokens = append(tokens, Token{
				Type:   TokenPlus,
				Value:  "+",
				Line:   startLine,
				Column: startColumn,
			})
		} else if ch == '-' {
			l.advance()
			if !l.atEnd() && unicode.IsDigit(l.peek()) {
				// It's a negative number
				token, err := l.scanNumber("-", startLine, startColumn)
				if err != nil {
					return nil, err
				}
				tokens = append(tokens, token)
			} else {
				// It's a subtraction operator
				tokens = append(tokens, Token{
					Type:   TokenMinus,
					Value:  "-",
					Line:   startLine,
					Column: startColumn,
				})
			}
		} else if ch == '*' {
			l.advance()
			tokens = append(tokens, Token{
				Type:   TokenMult,
				Value:  "*",
				Line:   startLine,
				Column: startColumn,
			})
		} else if ch == '/' {
			l.advance()
			tokens = append(tokens, Token{
				Type:   TokenDiv,
				Value:  "/",
				Line:   startLine,
				Column: startColumn,
			})
		} else {
			return nil, &LexerError{
				Message: "Unexpected character '" + string(ch) + "'",
				Line:    startLine,
				Column:  startColumn,
			}
		}
	}

	tokens = append(tokens, Token{Type: TokenEOF, Value: "", Line: l.line, Column: l.column})
	return tokens, nil
}

// scanNumber scans a numeric literal (integer or decimal).
func (l *Lexer) scanNumber(prefix string, startLine, startColumn int) (Token, error) {
	value := prefix

	// Scan integer part
	for !l.atEnd() && unicode.IsDigit(l.peek()) {
		value += string(l.advance())
	}

	// Scan decimal part (optional)
	if !l.atEnd() && l.peek() == '.' {
		value += string(l.advance())
		for !l.atEnd() && unicode.IsDigit(l.peek()) {
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

// peek returns the current character without consuming it.
func (l *Lexer) peek() rune {
	if l.atEnd() {
		return 0
	}
	return l.text[l.pos]
}

// advance consumes and returns the current character.
func (l *Lexer) advance() rune {
	if l.atEnd() {
		return 0
	}
	ch := l.text[l.pos]
	l.pos++
	if ch == '\n' {
		l.line++
		l.column = 1
	} else {
		l.column++
	}
	return ch
}

// atEnd checks if we've reached the end of input.
func (l *Lexer) atEnd() bool {
	return l.pos >= len(l.text)
}

// skipWhitespace skips whitespace characters.
func (l *Lexer) skipWhitespace() {
	for !l.atEnd() && unicode.IsSpace(l.peek()) {
		l.advance()
	}
}
