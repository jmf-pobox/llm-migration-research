//! CLI for the rpn2tex compiler.

use rpn2tex::{latex::LaTeXGenerator, lexer::Lexer, parser::Parser};
use std::env;
use std::io::{self, Read};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 {
        // Read from command-line argument
        args[1].clone()
    } else {
        // Read from stdin
        let mut buffer = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buffer) {
            eprintln!("Error reading from stdin: {e}");
            process::exit(1);
        }
        buffer.trim().to_string()
    };

    // Compile the input
    match compile(&input) {
        Ok(latex) => {
            println!("{latex}");
        }
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}

fn compile(input: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Error: {}", e.message))?;

    let mut parser = Parser::new(tokens);
    let expr = parser
        .parse()
        .map_err(|e| format!("Error: {}", e.message))?;

    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&expr))
}
