package rpn2tex

// Parser parses a list of tokens into an AST using RPN (stack-based) evaluation.
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

// Parse parses the tokens and returns the root AST node.
func (p *Parser) Parse() (Expr, error) {
	stack := []Expr{}

	for !p.atEnd() {
		token := p.current()

		switch token.Type {
		case TokenNumber:
			node := &Number{
				Line:   token.Line,
				Column: token.Column,
				Value:  token.Value,
			}
			stack = append(stack, node)
			p.advance()

		case TokenPlus:
			if len(stack) < 2 {
				return nil, &ParserError{
					Message: "Operator '+' requires two operands",
					Line:    token.Line,
					Column:  token.Column,
				}
			}
			// Pop right operand first (stack is LIFO)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			// Pop left operand
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			node := &BinaryOp{
				Line:     token.Line,
				Column:   token.Column,
				Operator: "+",
				Left:     left,
				Right:    right,
			}
			stack = append(stack, node)
			p.advance()

		case TokenMinus:
			if len(stack) < 2 {
				return nil, &ParserError{
					Message: "Operator '-' requires two operands",
					Line:    token.Line,
					Column:  token.Column,
				}
			}
			// Pop right operand first (stack is LIFO)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			// Pop left operand
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			node := &BinaryOp{
				Line:     token.Line,
				Column:   token.Column,
				Operator: "-",
				Left:     left,
				Right:    right,
			}
			stack = append(stack, node)
			p.advance()

		case TokenMul:
			if len(stack) < 2 {
				return nil, &ParserError{
					Message: "Operator '*' requires two operands",
					Line:    token.Line,
					Column:  token.Column,
				}
			}
			// Pop right operand first (stack is LIFO)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			// Pop left operand
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			node := &BinaryOp{
				Line:     token.Line,
				Column:   token.Column,
				Operator: "*",
				Left:     left,
				Right:    right,
			}
			stack = append(stack, node)
			p.advance()

		case TokenDiv:
			if len(stack) < 2 {
				return nil, &ParserError{
					Message: "Operator '/' requires two operands",
					Line:    token.Line,
					Column:  token.Column,
				}
			}
			// Pop right operand first (stack is LIFO)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			// Pop left operand
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			node := &BinaryOp{
				Line:     token.Line,
				Column:   token.Column,
				Operator: "/",
				Left:     left,
				Right:    right,
			}
			stack = append(stack, node)
			p.advance()

		case TokenEOF:
			break

		default:
			return nil, &ParserError{
				Message: "Unexpected token: " + token.Type.String(),
				Line:    token.Line,
				Column:  token.Column,
			}
		}
	}

	if len(stack) != 1 {
		return nil, &ParserError{
			Message: "Invalid RPN expression",
			Line:    1,
			Column:  1,
		}
	}

	return stack[0], nil
}

// current returns the current token.
func (p *Parser) current() Token {
	if p.atEnd() {
		return p.tokens[len(p.tokens)-1] // Return EOF
	}
	return p.tokens[p.pos]
}

// advance moves to the next token.
func (p *Parser) advance() {
	if !p.atEnd() {
		p.pos++
	}
}

// atEnd returns true if we've reached the end of tokens.
func (p *Parser) atEnd() bool {
	return p.pos >= len(p.tokens) || p.tokens[p.pos].Type == TokenEOF
}
