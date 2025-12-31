package rpn2tex

// Expr represents any expression node in the AST.
// This interface is implemented by Number and BinaryOp.
type Expr interface {
	// GetLine returns the source line number (1-based)
	GetLine() int
	// GetColumn returns the source column number (1-based)
	GetColumn() int
	// isExpr is a marker method to ensure only valid types implement this interface
	isExpr()
}

// Number represents a numeric literal in the AST.
// The value is stored as a string to preserve exact decimal representation.
type Number struct {
	Line   int
	Column int
	Value  string
}

// GetLine returns the source line number (1-based)
func (n *Number) GetLine() int {
	return n.Line
}

// GetColumn returns the source column number (1-based)
func (n *Number) GetColumn() int {
	return n.Column
}

// isExpr implements the Expr interface marker method
func (n *Number) isExpr() {}

// BinaryOp represents a binary operation in the AST.
// The operator is one of: "+", "-", "*", "/" (not LaTeX form).
// Left and Right are recursive Expr types (can be Number or BinaryOp).
type BinaryOp struct {
	Line     int
	Column   int
	Operator string
	Left     Expr
	Right    Expr
}

// GetLine returns the source line number (1-based)
func (b *BinaryOp) GetLine() int {
	return b.Line
}

// GetColumn returns the source column number (1-based)
func (b *BinaryOp) GetColumn() int {
	return b.Column
}

// isExpr implements the Expr interface marker method
func (b *BinaryOp) isExpr() {}
