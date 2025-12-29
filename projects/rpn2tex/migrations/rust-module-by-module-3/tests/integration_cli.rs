//! Integration tests for the CLI interface.
//!
//! These tests verify the complete end-to-end behavior including:
//! - Stdin input processing
//! - File input processing
//! - File output
//! - Error handling and reporting
//! - Exit codes

use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::TempDir;

/// Helper to get the path to the compiled binary
fn binary_path() -> String {
    // The binary will be in target/debug/ or target/release/
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove test binary name
    path.pop(); // Remove deps/
    path.push("rpn2tex");
    path.to_str().unwrap().to_string()
}

#[test]
fn test_stdin_simple_addition() {
    let output = run_with_stdin(&["-"], b"5 3 +");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "$5 + 3$");
    assert!(output.stderr.is_empty());
}

#[test]
fn test_stdin_with_parentheses() {
    let output = run_with_stdin(&["-"], b"5 3 + 2 *");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "$( 5 + 3 ) \\times 2$"
    );
}

#[test]
fn test_stdin_decimal() {
    let output = run_with_stdin(&["-"], b"3.14 2 *");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "$3.14 \\times 2$");
}

#[test]
fn test_stdin_error_unsupported_operator() {
    let output = run_with_stdin(&["-"], b"2 3 ^");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error: Unexpected character '^'"));
    assert!(stderr.contains("1 | 2 3 ^"));
    assert!(stderr.contains("^"));
}

#[test]
fn test_stdin_error_incomplete_expression() {
    let output = run_with_stdin(&["-"], b"5 3");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error:"));
}

#[test]
fn test_file_input_to_stdout() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.rpn");

    fs::write(&input_path, "5 3 +").unwrap();

    let output = Command::new(binary_path())
        .arg(input_path.to_str().unwrap())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "$5 + 3$");
}

#[test]
fn test_file_input_to_file_output() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.rpn");
    let output_path = temp_dir.path().join("output.tex");

    fs::write(&input_path, "5 3 + 2 *").unwrap();

    let output = Command::new(binary_path())
        .arg(input_path.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Generated:"));

    let contents = fs::read_to_string(&output_path).unwrap();
    assert_eq!(contents, "$( 5 + 3 ) \\times 2$\n");
}

#[test]
fn test_file_not_found() {
    let output = Command::new(binary_path())
        .arg("/nonexistent/file.rpn")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error: Input file not found"));
}

#[test]
fn test_directory_as_input() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg(temp_dir.path().to_str().unwrap())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error: Expected a file, got a directory"));
}

#[test]
fn test_complex_expression() {
    let output = run_with_stdin(&["-"], b"10 2 / 3 + 4 *");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "$( 10 \\div 2 + 3 ) \\times 4$"
    );
}

#[test]
fn test_all_operators() {
    let inputs = [
        ("5 3 +", "$5 + 3$"),
        ("5 3 -", "$5 - 3$"),
        ("5 3 *", "$5 \\times 3$"),
        ("10 2 /", "$10 \\div 2$"),
    ];

    for (input, expected) in &inputs {
        let output = run_with_stdin(&["-"], input.as_bytes());

        assert!(output.status.success(), "Failed for input: {input}");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            *expected,
            "Wrong output for input: {input}"
        );
    }
}

/// Helper function to run command with stdin
fn run_with_stdin(args: &[&str], input: &[u8]) -> std::process::Output {
    use std::process::Stdio;

    let mut cmd = Command::new(binary_path());
    for arg in args {
        cmd.arg(arg);
    }

    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input).expect("Failed to write to stdin");
    }

    child
        .wait_with_output()
        .expect("Failed to wait for command")
}
