//! Command-line interface for rpn2tex.
//!
//! This binary orchestrates the complete pipeline:
//! 1. Read RPN expression from command-line arguments or prompt user
//! 2. Tokenize with Lexer
//! 3. Parse with Parser
//! 4. Generate LaTeX with LatexGenerator
//! 5. Print result or error
//!
//! # Usage
//!
//! ```bash
//! # With argument
//! cargo run -- "5 3 +"
//!
//! # Without argument (prompts for input)
//! cargo run
//!
//! # With help
//! cargo run -- --help
//! ```
//!
//! # Exit Codes
//!
//! - 0: Success
//! - 1: Error (lexer, parser, or other)

use rpn2tex::latex::LatexGenerator;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;
use std::env;
use std::io::{self, Write};
use std::process;

/// Main entry point for rpn2tex CLI.
///
/// Orchestrates the complete pipeline: tokenize -> parse -> generate -> output.
///
/// # Returns
///
/// Exit code: 0 for success, 1 for error
fn main() {
    let exit_code = run();
    process::exit(exit_code);
}

/// Run the rpn2tex pipeline.
///
/// # Returns
///
/// Exit code: 0 for success, 1 for error
fn run() -> i32 {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    // Get input expression
    let expression = match get_input(&args) {
        Ok(expr) => expr,
        Err(err) => {
            eprintln!("{}", err);
            return 1;
        }
    };

    // Process the expression
    match process_expression(&expression) {
        Ok(latex) => {
            println!("{}", latex);
            0
        }
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    }
}

/// Get input expression from command-line args or prompt user.
///
/// # Arguments
///
/// * `args` - Command-line arguments (args[0] is program name)
///
/// # Returns
///
/// * `Ok(String)` - The input expression
/// * `Err(String)` - Error message if input cannot be obtained
fn get_input(args: &[String]) -> Result<String, String> {
    if args.len() > 1 {
        // Check for help flag
        if args[1] == "--help" || args[1] == "-h" {
            print_usage(&args[0]);
            process::exit(0);
        }

        // Use argument as expression
        Ok(args[1].clone())
    } else {
        // Prompt user for input
        prompt_for_input()
    }
}

/// Prompt user for input expression.
///
/// # Returns
///
/// * `Ok(String)` - The input expression
/// * `Err(String)` - Error message if input cannot be read
fn prompt_for_input() -> Result<String, String> {
    print!("Enter RPN expression: ");
    io::stdout()
        .flush()
        .map_err(|e| format!("IO error: {}", e))?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("IO error: {}", e))?;

    Ok(input.trim().to_string())
}

/// Process an RPN expression through the complete pipeline.
///
/// # Arguments
///
/// * `expression` - The RPN expression to process
///
/// # Returns
///
/// * `Ok(String)` - The generated LaTeX string
/// * `Err(String)` - Formatted error message if processing fails
fn process_expression(expression: &str) -> Result<String, String> {
    // Check for empty expression
    if expression.trim().is_empty() {
        return Err("Error: Empty expression".to_string());
    }

    // Tokenize
    let mut lexer = Lexer::new(expression);
    let tokens = lexer.tokenize()?;

    // Parse
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    // Generate LaTeX
    let generator = LatexGenerator::new();
    let latex = generator.generate(&ast);

    Ok(latex)
}

