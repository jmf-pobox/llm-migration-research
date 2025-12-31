package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the Expr interface and its implementations (Number and BinaryOp).
 *
 * <p>Tests verify:
 * <ul>
 *   <li>Node creation with correct field values</li>
 *   <li>Immutability of all node types</li>
 *   <li>Equality and hashCode contracts</li>
 *   <li>String representation formatting</li>
 *   <li>Position tracking (line and column)</li>
 *   <li>Null safety and validation</li>
 * </ul>
 */
class ExprTest {

    // ==================== Number Tests ====================

    @Test
    @DisplayName("Number node creation with integer value")
    void testNumberCreationInteger() {
        Number num = new Number(1, 5, "42");

        assertEquals(1, num.getLine(), "Line should be 1");
        assertEquals(5, num.getColumn(), "Column should be 5");
        assertEquals("42", num.getValue(), "Value should be '42'");
    }

    @Test
    @DisplayName("Number node creation with decimal value")
    void testNumberCreationDecimal() {
        Number num = new Number(2, 10, "3.14");

        assertEquals(2, num.getLine(), "Line should be 2");
        assertEquals(10, num.getColumn(), "Column should be 10");
        assertEquals("3.14", num.getValue(), "Value should preserve decimal format");
    }

    @Test
    @DisplayName("Number node creation with negative value")
    void testNumberCreationNegative() {
        Number num = new Number(1, 1, "-5");

        assertEquals(1, num.getLine());
        assertEquals(1, num.getColumn());
        assertEquals("-5", num.getValue(), "Negative sign should be preserved");
    }

    @Test
    @DisplayName("Number node rejects null value")
    void testNumberNullValue() {
        assertThrows(NullPointerException.class, () -> {
            new Number(1, 1, null);
        }, "Should throw NullPointerException for null value");
    }

    @Test
    @DisplayName("Number equality - same values")
    void testNumberEquality() {
        Number num1 = new Number(1, 5, "42");
        Number num2 = new Number(1, 5, "42");

        assertEquals(num1, num2, "Numbers with same values should be equal");
        assertEquals(num1.hashCode(), num2.hashCode(), "Equal numbers should have same hashCode");
    }

    @Test
    @DisplayName("Number equality - different values")
    void testNumberInequality() {
        Number num1 = new Number(1, 5, "42");
        Number num2 = new Number(1, 5, "43");
        Number num3 = new Number(2, 5, "42");

        assertNotEquals(num1, num2, "Numbers with different values should not be equal");
        assertNotEquals(num1, num3, "Numbers with different positions should not be equal");
    }

    @Test
    @DisplayName("Number toString format")
    void testNumberToString() {
        Number num = new Number(1, 5, "42");
        String expected = "Number(1, 5, \"42\")";

        assertEquals(expected, num.toString(), "toString should match expected format");
    }

    @ParameterizedTest
    @CsvSource({
        "1, 1, '5'",
        "1, 3, '3.14'",
        "2, 10, '-10'",
        "5, 20, '0.5'"
    })
    @DisplayName("Number preserves various numeric formats")
    void testNumberFormats(int line, int column, String value) {
        Number num = new Number(line, column, value);

        assertEquals(line, num.getLine());
        assertEquals(column, num.getColumn());
        assertEquals(value, num.getValue());
    }

    // ==================== BinaryOp Tests ====================

    @Test
    @DisplayName("BinaryOp node creation with addition")
    void testBinaryOpCreationAddition() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp op = new BinaryOp(1, 5, "+", left, right);

