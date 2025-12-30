package com.rpn2tex;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests validating the I/O contract from the migration specification.
 *
 * <p>These tests verify end-to-end behavior: Lexer → Parser → LaTeXGenerator.
 * All test cases are taken from the migration spec Phase 0 I/O contract.
 */
@DisplayName("I/O Contract Integration Tests")
class IOContractTest {

    /**
     * Helper method to process RPN input through the full pipeline.
     *
     * @param input RPN expression string
     * @return LaTeX output string
     * @throws RpnException if lexing or parsing fails
     */
    private String processRPN(String input) throws RpnException {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        return generator.generate(ast);
    }

    @ParameterizedTest
    @CsvSource(delimiter = '|', value = {
        "5 3 +              | $5 + 3$",
        "5 3 -              | $5 - 3$",
        "4 7 *              | $4 \\times 7$",
        "10 2 /             | $10 \\div 2$",
        "5 3 + 2 *          | $( 5 + 3 ) \\times 2$",
        "5 3 * 2 +          | $5 \\times 3 + 2$",
        "10 2 / 5 *         | $10 \\div 2 \\times 5$",
        "5 3 - 2 -          | $5 - 3 - 2$",
        "100 10 / 5 / 2 /   | $100 \\div 10 \\div 5 \\div 2$",
        "1 2 + 3 + 4 +      | $1 + 2 + 3 + 4$",
        "2 3 4 * +          | $2 + 3 \\times 4$",
        "2 3 + 4 *          | $( 2 + 3 ) \\times 4$",
        "2 3 4 + *          | $2 \\times ( 3 + 4 )$",
        "2 3 * 4 +          | $2 \\times 3 + 4$",
        "3.14 2 *           | $3.14 \\times 2$",
        "1.5 0.5 +          | $1.5 + 0.5$",
        "1 2 + 3 4 + *      | $( 1 + 2 ) \\times ( 3 + 4 )$",
        "10 2 / 3 + 4 *     | $( 10 \\div 2 + 3 ) \\times 4$"
    })
    @DisplayName("should generate correct LaTeX for valid RPN expressions")
    void testValidExpressions(String input, String expectedOutput) throws RpnException {
        String result = processRPN(input);
        assertEquals(expectedOutput, result,
            String.format("Input: '%s' should produce: '%s'", input, expectedOutput));
    }

    @Test
    @DisplayName("should generate LaTeX for basic addition: 5 3 +")
    void testBasicAddition() throws RpnException {
        String result = processRPN("5 3 +");
        assertEquals("$5 + 3$", result);
    }

    @Test
    @DisplayName("should generate LaTeX for basic subtraction: 5 3 -")
    void testBasicSubtraction() throws RpnException {
        String result = processRPN("5 3 -");
        assertEquals("$5 - 3$", result);
    }

    @Test
    @DisplayName("should generate LaTeX for basic multiplication: 4 7 *")
    void testBasicMultiplication() throws RpnException {
        String result = processRPN("4 7 *");
        assertEquals("$4 \\times 7$", result);
    }

    @Test
    @DisplayName("should generate LaTeX for basic division: 10 2 /")
    void testBasicDivision() throws RpnException {
        String result = processRPN("10 2 /");
        assertEquals("$10 \\div 2$", result);
    }

    @Test
    @DisplayName("should throw exception for unsupported exponentiation: 2 3 ^")
    void testExponentiationNotSupported() {
        RpnException exception = assertThrows(RpnException.class, () -> {
            processRPN("2 3 ^");
        });
        assertTrue(exception.getMessage().contains("Unexpected character '^'"),
            "Error message should mention unexpected character '^'");
    }

    @Test
    @DisplayName("should add parentheses for precedence: 5 3 + 2 *")
    void testPrecedenceWithParentheses() throws RpnException {
        String result = processRPN("5 3 + 2 *");
        assertEquals("$( 5 + 3 ) \\times 2$", result);
    }

    @Test
    @DisplayName("should handle operator precedence without parens: 5 3 * 2 +")
    void testPrecedenceNoParentheses() throws RpnException {
        String result = processRPN("5 3 * 2 +");
        assertEquals("$5 \\times 3 + 2$", result);
    }

    @Test
    @DisplayName("should handle left-associative operations: 10 2 / 5 *")
    void testLeftAssociativeOps() throws RpnException {
        String result = processRPN("10 2 / 5 *");
        assertEquals("$10 \\div 2 \\times 5$", result);
    }

    @Test
    @DisplayName("should handle multiple subtractions: 5 3 - 2 -")
    void testMultipleSubtractions() throws RpnException {
        String result = processRPN("5 3 - 2 -");
        assertEquals("$5 - 3 - 2$", result);
    }

    @Test
    @DisplayName("should handle multiple divisions: 100 10 / 5 / 2 /")
    void testMultipleDivisions() throws RpnException {
        String result = processRPN("100 10 / 5 / 2 /");
        assertEquals("$100 \\div 10 \\div 5 \\div 2$", result);
    }

