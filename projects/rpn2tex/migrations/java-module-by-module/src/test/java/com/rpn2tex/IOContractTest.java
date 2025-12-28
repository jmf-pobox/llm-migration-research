package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * I/O Contract validation tests for rpn2tex migration.
 *
 * <p>This test suite validates that the Java implementation produces identical
 * outputs to the Python implementation for all 21 test cases defined in the
 * I/O contract specification.
 *
 * <p>Test cases cover:
 * <ul>
 *   <li>Basic operations (addition, subtraction, multiplication, division)</li>
 *   <li>Operator precedence and associativity</li>
 *   <li>Complex nested expressions with proper parenthesization</li>
 *   <li>Floating-point number support</li>
 *   <li>Error cases (unsupported operators)</li>
 * </ul>
 *
 * <p>Reference: /Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/io_contract.md
 */
class IOContractTest {

    /**
     * Helper method to process RPN input through the full pipeline.
     */
    private String process(String input) throws RpnException {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        return generator.generate(ast);
    }

    @Test
    @DisplayName("Test 1: Basic addition - 5 3 +")
    void test01_basicAddition() throws RpnException {
        assertEquals("$5 + 3$", process("5 3 +"));
    }

    @Test
    @DisplayName("Test 2: Basic subtraction - 5 3 -")
    void test02_basicSubtraction() throws RpnException {
        assertEquals("$5 - 3$", process("5 3 -"));
    }

    @Test
    @DisplayName("Test 3: Basic multiplication - 4 7 *")
    void test03_basicMultiplication() throws RpnException {
        assertEquals("$4 \\times 7$", process("4 7 *"));
    }

    @Test
    @DisplayName("Test 4: Basic division - 10 2 /")
    void test04_basicDivision() throws RpnException {
        assertEquals("$10 \\div 2$", process("10 2 /"));
    }

    @Test
    @DisplayName("Test 5: Caret operator - 2 3 ^ (ERROR)")
    void test05_caretOperatorError() {
        RpnException exception = assertThrows(RpnException.class, () -> process("2 3 ^"));
        assertTrue(exception.getMessage().contains("Unexpected character"),
            "Expected 'Unexpected character' error, got: " + exception.getMessage());
    }

    @Test
    @DisplayName("Test 6: Addition grouped for multiplication - 5 3 + 2 *")
    void test06_additionGroupedForMultiplication() throws RpnException {
        assertEquals("$( 5 + 3 ) \\times 2$", process("5 3 + 2 *"));
    }

    @Test
    @DisplayName("Test 7: Multiplication has higher precedence - 5 3 * 2 +")
    void test07_multiplicationHigherPrecedence() throws RpnException {
        assertEquals("$5 \\times 3 + 2$", process("5 3 * 2 +"));
    }

    @Test
    @DisplayName("Test 8: Division and multiplication left-associative - 10 2 / 5 *")
    void test08_divisionMultiplicationLeftAssociative() throws RpnException {
        assertEquals("$10 \\div 2 \\times 5$", process("10 2 / 5 *"));
    }

    @Test
    @DisplayName("Test 9: Subtraction left-associative - 5 3 - 2 -")
    void test09_subtractionLeftAssociative() throws RpnException {
        assertEquals("$5 - 3 - 2$", process("5 3 - 2 -"));
    }

    @Test
    @DisplayName("Test 10: Chained division left-associative - 100 10 / 5 / 2 /")
    void test10_chainedDivisionLeftAssociative() throws RpnException {
        assertEquals("$100 \\div 10 \\div 5 \\div 2$", process("100 10 / 5 / 2 /"));
    }

    @Test
    @DisplayName("Test 11: Chained addition - 1 2 + 3 + 4 +")
    void test11_chainedAddition() throws RpnException {
        assertEquals("$1 + 2 + 3 + 4$", process("1 2 + 3 + 4 +"));
    }

    @Test
    @DisplayName("Test 12: Multiplication higher precedence than addition - 2 3 4 * +")
    void test12_multiplicationHigherThanAddition() throws RpnException {
        assertEquals("$2 + 3 \\times 4$", process("2 3 4 * +"));
    }

    @Test
    @DisplayName("Test 13: Addition grouped when multiplied - 2 3 + 4 *")
    void test13_additionGroupedWhenMultiplied() throws RpnException {
        assertEquals("$( 2 + 3 ) \\times 4$", process("2 3 + 4 *"));
    }

    @Test
    @DisplayName("Test 14: Addition grouped on right side - 2 3 4 + *")
    void test14_additionGroupedOnRight() throws RpnException {
        assertEquals("$2 \\times ( 3 + 4 )$", process("2 3 4 + *"));
    }

