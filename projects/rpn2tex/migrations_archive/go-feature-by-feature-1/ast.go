package rpn2tex

// Expr is the interface for all AST expression nodes.
type Expr interface {
	Position() (line int, column int)
	Precedence() int
}

// NumberNode represents a numeric literal in the AST.
type NumberNode struct {
	Line   int
	Column int
	Value  string // String representation preserves formatting
}

// Position returns the line and column of the node.
func (n *NumberNode) Position() (int, int) {
	return n.Line, n.Column
}

// Precedence returns the precedence level of the node.
// Numbers have the highest precedence (3) and never need parentheses.
func (n *NumberNode) Precedence() int {
	return 3
}

// BinaryOpNode represents a binary operation in the AST.
type BinaryOpNode struct {
	Line     int
	Column   int
	Operator string // Operator symbol ("+", "-", "*", "/")
	Left     Expr   // Left operand expression
	Right    Expr   // Right operand expression
}

// Position returns the line and column of the node.
func (b *BinaryOpNode) Position() (int, int) {
	return b.Line, b.Column
}

// Precedence returns the precedence level of the operator.
// Addition and subtraction have precedence 1 (lowest).
// Multiplication and division have precedence 2 (higher).
func (b *BinaryOpNode) Precedence() int {
	switch b.Operator {
	case "+", "-":
		return 1
	case "*", "/":
		return 2
	default:
		return 0
	}
}
