//! rpn2tex - Convert Reverse Polish Notation expressions to LaTeX
//!
//! This library provides functionality to parse RPN expressions and convert
//! them to properly formatted LaTeX output with correct operator precedence
//! and parenthesization.
//!
//! # Example
//!
//! ```
//! use rpn2tex::lexer::Lexer;
//! use rpn2tex::parser::Parser;
//! use rpn2tex::latex::LaTeXGenerator;
//!
//! let input = "5 3 +";
//! let tokens = Lexer::new(input).tokenize().unwrap();
//! let ast = Parser::new(tokens).parse().unwrap();
//! let latex = LaTeXGenerator.generate(&ast);
//! assert_eq!(latex, "$5 + 3$");
//! ```

pub mod ast;
pub mod error;
pub mod latex;
pub mod lexer;
pub mod parser;
pub mod tokens;