/// Print usage information.
///
/// # Arguments
///
/// * `program_name` - The name of the program (args[0])
fn print_usage(program_name: &str) {
    println!("rpn2tex - Convert Reverse Polish Notation to LaTeX");
    println!();
    println!("USAGE:");
    println!("    {} [EXPRESSION]", program_name);
    println!();
    println!("ARGUMENTS:");
    println!("    <EXPRESSION>    RPN expression to convert (e.g., \"5 3 +\")");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help      Print this help message");
    println!();
    println!("EXAMPLES:");
    println!("    {} \"5 3 +\"         # Outputs: $5 + 3$", program_name);
    println!(
        "    {} \"2 3 4 * +\"     # Outputs: $2 + 3 \\times 4$",
        program_name
    );
    println!();
    println!("If no expression is provided, you will be prompted to enter one.");
    println!();
    println!("EXIT CODES:");
    println!("    0    Success");
    println!("    1    Error (invalid input or processing failure)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_expression_simple_addition() {
        let result = process_expression("5 3 +");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$5 + 3$");
    }

    #[test]
    fn test_process_expression_simple_subtraction() {
        let result = process_expression("5 3 -");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$5 - 3$");
    }

    #[test]
    fn test_process_expression_simple_multiplication() {
        let result = process_expression("4 7 *");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$4 \\times 7$");
    }

    #[test]
    fn test_process_expression_simple_division() {
        let result = process_expression("10 2 /");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$10 \\div 2$");
    }

    #[test]
    fn test_process_expression_with_precedence() {
        let result = process_expression("5 3 + 2 *");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$( 5 + 3 ) \\times 2$");
    }

    #[test]
    fn test_process_expression_multiplication_then_addition() {
        let result = process_expression("5 3 * 2 +");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$5 \\times 3 + 2$");
    }

    #[test]
    fn test_process_expression_left_associative_division() {
        let result = process_expression("10 2 / 5 *");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$10 \\div 2 \\times 5$");
    }

    #[test]
    fn test_process_expression_left_associative_subtraction() {
        let result = process_expression("5 3 - 2 -");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$5 - 3 - 2$");
    }

    #[test]
    fn test_process_expression_chained_division() {
        let result = process_expression("100 10 / 5 / 2 /");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$100 \\div 10 \\div 5 \\div 2$");
    }

    #[test]
    fn test_process_expression_chained_addition() {
        let result = process_expression("1 2 + 3 + 4 +");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_process_expression_precedence_addition_after_mult() {
        let result = process_expression("2 3 4 * +");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$2 + 3 \\times 4$");
    }

    #[test]
    fn test_process_expression_explicit_grouping_via_rpn() {
        let result = process_expression("2 3 + 4 *");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$( 2 + 3 ) \\times 4$");
    }

    #[test]
    fn test_process_expression_grouping_on_right_operand() {
        let result = process_expression("2 3 4 + *");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$2 \\times ( 3 + 4 )$");
    }

    #[test]
    fn test_process_expression_mult_then_addition() {
        let result = process_expression("2 3 * 4 +");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$2 \\times 3 + 4$");
    }

    #[test]
    fn test_process_expression_floating_point_multiplication() {
        let result = process_expression("3.14 2 *");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$3.14 \\times 2$");
    }

    #[test]
    fn test_process_expression_floating_point_addition() {
        let result = process_expression("1.5 0.5 +");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$1.5 + 0.5$");
    }

    #[test]
    fn test_process_expression_multiple_subexpressions() {
        let result = process_expression("1 2 + 3 4 + *");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$( 1 + 2 ) \\times ( 3 + 4 )$");
    }

    #[test]
    fn test_process_expression_complex_expression() {
        let result = process_expression("10 2 / 3 + 4 *");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$( 10 \\div 2 + 3 ) \\times 4$");
    }

    #[test]
    fn test_process_expression_single_number() {
        let result = process_expression("5");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$5$");
    }

    #[test]
    fn test_process_expression_empty_string() {
        let result = process_expression("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Error: Empty expression");
    }

    #[test]
    fn test_process_expression_whitespace_only() {
        let result = process_expression("   ");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Error: Empty expression");
    }

    #[test]
    fn test_process_expression_missing_operator() {
        let result = process_expression("5 3");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Invalid RPN: 2 values remain on stack"));
    }

    #[test]
    fn test_process_expression_insufficient_operands() {
        let result = process_expression("5 3 + +");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Operator '+' requires two operands"));
    }

    #[test]
    fn test_process_expression_unsupported_exponentiation() {
        let result = process_expression("2 3 ^");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character '^'"));
    }

    #[test]
    fn test_process_expression_unsupported_in_expression() {
        let result = process_expression("2 3 ^ 4 *");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character '^'"));
    }

    #[test]
    fn test_process_expression_multiple_unsupported() {
        let result = process_expression("2 3 4 ^ ^");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character '^'"));
    }

    #[test]
    fn test_process_expression_invalid_character() {
        let result = process_expression("invalid");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character 'i'"));
    }

    #[test]
    fn test_process_expression_invalid_at_symbol() {
        let result = process_expression("5 @ 3");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected character '@'"));
    }

    #[test]
    fn test_get_input_with_argument() {
        let args = vec!["program".to_string(), "5 3 +".to_string()];
        let result = get_input(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "5 3 +");
    }

    #[test]
    fn test_get_input_multiple_arguments_uses_first() {
        // In typical CLI usage, all args after program name would be parsed
        // But our simple implementation just takes args[1]
        let args = vec![
            "program".to_string(),
            "5 3 +".to_string(),
            "extra".to_string(),
        ];
        let result = get_input(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "5 3 +");
    }

    #[test]
    fn test_process_expression_negative_numbers() {
        let result = process_expression("-5 3 +");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$-5 + 3$");
    }

    #[test]
    fn test_process_expression_with_newlines() {
        let result = process_expression("5\n3\n+");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$5 + 3$");
    }

    #[test]
    fn test_process_expression_with_tabs() {
        let result = process_expression("5\t3\t+");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$5 + 3$");
    }

    #[test]
    fn test_process_expression_right_associative_subtraction() {
        // This tests 5 - (3 - 2) which requires parentheses
        let result = process_expression("5 3 2 - -");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$5 - ( 3 - 2 )$");
    }

    #[test]
    fn test_process_expression_right_associative_division() {
        // This tests 10 / (5 / 2) which requires parentheses
        let result = process_expression("10 5 2 / /");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "$10 \\div ( 5 \\div 2 )$");
    }
}
