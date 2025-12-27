//! CLI entry point for rpn2tex.
//!
//! This binary provides a command-line interface for converting RPN expressions
//! to LaTeX format. It reads input from files or stdin, processes through the
//! lexer, parser, and LaTeX generator pipeline, and writes output to files or stdout.

use clap::Parser;
use rpn2tex::error::ErrorFormatter;
use rpn2tex::latex::LaTeXGenerator;
use rpn2tex::lexer::{Lexer, LexerError};
use rpn2tex::parser::{Parser as RpnParser, ParserError};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

/// Convert RPN expressions to LaTeX format
#[derive(Parser)]
#[command(name = "rpn2tex")]
#[command(about = "Convert Reverse Polish Notation expressions to LaTeX", long_about = None)]
struct Args {
    /// Input RPN file (use '-' for stdin)
    input: String,

    /// Output LaTeX file (default: stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    std::process::exit(run());
}

/// Main execution function.
///
/// Returns an exit code:
/// - 0: Success or parse error
/// - 1: I/O error
fn run() -> i32 {
    let args = Args::parse();

    // Read input
    let text = match read_input(&args.input) {
        Ok(text) => text,
        Err(msg) => {
            eprintln!("Error: {msg}");
            return 1;
        }
    };

    // Process pipeline
    let error_formatter = ErrorFormatter::new(text.clone());
    let latex = match process_pipeline(&text) {
        Ok(latex) => latex,
        Err(error_msg) => {
            // Format the error using ErrorFormatter
            let formatted_err = match error_msg {
                PipelineError::Lexer(err) => {
                    error_formatter.format_error(&err.message, err.line, err.column)
                }
                PipelineError::Parser(err) => {
                    error_formatter.format_error(&err.message, err.token.line, err.token.column)
                }
            };
            eprintln!("{formatted_err}");
            return 0; // Note: parse errors return 0!
        }
    };

    // Write output
    match write_output(args.output.as_ref(), &latex) {
        Ok(()) => 0,
        Err(msg) => {
            eprintln!("Error: {msg}");
            1
        }
    }
}

/// Reads input from a file or stdin.
///
/// # Arguments
///
/// * `input` - Path to input file, or "-" for stdin
///
/// # Returns
///
/// The input text as a string, or an error message.
fn read_input(input: &str) -> Result<String, String> {
    if input == "-" {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| format!("Failed to read stdin: {e}"))?;
        Ok(buffer)
    } else {
        fs::read_to_string(input).map_err(|e| format!("Failed to read file '{input}': {e}"))
    }
}

/// Writes output to a file or stdout.
///
/// # Arguments
///
/// * `output` - Optional path to output file, or None for stdout
/// * `latex` - The LaTeX string to write
///
/// # Returns
///
/// Ok(()) on success, or an error message.
fn write_output(output: Option<&PathBuf>, latex: &str) -> Result<(), String> {
    if let Some(path) = output {
        // Write to file with trailing newline
        let content = format!("{latex}\n");
        fs::write(path, content).map_err(|e| format!("Failed to write file: {e}"))?;
        eprintln!("Generated: {}", path.display());
        Ok(())
    } else {
        // Write to stdout with NO trailing newline
        print!("{latex}");
        Ok(())
    }
}

/// Pipeline error type combining lexer and parser errors.
#[derive(Debug)]
enum PipelineError {
    Lexer(LexerError),
    Parser(ParserError),
}

/// Processes the RPN expression through the complete pipeline.
///
/// # Arguments
///
/// * `text` - The RPN expression text
///
/// # Returns
///
/// The generated LaTeX string, or a pipeline error.
fn process_pipeline(text: &str) -> Result<String, PipelineError> {
    // Tokenize
    let tokens = Lexer::new(text).tokenize().map_err(PipelineError::Lexer)?;

    // Parse
    let ast = RpnParser::new(tokens)
        .parse()
        .map_err(PipelineError::Parser)?;

    // Generate LaTeX
    let generator = LaTeXGenerator;
    Ok(generator.generate(&ast))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_pipeline_basic() {
        let result = process_pipeline("5 3 +");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$5 + 3$");
    }

    #[test]
    fn test_process_pipeline_multiplication() {
        let result = process_pipeline("4 7 *");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r"$4 \times 7$");
    }

    #[test]
    fn test_process_pipeline_division() {
        let result = process_pipeline("10 2 /");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r"$10 \div 2$");
    }

    #[test]
    fn test_process_pipeline_complex() {
        let result = process_pipeline("5 3 + 2 *");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_process_pipeline_lexer_error() {
        let result = process_pipeline("2 3 ^");
        assert!(result.is_err());
    }

    #[test]
    fn test_process_pipeline_parser_error() {
        let result = process_pipeline("5 +");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_input_stdin() {
        // We can't easily test stdin reading in unit tests
        // This would require mocking stdin
    }

    #[test]
    fn test_write_output_stdout() {
        // We can't easily test stdout writing in unit tests
        // This would require capturing stdout
    }
}
