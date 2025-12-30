// Package rpn2tex provides tools for converting Reverse Polish Notation (RPN)
// expressions to LaTeX math mode output.
package rpn2tex

import "fmt"

// TokenType represents the type of a lexical token.
type TokenType int

// Token type constants define all lexical token types used by the lexer.
const (
	NUMBER TokenType = iota // Numeric literal (integer or decimal)
	PLUS                    // Addition operator '+'
	MINUS                   // Subtraction operator '-' or negative sign
	MULT                    // Multiplication operator '*'
	DIV                     // Division operator '/'
	EOF                     // End of file/input marker
)

// String returns a human-readable string representation of the token type.
func (tt TokenType) String() string {
	switch tt {
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
		return fmt.Sprintf("TokenType(%d)", tt)
	}
}

// Token represents a lexical token with its type, value, and position in the source.
// Position tracking uses 1-based line and column numbers for human-readable error messages.
type Token struct {
	Type   TokenType // The type of token
	Value  string    // The raw string value of the token
	Line   int       // Line number (1-based)
	Column int       // Column number (1-based)
}

// String returns a formatted string representation of the token for debugging.
func (t Token) String() string {
	return fmt.Sprintf("Token(%s, %q, %d, %d)", t.Type, t.Value, t.Line, t.Column)
}
