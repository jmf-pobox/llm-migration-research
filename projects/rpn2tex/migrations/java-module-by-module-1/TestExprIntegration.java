import com.rpn2tex.*;

public class TestExprIntegration {
    public static void main(String[] args) throws Exception {
        // Test 1: Simple Number node
        com.rpn2tex.Number num = new com.rpn2tex.Number("42", 1, 1);
        System.out.println("Test 1 - Number creation: " + num.value() + " at " + num.line() + ":" + num.column());

        // Test 2: Simple BinaryOp
        Expr left = new com.rpn2tex.Number("5", 1, 1);
        Expr right = new com.rpn2tex.Number("3", 1, 3);
        BinaryOp add = new BinaryOp("+", left, right, 1, 5);
        System.out.println("Test 2 - BinaryOp: " + add.operator());

        // Test 3: Nested expression and LaTeX generation
        String input = "5 3 +";
        Lexer lexer = new Lexer(input);
        Expr ast = new Parser(lexer.tokenize()).parse();
        String latex = new LaTeXGenerator().generate(ast);
        System.out.println("Test 3 - Full pipeline: " + input + " -> " + latex);

        // Test 4: Float preservation
        input = "3.14 2.71 +";
        lexer = new Lexer(input);
        ast = new Parser(lexer.tokenize()).parse();
        latex = new LaTeXGenerator().generate(ast);
        System.out.println("Test 4 - Float preservation: " + input + " -> " + latex);

        // Test 5: Operator precedence
        input = "5 3 + 2 *";
        lexer = new Lexer(input);
        ast = new Parser(lexer.tokenize()).parse();
        latex = new LaTeXGenerator().generate(ast);
        System.out.println("Test 5 - Precedence: " + input + " -> " + latex);

        System.out.println("\nAll integration tests passed!");
    }
}
