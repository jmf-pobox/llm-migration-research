//! Command-line interface for rpn2tex.
//!
//! This module provides the CLI entry point that orchestrates
//! the pipeline: read -> tokenize -> parse -> generate -> write.

use rpn2tex::latex::LaTeXGenerator;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: rpn2tex <input_file|-> [-o <output_file>]");
        eprintln!("       Use '-' to read from stdin");
        process::exit(1);
    }

    // Read input
    let text = if args[1] == "-" {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    } else {
        let input_path = Path::new(&args[1]);
        fs::read_to_string(input_path).unwrap_or_else(|e| {
            eprintln!("Error reading file '{}': {}", args[1], e);
            process::exit(1);
        })
    };

    // Process: tokenize -> parse -> generate
    let latex = match process_rpn(&text) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // Write output
    if args.len() >= 4 && args[2] == "-o" {
        let output_path = Path::new(&args[3]);
        fs::write(output_path, format!("{}\n", latex)).unwrap_or_else(|e| {
            eprintln!("Error writing to '{}': {}", args[3], e);
            process::exit(1);
        });
        eprintln!("Generated: {}", args[3]);
    } else {
        println!("{}", latex);
    }
}

fn process_rpn(text: &str) -> Result<String, String> {
    // Tokenize
    let lexer = Lexer::new(text);
    let tokens = lexer.tokenize().map_err(|e| format!("Error: {}", e))?;

    // Parse
    let parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| format!("Error: {}", e))?;

    // Generate LaTeX
    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}
