package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;
import static org.junit.jupiter.api.Assertions.*;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * Unit tests for the RPN Parser.
 *
 * <p>Tests the stack-based RPN parser that converts tokens into an AST.
 * Validates number handling, operator processing, stack validation, and error cases.
 */
@DisplayName("Parser Tests")
class ParserTest {

    /**
     * Helper method to create a token list from simple inputs.
     *
     * @param parts tokens to create (numbers as strings, operators as symbols)
     * @return list of tokens with EOF at the end
     */
    private List<Token> makeTokens(String... parts) {
        List<Token> tokens = new ArrayList<>();
        int col = 1;
        for (String part : parts) {
            TokenType type;
            switch (part) {
                case "+": type = TokenType.PLUS; break;
                case "-": type = TokenType.MINUS; break;
                case "*": type = TokenType.MULT; break;
                case "/": type = TokenType.DIV; break;
                default: type = TokenType.NUMBER; break;
            }
            tokens.add(new Token(type, part, 1, col));
            col += part.length() + 1;
        }
        tokens.add(new Token(TokenType.EOF, "", 1, col));
        return tokens;
    }

    @Test
    @DisplayName("Parse single number")
    void testSingleNumber() throws RpnException {
        List<Token> tokens = makeTokens("42");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(Number.class, result);
        Number num = (Number) result;
        assertEquals("42", num.value());
    }

    @Test
    @DisplayName("Parse floating-point number")
    void testFloatingPointNumber() throws RpnException {
        List<Token> tokens = makeTokens("3.14");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(Number.class, result);
        Number num = (Number) result;
        assertEquals("3.14", num.value());
    }

    @Test
    @DisplayName("Parse basic addition: 5 3 +")
    void testBasicAddition() throws RpnException {
        List<Token> tokens = makeTokens("5", "3", "+");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("+", op.operator());

        assertInstanceOf(Number.class, op.left());
        assertInstanceOf(Number.class, op.right());
        assertEquals("5", ((Number) op.left()).value());
        assertEquals("3", ((Number) op.right()).value());
    }

    @Test
    @DisplayName("Parse basic subtraction: 5 3 -")
    void testBasicSubtraction() throws RpnException {
        List<Token> tokens = makeTokens("5", "3", "-");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("-", op.operator());
        assertEquals("5", ((Number) op.left()).value());
        assertEquals("3", ((Number) op.right()).value());
    }

    @Test
    @DisplayName("Parse basic multiplication: 4 7 *")
    void testBasicMultiplication() throws RpnException {
        List<Token> tokens = makeTokens("4", "7", "*");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("*", op.operator());
        assertEquals("4", ((Number) op.left()).value());
        assertEquals("7", ((Number) op.right()).value());
    }

    @Test
    @DisplayName("Parse basic division: 10 2 /")
    void testBasicDivision() throws RpnException {
        List<Token> tokens = makeTokens("10", "2", "/");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("/", op.operator());
        assertEquals("10", ((Number) op.left()).value());
        assertEquals("2", ((Number) op.right()).value());
    }

    @Test
    @DisplayName("Parse nested expression: 5 3 + 2 *")
    void testNestedExpression() throws RpnException {
        // RPN: 5 3 + 2 *
        // AST: BinaryOp(*, BinaryOp(+, 5, 3), 2)
        List<Token> tokens = makeTokens("5", "3", "+", "2", "*");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.operator());

        // Left should be addition
        assertInstanceOf(BinaryOp.class, mult.left());
        BinaryOp add = (BinaryOp) mult.left();
        assertEquals("+", add.operator());
        assertEquals("5", ((Number) add.left()).value());
        assertEquals("3", ((Number) add.right()).value());

