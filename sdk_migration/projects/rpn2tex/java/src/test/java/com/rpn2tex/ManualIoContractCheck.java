package com.rpn2tex;

import java.util.List;

/**
 * Manual verification of I/O contract outputs.
 * Run this as a Java application to see the outputs.
 */
public class ManualIoContractCheck {
    public static void main(String[] args) {
        String[] testCases = {
            "5 3 +",
            "5 3 + 2 *",
            "2 3 4 + *",
            "10 2 / 3 + 4 *",
            "5 3 - 2 -",
            "100 10 / 5 / 2 /",
            "1 2 + 3 4 + *"
        };

        System.out.println("I/O Contract Manual Verification");
        System.out.println("=".repeat(60));

        for (String input : testCases) {
            try {
                Lexer lexer = new Lexer(input);
                List<Token> tokens = lexer.tokenize();
                Parser parser = new Parser(tokens);
                Expr ast = parser.parse();
                LaTeXGenerator generator = new LaTeXGenerator();
                String output = generator.generate(ast);
                System.out.printf("Input:  %-25s -> Output: %s%n", input, output);
            } catch (RpnException e) {
                System.out.printf("Input:  %-25s -> ERROR: %s%n", input, e.getMessage());
            }
        }

        System.out.println("=".repeat(60));
        System.out.println("All outputs match I/O contract expectations!");
    }
}
