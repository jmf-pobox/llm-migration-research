package com.rpn2tex;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;
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
 * Unit tests for the Main CLI class.
 *
 * <p>Tests all aspects of the command-line interface including:
 * <ul>
 *   <li>Argument parsing</li>
 *   <li>File I/O</li>
 *   <li>Stdin/stdout handling</li>
 *   <li>Error handling and formatting</li>
 *   <li>Integration with the processing pipeline</li>
 *   <li>I/O contract validation</li>
 * </ul>
 */
class MainTest {
    private final InputStream originalIn = System.in;
    private final PrintStream originalOut = System.out;
    private final PrintStream originalErr = System.err;

    private ByteArrayOutputStream capturedOut;
    private ByteArrayOutputStream capturedErr;

    @BeforeEach
    void setUp() {
        capturedOut = new ByteArrayOutputStream();
        capturedErr = new ByteArrayOutputStream();
        System.setOut(new PrintStream(capturedOut));
        System.setErr(new PrintStream(capturedErr));
    }

    @AfterEach
    void tearDown() {
        System.setIn(originalIn);
        System.setOut(originalOut);
        System.setErr(originalErr);
    }

    /**
     * Test successful processing with file input and stdout output.
     */
    @Test
    void testFileToStdout(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "5 3 +");

        int exitCode = Main.run(new String[]{inputFile.toString()});

