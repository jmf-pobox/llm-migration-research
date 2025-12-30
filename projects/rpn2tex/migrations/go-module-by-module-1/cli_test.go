package main

import (
	"os"
	"os/exec"
	"strings"
	"testing"
)

// TestRun tests the run function with various inputs.
func TestRun(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		wantErr  bool
		contains string // Expected substring in output or error
	}{
		{
			name:     "basic addition",
			input:    "5 3 +",
			wantErr:  false,
			contains: "$5 + 3$",
		},
		{
			name:     "basic subtraction",
			input:    "5 3 -",
			wantErr:  false,
			contains: "$5 - 3$",
		},
		{
			name:     "basic multiplication",
			input:    "4 7 *",
			wantErr:  false,
			contains: "$4 \\times 7$",
		},
		{
			name:     "basic division",
			input:    "10 2 /",
			wantErr:  false,
			contains: "$10 \\div 2$",
		},
		{
			name:     "invalid character",
			input:    "2 3 ^",
			wantErr:  true,
			contains: "Unexpected character '^'",
		},
		{
			name:     "empty expression",
			input:    "",
			wantErr:  true,
			contains: "Empty expression",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := run(tt.input)
			if (err != nil) != tt.wantErr {
				t.Errorf("run() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			// For error cases, check error message content
			if tt.wantErr && err != nil {
				errMsg := err.Error()
				if !strings.Contains(errMsg, tt.contains) {
					t.Errorf("run() error message = %q, should contain %q", errMsg, tt.contains)
				}
			}
		})
	}
}

// TestCLI_NoArguments tests that the CLI requires at least one argument.
func TestCLI_NoArguments(t *testing.T) {
	// Build the binary
	buildCmd := exec.Command("go", "build", "-o", "rpn2tex_cli_test")
	buildCmd.Dir = "."
	if err := buildCmd.Run(); err != nil {
		t.Fatalf("Failed to build binary: %v", err)
	}
	defer os.Remove("rpn2tex_cli_test")

	// Run without arguments
	cmd := exec.Command("./rpn2tex_cli_test")
	output, err := cmd.CombinedOutput()

	exitCode := 0
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			exitCode = exitErr.ExitCode()
		}
	}

	if exitCode != 1 {
		t.Errorf("exit code = %d, want 1", exitCode)
	}

	outStr := string(output)
	if !strings.Contains(outStr, "Usage:") {
		t.Errorf("output = %q, should contain 'Usage:'", outStr)
	}
}

// TestCLI_SuccessfulExecution tests successful execution with exit code 0.
func TestCLI_SuccessfulExecution(t *testing.T) {
	// Build the binary
	buildCmd := exec.Command("go", "build", "-o", "rpn2tex_cli_test")
	buildCmd.Dir = "."
	if err := buildCmd.Run(); err != nil {
		t.Fatalf("Failed to build binary: %v", err)
	}
	defer os.Remove("rpn2tex_cli_test")

	tests := []struct {
		args     []string
		expected string
	}{
		{[]string{"5", "3", "+"}, "$5 + 3$"},
		{[]string{"4", "7", "*"}, "$4 \\times 7$"},
		{[]string{"2", "3", "+", "4", "*"}, "$( 2 + 3 ) \\times 4$"},
	}

	for _, tt := range tests {
		t.Run(strings.Join(tt.args, " "), func(t *testing.T) {
			cmd := exec.Command("./rpn2tex_cli_test", tt.args...)
			output, err := cmd.Output()

			if err != nil {
				t.Fatalf("command failed: %v", err)
			}

			got := strings.TrimSpace(string(output))
			if got != tt.expected {
				t.Errorf("output = %q, want %q", got, tt.expected)
			}
		})
	}
}

// TestCLI_ErrorExecution tests error cases with exit code 1.
func TestCLI_ErrorExecution(t *testing.T) {
	// Build the binary
	buildCmd := exec.Command("go", "build", "-o", "rpn2tex_cli_test")
	buildCmd.Dir = "."
	if err := buildCmd.Run(); err != nil {
		t.Fatalf("Failed to build binary: %v", err)
	}
	defer os.Remove("rpn2tex_cli_test")

	tests := []struct {
		args         []string
		errorMessage string
	}{
		{[]string{"2", "3", "^"}, "Unexpected character '^'"},
		{[]string{"2", "3", "^", "4", "*"}, "Unexpected character '^'"},
	}

	for _, tt := range tests {
		t.Run(strings.Join(tt.args, " "), func(t *testing.T) {
			cmd := exec.Command("./rpn2tex_cli_test", tt.args...)
			output, err := cmd.CombinedOutput()

			exitCode := 0
			if err != nil {
				if exitErr, ok := err.(*exec.ExitError); ok {
					exitCode = exitErr.ExitCode()
				}
			}

			if exitCode != 1 {
				t.Errorf("exit code = %d, want 1", exitCode)
			}

			outStr := string(output)
			if !strings.Contains(outStr, tt.errorMessage) {
				t.Errorf("error output = %q, should contain %q", outStr, tt.errorMessage)
			}
		})
	}
}

// TestPipeline tests the complete pipeline with all components.
func TestPipeline(t *testing.T) {
	input := "2 3 + 4 *"
	expected := "$( 2 + 3 ) \\times 4$"

	// Step 1: Lexer
	lexer := NewLexer(input)
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("Lexer.Tokenize() error = %v", err)
	}
	if len(tokens) == 0 {
		t.Fatal("Lexer.Tokenize() returned empty token list")
	}

	// Step 2: Parser
	parser := NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		t.Fatalf("Parser.Parse() error = %v", err)
	}
	if ast == nil {
		t.Fatal("Parser.Parse() returned nil AST")
	}

	// Step 3: LaTeX Generator
	generator := NewLaTeXGenerator()
	latex := generator.Generate(ast)
	if latex != expected {
		t.Errorf("LaTeXGenerator.Generate() = %q, want %q", latex, expected)
	}
}

// TestErrorFormatting tests that errors are properly formatted with context.
func TestErrorFormatting(t *testing.T) {
	input := "2 3 ^"

	err := run(input)
	if err == nil {
		t.Fatal("run() expected error, got nil")
	}

	errMsg := err.Error()
	// Should contain the error message
	if !strings.Contains(errMsg, "Unexpected character '^'") {
		t.Errorf("Error message missing 'Unexpected character': %q", errMsg)
	}

	// Should contain context with the caret
	if !strings.Contains(errMsg, "^") {
		t.Errorf("Error message missing caret: %q", errMsg)
	}
}
