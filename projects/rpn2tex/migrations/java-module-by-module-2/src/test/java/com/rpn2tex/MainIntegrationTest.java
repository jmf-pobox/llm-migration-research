package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.PrintStream;
import java.nio.file.Files;
import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.*;

/**
 * End-to-end integration tests for the rpn2tex CLI.
 *
 * <p>These tests validate the complete I/O contract by testing the entire
 * pipeline from input to output, ensuring that all components work correctly
 * together.
 */
class MainIntegrationTest {

    /**
     * Tests all I/O contract test cases in a single comprehensive test.
     *
     * <p>This test verifies that the complete pipeline produces exactly
     * the expected output for all valid inputs from the I/O contract.
     */
    @ParameterizedTest
    @CsvSource({
        // Basic operations
        "'5 3 +', '$5 + 3$'",
        "'5 3 -', '$5 - 3$'",
        "'4 7 *', '$4 \\times 7$'",
        "'10 2 /', '$10 \\div 2$'",

        // Complex operations with precedence
        "'5 3 + 2 *', '$( 5 + 3 ) \\times 2$'",
        "'5 3 * 2 +', '$5 \\times 3 + 2$'",
        "'10 2 / 5 *', '$10 \\div 2 \\times 5$'",
        "'5 3 - 2 -', '$5 - 3 - 2$'",
        "'100 10 / 5 / 2 /', '$100 \\div 10 \\div 5 \\div 2$'",
        "'1 2 + 3 + 4 +', '$1 + 2 + 3 + 4$'",
        "'2 3 4 * +', '$2 + 3 \\times 4$'",
        "'2 3 + 4 *', '$( 2 + 3 ) \\times 4$'",
        "'2 3 4 + *', '$2 \\times ( 3 + 4 )$'",
        "'2 3 * 4 +', '$2 \\times 3 + 4$'",

        // Floating point
        "'3.14 2 *', '$3.14 \\times 2$'",
        "'1.5 0.5 +', '$1.5 + 0.5$'",

        // Advanced expressions
        "'1 2 + 3 4 + *', '$( 1 + 2 ) \\times ( 3 + 4 )$'",
        "'10 2 / 3 + 4 *', '$( 10 \\div 2 + 3 ) \\times 4$'"
    })
    void testIOContract(String input, String expectedOutput) {
        ByteArrayOutputStream outContent = new ByteArrayOutputStream();
        ByteArrayOutputStream errContent = new ByteArrayOutputStream();

        System.setIn(new ByteArrayInputStream(input.getBytes()));
        System.setOut(new PrintStream(outContent));
        System.setErr(new PrintStream(errContent));

        try {
            String[] args = {"-"};
            int exitCode = Main.run(args);

            assertEquals(0, exitCode, "Exit code should be 0 for valid input: " + input);
            assertEquals(expectedOutput + "\n", outContent.toString(),
                "Output mismatch for input: " + input);
            assertEquals("", errContent.toString(),
                "No errors should be written to stderr for valid input: " + input);
        } finally {
            System.setIn(System.in);
            System.setOut(System.out);
            System.setErr(System.err);
        }
    }

    /**
     * Tests error cases from the I/O contract.
     *
     * <p>Verifies that invalid inputs produce appropriate error messages.
     */
    @Test
    void testErrorCases() {
        String[] errorInputs = {
            "2 3 ^",      // Unsupported operator
            "5 +",        // Insufficient operands
            "5 3",        // Missing operators
            ""            // Empty expression
        };

        for (String input : errorInputs) {
            ByteArrayOutputStream outContent = new ByteArrayOutputStream();
            ByteArrayOutputStream errContent = new ByteArrayOutputStream();

            System.setIn(new ByteArrayInputStream(input.getBytes()));
            System.setOut(new PrintStream(outContent));
            System.setErr(new PrintStream(errContent));

            try {
                String[] args = {"-"};
                int exitCode = Main.run(args);

                assertEquals(1, exitCode, "Exit code should be 1 for invalid input: " + input);
                assertEquals("", outContent.toString(),
                    "No output should be written to stdout for invalid input: " + input);
                assertTrue(errContent.toString().length() > 0,
                    "Error message should be written to stderr for invalid input: " + input);
                assertTrue(errContent.toString().contains("Error:"),
                    "Error output should contain 'Error:' for input: " + input);
            } finally {
                System.setIn(System.in);
                System.setOut(System.out);
                System.setErr(System.err);
            }
        }
    }

