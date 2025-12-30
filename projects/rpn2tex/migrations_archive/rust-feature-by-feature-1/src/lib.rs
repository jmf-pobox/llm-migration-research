//! rpn2tex - Convert RPN expressions to LaTeX math mode.
//!
//! This library provides functionality to parse Reverse Polish Notation (RPN)
//! mathematical expressions and convert them to LaTeX format.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::lexer::Lexer;
//! use rpn2tex::parser::Parser;
//! use rpn2tex::latex::LaTeXGenerator;
//!
//! let mut lexer = Lexer::new("5");
//! let tokens = lexer.tokenize().unwrap();
//! let ast = Parser::new(tokens).parse().unwrap();
//! let latex = LaTeXGenerator::new().generate(&ast);
//! assert_eq!(latex, "$5$");
//! ```

pub mod ast;
pub mod error;
pub mod latex;
pub mod lexer;
pub mod parser;
pub mod tokens;