        assertEquals(1, op.getLine());
        assertEquals(5, op.getColumn());
        assertEquals("+", op.getOperator());
        assertEquals(left, op.getLeft());
        assertEquals(right, op.getRight());
    }

    @ParameterizedTest
    @CsvSource({
        "'+', addition",
        "'-', subtraction",
        "'*', multiplication",
        "'/', division"
    })
    @DisplayName("BinaryOp supports all four operators")
    void testBinaryOpOperators(String operator, String description) {
        Number left = new Number(1, 1, "10");
        Number right = new Number(1, 4, "2");
        BinaryOp op = new BinaryOp(1, 6, operator, left, right);

        assertEquals(operator, op.getOperator(), "Operator should be " + description);
    }

    @Test
    @DisplayName("BinaryOp with nested operations")
    void testBinaryOpNested() {
        // Represents: (5 + 3) * 2
        Number five = new Number(1, 1, "5");
        Number three = new Number(1, 3, "3");
        BinaryOp addition = new BinaryOp(1, 5, "+", five, three);
        Number two = new Number(1, 8, "2");
        BinaryOp multiplication = new BinaryOp(1, 10, "*", addition, two);

        assertEquals("*", multiplication.getOperator());
        assertEquals(addition, multiplication.getLeft());
        assertEquals(two, multiplication.getRight());
        assertTrue(multiplication.getLeft() instanceof BinaryOp, "Left operand should be BinaryOp");
        assertTrue(multiplication.getRight() instanceof Number, "Right operand should be Number");
    }

    @Test
    @DisplayName("BinaryOp rejects null operator")
    void testBinaryOpNullOperator() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");

        assertThrows(NullPointerException.class, () -> {
            new BinaryOp(1, 5, null, left, right);
        }, "Should throw NullPointerException for null operator");
    }

    @Test
    @DisplayName("BinaryOp rejects null left operand")
    void testBinaryOpNullLeft() {
        Number right = new Number(1, 3, "3");

        assertThrows(NullPointerException.class, () -> {
            new BinaryOp(1, 5, "+", null, right);
        }, "Should throw NullPointerException for null left operand");
    }

    @Test
    @DisplayName("BinaryOp rejects null right operand")
    void testBinaryOpNullRight() {
        Number left = new Number(1, 1, "5");

        assertThrows(NullPointerException.class, () -> {
            new BinaryOp(1, 5, "+", left, null);
        }, "Should throw NullPointerException for null right operand");
    }

    @Test
    @DisplayName("BinaryOp equality - same structure")
    void testBinaryOpEquality() {
        Number left1 = new Number(1, 1, "5");
        Number right1 = new Number(1, 3, "3");
        BinaryOp op1 = new BinaryOp(1, 5, "+", left1, right1);

        Number left2 = new Number(1, 1, "5");
        Number right2 = new Number(1, 3, "3");
        BinaryOp op2 = new BinaryOp(1, 5, "+", left2, right2);

        assertEquals(op1, op2, "BinaryOps with same structure should be equal");
        assertEquals(op1.hashCode(), op2.hashCode(), "Equal BinaryOps should have same hashCode");
    }

    @Test
    @DisplayName("BinaryOp equality - different operators")
    void testBinaryOpInequalityDifferentOperator() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp op1 = new BinaryOp(1, 5, "+", left, right);
        BinaryOp op2 = new BinaryOp(1, 5, "-", left, right);

        assertNotEquals(op1, op2, "BinaryOps with different operators should not be equal");
    }

    @Test
    @DisplayName("BinaryOp equality - different operands")
    void testBinaryOpInequalityDifferentOperands() {
        Number left1 = new Number(1, 1, "5");
        Number right1 = new Number(1, 3, "3");
        Number left2 = new Number(1, 1, "10");
        Number right2 = new Number(1, 3, "3");

        BinaryOp op1 = new BinaryOp(1, 5, "+", left1, right1);
        BinaryOp op2 = new BinaryOp(1, 5, "+", left2, right2);

        assertNotEquals(op1, op2, "BinaryOps with different operands should not be equal");
    }

    @Test
    @DisplayName("BinaryOp toString format")
    void testBinaryOpToString() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp op = new BinaryOp(1, 5, "+", left, right);

        String expected = "BinaryOp(1, 5, \"+\", Number(1, 1, \"5\"), Number(1, 3, \"3\"))";
        assertEquals(expected, op.toString(), "toString should match expected format");
    }

    // ==================== Interface Tests ====================

    @Test
    @DisplayName("Number implements Expr interface")
    void testNumberImplementsExpr() {
        Number num = new Number(1, 5, "42");
        Expr expr = num;

        assertEquals(1, expr.getLine(), "Should access line through Expr interface");
        assertEquals(5, expr.getColumn(), "Should access column through Expr interface");
    }

    @Test
    @DisplayName("BinaryOp implements Expr interface")
    void testBinaryOpImplementsExpr() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp op = new BinaryOp(1, 5, "+", left, right);
        Expr expr = op;

        assertEquals(1, expr.getLine(), "Should access line through Expr interface");
        assertEquals(5, expr.getColumn(), "Should access column through Expr interface");
    }

    @Test
    @DisplayName("Expr can hold either Number or BinaryOp")
    void testExprPolymorphism() {
        Expr numExpr = new Number(1, 1, "42");
        Expr opExpr = new BinaryOp(1, 5, "+",
            new Number(1, 1, "5"),
            new Number(1, 3, "3"));

        assertTrue(numExpr instanceof Number, "Expr should hold Number");
        assertTrue(opExpr instanceof BinaryOp, "Expr should hold BinaryOp");
    }

    // ==================== Position Tracking Tests ====================

    @Test
    @DisplayName("Position tracking is accurate for nested expressions")
    void testPositionTrackingNested() {
        // Test that each node maintains its own position correctly
        Number n1 = new Number(1, 1, "5");
        Number n2 = new Number(1, 5, "3");
        BinaryOp inner = new BinaryOp(1, 7, "+", n1, n2);
        Number n3 = new Number(2, 1, "2");
        BinaryOp outer = new BinaryOp(2, 3, "*", inner, n3);

        assertEquals(1, n1.getLine());
        assertEquals(1, n1.getColumn());
        assertEquals(1, n2.getLine());
        assertEquals(5, n2.getColumn());
        assertEquals(1, inner.getLine());
        assertEquals(7, inner.getColumn());
        assertEquals(2, n3.getLine());
        assertEquals(1, n3.getColumn());
        assertEquals(2, outer.getLine());
        assertEquals(3, outer.getColumn());
    }

    // ==================== Edge Cases ====================

    @Test
    @DisplayName("Number with empty string value")
    void testNumberEmptyString() {
        // While semantically questionable, we should allow empty string
        Number num = new Number(1, 1, "");
        assertEquals("", num.getValue());
    }

    @Test
    @DisplayName("Number and BinaryOp are not equal")
    void testNumberBinaryOpInequality() {
        Number num = new Number(1, 5, "42");
        BinaryOp op = new BinaryOp(1, 5, "+",
            new Number(1, 1, "1"),
            new Number(1, 3, "2"));

        assertNotEquals(num, op, "Number and BinaryOp should never be equal");
        assertNotEquals(op, num, "BinaryOp and Number should never be equal");
    }
}
