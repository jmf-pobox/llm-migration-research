package main

import "fmt"

// TokenType represents the type of a lexical token in the RPN expression.
type TokenType int

// Token types supported by the rpn2tex lexer.
const (
	NUMBER TokenType = iota
	PLUS
	MINUS
	MULT
	DIV
	EOF
)

// String returns the string representation of a TokenType.
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

// Token represents a lexical token with position information.
// It is immutable by design - all fields are read-only after creation.
type Token struct {
	Type   TokenType
	Value  string
	Line   int
	Column int
}

// String returns the string representation of a Token for debugging.
// Format: Token(TYPE, "value", line:column)
func (t Token) String() string {
	return fmt.Sprintf("Token(%s, %q, %d:%d)", t.Type, t.Value, t.Line, t.Column)
}
