import com.rpn2tex.*;
import java.util.*;

/**
 * Test program to validate LaTeXGenerator against I/O contract.
 */
public class test_latex_gen {
    public static void main(String[] args) {
        String[][] testCases = {
            {"5 3 +", "$5 + 3$"},
            {"5 3 -", "$5 - 3$"},
            {"4 7 *", "$4 \\times 7$"},
            {"10 2 /", "$10 \\div 2$"},
            {"5 3 + 2 *", "$( 5 + 3 ) \\times 2$"},
            {"5 3 * 2 +", "$5 \\times 3 + 2$"},
            {"10 2 / 5 *", "$10 \\div 2 \\times 5$"},
            {"5 3 - 2 -", "$5 - 3 - 2$"},
            {"100 10 / 5 / 2 /", "$100 \\div 10 \\div 5 \\div 2$"},
            {"1 2 + 3 + 4 +", "$1 + 2 + 3 + 4$"},
            {"2 3 4 * +", "$2 + 3 \\times 4$"},
            {"2 3 + 4 *", "$( 2 + 3 ) \\times 4$"},
            {"2 3 4 + *", "$2 \\times ( 3 + 4 )$"},
            {"2 3 * 4 +", "$2 \\times 3 + 4$"},
            {"3.14 2 *", "$3.14 \\times 2$"},
            {"1.5 0.5 +", "$1.5 + 0.5$"},
            {"1 2 + 3 4 + *", "$( 1 + 2 ) \\times ( 3 + 4 )$"},
            {"10 2 / 3 + 4 *", "$( 10 \\div 2 + 3 ) \\times 4$"}
        };

        int passed = 0;
        int failed = 0;

        System.out.println("LaTeX Generator I/O Contract Validation");
        System.out.println("========================================\n");

        for (String[] testCase : testCases) {
            String input = testCase[0];
            String expected = testCase[1];

            try {
                Lexer lexer = new Lexer(input);
                List<Token> tokens = lexer.tokenize();

                Parser parser = new Parser(tokens);
                Expr ast = parser.parse();

                LaTeXGenerator generator = new LaTeXGenerator();
                String actual = generator.generate(ast);

                if (actual.equals(expected)) {
                    System.out.println("[PASS] " + input);
                    System.out.println("  Expected: " + expected);
                    System.out.println("  Got:      " + actual);
                    passed++;
                } else {
                    System.out.println("[FAIL] " + input);
                    System.out.println("  Expected: " + expected);
                    System.out.println("  Got:      " + actual);
                    failed++;
                }
                System.out.println();

            } catch (Exception e) {
                System.out.println("[ERROR] " + input);
                System.out.println("  Expected: " + expected);
                System.out.println("  Error:    " + e.getMessage());
                System.out.println();
                failed++;
            }
        }

        System.out.println("========================================");
        System.out.println("Results: " + passed + " passed, " + failed + " failed");
        System.out.println("========================================");

        System.exit(failed == 0 ? 0 : 1);
    }
}
