package main

import (
	"bytes"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// TestCLIEndToEnd tests the complete CLI pipeline with all I/O contract test cases.
func TestCLIEndToEnd(t *testing.T) {
	// Build the executable first
	buildDir := t.TempDir()
	exePath := filepath.Join(buildDir, "rpn2tex")

	// Get current working directory (should be cmd/rpn2tex when running go test)
	wd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}

	cmd := exec.Command("go", "build", "-o", exePath, ".")
	cmd.Dir = wd
	output, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("Failed to build executable: %v\nOutput: %s", err, output)
	}

	tests := []struct {
		name        string
		input       string
		want        string
		wantErr     bool
		errContains string
	}{
		// Success cases (18 passing tests)
		{
			name:  "Test 1: Basic addition",
			input: "5 3 +",
			want:  "$5 + 3$\n",
		},
		{
			name:  "Test 2: Subtraction",
			input: "5 3 -",
			want:  "$5 - 3$\n",
		},
		{
			name:  "Test 3: Multiplication",
			input: "4 7 *",
			want:  "$4 \\times 7$\n",
		},
		{
			name:  "Test 4: Division",
			input: "10 2 /",
			want:  "$10 \\div 2$\n",
		},
		{
			name:  "Test 6: Operator precedence (addition + multiplication)",
			input: "5 3 + 2 *",
			want:  "$( 5 + 3 ) \\times 2$\n",
		},
		{
			name:  "Test 7: Operator precedence (multiplication + addition)",
			input: "5 3 * 2 +",
			want:  "$5 \\times 3 + 2$\n",
		},
		{
			name:  "Test 8: Left-to-right division and multiplication",
			input: "10 2 / 5 *",
			want:  "$10 \\div 2 \\times 5$\n",
		},
		{
			name:  "Test 9: Left-associative subtraction",
			input: "5 3 - 2 -",
			want:  "$5 - 3 - 2$\n",
		},
		{
			name:  "Test 10: Multiple divisions",
			input: "100 10 / 5 / 2 /",
			want:  "$100 \\div 10 \\div 5 \\div 2$\n",
		},
		{
			name:  "Test 11: Multiple additions",
			input: "1 2 + 3 + 4 +",
			want:  "$1 + 2 + 3 + 4$\n",
		},
		{
			name:  "Test 12: Operator precedence (addition inside multiplication)",
			input: "2 3 4 * +",
			want:  "$2 + 3 \\times 4$\n",
		},
		{
			name:  "Test 13: Parentheses for lower precedence left operand",
			input: "2 3 + 4 *",
			want:  "$( 2 + 3 ) \\times 4$\n",
		},
		{
			name:  "Test 14: Parentheses for lower precedence right operand",
			input: "2 3 4 + *",
			want:  "$2 \\times ( 3 + 4 )$\n",
		},
		{
			name:  "Test 15: Mixed operations",
			input: "2 3 * 4 +",
			want:  "$2 \\times 3 + 4$\n",
		},
		{
			name:  "Test 18: Decimal number multiplication",
			input: "3.14 2 *",
			want:  "$3.14 \\times 2$\n",
		},
		{
			name:  "Test 19: Decimal number addition",
			input: "1.5 0.5 +",
			want:  "$1.5 + 0.5$\n",
		},
		{
			name:  "Test 20: Two additions multiplied",
			input: "1 2 + 3 4 + *",
			want:  "$( 1 + 2 ) \\times ( 3 + 4 )$\n",
		},
		{
			name:  "Test 21: Complex expression",
			input: "10 2 / 3 + 4 *",
			want:  "$( 10 \\div 2 + 3 ) \\times 4$\n",
		},
		// Error cases (3 failing tests)
		{
			name:        "Test 5: Exponentiation operator (not supported)",
			input:       "2 3 ^",
			wantErr:     true,
			errContains: "Unexpected character '^'",
		},
		{
			name:        "Test 16: Exponentiation in expression (not supported)",
			input:       "2 3 ^ 4 *",
			wantErr:     true,
			errContains: "Unexpected character '^'",
		},
		{
			name:        "Test 17: Multiple exponentiation (not supported)",
			input:       "2 3 4 ^ ^",
			wantErr:     true,
			errContains: "Unexpected character '^'",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Create a command to run the executable
			cmd := exec.Command(exePath, "-")

			// Set up stdin with the input
			cmd.Stdin = strings.NewReader(tt.input)

			// Capture stdout and stderr
			var stdout, stderr bytes.Buffer
			cmd.Stdout = &stdout
			cmd.Stderr = &stderr

			// Run the command
			err := cmd.Run()

			if tt.wantErr {
				// Expecting an error
				if err == nil {
					t.Errorf("Expected error but got none")
					return
				}

				// Check that error message contains expected text
				stderrStr := stderr.String()
				if !strings.Contains(stderrStr, tt.errContains) {
					t.Errorf("Error output does not contain %q\nGot stderr: %s", tt.errContains, stderrStr)
				}

				// Check for proper error context formatting
				if !strings.Contains(stderrStr, "Error:") {
					t.Errorf("Error output missing 'Error:' prefix\nGot: %s", stderrStr)
				}

				// Check that caret is present in error output
				if !strings.Contains(stderrStr, "^") {
					t.Errorf("Error output missing caret (^) pointer\nGot: %s", stderrStr)
				}
			} else {
				// Expecting success
				if err != nil {
					t.Errorf("Unexpected error: %v\nStderr: %s", err, stderr.String())
					return
				}

				// Check output matches expected
				got := stdout.String()
				if got != tt.want {
					t.Errorf("Output mismatch\nGot:  %q\nWant: %q", got, tt.want)
				}

				// Ensure nothing was written to stderr (except maybe empty)
				if stderr.Len() > 0 {
					t.Errorf("Unexpected stderr output: %s", stderr.String())
				}
			}
		})
	}
}

