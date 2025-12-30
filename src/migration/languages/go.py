"""Go target language configuration."""

from .base import LanguageTarget


class GoTarget(LanguageTarget):
    """Go-specific configuration for the migration framework."""

    @property
    def name(self) -> str:
        return "go"

    @property
    def file_extension(self) -> str:
        return ".go"

    def get_project_subdir(self, project_name: str) -> str:
        return f"{project_name}-go"

    def get_quality_gates(self) -> list[str]:
        return [
            "go build ./...",
            "go vet ./...",
            "gofmt -l . | xargs -r test -z",  # Fails if any files need formatting
            "go test ./...",
        ]

    def get_migrator_idioms(self) -> str:
        return """
## GO IDIOM REQUIREMENTS

Apply these patterns for idiomatic Go:

1. **Package structure**:
   - Use `cmd/rpn2tex/main.go` for the CLI entry point
   - Use `internal/` or root package for library code
   - Package name matches directory name
   - One package per directory

2. **Naming conventions**:
   - PascalCase for exported (public) identifiers
   - camelCase for unexported (private) identifiers
   - Short, concise names (prefer `r` over `reader` in small scopes)
   - Acronyms stay uppercase: `HTTP`, `ID`, `URL`

3. **Error handling**:
   - Return `error` as last return value
   - Check errors immediately: `if err != nil { return err }`
   - Create custom error types with `errors.New()` or custom structs
   - Wrap errors with context: `fmt.Errorf("parsing token: %w", err)`
   - Never ignore errors silently

4. **Documentation**:
   - Doc comments start with the identifier name
   - `// Token represents a lexical token.`
   - Package comment in `doc.go` or first file

5. **Code style**:
   - Use `gofmt` formatting (tabs, not spaces)
   - Prefer early returns over deep nesting
   - Use named return values sparingly (only for documentation)
   - Group related declarations with `var ()` or `const ()`

6. **Interfaces**:
   - Define interfaces where they're used, not where implemented
   - Keep interfaces small (1-3 methods)
   - Use `io.Reader`, `io.Writer`, `fmt.Stringer` where applicable

7. **Slices and maps**:
   - Use `make()` for slices with known capacity
   - Check map membership: `val, ok := m[key]`
   - Prefer `nil` slices over empty slices

8. **Testing (REQUIRED)**:
   - Create `foo_test.go` for each `foo.go` source file
   - Use table-driven tests with `t.Run()`
   - Test all exported functions
   - Use `testing.T` for tests, `testing.B` for benchmarks
   - Coverage measured with: `go test -cover ./...`
"""

    def get_reviewer_checks(self) -> str:
        return """
### Go-Specific Checks
- All errors checked (no ignored error returns)
- Errors wrapped with context using %w
- No unused variables or imports
- Proper use of defer for cleanup
- No data races (consider -race flag)
- Interfaces defined at point of use
- Exported identifiers have doc comments
- No naked returns in long functions
"""

    def get_file_mapping(self, python_file: str) -> str:
        mappings = {
            "tokens.py": "token.go",
            "ast_nodes.py": "ast.go",
            "errors.py": "errors.go",
            "lexer.py": "lexer.go",
            "parser.py": "parser.go",
            "latex_gen.py": "latex.go",
            "cli.py": "cmd/rpn2tex/main.go",
            "__init__.py": "rpn2tex.go",
        }
        return mappings.get(python_file, python_file.replace(".py", ".go"))

    def get_project_init_commands(self, project_dir: str) -> list[str]:
        return [
            f"cd {project_dir} && go mod init rpn2tex",
            f"mkdir -p {project_dir}/cmd/rpn2tex",
        ]

    def get_source_dir(self, project_dir: str) -> str:
        return project_dir  # Go uses root directory for package

    def get_coverage_command(self, project_dir: str) -> str:
        # Go has built-in coverage support
        return f"cd {project_dir} && go test -cover ./... 2>&1"

    def parse_coverage_output(self, output: str) -> float | None:
        import re

        # Go test -cover output: "coverage: 92.5% of statements"
        # May have multiple packages, we want the overall or average
        matches = re.findall(r"coverage:\s*([\d.]+)%\s*of\s*statements", output)
        if matches:
            # Return the average if multiple packages
            coverages = [float(m) for m in matches]
            return sum(coverages) / len(coverages)
        return None
