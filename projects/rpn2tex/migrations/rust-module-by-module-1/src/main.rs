//! RPN to LaTeX Converter CLI
//!
//! Command-line interface for converting Reverse Polish Notation (RPN)
//! mathematical expressions to LaTeX format.
//!
//! # Usage
//!
//! ```text
//! rpn2tex "<expression>"
//! ```
//!
//! # Examples
//!
//! ```text
//! $ rpn2tex "5 3 +"
//! $5 + 3$
//!
//! $ rpn2tex "5 3 + 2 *"
//! $( 5 + 3 ) \times 2$
//! ```

use rpn2tex::{LatexGenerator, Lexer, Parser, Rpn2TexError};
use std::env;
use std::process;

/// Run the RPN to LaTeX conversion pipeline
///
/// # Arguments
///
/// * `input` - The RPN expression to convert
///
/// # Returns
///
/// The LaTeX output string or an error
///
/// # Errors
///
/// Returns an error if:
/// - Lexical analysis fails (invalid characters)
/// - Parsing fails (invalid RPN syntax)
fn run(input: &str) -> Result<String, Rpn2TexError> {
    // Lexer: tokenize input
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;

    // Parser: build AST from tokens
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;

    // LaTeX Generator: produce LaTeX output
    let generator = LatexGenerator::new();
    let latex = generator.generate(&ast);

    Ok(latex)
}

/// Print usage information to stderr
fn print_usage() {
    eprintln!("Usage: rpn2tex \"<expression>\"");
    eprintln!();
    eprintln!("Convert RPN expressions to LaTeX format.");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  rpn2tex \"5 3 +\"         # Output: $5 + 3$");
    eprintln!("  rpn2tex \"5 3 + 2 *\"     # Output: $( 5 + 3 ) \\times 2$");
    eprintln!("  rpn2tex \"4 7 *\"         # Output: $4 \\times 7$");
    eprintln!("  rpn2tex \"10 2 /\"        # Output: $10 \\div 2$");
}

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if expression argument is provided
    if args.len() < 2 {
        eprintln!("Error: Missing required argument: <expression>");
        eprintln!();
        print_usage();
        process::exit(1);
    }

    // Handle help flag
    if args[1] == "-h" || args[1] == "--help" {
        print_usage();
        process::exit(0);
    }

    let input = &args[1];

    // Run the conversion pipeline
    match run(input) {
        Ok(latex) => {
            // Print result to stdout
            println!("{latex}");
            process::exit(0);
        }
        Err(err) => {
            // Print error to stderr and exit with code 1
            eprintln!("{err}");
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_simple_addition() {
        let result = run("5 3 +").unwrap();
        assert_eq!(result, "$5 + 3$");
    }

    #[test]
    fn test_run_simple_subtraction() {
        let result = run("5 3 -").unwrap();
        assert_eq!(result, "$5 - 3$");
    }

    #[test]
    fn test_run_simple_multiplication() {
        let result = run("4 7 *").unwrap();
        assert_eq!(result, "$4 \\times 7$");
    }

    #[test]
    fn test_run_simple_division() {
        let result = run("10 2 /").unwrap();
        assert_eq!(result, "$10 \\div 2$");
    }

    #[test]
    fn test_run_with_precedence() {
        let result = run("5 3 + 2 *").unwrap();
        assert_eq!(result, "$( 5 + 3 ) \\times 2$");
    }

    #[test]
    fn test_run_no_parens_needed() {
        let result = run("5 3 * 2 +").unwrap();
        assert_eq!(result, "$5 \\times 3 + 2$");
    }

    #[test]
    fn test_run_division_multiplication_chain() {
        let result = run("10 2 / 5 *").unwrap();
        assert_eq!(result, "$10 \\div 2 \\times 5$");
    }

    #[test]
    fn test_run_subtraction_chain() {
        let result = run("5 3 - 2 -").unwrap();
        assert_eq!(result, "$5 - 3 - 2$");
    }

    #[test]
    fn test_run_floating_point() {
        let result = run("3.14 2 *").unwrap();
        assert_eq!(result, "$3.14 \\times 2$");
    }

    #[test]
    fn test_run_floating_point_addition() {
        let result = run("1.5 0.5 +").unwrap();
        assert_eq!(result, "$1.5 + 0.5$");
    }

    #[test]
    fn test_run_complex_precedence() {
        let result = run("10 2 / 3 + 4 *").unwrap();
        assert_eq!(result, "$( 10 \\div 2 + 3 ) \\times 4$");
    }

    #[test]
    fn test_run_both_operands_need_parens() {
        let result = run("1 2 + 3 4 + *").unwrap();
        assert_eq!(result, "$( 1 + 2 ) \\times ( 3 + 4 )$");
    }

    #[test]
    fn test_run_invalid_character_error() {
        let result = run("2 3 ^");
        assert!(result.is_err());
        match result {
            Err(Rpn2TexError::LexerError {
                message,
                line,
                column,
            }) => {
                assert!(message.contains("Unexpected character"));
                assert_eq!(line, 1);
                assert_eq!(column, 5);
            }
            _ => panic!("Expected LexerError"),
        }
    }

    #[test]
    fn test_run_parser_error_insufficient_operands() {
        let result = run("3 +");
        assert!(result.is_err());
        match result {
            Err(Rpn2TexError::ParserError { .. }) => {
                // Expected parser error
            }
            _ => panic!("Expected ParserError"),
        }
    }

    #[test]
    fn test_run_parser_error_too_many_operands() {
        let result = run("5 3 2");
        assert!(result.is_err());
        match result {
            Err(Rpn2TexError::ParserError { .. }) => {
                // Expected parser error
            }
            _ => panic!("Expected ParserError"),
        }
    }

    #[test]
    fn test_run_empty_expression() {
        let result = run("");
        assert!(result.is_err());
        match result {
            Err(Rpn2TexError::ParserError { .. }) => {
                // Expected parser error
            }
            _ => panic!("Expected ParserError"),
        }
    }

    #[test]
    fn test_error_display_format_lexer() {
        let err = Rpn2TexError::lexer_error("Unexpected character '^'", 1, 5);
        let display = format!("{err}");
        assert_eq!(
            display,
            "[LexerError] at line 1, column 5: Unexpected character '^'"
        );
    }

    #[test]
    fn test_error_display_format_parser() {
        let err = Rpn2TexError::parser_error("Not enough operands", 1, 3);
        let display = format!("{err}");
        assert_eq!(
            display,
            "[ParserError] at line 1, column 3: Not enough operands"
        );
    }
}
