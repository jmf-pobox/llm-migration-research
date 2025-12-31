package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests for Parser with Lexer to validate the full parsing pipeline.
 *
 * <p>These tests ensure that Parser correctly processes token streams produced
 * by the Lexer, creating valid AST structures that can be traversed by downstream
 * components like the LaTeX generator.
 */
class ParserIntegrationTest {

    @ParameterizedTest
    @DisplayName("Integration test: Lexer -> Parser for basic expressions")
    @CsvSource({
        "'5 3 +', +",
        "'5 3 -', -",
        "'4 7 *', *",
        "'10 2 /', /"
    })
    void testLexerToParserBasicOperations(String input, String expectedOp) throws Exception {
        // Lexer tokenizes
        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        // Parser builds AST
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Verify AST structure
        assertInstanceOf(BinaryOp.class, ast);
        BinaryOp binOp = (BinaryOp) ast;
        assertEquals(expectedOp, binOp.getOperator());
        assertInstanceOf(Number.class, binOp.getLeft());
        assertInstanceOf(Number.class, binOp.getRight());
    }

    @Test
    @DisplayName("Integration: Complex expression with Lexer")
    void testComplexExpressionIntegration() throws Exception {
        String input = "5 3 + 2 *";

        // Full pipeline: Lexer -> Parser
        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Verify: Root should be multiplication
        assertInstanceOf(BinaryOp.class, ast);
        BinaryOp mult = (BinaryOp) ast;
        assertEquals("*", mult.getOperator());

        // Left side should be addition
        assertInstanceOf(BinaryOp.class, mult.getLeft());
        BinaryOp add = (BinaryOp) mult.getLeft();
        assertEquals("+", add.getOperator());

        // Right side should be Number(2)
        assertInstanceOf(Number.class, mult.getRight());
        assertEquals("2", ((Number) mult.getRight()).getValue());
    }

    @Test
    @DisplayName("Integration: Decimal numbers preserved through pipeline")
    void testDecimalPreservation() throws Exception {
        String input = "3.14 2 *";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        assertInstanceOf(BinaryOp.class, ast);
        BinaryOp mult = (BinaryOp) ast;

        Number left = (Number) mult.getLeft();
        assertEquals("3.14", left.getValue()); // Decimal preserved
    }

    @Test
    @DisplayName("Integration: Negative numbers handled correctly")
    void testNegativeNumbers() throws Exception {
        String input = "-5 3 +";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        assertInstanceOf(BinaryOp.class, ast);
        BinaryOp add = (BinaryOp) ast;

        Number left = (Number) add.getLeft();
        assertEquals("-5", left.getValue()); // Negative sign preserved
    }

    @Test
    @DisplayName("Integration: Position tracking through pipeline")
    void testPositionTracking() throws Exception {
        String input = "10 5 -";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Verify position information is preserved
        assertInstanceOf(BinaryOp.class, ast);
        BinaryOp binOp = (BinaryOp) ast;
        assertEquals(1, binOp.getLine());
        assertTrue(binOp.getColumn() > 0);

        // Check operand positions
        Number left = (Number) binOp.getLeft();
        assertEquals(1, left.getLine());
        assertEquals(1, left.getColumn());

        Number right = (Number) binOp.getRight();
        assertEquals(1, right.getLine());
        assertTrue(right.getColumn() > 1);
    }

    @Test
    @DisplayName("Integration: Lexer error propagates correctly")
    void testLexerErrorHandling() {
        String input = "5 3 ^"; // Unsupported character

        Lexer lexer = new Lexer(input);

        // Lexer should throw RpnException
        assertThrows(RpnException.class, lexer::tokenize);
    }

    @Test
    @DisplayName("Integration: Parser error with valid tokens")
    void testParserErrorHandling() throws Exception {
        String input = "5 +"; // Insufficient operands

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);

        // Parser should throw RpnException
        RpnException ex = assertThrows(RpnException.class, parser::parse);
        assertTrue(ex.getErrorMessage().contains("requires two operands"));
    }

    @Test
    @DisplayName("Integration: Chained operations")
    void testChainedOperations() throws Exception {
        String input = "1 2 + 3 + 4 +";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Verify nested structure: ((1 + 2) + 3) + 4
        assertInstanceOf(BinaryOp.class, ast);
        BinaryOp topAdd = (BinaryOp) ast;
        assertEquals("+", topAdd.getOperator());

        // Left side should be nested addition
        assertInstanceOf(BinaryOp.class, topAdd.getLeft());

        // Right side should be Number(4)
        assertInstanceOf(Number.class, topAdd.getRight());
        assertEquals("4", ((Number) topAdd.getRight()).getValue());
    }

    @Test
    @DisplayName("Integration: Whitespace handling")
    void testWhitespaceHandling() throws Exception {
        String input = "  5   3   +  "; // Extra whitespace

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Should parse correctly despite extra whitespace
        assertInstanceOf(BinaryOp.class, ast);
        BinaryOp add = (BinaryOp) ast;
        assertEquals("+", add.getOperator());
    }

    @Test
    @DisplayName("Integration: Empty input")
    void testEmptyInput() throws Exception {
        String input = "";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);

        // Should throw RpnException for empty expression
        assertThrows(RpnException.class, parser::parse);
    }
}
