package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for Expr, Number, and BinaryOp AST node classes.
 */
@DisplayName("Expr AST Node Tests")
class ExprTest {

    @Test
    @DisplayName("Number node creation")
    void testNumberCreation() {
        Number num = new Number("42", 1, 5);
        assertEquals("42", num.value());
        assertEquals(1, num.line());
        assertEquals(5, num.column());
    }

    @Test
    @DisplayName("Number node equality")
    void testNumberEquality() {
        Number num1 = new Number("42", 1, 5);
        Number num2 = new Number("42", 1, 5);
        assertEquals(num1, num2);
        assertEquals(num1.hashCode(), num2.hashCode());
    }

    @Test
    @DisplayName("Number rejects null value")
    void testNumberNullValue() {
        assertThrows(NullPointerException.class, () -> new Number(null, 1, 1));
    }

    @Test
    @DisplayName("Number rejects invalid line")
    void testNumberInvalidLine() {
        assertThrows(IllegalArgumentException.class, () -> new Number("42", 0, 1));
    }

    @Test
    @DisplayName("BinaryOp creation")
    void testBinaryOpCreation() {
        Expr left = new Number("5", 1, 1);
        Expr right = new Number("3", 1, 3);
        BinaryOp op = new BinaryOp("+", left, right, 1, 5);
        
        assertEquals("+", op.operator());
        assertSame(left, op.left());
        assertSame(right, op.right());
        assertEquals(1, op.line());
        assertEquals(5, op.column());
    }

    @Test
    @DisplayName("BinaryOp rejects null operator")
    void testBinaryOpNullOperator() {
        Expr left = new Number("5", 1, 1);
        Expr right = new Number("3", 1, 3);
        assertThrows(NullPointerException.class, () -> new BinaryOp(null, left, right, 1, 5));
    }

    @Test
    @DisplayName("BinaryOp rejects null operands")
    void testBinaryOpNullOperands() {
        Expr num = new Number("5", 1, 1);
        assertThrows(NullPointerException.class, () -> new BinaryOp("+", null, num, 1, 5));
        assertThrows(NullPointerException.class, () -> new BinaryOp("+", num, null, 1, 5));
    }

    @Test
    @DisplayName("Nested BinaryOp tree structure")
    void testNestedBinaryOp() {
        Expr five = new Number("5", 1, 1);
        Expr three = new Number("3", 1, 3);
        Expr add = new BinaryOp("+", five, three, 1, 5);
        Expr two = new Number("2", 1, 7);
        Expr mult = new BinaryOp("*", add, two, 1, 9);
        
        assertInstanceOf(BinaryOp.class, mult);
        BinaryOp multOp = (BinaryOp) mult;
        assertInstanceOf(BinaryOp.class, multOp.left());
        assertInstanceOf(Number.class, multOp.right());
    }

    @Test
    @DisplayName("Expr interface polymorphism")
    void testExprInterface() {
        Expr num = new Number("42", 1, 1);
        Expr op = new BinaryOp("+", num, num, 1, 3);
        
        // Both implement Expr
        assertInstanceOf(Expr.class, num);
        assertInstanceOf(Expr.class, op);
        
        // Concrete types work
        assertInstanceOf(Number.class, num);
        assertInstanceOf(BinaryOp.class, op);
    }

    @Test
    @DisplayName("Number toString format")
    void testNumberToString() {
        Number num = new Number("3.14", 2, 10);
        String str = num.toString();
        assertTrue(str.contains("Number"));
        assertTrue(str.contains("3.14"));
    }

    @Test
    @DisplayName("BinaryOp toString format")
    void testBinaryOpToString() {
        Expr left = new Number("5", 1, 1);
        Expr right = new Number("3", 1, 3);
        BinaryOp op = new BinaryOp("+", left, right, 1, 5);
        String str = op.toString();
        assertTrue(str.contains("BinaryOp"));
        assertTrue(str.contains("+"));
    }
}