// TestCLIFileIO tests file input and output functionality.
func TestCLIFileIO(t *testing.T) {
	// Build the executable
	buildDir := t.TempDir()
	exePath := filepath.Join(buildDir, "rpn2tex")

	wd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}

	cmd := exec.Command("go", "build", "-o", exePath, ".")
	cmd.Dir = wd
	output, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("Failed to build executable: %v\nOutput: %s", err, output)
	}

	t.Run("Read from file", func(t *testing.T) {
		// Create input file
		inputFile := filepath.Join(t.TempDir(), "input.rpn")
		err := os.WriteFile(inputFile, []byte("5 3 +"), 0644)
		if err != nil {
			t.Fatalf("Failed to create input file: %v", err)
		}

		// Run command with file input
		cmd := exec.Command(exePath, inputFile)
		var stdout bytes.Buffer
		cmd.Stdout = &stdout

		err = cmd.Run()
		if err != nil {
			t.Fatalf("Command failed: %v", err)
		}

		got := stdout.String()
		want := "$5 + 3$\n"
		if got != want {
			t.Errorf("Output mismatch\nGot:  %q\nWant: %q", got, want)
		}
	})

	t.Run("Write to file", func(t *testing.T) {
		outputFile := filepath.Join(t.TempDir(), "output.tex")

		// Run command with file output
		cmd := exec.Command(exePath, "-o", outputFile, "-")
		cmd.Stdin = strings.NewReader("10 2 /")

		var stderr bytes.Buffer
		cmd.Stderr = &stderr

		err := cmd.Run()
		if err != nil {
			t.Fatalf("Command failed: %v\nStderr: %s", err, stderr.String())
		}

		// Check that file was created
		content, err := os.ReadFile(outputFile)
		if err != nil {
			t.Fatalf("Failed to read output file: %v", err)
		}

		got := string(content)
		want := "$10 \\div 2$\n"
		if got != want {
			t.Errorf("Output file content mismatch\nGot:  %q\nWant: %q", got, want)
		}

		// Check stderr message
		stderrStr := stderr.String()
		if !strings.Contains(stderrStr, "Generated:") {
			t.Errorf("Expected 'Generated:' message in stderr, got: %s", stderrStr)
		}
	})

	t.Run("File not found error", func(t *testing.T) {
		cmd := exec.Command(exePath, "/nonexistent/file.rpn")
		var stderr bytes.Buffer
		cmd.Stderr = &stderr

		err := cmd.Run()
		if err == nil {
			t.Error("Expected error for nonexistent file")
		}

		stderrStr := stderr.String()
		if !strings.Contains(stderrStr, "not found") && !strings.Contains(stderrStr, "no such file") {
			t.Errorf("Expected 'not found' error, got: %s", stderrStr)
		}
	})
}

