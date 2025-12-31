package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests for the complete pipeline.
 * These tests verify the I/O contract from the analysis document.
 */
class IntegrationTest {
    /**
     * Helper method to process RPN input through the complete pipeline.
     */
    private String process(String input) throws RpnException {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        return generator.generate(ast);
    }

    @ParameterizedTest(name = "Input: \"{0}\" -> Expected: \"{1}\"")
    @CsvSource({
            "5,                    $5$",
            "3.14,                 $3.14$",
            "5 3 +,                $5 + 3$",
            "1 2 + 3 + 4 +,        $1 + 2 + 3 + 4$",
            "5 3 -,                $5 - 3$",
            "5 3 - 2 -,            $5 - 3 - 2$",
            "4 7 *,                $4 \\times 7$",
            "2 3 4 * +,            $2 + 3 \\times 4$",
            "10 2 /,               $10 \\div 2$",
            "100 10 / 5 / 2 /,     $100 \\div 10 \\div 5 \\div 2$"
    })
    void testIOContract(String input, String expected) throws RpnException {
        String actual = process(input);
        assertEquals(expected, actual);
    }

    @Test
    void testIntegerOutput() throws RpnException {
        String result = process("5");
        assertEquals("$5$", result);
    }

    @Test
    void testDecimalOutput() throws RpnException {
        String result = process("3.14");
        assertEquals("$3.14$", result);
    }

    @Test
    void testNegativeInteger() throws RpnException {
        String result = process("-5");
        assertEquals("$-5$", result);
    }

    @Test
    void testNegativeDecimal() throws RpnException {
        String result = process("-3.14");
        assertEquals("$-3.14$", result);
    }

    @Test
    void testZero() throws RpnException {
        String result = process("0");
        assertEquals("$0$", result);
    }

    @Test
    void testLeadingZeros() throws RpnException {
        String result = process("007");
        assertEquals("$007$", result);
    }

    @Test
    void testTrailingDecimalPoint() throws RpnException {
        String result = process("5.");
        assertEquals("$5.$", result);
    }

    @Test
    void testDecimalStartingWithZero() throws RpnException {
        String result = process("0.5");
        assertEquals("$0.5$", result);
    }

    @Test
    void testWhitespaceVariations() throws RpnException {
        assertEquals("$42$", process("42"));
        assertEquals("$42$", process("  42"));
        assertEquals("$42$", process("42  "));
        assertEquals("$42$", process("  42  "));
    }

    @Test
    void testSimpleAddition() throws RpnException {
        String result = process("5 3 +");
        assertEquals("$5 + 3$", result);
    }

    @Test
    void testChainedAddition() throws RpnException {
        String result = process("1 2 + 3 + 4 +");
        assertEquals("$1 + 2 + 3 + 4$", result);
    }

    @Test
    void testAdditionWithDecimals() throws RpnException {
        String result = process("1.5 0.5 +");
        assertEquals("$1.5 + 0.5$", result);
    }

    @Test
    void testParserErrorTooManyOperands() {
        assertThrows(ParserException.class, () -> process("5 3"));
    }

    @Test
    void testParserErrorInsufficientOperands() {
        assertThrows(ParserException.class, () -> process("5 +"));
    }

    @Test
    void testParserErrorEmptyExpression() {
        assertThrows(ParserException.class, () -> process(""));
    }

    @Test
    void testSimpleSubtraction() throws RpnException {
        String result = process("5 3 -");
        assertEquals("$5 - 3$", result);
    }

    @Test
    void testChainedSubtraction() throws RpnException {
        String result = process("5 3 - 2 -");
        assertEquals("$5 - 3 - 2$", result);
    }

    @Test
    void testSubtractionWithDecimals() throws RpnException {
        String result = process("10.5 3.2 -");
        assertEquals("$10.5 - 3.2$", result);
    }

    @Test
    void testNegativeNumberVsSubtraction() throws RpnException {
        // -5 is a negative number
        String result1 = process("-5");
        assertEquals("$-5$", result1);

        // 5 -3 should be 5 and -3 as separate numbers (not subtraction)
        // This should throw ParserException: too many operands
        assertThrows(ParserException.class, () -> process("5 -3"));
    }

    @Test
    void testSubtractionWithNegativeOperand() throws RpnException {
        // 5 -3 - means subtract -3 from 5
        String result = process("5 -3 -");
        assertEquals("$5 - -3$", result);
    }

    @Test
    void testSimpleMultiplication() throws RpnException {
        String result = process("4 7 *");
        assertEquals("$4 \\times 7$", result);
    }

    @Test
    void testMultiplicationWithDecimals() throws RpnException {
        String result = process("3.14 2 *");
        assertEquals("$3.14 \\times 2$", result);
    }

    @Test
    void testMultiplicationWithAddition() throws RpnException {
        // Multiplication has higher precedence, no parens needed
        String result = process("2 3 4 * +");
        assertEquals("$2 + 3 \\times 4$", result);
    }

    @Test
    void testAdditionTimesConstant() throws RpnException {
        // Addition has lower precedence, needs parens
        String result = process("5 3 + 2 *");
        assertEquals("$( 5 + 3 ) \\times 2$", result);
    }

    @Test
    void testMultiplicationPlusConstant() throws RpnException {
        // Multiplication as left operand, no parens needed
        String result = process("2 3 * 4 +");
        assertEquals("$2 \\times 3 + 4$", result);
    }

    @Test
    void testChainedMultiplication() throws RpnException {
        // Left-associative: (2 * 3) * 4
        String result = process("2 3 * 4 *");
        assertEquals("$2 \\times 3 \\times 4$", result);
    }

    @Test
    void testMultiplicationBothOperandsAdditions() throws RpnException {
        // (1 + 2) * (3 + 4)
        String result = process("1 2 + 3 4 + *");
        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 )$", result);
    }

    @Test
    void testSimpleDivision() throws RpnException {
        String result = process("10 2 /");
        assertEquals("$10 \\div 2$", result);
    }

    @Test
    void testChainedDivision() throws RpnException {
        // Left-associative: ((100 / 10) / 5) / 2
        String result = process("100 10 / 5 / 2 /");
        assertEquals("$100 \\div 10 \\div 5 \\div 2$", result);
    }

    @Test
    void testDivisionWithDecimals() throws RpnException {
        String result = process("10.5 2.5 /");
        assertEquals("$10.5 \\div 2.5$", result);
    }

    @Test
    void testDivisionWithAddition() throws RpnException {
        // Division has higher precedence than addition
        String result = process("10 2 / 3 +");
        assertEquals("$10 \\div 2 + 3$", result);
    }

    @Test
    void testAdditionTimesDivision() throws RpnException {
        // (10 / 2 + 3) * 4 - from I/O contract
        String result = process("10 2 / 3 + 4 *");
        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$", result);
    }

    @Test
    void testDivisionAndMultiplicationSamePrecedence() throws RpnException {
        // 10 / 2 * 5 - from I/O contract
        String result = process("10 2 / 5 *");
        assertEquals("$10 \\div 2 \\times 5$", result);
    }

    @Test
    void testNonCommutativityOfDivision() throws RpnException {
        // Ensure 10 / 2 produces 10 div 2, not 2 div 10
        String result = process("10 2 /");
        assertEquals("$10 \\div 2$", result);
    }
}
