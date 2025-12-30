package main

import "fmt"

// ParserError represents a parsing error with the token where the error occurred.
type ParserError struct {
	Message string
	Token   Token
}

// Error implements the error interface for ParserError.
func (e *ParserError) Error() string {
	return fmt.Sprintf("%s at line %d, column %d",
		e.Message, e.Token.Line, e.Token.Column)
}

// Parser parses a sequence of tokens into an abstract syntax tree using
// stack-based evaluation for Reverse Polish Notation (RPN).
type Parser struct {
	Tokens []Token
	Pos    int
}

// NewParser creates a new Parser with the given token list.
func NewParser(tokens []Token) *Parser {
	return &Parser{
		Tokens: tokens,
		Pos:    0,
	}
}

// Parse parses the token stream into an AST expression tree.
// It implements stack-based RPN parsing:
// - NUMBER tokens are pushed as Number nodes onto the stack
// - OPERATOR tokens pop two operands, create a BinaryOp, and push the result
// - At EOF, the stack must contain exactly one expression
func (p *Parser) Parse() (Expr, error) {
	stack := []Expr{}

	for !p.atEnd() {
		token := p.current()
		p.advance()

		switch token.Type {
		case NUMBER:
			// Push number node onto stack
			node := &Number{
				Line:   token.Line,
				Column: token.Column,
				Value:  token.Value,
			}
			stack = append(stack, node)

		case PLUS, MINUS, MULT, DIV:
			// Pop two operands, create binary operation, push result
			if len(stack) < 2 {
				return nil, &ParserError{
					Message: fmt.Sprintf("Not enough operands for operator '%s'", token.Value),
					Token:   token,
				}
			}

			// Pop right operand (top of stack)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			// Pop left operand (new top of stack)
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			// Map token type to operator string
			operator := p.tokenTypeToOperator(token.Type)

			// Create binary operation node
			node := &BinaryOp{
				Line:     token.Line,
				Column:   token.Column,
				Operator: operator,
				Left:     left,
				Right:    right,
			}
			stack = append(stack, node)

		case EOF:
			// Stop processing at EOF
			break

		default:
			return nil, &ParserError{
				Message: fmt.Sprintf("Unexpected token type: %s", token.Type),
				Token:   token,
			}
		}
	}

	// Validate the final stack
	if len(stack) == 0 {
		// Use the EOF token for error reporting
		eofToken := p.Tokens[len(p.Tokens)-1]
		return nil, &ParserError{
			Message: "Empty expression",
			Token:   eofToken,
		}
	}

	if len(stack) > 1 {
		// Multiple items left means missing operators
		// Use the position of the second-to-last token
		lastToken := p.Tokens[len(p.Tokens)-2]
		return nil, &ParserError{
			Message: fmt.Sprintf("Too many operands (missing operators): %d items left on stack", len(stack)),
			Token:   lastToken,
		}
	}

	return stack[0], nil
}

// current returns the current token without advancing.
func (p *Parser) current() Token {
	if p.Pos >= len(p.Tokens) {
		// Return EOF token if past the end
		return Token{Type: EOF, Value: "", Line: 0, Column: 0}
	}
	return p.Tokens[p.Pos]
}

// atEnd checks if the parser is at the end of the token stream.
func (p *Parser) atEnd() bool {
	return p.Pos >= len(p.Tokens) || p.current().Type == EOF
}

// advance moves to the next token and returns the previous current token.
func (p *Parser) advance() Token {
	token := p.current()
	if p.Pos < len(p.Tokens) {
		p.Pos++
	}
	return token
}

// tokenTypeToOperator converts a TokenType to its operator string.
func (p *Parser) tokenTypeToOperator(tt TokenType) string {
	switch tt {
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
