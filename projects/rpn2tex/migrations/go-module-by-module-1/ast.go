package main

// Expr is the interface for all AST expression nodes.
// It provides a marker method and accessors for position information.
type Expr interface {
	exprNode()
	GetLine() int
	GetColumn() int
}

// Number represents a numeric literal in the AST.
type Number struct {
	Line   int
	Column int
	Value  string
}

// exprNode marks Number as an Expr implementation.
func (n *Number) exprNode() {}

// GetLine returns the line number where this number appears.
func (n *Number) GetLine() int {
	return n.Line
}

// GetColumn returns the column number where this number appears.
func (n *Number) GetColumn() int {
	return n.Column
}

// BinaryOp represents a binary operation in the AST.
type BinaryOp struct {
	Line     int
	Column   int
	Operator string
	Left     Expr
	Right    Expr
}

// exprNode marks BinaryOp as an Expr implementation.
func (b *BinaryOp) exprNode() {}

// GetLine returns the line number where this operation appears.
func (b *BinaryOp) GetLine() int {
	return b.Line
}

// GetColumn returns the column number where this operation appears.
func (b *BinaryOp) GetColumn() int {
	return b.Column
}
