//! RPN to LaTeX converter library.
//!
//! This library provides functionality to convert Reverse Polish Notation (RPN)
//! expressions into LaTeX mathematical notation.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::lexer::Lexer;
//! use rpn2tex::parser::Parser;
//! use rpn2tex::latex::LatexGenerator;
//!
//! let mut lexer = Lexer::new("42");
//! let tokens = lexer.scan_tokens().unwrap();
//! let mut parser = Parser::new(tokens);
//! let expr = parser.parse().unwrap();
//! let generator = LatexGenerator::new();
//! let latex = generator.generate(&expr);
//! assert_eq!(latex, "$42$");
//! ```

pub mod ast;
pub mod error;
pub mod latex;
pub mod lexer;
pub mod parser;
pub mod tokens;
