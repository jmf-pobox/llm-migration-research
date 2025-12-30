//! RPN to LaTeX converter library.
//!
//! This library provides functionality to convert Reverse Polish Notation (RPN)
//! expressions into LaTeX mathematical notation.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::{convert, lexer::Lexer, parser::Parser, latex::Generator};
//!
//! let result = convert("42").unwrap();
//! assert_eq!(result, "$42$");
//! ```

pub mod ast;
pub mod error;
pub mod latex;
pub mod lexer;
pub mod parser;
pub mod tokens;

use error::Result;

/// Converts an RPN expression to LaTeX.
///
/// This is the main entry point for the library.
///
/// # Arguments
///
/// * `input` - The RPN expression as a string
///
/// # Examples
///
/// ```
/// use rpn2tex::convert;
///
/// let latex = convert("42").unwrap();
/// assert_eq!(latex, "$42$");
///
/// let latex = convert("3.14").unwrap();
/// assert_eq!(latex, "$3.14$");
/// ```
///
/// # Errors
///
/// Returns an error if lexing, parsing, or generation fails.
#[must_use = "the converted LaTeX string should be used"]
pub fn convert(input: &str) -> Result<String> {
    // Lex
    let lexer = lexer::Lexer::new(input);
    let tokens = lexer.scan_tokens()?;

    // Parse
    let parser = parser::Parser::new(tokens);
    let ast = parser.parse()?;

    // Generate
    let generator = latex::Generator::new();
    generator
        .generate(&ast)
        .map_err(|e| error::Error::ParserError {
            message: e,
            line: 1,
            column: 1,
        })
}
