package rpn2tex

import "fmt"

// ParserError represents an error that occurs during parsing.
type ParserError struct {
	Message string
	Token   *Token
}

// Error implements the error interface for ParserError.
func (e *ParserError) Error() string {
	return fmt.Sprintf("%s at line %d, column %d", e.Message, e.Token.Line, e.Token.Column)
}

// Parser converts a stream of tokens into an Abstract Syntax Tree (AST).
// It implements a stack-based RPN (Reverse Polish Notation) parser.
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

// Parse parses the token stream and returns the root of the AST.
// It implements RPN parsing using a stack:
// - NUMBER tokens are pushed onto the stack
// - OPERATOR tokens pop two operands and push a BinaryOp node
// - At EOF, the stack must contain exactly one element (the result)
//
// Returns an error if the expression is malformed.
func (p *Parser) Parse() (Expr, error) {
	var stack []Expr

	for !p.atEnd() {
		token := p.current()
		p.advance()

		switch token.Type {
		case NUMBER:
			// Push number onto stack
			stack = append(stack, &Number{
				Line:   token.Line,
				Column: token.Column,
				Value:  token.Value,
			})

		case PLUS, MINUS, MULT, DIV:
			// Pop two operands and create BinaryOp
			if len(stack) < 2 {
				return nil, &ParserError{
					Message: "Insufficient operands for operator",
					Token:   &token,
				}
			}

			// CRITICAL: Stack order matters!
			// First pop is RIGHT operand, second pop is LEFT operand (LIFO)
			right := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			// Map TokenType to operator string
			operator := p.tokenTypeToOperator(token.Type)

			// Push BinaryOp onto stack
			stack = append(stack, &BinaryOp{
				Line:     token.Line,
				Column:   token.Column,
				Operator: operator,
				Left:     left,
				Right:    right,
			})

		case EOF:
			// End of input, validate stack
			break

		default:
			return nil, &ParserError{
				Message: fmt.Sprintf("Unexpected token type: %s", token.Type),
				Token:   &token,
			}
		}
	}

	// Validate: stack must have exactly 1 element
	if len(stack) == 0 {
		// Create a dummy EOF token for error reporting
		eofToken := Token{Type: EOF, Value: "", Line: 1, Column: 1}
		if p.pos > 0 && p.pos <= len(p.tokens) {
			lastToken := p.tokens[p.pos-1]
			eofToken.Line = lastToken.Line
			eofToken.Column = lastToken.Column + len(lastToken.Value)
		}
		return nil, &ParserError{
			Message: "Empty expression",
			Token:   &eofToken,
		}
	}

	if len(stack) > 1 {
		// Multiple elements remain: incomplete expression (missing operators)
		// Use the position of the first remaining element
		firstRemaining := stack[0]
		token := Token{
			Type:   NUMBER, // The remaining elements are likely numbers
			Value:  "",
			Line:   firstRemaining.GetLine(),
			Column: firstRemaining.GetColumn(),
		}
		return nil, &ParserError{
			Message: "Incomplete expression: too many operands",
			Token:   &token,
		}
	}

	return stack[0], nil
}

// current returns the current token without advancing.
func (p *Parser) current() Token {
	if p.pos >= len(p.tokens) {
		// Return EOF token if past end
		return Token{Type: EOF, Value: "", Line: 1, Column: 1}
	}
	return p.tokens[p.pos]
}

// atEnd returns true if the parser has reached EOF token.
func (p *Parser) atEnd() bool {
	return p.pos >= len(p.tokens) || p.current().Type == EOF
}

// advance moves to the next token.
func (p *Parser) advance() Token {
	if p.pos < len(p.tokens) {
		token := p.tokens[p.pos]
		p.pos++
		return token
	}
	return Token{Type: EOF, Value: "", Line: 1, Column: 1}
}

// tokenTypeToOperator maps TokenType to operator string.
// Returns: PLUS→"+", MINUS→"-", MULT→"*", DIV→"/"
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
