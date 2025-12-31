// Package rpn2tex provides a compiler pipeline for converting
// Reverse Polish Notation (RPN) expressions to LaTeX mathematical notation.
package rpn2tex

import "fmt"

// TokenType represents the type of a lexical token.
type TokenType int

const (
	NUMBER TokenType = iota // Numeric values: 5, 3.14, -2
	PLUS                    // + (addition)
	MINUS                   // - (subtraction)
	MULT                    // * (multiplication)
	DIV                     // / (division)
	EOF                     // End of input
)

// String returns the string representation of the TokenType.
func (t TokenType) String() string {
	switch t {
	case NUMBER:
		return "NUMBER"
	case PLUS:
		return "PLUS"
	case MINUS:
		return "MINUS"
	case MULT:
		return "MULT"
	case DIV:
		return "DIV"
	case EOF:
		return "EOF"
	default:
		return fmt.Sprintf("TokenType(%d)", int(t))
	}
}

// Token represents a lexical token with its type, value, and position.
type Token struct {
	Type   TokenType // The token type
	Value  string    // The lexeme (string representation)
	Line   int       // 1-based line number in source
	Column int       // 1-based column number in source
}

// String returns a string representation of the Token for debugging.
func (t Token) String() string {
	return fmt.Sprintf("Token(%s, %q, %d:%d)",
		t.Type.String(), t.Value, t.Line, t.Column)
}
