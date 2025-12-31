package rpn2tex

// TokenType represents the type of a lexical token.
type TokenType int

const (
	// TokenNumber represents numeric literals (integers and floats)
	TokenNumber TokenType = iota
	// TokenPlus represents the addition operator (+)
	TokenPlus
	// TokenMinus represents the subtraction operator (-)
	TokenMinus
	// TokenMul represents the multiplication operator (*)
	TokenMul
	// TokenDiv represents the division operator (/)
	TokenDiv
	// TokenEOF represents end of file
	TokenEOF
)

// String returns the string representation of a TokenType.
func (t TokenType) String() string {
	switch t {
	case TokenNumber:
		return "NUMBER"
	case TokenPlus:
		return "PLUS"
	case TokenMinus:
		return "MINUS"
	case TokenMul:
		return "MUL"
	case TokenDiv:
		return "DIV"
	case TokenEOF:
		return "EOF"
	default:
		return "UNKNOWN"
	}
}

// Token represents a lexical token with position information.
type Token struct {
	Type   TokenType
	Value  string
	Line   int
	Column int
}
