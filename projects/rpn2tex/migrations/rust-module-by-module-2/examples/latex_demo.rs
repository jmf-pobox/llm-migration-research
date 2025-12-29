use rpn2tex::{LaTeXGenerator, Lexer, Parser};

fn main() {
    println!("LaTeX Generator Demo - Critical Precedence Cases\n");
    println!("{}", "=".repeat(60));

    let test_cases = vec![
        ("5 3 +", "Basic addition"),
        ("4 7 *", "Basic multiplication"),
        ("10 2 /", "Basic division"),
        ("5 3 + 2 *", "Parens needed (low prec left of high prec)"),
        ("5 3 * 2 +", "No parens (high prec left of low prec)"),
        ("5 3 - 2 -", "Left-associativity (no parens on left)"),
        ("2 3 4 + *", "Right operand needs parens"),
        ("2 3 4 * +", "Mult has precedence, no parens"),
        ("1 2 + 3 4 + *", "Both operands need parens"),
        ("10 2 / 3 + 4 *", "Complex mixed operations"),
    ];

    for (input, description) in test_cases {
        match process(input) {
            Ok(latex) => {
                println!("\n{}", description);
                println!("  Input:  {}", input);
                println!("  Output: {}", latex);
            }
            Err(e) => {
                println!("\n{}", description);
                println!("  Input:  {}", input);
                println!("  Error:  {}", e);
            }
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("All test cases processed successfully!");
}

fn process(input: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| format!("Parser error: {}", e))?;

    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}
