// Test the multiplication I/O contract
use rpn2tex::{Lexer, Parser, LaTeXGenerator};

fn main() {
    let test_cases = vec![
        ("4 7 *", r"$4 \times 7$"),
        ("2 3 4 * +", r"$2 + 3 \times 4$"),
        ("3.14 2 *", r"$3.14 \times 2$"),
        ("5 3 * 2 +", r"$5 \times 3 + 2$"),
    ];

    println!("Testing Multiplication Feature I/O Contract:");
    println!("{}", "=".repeat(60));

    for (input, expected) in test_cases {
        let lexer = Lexer::new(input);
        match lexer.tokenize() {
            Ok(tokens) => {
                let parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(ast) => {
                        let generator = LaTeXGenerator::new();
                        let output = generator.generate(&ast);
                        let status = if output == expected { "PASS" } else { "FAIL" };
                        println!("Input:    {:20} {}", format!("\"{}\"", input), status);
                        println!("Expected: {}", expected);
                        println!("Got:      {}", output);
                        if output != expected {
                            println!("ERROR: Output mismatch!");
                        }
                        println!();
                    }
                    Err(e) => {
                        println!("Input: \"{}\" - FAIL (Parser Error)", input);
                        println!("Error: {}", e.message());
                        println!();
                    }
                }
            }
            Err(e) => {
                println!("Input: \"{}\" - FAIL (Lexer Error)", input);
                println!("Error: {}", e.message());
                println!();
            }
        }
    }
}