// TestCLIUsage tests the usage/help output.
func TestCLIUsage(t *testing.T) {
	// Build the executable
	buildDir := t.TempDir()
	exePath := filepath.Join(buildDir, "rpn2tex")

	wd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}

	cmd := exec.Command("go", "build", "-o", exePath, ".")
	cmd.Dir = wd
	output, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("Failed to build executable: %v\nOutput: %s", err, output)
	}

	t.Run("No arguments shows usage", func(t *testing.T) {
		cmd := exec.Command(exePath)
		var stderr bytes.Buffer
		cmd.Stderr = &stderr

		err := cmd.Run()
		if err == nil {
			t.Error("Expected error when no arguments provided")
		}

		stderrStr := stderr.String()
		if !strings.Contains(stderrStr, "Usage:") {
			t.Errorf("Expected usage message, got: %s", stderrStr)
		}
	})
}

// TestCLIStdin tests reading from stdin.
func TestCLIStdin(t *testing.T) {
	// Build the executable
	buildDir := t.TempDir()
	exePath := filepath.Join(buildDir, "rpn2tex")

	wd, err := os.Getwd()
	if err != nil {
		t.Fatalf("Failed to get working directory: %v", err)
	}

	cmd := exec.Command("go", "build", "-o", exePath, ".")
	cmd.Dir = wd
	output, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("Failed to build executable: %v\nOutput: %s", err, output)
	}

	t.Run("Read from stdin with - argument", func(t *testing.T) {
		cmd := exec.Command(exePath, "-")
		cmd.Stdin = strings.NewReader("3.14 2 *")

		var stdout bytes.Buffer
		cmd.Stdout = &stdout

		err := cmd.Run()
		if err != nil {
			t.Fatalf("Command failed: %v", err)
		}

		got := stdout.String()
		want := "$3.14 \\times 2$\n"
		if got != want {
			t.Errorf("Output mismatch\nGot:  %q\nWant: %q", got, want)
		}
	})
}

// Helper function to read file content (unused but kept for reference)
func readFile(t *testing.T, path string) string {
	t.Helper()
	content, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("Failed to read file %s: %v", path, err)
	}
	return string(content)
}

// Helper function to write file content (unused but kept for reference)
func writeFile(t *testing.T, path string, content string) {
	t.Helper()
	err := os.WriteFile(path, []byte(content), 0644)
	if err != nil {
		t.Fatalf("Failed to write file %s: %v", path, err)
	}
}

// captureOutput captures stdout and stderr while running a function.
func captureOutput(t *testing.T, f func()) (stdout, stderr string) {
	t.Helper()

	// Save original stdout/stderr
	oldStdout := os.Stdout
	oldStderr := os.Stderr

	// Create pipes
	rOut, wOut, _ := os.Pipe()
	rErr, wErr, _ := os.Pipe()

	os.Stdout = wOut
	os.Stderr = wErr

	// Run function
	f()

	// Restore stdout/stderr
	wOut.Close()
	wErr.Close()
	os.Stdout = oldStdout
	os.Stderr = oldStderr

	// Read captured output
	outBytes, _ := io.ReadAll(rOut)
	errBytes, _ := io.ReadAll(rErr)

	return string(outBytes), string(errBytes)
}
