package com.rpn2tex;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests for the {@link Parser} class working with the {@link Lexer}.
 *
 * <p>These tests validate the complete tokenization and parsing pipeline,
 * ensuring that real RPN expressions from strings are correctly converted to AST nodes.
 */
class ParserIntegrationTest {

    @Test
    void testLexerToParserSimpleAddition() throws RpnException {
        // Test the full pipeline: String -> Lexer -> Tokens -> Parser -> AST
        String input = "5 3 +";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("+", op.getOperator());
        assertEquals("5", ((Number) op.getLeft()).getValue());
        assertEquals("3", ((Number) op.getRight()).getValue());
    }

    @Test
    void testLexerToParserComplexExpression() throws RpnException {
        // "5 3 + 2 *" -> (5 + 3) * 2
        String input = "5 3 + 2 *";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("*", op.getOperator());

        // Left should be BinaryOp(+, 5, 3)
        assertInstanceOf(BinaryOp.class, op.getLeft());
        BinaryOp leftOp = (BinaryOp) op.getLeft();
        assertEquals("+", leftOp.getOperator());

        // Right should be Number(2)
        assertInstanceOf(Number.class, op.getRight());
        assertEquals("2", ((Number) op.getRight()).getValue());
    }

    @Test
    void testLexerToParserWithFloatingPoint() throws RpnException {
        String input = "3.14 2 *";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("*", op.getOperator());
        assertEquals("3.14", ((Number) op.getLeft()).getValue());
        assertEquals("2", ((Number) op.getRight()).getValue());
    }

    @Test
    void testLexerToParserNestedPrecedence() throws RpnException {
        // "2 3 4 * +" -> 2 + (3 * 4)
        String input = "2 3 4 * +";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

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
    void testLexerToParserTwoComplexSubexpressions() throws RpnException {
        // "1 2 + 3 4 + *" -> (1 + 2) * (3 + 4)
        String input = "1 2 + 3 4 + *";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

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
    void testLexerToParserInsufficientOperands() {
        // "5 +" should fail at parser stage
        String input = "5 +";

        Lexer lexer = new Lexer(input);
        assertDoesNotThrow(() -> {
            List<Token> tokens = lexer.tokenize();
            Parser parser = new Parser(tokens);

            RpnException exception = assertThrows(RpnException.class, parser::parse);
            assertTrue(exception.getMessage().contains("Operator '+' requires two operands"));
        });
    }

    @Test
    void testLexerToParserExtraOperands() {
        // "5 3" should fail at parser stage (missing operator)
        String input = "5 3";

        Lexer lexer = new Lexer(input);
        assertDoesNotThrow(() -> {
            List<Token> tokens = lexer.tokenize();
            Parser parser = new Parser(tokens);

            RpnException exception = assertThrows(RpnException.class, parser::parse);
            assertTrue(exception.getMessage().contains("Invalid RPN: 2 values remain on stack"));
        });
    }

    @Test
    void testLexerToParserMultipleSubtractions() throws RpnException {
        // "5 3 - 2 -" -> (5 - 3) - 2
        String input = "5 3 - 2 -";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("-", op.getOperator());

        // Left should be BinaryOp(-, 5, 3)
        assertInstanceOf(BinaryOp.class, op.getLeft());
        BinaryOp leftOp = (BinaryOp) op.getLeft();
        assertEquals("-", leftOp.getOperator());
        assertEquals("5", ((Number) leftOp.getLeft()).getValue());
        assertEquals("3", ((Number) leftOp.getRight()).getValue());

        // Right should be Number(2)
        assertInstanceOf(Number.class, op.getRight());
        assertEquals("2", ((Number) op.getRight()).getValue());
    }

    @Test
    void testLexerToParserMultipleDivisions() throws RpnException {
        // "100 10 / 5 / 2 /" -> ((100 / 10) / 5) / 2
        String input = "100 10 / 5 / 2 /";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp op = (BinaryOp) result;
        assertEquals("/", op.getOperator());

        // Verify nested structure (left-to-right associativity)
        assertInstanceOf(BinaryOp.class, op.getLeft());
        BinaryOp leftOp = (BinaryOp) op.getLeft();
        assertEquals("/", leftOp.getOperator());

        assertInstanceOf(BinaryOp.class, leftOp.getLeft());
        BinaryOp leftLeftOp = (BinaryOp) leftOp.getLeft();
        assertEquals("/", leftLeftOp.getOperator());
        assertEquals("100", ((Number) leftLeftOp.getLeft()).getValue());
        assertEquals("10", ((Number) leftLeftOp.getRight()).getValue());
    }
}