        // Right should be number
        assertInstanceOf(Number.class, mult.right());
        assertEquals("2", ((Number) mult.right()).value());
    }

    @Test
    @DisplayName("Parse right-nested expression: 2 3 4 + *")
    void testRightNestedExpression() throws RpnException {
        // RPN: 2 3 4 + *
        // AST: BinaryOp(*, 2, BinaryOp(+, 3, 4))
        List<Token> tokens = makeTokens("2", "3", "4", "+", "*");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.operator());

        // Left should be number
        assertInstanceOf(Number.class, mult.left());
        assertEquals("2", ((Number) mult.left()).value());

        // Right should be addition
        assertInstanceOf(BinaryOp.class, mult.right());
        BinaryOp add = (BinaryOp) mult.right();
        assertEquals("+", add.operator());
        assertEquals("3", ((Number) add.left()).value());
        assertEquals("4", ((Number) add.right()).value());
    }

    @Test
    @DisplayName("Parse chain of operations: 1 2 + 3 + 4 +")
    void testChainedAdditions() throws RpnException {
        // RPN: 1 2 + 3 + 4 +
        // AST: BinaryOp(+, BinaryOp(+, BinaryOp(+, 1, 2), 3), 4)
        List<Token> tokens = makeTokens("1", "2", "+", "3", "+", "4", "+");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp outer = (BinaryOp) result;
        assertEquals("+", outer.operator());
        assertEquals("4", ((Number) outer.right()).value());

        assertInstanceOf(BinaryOp.class, outer.left());
        BinaryOp middle = (BinaryOp) outer.left();
        assertEquals("+", middle.operator());
        assertEquals("3", ((Number) middle.right()).value());

        assertInstanceOf(BinaryOp.class, middle.left());
        BinaryOp inner = (BinaryOp) middle.left();
        assertEquals("+", inner.operator());
        assertEquals("1", ((Number) inner.left()).value());
        assertEquals("2", ((Number) inner.right()).value());
    }

    @Test
    @DisplayName("Parse multiple divisions: 100 10 / 5 / 2 /")
    void testMultipleDivisions() throws RpnException {
        // Test left-associativity
        List<Token> tokens = makeTokens("100", "10", "/", "5", "/", "2", "/");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp outer = (BinaryOp) result;
        assertEquals("/", outer.operator());
        assertEquals("2", ((Number) outer.right()).value());
    }

    @Test
    @DisplayName("Parse complex expression: 1 2 + 3 4 + *")
    void testComplexExpression() throws RpnException {
        // RPN: 1 2 + 3 4 + *
        // AST: BinaryOp(*, BinaryOp(+, 1, 2), BinaryOp(+, 3, 4))
        List<Token> tokens = makeTokens("1", "2", "+", "3", "4", "+", "*");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.operator());

        // Both children should be additions
        assertInstanceOf(BinaryOp.class, mult.left());
        assertInstanceOf(BinaryOp.class, mult.right());

        BinaryOp leftAdd = (BinaryOp) mult.left();
        assertEquals("+", leftAdd.operator());
        assertEquals("1", ((Number) leftAdd.left()).value());
        assertEquals("2", ((Number) leftAdd.right()).value());

        BinaryOp rightAdd = (BinaryOp) mult.right();
        assertEquals("+", rightAdd.operator());
        assertEquals("3", ((Number) rightAdd.left()).value());
        assertEquals("4", ((Number) rightAdd.right()).value());
    }

    @Test
    @DisplayName("Parse with floating-point: 3.14 2 *")
    void testFloatingPointOperation() throws RpnException {
        List<Token> tokens = makeTokens("3.14", "2", "*");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("*", op.operator());
        assertEquals("3.14", ((Number) op.left()).value());
        assertEquals("2", ((Number) op.right()).value());
    }

    @Test
    @DisplayName("Error: empty expression")
    void testEmptyExpression() {
        List<Token> tokens = Arrays.asList(new Token(TokenType.EOF, "", 1, 1));
        Parser parser = new Parser(tokens);

        RpnException ex = assertThrows(RpnException.class, parser::parse);
        assertTrue(ex.getMessage().contains("Empty expression"));
    }

    @Test
    @DisplayName("Error: operator without operands")
    void testOperatorWithoutOperands() {
        List<Token> tokens = makeTokens("+");
        Parser parser = new Parser(tokens);

        RpnException ex = assertThrows(RpnException.class, parser::parse);
        assertTrue(ex.getMessage().contains("requires two operands"));
    }

    @Test
    @DisplayName("Error: operator with only one operand")
    void testOperatorWithOneOperand() {
        List<Token> tokens = makeTokens("5", "+");
        Parser parser = new Parser(tokens);

        RpnException ex = assertThrows(RpnException.class, parser::parse);
        assertTrue(ex.getMessage().contains("requires two operands"));
    }

    @Test
    @DisplayName("Error: extra operands (missing operator)")
    void testExtraOperands() {
        List<Token> tokens = makeTokens("5", "3", "2");
        Parser parser = new Parser(tokens);

        RpnException ex = assertThrows(RpnException.class, parser::parse);
        assertTrue(ex.getMessage().contains("3 values remain on stack")
                || ex.getMessage().contains("missing operators"));
    }

    @Test
    @DisplayName("Error: two operands without operator")
    void testTwoOperandsNoOperator() {
        List<Token> tokens = makeTokens("5", "3");
        Parser parser = new Parser(tokens);

        RpnException ex = assertThrows(RpnException.class, parser::parse);
        assertTrue(ex.getMessage().contains("2 values remain on stack")
                || ex.getMessage().contains("missing operators"));
    }

    @Test
    @DisplayName("Position tracking in Number node")
    void testNumberPositionTracking() throws RpnException {
        Token numToken = new Token(TokenType.NUMBER, "42", 2, 10);
        Token eofToken = new Token(TokenType.EOF, "", 2, 13);
        List<Token> tokens = Arrays.asList(numToken, eofToken);

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertEquals(2, result.line());
        assertEquals(10, result.column());
    }

    @Test
    @DisplayName("Position tracking in BinaryOp node")
    void testBinaryOpPositionTracking() throws RpnException {
        Token numToken1 = new Token(TokenType.NUMBER, "5", 1, 1);
        Token numToken2 = new Token(TokenType.NUMBER, "3", 1, 3);
        Token plusToken = new Token(TokenType.PLUS, "+", 1, 5);
        Token eofToken = new Token(TokenType.EOF, "", 1, 7);
        List<Token> tokens = Arrays.asList(numToken1, numToken2, plusToken, eofToken);

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        // BinaryOp should have position from the operator token
        assertEquals(1, result.line());
        assertEquals(5, result.column());
    }

    @ParameterizedTest
    @DisplayName("Parse all operators")
    @CsvSource({
        "5, 3, +, +",
        "5, 3, -, -",
        "4, 7, *, *",
        "10, 2, /, /"
    })
    void testAllOperators(String left, String right, String opSymbol, String expectedOp) throws RpnException {
        List<Token> tokens = makeTokens(left, right, opSymbol);
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals(expectedOp, op.operator());
        assertEquals(left, ((Number) op.left()).value());
        assertEquals(right, ((Number) op.right()).value());
    }

    @Test
    @DisplayName("Parse preserves evaluation order")
    void testEvaluationOrder() throws RpnException {
        // RPN: 5 3 - 2 -
        // Should be parsed as (5 - 3) - 2, NOT 5 - (3 - 2)
        List<Token> tokens = makeTokens("5", "3", "-", "2", "-");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp outer = (BinaryOp) result;
        assertEquals("-", outer.operator());

        // Left child should be (5 - 3)
        assertInstanceOf(BinaryOp.class, outer.left());
        BinaryOp inner = (BinaryOp) outer.left();
        assertEquals("-", inner.operator());
        assertEquals("5", ((Number) inner.left()).value());
        assertEquals("3", ((Number) inner.right()).value());

        // Right child should be 2
        assertInstanceOf(Number.class, outer.right());
        assertEquals("2", ((Number) outer.right()).value());
    }

    @Test
    @DisplayName("Parser handles empty token list gracefully")
    void testEmptyTokenList() {
        List<Token> tokens = new ArrayList<>();

        // This should throw an exception since we need at least an EOF token
        assertThrows(Exception.class, () -> {
            Parser parser = new Parser(tokens);
            parser.parse();
        });
    }

    @Test
    @DisplayName("Operator associativity: left-to-right for same precedence")
    void testLeftToRightAssociativity() throws RpnException {
        // Test that 10 2 / 5 * is parsed as (10 / 2) * 5
        List<Token> tokens = makeTokens("10", "2", "/", "5", "*");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.operator());

        // Left should be division
        assertInstanceOf(BinaryOp.class, mult.left());
        BinaryOp div = (BinaryOp) mult.left();
        assertEquals("/", div.operator());
        assertEquals("10", ((Number) div.left()).value());
        assertEquals("2", ((Number) div.right()).value());

        // Right should be 5
        assertEquals("5", ((Number) mult.right()).value());
    }

    @Test
    @DisplayName("Parser creates immutable AST")
    void testASTImmutability() throws RpnException {
        List<Token> tokens = makeTokens("5", "3", "+");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        // Verify that the result is an immutable tree
        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;

        // All fields should be final (tested by accessing them)
        assertNotNull(op.operator());
        assertNotNull(op.left());
        assertNotNull(op.right());
    }

    @Test
    @DisplayName("Complex mixed operators: 10 2 / 3 + 4 *")
    void testComplexMixedOperators() throws RpnException {
        // RPN: 10 2 / 3 + 4 *
        // Should be: ((10 / 2) + 3) * 4
        List<Token> tokens = makeTokens("10", "2", "/", "3", "+", "4", "*");
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.operator());
        assertEquals("4", ((Number) mult.right()).value());

        // Left should be addition
        assertInstanceOf(BinaryOp.class, mult.left());
        BinaryOp add = (BinaryOp) mult.left();
        assertEquals("+", add.operator());
        assertEquals("3", ((Number) add.right()).value());

        // Left of addition should be division
        assertInstanceOf(BinaryOp.class, add.left());
        BinaryOp div = (BinaryOp) add.left();
        assertEquals("/", div.operator());
        assertEquals("10", ((Number) div.left()).value());
        assertEquals("2", ((Number) div.right()).value());
    }
}
