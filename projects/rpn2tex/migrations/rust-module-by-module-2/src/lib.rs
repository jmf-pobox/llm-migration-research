//! rpn2tex - Convert RPN expressions to LaTeX math mode
//!
//! This library provides functionality to convert Reverse Polish Notation (RPN)
//! mathematical expressions into LaTeX format.

pub mod ast;
pub mod error;
pub mod latex;
pub mod lexer;
pub mod parser;
pub mod tokens;

pub use ast::Expr;
pub use error::ErrorFormatter;
pub use latex::LaTeXGenerator;
pub use lexer::{Lexer, LexerError};
pub use parser::{Parser, ParserError};
pub use tokens::{Token, TokenType};
