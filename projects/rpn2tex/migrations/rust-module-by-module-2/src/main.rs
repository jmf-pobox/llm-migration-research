//! Command-line interface for the rpn2tex converter.
//!
//! This module provides a CLI that orchestrates the entire RPN to LaTeX conversion
//! pipeline. It handles argument parsing, file I/O, error reporting, and pipeline
//! orchestration.
//!
//! # Pipeline
//!
//! The CLI orchestrates these steps:
//! 1. Parse command-line arguments
//! 2. Read input from file or stdin
//! 3. Tokenize input (Lexer)
//! 4. Parse tokens into AST (Parser)
//! 5. Generate LaTeX from AST (LaTeX Generator)
//! 6. Write output to file or stdout
//!
//! # Exit Codes
//!
//! - 0: Success
//! - 1: Error (file I/O, lexing, or parsing error)

use rpn2tex::error::ErrorFormatter;
use rpn2tex::latex::LaTeXGenerator;
use rpn2tex::lexer::{Lexer, LexerError};
use rpn2tex::parser::{Parser, ParserError};
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process;

/// Main entry point for the rpn2tex CLI.
///
/// Returns 0 for success, 1 for any error.
fn main() {
    process::exit(run());
}

/// Runs the CLI application and returns exit code.
///
/// This is separated from main() to allow easier testing and error handling.
fn run() -> i32 {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input-file> [-o <output-file>]", args[0]);
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  <input-file>    Input RPN file (use '-' for stdin)");
        eprintln!("  -o <output>     Output LaTeX file (default: stdout)");
        return 1;
    }

    let input_path = &args[1];
    let mut output_path: Option<PathBuf> = None;

    // Parse optional output file argument
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: -o flag requires an argument");
                    return 1;
                }
                output_path = Some(PathBuf::from(&args[i + 1]));
                i += 2;
            }
            _ => {
                eprintln!("Error: Unknown argument '{}'", args[i]);
                return 1;
            }
        }
    }

    // Read input
    let source = match read_input(input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            return 1;
        }
    };

    // Run the conversion pipeline
    let latex_output = match convert_rpn_to_latex(&source) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };

    // Write output
    if let Err(e) = write_output(&latex_output, output_path.as_ref()) {
        eprintln!("Error writing output: {}", e);
        return 1;
    }

    // Print success message to stderr
    if let Some(path) = output_path {
        eprintln!("Generated: {}", path.display());
    }

    0
}

/// Reads input from a file or stdin.
///
/// # Arguments
///
/// * `path` - Path to input file, or "-" for stdin
///
/// # Errors
///
/// Returns an error if the file cannot be read or if there's an I/O error.
fn read_input(path: &str) -> io::Result<String> {
    if path == "-" {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        // Read from file with better error messages
        fs::read_to_string(path).map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => io::Error::new(
                io::ErrorKind::NotFound,
                format!("Input file not found: {}", path),
            ),
            io::ErrorKind::PermissionDenied => io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("Permission denied: {}", path),
            ),
            _ => {
                if PathBuf::from(path).is_dir() {
                    io::Error::other(format!("Expected a file, got directory: {}", path))
                } else {
                    e
                }
            }
        })
    }
}

/// Writes output to a file or stdout.
///
/// # Arguments
///
/// * `content` - The LaTeX content to write
/// * `path` - Optional output file path. If None, writes to stdout
///
/// # Errors
///
/// Returns an error if the file cannot be written or if there's an I/O error.
fn write_output(content: &str, path: Option<&PathBuf>) -> io::Result<()> {
    if let Some(output_path) = path {
        // Write to file with better error messages
        fs::write(output_path, content).map_err(|e| match e.kind() {
            io::ErrorKind::PermissionDenied => io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("Permission denied: {}", output_path.display()),
            ),
            _ => {
                if output_path.is_dir() {
                    io::Error::other(format!(
                        "Cannot write to directory: {}",
                        output_path.display()
                    ))
                } else {
                    e
                }
            }
        })
    } else {
        // Write to stdout
        print!("{}", content);
        io::stdout().flush()
    }
}

