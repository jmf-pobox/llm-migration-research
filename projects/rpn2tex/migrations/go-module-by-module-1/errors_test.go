package rpn2tex

import (
	"testing"
)

func TestLexerError(t *testing.T) {
	err := &LexerError{
		Message: "Unexpected character '^'",
		Line:    1,
		Column:  5,
	}

	expected := "Line 1, column 5: Unexpected character '^'"
	if err.Error() != expected {
		t.Errorf("LexerError.Error() = %q, want %q", err.Error(), expected)
	}
}

func TestErrorFormatterSingleCaret(t *testing.T) {
	source := "2 3 ^"
	ef := NewErrorFormatter(source)

	formatted := ef.FormatError("Error: Unexpected character '^'", 1, 5, 1)

	expected := "Error: Unexpected character '^'\n\n1 | 2 3 ^\n  |     ^"

	if formatted != expected {
		t.Errorf("FormatError() mismatch\nGot:\n%q\n\nWant:\n%q", formatted, expected)
	}
}

func TestErrorFormatterMultipleTokens(t *testing.T) {
	source := "2 3 ^ 4 *"
	ef := NewErrorFormatter(source)

	formatted := ef.FormatError("Error: Unexpected character '^'", 1, 5, 1)

	expected := "Error: Unexpected character '^'\n\n1 | 2 3 ^ 4 *\n  |     ^"

	if formatted != expected {
		t.Errorf("FormatError() mismatch\nGot:\n%q\n\nWant:\n%q", formatted, expected)
	}
}

func TestErrorFormatterColumnSeven(t *testing.T) {
	source := "2 3 4 ^ ^"
	ef := NewErrorFormatter(source)

	formatted := ef.FormatError("Error: Unexpected character '^'", 1, 7, 1)

	expected := "Error: Unexpected character '^'\n\n1 | 2 3 4 ^ ^\n  |       ^"

	if formatted != expected {
		t.Errorf("FormatError() mismatch\nGot:\n%q\n\nWant:\n%q", formatted, expected)
	}
}
