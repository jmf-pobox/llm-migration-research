package rpn2tex

import "fmt"

// Parser converts a token stream to an Abstract Syntax Tree using
// stack-based RPN (Reverse Polish Notation) parsing algorithm.
type Parser struct {
	tokens []Token // Token stream to parse
	pos    int     // Current position in tokens (0-based)
}

// NewParser creates a new parser for the given token stream.
func NewParser(tokens []Token) *Parser {
	return &Parser{
		tokens: tokens,
		pos:    0,
	}
}

// Parse parses the token stream and returns the root AST node.
// It implements the RPN stack-based parsing algorithm:
// - Numbers are pushed onto the stack as Number nodes
// - Operators pop two operands and push a BinaryOp node
// Returns an error if the expression is invalid (empty, insufficient operands,
// or too many operands remaining on the stack).
func (p *Parser) Parse() (Expr, error) {
	var stack []Expr

	// Process all tokens except EOF
	for !p.atEnd() {
		token := p.current()

		// Skip EOF token explicitly
		if token.Type == EOF {
			break
		}

		switch token.Type {
		case NUMBER:
			// Push number node onto stack
			numNode := NewNumber(token.Line, token.Column, token.Value)
			stack = append(stack, numNode)
			p.advance()

		case PLUS, MINUS, MULT, DIV:
			// Binary operator: pop two operands, create BinaryOp node
			if len(stack) < 2 {
				return nil, NewCompileError(
					fmt.Sprintf("Operator '%s' requires two operands", token.Value),
					p.getSource(),
					token.Line,
					token.Column,
				)
			}

			// Pop right operand
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			// Pop left operand
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			// Map token type to operator string
			operator := p.tokenTypeToOperator(token.Type)

			// Create BinaryOp node
			opNode := NewBinaryOp(token.Line, token.Column, operator, left, right)
			stack = append(stack, opNode)
			p.advance()

		default:
			// This shouldn't happen if the lexer is correct
			return nil, NewCompileError(
				fmt.Sprintf("Unexpected token type: %s", token.Type.String()),
				p.getSource(),
				token.Line,
				token.Column,
			)
		}
	}

	// Validate final stack state
	if len(stack) == 0 {
		// Get the EOF token for error position
		eofToken := p.tokens[len(p.tokens)-1]
		return nil, NewCompileError(
			"Empty expression",
			p.getSource(),
			eofToken.Line,
			eofToken.Column,
		)
	}

	if len(stack) > 1 {
		// Too many operands - missing operators
		eofToken := p.tokens[len(p.tokens)-1]
		return nil, NewCompileError(
			fmt.Sprintf("Invalid RPN: %d values remain on stack (expected 1)", len(stack)),
			p.getSource(),
			eofToken.Line,
			eofToken.Column,
		)
	}

	return stack[0], nil
}

// atEnd returns true if the parser has reached the end of the token stream.
func (p *Parser) atEnd() bool {
	return p.pos >= len(p.tokens)
}

// current returns the current token without advancing the position.
func (p *Parser) current() Token {
	if p.atEnd() {
		// Return the last token (should be EOF)
		return p.tokens[len(p.tokens)-1]
	}
	return p.tokens[p.pos]
}

// advance moves to the next token.
func (p *Parser) advance() {
	if !p.atEnd() {
		p.pos++
	}
}

// tokenTypeToOperator converts a token type to its operator string.
func (p *Parser) tokenTypeToOperator(tokenType TokenType) string {
	switch tokenType {
	case PLUS:
		return "+"
	case MINUS:
		return "-"
	case MULT:
		return "*"
	case DIV:
		return "/"
	default:
		return ""
	}
}

// getSource reconstructs the source text from tokens for error reporting.
// This is a helper method to provide source context in error messages.
func (p *Parser) getSource() string {
	// We don't have direct access to the original source, but we can
	// reconstruct a reasonable approximation from tokens.
	// For better error reporting, the caller should pass the original source.
	// For now, return empty string and let the CompileError handle it.
	return ""
}
