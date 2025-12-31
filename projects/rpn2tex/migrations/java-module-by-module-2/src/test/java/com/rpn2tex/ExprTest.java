package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the Expr AST node hierarchy.
 *
 * <p>Tests cover all implementations of the {@link Expr} interface:
 * {@link Number} and {@link BinaryOp}, including validation, immutability,
 * and the sealed interface contract.</p>
 */
class ExprTest {

    // ========== Number Tests ==========

    @Test
    @DisplayName("Number: Create valid number with integer value")
    void testNumberCreationInteger() {
        Number num = new Number(1, 5, "42");

        assertEquals(1, num.line());
        assertEquals(5, num.column());
        assertEquals("42", num.value());
        assertEquals("42", num.getValue());
    }

    @Test
    @DisplayName("Number: Create valid number with decimal value")
    void testNumberCreationDecimal() {
        Number num = new Number(1, 1, "3.14");

        assertEquals(1, num.line());
        assertEquals(1, num.column());
        assertEquals("3.14", num.value());
        assertEquals("3.14", num.getValue());
    }

    @Test
    @DisplayName("Number: Create valid number with negative value")
    void testNumberCreationNegative() {
        Number num = new Number(2, 3, "-5");

        assertEquals(2, num.line());
        assertEquals(3, num.column());
        assertEquals("-5", num.value());
    }

    @ParameterizedTest
    @ValueSource(ints = {0, -1, -10, Integer.MIN_VALUE})
    @DisplayName("Number: Reject invalid line numbers")
    void testNumberInvalidLine(int invalidLine) {
        assertThrows(IllegalArgumentException.class, () -> {
            new Number(invalidLine, 1, "42");
        });
    }

    @ParameterizedTest
    @ValueSource(ints = {0, -1, -10, Integer.MIN_VALUE})
    @DisplayName("Number: Reject invalid column numbers")
    void testNumberInvalidColumn(int invalidColumn) {
        assertThrows(IllegalArgumentException.class, () -> {
            new Number(1, invalidColumn, "42");
        });
    }

    @Test
    @DisplayName("Number: Reject null value")
    void testNumberNullValue() {
        assertThrows(NullPointerException.class, () -> {
            new Number(1, 1, null);
        });
    }

    @Test
    @DisplayName("Number: Implements Expr interface")
    void testNumberImplementsExpr() {
        Number num = new Number(1, 1, "5");
        assertTrue(num instanceof Expr);
    }

    @Test
    @DisplayName("Number: Records are immutable (equals/hashCode)")
    void testNumberImmutability() {
        Number num1 = new Number(1, 5, "42");
        Number num2 = new Number(1, 5, "42");
        Number num3 = new Number(1, 5, "43");

        // Same values should be equal
        assertEquals(num1, num2);
        assertEquals(num1.hashCode(), num2.hashCode());

        // Different values should not be equal
        assertNotEquals(num1, num3);
    }

    // ========== BinaryOp Tests ==========

    @Test
    @DisplayName("BinaryOp: Create valid addition operation")
    void testBinaryOpCreationAddition() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp op = new BinaryOp(1, 3, "+", left, right);

