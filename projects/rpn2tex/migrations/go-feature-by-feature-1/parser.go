package main

import "fmt"

// Parser parses tokens into an AST
type Parser struct {
	tokens []Token
	pos    int
}

// NewParser creates a new parser for the given tokens
func NewParser(tokens []Token) *Parser {
	return &Parser{
		tokens: tokens,
		pos:    0,
	}
}

// Parse parses the tokens into an expression AST
func (p *Parser) Parse() (Expr, error) {
	var stack []Expr

	for p.pos < len(p.tokens) {
		token := p.tokens[p.pos]

		switch token.Type {
		case NUMBER:
			numNode := &Number{
				Value:  token.Value,
				Line:   token.Line,
				Column: token.Column,
			}
			stack = append(stack, numNode)
			p.pos++
		case PLUS:
			if len(stack) < 2 {
				return nil, &ParserError{
					Message: fmt.Sprintf("Operator '%s' requires two operands", token.Value),
					Token:   token,
				}
			}
			// Pop right then left (RPN order)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			opNode := &BinaryOp{
				Operator: "+",
				Left:     left,
				Right:    right,
				Line:     token.Line,
				Column:   token.Column,
			}
			stack = append(stack, opNode)
			p.pos++
		case MINUS:
			if len(stack) < 2 {
				return nil, &ParserError{
					Message: fmt.Sprintf("Operator '%s' requires two operands", token.Value),
					Token:   token,
				}
			}
			// Pop right then left (RPN order)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			opNode := &BinaryOp{
				Operator: "-",
				Left:     left,
				Right:    right,
				Line:     token.Line,
				Column:   token.Column,
			}
			stack = append(stack, opNode)
			p.pos++
		case MULTIPLY:
			if len(stack) < 2 {
				return nil, &ParserError{
					Message: fmt.Sprintf("Operator '%s' requires two operands", token.Value),
					Token:   token,
				}
			}
			// Pop right then left (RPN order)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			opNode := &BinaryOp{
				Operator: "*",
				Left:     left,
				Right:    right,
				Line:     token.Line,
				Column:   token.Column,
			}
			stack = append(stack, opNode)
			p.pos++
		case DIVIDE:
			if len(stack) < 2 {
				return nil, &ParserError{
					Message: fmt.Sprintf("Operator '%s' requires two operands", token.Value),
					Token:   token,
				}
			}
			// Pop right then left (RPN order)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			opNode := &BinaryOp{
				Operator: "/",
				Left:     left,
				Right:    right,
				Line:     token.Line,
				Column:   token.Column,
			}
			stack = append(stack, opNode)
			p.pos++
		default:
			return nil, &ParserError{
				Message: fmt.Sprintf("Unknown token type: %v", token.Type),
				Token:   token,
			}
		}
	}

	// Should have exactly one expression on the stack
	if len(stack) == 0 {
		return nil, fmt.Errorf("empty input")
	}
	if len(stack) > 1 {
		return nil, fmt.Errorf("incomplete expression: too many operands")
	}

	return stack[0], nil
}
