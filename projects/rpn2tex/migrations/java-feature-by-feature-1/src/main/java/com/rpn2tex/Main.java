package com.rpn2tex;

import java.io.IOException;
import java.nio.file.AccessDeniedException;
import java.nio.file.Files;
import java.nio.file.NoSuchFileException;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.util.Scanner;

/**
 * Command-line interface for rpn2tex.
 *
 * <p>Converts RPN (Reverse Polish Notation) expressions to LaTeX math mode format.
 * Supports reading from files or stdin, and writing to files or stdout.
 *
 * <p>This CLI orchestrates the complete pipeline:
 * <ol>
 *   <li>Read input (file or stdin)</li>
 *   <li>Tokenize with {@link Lexer}</li>
 *   <li>Parse with {@link Parser}</li>
 *   <li>Generate LaTeX with {@link LaTeXGenerator}</li>
 *   <li>Write output (file or stdout)</li>
 * </ol>
 *
 * <p>Usage examples:
 * <pre>
 * # Output to stdout
 * java com.rpn2tex.Main input.rpn
 *
 * # Output to file
 * java com.rpn2tex.Main input.rpn -o output.tex
 *
 * # Read from stdin
 * echo "5" | java com.rpn2tex.Main -
 * </pre>
 *
 * <p>Exit codes:
 * <ul>
 *   <li>0: Success</li>
 *   <li>1: Error (file not found, parse error, etc.)</li>
 * </ul>
 *
 * @see Lexer
 * @see Parser
 * @see LaTeXGenerator
 */
public final class Main {
    /**
     * Main entry point for the CLI application.
     *
     * <p>Parses command-line arguments and invokes {@link #run(String[])} to
     * execute the application logic. Exits with the return code from run().
     *
     * @param args command-line arguments
     */
    public static void main(String[] args) {
        System.exit(run(args));
    }

    /**
     * Executes the main application logic.
     *
     * <p>Parses arguments, reads input, processes the RPN expression through
     * the lexer, parser, and LaTeX generator, and writes output. Handles all
     * errors and returns appropriate exit codes.
     *
     * @param args command-line arguments where:
     *             <ul>
     *               <li>First positional arg: input file path (or "-" for stdin)</li>
     *               <li>-o/--output: optional output file path</li>
     *             </ul>
     * @return exit code: 0 for success, 1 for error
     */
    public static int run(String[] args) {
        // Parse command-line arguments
        String inputPath = null;
        String outputPath = null;

        for (int i = 0; i < args.length; i++) {
            if (args[i].equals("-o") || args[i].equals("--output")) {
                if (i + 1 < args.length) {
                    outputPath = args[++i];
                } else {
                    System.err.println("Error: -o/--output requires a file path");
                    return 1;
                }
            } else if (!args[i].startsWith("-") || args[i].equals("-")) {
                inputPath = args[i];
            }
        }

        if (inputPath == null) {
            System.err.println("Error: Input file required");
            System.err.println("Usage: java com.rpn2tex.Main <input> [-o <output>]");
            return 1;
        }

        // Read input
        String text;
        try {
            if (inputPath.equals("-")) {
                text = readStdin();
            } else {
                text = readFile(inputPath);
            }
        } catch (NoSuchFileException e) {
            System.err.println("Error: Input file not found: " + inputPath);
            return 1;
        } catch (AccessDeniedException e) {
            System.err.println("Error: Permission denied reading: " + inputPath);
            return 1;
        } catch (IOException e) {
            if (Files.isDirectory(Paths.get(inputPath))) {
                System.err.println("Error: Expected a file, got a directory: " + inputPath);
            } else {
                System.err.println("Error: Failed to read input: " + e.getMessage());
            }
            return 1;
        }

        // Process: tokenize → parse → generate
        String latex;

        try {
            // Tokenize
            Lexer lexer = new Lexer(text);
            List<Token> tokens = lexer.tokenize();

            // Parse
            Parser parser = new Parser(tokens);
            Expr ast = parser.parse();

            // Generate LaTeX
            LaTeXGenerator generator = new LaTeXGenerator();
            latex = generator.generate(ast);

        } catch (RpnException e) {
            // Format error with source context
            String formatted = e.format(text);
            System.err.println(formatted);
            return 1;
        }

        // Write output
        if (outputPath != null) {
            try {
                writeFile(outputPath, latex + "\n");
                System.err.println("Generated: " + outputPath);
            } catch (AccessDeniedException e) {
                System.err.println("Error: Permission denied writing: " + outputPath);
                return 1;
            } catch (IOException e) {
                if (Files.isDirectory(Paths.get(outputPath))) {
                    System.err.println("Error: Cannot write to directory: " + outputPath);
                } else {
                    System.err.println("Error: Failed to write output: " + e.getMessage());
                }
                return 1;
            }
        } else {
            System.out.println(latex);
        }

        return 0;
    }

    /**
     * Reads all text from stdin.
     *
     * <p>Reads until EOF, collecting all lines into a single string.
     * Line separators are preserved as newlines (\n).
     *
     * @return the complete stdin content
     * @throws IOException if reading fails
     */
    private static String readStdin() throws IOException {
        StringBuilder sb = new StringBuilder();
        try (Scanner scanner = new Scanner(System.in)) {
            while (scanner.hasNextLine()) {
                String line = scanner.nextLine();
                if (sb.length() > 0) {
                    sb.append("\n");
                }
                sb.append(line);
            }
        }
        return sb.toString();
    }

    /**
     * Reads all text from a file.
     *
     * <p>Uses UTF-8 encoding by default.
     *
     * @param path the file path to read
     * @return the complete file content
     * @throws NoSuchFileException if the file does not exist
     * @throws AccessDeniedException if permission is denied
     * @throws IOException if reading fails for any other reason
     */
    private static String readFile(String path) throws IOException {
        Path filePath = Paths.get(path);
        return Files.readString(filePath);
    }

    /**
     * Writes text to a file.
     *
     * <p>Creates the file if it doesn't exist, overwrites if it does.
     * Uses UTF-8 encoding by default.
     *
     * @param path the file path to write
     * @param content the text to write
     * @throws AccessDeniedException if permission is denied
     * @throws IOException if writing fails for any other reason
     */
    private static void writeFile(String path, String content) throws IOException {
        Path filePath = Paths.get(path);
        Files.writeString(filePath, content);
    }
}
