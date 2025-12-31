//! Command-line interface for rpn2tex.
//!
//! This binary provides a CLI for converting RPN expressions to LaTeX.

use rpn2tex::process_input;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: rpn2tex <expression>");
        eprintln!("Example: rpn2tex \"5 3 +\"");
        process::exit(1);
    }

    let input = &args[1];

    match process_input(input) {
        Ok(latex) => println!("{}", latex),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use rpn2tex::process_input;

    #[test]
    fn test_process_integer() {
        let result = process_input("5");
        assert_eq!(result.unwrap(), "$5$");
    }

    #[test]
    fn test_process_float() {
        let result = process_input("3.14");
        assert_eq!(result.unwrap(), "$3.14$");
    }

    #[test]
    fn test_io_contract_5() {
        let result = process_input("5");
        assert_eq!(result.unwrap(), "$5$");
    }

    #[test]
    fn test_io_contract_3_14() {
        let result = process_input("3.14");
        assert_eq!(result.unwrap(), "$3.14$");
    }

    // Feature 2: Addition tests
    #[test]
    fn test_io_contract_addition_5_3() {
        let result = process_input("5 3 +");
        assert_eq!(result.unwrap(), "$5 + 3$");
    }

    #[test]
    fn test_io_contract_chained_addition() {
        let result = process_input("1 2 + 3 + 4 +");
        assert_eq!(result.unwrap(), "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_insufficient_operands_error() {
        let result = process_input("5 +");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires two operands"));
    }

    // Feature 4: Multiplication tests
    #[test]
    fn test_io_contract_multiplication_4_7() {
        let result = process_input("4 7 *");
        assert_eq!(result.unwrap(), r"$4 \times 7$");
    }

    #[test]
    fn test_io_contract_multiplication_mixed() {
        let result = process_input("2 3 4 * +");
        assert_eq!(result.unwrap(), r"$2 + 3 \times 4$");
    }

    #[test]
    fn test_io_contract_multiplication_float() {
        let result = process_input("3.14 2 *");
        assert_eq!(result.unwrap(), r"$3.14 \times 2$");
    }

    #[test]
    fn test_multiplication_insufficient_operands() {
        let result = process_input("5 *");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires two operands"));
    }

    // Feature 5: Division tests
    #[test]
    fn test_io_contract_division_10_2() {
        let result = process_input("10 2 /");
        assert_eq!(result.unwrap(), r"$10 \div 2$");
    }

    #[test]
    fn test_io_contract_chained_division() {
        let result = process_input("100 10 / 5 / 2 /");
        assert_eq!(result.unwrap(), r"$100 \div 10 \div 5 \div 2$");
    }

    #[test]
    fn test_io_contract_division_multiplication() {
        let result = process_input("10 2 / 5 *");
        assert_eq!(result.unwrap(), r"$10 \div 2 \times 5$");
    }

    #[test]
    fn test_division_insufficient_operands() {
        let result = process_input("5 /");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires two operands"));
    }
}
