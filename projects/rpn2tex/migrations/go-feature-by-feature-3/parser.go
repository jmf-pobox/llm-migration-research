package rpn2tex

import "fmt"

// Parser parses tokens into an Abstract Syntax Tree.
type Parser struct {
	tokens []Token
	pos    int
}

// NewParser creates a new parser for the given tokens.
func NewParser(tokens []Token) *Parser {
	return &Parser{
		tokens: tokens,
		pos:    0,
	}
}

// Parse parses the tokens using RPN stack-based algorithm.
func (p *Parser) Parse() (Expr, error) {
	var stack []Expr

	for !p.isAtEnd() {
		token := p.current()

		if token.Type == TokenNumber {
			node := &Number{
				Value:  token.Value,
				Line:   token.Line,
				Column: token.Column,
			}
			stack = append(stack, node)
			p.advance()
		} else if token.Type == TokenPlus || token.Type == TokenMinus || token.Type == TokenStar || token.Type == TokenSlash {
			if len(stack) < 2 {
				operator := "+"
				if token.Type == TokenMinus {
					operator = "-"
				} else if token.Type == TokenStar {
					operator = "*"
				} else if token.Type == TokenSlash {
					operator = "/"
				}
				return nil, &ParserError{
					Message: fmt.Sprintf("not enough operands for %s operator", operator),
					Line:    token.Line,
					Column:  token.Column,
				}
			}
			// Pop two operands
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			// Determine operator string
			operator := "+"
			if token.Type == TokenMinus {
				operator = "-"
			} else if token.Type == TokenStar {
				operator = "*"
			} else if token.Type == TokenSlash {
				operator = "/"
			}

			// Create BinaryOp node
			node := &BinaryOp{
				Operator: operator,
				Left:     left,
				Right:    right,
				Line:     token.Line,
				Column:   token.Column,
			}
			stack = append(stack, node)
			p.advance()
		} else if token.Type == TokenEOF {
			break
		} else {
			return nil, &ParserError{
				Message: fmt.Sprintf("unexpected token type: %v", token.Type),
				Line:    token.Line,
				Column:  token.Column,
			}
		}
	}

	if len(stack) == 0 {
		return nil, &ParserError{
			Message: "empty expression",
			Line:    1,
			Column:  1,
		}
	}

	if len(stack) != 1 {
		return nil, &ParserError{
			Message: fmt.Sprintf("expected single result, got %d values on stack", len(stack)),
			Line:    p.current().Line,
			Column:  p.current().Column,
		}
	}

	return stack[0], nil
}

func (p *Parser) isAtEnd() bool {
	return p.pos >= len(p.tokens) || p.current().Type == TokenEOF
}

func (p *Parser) current() Token {
	if p.pos >= len(p.tokens) {
		return Token{Type: TokenEOF}
	}
	return p.tokens[p.pos]
}

func (p *Parser) advance() {
	if !p.isAtEnd() {
		p.pos++
	}
}
