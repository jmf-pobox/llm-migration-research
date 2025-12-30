//! rpn2tex - Convert RPN expressions to LaTeX math mode.
//!
//! This library provides functionality to parse Reverse Polish Notation (RPN)
//! expressions and convert them to LaTeX format.

pub mod ast;
pub mod error;
pub mod latex;
pub mod lexer;
pub mod parser;
pub mod tokens;

use error::Error;
use latex::LaTeXGenerator;
use lexer::Lexer;
use parser::Parser;

/// Convert an RPN expression to LaTeX.
///
/// # Errors
///
/// Returns an error if the input is invalid.
///
/// # Examples
///
/// ```
/// use rpn2tex::convert;
///
/// let latex = convert("5").unwrap();
/// assert_eq!(latex, "$5$");
/// ```
pub fn convert(input: &str) -> Result<String, Error> {
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}
