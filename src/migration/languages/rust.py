"""Rust target language configuration."""

from .base import LanguageTarget


class RustTarget(LanguageTarget):
    """Rust-specific configuration for the migration framework."""

    @property
    def name(self) -> str:
        return "rust"

    @property
    def file_extension(self) -> str:
        return ".rs"

    def get_project_subdir(self, project_name: str) -> str:
        return f"{project_name}-rs"

    def get_quality_gates(self) -> list[str]:
        return [
            "cargo check && cargo clippy -- -D warnings",
            "cargo fmt --check",
            "cargo test",
        ]

    def get_migrator_idioms(self) -> str:
        return """
## RUST IDIOM REQUIREMENTS

Apply these patterns to pass clippy on first attempt:

1. **Attributes**:
   - `#[must_use]` on all public functions returning values
   - `#[derive(Debug, Clone, PartialEq, Eq)]` on structs/enums

2. **Documentation**:
   - `//!` module-level doc comments
   - `///` doc comments on all public items
   - Include `# Examples` in doc comments

3. **Function signatures**:
   - Use `impl Into<String>` or `&str` for string params
   - Prefer `&self` over `self` for non-consuming methods
   - Return `Self` from constructors

4. **Code style**:
   - Use `Self::` in impl blocks
   - Use `matches!()` for boolean matches
   - Prefer iterators over explicit loops
   - Avoid unnecessary `.clone()`

5. **Error handling**:
   - Implement `std::error::Error` and `Display` for error types
   - Use `Result<T, E>` for fallible operations
   - Use `Option<T>` for optional values

6. **Testing (REQUIRED)**:
   - Write unit tests in `#[cfg(test)]` modules within each source file
   - Test all public functions
   - Use `#[test]` attribute for test functions
   - Include doc tests with `///` examples that are runnable

7. **Coverage setup**:
   - Install cargo-llvm-cov: `cargo install cargo-llvm-cov`
   - Ensure llvm-tools-preview is installed: `rustup component add llvm-tools-preview`
   - Test coverage can be measured with: `cargo llvm-cov --text`
"""

    def get_reviewer_checks(self) -> str:
        return """
### Rust-Specific Checks
- Proper Result/Option usage
- No unnecessary unwrap() or expect()
- Correct ownership/borrowing patterns
- No unnecessary clones
- Proper lifetime annotations where needed
- Error types implement std::error::Error
"""

    def get_file_mapping(self, python_file: str) -> str:
        mappings = {
            "tokens.py": "tokens.rs",
            "ast_nodes.py": "ast.rs",
            "errors.py": "error.rs",
            "lexer.py": "lexer.rs",
            "parser.py": "parser.rs",
            "latex_gen.py": "latex.rs",
            "cli.py": "main.rs",
            "__init__.py": "lib.rs",
        }
        return mappings.get(python_file, python_file.replace(".py", ".rs"))

    def get_project_init_commands(self, project_dir: str) -> list[str]:
        return [
            f"cd {project_dir} && cargo init --name rpn2tex",
        ]

    def get_source_dir(self, project_dir: str) -> str:
        return f"{project_dir}/src"

    def get_coverage_command(self, project_dir: str) -> str:
        # Use cargo-llvm-cov with explicit paths to llvm tools
        # Falls back to tarpaulin if llvm-cov unavailable
        return f"""cd {project_dir} && (
            LLVM_COV=$HOME/.rustup/toolchains/stable-*/lib/rustlib/*/bin/llvm-cov
            LLVM_PROFDATA=$HOME/.rustup/toolchains/stable-*/lib/rustlib/*/bin/llvm-profdata
            export LLVM_COV=$(ls $LLVM_COV 2>/dev/null | head -1)
            export LLVM_PROFDATA=$(ls $LLVM_PROFDATA 2>/dev/null | head -1)
            cargo llvm-cov 2>/dev/null || cargo tarpaulin --out Stdout 2>/dev/null || echo 'coverage tool not available'
        )"""

    def parse_coverage_output(self, output: str) -> float | None:
        import re
        # cargo-llvm-cov format: "TOTAL  XX.XX%"
        # tarpaulin format: "XX.XX% coverage"
        patterns = [
            r"TOTAL\s+[\d.]+%\s+[\d.]+%\s+([\d.]+)%",  # llvm-cov
            r"(\d+\.?\d*)%\s*coverage",  # tarpaulin
        ]
        for pattern in patterns:
            match = re.search(pattern, output)
            if match:
                return float(match.group(1))
        return None
