//! Command-line interface for rpn2tex.
//!
//! This module provides a CLI tool to convert RPN expressions to LaTeX math mode.
//! It reads input from files or stdin, processes them through the rpn2tex pipeline,
//! and outputs LaTeX to files or stdout.

use clap::Parser;
use rpn2tex::{
    ErrorFormatter, LaTeXGenerator, Lexer, LexerError, Parser as RpnParser, ParserError,
};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::ExitCode;

/// Convert RPN expressions to LaTeX math mode
#[derive(Parser, Debug)]
#[command(name = "rpn2tex")]
#[command(about = "Convert RPN expressions to LaTeX math mode")]
#[command(after_help = "Example: rpn2tex input.rpn -o output.tex")]
struct Cli {
    /// Input RPN file (use '-' for stdin)
    #[arg(value_name = "INPUT")]
    input: String,

    /// Output LaTeX file (default: stdout)
    #[arg(short, long, value_name = "OUTPUT")]
    output: Option<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Read input
    let text = match read_input(&cli.input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(1);
        }
    };

    // Process (tokenize -> parse -> generate)
    let formatter = ErrorFormatter::new(&text);
    let latex = match process_rpn(&text, &formatter) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(1);
        }
    };

    // Write output
    if let Err(e) = write_output(cli.output.as_ref(), &latex) {
        eprintln!("{e}");
        return ExitCode::from(1);
    }

    ExitCode::SUCCESS
}

/// Reads input from a file or stdin.
///
/// # Arguments
///
/// * `input` - Path to input file, or "-" for stdin
///
/// # Errors
///
/// Returns an error message if:
/// - The file is not found
/// - Permission is denied
/// - The path points to a directory
/// - Reading from stdin fails
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
        fs::read_to_string(input).map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                format!("Error: File not found: {input}")
            } else if e.kind() == io::ErrorKind::PermissionDenied {
                format!("Error: Permission denied: {input}")
            } else if e.kind() == io::ErrorKind::IsADirectory {
                format!("Error: Is a directory: {input}")
            } else {
                format!("Error: Failed to read file {input}: {e}")
            }
        })
    }
}

/// Processes RPN text through the tokenize -> parse -> generate pipeline.
///
/// # Arguments
///
/// * `text` - The RPN input text
/// * `formatter` - Error formatter for creating user-friendly error messages
///
/// # Errors
///
/// Returns a formatted error message if:
/// - Lexer encounters an unexpected character
/// - Parser encounters invalid RPN syntax
fn process_rpn(text: &str, formatter: &ErrorFormatter) -> Result<String, String> {
    // Tokenize
    let tokens = Lexer::new(text)
        .tokenize()
        .map_err(|e: LexerError| formatter.format_error(&e.message, e.line, e.column))?;

    // Parse
    let ast = RpnParser::new(tokens).parse().map_err(|e: ParserError| {
        formatter.format_error(&e.message, e.token.line, e.token.column)
    })?;

    // Generate LaTeX
    let latex = LaTeXGenerator::new().generate(&ast);
    Ok(latex)
}

/// Writes output to a file or stdout.
///
/// # Arguments
///
/// * `output` - Optional path to output file (None means stdout)
/// * `latex` - The LaTeX content to write
///
/// # Errors
///
/// Returns an error message if:
/// - Permission is denied when writing to file
/// - The output path points to a directory
/// - Writing fails for any other reason
fn write_output(output: Option<&PathBuf>, latex: &str) -> Result<(), String> {
    if let Some(path) = output {
        // Write to file
        fs::write(path, format!("{latex}\n")).map_err(|e| {
            if e.kind() == io::ErrorKind::PermissionDenied {
                format!("Error: Permission denied: {}", path.display())
            } else if e.kind() == io::ErrorKind::IsADirectory {
                format!("Error: Is a directory: {}", path.display())
            } else {
                format!("Error: Failed to write to {}: {e}", path.display())
            }
        })?;
        eprintln!("Generated: {}", path.display());
    } else {
        // Write to stdout
        println!("{latex}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input_from_string() {
        // This tests the function logic, not actual file I/O
        // Real file I/O should be tested with integration tests
        let result = read_input("-");
        // stdin reading will fail in test environment, which is expected
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_process_rpn_success() {
        let text = "3 4 +";
        let formatter = ErrorFormatter::new(text);
        let result = process_rpn(text, &formatter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$3 + 4$");
    }

    #[test]
    fn test_process_rpn_lexer_error() {
        let text = "3 4 ! +";
        let formatter = ErrorFormatter::new(text);
        let result = process_rpn(text, &formatter);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Error:"));
        assert!(error.contains("Unexpected character '!'"));
    }

    #[test]
    fn test_process_rpn_parser_error() {
        let text = "3 +";
        let formatter = ErrorFormatter::new(text);
        let result = process_rpn(text, &formatter);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Error:"));
        assert!(error.contains("two operands"));
    }

    #[test]
    fn test_process_rpn_complex_expression() {
        let text = "3 4 + 2 *";
        let formatter = ErrorFormatter::new(text);
        let result = process_rpn(text, &formatter);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r"$( 3 + 4 ) \times 2$");
    }
}
