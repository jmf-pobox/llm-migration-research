//! CLI application for converting RPN expressions to LaTeX math mode.
//!
//! This is the main entry point for the rpn2tex command-line tool. It orchestrates
//! the entire pipeline: reading input, tokenizing, parsing, generating LaTeX, and
//! writing output.
//!
//! # Usage
//!
//! ```bash
//! # Read from stdin, write to stdout
//! echo "5 3 +" | rpn2tex -
//!
//! # Read from file, write to stdout
//! rpn2tex input.rpn
//!
//! # Read from stdin, write to file
//! echo "5 3 +" | rpn2tex - -o output.tex
//!
//! # Read from file, write to file
//! rpn2tex input.rpn -o output.tex
//! ```

use clap::Parser as ClapParser;
use rpn2tex::{ErrorFormatter, LaTeXGenerator, Lexer, Parser};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;

/// Command-line arguments for the rpn2tex tool.
#[derive(ClapParser, Debug)]
#[command(name = "rpn2tex")]
#[command(about = "Convert RPN expressions to LaTeX math mode")]
#[command(
    long_about = "Converts Reverse Polish Notation expressions to LaTeX format.\n\nExample: rpn2tex input.rpn -o output.tex"
)]
struct Args {
    /// Input RPN file (use '-' for stdin)
    #[arg(value_name = "INPUT")]
    input: String,

    /// Output LaTeX file (default: stdout)
    #[arg(short = 'o', long = "output", value_name = "OUTPUT")]
    output: Option<PathBuf>,
}

fn main() {
    let exit_code = run();
    process::exit(exit_code);
}

/// Main application logic with error handling.
///
/// Returns exit code: 0 for success, 1 for any error.
fn run() -> i32 {
    let args = Args::parse();

    // Read input
    let text = match read_input(&args.input) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error: {e}");
            return 1;
        }
    };

    // Process: tokenize → parse → generate
    let formatter = ErrorFormatter::new(&text);

    let latex = match process_text(&text) {
        Ok(latex) => latex,
        Err(e) => {
            let formatted = format_error(&formatter, &e);
            eprintln!("{formatted}");
            return 1;
        }
    };

    // Write output
    if let Err(e) = write_output(&args.output, &latex) {
        eprintln!("Error: {e}");
        return 1;
    }

    0
}

/// Reads input from a file or stdin.
///
/// # Arguments
///
/// * `input` - File path or "-" for stdin
///
/// # Errors
///
/// Returns an error if the file cannot be read or stdin cannot be read.
fn read_input(input: &str) -> io::Result<String> {
    if input == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        fs::read_to_string(input)
            .map_err(|e| io::Error::new(e.kind(), format!("Failed to read '{input}': {e}")))
    }
}

/// Processes RPN text through the complete pipeline.
///
/// # Arguments
///
/// * `text` - The RPN expression to process
///
/// # Errors
///
/// Returns a `ProcessError` if:
/// - Lexical analysis fails (invalid characters)
/// - Parsing fails (invalid RPN structure)
fn process_text(text: &str) -> Result<String, ProcessError> {
    // Tokenize
    let mut lexer = Lexer::new(text);
    let tokens = lexer.tokenize().map_err(ProcessError::Lexer)?;

    // Parse
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(ProcessError::Parser)?;

    // Generate LaTeX
    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}

/// Writes LaTeX output to a file or stdout.
///
/// # Arguments
///
/// * `output` - Optional output file path
/// * `latex` - The LaTeX string to write
///
/// # Errors
///
/// Returns an error if the file cannot be written.
fn write_output(output: &Option<PathBuf>, latex: &str) -> io::Result<()> {
    match output {
        Some(path) => {
            // Write to file with newline
            fs::write(path, format!("{latex}\n"))?;
            eprintln!("Generated: {}", path.display());
            Ok(())
        }
        None => {
            // Write to stdout without newline
            print!("{latex}");
            Ok(())
        }
    }
}

/// Formats a process error using the error formatter.
///
/// # Arguments
///
/// * `formatter` - The error formatter with source context
/// * `error` - The process error to format
fn format_error(formatter: &ErrorFormatter, error: &ProcessError) -> String {
    match error {
        ProcessError::Lexer(e) => formatter.format_error(&e.message, e.line, e.column),
        ProcessError::Parser(e) => formatter.format_error(&e.message, e.token.line, e.token.column),
    }
}

/// Errors that can occur during text processing.
#[derive(Debug)]
enum ProcessError {
    /// Error during lexical analysis
    Lexer(rpn2tex::LexerError),
    /// Error during parsing
    Parser(rpn2tex::ParserError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_simple_expression() {
        let result = process_text("5 3 +").unwrap();
        assert_eq!(result, "$5 + 3$");
    }

    #[test]
    fn test_process_multiplication() {
        let result = process_text("4 7 *").unwrap();
        assert_eq!(result, r"$4 \times 7$");
    }

    #[test]
    fn test_process_division() {
        let result = process_text("10 2 /").unwrap();
        assert_eq!(result, r"$10 \div 2$");
    }

    #[test]
    fn test_process_with_precedence() {
        let result = process_text("5 3 + 2 *").unwrap();
        assert_eq!(result, r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_process_lexer_error() {
        let result = process_text("5 3 ^");
        assert!(result.is_err());
        match result.unwrap_err() {
            ProcessError::Lexer(_) => {}
            _ => panic!("Expected LexerError"),
        }
    }

    #[test]
    fn test_process_parser_error() {
        let result = process_text("5 +");
        assert!(result.is_err());
        match result.unwrap_err() {
            ProcessError::Parser(_) => {}
            _ => panic!("Expected ParserError"),
        }
    }

    #[test]
    fn test_format_lexer_error() {
        let text = "2 3 ^";
        let formatter = ErrorFormatter::new(text);
        let mut lexer = Lexer::new(text);
        let err = lexer.tokenize().unwrap_err();
        let process_err = ProcessError::Lexer(err);

        let formatted = format_error(&formatter, &process_err);
        assert!(formatted.contains("Error:"));
        assert!(formatted.contains("Unexpected character '^'"));
        assert!(formatted.contains("1 | 2 3 ^"));
        assert!(formatted.contains("^"));
    }

    #[test]
    fn test_format_parser_error() {
        let text = "5 +";
        let formatter = ErrorFormatter::new(text);
        let mut lexer = Lexer::new(text);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let err = parser.parse().unwrap_err();
        let process_err = ProcessError::Parser(err);

        let formatted = format_error(&formatter, &process_err);
        assert!(formatted.contains("Error:"));
        assert!(formatted.contains("requires two operands"));
    }

    #[test]
    fn test_process_complex_expression() {
        let result = process_text("1 2 + 3 4 + *").unwrap();
        assert_eq!(result, r"$( 1 + 2 ) \times ( 3 + 4 )$");
    }

    #[test]
    fn test_process_float() {
        let result = process_text("3.14 2 *").unwrap();
        assert_eq!(result, r"$3.14 \times 2$");
    }

    #[test]
    fn test_process_empty_input() {
        let result = process_text("");
        assert!(result.is_err());
        match result.unwrap_err() {
            ProcessError::Parser(_) => {}
            _ => panic!("Expected ParserError for empty input"),
        }
    }
}
