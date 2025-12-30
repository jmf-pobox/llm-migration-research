//! Command-line interface for rpn2tex.
//!
//! This module provides the CLI entry point that orchestrates
//! the full compilation pipeline: read → tokenize → parse → generate → write.
//!
//! # Usage
//!
//! ```bash
//! # Output to stdout
//! rpn2tex input.rpn
//!
//! # Output to file
//! rpn2tex input.rpn -o output.tex
//!
//! # Read from stdin
//! echo "5 3 +" | rpn2tex -
//! ```
//!
//! # Exit Codes
//!
//! - 0: Success
//! - 1: Error (lexer error, parser error, or I/O error)

use clap::Parser as ClapParser;
use rpn2tex::error::ErrorFormatter;
use rpn2tex::latex::LaTeXGenerator;
use rpn2tex::lexer::{Lexer, LexerError};
use rpn2tex::parser::{Parser, ParserError};
use std::fs;
use std::io::{self, Read, Write};
use std::process;

/// Convert RPN expressions to LaTeX
#[derive(ClapParser)]
#[command(name = "rpn2tex")]
#[command(version = "0.1.0")]
#[command(about = "Convert RPN expressions to LaTeX math mode", long_about = None)]
struct Args {
    /// Input RPN file (use '-' for stdin)
    input: String,

    /// Output LaTeX file (default: stdout)
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    process::exit(run());
}

/// Main application logic.
///
/// Returns exit code: 0 for success, 1 for any error.
fn run() -> i32 {
    // Parse command-line arguments
    let args = Args::parse();

    // Read input
    let text = match read_input(&args.input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };

    // Create error formatter for potential errors
    let formatter = ErrorFormatter::new(text.clone());

    // Process pipeline: tokenize → parse → generate
    let latex = match process_pipeline(&text, &formatter) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("{e}");
            return 1;
        }
    };

    // Write output
    if let Err(e) = write_output(args.output.as_deref(), &latex) {
        eprintln!("{e}");
        return 1;
    }

    0
}

/// Reads input from file or stdin.
///
/// # Arguments
///
/// * `input` - Input file path, or "-" for stdin
///
/// # Errors
///
/// Returns error if file cannot be read or stdin fails.
fn read_input(input: &str) -> Result<String, String> {
    if input == "-" {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| format!("Error: Failed to read from stdin: {e}"))?;
        Ok(buffer)
    } else {
        // Read from file
        fs::read_to_string(input).map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => format!("Error: Input file not found: {input}"),
            io::ErrorKind::PermissionDenied => format!("Error: Permission denied reading: {input}"),
            io::ErrorKind::IsADirectory => {
                format!("Error: Expected a file, got a directory: {input}")
            }
            _ => format!("Error: Failed to read input file: {e}"),
        })
    }
}

/// Processes the full compilation pipeline.
///
/// # Arguments
///
/// * `text` - The source text to process
/// * `formatter` - Error formatter for generating contextual error messages
///
/// # Errors
///
/// Returns formatted error string if any stage fails.
fn process_pipeline(text: &str, formatter: &ErrorFormatter) -> Result<String, String> {
    // Tokenize
    let lexer = Lexer::new(text.to_string());
    let tokens = lexer
        .tokenize()
        .map_err(|e| format_lexer_error(&e, formatter))?;

    // Parse
    let parser = Parser::new(tokens);
    let ast = parser
        .parse()
        .map_err(|e| format_parser_error(&e, formatter))?;

    // Generate LaTeX
    let generator = LaTeXGenerator::new();
    let latex = generator.generate(&ast);

    Ok(latex)
}

/// Formats a lexer error with source context.
///
/// # Arguments
///
/// * `error` - The lexer error to format
/// * `formatter` - Error formatter with source context
fn format_lexer_error(error: &LexerError, formatter: &ErrorFormatter) -> String {
    formatter.format_error(&error.message, error.line, error.column, 0)
}

/// Formats a parser error with source context.
///
/// # Arguments
///
/// * `error` - The parser error to format
/// * `formatter` - Error formatter with source context
fn format_parser_error(error: &ParserError, formatter: &ErrorFormatter) -> String {
    formatter.format_error(&error.message, error.token.line, error.token.column, 0)
}

/// Writes output to file or stdout.
///
/// # Arguments
///
/// * `output` - Optional output file path
/// * `latex` - The LaTeX string to write
///
/// # Errors
///
/// Returns error if file cannot be written or stdout fails.
fn write_output(output: Option<&str>, latex: &str) -> Result<(), String> {
    match output {
        Some(path) => {
            // Write to file with newline
            let content = format!("{latex}\n");
            fs::write(path, content).map_err(|e| match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    format!("Error: Permission denied writing: {path}")
                }
                io::ErrorKind::IsADirectory => format!("Error: Cannot write to directory: {path}"),
                _ => format!("Error: Failed to write output file: {e}"),
            })?;
            // Print success message to stderr
            eprintln!("Generated: {path}");
            Ok(())
        }
        None => {
            // Write to stdout without extra newline
            print!("{latex}");
            io::stdout()
                .flush()
                .map_err(|e| format!("Error: Failed to write to stdout: {e}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_pipeline_simple_addition() {
        let text = "5 3 +";
        let formatter = ErrorFormatter::new(text.to_string());
        let result = process_pipeline(text, &formatter);
        assert_eq!(result.unwrap(), "$5 + 3$");
    }

    #[test]
    fn test_process_pipeline_with_parentheses() {
        let text = "5 3 + 2 *";
        let formatter = ErrorFormatter::new(text.to_string());
        let result = process_pipeline(text, &formatter);
        assert_eq!(result.unwrap(), "$( 5 + 3 ) \\times 2$");
    }

    #[test]
    fn test_process_pipeline_decimal() {
        let text = "3.14 2 *";
        let formatter = ErrorFormatter::new(text.to_string());
        let result = process_pipeline(text, &formatter);
        assert_eq!(result.unwrap(), "$3.14 \\times 2$");
    }

    #[test]
    fn test_process_pipeline_lexer_error() {
        let text = "2 3 ^";
        let formatter = ErrorFormatter::new(text.to_string());
        let result = process_pipeline(text, &formatter);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Error:"));
        assert!(error.contains("Unexpected character"));
    }

    #[test]
    fn test_process_pipeline_parser_error() {
        let text = "5 3";
        let formatter = ErrorFormatter::new(text.to_string());
        let result = process_pipeline(text, &formatter);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Error:"));
    }

    #[test]
    fn test_format_lexer_error() {
        let text = "2 3 ^";
        let formatter = ErrorFormatter::new(text.to_string());
        let error = LexerError::new("Unexpected character '^'".to_string(), 1, 5);
        let formatted = format_lexer_error(&error, &formatter);
        assert!(formatted.contains("Error: Unexpected character '^'"));
        assert!(formatted.contains("1 | 2 3 ^"));
        assert!(formatted.contains("^"));
    }

    #[test]
    fn test_read_input_stdin_placeholder() {
        // Note: Cannot easily test stdin reading in unit tests
        // This would be covered by integration tests
    }

    #[test]
    fn test_write_output_stdout() {
        // Test that writing to stdout (None) succeeds
        let result = write_output(None, "$5 + 3$");
        assert!(result.is_ok());
    }
}
