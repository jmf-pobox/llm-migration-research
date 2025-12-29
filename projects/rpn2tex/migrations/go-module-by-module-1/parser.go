// Package rpn2tex implements a converter from Reverse Polish Notation to LaTeX.
package rpn2tex

import "fmt"

// ParserError represents an error that occurred during parsing.
// It includes the error message and the token where the error occurred.
type ParserError struct {
	Message string
	Token   Token
}

// Error implements the error interface for ParserError.
func (e *ParserError) Error() string {
	return fmt.Sprintf("%s at line %d, column %d",
		e.Message, e.Token.Line, e.Token.Column)
}

// Parser converts a stream of tokens into an Abstract Syntax Tree (AST)
// using a stack-based algorithm for Reverse Polish Notation (RPN).
type Parser struct {
	tokens []Token
	pos    int
}

// NewParser creates a new Parser with the given token stream.
func NewParser(tokens []Token) *Parser {
	return &Parser{
		tokens: tokens,
		pos:    0,
	}
}

// Parse parses the token stream and returns an AST expression.
// It implements a stack-based RPN parsing algorithm:
// 1. For each number token, create a Number node and push to stack
// 2. For each operator token, pop two operands, create BinaryOp, push result
// 3. At EOF, validate that exactly one expression remains on stack
//
// Returns an error if:
// - The expression is empty
// - An operator has insufficient operands
// - Multiple values remain on stack (missing operators)
func (p *Parser) Parse() (Expr, error) {
	var stack []Expr

	for !p.atEnd() {
		token := p.current()

		switch token.Type {
		case NUMBER:
			// Create Number node and push to stack
			node := &Number{
				Line:   token.Line,
				Column: token.Column,
				Value:  token.Value,
			}
			stack = append(stack, node)
			p.advance()

		case PLUS, MINUS, MULT, DIV:
			// Check for sufficient operands
			if len(stack) < 2 {
				return nil, &ParserError{
					Message: fmt.Sprintf("Operator '%s' requires two operands", token.Value),
					Token:   token,
				}
			}

			// Pop right operand (last pushed, popped first)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			// Pop left operand (first pushed, popped second)
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			// Create BinaryOp node
			node := &BinaryOp{
				Line:     token.Line,
				Column:   token.Column,
				Operator: token.Value,
				Left:     left,
				Right:    right,
			}
			stack = append(stack, node)
			p.advance()

		case EOF:
			// End of token stream
			break

		default:
			// Unexpected token type (should not occur with valid lexer)
			return nil, &ParserError{
				Message: fmt.Sprintf("Unexpected token type %s", token.Type),
				Token:   token,
			}
		}
	}

	// Validate stack has exactly one element
	if len(stack) == 0 {
		// Find EOF token for error reporting
		eofToken := Token{Type: EOF, Value: "", Line: 1, Column: 1}
		if len(p.tokens) > 0 {
			eofToken = p.tokens[len(p.tokens)-1]
		}
		return nil, &ParserError{
			Message: "Empty expression",
			Token:   eofToken,
		}
	}

	if len(stack) > 1 {
		// Find EOF token for error reporting
		eofToken := Token{Type: EOF, Value: "", Line: 1, Column: 1}
		if len(p.tokens) > 0 {
			eofToken = p.tokens[len(p.tokens)-1]
		}
		return nil, &ParserError{
			Message: fmt.Sprintf("Invalid RPN: %d values remain on stack (missing operators?)", len(stack)),
			Token:   eofToken,
		}
	}

	return stack[0], nil
}

// current returns the current token without advancing.
func (p *Parser) current() Token {
	if p.pos >= len(p.tokens) {
		// Return EOF token if past end
		if len(p.tokens) > 0 {
			lastToken := p.tokens[len(p.tokens)-1]
			return Token{
				Type:   EOF,
				Value:  "",
				Line:   lastToken.Line,
				Column: lastToken.Column,
			}
		}
		return Token{Type: EOF, Value: "", Line: 1, Column: 1}
	}
	return p.tokens[p.pos]
}

// atEnd checks if we're at the end of the token stream or at EOF token.
func (p *Parser) atEnd() bool {
	if p.pos >= len(p.tokens) {
		return true
	}
	return p.tokens[p.pos].Type == EOF
}

// advance moves to the next token.
func (p *Parser) advance() Token {
	if !p.atEnd() {
		p.pos++
	}
	return p.current()
}