    /**
     * Tests the complete workflow: read from file, process, write to file.
     */
    @Test
    void testCompleteFileWorkflow(@TempDir Path tempDir) throws IOException {
        // Create test input file
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "5 3 + 2 *");

        // Create output file path
        Path outputFile = tempDir.resolve("output.tex");

        // Run the CLI
        ByteArrayOutputStream errContent = new ByteArrayOutputStream();
        System.setErr(new PrintStream(errContent));

        try {
            String[] args = {inputFile.toString(), "-o", outputFile.toString()};
            int exitCode = Main.run(args);

            assertEquals(0, exitCode);
            assertTrue(Files.exists(outputFile));
            assertEquals("$( 5 + 3 ) \\times 2$\n", Files.readString(outputFile));
            assertTrue(errContent.toString().contains("Generated:"));
        } finally {
            System.setErr(System.err);
        }
    }

    /**
     * Tests that the CLI properly handles all operator types.
     */
    @Test
    void testAllOperators() {
        String[][] testCases = {
            {"5 3 +", "$5 + 3$"},
            {"5 3 -", "$5 - 3$"},
            {"5 3 *", "$5 \\times 3$"},
            {"5 3 /", "$5 \\div 3$"}
        };

        for (String[] testCase : testCases) {
            String input = testCase[0];
            String expected = testCase[1];

            ByteArrayOutputStream outContent = new ByteArrayOutputStream();
            System.setIn(new ByteArrayInputStream(input.getBytes()));
            System.setOut(new PrintStream(outContent));

            try {
                String[] args = {"-"};
                int exitCode = Main.run(args);

                assertEquals(0, exitCode);
                assertEquals(expected + "\n", outContent.toString());
            } finally {
                System.setIn(System.in);
                System.setOut(System.out);
            }
        }
    }

    /**
     * Tests that the CLI handles multi-line input correctly.
     *
     * <p>Newlines are treated as whitespace, so this should parse correctly.
     */
    @Test
    void testMultiLineInput(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("multiline.rpn");
        Files.writeString(inputFile, "5 3 +\n2 *");  // Two lines (valid RPN)

        ByteArrayOutputStream outContent = new ByteArrayOutputStream();
        System.setOut(new PrintStream(outContent));

        try {
            String[] args = {inputFile.toString()};
            int exitCode = Main.run(args);

            // Newlines are treated as whitespace, so this is valid: "5 3 + 2 *"
            assertEquals(0, exitCode);
            assertEquals("$( 5 + 3 ) \\times 2$\n", outContent.toString());
        } finally {
            System.setOut(System.out);
        }
    }

    /**
     * Tests that negative numbers are handled correctly.
     */
    @Test
    void testNegativeNumbers() {
        ByteArrayOutputStream outContent = new ByteArrayOutputStream();
        System.setIn(new ByteArrayInputStream("-5 3 +".getBytes()));
        System.setOut(new PrintStream(outContent));

        try {
            String[] args = {"-"};
            int exitCode = Main.run(args);

            assertEquals(0, exitCode);
            assertEquals("$-5 + 3$\n", outContent.toString());
        } finally {
            System.setIn(System.in);
            System.setOut(System.out);
        }
    }

    /**
     * Tests help flag output.
     */
    @Test
    void testHelpOutput() {
        ByteArrayOutputStream outContent = new ByteArrayOutputStream();
        System.setOut(new PrintStream(outContent));

        try {
            String[] args = {"--help"};
            Main.run(args);

            String output = outContent.toString();
            assertTrue(output.contains("Convert RPN expressions to LaTeX math mode"));
            assertTrue(output.contains("positional arguments:"));
            assertTrue(output.contains("optional arguments:"));
        } finally {
            System.setOut(System.out);
        }
    }
}