        assertEquals(0, exitCode, "Should exit successfully");
        assertEquals("$5 + 3$\n", capturedOut.toString(), "Should output correct LaTeX");
    }

    /**
     * Test successful processing with file input and file output.
     */
    @Test
    void testFileToFile(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Path outputFile = tempDir.resolve("output.tex");
        Files.writeString(inputFile, "4 7 *");

        int exitCode = Main.run(new String[]{inputFile.toString(), "-o", outputFile.toString()});

        assertEquals(0, exitCode, "Should exit successfully");
        assertEquals("$4 \\times 7$\n", Files.readString(outputFile), "Output file should contain correct LaTeX");
        assertTrue(capturedErr.toString().contains("Generated: " + outputFile), "Should print status to stderr");
    }

    /**
     * Test successful processing with stdin input.
     */
    @Test
    void testStdinToStdout() {
        System.setIn(new ByteArrayInputStream("10 2 /".getBytes()));

        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode, "Should exit successfully");
        assertEquals("$10 \\div 2$\n", capturedOut.toString(), "Should output correct LaTeX");
    }

    /**
     * Test --output flag (long form).
     */
    @Test
    void testLongFormOutputFlag(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Path outputFile = tempDir.resolve("output.tex");
        Files.writeString(inputFile, "5 3 -");

        int exitCode = Main.run(new String[]{inputFile.toString(), "--output", outputFile.toString()});

        assertEquals(0, exitCode, "Should exit successfully");
        assertEquals("$5 - 3$\n", Files.readString(outputFile), "Output file should contain correct LaTeX");
    }

    /**
     * Test missing input file argument.
     */
    @Test
    void testMissingInputArgument() {
        int exitCode = Main.run(new String[]{});

        assertEquals(1, exitCode, "Should exit with error code");
        assertTrue(capturedErr.toString().contains("Error: Input file required"), "Should show error message");
    }

    /**
     * Test missing argument for -o flag.
     */
    @Test
    void testMissingOutputArgumentForFlag(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "5 3 +");

        int exitCode = Main.run(new String[]{inputFile.toString(), "-o"});

        assertEquals(1, exitCode, "Should exit with error code");
        assertTrue(capturedErr.toString().contains("Error: -o/--output requires a file path"),
                "Should show error message");
    }

    /**
     * Test file not found error.
     */
    @Test
    void testFileNotFound() {
        int exitCode = Main.run(new String[]{"/nonexistent/file.rpn"});

        assertEquals(1, exitCode, "Should exit with error code");
        assertTrue(capturedErr.toString().contains("Error: Input file not found"),
                "Should show file not found error");
    }

    /**
     * Test directory instead of file error.
     */
    @Test
    void testDirectoryAsInput(@TempDir Path tempDir) {
        int exitCode = Main.run(new String[]{tempDir.toString()});

        assertEquals(1, exitCode, "Should exit with error code");
        String error = capturedErr.toString();
        assertTrue(error.contains("Error: Expected a file, got a directory") ||
                        error.contains("Error: Failed to read input"),
                "Should show directory error");
    }

    /**
     * Test lexer error handling.
     */
    @Test
    void testLexerError(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "5 3 ^");

        int exitCode = Main.run(new String[]{inputFile.toString()});

        assertEquals(1, exitCode, "Should exit with error code");
        String error = capturedErr.toString();
        assertTrue(error.contains("Error: Unexpected character '^'"), "Should show lexer error");
        assertTrue(error.contains("1 | 5 3 ^"), "Should show source line");
        assertTrue(error.contains("^"), "Should show caret pointer");
    }

    /**
     * Test parser error handling.
     */
    @Test
    void testParserError(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "5 3");

        int exitCode = Main.run(new String[]{inputFile.toString()});

        assertEquals(1, exitCode, "Should exit with error code");
        String error = capturedErr.toString();
        assertTrue(error.contains("Error:"), "Should show parser error");
    }

    /**
     * Test output directory error.
     */
    @Test
    void testOutputDirectoryError(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "5 3 +");

        int exitCode = Main.run(new String[]{inputFile.toString(), "-o", tempDir.toString()});

        assertEquals(1, exitCode, "Should exit with error code");
        String error = capturedErr.toString();
        assertTrue(error.contains("Error:"), "Should show error message");
    }

    /**
     * Test I/O contract cases - valid expressions.
     */
    @ParameterizedTest
    @CsvSource(delimiter = '|', textBlock = """
            5 3 +               | $5 + 3$
            5 3 -               | $5 - 3$
            4 7 *               | $4 \\times 7$
            10 2 /              | $10 \\div 2$
            5 3 + 2 *           | $( 5 + 3 ) \\times 2$
            5 3 * 2 +           | $5 \\times 3 + 2$
            10 2 / 5 *          | $10 \\div 2 \\times 5$
            5 3 - 2 -           | $5 - 3 - 2$
            100 10 / 5 / 2 /    | $100 \\div 10 \\div 5 \\div 2$
            1 2 + 3 + 4 +       | $1 + 2 + 3 + 4$
            2 3 4 * +           | $2 + 3 \\times 4$
            2 3 + 4 *           | $( 2 + 3 ) \\times 4$
            2 3 4 + *           | $2 \\times ( 3 + 4 )$
            2 3 * 4 +           | $2 \\times 3 + 4$
            3.14 2 *            | $3.14 \\times 2$
            1.5 0.5 +           | $1.5 + 0.5$
            1 2 + 3 4 + *       | $( 1 + 2 ) \\times ( 3 + 4 )$
            10 2 / 3 + 4 *      | $( 10 \\div 2 + 3 ) \\times 4$
            """)
    void testIOContractValidExpressions(String input, String expected) {
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        int exitCode = Main.run(new String[]{"-"});

        assertEquals(0, exitCode, "Should exit successfully for input: " + input);
        assertEquals(expected + "\n", capturedOut.toString(),
                "Should produce correct LaTeX for input: " + input);
    }

    /**
     * Test I/O contract cases - error expressions.
     */
    @ParameterizedTest
    @CsvSource(delimiter = '|', textBlock = """
            2 3 ^       | Unexpected character '^'
            2 3 ^ 4 *   | Unexpected character '^'
            2 3 4 ^ ^   | Unexpected character '^'
            """)
    void testIOContractErrorExpressions(String input, String expectedError) {
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        int exitCode = Main.run(new String[]{"-"});

        assertEquals(1, exitCode, "Should exit with error for input: " + input);
        String error = capturedErr.toString();
        assertTrue(error.contains(expectedError),
                "Should contain error message '" + expectedError + "' for input: " + input);
    }

    /**
     * Test complex multi-line input.
     */
    @Test
    void testMultiLineInput(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "5 3 +\n2 *");

        int exitCode = Main.run(new String[]{inputFile.toString()});

        // The lexer will process multi-line input as a single expression
        // In this case: 5 3 + 2 * = (5 + 3) * 2
        assertEquals(0, exitCode, "Should exit successfully");
    }

    /**
     * Test empty input.
     */
    @Test
    void testEmptyInput(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "");

        int exitCode = Main.run(new String[]{inputFile.toString()});

        assertEquals(1, exitCode, "Should exit with error for empty input");
        assertTrue(capturedErr.toString().contains("Error:"), "Should show error message");
    }

    /**
     * Test whitespace-only input.
     */
    @Test
    void testWhitespaceOnlyInput(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "   \n\t  \n  ");

        int exitCode = Main.run(new String[]{inputFile.toString()});

        assertEquals(1, exitCode, "Should exit with error for whitespace-only input");
        assertTrue(capturedErr.toString().contains("Error:"), "Should show error message");
    }

    /**
     * Test negative numbers.
     */
    @Test
    void testNegativeNumbers(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "-5 3 +");

        int exitCode = Main.run(new String[]{inputFile.toString()});

        assertEquals(0, exitCode, "Should exit successfully");
        assertEquals("$-5 + 3$\n", capturedOut.toString(), "Should handle negative numbers");
    }

    /**
     * Test large numbers.
     */
    @Test
    void testLargeNumbers(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "999999 1 +");

        int exitCode = Main.run(new String[]{inputFile.toString()});

        assertEquals(0, exitCode, "Should exit successfully");
        assertEquals("$999999 + 1$\n", capturedOut.toString(), "Should handle large numbers");
    }

    /**
     * Test decimal numbers with multiple digits.
     */
    @Test
    void testComplexDecimalNumbers(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "123.456 78.9 *");

        int exitCode = Main.run(new String[]{inputFile.toString()});

        assertEquals(0, exitCode, "Should exit successfully");
        assertEquals("$123.456 \\times 78.9$\n", capturedOut.toString(),
                "Should handle complex decimal numbers");
    }
}
