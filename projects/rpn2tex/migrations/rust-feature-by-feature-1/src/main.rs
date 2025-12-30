//! Command-line interface for RPN to LaTeX conversion.
//!
//! This program reads RPN expressions from stdin or command-line arguments
//! and outputs LaTeX mathematical notation.

use std::env;
use std::io::{self, Read};
use std::process;

use rpn2tex::{LaTeXGenerator, Lexer, Parser};

/// Main entry point for the CLI.
fn main() {
    let input = get_input().unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    match process_input(&input) {
        Ok(latex) => println!("{}", latex),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

/// Gets input from command-line arguments or stdin.
fn get_input() -> io::Result<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Use command-line argument
        Ok(args[1..].join(" "))
    } else {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer.trim().to_string())
    }
}

/// Processes RPN input and returns LaTeX output.
fn process_input(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;

    let parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_single_number() {
        let result = process_input("5").unwrap();
        assert_eq!(result, "$5$");
    }

    #[test]
    fn test_process_decimal() {
        let result = process_input("3.14").unwrap();
        assert_eq!(result, "$3.14$");
    }

    #[test]
    fn test_process_whitespace() {
        let result = process_input("  42  ").unwrap();
        assert_eq!(result, "$42$");
    }

    #[test]
    fn test_process_invalid() {
        let result = process_input("@");
        assert!(result.is_err());
    }
}
