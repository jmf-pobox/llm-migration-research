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

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit and integration tests for the {@link Main} class.
 *
 * <p>Tests cover:
 * <ul>
 *   <li>Command-line argument parsing</li>
 *   <li>Reading from stdin and files</li>
 *   <li>Writing to stdout and files</li>
 *   <li>Error handling and exit codes</li>
 *   <li>Complete I/O contract validation (end-to-end)</li>
 * </ul>
 */
class MainTest {

    private final InputStream originalIn = System.in;
    private final PrintStream originalOut = System.out;
    private final PrintStream originalErr = System.err;

    private ByteArrayOutputStream outContent;
    private ByteArrayOutputStream errContent;

    @BeforeEach
    void setUpStreams() {
        outContent = new ByteArrayOutputStream();
        errContent = new ByteArrayOutputStream();
        System.setOut(new PrintStream(outContent));
        System.setErr(new PrintStream(errContent));
    }

    @AfterEach
    void restoreStreams() {
        System.setIn(originalIn);
        System.setOut(originalOut);
        System.setErr(originalErr);
    }

    @Test
    void testNoArguments() {
        String[] args = {};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        assertTrue(errContent.toString().contains("usage: rpn2tex"));
    }

    @Test
    void testHelpShortFlag() {
        String[] args = {"-h"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        assertTrue(outContent.toString().contains("Convert RPN expressions to LaTeX math mode"));
    }

    @Test
    void testHelpLongFlag() {
        String[] args = {"--help"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        assertTrue(outContent.toString().contains("Convert RPN expressions to LaTeX math mode"));
    }

    @Test
    void testMissingOutputArgument() {
        String[] args = {"-o"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        assertTrue(errContent.toString().contains("requires an argument"));
    }

    @Test
    void testUnknownOption() {
        String[] args = {"--unknown", "test.rpn"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        assertTrue(errContent.toString().contains("Unknown option"));
    }

    @Test
    void testMultipleInputFiles() {
        String[] args = {"input1.rpn", "input2.rpn"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        assertTrue(errContent.toString().contains("Multiple input files"));
    }

    @Test
    void testStdinInput() {
        String input = "5 3 +";
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(0, exitCode);
        assertEquals("$5 + 3$\n", outContent.toString());
    }

    @Test
    void testFileNotFound() {
        String[] args = {"nonexistent.rpn"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        assertTrue(errContent.toString().contains("file not found"));
    }

    @Test
    void testFileInput(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "5 3 +");

        String[] args = {inputFile.toString()};
        int exitCode = Main.run(args);

        assertEquals(0, exitCode);
        assertEquals("$5 + 3$\n", outContent.toString());
    }

    @Test
    void testFileOutput(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Path outputFile = tempDir.resolve("output.tex");
        Files.writeString(inputFile, "5 3 +");

        String[] args = {inputFile.toString(), "-o", outputFile.toString()};
        int exitCode = Main.run(args);

        assertEquals(0, exitCode);
        assertTrue(errContent.toString().contains("Generated:"));
        assertEquals("$5 + 3$\n", Files.readString(outputFile));
    }

    @Test
    void testLexerError() {
        String input = "2 3 ^";
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        String error = errContent.toString();
        assertTrue(error.contains("Error: Unexpected character '^'"));
        assertTrue(error.contains("1 | 2 3 ^"));
        assertTrue(error.contains("^"));
    }

    @Test
    void testParserError() {
        String input = "5 3";
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        String error = errContent.toString();
        assertTrue(error.contains("Error: Invalid RPN"));
        assertTrue(error.contains("missing operators"));
    }

    // I/O Contract Tests - Basic Operations

    @ParameterizedTest
    @CsvSource({
        "'5 3 +', '$5 + 3$'",
        "'5 3 -', '$5 - 3$'",
        "'4 7 *', '$4 \\times 7$'",
        "'10 2 /', '$10 \\div 2$'"
    })
    void testBasicOperations(String input, String expected) {
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(0, exitCode);
        assertEquals(expected + "\n", outContent.toString());
    }

    // I/O Contract Tests - Complex Operations

    @ParameterizedTest
    @CsvSource({
        "'5 3 + 2 *', '$( 5 + 3 ) \\times 2$'",
        "'5 3 * 2 +', '$5 \\times 3 + 2$'",
        "'10 2 / 5 *', '$10 \\div 2 \\times 5$'",
        "'5 3 - 2 -', '$5 - 3 - 2$'",
        "'100 10 / 5 / 2 /', '$100 \\div 10 \\div 5 \\div 2$'",
        "'1 2 + 3 + 4 +', '$1 + 2 + 3 + 4$'",
        "'2 3 4 * +', '$2 + 3 \\times 4$'",
        "'2 3 + 4 *', '$( 2 + 3 ) \\times 4$'",
        "'2 3 4 + *', '$2 \\times ( 3 + 4 )$'",
        "'2 3 * 4 +', '$2 \\times 3 + 4$'"
    })
    void testComplexOperations(String input, String expected) {
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(0, exitCode);
        assertEquals(expected + "\n", outContent.toString());
    }

    // I/O Contract Tests - Floating Point

    @ParameterizedTest
    @CsvSource({
        "'3.14 2 *', '$3.14 \\times 2$'",
        "'1.5 0.5 +', '$1.5 + 0.5$'"
    })
    void testFloatingPoint(String input, String expected) {
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(0, exitCode);
        assertEquals(expected + "\n", outContent.toString());
    }

    // I/O Contract Tests - Advanced Expressions

    @ParameterizedTest
    @CsvSource({
        "'1 2 + 3 4 + *', '$( 1 + 2 ) \\times ( 3 + 4 )$'",
        "'10 2 / 3 + 4 *', '$( 10 \\div 2 + 3 ) \\times 4$'"
    })
    void testAdvancedExpressions(String input, String expected) {
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(0, exitCode);
        assertEquals(expected + "\n", outContent.toString());
    }

    // I/O Contract Tests - Error Cases

    @Test
    void testExponentiationNotSupported() {
        String input = "2 3 ^";
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        String error = errContent.toString();
        assertTrue(error.contains("Error: Unexpected character '^'"));
        assertTrue(error.contains("2 3 ^"));
    }

    @Test
    void testExponentiationInComplexExpression() {
        String input = "2 3 ^ 4 *";
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        String error = errContent.toString();
        assertTrue(error.contains("Error: Unexpected character '^'"));
        assertTrue(error.contains("2 3 ^ 4 *"));
    }

    @Test
    void testMultipleExponentiations() {
        String input = "2 3 4 ^ ^";
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        String error = errContent.toString();
        assertTrue(error.contains("Error: Unexpected character '^'"));
        assertTrue(error.contains("2 3 4 ^ ^"));
    }

    @Test
    void testEmptyExpression() {
        String input = "";
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        String error = errContent.toString();
        assertTrue(error.contains("Empty expression"));
    }

    @Test
    void testInsufficientOperands() {
        String input = "5 +";
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        String error = errContent.toString();
        assertTrue(error.contains("requires two operands"));
    }

    @Test
    void testMissingOperators() {
        String input = "5 3 7";
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-"};
        int exitCode = Main.run(args);

        assertEquals(1, exitCode);
        String error = errContent.toString();
        assertTrue(error.contains("Invalid RPN"));
        assertTrue(error.contains("missing operators"));
    }

    // Integration test: File I/O with output flag

    @Test
    void testFileToFileConversion(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("test.rpn");
        Path outputFile = tempDir.resolve("test.tex");

        Files.writeString(inputFile, "5 3 + 2 *");

        String[] args = {inputFile.toString(), "-o", outputFile.toString()};
        int exitCode = Main.run(args);

        assertEquals(0, exitCode);
        assertEquals("$( 5 + 3 ) \\times 2$\n", Files.readString(outputFile));
        assertTrue(errContent.toString().contains("Generated: " + outputFile));
    }

    @Test
    void testStdinWithOutputFile(@TempDir Path tempDir) throws IOException {
        Path outputFile = tempDir.resolve("output.tex");
        String input = "10 2 / 3 + 4 *";
        System.setIn(new ByteArrayInputStream(input.getBytes()));

        String[] args = {"-", "-o", outputFile.toString()};
        int exitCode = Main.run(args);

        assertEquals(0, exitCode);
        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$\n", Files.readString(outputFile));
    }

    @Test
    void testLongFormOutputFlag(@TempDir Path tempDir) throws IOException {
        Path inputFile = tempDir.resolve("input.rpn");
        Path outputFile = tempDir.resolve("output.tex");
        Files.writeString(inputFile, "3.14 2 *");

        String[] args = {inputFile.toString(), "--output", outputFile.toString()};
        int exitCode = Main.run(args);

        assertEquals(0, exitCode);
        assertEquals("$3.14 \\times 2$\n", Files.readString(outputFile));
    }
}
