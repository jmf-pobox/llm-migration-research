package com.rpn2tex;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the {@link Parser} class.
 *
 * <p>Tests cover:
 * <ul>
 *   <li>Basic RPN parsing (single operations)</li>
 *   <li>Complex nested expressions</li>
 *   <li>Stack validation (empty expressions, missing operators)</li>
 *   <li>Error handling (insufficient operands, extra operands)</li>
 *   <li>All four operators (+, -, *, /)</li>
 *   <li>I/O contract test cases</li>
 * </ul>
 */
class ParserTest {

    @Test
    void testEmptyExpressionThrowsException() {
        // Only EOF token, no operands
        List<Token> tokens = List.of(
            new Token(TokenType.EOF, "", 1, 1)
        );

        Parser parser = new Parser(tokens);
        RpnException exception = assertThrows(RpnException.class, parser::parse);

        assertTrue(exception.getMessage().contains("Empty expression"));
        assertEquals(1, exception.getLine());
        assertEquals(1, exception.getColumn());
    }

    @Test
    void testSingleNumber() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "42", 1, 1),
            new Token(TokenType.EOF, "", 1, 3)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(Number.class, result);
        Number num = (Number) result;
        assertEquals("42", num.getValue());
        assertEquals(1, num.line());
        assertEquals(1, num.column());
    }

    @Test
    void testSimpleAddition() throws RpnException {
        // "5 3 +" -> BinaryOp(+, Number(5), Number(3))
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.PLUS, "+", 1, 5),
            new Token(TokenType.EOF, "", 1, 6)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("+", op.getOperator());
        assertEquals(1, op.line());
        assertEquals(5, op.column());

        assertInstanceOf(Number.class, op.getLeft());
        Number left = (Number) op.getLeft();
        assertEquals("5", left.getValue());

        assertInstanceOf(Number.class, op.getRight());
        Number right = (Number) op.getRight();
        assertEquals("3", right.getValue());
    }

    @Test
    void testSimpleSubtraction() throws RpnException {
        // "5 3 -"
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.MINUS, "-", 1, 5),
            new Token(TokenType.EOF, "", 1, 6)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("-", op.getOperator());
    }

    @Test
    void testSimpleMultiplication() throws RpnException {
        // "4 7 *"
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "4", 1, 1),
            new Token(TokenType.NUMBER, "7", 1, 3),
            new Token(TokenType.MULT, "*", 1, 5),
            new Token(TokenType.EOF, "", 1, 6)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("*", op.getOperator());
    }

    @Test
    void testSimpleDivision() throws RpnException {
        // "10 2 /"
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "10", 1, 1),
            new Token(TokenType.NUMBER, "2", 1, 4),
            new Token(TokenType.DIV, "/", 1, 6),
            new Token(TokenType.EOF, "", 1, 7)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("/", op.getOperator());
    }

    @Test
    void testNestedExpressionAddThenMultiply() throws RpnException {
        // "5 3 + 2 *" -> BinaryOp(*, BinaryOp(+, 5, 3), 2)
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.PLUS, "+", 1, 5),
            new Token(TokenType.NUMBER, "2", 1, 7),
            new Token(TokenType.MULT, "*", 1, 9),
            new Token(TokenType.EOF, "", 1, 10)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("*", op.getOperator());

        // Left should be BinaryOp(+, 5, 3)
        assertInstanceOf(BinaryOp.class, op.getLeft());
        BinaryOp leftOp = (BinaryOp) op.getLeft();
        assertEquals("+", leftOp.getOperator());

        assertInstanceOf(Number.class, leftOp.getLeft());
        assertEquals("5", ((Number) leftOp.getLeft()).getValue());

        assertInstanceOf(Number.class, leftOp.getRight());
        assertEquals("3", ((Number) leftOp.getRight()).getValue());

        // Right should be Number(2)
        assertInstanceOf(Number.class, op.getRight());
        assertEquals("2", ((Number) op.getRight()).getValue());
    }

    @Test
    void testNestedExpressionMultiplyThenAdd() throws RpnException {
        // "5 3 * 2 +" -> BinaryOp(+, BinaryOp(*, 5, 3), 2)
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.MULT, "*", 1, 5),
            new Token(TokenType.NUMBER, "2", 1, 7),
            new Token(TokenType.PLUS, "+", 1, 9),
            new Token(TokenType.EOF, "", 1, 10)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("+", op.getOperator());

        // Left should be BinaryOp(*, 5, 3)
        assertInstanceOf(BinaryOp.class, op.getLeft());
        BinaryOp leftOp = (BinaryOp) op.getLeft();
        assertEquals("*", leftOp.getOperator());
    }

    @Test
    void testMultipleOperationsThreeAdditions() throws RpnException {
        // "1 2 + 3 + 4 +" -> ((1 + 2) + 3) + 4
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "1", 1, 1),
            new Token(TokenType.NUMBER, "2", 1, 3),
            new Token(TokenType.PLUS, "+", 1, 5),
            new Token(TokenType.NUMBER, "3", 1, 7),
            new Token(TokenType.PLUS, "+", 1, 9),
            new Token(TokenType.NUMBER, "4", 1, 11),
            new Token(TokenType.PLUS, "+", 1, 13),
            new Token(TokenType.EOF, "", 1, 14)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("+", op.getOperator());

        // Left should be nested BinaryOp
        assertInstanceOf(BinaryOp.class, op.getLeft());
        BinaryOp leftOp = (BinaryOp) op.getLeft();
        assertEquals("+", leftOp.getOperator());

        // Right should be Number(4)
        assertInstanceOf(Number.class, op.getRight());
        assertEquals("4", ((Number) op.getRight()).getValue());
    }

    @Test
    void testComplexExpressionNestedPrecedence() throws RpnException {
        // "2 3 4 * +" -> BinaryOp(+, Number(2), BinaryOp(*, Number(3), Number(4)))
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "2", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.NUMBER, "4", 1, 5),
            new Token(TokenType.MULT, "*", 1, 7),
            new Token(TokenType.PLUS, "+", 1, 9),
            new Token(TokenType.EOF, "", 1, 10)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("+", op.getOperator());

        // Left should be Number(2)
        assertInstanceOf(Number.class, op.getLeft());
        assertEquals("2", ((Number) op.getLeft()).getValue());

        // Right should be BinaryOp(*, 3, 4)
        assertInstanceOf(BinaryOp.class, op.getRight());
        BinaryOp rightOp = (BinaryOp) op.getRight();
        assertEquals("*", rightOp.getOperator());
        assertEquals("3", ((Number) rightOp.getLeft()).getValue());
        assertEquals("4", ((Number) rightOp.getRight()).getValue());
    }

    @Test
    void testFloatingPointNumbers() throws RpnException {
        // "3.14 2 *"
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "3.14", 1, 1),
            new Token(TokenType.NUMBER, "2", 1, 6),
            new Token(TokenType.MULT, "*", 1, 8),
            new Token(TokenType.EOF, "", 1, 9)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("*", op.getOperator());

        assertInstanceOf(Number.class, op.getLeft());
        assertEquals("3.14", ((Number) op.getLeft()).getValue());

        assertInstanceOf(Number.class, op.getRight());
        assertEquals("2", ((Number) op.getRight()).getValue());
    }

    @Test
    void testInsufficientOperandsOperatorWithNoOperands() {
        // Just "+" with no operands
        List<Token> tokens = List.of(
            new Token(TokenType.PLUS, "+", 1, 1),
            new Token(TokenType.EOF, "", 1, 2)
        );

        Parser parser = new Parser(tokens);
        RpnException exception = assertThrows(RpnException.class, parser::parse);

        assertTrue(exception.getMessage().contains("Operator '+' requires two operands"));
        assertEquals(1, exception.getLine());
        assertEquals(1, exception.getColumn());
    }

    @Test
    void testInsufficientOperandsOperatorWithOneOperand() {
        // "5 +" - only one operand for plus
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.PLUS, "+", 1, 3),
            new Token(TokenType.EOF, "", 1, 4)
        );

        Parser parser = new Parser(tokens);
        RpnException exception = assertThrows(RpnException.class, parser::parse);

        assertTrue(exception.getMessage().contains("Operator '+' requires two operands"));
        assertEquals(1, exception.getLine());
        assertEquals(3, exception.getColumn());
    }

    @Test
    void testExtraOperandsTwoNumbersNoOperator() {
        // "5 3" - missing operator
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.EOF, "", 1, 4)
        );

        Parser parser = new Parser(tokens);
        RpnException exception = assertThrows(RpnException.class, parser::parse);

        assertTrue(exception.getMessage().contains("Invalid RPN: 2 values remain on stack"));
        assertTrue(exception.getMessage().contains("missing operators?"));
    }

    @Test
    void testExtraOperandsThreeNumbersOneOperator() {
        // "5 3 2 +" - one number too many
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.NUMBER, "2", 1, 5),
            new Token(TokenType.PLUS, "+", 1, 7),
            new Token(TokenType.EOF, "", 1, 8)
        );

        Parser parser = new Parser(tokens);
        RpnException exception = assertThrows(RpnException.class, parser::parse);

        assertTrue(exception.getMessage().contains("Invalid RPN: 2 values remain on stack"));
    }

    @Test
    void testMultipleSubtractions() throws RpnException {
        // "5 3 - 2 -" -> (5 - 3) - 2
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.MINUS, "-", 1, 5),
            new Token(TokenType.NUMBER, "2", 1, 7),
            new Token(TokenType.MINUS, "-", 1, 9),
            new Token(TokenType.EOF, "", 1, 10)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("-", op.getOperator());

        // Left should be BinaryOp(-, 5, 3)
        assertInstanceOf(BinaryOp.class, op.getLeft());
        BinaryOp leftOp = (BinaryOp) op.getLeft();
        assertEquals("-", leftOp.getOperator());
    }

    @Test
    void testMultipleDivisions() throws RpnException {
        // "100 10 / 5 / 2 /" -> ((100 / 10) / 5) / 2
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "100", 1, 1),
            new Token(TokenType.NUMBER, "10", 1, 5),
            new Token(TokenType.DIV, "/", 1, 8),
            new Token(TokenType.NUMBER, "5", 1, 10),
            new Token(TokenType.DIV, "/", 1, 12),
            new Token(TokenType.NUMBER, "2", 1, 14),
            new Token(TokenType.DIV, "/", 1, 16),
            new Token(TokenType.EOF, "", 1, 17)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("/", op.getOperator());

        // Left should be nested divisions
        assertInstanceOf(BinaryOp.class, op.getLeft());
        BinaryOp leftOp = (BinaryOp) op.getLeft();
        assertEquals("/", leftOp.getOperator());
    }

    @Test
    void testComplexTwoSubexpressions() throws RpnException {
        // "1 2 + 3 4 + *" -> (1 + 2) * (3 + 4)
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "1", 1, 1),
            new Token(TokenType.NUMBER, "2", 1, 3),
            new Token(TokenType.PLUS, "+", 1, 5),
            new Token(TokenType.NUMBER, "3", 1, 7),
            new Token(TokenType.NUMBER, "4", 1, 9),
            new Token(TokenType.PLUS, "+", 1, 11),
            new Token(TokenType.MULT, "*", 1, 13),
            new Token(TokenType.EOF, "", 1, 14)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("*", op.getOperator());

        // Left should be BinaryOp(+, 1, 2)
        assertInstanceOf(BinaryOp.class, op.getLeft());
        BinaryOp leftOp = (BinaryOp) op.getLeft();
        assertEquals("+", leftOp.getOperator());
        assertEquals("1", ((Number) leftOp.getLeft()).getValue());
        assertEquals("2", ((Number) leftOp.getRight()).getValue());

        // Right should be BinaryOp(+, 3, 4)
        assertInstanceOf(BinaryOp.class, op.getRight());
        BinaryOp rightOp = (BinaryOp) op.getRight();
        assertEquals("+", rightOp.getOperator());
        assertEquals("3", ((Number) rightOp.getLeft()).getValue());
        assertEquals("4", ((Number) rightOp.getRight()).getValue());
    }

    @Test
    void testComplexMixedOperations() throws RpnException {
        // "10 2 / 3 + 4 *" -> ((10 / 2) + 3) * 4
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "10", 1, 1),
            new Token(TokenType.NUMBER, "2", 1, 4),
            new Token(TokenType.DIV, "/", 1, 6),
            new Token(TokenType.NUMBER, "3", 1, 8),
            new Token(TokenType.PLUS, "+", 1, 10),
            new Token(TokenType.NUMBER, "4", 1, 12),
            new Token(TokenType.MULT, "*", 1, 14),
            new Token(TokenType.EOF, "", 1, 15)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("*", op.getOperator());

        // Left should be BinaryOp(+, BinaryOp(/, 10, 2), 3)
        assertInstanceOf(BinaryOp.class, op.getLeft());
        BinaryOp leftOp = (BinaryOp) op.getLeft();
        assertEquals("+", leftOp.getOperator());

        assertInstanceOf(BinaryOp.class, leftOp.getLeft());
        BinaryOp divOp = (BinaryOp) leftOp.getLeft();
        assertEquals("/", divOp.getOperator());
    }

    @Test
    void testNullTokenListThrowsException() {
        assertThrows(NullPointerException.class, () -> new Parser(null));
    }

    @Test
    void testContractCase53Plus() throws RpnException {
        // I/O contract: "5 3 +" -> BinaryOp(+, Number(5), Number(3))
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.PLUS, "+", 1, 5),
            new Token(TokenType.EOF, "", 1, 6)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("+", op.getOperator());
        assertEquals("5", ((Number) op.getLeft()).getValue());
        assertEquals("3", ((Number) op.getRight()).getValue());
    }

    @Test
    void testContractCase53Minus() throws RpnException {
        // I/O contract: "5 3 -"
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.MINUS, "-", 1, 5),
            new Token(TokenType.EOF, "", 1, 6)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("-", op.getOperator());
    }

    @Test
    void testContractCase47Mult() throws RpnException {
        // I/O contract: "4 7 *"
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "4", 1, 1),
            new Token(TokenType.NUMBER, "7", 1, 3),
            new Token(TokenType.MULT, "*", 1, 5),
            new Token(TokenType.EOF, "", 1, 6)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("*", op.getOperator());
    }

    @Test
    void testContractCase102Div() throws RpnException {
        // I/O contract: "10 2 /"
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "10", 1, 1),
            new Token(TokenType.NUMBER, "2", 1, 4),
            new Token(TokenType.DIV, "/", 1, 6),
            new Token(TokenType.EOF, "", 1, 7)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("/", op.getOperator());
    }
}
