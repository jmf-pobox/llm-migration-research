# PHASE 1: Comprehensive Migration Specification for rpn2tex

**Document Version:** 1.0
**Generated:** 2025-12-30
**Status:** Complete Analysis of All Python Modules

---

## I/O Contract (from Phase 0)

This contract specifies the exact behavioral requirements that the Rust implementation must satisfy.

### Supported Features

**Operators:**
- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`

**Data Types:**
- Integers: e.g., `5`, `42`, `-3`
- Floating-point: e.g., `3.14`, `1.5`, `0.5`
- Negative numbers: e.g., `-5` (hyphen immediately before digits)

**Not Implemented:**
- Exponentiation: `^` (not supported by Python lexer)
- Square root: `sqrt` (not supported)
- Nth root: `root` (not supported)

### Valid Test Cases (Success Cases)

| Input | Expected Output | Exit Code | Notes |
|-------|-----------------|-----------|-------|
| `5 3 +` | `$5 + 3$` | 0 | Simple addition |
| `5 3 -` | `$5 - 3$` | 0 | Simple subtraction |
| `4 7 *` | `$4 \times 7$` | 0 | Simple multiplication |
| `10 2 /` | `$10 \div 2$` | 0 | Simple division |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | 0 | Operator precedence: (5+3)*2 |
| `5 3 * 2 +` | `$5 \times 3 + 2$` | 0 | Operator precedence: 5*3+2 |
| `10 2 / 5 *` | `$10 \div 2 \times 5$` | 0 | Left-to-right: (10/2)*5 |
| `5 3 - 2 -` | `$5 - 3 - 2$` | 0 | Left-to-right: (5-3)-2 |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | 0 | Chained division |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | 0 | Chained addition |
| `2 3 4 * +` | `$2 + 3 \times 4$` | 0 | Precedence: 2+(3*4) |
| `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | 0 | Explicit grouping via RPN |
| `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | 0 | Grouping on right operand |
| `2 3 * 4 +` | `$2 \times 3 + 4$` | 0 | Multiplication then addition |
| `3.14 2 *` | `$3.14 \times 2$` | 0 | Floating-point multiplication |
| `1.5 0.5 +` | `$1.5 + 0.5$` | 0 | Floating-point addition |
| `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | 0 | Multiple subexpressions |
| `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | 0 | Complex expression |
| `5` | `$5$` | 0 | Single number (no operation) |

### Error Cases

| Input | Expected Output | Exit Code | Error Message | Notes |
|-------|-----------------|-----------|---------------|-------|
| `` (empty) | (empty) | 1 | `Error: Empty expression` | Empty input |
| `5 3` | (empty) | 1 | `Error: Invalid RPN: 2 values remain on stack (missing operators?)` | Missing operator |
| `5 3 + +` | (empty) | 1 | `Error: Operator '+' requires two operands` | Insufficient operands for operator |
| `2 3 ^` | (empty) | 1 | `Error: Unexpected character '^'` | Unsupported operator (exponentiation) |
| `2 3 ^ 4 *` | (empty) | 1 | `Error: Unexpected character '^'` | Unsupported operator in expression |
| `2 3 4 ^ ^` | (empty) | 1 | `Error: Unexpected character '^'` | Multiple unsupported operators |
| `invalid` | (empty) | 1 | `Error: Unexpected character 'i'` | Unrecognized token |
| `5 @ 3` | (empty) | 1 | `Error: Unexpected character '@'` | Invalid character |

### LaTeX Output Format Specification

**Operator Representation:**
- **Addition**: ` + ` (space-padded)
- **Subtraction**: ` - ` (space-padded)
- **Multiplication**: ` \times ` (with spaces)
- **Division**: ` \div ` (with spaces)

**Math Mode Delimiters:**
- All outputs wrapped in `$...$` (inline math mode)

**Parentheses Handling:**
- Parentheses added when needed to preserve operator precedence
- Format: `( expr )` (spaces inside parentheses)
- Example: `$( 5 + 3 ) \times 2$`

**Numeric Literals:**
- Integers: rendered as-is (e.g., `5`)
- Floats: rendered as-is (e.g., `3.14`)
- Negative numbers: rendered with hyphen (e.g., `-5`)
- No scientific notation

### Implementation Details (from I/O Contract)

**Parser Stack-Based Algorithm:**
1. Number tokens are pushed onto the stack
2. Operator tokens pop two operands (right then left)
3. A binary operation node is created and pushed back
4. At EOF, exactly one item should remain on stack

**Error Handling Priority:**
1. **Lexer errors** (invalid characters) - caught first
2. **Parser errors** (invalid RPN structure) - caught second
3. Error messages include line and column information

**Whitespace Handling:**
- Whitespace acts as token delimiter
- Multiple spaces treated as single delimiter
- Newlines accepted as delimiters
- No leading/trailing whitespace issues

### Behavioral Notes for Rust Implementation

1. **Exact Output Matching Required**: Identical LaTeX output including exact spacing and parenthesization
2. **Error Messages Must Match**: Identical wording (formatter may differ)
3. **Exit Codes**: 0 for success, 1 for errors
4. **No Exponentiation**: Not implemented
5. **Numeric Precision**: Floating-point numbers passed through as-is
6. **Negative Number Handling**: `-5 3 +` should work

---

## Module Dependency Graph

```
cli.py (MAIN ENTRY POINT)
  ├─ lexer.py
  │  ├─ tokens.py
  │  └─ errors.py (indirectly used)
  ├─ parser.py
  │  ├─ tokens.py
  │  └─ ast_nodes.py
  ├─ latex_gen.py
  │  └─ ast_nodes.py
  └─ errors.py

