package com.rpn2tex;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.NoSuchFileException;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.util.Scanner;

/**
 * Command-line interface for converting RPN expressions to LaTeX math mode.
 *
 * <p>This is the main entry point for the rpn2tex application. It orchestrates
 * the complete pipeline:
 * <ol>
 *   <li>Parse command-line arguments</li>
 *   <li>Read input from file or stdin</li>
 *   <li>Tokenize with {@link Lexer}</li>
 *   <li>Parse with {@link Parser}</li>
 *   <li>Generate LaTeX with {@link LaTeXGenerator}</li>
 *   <li>Write output to file or stdout</li>
 * </ol>
 *
 * <h2>Usage</h2>
 * <pre>
 * rpn2tex [-h] [-o OUTPUT] input
 *
 * Convert RPN expressions to LaTeX math mode
 *
 * positional arguments:
 *   input              Input RPN file (use '-' for stdin)
 *
 * optional arguments:
 *   -h, --help         show this help message and exit
 *   -o OUTPUT, --output OUTPUT
 *                      Output LaTeX file (default: stdout)
 * </pre>
 *
 * <h2>Exit Codes</h2>
 * <ul>
 *   <li>0: Success</li>
 *   <li>1: Error (lexer error, parser error, I/O error, etc.)</li>
 * </ul>
 *
 * <h2>Example Usage</h2>
 * <pre>{@code
 * # Read from stdin, write to stdout
 * echo "5 3 +" | java -cp build/classes/java/main com.rpn2tex.Main -
 *
 * # Read from file, write to stdout
 * java -cp build/classes/java/main com.rpn2tex.Main input.rpn
 *
 * # Read from file, write to file
 * java -cp build/classes/java/main com.rpn2tex.Main input.rpn -o output.tex
 * }</pre>
 *
 * @since 1.0
 */
public class Main {

    /**
     * Main entry point for the application.
     *
     * @param args command-line arguments
     */
    public static void main(String[] args) {
        System.exit(run(args));
    }

    /**
     * Runs the application with the given arguments.
     *
     * <p>This method is separated from {@link #main(String[])} to enable
     * easier testing by returning an exit code rather than calling {@link System#exit(int)}.
     *
     * @param args command-line arguments
     * @return exit code (0 for success, 1 for error)
     */
    public static int run(String[] args) {
        // Parse command-line arguments
        Arguments parsedArgs = parseArguments(args);
        if (parsedArgs == null) {
            return 1;
        }

        // Read input
        String text;
        try {
            if ("-".equals(parsedArgs.input)) {
                text = readFromStdin();
            } else {
                text = readFromFile(parsedArgs.input);
            }
        } catch (NoSuchFileException e) {
            System.err.println("Error: Input file not found: " + parsedArgs.input);
            return 1;
        } catch (IOException e) {
            System.err.println("Error reading input: " + e.getMessage());
            return 1;
        }

        // Process: tokenize -> parse -> generate
        ErrorFormatter formatter = new ErrorFormatter(text);
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
            if (parsedArgs.output != null) {
                try {
                    writeToFile(parsedArgs.output, latex);
                    System.err.println("Generated: " + parsedArgs.output);
                } catch (IOException e) {
                    System.err.println("Error writing output: " + e.getMessage());
                    return 1;
                }
            } else {
                System.out.println(latex);
            }

        } catch (RpnException e) {
            String formatted = formatter.formatError(
                e.getErrorMessage(),
                e.getLine(),
                e.getColumn()
            );
            System.err.println(formatted);
            return 1;
        }

        return 0;
    }

    /**
     * Reads input text from stdin.
     *
     * <p>This method reads all available lines from {@link System#in} until EOF.
     *
     * @return the complete input text
     * @throws IOException if an I/O error occurs
     */
    private static String readFromStdin() throws IOException {
        StringBuilder sb = new StringBuilder();
        try (Scanner scanner = new Scanner(System.in)) {
            while (scanner.hasNextLine()) {
                if (sb.length() > 0) {
                    sb.append("\n");
                }
                sb.append(scanner.nextLine());
            }
        }
        return sb.toString();
    }

    /**
     * Reads input text from a file.
     *
     * @param filePath the path to the input file
     * @return the complete file contents
     * @throws IOException if an I/O error occurs
     */
    private static String readFromFile(String filePath) throws IOException {
        Path path = Paths.get(filePath);
        return Files.readString(path);
    }

    /**
     * Writes output text to a file.
     *
     * @param filePath the path to the output file
     * @param content  the content to write
     * @throws IOException if an I/O error occurs
     */
    private static void writeToFile(String filePath, String content) throws IOException {
        Path path = Paths.get(filePath);
        // Ensure newline at end of file
        Files.writeString(path, content + "\n");
    }

    /**
     * Parses command-line arguments.
     *
     * <p>This method implements a simple argument parser that handles:
     * <ul>
     *   <li>-h, --help: Display help and exit</li>
     *   <li>-o, --output: Specify output file</li>
     *   <li>Positional argument: Input file (required)</li>
     * </ul>
     *
     * @param args command-line arguments
     * @return parsed arguments, or null if help was requested or an error occurred
     */
    private static Arguments parseArguments(String[] args) {
        if (args.length == 0) {
            printUsage();
            return null;
        }

        String input = null;
        String output = null;

        for (int i = 0; i < args.length; i++) {
            String arg = args[i];

            if ("-h".equals(arg) || "--help".equals(arg)) {
                printHelp();
                return null;
            } else if ("-o".equals(arg) || "--output".equals(arg)) {
                if (i + 1 < args.length) {
                    output = args[++i];
                } else {
                    System.err.println("Error: -o/--output requires an argument");
                    printUsage();
                    return null;
                }
            } else if ("-".equals(arg) || !arg.startsWith("-")) {
                // "-" is treated as stdin, not as an option flag
                if (input == null) {
                    input = arg;
                } else {
                    System.err.println("Error: Multiple input files specified");
                    printUsage();
                    return null;
                }
            } else {
                System.err.println("Error: Unknown option: " + arg);
                printUsage();
                return null;
            }
        }

        if (input == null) {
            System.err.println("Error: input argument required");
            printUsage();
            return null;
        }

        return new Arguments(input, output);
    }

    /**
     * Prints usage information to stderr.
     */
    private static void printUsage() {
        System.err.println("usage: rpn2tex [-h] [-o OUTPUT] input");
    }

    /**
     * Prints full help information to stdout.
     */
    private static void printHelp() {
        System.out.println("usage: rpn2tex [-h] [-o OUTPUT] input");
        System.out.println();
        System.out.println("Convert RPN expressions to LaTeX math mode");
        System.out.println();
        System.out.println("positional arguments:");
        System.out.println("  input              Input RPN file (use '-' for stdin)");
        System.out.println();
        System.out.println("optional arguments:");
        System.out.println("  -h, --help         show this help message and exit");
        System.out.println("  -o OUTPUT, --output OUTPUT");
        System.out.println("                     Output LaTeX file (default: stdout)");
    }

    /**
     * Container for parsed command-line arguments.
     *
     * @param input  the input file path or "-" for stdin
     * @param output the output file path, or null for stdout
     */
    static class Arguments {
        final String input;
        final String output;

        Arguments(String input, String output) {
            this.input = input;
            this.output = output;
        }
    }
}