    @Test
    @DisplayName("Test 15: Multiplication has higher precedence - 2 3 * 4 +")
    void test15_multiplicationHigherPrecedence2() throws RpnException {
        assertEquals("$2 \\times 3 + 4$", process("2 3 * 4 +"));
    }

    @Test
    @DisplayName("Test 16: Caret operator in complex expression - 2 3 ^ 4 * (ERROR)")
    void test16_caretOperatorComplexError() {
        RpnException exception = assertThrows(RpnException.class, () -> process("2 3 ^ 4 *"));
        assertTrue(exception.getMessage().contains("Unexpected character"),
            "Expected 'Unexpected character' error, got: " + exception.getMessage());
    }

    @Test
    @DisplayName("Test 17: Multiple caret operators - 2 3 4 ^ ^ (ERROR)")
    void test17_multipleCaretOperatorsError() {
        RpnException exception = assertThrows(RpnException.class, () -> process("2 3 4 ^ ^"));
        assertTrue(exception.getMessage().contains("Unexpected character"),
            "Expected 'Unexpected character' error, got: " + exception.getMessage());
    }

    @Test
    @DisplayName("Test 18: Floating point multiplication - 3.14 2 *")
    void test18_floatingPointMultiplication() throws RpnException {
        assertEquals("$3.14 \\times 2$", process("3.14 2 *"));
    }

    @Test
    @DisplayName("Test 19: Floating point addition - 1.5 0.5 +")
    void test19_floatingPointAddition() throws RpnException {
        assertEquals("$1.5 + 0.5$", process("1.5 0.5 +"));
    }

    @Test
    @DisplayName("Test 20: Complex with multiple sub-expressions - 1 2 + 3 4 + *")
    void test20_complexMultipleSubExpressions() throws RpnException {
        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 )$", process("1 2 + 3 4 + *"));
    }

    @Test
    @DisplayName("Test 21: Complex with division and addition grouped - 10 2 / 3 + 4 *")
    void test21_complexDivisionAdditionGrouped() throws RpnException {
        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$", process("10 2 / 3 + 4 *"));
    }

    @Test
    @DisplayName("Summary: All 18 successful test cases pass")
    void testSummary() throws RpnException {
        // This test runs all successful cases in sequence to verify the complete contract
        String[][] testCases = {
            {"5 3 +", "$5 + 3$"},
            {"5 3 -", "$5 - 3$"},
            {"4 7 *", "$4 \\times 7$"},
            {"10 2 /", "$10 \\div 2$"},
            {"5 3 + 2 *", "$( 5 + 3 ) \\times 2$"},
            {"5 3 * 2 +", "$5 \\times 3 + 2$"},
            {"10 2 / 5 *", "$10 \\div 2 \\times 5$"},
            {"5 3 - 2 -", "$5 - 3 - 2$"},
            {"100 10 / 5 / 2 /", "$100 \\div 10 \\div 5 \\div 2$"},
            {"1 2 + 3 + 4 +", "$1 + 2 + 3 + 4$"},
            {"2 3 4 * +", "$2 + 3 \\times 4$"},
            {"2 3 + 4 *", "$( 2 + 3 ) \\times 4$"},
            {"2 3 4 + *", "$2 \\times ( 3 + 4 )$"},
            {"2 3 * 4 +", "$2 \\times 3 + 4$"},
            {"3.14 2 *", "$3.14 \\times 2$"},
            {"1.5 0.5 +", "$1.5 + 0.5$"},
            {"1 2 + 3 4 + *", "$( 1 + 2 ) \\times ( 3 + 4 )$"},
            {"10 2 / 3 + 4 *", "$( 10 \\div 2 + 3 ) \\times 4$"}
        };

        int passed = 0;
        int failed = 0;
        StringBuilder failures = new StringBuilder();

        for (String[] testCase : testCases) {
            String input = testCase[0];
            String expected = testCase[1];
            try {
                String actual = process(input);
                if (actual.equals(expected)) {
                    passed++;
                } else {
                    failed++;
                    failures.append(String.format("FAILED: '%s' expected '%s' but got '%s'\n",
                        input, expected, actual));
                }
            } catch (Exception e) {
                failed++;
                failures.append(String.format("ERROR: '%s' threw exception: %s\n",
                    input, e.getMessage()));
            }
        }

        if (failed > 0) {
            fail(String.format("I/O Contract validation failed:\n%d passed, %d failed\n\n%s",
                passed, failed, failures));
        }

        assertEquals(18, passed, "Expected all 18 successful test cases to pass");
    }
}
