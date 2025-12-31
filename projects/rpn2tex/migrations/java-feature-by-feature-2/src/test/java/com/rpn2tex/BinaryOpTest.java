package com.rpn2tex;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Tests for the BinaryOp record.
 */
class BinaryOpTest {

    @Test
    void testBinaryOpCreation() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp binOp = new BinaryOp(1, 5, "+", left, right);

        assertEquals(1, binOp.line());
        assertEquals(5, binOp.column());
        assertEquals("+", binOp.operator());
        assertEquals(left, binOp.left());
        assertEquals(right, binOp.right());
    }

    @Test
    void testBinaryOpImplementsExpr() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp binOp = new BinaryOp(1, 5, "+", left, right);

        assertInstanceOf(Expr.class, binOp);
    }

    @Test
    void testBinaryOpEquality() {
        Number left1 = new Number(1, 1, "5");
        Number right1 = new Number(1, 3, "3");
        BinaryOp binOp1 = new BinaryOp(1, 5, "+", left1, right1);

        Number left2 = new Number(1, 1, "5");
        Number right2 = new Number(1, 3, "3");
        BinaryOp binOp2 = new BinaryOp(1, 5, "+", left2, right2);

        assertEquals(binOp1, binOp2);
        assertEquals(binOp1.hashCode(), binOp2.hashCode());
    }

    @Test
    void testBinaryOpToString() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp binOp = new BinaryOp(1, 5, "+", left, right);

        String str = binOp.toString();
        assertTrue(str.contains("+"));
        assertTrue(str.contains("5"));
        assertTrue(str.contains("3"));
    }

    @Test
    void testNestedBinaryOp() {
        Number num1 = new Number(1, 1, "1");
        Number num2 = new Number(1, 3, "2");
        BinaryOp inner = new BinaryOp(1, 5, "+", num1, num2);

        Number num3 = new Number(1, 7, "3");
        BinaryOp outer = new BinaryOp(1, 9, "+", inner, num3);

        assertEquals("+", outer.operator());
        assertEquals(inner, outer.left());
        assertEquals(num3, outer.right());
        assertInstanceOf(BinaryOp.class, outer.left());
        assertInstanceOf(Number.class, outer.right());
    }
}
