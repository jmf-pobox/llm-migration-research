package rpn2tex

import "testing"

func TestTokenTypeString(t *testing.T) {
	tests := []struct {
		tokenType TokenType
		want      string
	}{
		{TokenNumber, "NUMBER"},
		{TokenPlus, "PLUS"},
		{TokenEOF, "EOF"},
	}

	for _, tt := range tests {
		t.Run(tt.want, func(t *testing.T) {
			got := tt.tokenType.String()
			if got != tt.want {
				t.Errorf("TokenType.String() = %v, want %v", got, tt.want)
			}
		})
	}
}
