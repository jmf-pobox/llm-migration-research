//! A compiler that converts Reverse Polish Notation (RPN) expressions to LaTeX.
//!
//! This library provides functionality to parse RPN mathematical expressions
//! and generate corresponding LaTeX output.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::{lexer::Lexer, parser::Parser, latex::LaTeXGenerator};
//!
//! let mut lexer = Lexer::new("5");
//! let tokens = lexer.tokenize().unwrap();
//! let mut parser = Parser::new(tokens);
//! let expr = parser.parse().unwrap();
//! let generator = LaTeXGenerator::new();
//! let latex = generator.generate(&expr);
//! assert_eq!(latex, "$5$");
//! ```

pub mod ast;
pub mod error;
pub mod latex;
pub mod lexer;
pub mod parser;
pub mod tokens;

/// Converts an RPN expression to LaTeX.
///
/// This is a convenience function that combines lexing, parsing, and generation.
///
/// # Examples
///
/// ```
/// use rpn2tex::compile;
///
/// let latex = compile("5").unwrap();
/// assert_eq!(latex, "$5$");
///
/// let latex = compile("3.14").unwrap();
/// assert_eq!(latex, "$3.14$");
/// ```
///
/// # Errors
///
/// Returns an error string if the expression cannot be compiled.
pub fn compile(input: &str) -> Result<String, String> {
    let mut lexer = lexer::Lexer::new(input);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Error: {}", e.message))?;

    let mut parser = parser::Parser::new(tokens);
    let expr = parser
        .parse()
        .map_err(|e| format!("Error: {}", e.message))?;

    let generator = latex::LaTeXGenerator::new();
    Ok(generator.generate(&expr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_number() {
        let result = compile("5");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$5$");
    }

    #[test]
    fn test_compile_decimal() {
        let result = compile("3.14");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$3.14$");
    }

    #[test]
    fn test_compile_empty() {
        let result = compile("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Empty expression"));
    }

    #[test]
    fn test_compile_invalid() {
        let result = compile("5 @");
        assert!(result.is_err());
    }

    #[test]
    fn test_compile_simple_addition() {
        let result = compile("5 3 +");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$5 + 3$");
    }

    #[test]
    fn test_compile_chained_addition() {
        let result = compile("1 2 + 3 + 4 +");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_compile_addition_insufficient_operands() {
        let result = compile("5 +");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires two operands"));
    }

    #[test]
    fn test_compile_simple_division() {
        let result = compile("10 2 /");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r"$10 \div 2$");
    }

    #[test]
    fn test_compile_chained_division() {
        let result = compile("100 10 / 5 / 2 /");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r"$100 \div 10 \div 5 \div 2$");
    }

    #[test]
    fn test_compile_division_insufficient_operands() {
        let result = compile("10 /");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires two operands"));
    }
}
