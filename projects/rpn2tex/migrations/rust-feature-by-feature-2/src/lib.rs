//! rpn2tex - A compiler that converts Reverse Polish Notation to LaTeX.
//!
//! This library provides functionality to parse RPN expressions and convert them
//! to LaTeX format for mathematical typesetting.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::process_input;
//!
//! let result = process_input("5");
//! assert_eq!(result.unwrap(), "$5$");
//! ```

pub mod ast;
pub mod error;
pub mod latex;
pub mod lexer;
pub mod parser;
pub mod tokens;

use latex::LatexGenerator;
use lexer::Lexer;
use parser::Parser;

/// Processes an RPN input string and returns LaTeX output.
///
/// # Examples
///
/// ```
/// use rpn2tex::process_input;
///
/// let result = process_input("5");
/// assert_eq!(result.unwrap(), "$5$");
/// ```
///
/// # Errors
///
/// Returns an error string if lexing, parsing, or generation fails.
pub fn process_input(input: &str) -> Result<String, String> {
    // Lex
    let mut lexer = Lexer::new(input);
    let tokens = lexer.scan_tokens().map_err(|e| e.to_string())?;

    // Parse
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| e.to_string())?;

    // Generate
    let generator = LatexGenerator::new();
    Ok(generator.generate(&ast))
}
