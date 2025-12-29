// Package rpn2tex implements a converter from Reverse Polish Notation to LaTeX.
package rpn2tex

// Expr is the interface for all AST expression nodes.
// It represents either a Number or a BinaryOp node.
type Expr interface {
	// Accept implements the visitor pattern for traversing the AST.
	Accept(v Visitor) string
	// GetLine returns the line number where this expression appears in the source.
	GetLine() int
	// GetColumn returns the column number where this expression appears in the source.
	GetColumn() int
}

// Number represents a numeric literal in the AST.
// The value is stored as a string to preserve exact formatting (e.g., "3.14", "5").
type Number struct {
	Line   int
	Column int
	Value  string
}

// GetLine returns the line number of this number node.
func (n *Number) GetLine() int {
	return n.Line
}

// GetColumn returns the column number of this number node.
func (n *Number) GetColumn() int {
	return n.Column
}

// Accept implements the visitor pattern for Number nodes.
func (n *Number) Accept(v Visitor) string {
	return v.VisitNumber(n)
}

// BinaryOp represents a binary operation in the AST.
// It has an operator ("+", "-", "*", "/") and left and right operands.
type BinaryOp struct {
	Line     int
	Column   int
	Operator string
	Left     Expr
	Right    Expr
}

// GetLine returns the line number of this binary operation node.
func (b *BinaryOp) GetLine() int {
	return b.Line
}

// GetColumn returns the column number of this binary operation node.
func (b *BinaryOp) GetColumn() int {
	return b.Column
}

// Accept implements the visitor pattern for BinaryOp nodes.
func (b *BinaryOp) Accept(v Visitor) string {
	return v.VisitBinaryOp(b)
}

// Visitor defines the interface for visiting AST nodes.
// This enables the visitor pattern for tree traversal and processing.
type Visitor interface {
	// VisitNumber processes a Number node and returns a string result.
	VisitNumber(n *Number) string
	// VisitBinaryOp processes a BinaryOp node and returns a string result.
	VisitBinaryOp(b *BinaryOp) string
}
