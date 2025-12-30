package rpn2tex

import "fmt"

// TokenType represents the type of a lexical token.
type TokenType int

const (
	TokenNumber TokenType = iota
	TokenPlus
	TokenMinus
	TokenTimes
	TokenDivide
	TokenEOF
)

// Token represents a lexical token with position information.
type Token struct {
	Type   TokenType
	Value  string
	Line   int // 1-based
	Column int // 1-based
}

// String returns a string representation of the token for debugging.
func (t Token) String() string {
	return fmt.Sprintf("Token(%s, %q, %d, %d)", t.Type, t.Value, t.Line, t.Column)
}

// String returns a string representation of the token type.
func (tt TokenType) String() string {
	switch tt {
	case TokenNumber:
		return "NUMBER"
	case TokenPlus:
		return "PLUS"
	case TokenMinus:
		return "MINUS"
	case TokenTimes:
		return "TIMES"
	case TokenDivide:
		return "DIVIDE"
	case TokenEOF:
		return "EOF"
	default:
		return "UNKNOWN"
	}
}
