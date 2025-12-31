package rpn2tex

import "fmt"

// TokenType represents the type of a lexical token.
type TokenType int

const (
	// NUMBER represents a numeric literal token (integer or decimal).
	NUMBER TokenType = iota
	// PLUS represents the addition operator '+'.
	PLUS
	// MINUS represents the subtraction operator '-'.
	MINUS
	// MULT represents the multiplication operator '*'.
	MULT
	// DIV represents the division operator '/'.
	DIV
	// EOF represents the end of input.
	EOF
)

// String returns the string representation of a TokenType.
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

// Token represents a lexical token with its type, value, and source position.
// Tokens are immutable once created.
type Token struct {
	Type   TokenType // The type of the token
	Value  string    // The string value of the token
	Line   int       // Line number in source (1-based)
	Column int       // Column number in source (1-based)
}

// String returns a formatted string representation of the token.
// Format: Token(TYPE_NAME, 'value', line:column)
// This matches the Python __repr__ output for debugging purposes.
func (t Token) String() string {
	return fmt.Sprintf("Token(%s, '%s', %d:%d)", t.Type, t.Value, t.Line, t.Column)
}
