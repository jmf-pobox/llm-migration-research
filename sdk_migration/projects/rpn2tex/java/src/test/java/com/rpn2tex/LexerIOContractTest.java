package com.rpn2tex;

import org.junit.jupiter.api.Test;
import java.util.List;
import static org.junit.jupiter.api.Assertions.*;

/**
 * I/O Contract validation tests for Lexer.
 *
 * These tests validate that the Lexer correctly tokenizes all inputs from the
 * I/O contract specification, including both successful cases and error cases.
 */
class LexerIOContractTest {

    @Test
    void testCase1_BasicAddition() throws RpnException {
        // Input: "5 3 +"
        Lexer lexer = new Lexer("5 3 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "5", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "3", 1, 3);
        assertToken(tokens.get(2), TokenType.PLUS, "+", 1, 5);
        assertToken(tokens.get(3), TokenType.EOF, "", 1, 6);
    }

    @Test
    void testCase2_BasicSubtraction() throws RpnException {
        // Input: "5 3 -"
        Lexer lexer = new Lexer("5 3 -");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "5", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "3", 1, 3);
        assertToken(tokens.get(2), TokenType.MINUS, "-", 1, 5);
        assertToken(tokens.get(3), TokenType.EOF, "", 1, 6);
    }

    @Test
    void testCase3_BasicMultiplication() throws RpnException {
        // Input: "4 7 *"
        Lexer lexer = new Lexer("4 7 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "4", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "7", 1, 3);
        assertToken(tokens.get(2), TokenType.MULT, "*", 1, 5);
        assertToken(tokens.get(3), TokenType.EOF, "", 1, 6);
    }

    @Test
    void testCase4_BasicDivision() throws RpnException {
        // Input: "10 2 /"
        Lexer lexer = new Lexer("10 2 /");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "10", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "2", 1, 4);
        assertToken(tokens.get(2), TokenType.DIV, "/", 1, 6);
        assertToken(tokens.get(3), TokenType.EOF, "", 1, 7);
    }

    @Test
    void testCase5_CaretOperatorError() {
        // Input: "2 3 ^"
        // Expected: LexerError "Unexpected character '^'" at line 1, column 5
        Lexer lexer = new Lexer("2 3 ^");

        RpnException exception = assertThrows(RpnException.class, () -> {
            lexer.tokenize();
        });

        assertTrue(exception.getMessage().contains("Unexpected character"));
        assertTrue(exception.getMessage().contains("^"));
        assertEquals(1, exception.getLine());
        assertEquals(5, exception.getColumn());
    }

