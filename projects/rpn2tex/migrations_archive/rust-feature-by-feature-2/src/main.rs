//! RPN to LaTeX converter - Command-line interface.

use std::env;
use std::io::{self, Read};
use std::process;

mod ast;
mod error;
mod latex;
mod lexer;
mod parser;
mod tokens;

use error::ErrorFormatter;
use latex::LatexGenerator;
use lexer::Lexer;
use parser::Parser;

/// Converts RPN expression to LaTeX.
///
/// # Errors
///
/// Returns an error message if lexing, parsing, or generation fails.
fn convert_rpn_to_latex(input: &str) -> Result<String, String> {
    // Lexical analysis
    let mut lexer = Lexer::new(input);
    let tokens = lexer
        .scan_tokens()
        .map_err(|e| ErrorFormatter::format(&e, input))?;

    // Parsing
    let mut parser = Parser::new(tokens);
    let expr = parser
        .parse()
        .map_err(|e| ErrorFormatter::format(&e, input))?;

    // Code generation
    let generator = LatexGenerator::new();
    let latex = generator.generate(&expr);

    Ok(latex)
}

/// Reads input from stdin or a file specified by command-line argument.
///
/// # Errors
///
/// Returns an error if reading from stdin or a file fails.
fn read_input() -> io::Result<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "-" {
        // Read from stdin
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        Ok(input.trim().to_string())
    } else if args.len() > 1 {
        // Read from file
        std::fs::read_to_string(&args[1]).map(|s| s.trim().to_string())
    } else {
        // No arguments, read from stdin
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        Ok(input.trim().to_string())
    }
}

fn main() {
    let input = match read_input() {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            process::exit(1);
        }
    };

    match convert_rpn_to_latex(&input) {
        Ok(latex) => println!("{}", latex),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_integer() {
        let result = convert_rpn_to_latex("5").unwrap();
        assert_eq!(result, "$5$");
    }

    #[test]
    fn test_convert_decimal() {
        let result = convert_rpn_to_latex("3.14").unwrap();
        assert_eq!(result, "$3.14$");
    }

    #[test]
    fn test_convert_negative() {
        let result = convert_rpn_to_latex("-5").unwrap();
        assert_eq!(result, "$-5$");
    }

    #[test]
    fn test_convert_with_whitespace() {
        let result = convert_rpn_to_latex("  42  ").unwrap();
        assert_eq!(result, "$42$");
    }
}
