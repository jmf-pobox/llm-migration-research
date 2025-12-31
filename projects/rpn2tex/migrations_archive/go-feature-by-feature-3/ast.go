package rpn2tex

// Expr is the interface for all AST expression nodes.
type Expr interface {
	// Position returns the line and column of this expression.
	Position() (line, column int)
}

// Number represents a numeric literal in the AST.
type Number struct {
	Line   int
	Column int
	Value  string
}

// Position returns the line and column of the number.
func (n *Number) Position() (int, int) {
	return n.Line, n.Column
}

// BinaryOp represents a binary operation in the AST.
type BinaryOp struct {
	Line     int
	Column   int
	Operator string
	Left     Expr
	Right    Expr
}

// Position returns the line and column of the binary operation.
func (b *BinaryOp) Position() (int, int) {
	return b.Line, b.Column
}
