//! RPN to LaTeX Converter
//!
//! A library for converting Reverse Polish Notation (RPN) mathematical
//! expressions to LaTeX format.

pub mod ast;
pub mod error;
pub mod latex;
pub mod lexer;
pub mod parser;
pub mod tokens;

// Re-export commonly used types
pub use ast::{AstNode, Operator};
pub use error::Rpn2TexError;
pub use latex::LatexGenerator;
pub use lexer::Lexer;
pub use parser::Parser;
pub use tokens::{Token, TokenType};
