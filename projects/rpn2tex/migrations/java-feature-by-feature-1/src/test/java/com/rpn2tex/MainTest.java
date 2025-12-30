package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.PrintStream;
import java.nio.file.Files;
import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests for the Main CLI class.
 */
public class MainTest {
    @Test
    public void testMainWithValidInput(@TempDir Path tempDir) throws IOException {
        // Create input file
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "5");

        // Capture stdout
        ByteArrayOutputStream outContent = new ByteArrayOutputStream();
        PrintStream originalOut = System.out;
        System.setOut(new PrintStream(outContent));

        try {
            int exitCode = Main.run(new String[]{inputFile.toString()});
            assertEquals(0, exitCode);
            assertEquals("$5$\n", outContent.toString());
        } finally {
            System.setOut(originalOut);
        }
    }

    @Test
    public void testMainWithOutputFile(@TempDir Path tempDir) throws IOException {
        // Create input file
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "3.14");

        Path outputFile = tempDir.resolve("output.tex");

        int exitCode = Main.run(new String[]{
            inputFile.toString(),
            "-o",
            outputFile.toString()
        });

        assertEquals(0, exitCode);
        assertTrue(Files.exists(outputFile));
        assertEquals("$3.14$\n", Files.readString(outputFile));
    }

    @Test
    public void testMainWithInvalidInput(@TempDir Path tempDir) throws IOException {
        // Create input file with invalid content
        Path inputFile = tempDir.resolve("input.rpn");
        Files.writeString(inputFile, "5 # 3");

        // Capture stderr
        ByteArrayOutputStream errContent = new ByteArrayOutputStream();
        PrintStream originalErr = System.err;
        System.setErr(new PrintStream(errContent));

        try {
            int exitCode = Main.run(new String[]{inputFile.toString()});
            assertEquals(1, exitCode);
            String errorOutput = errContent.toString();
            assertTrue(errorOutput.contains("Error:"));
            assertTrue(errorOutput.contains("Unexpected character"));
        } finally {
            System.setErr(originalErr);
        }
    }

    @Test
    public void testMainWithNonexistentFile() {
        // Capture stderr
        ByteArrayOutputStream errContent = new ByteArrayOutputStream();
        PrintStream originalErr = System.err;
        System.setErr(new PrintStream(errContent));

        try {
            int exitCode = Main.run(new String[]{"nonexistent.rpn"});
            assertEquals(1, exitCode);
            assertTrue(errContent.toString().contains("not found"));
        } finally {
            System.setErr(originalErr);
        }
    }

    @Test
    public void testMainWithNoArguments() {
        // Capture stderr
        ByteArrayOutputStream errContent = new ByteArrayOutputStream();
        PrintStream originalErr = System.err;
        System.setErr(new PrintStream(errContent));

        try {
            int exitCode = Main.run(new String[]{});
            assertEquals(1, exitCode);
            assertTrue(errContent.toString().contains("Input file required"));
        } finally {
            System.setErr(originalErr);
        }
    }
}
