//! Command-line interface for the RPN to LaTeX converter.

use rpn2tex::convert;
use std::env;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 {
        // Use command-line argument
        args[1].clone()
    } else {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer.trim().to_string()
    };

    match convert(&input) {
        Ok(latex) => println!("{latex}"),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