Dependency Order for Migration:
1. tokens.py (no dependencies)
2. ast_nodes.py (depends on tokens.py for type hints)
3. errors.py (no dependencies)
4. lexer.py (depends on tokens.py)
5. parser.py (depends on tokens.py, ast_nodes.py)
6. latex_gen.py (depends on ast_nodes.py)
7. cli.py (depends on all modules)
```

---

## Module Analyses

---

### Module: tokens.py

**File Location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/tokens.py`

**Purpose:** Define token types and the Token data structure used throughout the lexer/parser pipeline.

#### Public API

**Classes:**

1. **TokenType (Enum)**
   - An enumeration of all possible token types in RPN expressions
   - Members:
     - `NUMBER` - Numeric literals (integers and decimals)
     - `PLUS` - Addition operator (`+`)
     - `MINUS` - Subtraction operator (`-`)
     - `MULT` - Multiplication operator (`*`)
     - `DIV` - Division operator (`/`)
     - `EOF` - End of file marker
   - Each member is created with `auto()` which automatically assigns unique values

2. **Token (frozen dataclass)**
   - Immutable data structure representing a single lexical token
   - **Attributes:**
     - `type: TokenType` - The token type
     - `value: str` - The string value of the token (e.g., "42", "+", "3.14")
     - `line: int` - Line number (1-based) where token appears
     - `column: int` - Column number (1-based) where token starts
   - **Methods:**
     - `__repr__() -> str` - Returns debug string in format `Token(TYPE, 'value', line:column)`

**Functions:** None

**Constants:** None (TokenType members are enum variants)

#### Dependencies

**Internal:** None
**External:**
- `dataclasses` (standard library) - for `@dataclass` decorator
- `enum` (standard library) - for `Enum` and `auto()`

#### Rust Migration Notes

**Type Mappings:**
- `TokenType` enum → Rust `enum TokenType`
- `Token` dataclass → Rust `struct Token`
- `@frozen=True` dataclass → Rust struct (immutable by default)
- `type: TokenType` → `type_: TokenType` (avoid keyword collision)
- `value: str` → `value: String` (or `&str` for borrowed references)
- `line: int` → `line: u32` or `usize`
- `column: int` → `column: u32` or `usize`

**Pattern Changes:**
- Python's `auto()` → Rust's numeric enum variants with explicit values
- Frozen dataclass → Rust struct (immutable by default, no need for `#[derive(Copy)]`)
- `__repr__` magic method → Rust `impl Display` or `Debug` trait
- Type union (if needed) → Use Rust's type system or `#[non_exhaustive]`

**Special Handling:**
- `repr()` output format must be preserved for error reporting
- Line and column numbers are 1-based (not 0-based)
- EOF token uses empty string for value

#### Key Implementation Details

Token is a simple value type carrying position information. The Token struct must preserve exact layout and semantics for position tracking used in error reporting. The repr() format is not directly visible to users but follows the pattern `Token(TYPE, 'value', line:column)`.

TokenType enum has exactly 6 members. The order doesn't matter for functionality, but Python's implementation uses `auto()`. In Rust, explicit values (0-5) are typical.

---

### Module: ast_nodes.py

**File Location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/ast_nodes.py`

**Purpose:** Define the Abstract Syntax Tree (AST) node types that represent the structure of parsed RPN expressions.

#### Public API

**Classes:**

1. **ASTNode (frozen dataclass, base class)**
   - Base class for all AST node types
   - **Attributes:**
     - `line: int` - Line number (1-based) where node appears
     - `column: int` - Column number (1-based) where node starts
   - **Purpose:** Provides position information for error reporting
   - **No methods defined**

2. **Number (frozen dataclass, inherits from ASTNode)**
   - Represents numeric literals
   - **Attributes:**
     - `line: int` (inherited)
     - `column: int` (inherited)
     - `value: str` - The string representation of the number (e.g., "42", "3.14", "-5")
   - **Example:** `Number(line=1, column=1, value="42")`
   - Numbers are stored as strings (not parsed to actual numeric types)

3. **BinaryOp (frozen dataclass, inherits from ASTNode)**
   - Represents binary operations (+, -, *, /)
   - **Attributes:**
     - `line: int` (inherited)
     - `column: int` (inherited)
     - `operator: str` - The operator as a string: "+", "-", "*", or "/"
     - `left: Expr` - The left operand (recursive type)
     - `right: Expr` - The right operand (recursive type)
   - **Example:**
     ```python
     BinaryOp(
         line=1, column=3,
         operator="+",
         left=Number(1, 1, "5"),
         right=Number(1, 3, "3")
     )
     ```

**Functions:** None

**Type Aliases:**
- `Expr = Number | BinaryOp` - Union type representing any expression

#### Dependencies

**Internal:** None
**External:**
- `dataclasses` (standard library) - for `@dataclass` decorator

#### Rust Migration Notes

**Type Mappings:**
- `ASTNode` base class → Rust trait or enum variant approach
  - Option 1: Use trait inheritance pattern
  - Option 2: Flatten to enum with position fields in each variant
- `Number` → Rust struct with explicit position fields
- `BinaryOp` → Rust struct with position fields
- `Expr` type alias → Rust `enum Expr` or `Box<Expr>` for recursive types
- Frozen dataclass → Rust struct (all fields immutable)

**Pattern Changes:**
- Python inheritance → Rust enums or trait objects
- Type union (`|`) → Rust `enum` type
- Recursive types with `Expr` → Need `Box<Expr>` for heap allocation
- String operators → Can stay as `String` or use enum for operators

**Special Handling:**
- Recursive types require boxing in Rust (use `Box<Expr>`)
- Position fields are critical for error reporting and must be preserved
- Number values are always strings (not parsed to f64 or i64)
- Operator must distinguish between "+", "-", "*", "/" (exact matching)

#### Key Implementation Details

AST is a tree of immutable nodes. The design choice to store numbers as strings is important—numbers are NOT evaluated during parsing, only during LaTeX generation (where they're still kept as strings).

The hierarchy is flat: both Number and BinaryOp are leaf/branch nodes respectively. Position tracking is essential for error messages.

Recursive structure: BinaryOp contains `left` and `right` which are both `Expr`, creating a tree. This requires careful handling in Rust with Box allocations.

---

### Module: errors.py

**File Location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/errors.py`

