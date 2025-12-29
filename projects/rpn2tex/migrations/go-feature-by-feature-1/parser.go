package rpn2tex

import (
	"fmt"
)

// Parser parses tokens into an AST using RPN semantics.
type Parser struct {
	lexer   *Lexer
	current Token
}

// NewParser creates a new parser for the given lexer.
func NewParser(lexer *Lexer) *Parser {
	return &Parser{
		lexer: lexer,
	}
}

// Parse parses the input and returns the root AST node.
func (p *Parser) Parse() (Expr, error) {
	stack := []Expr{}

	// Initialize by getting first token
	if err := p.advance(); err != nil {
		return nil, err
	}

	for p.current.Type != TokenEOF {
		switch p.current.Type {
		case TokenNumber:
			// Create number node and push to stack
			node := &NumberNode{
				Line:   p.current.Line,
				Column: p.current.Column,
				Value:  p.current.Value,
			}
			stack = append(stack, node)
			if err := p.advance(); err != nil {
				return nil, err
			}
		case TokenPlus:
			// Binary operator: pop two operands, create BinaryOp, push result
			if len(stack) < 2 {
				return nil, fmt.Errorf("Operator '+' requires two operands")
			}
			// Pop right operand first (stack order)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			// Pop left operand
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			// Create binary operation node
			opNode := &BinaryOpNode{
				Line:     p.current.Line,
				Column:   p.current.Column,
				Operator: "+",
				Left:     left,
				Right:    right,
			}
			stack = append(stack, opNode)
			if err := p.advance(); err != nil {
				return nil, err
			}
		case TokenMinus:
			// Binary operator: pop two operands, create BinaryOp, push result
			if len(stack) < 2 {
				return nil, fmt.Errorf("Operator '-' requires two operands")
			}
			// Pop right operand first (stack order) - CRITICAL for non-commutative ops
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			// Pop left operand
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			// Create binary operation node
			opNode := &BinaryOpNode{
				Line:     p.current.Line,
				Column:   p.current.Column,
				Operator: "-",
				Left:     left,
				Right:    right,
			}
			stack = append(stack, opNode)
			if err := p.advance(); err != nil {
				return nil, err
			}
		case TokenTimes:
			// Binary operator: pop two operands, create BinaryOp, push result
			if len(stack) < 2 {
				return nil, fmt.Errorf("Operator '*' requires two operands")
			}
			// Pop right operand first (stack order)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			// Pop left operand
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			// Create binary operation node
			opNode := &BinaryOpNode{
				Line:     p.current.Line,
				Column:   p.current.Column,
				Operator: "*",
				Left:     left,
				Right:    right,
			}
			stack = append(stack, opNode)
			if err := p.advance(); err != nil {
				return nil, err
			}
		case TokenDivide:
			// Binary operator: pop two operands, create BinaryOp, push result
			if len(stack) < 2 {
				return nil, fmt.Errorf("Operator '/' requires two operands")
			}
			// Pop right operand first (stack order) - CRITICAL for non-commutative ops
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			// Pop left operand
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			// Create binary operation node
			opNode := &BinaryOpNode{
				Line:     p.current.Line,
				Column:   p.current.Column,
				Operator: "/",
				Left:     left,
				Right:    right,
			}
			stack = append(stack, opNode)
			if err := p.advance(); err != nil {
				return nil, err
			}
		default:
			return nil, fmt.Errorf("unexpected token: %s", p.current.Type)
		}
	}

	// Should have exactly one expression on stack
	if len(stack) == 0 {
		return nil, fmt.Errorf("empty input")
	}
	if len(stack) != 1 {
		return nil, fmt.Errorf("incomplete expression: %d operands remaining", len(stack))
	}

	return stack[0], nil
}

// advance moves to the next token.
func (p *Parser) advance() error {
	token, err := p.lexer.NextToken()
	if err != nil {
		return err
	}
	p.current = token
	return nil
}