/// Converts RPN input to LaTeX output through the complete pipeline.
///
/// This function orchestrates the lexer, parser, and LaTeX generator to
/// perform the full conversion. It formats errors with source context when
/// they occur.
///
/// # Arguments
///
/// * `source` - The RPN source text
///
/// # Returns
///
/// The LaTeX output string on success, or a formatted error message on failure.
///
/// # Errors
///
/// Returns a formatted error string if:
/// - Lexing fails (invalid characters, malformed tokens)
/// - Parsing fails (invalid RPN structure, missing operands)
fn convert_rpn_to_latex(source: &str) -> Result<String, String> {
    let formatter = ErrorFormatter::new(source);

    // Tokenize
    let lexer = Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format_lexer_error(&formatter, &e))?;

    // Parse
    let mut parser = Parser::new(tokens);
    let ast = parser
        .parse()
        .map_err(|e| format_parser_error(&formatter, &e))?;

    // Generate LaTeX
    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}

/// Formats a lexer error with source context.
///
/// # Arguments
///
/// * `formatter` - The error formatter with source context
/// * `error` - The lexer error to format
///
/// # Returns
///
/// A formatted error message with source context, ready to display to the user.
fn format_lexer_error(formatter: &ErrorFormatter, error: &LexerError) -> String {
    formatter.format_error(error.message(), error.line(), error.column(), 1)
}