**Purpose:** Format parse and lexer errors with source context (gcc/rustc style).

#### Public API

**Classes:**

1. **ErrorFormatter**
   - Formats errors with source context and helpful hints
   - **Attributes:**
     - `source: str` - The complete source text being parsed
     - `lines: list[str]` - Source text split into lines
   - **Methods:**
     - `__init__(source: str) -> None`
       - Initializes formatter with source text
       - Splits source into lines
     - `format_error(message: str, line: int, column: int, *, context_lines: int = 1) -> str`
       - Formats an error with source context
       - **Parameters:**
         - `message: str` - The error message
         - `line: int` - Line number (1-based)
         - `column: int` - Column number (1-based)
         - `context_lines: int` - Number of lines to show before/after error (keyword-only, default 1)
       - **Returns:** Formatted error string with context and caret pointing to error location
       - **Example Output:**
         ```
         Error: Unexpected character '@'

         1 | 5 3 @
             ^
         ```
     - `_get_context(line: int, column: int, context_lines: int) -> str` (private)
       - Extracts source context around error position
       - Returns formatted context with line numbers and caret
       - **Algorithm:**
         1. Convert line number from 1-based to 0-based index
         2. Calculate start_idx and end_idx (clamped to valid range)
         3. Calculate line number width for alignment
         4. For each line in range:
            - Format with line number prefix
            - Add caret line on error line at column position

**Functions:** None

**Constants:** None

#### Dependencies

**Internal:** None
**External:** None (uses only built-in Python)

#### Rust Migration Notes

**Type Mappings:**
- `ErrorFormatter` class → Rust struct
- `source: str` → `source: String`
- `lines: list[str]` → `lines: Vec<String>`
- `str` return types → `String` (owned) or `&str` (borrowed)

**Pattern Changes:**
- Method with keyword-only argument → Rust doesn't have keyword-only args, use builder pattern or explicit param names
- Private method `_get_context` → Use `fn` with `self` (no privacy modifier needed, or use module visibility)
- List comprehensions → Iterator chains or explicit loops

**Special Handling:**
- String formatting for line numbers (right-aligned with padding)
- Caret positioning must be exact (column-1 spaces before caret)
- 1-based line/column numbers must be preserved throughout
- Handling of multi-line context with proper clamping

#### Key Implementation Details

The formatter doesn't parse; it just provides visual error context. It's called AFTER a lexer or parser error is detected.

Output format is critical:
- Header: `Error: {message}`
- Blank line
- Line number prefix: `{line_num:>width} | {content}`
- Caret line: spaces aligned with prefix + spaces for column + `^`

Example with wider context (context_lines=1):
```
Error: Unexpected character 'x'

2 | 5 3 x
      ^
```

The caret position is calculated as: `column - 1` spaces (since columns are 1-based).

---

### Module: lexer.py

