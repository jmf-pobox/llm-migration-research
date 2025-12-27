//! RPN to LaTeX converter library.
//!
//! This library provides functionality to convert Reverse Polish Notation (RPN)
//! expressions to LaTeX mathematical notation.

pub mod ast;
pub mod error;
pub mod latex;
pub mod lexer;
pub mod parser;
pub mod tokens;

// Re-export public API
pub use ast::{BinaryOp, Expr, Number};
pub use error::ErrorFormatter;
pub use latex::LaTeXGenerator;
pub use lexer::{Lexer, LexerError};
pub use parser::{Parser, ParserError};
pub use tokens::{Token, TokenType};