/// Formats a parser error with source context.
///
/// # Arguments
///
/// * `formatter` - The error formatter with source context
/// * `error` - The parser error to format
///
/// # Returns
///
/// A formatted error message with source context, ready to display to the user.
fn format_parser_error(formatter: &ErrorFormatter, error: &ParserError) -> String {
    formatter.format_error(error.message(), error.token().line, error.token().column, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_simple_addition() {
        let result = convert_rpn_to_latex("5 3 +").expect("should succeed");
        assert_eq!(result, "$5 + 3$");
    }

    #[test]
    fn test_convert_simple_multiplication() {
        let result = convert_rpn_to_latex("4 7 *").expect("should succeed");
        assert_eq!(result, r"$4 \times 7$");
    }

    #[test]
    fn test_convert_with_precedence() {
        let result = convert_rpn_to_latex("5 3 + 2 *").expect("should succeed");
        assert_eq!(result, r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_convert_lexer_error() {
        let result = convert_rpn_to_latex("2 3 ^");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Line 1, column 5"));
        assert!(error.contains("Unexpected character"));
    }

    #[test]
    fn test_convert_parser_error_too_few_operands() {
        let result = convert_rpn_to_latex("5 +");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Too few operands"));
    }

    #[test]
    fn test_convert_parser_error_empty() {
        let result = convert_rpn_to_latex("");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Empty expression"));
    }

    #[test]
    fn test_convert_parser_error_missing_operator() {
        let result = convert_rpn_to_latex("5 3");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Too many values"));
    }

    #[test]
    fn test_convert_floating_point() {
        let result = convert_rpn_to_latex("3.14 2 *").expect("should succeed");
        assert_eq!(result, r"$3.14 \times 2$");
    }

    #[test]
    fn test_convert_negative_number() {
        let result = convert_rpn_to_latex("-5 3 +").expect("should succeed");
        assert_eq!(result, "$-5 + 3$");
    }

    #[test]
    fn test_convert_complex_expression() {
        let result = convert_rpn_to_latex("1 2 + 3 4 + *").expect("should succeed");
        assert_eq!(result, r"$( 1 + 2 ) \times ( 3 + 4 )$");
    }

    #[test]
    fn test_format_lexer_error() {
        let source = "2 3 ^";
        let formatter = ErrorFormatter::new(source);
        let error = LexerError::new("Unexpected character '^'", 1, 5);

        let formatted = format_lexer_error(&formatter, &error);
        assert!(formatted.contains("Line 1, column 5"));
        assert!(formatted.contains("Unexpected character"));
        assert!(formatted.contains("2 3 ^"));
        assert!(formatted.contains("^")); // Should have caret pointer
    }

    #[test]
    fn test_format_parser_error() {
        use rpn2tex::tokens::{Token, TokenType};

        let source = "5 +";
        let formatter = ErrorFormatter::new(source);
        let token = Token::new(TokenType::Plus, "+".to_string(), 1, 3);
        let error = ParserError::new("Too few operands", token);

        let formatted = format_parser_error(&formatter, &error);
        assert!(formatted.contains("Line 1, column 3"));
        assert!(formatted.contains("Too few operands"));
    }

    #[test]
    fn test_read_input_stdin() {
        // Can't easily test stdin reading without mocking
        // This is a placeholder for manual testing
    }

    #[test]
    fn test_write_output_stdout() {
        // Can't easily test stdout writing without capturing
        // This is a placeholder for manual testing
    }

    // Test all I/O contract cases through the convert function
    #[test]
    fn test_io_contract_case_01() {
        assert_eq!(convert_rpn_to_latex("5 3 +").unwrap(), "$5 + 3$");
    }

    #[test]
    fn test_io_contract_case_02() {
        assert_eq!(convert_rpn_to_latex("5 3 -").unwrap(), "$5 - 3$");
    }

    #[test]
    fn test_io_contract_case_03() {
        assert_eq!(convert_rpn_to_latex("4 7 *").unwrap(), r"$4 \times 7$");
    }

    #[test]
    fn test_io_contract_case_04() {
        assert_eq!(convert_rpn_to_latex("10 2 /").unwrap(), r"$10 \div 2$");
    }

    #[test]
    fn test_io_contract_case_05() {
        assert_eq!(
            convert_rpn_to_latex("5 3 + 2 *").unwrap(),
            r"$( 5 + 3 ) \times 2$"
        );
    }

    #[test]
    fn test_io_contract_case_06() {
        assert_eq!(
            convert_rpn_to_latex("5 3 * 2 +").unwrap(),
            r"$5 \times 3 + 2$"
        );
    }

    #[test]
    fn test_io_contract_case_07() {
        assert_eq!(
            convert_rpn_to_latex("10 2 / 5 *").unwrap(),
            r"$10 \div 2 \times 5$"
        );
    }

    #[test]
    fn test_io_contract_case_08() {
        assert_eq!(convert_rpn_to_latex("5 3 - 2 -").unwrap(), "$5 - 3 - 2$");
    }

    #[test]
    fn test_io_contract_case_09() {
        assert_eq!(
            convert_rpn_to_latex("100 10 / 5 / 2 /").unwrap(),
            r"$100 \div 10 \div 5 \div 2$"
        );
    }

    #[test]
    fn test_io_contract_case_10() {
        assert_eq!(
            convert_rpn_to_latex("1 2 + 3 + 4 +").unwrap(),
            "$1 + 2 + 3 + 4$"
        );
    }

    #[test]
    fn test_io_contract_case_11() {
        assert_eq!(
            convert_rpn_to_latex("2 3 4 * +").unwrap(),
            r"$2 + 3 \times 4$"
        );
    }

    #[test]
    fn test_io_contract_case_12() {
        assert_eq!(
            convert_rpn_to_latex("2 3 + 4 *").unwrap(),
            r"$( 2 + 3 ) \times 4$"
        );
    }

    #[test]
    fn test_io_contract_case_13() {
        assert_eq!(
            convert_rpn_to_latex("2 3 4 + *").unwrap(),
            r"$2 \times ( 3 + 4 )$"
        );
    }

    #[test]
    fn test_io_contract_case_14() {
        assert_eq!(
            convert_rpn_to_latex("2 3 * 4 +").unwrap(),
            r"$2 \times 3 + 4$"
        );
    }

    #[test]
    fn test_io_contract_case_15() {
        assert_eq!(
            convert_rpn_to_latex("3.14 2 *").unwrap(),
            r"$3.14 \times 2$"
        );
    }

    #[test]
    fn test_io_contract_case_16() {
        assert_eq!(convert_rpn_to_latex("1.5 0.5 +").unwrap(), "$1.5 + 0.5$");
    }

    #[test]
    fn test_io_contract_case_17() {
        assert_eq!(
            convert_rpn_to_latex("1 2 + 3 4 + *").unwrap(),
            r"$( 1 + 2 ) \times ( 3 + 4 )$"
        );
    }

    #[test]
    fn test_io_contract_case_18() {
        assert_eq!(
            convert_rpn_to_latex("10 2 / 3 + 4 *").unwrap(),
            r"$( 10 \div 2 + 3 ) \times 4$"
        );
    }

    // Error test cases
    #[test]
    fn test_io_contract_error_01() {
        assert!(convert_rpn_to_latex("2 3 ^").is_err());
    }

    #[test]
    fn test_io_contract_error_02() {
        assert!(convert_rpn_to_latex("2 3 ^ 4 *").is_err());
    }

    #[test]
    fn test_io_contract_error_03() {
        assert!(convert_rpn_to_latex("2 3 4 ^ ^").is_err());
    }
}
