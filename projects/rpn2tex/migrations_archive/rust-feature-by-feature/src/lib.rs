//! RPN to LaTeX converter library.
//!
//! This library converts Reverse Polish Notation (RPN) expressions into LaTeX math mode.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::lexer::Lexer;
//! use rpn2tex::parser::Parser;
//! use rpn2tex::latex::LaTeXGenerator;
//!
//! let lexer = Lexer::new("5 3 +");
//! let tokens = lexer.tokenize().unwrap();
//! let ast = Parser::new(tokens).parse().unwrap();
//! let latex = LaTeXGenerator::new().generate(&ast);
//! assert_eq!(latex, "$5 + 3$");
//! ```

pub mod ast;
pub mod error;
pub mod latex;
pub mod lexer;
pub mod parser;
pub mod tokens;