        assertEquals(1, op.line());
        assertEquals(3, op.column());
        assertEquals("+", op.operator());
        assertEquals("+", op.getOperator());
        assertEquals(left, op.left());
        assertEquals(left, op.getLeft());
        assertEquals(right, op.right());
        assertEquals(right, op.getRight());
    }

    @Test
    @DisplayName("BinaryOp: Create valid multiplication operation")
    void testBinaryOpCreationMultiplication() {
        Number left = new Number(1, 1, "4");
        Number right = new Number(1, 3, "7");
        BinaryOp op = new BinaryOp(1, 3, "*", left, right);

        assertEquals("*", op.operator());
    }

    @Test
    @DisplayName("BinaryOp: Create nested operation (recursive structure)")
    void testBinaryOpNested() {
        // Create: (5 + 3) * 2
        Number five = new Number(1, 1, "5");
        Number three = new Number(1, 3, "3");
        Number two = new Number(1, 7, "2");

        BinaryOp addition = new BinaryOp(1, 3, "+", five, three);
        BinaryOp multiplication = new BinaryOp(1, 7, "*", addition, two);

        assertEquals("*", multiplication.operator());
        assertTrue(multiplication.left() instanceof BinaryOp);
        assertTrue(multiplication.right() instanceof Number);

        BinaryOp innerOp = (BinaryOp) multiplication.left();
        assertEquals("+", innerOp.operator());
        assertEquals("5", ((Number) innerOp.left()).value());
        assertEquals("3", ((Number) innerOp.right()).value());
    }

    @ParameterizedTest
    @ValueSource(ints = {0, -1, -10, Integer.MIN_VALUE})
    @DisplayName("BinaryOp: Reject invalid line numbers")
    void testBinaryOpInvalidLine(int invalidLine) {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");

        assertThrows(IllegalArgumentException.class, () -> {
            new BinaryOp(invalidLine, 1, "+", left, right);
        });
    }

    @ParameterizedTest
    @ValueSource(ints = {0, -1, -10, Integer.MIN_VALUE})
    @DisplayName("BinaryOp: Reject invalid column numbers")
    void testBinaryOpInvalidColumn(int invalidColumn) {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");

        assertThrows(IllegalArgumentException.class, () -> {
            new BinaryOp(1, invalidColumn, "+", left, right);
        });
    }

    @Test
    @DisplayName("BinaryOp: Reject null operator")
    void testBinaryOpNullOperator() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");

        assertThrows(NullPointerException.class, () -> {
            new BinaryOp(1, 3, null, left, right);
        });
    }

    @Test
    @DisplayName("BinaryOp: Reject null left operand")
    void testBinaryOpNullLeft() {
        Number right = new Number(1, 3, "3");

        assertThrows(NullPointerException.class, () -> {
            new BinaryOp(1, 3, "+", null, right);
        });
    }

    @Test
    @DisplayName("BinaryOp: Reject null right operand")
    void testBinaryOpNullRight() {
        Number left = new Number(1, 1, "5");

        assertThrows(NullPointerException.class, () -> {
            new BinaryOp(1, 3, "+", left, null);
        });
    }

    @Test
    @DisplayName("BinaryOp: Implements Expr interface")
    void testBinaryOpImplementsExpr() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp op = new BinaryOp(1, 3, "+", left, right);

        assertTrue(op instanceof Expr);
    }

    @Test
    @DisplayName("BinaryOp: Records are immutable (equals/hashCode)")
    void testBinaryOpImmutability() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");

        BinaryOp op1 = new BinaryOp(1, 3, "+", left, right);
        BinaryOp op2 = new BinaryOp(1, 3, "+", left, right);
        BinaryOp op3 = new BinaryOp(1, 3, "-", left, right);

        // Same values should be equal
        assertEquals(op1, op2);
        assertEquals(op1.hashCode(), op2.hashCode());

        // Different operators should not be equal
        assertNotEquals(op1, op3);
    }

    // ========== Sealed Interface Tests ==========

    @Test
    @DisplayName("Expr: Sealed interface permits only Number and BinaryOp")
    void testExprSealedInterface() {
        // This test verifies that the sealed interface works correctly
        Expr numberExpr = new Number(1, 1, "5");
        Expr binaryOpExpr = new BinaryOp(1, 3, "+",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );

        // Verify types using instanceof
        assertTrue(numberExpr instanceof Number);
        assertFalse(numberExpr instanceof BinaryOp);

        assertTrue(binaryOpExpr instanceof BinaryOp);
        assertFalse(binaryOpExpr instanceof Number);

        // Verify both implement Expr
        assertTrue(numberExpr instanceof Expr);
        assertTrue(binaryOpExpr instanceof Expr);
    }

    // ========== I/O Contract Tests ==========

    @Test
    @DisplayName("I/O Contract: Construct AST for '5 3 +'")
    void testIOContractSimpleAddition() {
        // RPN: "5 3 +"
        // Expected AST: BinaryOp("+", Number("5"), Number("3"))

        Number five = new Number(1, 1, "5");
        Number three = new Number(1, 3, "3");
        BinaryOp addition = new BinaryOp(1, 5, "+", five, three);

        assertEquals("+", addition.operator());
        assertEquals("5", ((Number) addition.left()).value());
        assertEquals("3", ((Number) addition.right()).value());
    }

    @Test
    @DisplayName("I/O Contract: Construct AST for '5 3 + 2 *'")
    void testIOContractComplexExpression() {
        // RPN: "5 3 + 2 *"
        // Expected AST: BinaryOp("*", BinaryOp("+", Number("5"), Number("3")), Number("2"))

        Number five = new Number(1, 1, "5");
        Number three = new Number(1, 3, "3");
        Number two = new Number(1, 7, "2");

        BinaryOp innerAdd = new BinaryOp(1, 5, "+", five, three);
        BinaryOp outerMult = new BinaryOp(1, 9, "*", innerAdd, two);

        assertEquals("*", outerMult.operator());
        assertTrue(outerMult.left() instanceof BinaryOp);

        BinaryOp innerOp = (BinaryOp) outerMult.left();
        assertEquals("+", innerOp.operator());
        assertEquals("5", ((Number) innerOp.left()).value());
        assertEquals("3", ((Number) innerOp.right()).value());
        assertEquals("2", ((Number) outerMult.right()).value());
    }

    @Test
    @DisplayName("I/O Contract: Construct AST with floating point numbers")
    void testIOContractFloatingPoint() {
        // RPN: "3.14 2 *"

        Number pi = new Number(1, 1, "3.14");
        Number two = new Number(1, 6, "2");
        BinaryOp mult = new BinaryOp(1, 8, "*", pi, two);

        assertEquals("3.14", ((Number) mult.left()).value());
        assertEquals("2", ((Number) mult.right()).value());
    }

    @Test
    @DisplayName("I/O Contract: All operators supported")
    void testIOContractAllOperators() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");

        BinaryOp addition = new BinaryOp(1, 3, "+", left, right);
        BinaryOp subtraction = new BinaryOp(1, 3, "-", left, right);
        BinaryOp multiplication = new BinaryOp(1, 3, "*", left, right);
        BinaryOp division = new BinaryOp(1, 3, "/", left, right);

        assertEquals("+", addition.operator());
        assertEquals("-", subtraction.operator());
        assertEquals("*", multiplication.operator());
        assertEquals("/", division.operator());
    }
}
