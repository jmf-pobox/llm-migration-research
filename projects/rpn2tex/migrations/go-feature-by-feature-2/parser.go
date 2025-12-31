package rpn2tex

// Parser converts a token stream into an AST.
type Parser struct {
	tokens []Token
	pos    int
}

// NewParser creates a new Parser for the given token stream.
func NewParser(tokens []Token) *Parser {
	return &Parser{
		tokens: tokens,
		pos:    0,
	}
}

// Parse parses the token stream and returns the root AST node.
func (p *Parser) Parse() (Expr, error) {
	var stack []Expr

	for !p.atEnd() {
		token := p.current()

		if token.Type == TokenNumber {
			node := &Number{
				Value:  token.Value,
				Line:   token.Line,
				Column: token.Column,
			}
			stack = append(stack, node)
			p.advance()
		} else if token.Type == TokenPlus || token.Type == TokenMinus || token.Type == TokenMult || token.Type == TokenDiv {
			// Binary operator: pop 2 operands
			opSymbol := "+"
			if token.Type == TokenMinus {
				opSymbol = "-"
			} else if token.Type == TokenMult {
				opSymbol = "*"
			} else if token.Type == TokenDiv {
				opSymbol = "/"
			}

			if len(stack) < 2 {
				return nil, &ParserError{
					Message: "Operator '" + opSymbol + "' requires two operands",
					Token:   token,
				}
			}
			// Pop right, then left (stack order)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			node := &BinaryOp{
				Operator: opSymbol,
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
				Message: "Unexpected token type",
				Token:   token,
			}
		}
	}

	if len(stack) == 0 {
		return nil, &ParserError{
			Message: "Empty expression",
			Token:   Token{Line: 1, Column: 1},
		}
	}

	if len(stack) > 1 {
		return nil, &ParserError{
			Message: "Too many operands",
			Token:   p.tokens[0],
		}
	}

	return stack[0], nil
}

// current returns the current token without consuming it.
func (p *Parser) current() Token {
	if p.atEnd() {
		return p.tokens[len(p.tokens)-1] // Return EOF token
	}
	return p.tokens[p.pos]
}

// advance moves to the next token.
func (p *Parser) advance() {
	if !p.atEnd() {
		p.pos++
	}
}

// atEnd checks if we've reached the end of the token stream.
func (p *Parser) atEnd() bool {
	return p.pos >= len(p.tokens) || p.tokens[p.pos].Type == TokenEOF
}