    @Test
    void testCase6_AdditionAndMultiplication() throws RpnException {
        // Input: "5 3 + 2 *"
        Lexer lexer = new Lexer("5 3 + 2 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(6, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "5", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "3", 1, 3);
        assertToken(tokens.get(2), TokenType.PLUS, "+", 1, 5);
        assertToken(tokens.get(3), TokenType.NUMBER, "2", 1, 7);
        assertToken(tokens.get(4), TokenType.MULT, "*", 1, 9);
        assertToken(tokens.get(5), TokenType.EOF, "", 1, 10);
    }

    @Test
    void testCase7_MultiplicationAndAddition() throws RpnException {
        // Input: "5 3 * 2 +"
        Lexer lexer = new Lexer("5 3 * 2 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(6, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "5", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "3", 1, 3);
        assertToken(tokens.get(2), TokenType.MULT, "*", 1, 5);
        assertToken(tokens.get(3), TokenType.NUMBER, "2", 1, 7);
        assertToken(tokens.get(4), TokenType.PLUS, "+", 1, 9);
        assertToken(tokens.get(5), TokenType.EOF, "", 1, 10);
    }

    @Test
    void testCase8_DivisionAndMultiplication() throws RpnException {
        // Input: "10 2 / 5 *"
        Lexer lexer = new Lexer("10 2 / 5 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(6, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "10", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "2", 1, 4);
        assertToken(tokens.get(2), TokenType.DIV, "/", 1, 6);
        assertToken(tokens.get(3), TokenType.NUMBER, "5", 1, 8);
        assertToken(tokens.get(4), TokenType.MULT, "*", 1, 10);
        assertToken(tokens.get(5), TokenType.EOF, "", 1, 11);
    }

    @Test
    void testCase9_ChainedSubtraction() throws RpnException {
        // Input: "5 3 - 2 -"
        Lexer lexer = new Lexer("5 3 - 2 -");
        List<Token> tokens = lexer.tokenize();

        assertEquals(6, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "5", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "3", 1, 3);
        assertToken(tokens.get(2), TokenType.MINUS, "-", 1, 5);
        assertToken(tokens.get(3), TokenType.NUMBER, "2", 1, 7);
        assertToken(tokens.get(4), TokenType.MINUS, "-", 1, 9);
        assertToken(tokens.get(5), TokenType.EOF, "", 1, 10);
    }

    @Test
    void testCase10_ChainedDivision() throws RpnException {
        // Input: "100 10 / 5 / 2 /"
        Lexer lexer = new Lexer("100 10 / 5 / 2 /");
        List<Token> tokens = lexer.tokenize();

        assertEquals(8, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "100", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "10", 1, 5);
        assertToken(tokens.get(2), TokenType.DIV, "/", 1, 8);
        assertToken(tokens.get(3), TokenType.NUMBER, "5", 1, 10);
        assertToken(tokens.get(4), TokenType.DIV, "/", 1, 12);
        assertToken(tokens.get(5), TokenType.NUMBER, "2", 1, 14);
        assertToken(tokens.get(6), TokenType.DIV, "/", 1, 16);
        assertToken(tokens.get(7), TokenType.EOF, "", 1, 17);
    }

    @Test
    void testCase11_ChainedAddition() throws RpnException {
        // Input: "1 2 + 3 + 4 +"
        Lexer lexer = new Lexer("1 2 + 3 + 4 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(8, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "1", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "2", 1, 3);
        assertToken(tokens.get(2), TokenType.PLUS, "+", 1, 5);
        assertToken(tokens.get(3), TokenType.NUMBER, "3", 1, 7);
        assertToken(tokens.get(4), TokenType.PLUS, "+", 1, 9);
        assertToken(tokens.get(5), TokenType.NUMBER, "4", 1, 11);
        assertToken(tokens.get(6), TokenType.PLUS, "+", 1, 13);
        assertToken(tokens.get(7), TokenType.EOF, "", 1, 14);
    }

    @Test
    void testCase12_MixedPrecedence1() throws RpnException {
        // Input: "2 3 4 * +"
        Lexer lexer = new Lexer("2 3 4 * +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(6, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "2", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "3", 1, 3);
        assertToken(tokens.get(2), TokenType.NUMBER, "4", 1, 5);
        assertToken(tokens.get(3), TokenType.MULT, "*", 1, 7);
        assertToken(tokens.get(4), TokenType.PLUS, "+", 1, 9);
        assertToken(tokens.get(5), TokenType.EOF, "", 1, 10);
    }

    @Test
    void testCase16_CaretInComplexExpression() {
        // Input: "2 3 ^ 4 *"
        // Expected: LexerError at line 1, column 5
        Lexer lexer = new Lexer("2 3 ^ 4 *");

        RpnException exception = assertThrows(RpnException.class, () -> {
            lexer.tokenize();
        });

        assertTrue(exception.getMessage().contains("Unexpected character"));
        assertTrue(exception.getMessage().contains("^"));
        assertEquals(1, exception.getLine());
        assertEquals(5, exception.getColumn());
    }

    @Test
    void testCase17_MultipleCarets() {
        // Input: "2 3 4 ^ ^"
        // Expected: LexerError at line 1, column 7 (first caret)
        Lexer lexer = new Lexer("2 3 4 ^ ^");

        RpnException exception = assertThrows(RpnException.class, () -> {
            lexer.tokenize();
        });

        assertTrue(exception.getMessage().contains("Unexpected character"));
        assertTrue(exception.getMessage().contains("^"));
        assertEquals(1, exception.getLine());
        assertEquals(7, exception.getColumn());
    }

    @Test
    void testCase18_FloatingPointMultiplication() throws RpnException {
        // Input: "3.14 2 *"
        Lexer lexer = new Lexer("3.14 2 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "3.14", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "2", 1, 6);
        assertToken(tokens.get(2), TokenType.MULT, "*", 1, 8);
        assertToken(tokens.get(3), TokenType.EOF, "", 1, 9);
    }

    @Test
    void testCase19_FloatingPointAddition() throws RpnException {
        // Input: "1.5 0.5 +"
        Lexer lexer = new Lexer("1.5 0.5 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "1.5", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "0.5", 1, 5);
        assertToken(tokens.get(2), TokenType.PLUS, "+", 1, 9);
        assertToken(tokens.get(3), TokenType.EOF, "", 1, 10);
    }

    @Test
    void testCase20_ComplexExpressionWithMultipleSubExpressions() throws RpnException {
        // Input: "1 2 + 3 4 + *"
        Lexer lexer = new Lexer("1 2 + 3 4 + *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(8, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "1", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "2", 1, 3);
        assertToken(tokens.get(2), TokenType.PLUS, "+", 1, 5);
        assertToken(tokens.get(3), TokenType.NUMBER, "3", 1, 7);
        assertToken(tokens.get(4), TokenType.NUMBER, "4", 1, 9);
        assertToken(tokens.get(5), TokenType.PLUS, "+", 1, 11);
        assertToken(tokens.get(6), TokenType.MULT, "*", 1, 13);
        assertToken(tokens.get(7), TokenType.EOF, "", 1, 14);
    }

    @Test
    void testCase21_ComplexExpressionWithDivisionAndAddition() throws RpnException {
        // Input: "10 2 / 3 + 4 *"
        Lexer lexer = new Lexer("10 2 / 3 + 4 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(8, tokens.size());
        assertToken(tokens.get(0), TokenType.NUMBER, "10", 1, 1);
        assertToken(tokens.get(1), TokenType.NUMBER, "2", 1, 4);
        assertToken(tokens.get(2), TokenType.DIV, "/", 1, 6);
        assertToken(tokens.get(3), TokenType.NUMBER, "3", 1, 8);
        assertToken(tokens.get(4), TokenType.PLUS, "+", 1, 10);
        assertToken(tokens.get(5), TokenType.NUMBER, "4", 1, 12);
        assertToken(tokens.get(6), TokenType.MULT, "*", 1, 14);
        assertToken(tokens.get(7), TokenType.EOF, "", 1, 15);
    }

    /**
     * Helper method to assert token properties.
     */
    private void assertToken(Token token, TokenType expectedType, String expectedValue,
                            int expectedLine, int expectedColumn) {
        assertEquals(expectedType, token.type(),
            String.format("Expected type %s but got %s", expectedType, token.type()));
        assertEquals(expectedValue, token.value(),
            String.format("Expected value '%s' but got '%s'", expectedValue, token.value()));
        assertEquals(expectedLine, token.line(),
            String.format("Expected line %d but got %d", expectedLine, token.line()));
        assertEquals(expectedColumn, token.column(),
            String.format("Expected column %d but got %d", expectedColumn, token.column()));
    }
}
