package com.rpn2tex;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

/**
 * Command-line interface for rpn2tex converter.
 *
 * <p>This class orchestrates the complete pipeline for converting RPN (Reverse Polish Notation)
 * expressions to LaTeX mathematical notation. The pipeline consists of:
 * <ol>
 *   <li>Lexical analysis (tokenization)</li>
 *   <li>Parsing (AST construction)</li>
 *   <li>Code generation (LaTeX output)</li>
 * </ol>
 *
 * <h2>Usage</h2>
 * <pre>
 * # Read from stdin, write to stdout
 * echo "5 3 +" | java -cp build/classes/java/main com.rpn2tex.Main
 *
 * # Read from file
 * java -cp build/classes/java/main com.rpn2tex.Main input.rpn
 *
 * # Write to file
 * java -cp build/classes/java/main com.rpn2tex.Main input.rpn -o output.tex
 * </pre>
 *
 * <h2>Exit Codes</h2>
 * <ul>
 *   <li>0: Success - expression converted successfully</li>
 *   <li>1: Error - lexer error, parser error, or I/O error</li>
 * </ul>
 *
 * <h2>Error Handling</h2>
 * <p>All errors are formatted with source context and printed to stderr:
 * <pre>
 * Error: Unexpected character '^'
 *
 * 1 | 2 3 ^ 4 *
 *         ^
 * </pre>
 *
 * @see Lexer
 * @see Parser
 * @see LaTeXGenerator
 * @see RpnException
 * @since 1.0.0
 */
public final class Main {
    /** Exit code for successful execution. */
    private static final int SUCCESS = 0;

    /** Exit code for any error (lexer, parser, or I/O). */
    private static final int ERROR = 1;

    /**
     * Private constructor to prevent instantiation.
     * This class only contains static methods.
     */
    private Main() {
        throw new AssertionError("Main class should not be instantiated");
    }

    /**
     * Main entry point for the rpn2tex converter.
     *
     * <p>Accepts command-line arguments:
     * <ul>
     *   <li>First argument: input file path, or "-" for stdin (required)</li>
     *   <li>-o/--output: output file path (optional, defaults to stdout)</li>
     * </ul>
     *
     * <p>The program reads RPN expressions, converts them to LaTeX, and outputs
     * the result. Errors are reported with source context to stderr.
     *
     * @param args command-line arguments
     */
    public static void main(String[] args) {
        System.exit(run(args));
    }

    /**
     * Core execution logic with testable return value.
     *
     * <p>This method is separated from main() to allow testing without System.exit().
     * It orchestrates the entire pipeline and handles all error cases.
     *
     * @param args command-line arguments
     * @return exit code (0 for success, 1 for error)
     */
    static int run(String[] args) {
        // Parse arguments
        String input = null;
        String output = null;

        // Simple argument parsing
        for (int i = 0; i < args.length; i++) {
            if (args[i].equals("-o") || args[i].equals("--output")) {
                if (i + 1 < args.length) {
                    output = args[i + 1];
                    i++; // Skip next argument
                } else {
                    System.err.println("Error: -o/--output requires a file path");
                    return ERROR;
                }
            } else if (input == null) {
                // First non-option argument is the input
                input = args[i];
            } else {
                // Extra positional argument
                System.err.println("Error: Unexpected argument '" + args[i] + "'");
                return ERROR;
            }
        }

        // Validate required input argument
        if (input == null) {
            System.err.println("Usage: java com.rpn2tex.Main <input> [-o <output>]");
            System.err.println("  <input>   Input RPN file, or '-' for stdin");
            System.err.println("  -o        Output LaTeX file (default: stdout)");
            return ERROR;
        }

        // Read input
        String text;
        try {
            text = readInput(input);
        } catch (IOException e) {
            System.err.println("Error: " + e.getMessage());
            return ERROR;
        }

        // Process pipeline: Lexer → Parser → LaTeXGenerator
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(text);
        try {
            // Tokenize
            Lexer lexer = new Lexer(text);
            List<Token> tokens = lexer.tokenize();

            // Parse
            Parser parser = new Parser(tokens);
            Expr ast = parser.parse();

            // Generate LaTeX
            LaTeXGenerator generator = new LaTeXGenerator();
            String latex = generator.generate(ast);

            // Write output
            writeOutput(latex, output);

            return SUCCESS;
        } catch (RpnException e) {
            // Format error with source context
            String formatted = formatter.formatError(e.getErrorMessage(), e.getLine(), e.getColumn());
            System.err.println(formatted);
            return ERROR;
        } catch (IOException e) {
            // Output write error
            System.err.println("Error: " + e.getMessage());
            return ERROR;
        }
    }

    /**
     * Reads input from file or stdin.
     *
     * <p>If the input path is "-", reads from stdin. Otherwise, reads from
     * the specified file path.
     *
     * @param input the input path, or "-" for stdin
     * @return the complete input text
     * @throws IOException if reading fails
     */
    private static String readInput(String input) throws IOException {
        if ("-".equals(input)) {
            // Read from stdin
            return new String(System.in.readAllBytes(), StandardCharsets.UTF_8);
        } else {
            // Read from file
            try {
                return Files.readString(Paths.get(input), StandardCharsets.UTF_8);
            } catch (IOException e) {
                // Enhance error message with context
                throw new IOException("Cannot read input file '" + input + "': " + e.getMessage());
            }
        }
    }

    /**
     * Writes output to file or stdout.
     *
     * <p>If the output path is null, writes to stdout with a newline.
     * Otherwise, writes to the specified file path with a newline.
     *
     * @param latex the LaTeX string to write
     * @param output the output path, or null for stdout
     * @throws IOException if writing to file fails
     */
    private static void writeOutput(String latex, String output) throws IOException {
        if (output == null) {
            // Write to stdout
            System.out.println(latex);
        } else {
            // Write to file
            try {
                Files.writeString(Paths.get(output), latex + "\n", StandardCharsets.UTF_8);
                System.err.println("Generated: " + output);
            } catch (IOException e) {
                // Enhance error message with context
                throw new IOException("Cannot write output file '" + output + "': " + e.getMessage());
            }
        }
    }
}