**File Location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/lexer.py`

**Purpose:** Tokenize RPN input text into a stream of Token objects.

#### Public API

**Classes:**

1. **LexerError (Exception subclass)**
   - Raised when lexer encounters invalid input
   - **Attributes:**
     - `message: str` - Description of the error
     - `line: int` - Line number where error occurred (1-based)
     - `column: int` - Column number where error occurred (1-based)
   - **Methods:**
     - `__init__(message: str, line: int, column: int) -> None`
       - Constructs error with position information
       - Calls parent with formatted message: `f"Line {line}, column {column}: {message}"`
   - **Example:** `LexerError("Unexpected character '@'", 1, 5)`

2. **Lexer**
   - Tokenizes RPN input text character by character
   - **Attributes:**
     - `text: str` - The input text to tokenize
     - `pos: int` - Current position in text (0-based)
     - `line: int` - Current line number (1-based)
     - `column: int` - Current column number (1-based)
   - **Methods:**
     - `__init__(text: str) -> None`
       - Initialize lexer with input text
       - Sets pos=0, line=1, column=1
     - `tokenize() -> list[Token]`
       - Tokenize entire input text
       - **Algorithm:**
         1. Initialize empty token list
         2. While not at end:
            - Skip whitespace
            - If at end after skipping, break
            - Scan next token and append
         3. Append EOF token
         4. Return token list
       - **Returns:** List of tokens ending with EOF
       - **Raises:** `LexerError` if invalid character encountered
       - **Example:** `Lexer("2 3 + 4 *").tokenize()` → `[Token(NUMBER, '2', 1:1), Token(NUMBER, '3', 1:3), ...]`
     - `_at_end() -> bool` (private)
       - Check if at end of input: `pos >= len(text)`
     - `_peek() -> str` (private)
       - Look at current character without consuming (returns "" if at end)
     - `_advance() -> str` (private)
       - Consume and return current character
       - Updates line and column tracking:
         - If char is `\n`: increment line, reset column to 1
         - Else: increment column
     - `_skip_whitespace() -> None` (private)
       - Skip over whitespace characters (space, tab, newline, carriage return)
     - `_scan_token() -> Token` (private)
       - Scan and return next token
       - **Algorithm:**
         1. Save start_line and start_column
         2. Peek at current character
         3. If `+`: consume, return PLUS token
         4. If `-`: consume, check if next is digit
            - If digit follows: scan as negative number (recursion to _scan_number)
            - Else: return MINUS token
         5. If `*`: consume, return MULT token
         6. If `/`: consume, return DIV token
         7. If digit: scan as number
         8. Else: raise LexerError
       - **Returns:** Token of appropriate type
       - **Raises:** LexerError for invalid characters
     - `_scan_number(prefix: str, start_line: int, start_column: int) -> Token` (private)
       - Scan a numeric literal
       - **Parameters:**
         - `prefix: str` - Any prefix already consumed (e.g., "-" for negatives)
         - `start_line: int` - Line where number started
         - `start_column: int` - Column where number started
       - **Algorithm:**
         1. Initialize value = prefix
         2. Scan integer part: consume digits
         3. Check for decimal point: if present, consume and scan fractional digits
         4. Return NUMBER token with accumulated value
       - **Returns:** NUMBER token
       - **Important:** Numbers stored as strings, not parsed

**Functions:** None

**Constants:** None

#### Dependencies

**Internal:**
- `tokens.py` - imports `Token`, `TokenType`

**External:**
- `__future__` (standard library) - for annotations

#### Rust Migration Notes

**Type Mappings:**
- `LexerError` exception → Rust enum variant or custom Error type
- `Lexer` class → Rust struct with methods
- `pos: int` → `pos: usize`
- `line: int` → `line: u32` or `usize`
- `column: int` → `column: u32` or `usize`
- `text: str` → `text: &str` or `String`

**Pattern Changes:**
- Exception raising → Rust `Result<T, E>` or `std::error::Error`
- Private methods with leading `_` → Rust modules/visibility (private by default)
- String character checking (`.isdigit()`) → Rust `char::is_numeric()` or `char::is_ascii_digit()`
- Whitespace check (`in " \t\n\r"`) → Rust `.is_whitespace()` or explicit checks
- Recursive method call in `_scan_token` → Direct call to `_scan_number` with "-" prefix

**Special Handling:**
- Position tracking (line, column) must exactly match Python behavior
- Negative number detection: `-` followed immediately by digit
- Whitespace-as-delimiter behavior
- EOF token with empty value
- Error messages must match exactly: `"Unexpected character '{char}'"`

#### Key Implementation Details

The lexer implements a simple character-at-a-time scanner with position tracking. Key behaviors:

1. **Position Tracking:** Both line and column are 1-based. Newlines reset column to 1 and increment line.

2. **Negative Numbers:** The lexer treats `-` followed immediately by a digit as the start of a negative number (not minus operator). This is a specific RPN convention.

3. **Token Generation:**
   - Numbers: stored as strings (e.g., "42", "3.14", "-5")
   - Operators: +, -, *, /
   - EOF: special marker token with empty value

4. **Error Handling:** Any unrecognized character raises LexerError with position.

5. **Whitespace:** Acts as token delimiter. Tabs, spaces, newlines all treated equally.

---

### Module: parser.py

**File Location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/parser.py`

**Purpose:** Convert token stream into an Abstract Syntax Tree using stack-based RPN parsing algorithm.

#### Public API

**Classes:**

1. **ParserError (Exception subclass)**
   - Raised when parser encounters invalid input
   - **Attributes:**
     - `message: str` - Description of the error
     - `token: Token` - The token where error occurred
   - **Methods:**
     - `__init__(message: str, token: Token) -> None`
       - Constructs error with token context
       - Calls parent with: `f"{message} at line {token.line}, column {token.column}"`
   - **Example:** `ParserError("Not enough operands", token)`

2. **Parser**
   - Stack-based RPN parser converting tokens to AST
   - **Attributes:**
     - `tokens: list[Token]` - List of tokens from lexer (must end with EOF)
     - `pos: int` - Current position in token list (0-based)
   - **Methods:**
     - `__init__(tokens: list[Token]) -> None`
       - Initialize parser with token list
       - Sets pos=0
     - `parse() -> Expr`
       - Parse tokens into an AST
       - **Algorithm (Stack-based RPN):**
         1. Initialize empty stack
         2. While not at EOF:
            - Get current token
            - If NUMBER: create Number node, push to stack, advance
            - If OPERATOR (+, -, *, /):
              - Check stack has at least 2 items
              - Pop right, pop left (order matters!)
              - Create BinaryOp node with left, operator, right
              - Push result back, advance
            - If EOF: break
            - Else: raise ParserError
         3. Validate final state:
            - If stack is empty: raise "Empty expression"
            - If stack has >1 items: raise "Invalid RPN: N values remain..."
         4. Return stack[0]
       - **Returns:** Root Expr node (Number or BinaryOp)
       - **Raises:** ParserError for invalid RPN
       - **Example:** `Parser(Lexer("5 3 +").tokenize()).parse()` → `BinaryOp(...)`
     - `_current() -> Token` (private)
       - Get current token: `tokens[pos]`
     - `_at_end() -> bool` (private)
       - Check if at EOF: `tokens[pos].type == TokenType.EOF`
     - `_advance() -> Token` (private)
       - Consume current token, advance to next
       - Returns old token, increments pos (if not already at EOF)

