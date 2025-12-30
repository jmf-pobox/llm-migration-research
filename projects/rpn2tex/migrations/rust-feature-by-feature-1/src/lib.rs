//! RPN to LaTeX converter library.
//!
//! This library provides functionality to convert Reverse Polish Notation (RPN)
//! expressions into LaTeX mathematical notation.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```
//! use rpn2tex::{Lexer, Parser, LaTeXGenerator};
//!
//! let input = "5";
//!
//! // Tokenize
//! let lexer = Lexer::new(input);
//! let tokens = lexer.tokenize().unwrap();
//!
//! // Parse
//! let parser = Parser::new(tokens);
//! let ast = parser.parse().unwrap();
//!
//! // Generate LaTeX
//! let generator = LaTeXGenerator::new();
//! let latex = generator.generate(&ast);
//!
//! assert_eq!(latex, "$5$");
//! ```

mod ast;
mod error;
mod latex;
mod lexer;
mod parser;
mod tokens;

// Re-export public API
pub use ast::{BinaryOp, Expr, Number};
pub use error::{LexerError, ParserError};
pub use latex::LaTeXGenerator;
pub use lexer::Lexer;
pub use parser::Parser;
pub use tokens::{Token, TokenType};

/// Converts an RPN expression to LaTeX notation.
///
/// This is a convenience function that runs the entire conversion pipeline.
///
/// # Errors
///
/// Returns an error if lexing or parsing fails.
///
/// # Examples
///
/// ```
/// use rpn2tex::convert;
///
/// let latex = convert("5").unwrap();
/// assert_eq!(latex, "$5$");
///
/// let latex = convert("3.14").unwrap();
/// assert_eq!(latex, "$3.14$");
/// ```
pub fn convert(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;

    let parser = Parser::new(tokens);
    let ast = parser.parse()?;

    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_single_digit() {
        let result = convert("5").unwrap();
        assert_eq!(result, "$5$");
    }

    #[test]
    fn test_convert_decimal() {
        let result = convert("3.14").unwrap();
        assert_eq!(result, "$3.14$");
    }

    #[test]
    fn test_convert_negative() {
        let result = convert("-5").unwrap();
        assert_eq!(result, "$-5$");
    }

    #[test]
    fn test_convert_multi_digit() {
        let result = convert("12345").unwrap();
        assert_eq!(result, "$12345$");
    }

    #[test]
    fn test_convert_invalid_input() {
        let result = convert("@");
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_empty() {
        let result = convert("");
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_multiple_numbers() {
        let result = convert("5 3");
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_addition() {
        let result = convert("5 3 +").unwrap();
        assert_eq!(result, "$5 + 3$");
    }

    #[test]
    fn test_convert_chained_addition() {
        let result = convert("1 2 + 3 + 4 +").unwrap();
        assert_eq!(result, "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_convert_addition_with_floats() {
        let result = convert("1.5 0.5 +").unwrap();
        assert_eq!(result, "$1.5 + 0.5$");
    }

    #[test]
    fn test_convert_addition_missing_operand() {
        let result = convert("5 +");
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_addition_extra_operand() {
        let result = convert("5 3 2 +");
        assert!(result.is_err());
    }

    // Subtraction tests (Feature 3)
    #[test]
    fn test_convert_subtraction() {
        let result = convert("5 3 -").unwrap();
        assert_eq!(result, "$5 - 3$");
    }

    #[test]
    fn test_convert_chained_subtraction() {
        let result = convert("5 3 - 2 -").unwrap();
        assert_eq!(result, "$5 - 3 - 2$");
    }

    #[test]
    fn test_convert_subtraction_with_floats() {
        let result = convert("5.5 2.3 -").unwrap();
        assert_eq!(result, "$5.5 - 2.3$");
    }

    #[test]
    fn test_convert_subtraction_missing_operand() {
        let result = convert("5 -");
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_subtraction_extra_operand() {
        let result = convert("5 3 2 -");
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_negative_number_vs_operator() {
        // Negative number
        let result = convert("-5").unwrap();
        assert_eq!(result, "$-5$");

        // Negative number in expression
        let result = convert("10 -5 +").unwrap();
        assert_eq!(result, "$10 + -5$");
    }
}
