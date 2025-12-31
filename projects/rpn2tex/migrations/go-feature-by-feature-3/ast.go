package rpn2tex

// Expr is the interface for all AST nodes.
type Expr interface {
	expr()
}

// Number represents a numeric literal in the AST.
type Number struct {
	Value  string
	Line   int
	Column int
}

func (n *Number) expr() {}

// BinaryOp represents a binary operation in the AST.
type BinaryOp struct {
	Operator string
	Left     Expr
	Right    Expr
	Line     int
	Column   int
}

func (b *BinaryOp) expr() {}
