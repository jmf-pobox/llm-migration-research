package rpn2tex

// TokenType represents the type of a token.
type TokenType int

const (
	TokenNumber TokenType = iota
	TokenPlus
	TokenMinus
	TokenStar
	TokenSlash
	TokenEOF
)

// Token represents a lexical token with position information.
type Token struct {
	Type   TokenType
	Value  string
	Line   int
	Column int
}
