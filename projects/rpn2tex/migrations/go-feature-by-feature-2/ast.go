package rpn2tex

// Expr is a marker interface for AST expression nodes.
type Expr interface {
	isExpr()
}

// Number represents a numeric literal in the AST.
type Number struct {
	Value  string
	Line   int
	Column int
}

// isExpr marks Number as an Expr implementer.
func (n *Number) isExpr() {}

// BinaryOp represents a binary operation in the AST.
type BinaryOp struct {
	Operator string
	Left     Expr
	Right    Expr
	Line     int
	Column   int
}

// isExpr marks BinaryOp as an Expr implementer.
func (b *BinaryOp) isExpr() {}
