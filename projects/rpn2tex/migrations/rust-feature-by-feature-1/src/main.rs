//! Command-line interface for rpn2tex.
//!
//! This module provides the CLI entry point that orchestrates
//! the pipeline: read → tokenize → parse → generate → write.

use std::io::{self, Read};
use std::process;

use rpn2tex::error::ErrorFormatter;
use rpn2tex::latex::LaTeXGenerator;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;

fn main() {
    process::exit(run());
}

fn run() -> i32 {
    // Read from stdin
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error reading input: {e}");
        return 1;
    }

    // Strip trailing newline if present
    let input = input.trim_end();

    // Create error formatter
    let formatter = ErrorFormatter::new(input);

    // Tokenize
    let mut lexer = Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            let formatted = formatter.format_error(&e.message, e.line, e.column);
            eprintln!("{formatted}");
            return 1;
        }
    };

    // Parse
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            let formatted = formatter.format_error(&e.message, e.token.line, e.token.column);
            eprintln!("{formatted}");
            return 1;
        }
    };

    // Generate LaTeX
    let generator = LaTeXGenerator::new();
    let latex = generator.generate(&ast);

    // Output
    println!("{latex}");

    0
}
