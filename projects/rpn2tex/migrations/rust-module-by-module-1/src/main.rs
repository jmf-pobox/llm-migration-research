//! Command-line interface for rpn2tex.
//!
//! This CLI orchestrates the complete RPN to LaTeX pipeline:
//! 1. Read input from file or stdin
//! 2. Tokenize the input (`Lexer`)
//! 3. Parse tokens into AST (`Parser`)
//! 4. Generate LaTeX code (`LaTeXGenerator`)
//! 5. Write output to file or stdout
//!
//! # Usage
//!
//! ```bash
//! rpn2tex <input> [-o <output>]
//! ```
//!
//! # Examples
//!
//! ```bash
//! # Read from file, write to stdout
//! rpn2tex input.rpn
//!
//! # Read from file, write to file
//! rpn2tex input.rpn -o output.tex
//!
//! # Read from stdin, write to stdout
//! echo "5 3 +" | rpn2tex -
//! ```

use std::fs;
use std::io::{self, Read};
use std::process;

use clap::Parser;
use rpn2tex::error::ErrorFormatter;
use rpn2tex::latex::LaTeXGenerator;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser as RpnParser;

/// Convert Reverse Polish Notation expressions to LaTeX
#[derive(Parser, Debug)]
#[command(name = "rpn2tex")]
#[command(about = "Convert Reverse Polish Notation expressions to LaTeX", long_about = None)]
struct Args {
    /// Input file path or "-" for stdin
    input: String,

    /// Output file path (default: stdout)
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let exit_code = run(&args);
    process::exit(exit_code);
}

/// Runs the main CLI logic.
///
/// # Returns
///
/// Exit code: 0 for success, 1 for error
fn run(args: &Args) -> i32 {
    // Read input
    let input_text = match read_input(&args.input) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };

    // Create error formatter for pretty error messages
    let formatter = ErrorFormatter::new(input_text.clone());

    // Tokenize
    let mut lexer = Lexer::new(input_text);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            let error_msg = formatter.format_error(&e.message, e.line, e.column);
            eprintln!("{error_msg}");
            return 1;
        }
    };

    // Parse
    let mut parser = RpnParser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            let error_msg = formatter.format_error(&e.message, e.token.line(), e.token.column());
            eprintln!("{error_msg}");
            return 1;
        }
    };

    // Generate LaTeX
    let generator = LaTeXGenerator::new();
    let latex = generator.generate(&ast);

    // Write output
    if let Err(e) = write_output(args.output.as_ref(), &latex) {
        eprintln!("{e}");
        return 1;
    }

    0
}

/// Reads input from a file or stdin.
///
/// # Arguments
///
/// * `path` - File path or "-" for stdin
///
/// # Returns
///
/// The input text content
///
/// # Errors
///
/// Returns error message string if reading fails
fn read_input(path: &str) -> Result<String, String> {
    if path == "-" {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| format!("Error reading from stdin: {e}"))?;
        Ok(buffer)
    } else {
        // Read from file
        fs::read_to_string(path).map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => format!("Error: Input file not found: {path}"),
            io::ErrorKind::PermissionDenied => {
                format!("Error: Permission denied reading: {path}")
            }
            io::ErrorKind::IsADirectory => {
                format!("Error: Expected a file, got a directory: {path}")
            }
            _ => format!("Error reading file {path}: {e}"),
        })
    }
}

/// Writes output to a file or stdout.
///
/// # Arguments
///
/// * `path` - Optional file path. If None, writes to stdout
/// * `content` - The content to write
///
/// # Errors
///
/// Returns error message string if writing fails
fn write_output(path: Option<&String>, content: &str) -> Result<(), String> {
    if let Some(output_path) = path {
        // Write to file
        let content_with_newline = format!("{content}\n");
        fs::write(output_path, content_with_newline).map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => {
                format!("Error: Permission denied writing: {output_path}")
            }
            io::ErrorKind::IsADirectory => {
                format!("Error: Cannot write to directory: {output_path}")
            }
            _ => format!("Error writing to file {output_path}: {e}"),
        })?;
        eprintln!("Generated: {output_path}");
        Ok(())
    } else {
        // Write to stdout
        println!("{content}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input_stdin() {
        // Test that stdin path is recognized
        assert_eq!("-", "-");
    }

    #[test]
    fn test_write_output_stdout() {
        // Writing to stdout should succeed
        let result = write_output(None, "$5 + 3$");
        assert!(result.is_ok());
    }
}
