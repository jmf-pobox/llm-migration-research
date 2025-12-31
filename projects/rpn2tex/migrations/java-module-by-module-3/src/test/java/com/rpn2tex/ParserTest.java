package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the Parser class.
 *
 * <p>Tests cover:
 * <ul>
 *   <li>Basic parsing of numbers and operators</li>
 *   <li>Complex nested expressions</li>
 *   <li>Error conditions (insufficient operands, extra operands, empty expressions)</li>
 *   <li>AST structure validation</li>
 *   <li>Position tracking in generated nodes</li>
 * </ul>
 */
class ParserTest {

    @Test
    @DisplayName("Parse single number")
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
        assertEquals(1, num.getLine());
        assertEquals(1, num.getColumn());
    }

    @Test
    @DisplayName("Parse addition: 5 3 +")
    void testAddition() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.PLUS, "+", 1, 5),
            new Token(TokenType.EOF, "", 1, 6)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp binOp = (BinaryOp) result;
        assertEquals("+", binOp.getOperator());
        assertEquals(1, binOp.getLine());
        assertEquals(5, binOp.getColumn());

        // Check left operand
        assertInstanceOf(Number.class, binOp.getLeft());
        Number left = (Number) binOp.getLeft();
        assertEquals("5", left.getValue());

        // Check right operand
        assertInstanceOf(Number.class, binOp.getRight());
        Number right = (Number) binOp.getRight();
        assertEquals("3", right.getValue());
    }

    @Test
    @DisplayName("Parse subtraction: 10 2 -")
    void testSubtraction() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "10", 1, 1),
            new Token(TokenType.NUMBER, "2", 1, 4),
            new Token(TokenType.MINUS, "-", 1, 6),
            new Token(TokenType.EOF, "", 1, 7)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp binOp = (BinaryOp) result;
        assertEquals("-", binOp.getOperator());
    }

    @Test
    @DisplayName("Parse multiplication: 4 7 *")
    void testMultiplication() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "4", 1, 1),
            new Token(TokenType.NUMBER, "7", 1, 3),
            new Token(TokenType.MULT, "*", 1, 5),
            new Token(TokenType.EOF, "", 1, 6)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp binOp = (BinaryOp) result;
        assertEquals("*", binOp.getOperator());
    }

    @Test
    @DisplayName("Parse division: 10 2 /")
    void testDivision() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "10", 1, 1),
            new Token(TokenType.NUMBER, "2", 1, 4),
            new Token(TokenType.DIV, "/", 1, 6),
            new Token(TokenType.EOF, "", 1, 7)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp binOp = (BinaryOp) result;
        assertEquals("/", binOp.getOperator());
    }

    @Test
    @DisplayName("Parse complex expression: 5 3 + 2 * -> (5 + 3) * 2")
    void testComplexExpression1() throws RpnException {
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

        // Root should be multiplication
        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.getOperator());

        // Left operand should be addition (5 + 3)
        assertInstanceOf(BinaryOp.class, mult.getLeft());
        BinaryOp add = (BinaryOp) mult.getLeft();
        assertEquals("+", add.getOperator());

        // Right operand should be number 2
        assertInstanceOf(Number.class, mult.getRight());
        Number two = (Number) mult.getRight();
        assertEquals("2", two.getValue());
    }

    @Test
    @DisplayName("Parse complex expression: 2 3 4 * + -> 2 + (3 * 4)")
    void testComplexExpression2() throws RpnException {
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

        // Root should be addition
        assertInstanceOf(BinaryOp.class, result);
        BinaryOp add = (BinaryOp) result;
        assertEquals("+", add.getOperator());

        // Left operand should be number 2
        assertInstanceOf(Number.class, add.getLeft());
        Number two = (Number) add.getLeft();
        assertEquals("2", two.getValue());

        // Right operand should be multiplication (3 * 4)
        assertInstanceOf(BinaryOp.class, add.getRight());
        BinaryOp mult = (BinaryOp) add.getRight();
        assertEquals("*", mult.getOperator());
    }

    @Test
    @DisplayName("Parse chained operations: 1 2 + 3 + 4 +")
    void testChainedAdditions() throws RpnException {
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

        // Root should be addition
        assertInstanceOf(BinaryOp.class, result);
        BinaryOp topAdd = (BinaryOp) result;
        assertEquals("+", topAdd.getOperator());

        // Should be nested left-associative: ((1 + 2) + 3) + 4
        assertInstanceOf(BinaryOp.class, topAdd.getLeft());
        assertInstanceOf(Number.class, topAdd.getRight());
        assertEquals("4", ((Number) topAdd.getRight()).getValue());
    }

    @Test
    @DisplayName("Parse decimal numbers: 3.14 2 *")
    void testDecimalNumbers() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "3.14", 1, 1),
            new Token(TokenType.NUMBER, "2", 1, 6),
            new Token(TokenType.MULT, "*", 1, 8),
            new Token(TokenType.EOF, "", 1, 9)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.getOperator());

        // Check decimal number is preserved
        assertInstanceOf(Number.class, mult.getLeft());
        Number pi = (Number) mult.getLeft();
        assertEquals("3.14", pi.getValue());
    }

    @Test
    @DisplayName("Parse negative numbers: -5 3 +")
    void testNegativeNumbers() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "-5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 4),
            new Token(TokenType.PLUS, "+", 1, 6),
            new Token(TokenType.EOF, "", 1, 7)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp add = (BinaryOp) result;
        assertEquals("+", add.getOperator());

        // Check negative number is preserved
        assertInstanceOf(Number.class, add.getLeft());
        Number neg = (Number) add.getLeft();
        assertEquals("-5", neg.getValue());
    }

    @Test
    @DisplayName("Error: Empty expression")
    void testEmptyExpression() {
        List<Token> tokens = List.of(
            new Token(TokenType.EOF, "", 1, 1)
        );

        Parser parser = new Parser(tokens);
        RpnException ex = assertThrows(RpnException.class, parser::parse);

        assertTrue(ex.getErrorMessage().contains("Empty expression"));
        assertEquals(1, ex.getLine());
        assertEquals(1, ex.getColumn());
    }

    @Test
    @DisplayName("Error: Insufficient operands for operator")
    void testInsufficientOperands() {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.PLUS, "+", 1, 3),
            new Token(TokenType.EOF, "", 1, 4)
        );

        Parser parser = new Parser(tokens);
        RpnException ex = assertThrows(RpnException.class, parser::parse);

        assertTrue(ex.getErrorMessage().contains("requires two operands"));
        assertEquals(1, ex.getLine());
        assertEquals(3, ex.getColumn());
    }

    @Test
    @DisplayName("Error: No operands for operator")
    void testNoOperands() {
        List<Token> tokens = List.of(
            new Token(TokenType.PLUS, "+", 1, 1),
            new Token(TokenType.EOF, "", 1, 2)
        );

        Parser parser = new Parser(tokens);
        RpnException ex = assertThrows(RpnException.class, parser::parse);

        assertTrue(ex.getErrorMessage().contains("requires two operands"));
        assertEquals(1, ex.getLine());
        assertEquals(1, ex.getColumn());
    }

    @Test
    @DisplayName("Error: Extra operands (missing operators)")
    void testExtraOperands() {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.NUMBER, "2", 1, 5),
            new Token(TokenType.EOF, "", 1, 6)
        );

        Parser parser = new Parser(tokens);
        RpnException ex = assertThrows(RpnException.class, parser::parse);

        assertTrue(ex.getErrorMessage().contains("values remain on stack"));
        assertTrue(ex.getErrorMessage().contains("missing operators"));
        assertEquals(1, ex.getLine());
        assertEquals(6, ex.getColumn());
    }

    @Test
    @DisplayName("Error: Multiple extra operands")
    void testMultipleExtraOperands() {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "1", 1, 1),
            new Token(TokenType.NUMBER, "2", 1, 3),
            new Token(TokenType.PLUS, "+", 1, 5),
            new Token(TokenType.NUMBER, "3", 1, 7),
            new Token(TokenType.NUMBER, "4", 1, 9),
            new Token(TokenType.EOF, "", 1, 10)
        );

        Parser parser = new Parser(tokens);
        RpnException ex = assertThrows(RpnException.class, parser::parse);

        assertTrue(ex.getErrorMessage().contains("3 values remain on stack"));
    }

    @Test
    @DisplayName("Position tracking in AST nodes")
    void testPositionTracking() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "10", 1, 1),
            new Token(TokenType.NUMBER, "5", 1, 4),
            new Token(TokenType.MINUS, "-", 1, 6),
            new Token(TokenType.EOF, "", 1, 7)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        // Check operator node position
        assertInstanceOf(BinaryOp.class, result);
        BinaryOp binOp = (BinaryOp) result;
        assertEquals(1, binOp.getLine());
        assertEquals(6, binOp.getColumn()); // Position of operator

        // Check left operand position
        Number left = (Number) binOp.getLeft();
        assertEquals(1, left.getLine());
        assertEquals(1, left.getColumn());

        // Check right operand position
        Number right = (Number) binOp.getRight();
        assertEquals(1, right.getLine());
        assertEquals(4, right.getColumn());
    }

    @Test
    @DisplayName("Null token list throws NullPointerException")
    void testNullTokenList() {
        assertThrows(NullPointerException.class, () -> new Parser(null));
    }

    @Test
    @DisplayName("Complex nested expression: 1 2 + 3 4 + *")
    void testComplexNested() throws RpnException {
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

        // Root: multiplication
        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.getOperator());

        // Left: (1 + 2)
        assertInstanceOf(BinaryOp.class, mult.getLeft());
        BinaryOp leftAdd = (BinaryOp) mult.getLeft();
        assertEquals("+", leftAdd.getOperator());

        // Right: (3 + 4)
        assertInstanceOf(BinaryOp.class, mult.getRight());
        BinaryOp rightAdd = (BinaryOp) mult.getRight();
        assertEquals("+", rightAdd.getOperator());
    }

    @Test
    @DisplayName("Four-level deep nesting: 10 2 / 3 + 4 *")
    void testDeepNesting() throws RpnException {
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

        // Root: multiplication
        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.getOperator());

        // Left side should be addition
        assertInstanceOf(BinaryOp.class, mult.getLeft());
        BinaryOp add = (BinaryOp) mult.getLeft();
        assertEquals("+", add.getOperator());

        // Left of addition should be division
        assertInstanceOf(BinaryOp.class, add.getLeft());
        BinaryOp div = (BinaryOp) add.getLeft();
        assertEquals("/", div.getOperator());

        // Division operands should be numbers
        assertInstanceOf(Number.class, div.getLeft());
        assertInstanceOf(Number.class, div.getRight());
        assertEquals("10", ((Number) div.getLeft()).getValue());
        assertEquals("2", ((Number) div.getRight()).getValue());
    }
}
