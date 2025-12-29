package com.rpn2tex;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

/**
 * Command-line interface for rpn2tex.
 *
 * <p>Orchestrates the pipeline: read → tokenize → parse → generate → write.
 *
 * <p>Usage examples:
 * <pre>
 *   java com.rpn2tex.Main input.rpn              # Output to stdout
 *   echo "5" | java com.rpn2tex.Main -           # Read from stdin
 * </pre>
 */
public final class Main {

    private Main() {
        // Utility class - no instantiation
    }

    /**
     * Main entry point for rpn2tex CLI.
     *
     * @param args command line arguments
     */
    public static void main(String[] args) {
        int exitCode = run(args);
        System.exit(exitCode);
    }

    /**
     * Runs the application logic.
     *
     * @param args command line arguments
     * @return exit code: 0 for success, 1 for error
     */
    static int run(String[] args) {
        // Read input
        String text;
        try {
            if (args.length == 0 || "-".equals(args[0])) {
                // Read from stdin
                text = readStdin();
            } else {
                // Read from file
                Path inputPath = Path.of(args[0]);
                text = Files.readString(inputPath, StandardCharsets.UTF_8);
            }
        } catch (IOException e) {
            System.err.println("Error reading input: " + e.getMessage());
            return 1;
        }

        // Process: tokenize → parse → generate
        try {
            // Tokenize
            Lexer lexer = new Lexer(text);
            List<Token> tokens = lexer.tokenize();

            // Parse
            Parser parser = new Parser(tokens);
            ASTNode ast = parser.parse();

            // Generate LaTeX
            LaTeXGenerator generator = new LaTeXGenerator();
            String latex = generator.generate(ast);

            // Write output to stdout
            System.out.println(latex);

        } catch (LexerException | ParserException e) {
            System.err.println("Error: " + e.getMessage());
            return 1;
        } catch (Exception e) {
            System.err.println("Unexpected error: " + e.getMessage());
            e.printStackTrace(System.err);
            return 1;
        }

        return 0;
    }

    private static String readStdin() throws IOException {
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
    }
}
