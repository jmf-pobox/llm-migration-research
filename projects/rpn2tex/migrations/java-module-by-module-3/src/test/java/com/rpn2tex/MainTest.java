package com.rpn2tex;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.PrintStream;
import java.nio.file.Files;
import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Comprehensive test suite for Main CLI class.
 *
 * <p>Tests the complete end-to-end pipeline including:
 * <ul>
 *   <li>All 21 I/O contract test cases</li>
 *   <li>Argument parsing</li>
 *   <li>File I/O operations</li>
 *   <li>Error handling and formatting</li>
 *   <li>Exit codes</li>
 * </ul>
 */
class MainTest {
    private final PrintStream originalOut = System.out;
    private final PrintStream originalErr = System.err;
    private final InputStream originalIn = System.in;
    private ByteArrayOutputStream testOut;
    private ByteArrayOutputStream testErr;

    @BeforeEach
    void setUp() {
        testOut = new ByteArrayOutputStream();
        testErr = new ByteArrayOutputStream();
        System.setOut(new PrintStream(testOut));
        System.setErr(new PrintStream(testErr));
    }

    @AfterEach
    void tearDown() {
        System.setOut(originalOut);
        System.setErr(originalErr);
        System.setIn(originalIn);
    }

    /**
     * Parameterized test for all 18 successful I/O contract cases.
     * These should all exit with code 0 and produce exact LaTeX output.
     */
    @ParameterizedTest(name = "Test {index}: {0}")
    @CsvSource(delimiterString = "|||", value = {
        "5 3 +|||$5 + 3$",
        "5 3 -|||$5 - 3$",
        "4 7 *|||$4 \\times 7$",
        "10 2 /|||$10 \\div 2$",
        "5 3 + 2 *|||$( 5 + 3 ) \\times 2$",
        "5 3 * 2 +|||$5 \\times 3 + 2$",
        "10 2 / 5 *|||$10 \\div 2 \\times 5$",
        "5 3 - 2 -|||$5 - 3 - 2$",
        "100 10 / 5 / 2 /|||$100 \\div 10 \\div 5 \\div 2$",
        "1 2 + 3 + 4 +|||$1 + 2 + 3 + 4$",
        "2 3 4 * +|||$2 + 3 \\times 4$",
        "2 3 + 4 *|||$( 2 + 3 ) \\times 4$",
        "2 3 4 + *|||$2 \\times ( 3 + 4 )$",
        "2 3 * 4 +|||$2 \\times 3 + 4$",
        "3.14 2 *|||$3.14 \\times 2$",
        "1.5 0.5 +|||$1.5 + 0.5$",
        "1 2 + 3 4 + *|||$( 1 + 2 ) \\times ( 3 + 4 )$",
        "10 2 / 3 + 4 *|||$( 10 \\div 2 + 3 ) \\times 4$"
    })
    void testIOContractSuccess(String input, String expectedOutput) {
        int exitCode = Main.run(new String[]{"-", "-o", null});

        // Simulate stdin input
        System.setIn(new ByteArrayInputStream(input.getBytes()));
        exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode, "Should exit with code 0 for valid input");
        String output = testOut.toString().trim();
        assertEquals(expectedOutput, output, "LaTeX output should match expected");
    }

    /**
     * Test case 1: Basic addition.
     */
    @Test
    void testCase1_BasicAddition() {
        System.setIn(new ByteArrayInputStream("5 3 +".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$5 + 3$", testOut.toString().trim());
    }

    /**
     * Test case 2: Basic subtraction.
     */
    @Test
    void testCase2_BasicSubtraction() {
        System.setIn(new ByteArrayInputStream("5 3 -".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$5 - 3$", testOut.toString().trim());
    }

    /**
     * Test case 3: Basic multiplication.
     */
    @Test
    void testCase3_BasicMultiplication() {
        System.setIn(new ByteArrayInputStream("4 7 *".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$4 \\times 7$", testOut.toString().trim());
    }

    /**
     * Test case 4: Basic division.
     */
    @Test
    void testCase4_BasicDivision() {
        System.setIn(new ByteArrayInputStream("10 2 /".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$10 \\div 2$", testOut.toString().trim());
    }

    /**
     * Test case 5: Exponentiation (unsupported operator) - should fail.
     */
    @Test
    void testCase5_ExponentiationError() {
        System.setIn(new ByteArrayInputStream("2 3 ^".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(1, exitCode, "Should exit with code 1 for unsupported operator");
        String errorOutput = testErr.toString();
        assertTrue(errorOutput.contains("Error: Unexpected character '^'"),
            "Should report unexpected character error");
        assertTrue(errorOutput.contains("^"), "Should show caret pointer");
    }

    /**
     * Test case 6: Precedence with parentheses.
     */
    @Test
    void testCase6_PrecedenceWithParens() {
        System.setIn(new ByteArrayInputStream("5 3 + 2 *".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$( 5 + 3 ) \\times 2$", testOut.toString().trim());
    }

    /**
     * Test case 7: Precedence without parentheses.
     */
    @Test
    void testCase7_PrecedenceNoParens() {
        System.setIn(new ByteArrayInputStream("5 3 * 2 +".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$5 \\times 3 + 2$", testOut.toString().trim());
    }

    /**
     * Test case 8: Left-associative operations.
     */
    @Test
    void testCase8_LeftAssociative() {
        System.setIn(new ByteArrayInputStream("10 2 / 5 *".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$10 \\div 2 \\times 5$", testOut.toString().trim());
    }

    /**
     * Test case 9: Chained subtraction.
     */
    @Test
    void testCase9_ChainedSubtraction() {
        System.setIn(new ByteArrayInputStream("5 3 - 2 -".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$5 - 3 - 2$", testOut.toString().trim());
    }

    /**
     * Test case 10: Chained division.
     */
    @Test
    void testCase10_ChainedDivision() {
        System.setIn(new ByteArrayInputStream("100 10 / 5 / 2 /".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$100 \\div 10 \\div 5 \\div 2$", testOut.toString().trim());
    }

    /**
     * Test case 11: Chained addition.
     */
    @Test
    void testCase11_ChainedAddition() {
        System.setIn(new ByteArrayInputStream("1 2 + 3 + 4 +".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$1 + 2 + 3 + 4$", testOut.toString().trim());
    }

    /**
     * Test case 12: Mixed operations without parentheses.
     */
    @Test
    void testCase12_MixedOpsNoParens() {
        System.setIn(new ByteArrayInputStream("2 3 4 * +".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$2 + 3 \\times 4$", testOut.toString().trim());
    }

    /**
     * Test case 13: Addition parenthesized.
     */
    @Test
    void testCase13_AdditionParenthesized() {
        System.setIn(new ByteArrayInputStream("2 3 + 4 *".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$( 2 + 3 ) \\times 4$", testOut.toString().trim());
    }

    /**
     * Test case 14: Right operand with parentheses.
     */
    @Test
    void testCase14_RightOperandParens() {
        System.setIn(new ByteArrayInputStream("2 3 4 + *".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$2 \\times ( 3 + 4 )$", testOut.toString().trim());
    }

    /**
     * Test case 15: Multiplication then addition.
     */
    @Test
    void testCase15_MultThenAdd() {
        System.setIn(new ByteArrayInputStream("2 3 * 4 +".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$2 \\times 3 + 4$", testOut.toString().trim());
    }

    /**
     * Test case 16: Exponentiation in expression - should fail.
     */
    @Test
    void testCase16_ExponentiationInExpression() {
        System.setIn(new ByteArrayInputStream("2 3 ^ 4 *".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(1, exitCode, "Should exit with code 1 for unsupported operator");
        String errorOutput = testErr.toString();
        assertTrue(errorOutput.contains("Error: Unexpected character '^'"),
            "Should report unexpected character error");
    }

    /**
     * Test case 17: Multiple exponentiation operators - should fail.
     */
    @Test
    void testCase17_MultipleExponentiation() {
        System.setIn(new ByteArrayInputStream("2 3 4 ^ ^".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(1, exitCode, "Should exit with code 1 for unsupported operator");
        String errorOutput = testErr.toString();
        assertTrue(errorOutput.contains("Error: Unexpected character '^'"),
            "Should report unexpected character error");
    }

    /**
     * Test case 18: Decimal numbers.
     */
    @Test
    void testCase18_DecimalNumbers() {
        System.setIn(new ByteArrayInputStream("3.14 2 *".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$3.14 \\times 2$", testOut.toString().trim());
    }

    /**
     * Test case 19: Decimal addition.
     */
    @Test
    void testCase19_DecimalAddition() {
        System.setIn(new ByteArrayInputStream("1.5 0.5 +".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$1.5 + 0.5$", testOut.toString().trim());
    }

    /**
     * Test case 20: Double parenthesized expression.
     */
    @Test
    void testCase20_DoubleParenthesized() {
        System.setIn(new ByteArrayInputStream("1 2 + 3 4 + *".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 )$", testOut.toString().trim());
    }

    /**
     * Test case 21: Complex mixed operations.
     */
    @Test
    void testCase21_ComplexMixed() {
        System.setIn(new ByteArrayInputStream("10 2 / 3 + 4 *".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode);
        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$", testOut.toString().trim());
    }

    /**
     * Test argument parsing: missing input argument.
     */
    @Test
    void testMissingInputArgument() {
        int exitCode = Main.run(new String[]{});

        assertEquals(1, exitCode, "Should exit with code 1 for missing input");
        String errorOutput = testErr.toString();
        assertTrue(errorOutput.contains("Usage:"), "Should show usage message");
    }

    /**
     * Test argument parsing: -o without file path.
     */
    @Test
    void testOutputWithoutPath() {
        int exitCode = Main.run(new String[]{"-", "-o"});

        assertEquals(1, exitCode, "Should exit with code 1 for -o without path");
        String errorOutput = testErr.toString();
        assertTrue(errorOutput.contains("-o/--output requires a file path"),
            "Should report missing output path");
    }

    /**
     * Test file I/O: read from file, write to file.
     */
    @Test
    void testFileInputOutput() throws IOException {
        // Create temporary input file
        Path inputFile = Files.createTempFile("rpn_input_", ".txt");
        Files.writeString(inputFile, "5 3 +");

        // Create temporary output file
        Path outputFile = Files.createTempFile("rpn_output_", ".tex");

        try {
            int exitCode = Main.run(new String[]{
                inputFile.toString(),
                "-o",
                outputFile.toString()
            });

            assertEquals(0, exitCode, "Should exit with code 0");
            String output = Files.readString(outputFile);
            assertEquals("$5 + 3$\n", output, "Output file should contain LaTeX with newline");

            // Check stderr for generation message
            String errOutput = testErr.toString();
            assertTrue(errOutput.contains("Generated:"), "Should print generation message to stderr");
        } finally {
            // Clean up
            Files.deleteIfExists(inputFile);
            Files.deleteIfExists(outputFile);
        }
    }

    /**
     * Test stdin input with file output.
     */
    @Test
    void testStdinInputFileOutput() throws IOException {
        System.setIn(new ByteArrayInputStream("5 3 +".getBytes()));
        Path outputFile = Files.createTempFile("rpn_output_", ".tex");

        try {
            int exitCode = Main.run(new String[]{"-", "-o", outputFile.toString()});

            assertEquals(0, exitCode, "Should exit with code 0");
            String output = Files.readString(outputFile);
            assertEquals("$5 + 3$\n", output, "Output file should contain LaTeX with newline");
        } finally {
            Files.deleteIfExists(outputFile);
        }
    }

    /**
     * Test file input with stdout output.
     */
    @Test
    void testFileInputStdoutOutput() throws IOException {
        Path inputFile = Files.createTempFile("rpn_input_", ".txt");
        Files.writeString(inputFile, "5 3 +");

        try {
            int exitCode = Main.run(new String[]{inputFile.toString()});

            assertEquals(0, exitCode, "Should exit with code 0");
            assertEquals("$5 + 3$", testOut.toString().trim(),
                "Stdout should contain LaTeX");
        } finally {
            Files.deleteIfExists(inputFile);
        }
    }

    /**
     * Test error handling: nonexistent input file.
     */
    @Test
    void testNonexistentInputFile() {
        int exitCode = Main.run(new String[]{"/nonexistent/file.txt"});

        assertEquals(1, exitCode, "Should exit with code 1 for nonexistent file");
        String errorOutput = testErr.toString();
        assertTrue(errorOutput.contains("Error:"), "Should report error");
    }

    /**
     * Test empty input expression.
     */
    @Test
    void testEmptyInput() {
        System.setIn(new ByteArrayInputStream("".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(1, exitCode, "Should exit with code 1 for empty input");
        String errorOutput = testErr.toString();
        assertTrue(errorOutput.contains("Empty expression"), "Should report empty expression");
    }

    /**
     * Test insufficient operands error.
     */
    @Test
    void testInsufficientOperands() {
        System.setIn(new ByteArrayInputStream("5 +".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(1, exitCode, "Should exit with code 1 for insufficient operands");
        String errorOutput = testErr.toString();
        assertTrue(errorOutput.contains("requires two operands"),
            "Should report insufficient operands");
    }

    /**
     * Test extra operands error (missing operators).
     */
    @Test
    void testExtraOperands() {
        System.setIn(new ByteArrayInputStream("5 3 2".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(1, exitCode, "Should exit with code 1 for extra operands");
        String errorOutput = testErr.toString();
        assertTrue(errorOutput.contains("values remain on stack"),
            "Should report extra operands");
    }

    /**
     * Test error formatting includes source context.
     */
    @Test
    void testErrorFormattingWithContext() {
        System.setIn(new ByteArrayInputStream("2 3 ^ 4".getBytes()));
        int exitCode = Main.run(new String[]{"-"});

        assertEquals(1, exitCode, "Should exit with code 1 for error");
        String errorOutput = testErr.toString();

        // Check for proper error formatting
        assertTrue(errorOutput.contains("Error:"), "Should have error header");
        assertTrue(errorOutput.contains("2 3 ^ 4"), "Should show source line");
        assertTrue(errorOutput.contains("^"), "Should show caret pointer");
        assertTrue(errorOutput.contains("| "), "Should show line number prefix");
    }
}