    @Test
    @DisplayName("should handle multiple additions: 1 2 + 3 + 4 +")
    void testMultipleAdditions() throws RpnException {
        String result = processRPN("1 2 + 3 + 4 +");
        assertEquals("$1 + 2 + 3 + 4$", result);
    }

    @Test
    @DisplayName("should handle multiplication with higher precedence: 2 3 4 * +")
    void testMultiplicationHigherPrecedence() throws RpnException {
        String result = processRPN("2 3 4 * +");
        assertEquals("$2 + 3 \\times 4$", result);
    }

    @Test
    @DisplayName("should add parens for addition before mult: 2 3 + 4 *")
    void testAdditionBeforeMultiplication() throws RpnException {
        String result = processRPN("2 3 + 4 *");
        assertEquals("$( 2 + 3 ) \\times 4$", result);
    }

    @Test
    @DisplayName("should add parens for addition as second operand: 2 3 4 + *")
    void testAdditionAsSecondOperand() throws RpnException {
        String result = processRPN("2 3 4 + *");
        assertEquals("$2 \\times ( 3 + 4 )$", result);
    }

    @Test
    @DisplayName("should not add unnecessary parens: 2 3 * 4 +")
    void testNoUnnecessaryParens() throws RpnException {
        String result = processRPN("2 3 * 4 +");
        assertEquals("$2 \\times 3 + 4$", result);
    }

    @Test
    @DisplayName("should handle floating-point multiplication: 3.14 2 *")
    void testFloatingPointMultiplication() throws RpnException {
        String result = processRPN("3.14 2 *");
        assertEquals("$3.14 \\times 2$", result);
    }

    @Test
    @DisplayName("should handle floating-point addition: 1.5 0.5 +")
    void testFloatingPointAddition() throws RpnException {
        String result = processRPN("1.5 0.5 +");
        assertEquals("$1.5 + 0.5$", result);
    }

    @Test
    @DisplayName("should handle both operands parenthesized: 1 2 + 3 4 + *")
    void testBothOperandsParenthesized() throws RpnException {
        String result = processRPN("1 2 + 3 4 + *");
        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 )$", result);
    }

    @Test
    @DisplayName("should handle complex mixed expression: 10 2 / 3 + 4 *")
    void testComplexMixedExpression() throws RpnException {
        String result = processRPN("10 2 / 3 + 4 *");
        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$", result);
    }

    @Test
    @DisplayName("should reject exponentiation in complex expression: 2 3 ^ 4 *")
    void testExponentiationInComplexExpression() {
        RpnException exception = assertThrows(RpnException.class, () -> {
            processRPN("2 3 ^ 4 *");
        });
        assertTrue(exception.getMessage().contains("Unexpected character '^'"));
    }

    @Test
    @DisplayName("should reject multiple exponentiations: 2 3 4 ^ ^")
    void testMultipleExponentiations() {
        RpnException exception = assertThrows(RpnException.class, () -> {
            processRPN("2 3 4 ^ ^");
        });
        assertTrue(exception.getMessage().contains("Unexpected character '^'"));
    }

    @Test
    @DisplayName("should handle negative numbers")
    void testNegativeNumbers() throws RpnException {
        String result = processRPN("-5 3 +");
        assertEquals("$-5 + 3$", result);
    }

    @Test
    @DisplayName("should handle negative floating-point numbers")
    void testNegativeFloatingPoint() throws RpnException {
        String result = processRPN("-3.14 2 *");
        assertEquals("$-3.14 \\times 2$", result);
    }

    @Test
    @DisplayName("should preserve number format from input")
    void testNumberFormatPreservation() throws RpnException {
        // Leading zero
        String result1 = processRPN("0.5 0.5 +");
        assertEquals("$0.5 + 0.5$", result1);

        // Large numbers
        String result2 = processRPN("1000 2000 +");
        assertEquals("$1000 + 2000$", result2);
    }

    @Test
    @DisplayName("should handle whitespace variations")
    void testWhitespaceVariations() throws RpnException {
        // Multiple spaces
        String result1 = processRPN("5   3   +");
        assertEquals("$5 + 3$", result1);

        // Tabs
        String result2 = processRPN("5\t3\t+");
        assertEquals("$5 + 3$", result2);

        // Mixed whitespace
        String result3 = processRPN("5 \t 3  \t+");
        assertEquals("$5 + 3$", result3);
    }

    @Test
    @DisplayName("should handle expressions with newlines")
    void testNewlineHandling() throws RpnException {
        String result = processRPN("5 3 +\n2 *");
        assertEquals("$( 5 + 3 ) \\times 2$", result);
    }

    @Test
    @DisplayName("LaTeX output should always be wrapped in dollar signs")
    void testMathModeWrapping() throws RpnException {
        String result = processRPN("42");
        assertTrue(result.startsWith("$"));
        assertTrue(result.endsWith("$"));
    }

    @Test
    @DisplayName("should properly escape LaTeX commands")
    void testLatexCommandEscaping() throws RpnException {
        String multResult = processRPN("2 3 *");
        assertTrue(multResult.contains("\\times"), "Multiplication should use \\times");

        String divResult = processRPN("10 2 /");
        assertTrue(divResult.contains("\\div"), "Division should use \\div");
    }
}
