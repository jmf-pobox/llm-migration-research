use rpn2tex::error::ErrorFormatter;

fn main() {
    let source = "abc".to_string();
    let formatter = ErrorFormatter::new(source);
    let context = formatter.get_context(1, 1, 1);
    println!("Context output:");
    for (i, line) in context.lines().enumerate() {
        println!("Line {}: '{}'", i, line);
    }

    let source2 = "2 3 ^".to_string();
    let formatter2 = ErrorFormatter::new(source2);
    let context2 = formatter2.get_context(1, 5, 1);
    println!("\nContext2 output:");
    for (i, line) in context2.lines().enumerate() {
        println!("Line {}: '{}'", i, line);
    }
}
