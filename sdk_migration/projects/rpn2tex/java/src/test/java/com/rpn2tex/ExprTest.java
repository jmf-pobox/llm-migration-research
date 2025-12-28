package com.rpn2tex;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the AST classes (Expr, Number, BinaryOp).
 */
class ExprTest {

    @Test
    void testNumberCreation() {
        Number num = new Number(1, 1, "42");
        assertEquals(1, num.line());
        assertEquals(1, num.column());
        assertEquals("42", num.value());
    }

    @Test
    void testNumberWithDecimal() {
        Number num = new Number(1, 5, "3.14");
        assertEquals(1, num.line());
        assertEquals(5, num.column());
        assertEquals("3.14", num.value());
    }

    @Test
    void testNumberNullValueThrows() {
        assertThrows(NullPointerException.class, () -> {
            new Number(1, 1, null);
        });
    }

    @Test
    void testNumberInvalidLineThrows() {
        assertThrows(IllegalArgumentException.class, () -> {
            new Number(0, 1, "42");
        });
    }

    @Test
    void testNumberInvalidColumnThrows() {
        assertThrows(IllegalArgumentException.class, () -> {
            new Number(1, 0, "42");
        });
    }

    @Test
    void testBinaryOpCreation() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp add = new BinaryOp(1, 3, "+", left, right);

        assertEquals(1, add.line());
        assertEquals(3, add.column());
        assertEquals("+", add.operator());
        assertEquals(left, add.left());
        assertEquals(right, add.right());
    }

    @Test
    void testBinaryOpWithAllOperators() {
        Number left = new Number(1, 1, "10");
        Number right = new Number(1, 4, "2");

        BinaryOp add = new BinaryOp(1, 4, "+", left, right);
        assertEquals("+", add.operator());

        BinaryOp sub = new BinaryOp(1, 4, "-", left, right);
        assertEquals("-", sub.operator());

        BinaryOp mult = new BinaryOp(1, 4, "*", left, right);
        assertEquals("*", mult.operator());

        BinaryOp div = new BinaryOp(1, 4, "/", left, right);
        assertEquals("/", div.operator());
    }

    @Test
    void testBinaryOpNested() {
        // Represents: (5 + 3) * 2
        Number five = new Number(1, 1, "5");
        Number three = new Number(1, 3, "3");
        Number two = new Number(1, 7, "2");

        BinaryOp add = new BinaryOp(1, 3, "+", five, three);
        BinaryOp mult = new BinaryOp(1, 7, "*", add, two);

        assertEquals("*", mult.operator());
        assertTrue(mult.left() instanceof BinaryOp);
        assertTrue(mult.right() instanceof Number);

        BinaryOp leftOp = (BinaryOp) mult.left();
        assertEquals("+", leftOp.operator());
    }

    @Test
    void testBinaryOpNullOperatorThrows() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");

        assertThrows(NullPointerException.class, () -> {
            new BinaryOp(1, 3, null, left, right);
        });
    }

    @Test
    void testBinaryOpNullLeftThrows() {
        Number right = new Number(1, 3, "3");

        assertThrows(NullPointerException.class, () -> {
            new BinaryOp(1, 3, "+", null, right);
        });
    }

    @Test
    void testBinaryOpNullRightThrows() {
        Number left = new Number(1, 1, "5");

        assertThrows(NullPointerException.class, () -> {
            new BinaryOp(1, 3, "+", left, null);
        });
    }

    @Test
    void testBinaryOpInvalidLineThrows() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");

        assertThrows(IllegalArgumentException.class, () -> {
            new BinaryOp(0, 3, "+", left, right);
        });
    }

    @Test
    void testBinaryOpInvalidColumnThrows() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");

        assertThrows(IllegalArgumentException.class, () -> {
            new BinaryOp(1, 0, "+", left, right);
        });
    }

    @Test
    void testExprInterfaceMethods() {
        // Test that both Number and BinaryOp properly implement Expr interface
        Number num = new Number(2, 5, "42");
        Expr numExpr = num;
        assertEquals(2, numExpr.line());
        assertEquals(5, numExpr.column());

        Number left = new Number(3, 1, "5");
        Number right = new Number(3, 3, "3");
        BinaryOp op = new BinaryOp(3, 3, "+", left, right);
        Expr opExpr = op;
        assertEquals(3, opExpr.line());
        assertEquals(3, opExpr.column());
    }

    @Test
    void testNumberEquality() {
        Number num1 = new Number(1, 1, "42");
        Number num2 = new Number(1, 1, "42");
        Number num3 = new Number(1, 1, "43");

        assertEquals(num1, num2);
        assertNotEquals(num1, num3);
    }

    @Test
    void testBinaryOpEquality() {
        Number left1 = new Number(1, 1, "5");
        Number right1 = new Number(1, 3, "3");
        BinaryOp op1 = new BinaryOp(1, 3, "+", left1, right1);

        Number left2 = new Number(1, 1, "5");
        Number right2 = new Number(1, 3, "3");
        BinaryOp op2 = new BinaryOp(1, 3, "+", left2, right2);

        Number left3 = new Number(1, 1, "5");
        Number right3 = new Number(1, 3, "3");
        BinaryOp op3 = new BinaryOp(1, 3, "-", left3, right3);

        assertEquals(op1, op2);
        assertNotEquals(op1, op3);
    }

    @Test
    void testNumberToString() {
        Number num = new Number(1, 5, "42");
        String str = num.toString();
        assertTrue(str.contains("42"));
        assertTrue(str.contains("line=1"));
        assertTrue(str.contains("column=5"));
    }

    @Test
    void testBinaryOpToString() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp op = new BinaryOp(1, 3, "+", left, right);
        String str = op.toString();
        assertTrue(str.contains("+"));
        assertTrue(str.contains("line=1"));
        assertTrue(str.contains("column=3"));
    }
}
