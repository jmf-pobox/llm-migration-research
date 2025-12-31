//! Demo program showing LaTeX generation output
//!
//! This demonstrates the complete pipeline: Lexer → Parser → LaTeX Generator

use rpn2tex::latex::LatexGenerator;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;

fn main() {
    let test_cases = vec![
        ("5 3 +", "Simple addition"),
        ("5 3 + 2 *", "Precedence: (5+3)*2"),
        ("5 3 * 2 +", "Precedence: 5*3+2"),
        ("10 2 / 3 + 4 *", "Complex expression"),
        ("2 3 4 + *", "Right operand grouping"),
        ("5 3 - 2 -", "Left associativity"),
        ("5 3 2 - -", "Right associativity (needs parens)"),
        ("3.14 2 *", "Floating point"),
        ("5", "Single number"),
    ];

    println!("LaTeX Generation Examples");
    println!("=========================\n");

    for (input, description) in test_cases {
        match process_rpn(input) {
            Ok(output) => {
                println!("Input:  {}", input);
                println!("Desc:   {}", description);
                println!("Output: {}", output);
                println!();
            }
            Err(err) => {
                println!("Input:  {}", input);
                println!("Error:  {}", err);
                println!();
            }
        }
    }

    println!("\nError Cases");
    println!("===========\n");

    let error_cases = vec![
        ("", "Empty expression"),
        ("5 3", "Missing operator"),
        ("5 3 + +", "Insufficient operands"),
        ("2 3 ^", "Unsupported operator"),
    ];

    for (input, description) in error_cases {
        match process_rpn(input) {
            Ok(output) => {
                println!("Input:  {:?}", input);
                println!("Output: {} (unexpected!)", output);
                println!();
            }
            Err(err) => {
                println!("Input:  {:?}", input);
                println!("Desc:   {}", description);
                println!("Error:  {}", err);
                println!();
            }
        }
    }
}

fn process_rpn(input: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let generator = LatexGenerator::new();
    Ok(generator.generate(&ast))
}
