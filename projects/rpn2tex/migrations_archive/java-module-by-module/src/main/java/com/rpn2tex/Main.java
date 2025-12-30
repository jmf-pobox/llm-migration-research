package com.rpn2tex;

import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileWriter;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;

/**
 * Command-line interface for the RPN to LaTeX converter.
 *
 * <p>This CLI orchestrates the complete pipeline:
 * <ol>
 *   <li>Read input from file or stdin</li>
 *   <li>Tokenize with Lexer</li>
 *   <li>Parse with Parser</li>
 *   <li>Generate LaTeX with LaTeXGenerator</li>
 *   <li>Write output to file or stdout</li>
 * </ol>
 *
 * <p>Usage:
 * <pre>
 * java -jar rpn2tex.jar &lt;input&gt; [-o &lt;output&gt;]
 * java -jar rpn2tex.jar - [-o &lt;output&gt;]  # Read from stdin
 * </pre>
 *
 * <p>Arguments:
 * <ul>
 *   <li>input: Path to RPN input file, or "-" to read from stdin</li>
 *   <li>-o, --output: Optional output file path (default: stdout)</li>
 * </ul>
 *
 * <p>Exit codes:
 * <ul>
 *   <li>0: Success (including parse errors)</li>
 *   <li>1: I/O error (file not found, cannot read, cannot write)</li>
 * </ul>
 *
 * <p>Example:
 * <pre>{@code
 * $ echo "5 3 +" | java -jar rpn2tex.jar -
 * $5 + 3$
 *
 * $ java -jar rpn2tex.jar input.rpn -o output.tex
 * Generated: output.tex
 * }</pre>
 */
public final class Main {

    /**
     * Entry point for the CLI application.
     *
     * @param args command-line arguments
     */
    public static void main(String[] args) {
        System.exit(run(args));
    }

    /**
     * Main application logic.
     *
     * @param args command-line arguments
     * @return exit code (0 for success/parse errors, 1 for I/O errors)
     */
    static int run(String[] args) {
        // Parse command-line arguments
        if (args.length == 0) {
            System.err.println("Usage: rpn2tex <input> [-o <output>]");
            System.err.println("  input:  Input RPN file (use '-' for stdin)");
            System.err.println("  -o:     Output LaTeX file (default: stdout)");
            return 1;
        }

        String inputPath = args[0];
        String outputPath = null;

        // Parse optional output flag
        for (int i = 1; i < args.length; i++) {
            if (("-o".equals(args[i]) || "--output".equals(args[i])) && i + 1 < args.length) {
                outputPath = args[i + 1];
                i++; // Skip next argument
            }
        }

        // Read input
        String text;
        try {
            text = readInput(inputPath);
        } catch (FileNotFoundException e) {
            System.err.println("Error: File not found: " + inputPath);
            return 1;
        } catch (IOException e) {
            System.err.println("Error: Failed to read input: " + e.getMessage());
            return 1;
        }

        // Process the pipeline
        ErrorFormatter formatter = new ErrorFormatter(text);
        String latex;

        try {
            // Lexer: text -> tokens
            Lexer lexer = new Lexer(text);
            List<Token> tokens = lexer.tokenize();

            // Parser: tokens -> AST
            Parser parser = new Parser(tokens);
            Expr ast = parser.parse();

            // Generator: AST -> LaTeX
            LaTeXGenerator generator = new LaTeXGenerator();
            latex = generator.generate(ast);

        } catch (RpnException e) {
            // Format and display parse/lex errors
            String formattedError = formatter.formatError(e.getMessage(), e.getLine(), e.getColumn());
            System.err.println(formattedError);
            return 0; // Parse errors return 0, not 1
        }

        // Write output
        try {
            writeOutput(latex, outputPath);
            if (outputPath != null) {
                System.err.println("Generated: " + outputPath);
            }
        } catch (IOException e) {
            System.err.println("Error: Failed to write output: " + e.getMessage());
            return 1;
        }

        return 0;
    }

    /**
     * Reads input from a file or stdin.
     *
     * @param inputPath path to input file, or "-" for stdin
     * @return the input text
     * @throws IOException if reading fails
     */
    private static String readInput(String inputPath) throws IOException {
        if ("-".equals(inputPath)) {
            // Read from stdin
            StringBuilder sb = new StringBuilder();
            try (BufferedReader reader = new BufferedReader(
                    new InputStreamReader(System.in, StandardCharsets.UTF_8))) {
                String line;
                while ((line = reader.readLine()) != null) {
                    if (sb.length() > 0) {
                        sb.append('\n');
                    }
                    sb.append(line);
                }
            }
            return sb.toString();
        } else {
            // Read from file
            Path path = Paths.get(inputPath);
            return Files.readString(path, StandardCharsets.UTF_8);
        }
    }

    /**
     * Writes output to a file or stdout.
     *
     * @param content the content to write
     * @param outputPath path to output file, or null for stdout
     * @throws IOException if writing fails
     */
    private static void writeOutput(String content, String outputPath) throws IOException {
        if (outputPath == null) {
            // Write to stdout WITHOUT trailing newline
            System.out.print(content);
        } else {
            // Write to file WITH trailing newline
            try (FileWriter writer = new FileWriter(outputPath, StandardCharsets.UTF_8)) {
                writer.write(content);
                writer.write('\n');
            }
        }
    }
}
