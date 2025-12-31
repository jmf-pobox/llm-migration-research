package com.rpn2tex;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests for the full pipeline: Lexer → Parser → LaTeXGenerator.
 *
 * <p>These tests validate that the complete processing pipeline produces
 * the exact output specified in the I/O contract for all test cases.</p>
 */
class LaTeXGeneratorIntegrationTest {

    private Lexer lexer;
    private Parser parser;
    private LaTeXGenerator generator;

    @BeforeEach
    void setUp() {
        generator = new LaTeXGenerator();
    }

    /**
     * Helper method to run the full pipeline: tokenize → parse → generate.
     */
    private String processRPN(String input) throws Exception {
        lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

        parser = new Parser(tokens);
        Expr ast = parser.parse();

        return generator.generate(ast);
    }

    // ========== I/O Contract Test Cases ==========

    @ParameterizedTest
    @CsvSource(delimiter = '|', value = {
        "5 3 +               | $5 + 3$",
        "5 3 -               | $5 - 3$",
        "4 7 *               | $4 \\times 7$",
        "10 2 /              | $10 \\div 2$",
        "5 3 + 2 *           | $( 5 + 3 ) \\times 2$",
        "5 3 * 2 +           | $5 \\times 3 + 2$",
        "10 2 / 5 *          | $10 \\div 2 \\times 5$",
        "5 3 - 2 -           | $5 - 3 - 2$",
        "100 10 / 5 / 2 /    | $100 \\div 10 \\div 5 \\div 2$",
        "1 2 + 3 + 4 +       | $1 + 2 + 3 + 4$",
        "2 3 4 * +           | $2 + 3 \\times 4$",
        "2 3 + 4 *           | $( 2 + 3 ) \\times 4$",
        "2 3 4 + *           | $2 \\times ( 3 + 4 )$",
        "2 3 * 4 +           | $2 \\times 3 + 4$",
        "3.14 2 *            | $3.14 \\times 2$",
        "1.5 0.5 +           | $1.5 + 0.5$",
        "1 2 + 3 4 + *       | $( 1 + 2 ) \\times ( 3 + 4 )$",
        "10 2 / 3 + 4 *      | $( 10 \\div 2 + 3 ) \\times 4$"
    })
    @DisplayName("I/O Contract: Full pipeline validation")
    void testIOContractFullPipeline(String input, String expectedOutput) throws Exception {
        String actualOutput = processRPN(input);
        assertEquals(expectedOutput, actualOutput,
            String.format("Input: '%s' produced incorrect output", input));
    }

    @ParameterizedTest
    @CsvSource(delimiter = '|', value = {
        "5 3 +               | $5 + 3$                      | Basic addition",
        "5 3 -               | $5 - 3$                      | Basic subtraction",
        "4 7 *               | $4 \\times 7$                | Basic multiplication with \\times",
        "10 2 /              | $10 \\div 2$                 | Basic division with \\div"
    })
    @DisplayName("I/O Contract: Basic operations")
    void testIOContractBasicOperations(String input, String expectedOutput, String description) throws Exception {
        String actualOutput = processRPN(input);
        assertEquals(expectedOutput, actualOutput, description);
    }

    @ParameterizedTest
    @CsvSource(delimiter = '|', value = {
        "5 3 + 2 *           | $( 5 + 3 ) \\times 2$        | Parentheses added for operator precedence",
        "5 3 * 2 +           | $5 \\times 3 + 2$            | No parentheses when precedence is natural",
        "2 3 4 * +           | $2 + 3 \\times 4$            | Multiplication has higher precedence",
        "2 3 + 4 *           | $( 2 + 3 ) \\times 4$        | Parentheses for lower precedence operation",
        "2 3 4 + *           | $2 \\times ( 3 + 4 )$        | Parentheses around addition in multiplication",
        "2 3 * 4 +           | $2 \\times 3 + 4$            | Multiplication before addition"
    })
    @DisplayName("I/O Contract: Precedence handling")
    void testIOContractPrecedence(String input, String expectedOutput, String description) throws Exception {
        String actualOutput = processRPN(input);
        assertEquals(expectedOutput, actualOutput, description);
    }

    @ParameterizedTest
    @CsvSource(delimiter = '|', value = {
        "10 2 / 5 *          | $10 \\div 2 \\times 5$       | Left-to-right evaluation for same precedence",
        "5 3 - 2 -           | $5 - 3 - 2$                  | Multiple subtractions",
        "100 10 / 5 / 2 /    | $100 \\div 10 \\div 5 \\div 2$ | Multiple divisions",
        "1 2 + 3 + 4 +       | $1 + 2 + 3 + 4$              | Multiple additions"
    })
    @DisplayName("I/O Contract: Associativity")
    void testIOContractAssociativity(String input, String expectedOutput, String description) throws Exception {
        String actualOutput = processRPN(input);
        assertEquals(expectedOutput, actualOutput, description);
    }

    @ParameterizedTest
    @CsvSource(delimiter = '|', value = {
        "3.14 2 *            | $3.14 \\times 2$             | Floating point numbers supported",
        "1.5 0.5 +           | $1.5 + 0.5$                  | Floating point addition"
    })
    @DisplayName("I/O Contract: Floating point support")
    void testIOContractFloatingPoint(String input, String expectedOutput, String description) throws Exception {
        String actualOutput = processRPN(input);
        assertEquals(expectedOutput, actualOutput, description);
    }

    @ParameterizedTest
    @CsvSource(delimiter = '|', value = {
        "1 2 + 3 4 + *       | $( 1 + 2 ) \\times ( 3 + 4 )$ | Multiple complex subexpressions",
        "10 2 / 3 + 4 *      | $( 10 \\div 2 + 3 ) \\times 4$ | Mixed operations with precedence"
    })
    @DisplayName("I/O Contract: Complex expressions")
    void testIOContractComplexExpressions(String input, String expectedOutput, String description) throws Exception {
        String actualOutput = processRPN(input);
        assertEquals(expectedOutput, actualOutput, description);
    }
}
