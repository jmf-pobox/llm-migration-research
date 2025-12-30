package main

// TokenType represents the type of a lexical token
type TokenType int

const (
	// NUMBER represents numeric literals (integers and decimals)
	NUMBER TokenType = iota
	// PLUS represents the addition operator (+)
	PLUS
	// MINUS represents the subtraction operator (-)
	MINUS
	// MULTIPLY represents the multiplication operator (*)
	MULTIPLY
	// DIVIDE represents the division operator (/)
	DIVIDE
)

// Token represents a lexical token with position information
type Token struct {
	Type   TokenType
	Value  string
	Line   int
	Column int
}
