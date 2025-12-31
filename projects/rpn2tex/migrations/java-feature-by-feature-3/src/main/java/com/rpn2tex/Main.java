package com.rpn2tex;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

/**
 * Main entry point for the rpn2tex application.
 * <p>
 * This application converts Reverse Polish Notation (RPN) expressions
 * to LaTeX mathematical notation.
 * </p>
 * <p>
 * Usage:
 * </p>
 * <pre>
 * # Read from stdin
 * echo "5" | java -jar rpn2tex.jar
 *
 * # Read from command-line argument
 * java -jar rpn2tex.jar "5 3.14"
 *
 * # Read from file
 * java -jar rpn2tex.jar input.txt
 * </pre>
 */
public class Main {
    /**
     * Main method - processes RPN input and outputs LaTeX.
     *
     * @param args Command-line arguments (optional input)
     */
    public static void main(String[] args) {
        try {
            String input = getInput(args);
            String output = process(input);
            System.out.println(output);
        } catch (RpnException e) {
            System.err.println("Error: " + e.getMessage());
            System.err.printf("  at line %d, column %d%n", e.getLine(), e.getColumn());
            System.exit(1);
        } catch (IOException e) {
            System.err.println("I/O Error: " + e.getMessage());
            System.exit(1);
        } catch (Exception e) {
            System.err.println("Unexpected error: " + e.getMessage());
            e.printStackTrace(System.err);
            System.exit(1);
        }
    }

    /**
     * Gets input from command-line args, stdin, or file.
     *
     * @param args Command-line arguments
     * @return The input text
     * @throws IOException if file reading fails
     */
    private static String getInput(String[] args) throws IOException {
        if (args.length == 0) {
            // Read from stdin
            return readStdin();
        } else if (args.length == 1) {
            String arg = args[0];
            Path path = Path.of(arg);

            // Check if it's a file
            if (Files.exists(path) && Files.isRegularFile(path)) {
                return Files.readString(path, StandardCharsets.UTF_8);
            } else {
                // Treat as direct input
                return arg;
            }
        } else {
            throw new IllegalArgumentException("Usage: rpn2tex [input|file]");
        }
    }

    /**
     * Reads all input from stdin.
     *
     * @return The input text
     * @throws IOException if reading fails
     */
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

    /**
     * Processes RPN input through the lexer, parser, and generator.
     *
     * @param input The RPN input text
     * @return The LaTeX output
     * @throws RpnException if lexing or parsing fails
     */
    private static String process(String input) throws RpnException {
        // Lexer: text -> tokens
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

        // Parser: tokens -> AST
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Generator: AST -> LaTeX
        LaTeXGenerator generator = new LaTeXGenerator();
        return generator.generate(ast);
    }
}
