# rpn2tex Java Migration - Feature-by-Feature Approach

## Phase 2: Numbers Feature (COMPLETED)

This directory contains the Java implementation of rpn2tex using a feature-by-feature migration approach.

### Current Status

**Completed Features**:
- ✓ Numbers (integers, decimals, negative numbers)

**Pending Features**:
- Binary operators (+, -, *, /)
- Exponentiation (^)
- Square root (sqrt)
- Nth root (root)

## Quick Start

### Build

```bash
./gradlew compileJava
```

### Run Tests

```bash
./gradlew test
```

### Run Application

```bash
# From stdin
echo "5" | java -cp build/classes/java/main com.rpn2tex.Main -

# From file
echo "3.14" > input.txt
java -cp build/classes/java/main com.rpn2tex.Main input.txt
```

Or use Gradle's run task:

```bash
./gradlew run --args="-"
# Then type input and press Ctrl+D
```

## Project Structure

```
java-feature-by-feature-1/
├── build.gradle                 # Build configuration
├── src/
│   ├── main/java/com/rpn2tex/
│   │   ├── TokenType.java       # Token type enumeration
│   │   ├── Token.java           # Token record
│   │   ├── RpnException.java    # Base exception
│   │   ├── LexerException.java  # Lexer-specific exception
│   │   ├── ParserException.java # Parser-specific exception
│   │   ├── ASTNode.java         # AST node interface
│   │   ├── NumberNode.java      # Number AST node
│   │   ├── Lexer.java           # Tokenizer
│   │   ├── Parser.java          # RPN parser
│   │   ├── LaTeXGenerator.java  # LaTeX code generator
│   │   └── Main.java            # CLI entry point
│   └── test/java/com/rpn2tex/
│       └── NumberFeatureTest.java
├── config/
│   └── checkstyle/
│       └── checkstyle.xml       # Code style rules
├── MIGRATION_REPORT.md          # Detailed migration report
└── README.md                    # This file
```

## Architecture

The implementation follows a clean pipeline architecture:

```
Input → Lexer → Parser → Generator → Output
         ↓        ↓          ↓
       Tokens   AST       LaTeX
```

### Component Responsibilities

1. **Lexer**: Scans text character-by-character, produces tokens
2. **Parser**: Consumes tokens, builds Abstract Syntax Tree (AST)
3. **Generator**: Traverses AST using visitor pattern, produces LaTeX

### Key Design Decisions

- **Immutability**: All data structures (Token, AST nodes) are immutable
- **Records**: Modern Java records for concise value objects
- **Sealed Types**: Closed type hierarchy for AST nodes
- **String Storage**: Numbers stored as strings to preserve format
- **Position Tracking**: All tokens and nodes track source position

## Development Workflow

### Adding a New Feature

1. Add token types to `TokenType.java`
2. Create AST node classes (records implementing `ASTNode`)
3. Update `Lexer.java` to scan new tokens
4. Update `Parser.java` to handle new tokens
5. Update `LaTeXGenerator.java` with visitor methods
6. Add tests to verify behavior
7. Run quality gates

### Quality Gates

```bash
# Compile
./gradlew compileJava

# Code style
./gradlew checkstyleMain

# Tests
./gradlew test

# All checks
./gradlew build
```

## Testing

### Unit Tests

```bash
./gradlew test
```

Tests are located in `src/test/java/com/rpn2tex/`.

### Manual Testing

```bash
# Basic integer
echo "5" | java -cp build/classes/java/main com.rpn2tex.Main -
# Expected: $5$

# Decimal
echo "3.14" | java -cp build/classes/java/main com.rpn2tex.Main -
# Expected: $3.14$

# Negative number
echo "-2" | java -cp build/classes/java/main com.rpn2tex.Main -
# Expected: $-2$
```

## Code Style

The project uses Checkstyle with standard Java conventions:
- PascalCase for classes and interfaces
- camelCase for methods and variables
- UPPER_SNAKE_CASE for constants
- Comprehensive Javadoc on public APIs

## Requirements

- Java 17 or higher
- Gradle (via included wrapper)

## Next Phase

**Phase 3**: Implement binary operators (+, -, *, /)
- Add PLUS, MINUS, MULT, DIV to TokenType
- Create BinaryOpNode record
- Update Lexer to recognize operator characters
- Update Parser to pop operands and build operation nodes
- Update Generator with precedence-aware parenthesization

See `MIGRATION_REPORT.md` for detailed implementation notes.
