package com.rpn2tex;

import java.io.*;
import java.nio.file.*;
import java.util.*;

/**
 * Command-line interface for rpn2tex.
 *
 * <p>Converts RPN (Reverse Polish Notation) expressions to LaTeX format.
 * Supports reading from files or stdin, and writing to files or stdout.
 *
 * <p>Usage:
 * <pre>
 * java com.rpn2tex.Main input.txt [-o output.txt]
 * echo "5 3 +" | java com.rpn2tex.Main -
 * </pre>
 */
public class Main {
    public static void main(String[] args) {
        System.exit(run(args));
    }

    /**
     * Main application logic.
     *
     * @param args Command-line arguments
     * @return Exit code: 0 for success, 1 for error
     */
    public static int run(String[] args) {
        String input = null;
        String output = null;

        // Parse command-line arguments
        for (int i = 0; i < args.length; i++) {
            if (args[i].equals("-o") || args[i].equals("--output")) {
                if (i + 1 < args.length) {
                    output = args[++i];
                }
            } else if (args[i].equals("-") || !args[i].startsWith("-")) {
                input = args[i];
            }
        }

        if (input == null) {
            System.err.println("Error: Input file required");
            return 1;
        }

        // Read input from file or stdin
        String text;
        try {
            if (input.equals("-")) {
                text = readStdin();
            } else {
                text = Files.readString(Paths.get(input));
            }
        } catch (NoSuchFileException e) {
            System.err.println("Error: Input file not found: " + input);
            return 1;
        } catch (IOException e) {
            System.err.println("Error: " + e.getMessage());
            return 1;
        }

        // Process: Lexer -> Parser -> LaTeX Generator
        String latex;
        try {
            Lexer lexer = new Lexer(text);
            List<Token> tokens = lexer.tokenize();

            Parser parser = new Parser(tokens);
            Expr ast = parser.parse();

            LaTeXGenerator generator = new LaTeXGenerator();
            latex = generator.generate(ast);

        } catch (RpnException e) {
            System.err.println(e.format(text));
            return 1;
        }

        // Write output to file or stdout
        if (output != null) {
            try {
                Files.writeString(Paths.get(output), latex + "\n");
                System.err.println("Generated: " + output);
            } catch (IOException e) {
                System.err.println("Error: " + e.getMessage());
                return 1;
            }
        } else {
            System.out.println(latex);
        }

        return 0;
    }

    /**
     * Reads all available input from stdin.
     *
     * @return The input text (without trailing newline if present)
     * @throws IOException If reading fails
     */
    private static String readStdin() throws IOException {
        StringBuilder sb = new StringBuilder();
        try (Scanner scanner = new Scanner(System.in)) {
            while (scanner.hasNextLine()) {
                sb.append(scanner.nextLine()).append("\n");
            }
        }
        String result = sb.toString();
        // Remove trailing newline added by our loop
        if (result.endsWith("\n")) {
            result = result.substring(0, result.length() - 1);
        }
        return result;
    }
}