**Functions:** None

**Constants:** None

#### Dependencies

**Internal:**
- `ast_nodes.py` - imports `BinaryOp`, `Expr`, `Number`
- `tokens.py` - imports `Token`, `TokenType`

**External:**
- `__future__` (standard library) - for annotations

#### Rust Migration Notes

**Type Mappings:**
- `ParserError` exception → Rust enum variant or custom Error type
- `Parser` class → Rust struct with methods
- `pos: int` → `usize`
- `stack: list[Expr]` → `Vec<Expr>`
- `Expr` union type → Rust enum
- Token type checks → Rust pattern matching

**Pattern Changes:**
- Exception raising → Rust `Result<Expr, ParserError>`
- List operations (push, pop) → Rust `Vec` methods
- Dictionary lookup (`op_map[token.type]`) → Rust match or HashMap
- Private methods → Rust visibility (private by default in impl blocks)

**Special Handling:**
- Stack operations: pop order is critical (right first, then left)
- Error messages must match exactly
- Token position information must be preserved for error reporting
- Recursive types (Expr) need Box allocations in Rust
- EOF validation is important (stack shouldn't be empty after parsing)

#### Key Implementation Details

The parser implements a classic stack-based RPN algorithm:

1. **Number tokens** → Push Number node to stack
2. **Operator tokens** → Pop two operands, create BinaryOp, push result
3. **EOF** → Stop processing

**Critical detail:** When popping operands, the RIGHT operand is popped first, then LEFT. This preserves correct semantics for non-commutative operations (- and /).

**Example trace for "5 3 - 2 /":**
- `5` → push 5
- `3` → push 3
- `-` → pop 3 (right), pop 5 (left), push BinaryOp("-", 5, 3)
- `2` → push 2
- `/` → pop 2 (right), pop BinaryOp(...), push BinaryOp("/", BinaryOp("-", 5, 3), 2)

**Error validation:**
- Empty stack after parsing → "Empty expression" (no operators at all)
- Multiple items on stack → "Invalid RPN: N values remain on stack (missing operators?)"
- Operator with insufficient operands → "Operator 'X' requires two operands"

---

### Module: latex_gen.py

**File Location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/latex_gen.py`

**Purpose:** Convert AST into LaTeX math mode string with proper operator precedence and parenthesization.

#### Public API

**Classes:**

1. **LaTeXGenerator**
   - Converts rpn2tex AST to LaTeX source code
   - Uses visitor pattern with `@singledispatchmethod`
   - **Class Attributes:**
     - `BINARY_OPS: dict[str, str]` - Operator to LaTeX command mapping:
       - `"+": "+"`
       - `"-": "-"`
       - `"*": r"\times"`
       - `"/": r"\div"`
     - `PRECEDENCE: dict[str, int]` - Operator precedence (higher = binds tighter):
       - `"+": 1`
       - `"-": 1`
       - `"*": 2`
       - `"/": 2`
   - **Methods:**
     - `__init__()` - No parameters, simple constructor
     - `generate(ast: Expr) -> str`
       - Generate LaTeX from AST
       - **Algorithm:**
         1. Call _visit(ast) to generate content
         2. Wrap in math delimiters: `f"${content}$"`
       - **Parameters:** `ast: Expr` - root expression node
       - **Returns:** LaTeX string like `"$5 + 3$"`
       - **Example:** `LaTeXGenerator().generate(ast)` → `"$5 + 3$"`
     - `_visit(node: Expr) -> str` (private, singledispatch)
       - Main visitor dispatcher
       - Dispatches to specific _visit_* methods based on node type
       - **Raises:** `NotImplementedError` for unhandled node types
     - `_visit_number(node: Number) -> str` (private, registered handler)
       - Generate LaTeX for number literal
       - **Algorithm:** Return node.value as-is
       - **Returns:** String representation of number
       - **Example:** Number("42") → "42"
     - `_visit_binary_op(node: BinaryOp) -> str` (private, registered handler)
       - Generate LaTeX for binary operation
       - **Algorithm:**
         1. Look up operator LaTeX: `BINARY_OPS[node.operator]`
         2. Get operator precedence: `PRECEDENCE[node.operator]`
         3. Recursively generate left operand
         4. Add parentheses to left if needed: `_needs_parens(node.left, my_precedence, is_right=False)`
         5. Recursively generate right operand
         6. Add parentheses to right if needed: `_needs_parens(node.right, my_precedence, is_right=True)`
         7. Combine: `f"{left} {op_latex} {right}"`
       - **Returns:** LaTeX string with proper parentheses
       - **Example:** `BinaryOp("+", 5, 3)` → `"5 + 3"`
       - **Example with parens:** `BinaryOp("*", BinaryOp("+", 5, 3), 2)` → `"( 5 + 3 ) \times 2"`
     - `_needs_parens(child: Expr, parent_precedence: int, *, is_right: bool) -> bool` (private)
       - Determine if child expression needs parentheses
       - **Algorithm:**
         1. If child is not BinaryOp: return False (numbers don't need parens)
         2. Get child precedence: `PRECEDENCE[child.operator]`
         3. If child_precedence < parent_precedence: return True (lower precedence needs parens)
         4. If child_precedence == parent_precedence AND is_right AND child.operator in ("-", "/"):
            - Return True (right-associative operators on non-commutative ops need parens)
         5. Else: return False
       - **Returns:** Boolean
       - **Purpose:** Implements left-associativity for - and /
       - **Examples:**
         - `5 - 3 - 2` → `"5 - 3 - 2"` (left-associative, no parens)
         - `5 - (3 - 2)` → Would need parens on right: `"5 - ( 3 - 2 )"`
         - `5 * 3 + 2` → `"5 \times 3 + 2"` (no parens, + has lower precedence)
         - `5 * (3 + 2)` → `"5 \times ( 3 + 2 )"` (parens needed)

**Functions:** None

**Constants:** See Class Attributes above

#### Dependencies

**Internal:**
- `ast_nodes.py` - imports `BinaryOp`, `Expr`, `Number`

**External:**
- `functools` (standard library) - for `singledispatchmethod`
- `typing` (standard library) - for `ClassVar`

#### Rust Migration Notes

**Type Mappings:**
- `LaTeXGenerator` class → Rust struct with methods
- `BINARY_OPS: ClassVar[dict]` → Rust `const` or static HashMap
- `PRECEDENCE: ClassVar[dict]` → Rust `const` or static HashMap
- `@singledispatchmethod` → Rust match statement or similar pattern

**Pattern Changes:**
- `@singledispatchmethod` decorator → Rust pattern matching on enum variant
- Private methods with leading `_` → Rust impl block methods
- `isinstance(child, BinaryOp)` → Rust match on enum or type check
- f-string formatting → Rust format! macro or String concatenation
- Dictionary lookups → Rust match or match with .get()

**Special Handling:**
- Operator precedence must be exact (values and meaning)
- Parenthesization rules for non-commutative operators (-, /)
- LaTeX command escaping (backslash needs to be literal)
- Space formatting: exactly one space on each side of operator
- Parentheses format: `( expr )` (spaces inside)
- Math mode delimiters: `$...$` (not brackets)

#### Key Implementation Details

The LaTeX generator uses a visitor pattern to traverse the AST and generate infix notation (the opposite of input RPN).

**Precedence and Parenthesization:**
- Addition/subtraction: level 1 (lowest)
- Multiplication/division: level 2 (highest)
- Parentheses added when:
  1. Child has lower precedence than parent, OR
  2. Child is on right side of non-commutative operator (- or /) with same precedence

**Example:** `5 - 3 - 2` (left-to-right: (5-3)-2)
- Parse creates: `BinaryOp("-", BinaryOp("-", 5, 3), 2)`
- Generate: outer `-` checks right child
- Right child is also `-` with same precedence and is_right=True
- But it's on left position in the outer BinaryOp, so:
  - Left check: `-` with same precedence but is_right=False → no parens
  - Right child is just `2` (not BinaryOp) → no parens
- Result: `5 - 3 - 2` (correct!)

**Another example:** `5 3 - 2 -` (RPN for (5-3)-2)
- Same result as above

**Example with required parens:** `5 3 2 - -` (RPN for 5-(3-2))
- Parse creates: `BinaryOp("-", 5, BinaryOp("-", 3, 2))`
- Generate: outer `-` checks right child
- Right child is `-` with same precedence and is_right=True
- Operator is `-` (non-commutative) → return True
- Result: `5 - ( 3 - 2 )` (correct!)

---

### Module: cli.py

**File Location:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/cli.py`

**Purpose:** Command-line interface orchestrating the full pipeline: read input → tokenize → parse → generate → write output.

#### Public API

**Classes:** None

**Functions:**

1. **main() -> int**
   - Main entry point for rpn2tex CLI
   - **Algorithm:**
     1. Parse command-line arguments using argparse
        - `input`: positional argument, input RPN file (or "-" for stdin)
        - `-o/--output`: optional flag, output LaTeX file (default stdout)
     2. Read input text:
        - If input == "-": read from stdin
        - Else: read from file
        - Handle FileNotFoundError, PermissionError, IsADirectoryError
     3. Create ErrorFormatter with source text
     4. Process (tokenize → parse → generate):
        - Lexer(text).tokenize() → token list
        - Parser(tokens).parse() → AST
        - LaTeXGenerator().generate(ast) → LaTeX string
     5. Handle exceptions:
        - LexerError: extract message, line, column; format with ErrorFormatter
        - ParserError: extract message, token position; format with ErrorFormatter
        - Print formatted error to stderr, return 1
     6. Write output:
        - If output specified: write to file with newline appended
        - Else: print to stdout
     7. Return exit code (0 for success, 1 for error)
   - **Returns:** `int` - Exit code (0 = success, 1 = error)
   - **Error Handling:**
     - FileNotFoundError → "Error: Input file not found: {path}"
     - PermissionError (read) → "Error: Permission denied reading: {path}"
     - IsADirectoryError (read) → "Error: Expected a file, got a directory: {path}"
     - LexerError → Formatted with ErrorFormatter
     - ParserError → Formatted with ErrorFormatter
     - PermissionError (write) → "Error: Permission denied writing: {path}"
     - IsADirectoryError (write) → "Error: Cannot write to directory: {path}"
   - **Output:**
     - Success: LaTeX output to stdout or file
     - Error: Error message to stderr
     - File output adds message to stderr: `f"Generated: {args.output}"`

**Script Block:**
```python
if __name__ == "__main__":
    sys.exit(main())
```

**Constants:** None

#### Dependencies

**Internal:**
- `errors.py` - imports `ErrorFormatter`
- `latex_gen.py` - imports `LaTeXGenerator`
- `lexer.py` - imports `Lexer`, `LexerError`
- `parser.py` - imports `Parser`, `ParserError`

**External:**
- `argparse` (standard library) - for command-line argument parsing
- `sys` (standard library) - for stdin/stdout/stderr and sys.exit()
- `pathlib` (standard library) - for `Path` class

#### Rust Migration Notes

**Type Mappings:**
- `main() -> int` → Rust `fn main()` with `std::process::exit()` or similar
- `argparse.ArgumentParser` → Rust `clap` crate or manual parsing
- `Path` → Rust `std::path::Path` or `std::fs` operations
- `sys.stdin.read()` → Rust `std::io::stdin().read_to_string()`
- `sys.stderr` → Rust `eprintln!()` or `stderr().write_all()`

**Pattern Changes:**
- Exception handling (try/except) → Rust `Result<T, E>` and pattern matching
- Argument parsing library → Use `clap` for ergonomic CLI in Rust
- File I/O with exceptions → Rust `std::fs` and Result types
- Print operations → Rust `println!()`, `eprintln!()`
- Exit codes → Use `std::process::exit()` or return from main

**Special Handling:**
- Stdin detection: "-" for stdin is a common convention
- Exit code semantics (0 = success, 1 = error)
- Error messages to stderr (not stdout)
- File operations with proper error handling
- Text reading/writing (UTF-8 by default)
- Pipeline orchestration with clear error points

#### Key Implementation Details

The CLI is the application's entry point and orchestrates the full processing pipeline.

**Pipeline Architecture:**
1. **Input Reading** → Plain text (from file or stdin)
2. **Lexer** → Token stream
3. **Parser** → AST
4. **LaTeX Generator** → LaTeX string
5. **Output Writing** → File or stdout

**Error Handling Strategy:**
- File I/O errors are caught early (before processing)
- Lexer errors are caught and formatted
- Parser errors are caught and formatted
- All errors go to stderr with formatted context
- Exit code 1 on any error

**Output Conventions:**
- Successful LaTeX output to stdout (unless -o specified)
- File writes append newline
- Status messages go to stderr
- Error messages formatted with source context

**Example usage:**
```bash
rpn2tex input.rpn                 # Read from file, output to stdout
rpn2tex input.rpn -o output.tex   # Read from file, write to file
echo "5 3 +" | rpn2tex -          # Read from stdin, output to stdout
```

---

## Cross-Cutting Concerns

### Error Propagation Strategy

**Error Handling Flow:**
1. **Lexer** raises `LexerError(message, line, column)`
2. **Parser** raises `ParserError(message, token)`
3. **CLI** catches both exception types
4. **ErrorFormatter** formats exception context
5. **Stderr** receives formatted error
6. **Exit code 1** returned

**Error Message Format:**
- Simple error header: `Error: {message}`
- Source context with line numbers and caret
- Line format: `{line_num} | {source_line}`
- Caret line: spaces to column, then `^`

**Key principle:** Errors must be caught at the top level (CLI) and formatted uniformly.

### String Handling Approach

**Python vs Rust:**
- Numbers: always stored and passed as strings (never parsed to f64)
- Operators: small fixed set ("+", "-", "*", "/")
- LaTeX output: string concatenation with formatting
- Source code: kept as strings for error reporting

**Rust considerations:**
- `String` for owned strings (file contents, tokens)
- `&str` for borrowed slices (operator matching, token values)
- Consider `Cow<str>` for efficiency where appropriate

### Numeric Precision Requirements

**Key principle:** Numbers are NEVER evaluated, only stored and passed through.

- Input: `-5`, `3.14`, `42` → stored as string
- Processing: Tokens carry value as string
- AST: Number nodes store value as string
- Output: String value passed directly to LaTeX
- No arithmetic evaluation needed

**Implication:** Rust doesn't need f64 or i64 types for number storage, only String.

### Testing Strategy

**Test inputs focus on:**
1. **Valid RPN:** All 18+ success cases from I/O contract
2. **Operator precedence:** (5+3)*2 vs 5*3+2 vs 5+3*4
3. **Left-associativity:** 5-3-2 and 10/2/5
4. **Error cases:** Invalid characters, missing operators, empty input
5. **Floating-point:** 3.14, 1.5, 0.5
6. **Negative numbers:** -5, -3.14

**Validation:**
- LaTeX output must match exactly (including spaces and parentheses)
- Error messages must match exact wording
- Exit codes must be correct
- Position information (line, column) must be preserved

---

## Migration Order Justification

### Recommended Migration Order

**Phase 1: Foundation Types**
1. **tokens.py** - No dependencies, foundational
   - Simple enum and dataclass
   - Must exist before lexer

**Phase 2: Error Infrastructure**
2. **errors.py** - No dependencies
   - Error formatting logic
   - Optional but useful early

**Phase 3: Basic Processing**
3. **ast_nodes.py** - Depends on tokens.py (indirectly via type imports)
   - AST structures
   - Required by parser and lexer

4. **lexer.py** - Depends on tokens.py
   - Tokenization logic
   - Required by parser
   - Can test in isolation with unit tests

5. **parser.py** - Depends on tokens.py and ast_nodes.py
   - RPN parsing algorithm
   - Core logic of application

**Phase 4: Output Generation**
6. **latex_gen.py** - Depends on ast_nodes.py
   - LaTeX generation
   - Can test separately with pre-built ASTs

**Phase 5: Integration**
7. **cli.py** - Depends on all modules
   - Entry point and orchestration
   - Last to implement, ties everything together

### Dependency Justification

- **tokens.py first:** Lightweight, no dependencies, widely used
- **ast_nodes.py early:** Required by parser and generator
- **lexer and parser together:** Form the core processing chain
- **latex_gen separate:** Can be tested independently
- **cli last:** Needs all other modules working

### Testing Strategy During Migration

1. **Tokens module:** Direct unit tests on Token creation
2. **AST module:** Unit tests on node creation
3. **Lexer:** Unit tests with example inputs, verify token stream
4. **Parser:** Unit tests with pre-made token lists, verify AST structure
5. **LaTeX Gen:** Unit tests with pre-made ASTs, verify output string
6. **CLI:** Integration tests with file I/O

---

## Summary and Migration Strategy

### Key Files and Code Snippets

**tokens.py - TokenType enum:**
```python
class TokenType(Enum):
    NUMBER = auto()
    PLUS = auto()
    MINUS = auto()
    MULT = auto()
    DIV = auto()
    EOF = auto()
```

**tokens.py - Token dataclass:**
```python
@dataclass(frozen=True)
class Token:
    type: TokenType
    value: str
    line: int
    column: int
```

**ast_nodes.py - Core classes:**
```python
@dataclass(frozen=True)
class ASTNode:
    line: int
    column: int

@dataclass(frozen=True)
class Number(ASTNode):
    value: str

@dataclass(frozen=True)
class BinaryOp(ASTNode):
    operator: str
    left: Expr
    right: Expr

Expr = Number | BinaryOp
```

**lexer.py - Key algorithm (tokenize):**
```python
def tokenize(self) -> list[Token]:
    tokens: list[Token] = []
    while not self._at_end():
        self._skip_whitespace()
        if self._at_end():
            break
        tokens.append(self._scan_token())
    tokens.append(Token(TokenType.EOF, "", self.line, self.column))
    return tokens
```

**parser.py - Core algorithm (parse):**
```python
def parse(self) -> Expr:
    stack: list[Expr] = []
    while not self._at_end():
        token: Token = self._current()
        if token.type == TokenType.NUMBER:
            num_node = Number(...)
            stack.append(num_node)
            self._advance()
        elif token.type in (TokenType.PLUS, TokenType.MINUS, ...):
            right = stack.pop()
            left = stack.pop()
            op_node = BinaryOp(...)
            stack.append(op_node)
            self._advance()
        elif token.type == TokenType.EOF:
            break
```

**latex_gen.py - Precedence and parenthesization:**
```python
PRECEDENCE: ClassVar[dict[str, int]] = {
    "+": 1, "-": 1,  # Addition/subtraction (lower)
    "*": 2, "/": 2,  # Multiplication/division (higher)
}

def _needs_parens(self, child: Expr, parent_precedence: int, *, is_right: bool) -> bool:
    if not isinstance(child, BinaryOp):
        return False
    child_precedence = self.PRECEDENCE[child.operator]
    if child_precedence < parent_precedence:
        return True
    return (
        child_precedence == parent_precedence
        and is_right
        and child.operator in ("-", "/")
    )
```

### Critical Invariants to Preserve

1. **Numbers are strings:** No numeric evaluation
2. **1-based line/column:** Position tracking must be exact
3. **Exact spacing in LaTeX:** Single space around operators
4. **Parentheses with spaces:** `( expr )` not `(expr)`
5. **Left-associativity:** Handle - and / specially
6. **Error message wording:** Must match exactly
7. **Exit codes:** 0 for success, 1 for error
8. **Stack-based RPN:** Right operand popped first

### Rust-Specific Patterns

1. **Enum for TokenType:** Direct match on variants
2. **Struct for Token:** Use `Copy` or `Clone` for efficiency
3. **Box<Expr> for recursion:** Heap allocation for AST nodes
4. **Result<T, E> for errors:** Rust error handling
5. **match expressions:** Replace Python's isinstance checks
6. **Pattern matching:** For visitor dispatch in latex_gen

---

## Document Metadata

**Analysis Scope:** All 7 modules in rpn2tex Python implementation
**Total Lines of Python Code:** ~600 lines
**Analysis Methodology:** Line-by-line code review with semantic understanding
**Output Format:** Markdown specification for Rust migration team
**Status:** Complete and ready for implementation phase

**File References:**
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/tokens.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/ast_nodes.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/errors.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/lexer.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/parser.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/latex_gen.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/cli.py`
- `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/migrations/rust-module-by-module-3/artifacts/PHASE_0_IO_CONTRACT.md`
