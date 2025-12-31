# rpn2tex - Java Implementation

Convert Reverse Polish Notation (RPN) expressions to LaTeX mathematical notation.

## Overview

rpn2tex is a command-line tool that transforms RPN expressions into beautifully formatted LaTeX math expressions. It handles operator precedence automatically and inserts parentheses only when necessary.

### Example

```bash
echo "5 3 + 2 *" | java -cp build/classes/java/main com.rpn2tex.Main -
```

**Output**: `$( 5 + 3 ) \times 2$`

## Features

- **Automatic precedence handling**: Inserts parentheses only when needed
- **Proper LaTeX symbols**: `\times` for multiplication, `\div` for division
- **Decimal number support**: Preserves decimal points (e.g., 3.14)
- **Comprehensive error messages**: Shows source context with caret positioning
- **Flexible I/O**: Read from file or stdin, write to file or stdout

## Supported Operators

| Operator | Symbol | LaTeX | Precedence |
|----------|--------|-------|------------|
| Addition | `+` | `+` | 1 |
| Subtraction | `-` | `-` | 1 |
| Multiplication | `*` | `\times` | 2 |
| Division | `/` | `\div` | 2 |

## Installation

### Prerequisites
- Java 21 or higher
- Gradle 8.x (included via wrapper)

### Build

```bash
./gradlew build
```

## Usage

### Basic Usage

```bash
# Read from stdin, write to stdout
echo "5 3 +" | java -cp build/classes/java/main com.rpn2tex.Main -

# Read from file, write to stdout
java -cp build/classes/java/main com.rpn2tex.Main input.rpn

# Read from file, write to file
java -cp build/classes/java/main com.rpn2tex.Main input.rpn -o output.tex
```

### Command-Line Options

```
Usage: java com.rpn2tex.Main <input> [-o <output>]

Arguments:
  <input>     Input RPN file, or '-' for stdin (required)
  -o FILE     Output LaTeX file (optional, defaults to stdout)
```

### Examples

#### Example 1: Basic Arithmetic
```bash
echo "10 2 /" | java -cp build/classes/java/main com.rpn2tex.Main -
```
**Output**: `$10 \div 2$`

#### Example 2: Precedence with Parentheses
```bash
echo "5 3 + 2 *" | java -cp build/classes/java/main com.rpn2tex.Main -
```
**Output**: `$( 5 + 3 ) \times 2$`

#### Example 3: Complex Expression
```bash
echo "10 2 / 3 + 4 *" | java -cp build/classes/java/main com.rpn2tex.Main -
```
**Output**: `$( 10 \div 2 + 3 ) \times 4$`

#### Example 4: Decimal Numbers
```bash
echo "3.14 2 *" | java -cp build/classes/java/main com.rpn2tex.Main -
```
**Output**: `$3.14 \times 2$`

#### Example 5: File I/O
```bash
echo "1 2 + 3 4 + *" > input.rpn
java -cp build/classes/java/main com.rpn2tex.Main input.rpn -o output.tex
cat output.tex
```
**Output**: `$( 1 + 2 ) \times ( 3 + 4 )$`

### Error Handling

The tool provides detailed error messages with source context:

```bash
echo "2 3 ^" | java -cp build/classes/java/main com.rpn2tex.Main - 2>&1
```

**Output**:
```
Error: Unexpected character '^'

1 | 2 3 ^
  |     ^
```

**Exit code**: 1

## Exit Codes

- **0**: Success - expression converted successfully
- **1**: Error - lexer error, parser error, or I/O error

## Development

### Build Commands

```bash
# Compile source
./gradlew compileJava

# Run tests
./gradlew test

# Generate coverage report
./gradlew test jacocoTestReport

# Run checkstyle
./gradlew checkstyleMain

# Clean build
./gradlew clean build
```

### Test Coverage

- **Total Tests**: 285
- **Test Classes**: 11
- **Coverage**: Generated in `build/reports/jacoco/test/html/index.html`

### Project Structure

```
src/
├── main/java/com/rpn2tex/
│   ├── Main.java              # CLI entry point
│   ├── Lexer.java             # Tokenizer
│   ├── Parser.java            # RPN parser
│   ├── LaTeXGenerator.java    # LaTeX code generator
│   ├── Token.java             # Token data structure
│   ├── TokenType.java         # Token types
│   ├── Expr.java              # Expression interface
│   ├── Number.java            # Number AST node
│   ├── BinaryOp.java          # Binary operation AST node
│   └── RpnException.java      # Exception with ErrorFormatter
└── test/java/com/rpn2tex/
    ├── MainTest.java          # CLI tests (49 tests)
    ├── LexerTest.java         # Lexer tests
    ├── ParserTest.java        # Parser tests
    └── ...                    # Additional test files
```

## RPN Expression Format

### What is RPN?

Reverse Polish Notation (RPN) is a mathematical notation where operators follow their operands. This eliminates the need for parentheses in expressions.

### Examples

| Infix Notation | RPN | LaTeX Output |
|----------------|-----|--------------|
| `5 + 3` | `5 3 +` | `$5 + 3$` |
| `(5 + 3) * 2` | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` |
| `5 * 3 + 2` | `5 3 * 2 +` | `$5 \times 3 + 2$` |
| `2 * (3 + 4)` | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` |

### How to Convert Infix to RPN

1. Start with an empty stack
2. For each token from left to right:
   - If it's a number, output it
   - If it's an operator:
     - While there's an operator on top of the stack with higher or equal precedence, output it
     - Push the current operator onto the stack
3. Output all remaining operators on the stack

## Parenthesization Rules

The tool automatically inserts parentheses based on operator precedence:

- **Lower precedence needs parens**: `(5 + 3) * 2`
- **Higher precedence doesn't**: `5 * 3 + 2`
- **Left-associative operators**: Subtraction and division are left-associative
  - `5 - (3 - 2)` needs parens on right
  - `5 - 3 - 2` doesn't need parens (left-to-right)

## Troubleshooting

### Error: "Unexpected character"

**Cause**: Unsupported character in input (e.g., `^` for exponentiation)

**Solution**: Use only supported operators: `+`, `-`, `*`, `/`

### Error: "Operator requires two operands"

**Cause**: Insufficient operands on stack for operator

**Example**: `5 +` (missing second operand)

**Solution**: Ensure RPN expression is well-formed

### Error: "Invalid RPN: N values remain on stack"

**Cause**: Too many operands, not enough operators

**Example**: `5 3 2` (missing operators)

**Solution**: Add operators to consume all operands

### Error: "Empty expression"

**Cause**: No input provided or whitespace-only input

**Solution**: Provide a valid RPN expression

## License

This project is part of the rpn2tex migration study.

## Authors

- Original Python implementation: Source project
- Java migration: Module-by-module migration (Phase 3)

## See Also

- [Migration Summary](MAIN_MIGRATION_SUMMARY.md)
- [Validation Report](VALIDATION_REPORT.md)
- [Phase 1 Migration Spec](artifacts/PHASE_1_MIGRATION_SPEC.md)
