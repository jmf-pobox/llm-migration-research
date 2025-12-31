package com.rpn2tex;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.List;

/**
 * Main entry point for the RPN to LaTeX converter.
 * Reads from stdin and writes to stdout.
 */
public class Main {

    /**
     * Main method.
     *
     * @param args Command line arguments (currently unused)
     */
    public static void main(String[] args) {
        try {
            String input = readStdin();
            String output = convert(input);
            System.out.println(output);
        } catch (RpnException e) {
            System.err.println("Error at line " + e.getLine() + ", column " + e.getColumn() + ": " + e.getMessage());
            System.exit(1);
        } catch (IOException e) {
            System.err.println("IO Error: " + e.getMessage());
            System.exit(1);
        }
    }

    private static String readStdin() throws IOException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
        StringBuilder input = new StringBuilder();
        String line;
        while ((line = reader.readLine()) != null) {
            input.append(line).append('\n');
        }
        return input.toString();
    }

    /**
     * Converts RPN input to LaTeX output.
     *
     * @param input RPN expression
     * @return LaTeX expression
     * @throws RpnException If conversion fails
     */
    public static String convert(String input) throws RpnException {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        return generator.generate(ast);
    }
}
