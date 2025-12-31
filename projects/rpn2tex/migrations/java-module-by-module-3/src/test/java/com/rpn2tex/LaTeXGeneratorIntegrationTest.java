package com.rpn2tex;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests for the complete RPN to LaTeX pipeline.
 *
 * <p>Tests the full flow: RPN input → Lexer → Parser → LaTeXGenerator → LaTeX output.
 * Validates the I/O contract from Phase 0 to ensure exact output matching.
 */
class LaTeXGeneratorIntegrationTest {
    private final LaTeXGenerator generator = new LaTeXGenerator();

    /**
     * Helper method to convert RPN input to LaTeX output using the full pipeline.
     */
    private String rpnToLatex(String rpnInput) throws RpnException {
        Lexer lexer = new Lexer(rpnInput);
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        return generator.generate(ast);
    }

    /**
     * Tests all successful I/O contract cases from Phase 0.
     * These test cases must produce exact LaTeX output matching the Python implementation.
     */
    @ParameterizedTest(name = "{index}: {0} -> {1}")
    @CsvSource(delimiter = '|', textBlock = """
        5 3 +                   | $5 + 3$
        5 3 -                   | $5 - 3$
        4 7 *                   | $4 \\times 7$
        10 2 /                  | $10 \\div 2$
        5 3 + 2 *               | $( 5 + 3 ) \\times 2$
        5 3 * 2 +               | $5 \\times 3 + 2$
        10 2 / 5 *              | $10 \\div 2 \\times 5$
        5 3 - 2 -               | $5 - 3 - 2$
        100 10 / 5 / 2 /        | $100 \\div 10 \\div 5 \\div 2$
        1 2 + 3 + 4 +           | $1 + 2 + 3 + 4$
        2 3 4 * +               | $2 + 3 \\times 4$
        2 3 + 4 *               | $( 2 + 3 ) \\times 4$
        2 3 4 + *               | $2 \\times ( 3 + 4 )$
        2 3 * 4 +               | $2 \\times 3 + 4$
        3.14 2 *                | $3.14 \\times 2$
        1.5 0.5 +               | $1.5 + 0.5$
        1 2 + 3 4 + *           | $( 1 + 2 ) \\times ( 3 + 4 )$
        10 2 / 3 + 4 *          | $( 10 \\div 2 + 3 ) \\times 4$
        """)
    void testIOContractSuccess(String rpnInput, String expectedLatex) throws RpnException {
        String actualLatex = rpnToLatex(rpnInput);
        assertEquals(expectedLatex, actualLatex,
            String.format("Failed for input: %s", rpnInput));
    }

    /**
     * Tests that the LaTeX generator properly handles decimal preservation.
     */
    @ParameterizedTest(name = "Decimal test: {0}")
    @CsvSource(delimiter = '|', textBlock = """
        3.14          | $3.14$
        1.5 0.5 +     | $1.5 + 0.5$
        3.14 2 *      | $3.14 \\times 2$
        """)
    void testDecimalPreservation(String rpnInput, String expectedLatex) throws RpnException {
        String actualLatex = rpnToLatex(rpnInput);
        assertEquals(expectedLatex, actualLatex);
    }

    /**
     * Tests precedence handling to ensure parentheses are inserted correctly.
     */
    @ParameterizedTest(name = "Precedence test: {0}")
    @CsvSource(delimiter = '|', textBlock = """
        2 3 + 4 *               | $( 2 + 3 ) \\times 4$
        2 3 4 + *               | $2 \\times ( 3 + 4 )$
        5 3 + 2 *               | $( 5 + 3 ) \\times 2$
        2 3 4 * +               | $2 + 3 \\times 4$
        5 3 * 2 +               | $5 \\times 3 + 2$
        """)
    void testPrecedenceHandling(String rpnInput, String expectedLatex) throws RpnException {
        String actualLatex = rpnToLatex(rpnInput);
        assertEquals(expectedLatex, actualLatex);
    }

    /**
     * Tests left-associativity for subtraction and division.
     */
    @ParameterizedTest(name = "Associativity test: {0}")
    @CsvSource(delimiter = '|', textBlock = """
        5 3 - 2 -               | $5 - 3 - 2$
        100 10 / 5 / 2 /        | $100 \\div 10 \\div 5 \\div 2$
        """)
    void testLeftAssociativity(String rpnInput, String expectedLatex) throws RpnException {
        String actualLatex = rpnToLatex(rpnInput);
        assertEquals(expectedLatex, actualLatex);
    }

    /**
     * Tests complex nested expressions with multiple operators.
     */
    @ParameterizedTest(name = "Complex test: {0}")
    @CsvSource(delimiter = '|', textBlock = """
        1 2 + 3 4 + *           | $( 1 + 2 ) \\times ( 3 + 4 )$
        10 2 / 3 + 4 *          | $( 10 \\div 2 + 3 ) \\times 4$
        1 2 + 3 + 4 +           | $1 + 2 + 3 + 4$
        """)
    void testComplexExpressions(String rpnInput, String expectedLatex) throws RpnException {
        String actualLatex = rpnToLatex(rpnInput);
        assertEquals(expectedLatex, actualLatex);
    }
}
