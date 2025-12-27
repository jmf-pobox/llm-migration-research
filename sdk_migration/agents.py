"""Subagent definitions for the Claude Agent SDK migration framework.

Version 3: Multi-Phase Orchestration (Option B)

Key design:
- Phase 1: Analyst reads ALL Python files ONCE, produces comprehensive migration spec
- Phase 2: Migrators receive analysis summary (not raw source), read only Rust files
- Phase 3: Reviewers compare using analysis + Rust files

This keeps context small per subagent by avoiding redundant source embedding.
"""

# Absolute paths to eliminate file discovery overhead
SOURCE_DIR = "/Users/jfreeman/Coding/rpn2tex/src/rpn2tex"
TARGET_DIR = "/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/rpn2tex-rs/src"
PROJECT_DIR = "/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/rpn2tex-rs"

# Explicit file manifest - no searching required
SOURCE_FILES = {
    "tokens.py": f"{SOURCE_DIR}/tokens.py",
    "ast_nodes.py": f"{SOURCE_DIR}/ast_nodes.py",
    "errors.py": f"{SOURCE_DIR}/errors.py",
    "lexer.py": f"{SOURCE_DIR}/lexer.py",
    "parser.py": f"{SOURCE_DIR}/parser.py",
    "latex_gen.py": f"{SOURCE_DIR}/latex_gen.py",
    "cli.py": f"{SOURCE_DIR}/cli.py",
}

TARGET_FILES = {
    "tokens.rs": f"{TARGET_DIR}/tokens.rs",
    "ast.rs": f"{TARGET_DIR}/ast.rs",
    "error.rs": f"{TARGET_DIR}/error.rs",
    "lexer.rs": f"{TARGET_DIR}/lexer.rs",
    "parser.rs": f"{TARGET_DIR}/parser.rs",
    "latex.rs": f"{TARGET_DIR}/latex.rs",
    "main.rs": f"{TARGET_DIR}/main.rs",
    "lib.rs": f"{TARGET_DIR}/lib.rs",
}

# Phase 1: Comprehensive Analysis Agent
# Reads ALL Python files once, produces detailed migration spec
ANALYST_AGENT = {
    "description": "Phase 1: Analyzes ALL Python modules and produces comprehensive migration specification.",
    "prompt": f"""You are a source code analyst specializing in Python-to-Rust migrations.

## Your Task: Comprehensive Codebase Analysis

Read and analyze ALL Python source files to produce a migration specification document.

### Source Files to Read (read ALL of these):
{chr(10).join(f'  - {v}' for v in SOURCE_FILES.values())}

### Output Format

Produce a structured analysis for EACH module:

```
## Module: <filename>
### Public API
- Classes: <list with method signatures>
- Functions: <list with signatures>
- Constants: <list>

### Dependencies
- Internal: <which other modules it imports>
- External: <third-party packages>

### Rust Migration Notes
- Type mappings: <Python type -> Rust type>
- Pattern changes: <inheritance -> traits, etc.>
- Special handling: <any tricky parts>

### Key Implementation Details
<Brief summary of core logic that must be preserved>
```

### Important
- Read each file completely
- Focus on PUBLIC APIs that must be preserved
- Document dependencies to ensure correct migration order
- Note any Python patterns that need special Rust handling
- Be thorough but concise - this spec will guide all migrations""",
    "tools": ["Read", "Glob", "Grep"],
    "model": "haiku"
}

# Phase 2: Migration Agent
# Receives analysis, reads only Rust files for API reference
MIGRATOR_AGENT = {
    "description": "Phase 2: Migrates a single Python module to Rust using the analysis specification.",
    "prompt": f"""You are a code migration specialist converting Python to idiomatic Rust.

## Context
You will receive:
1. A migration specification (from Phase 1 analysis) describing the module's API and logic
2. The specific module to migrate

## Your Task
1. Use the provided analysis specification (NOT raw Python source)
2. Read any previously migrated Rust modules at these paths for API reference:
{chr(10).join(f'   - {v}' for v in TARGET_FILES.values())}
3. Generate idiomatic Rust code
4. Write to the exact target path specified
5. Verify with cargo check && cargo clippy

## Target Paths
{chr(10).join(f'  - {k}: {v}' for k, v in TARGET_FILES.items())}

Project directory: {PROJECT_DIR}

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

## Verification Commands

```bash
cd {PROJECT_DIR} && cargo check && cargo clippy -- -D warnings
cd {PROJECT_DIR} && cargo fmt && cargo test
```

Only report success when ALL quality gates pass.""",
    "tools": ["Read", "Write", "Edit", "Bash", "Glob", "Grep"],
    "model": "sonnet"
}

# Phase 3: Review Agent
# Compares migrated Rust against analysis spec
REVIEWER_AGENT = {
    "description": "Phase 3: Reviews migrated Rust code against the analysis specification.",
    "prompt": f"""You are a code review specialist validating Python-to-Rust migrations.

## Context
You will receive:
1. The analysis specification describing expected API and behavior
2. The specific Rust file to review

## Your Task
1. Read the migrated Rust file at the specified path
2. Compare against the analysis specification
3. Verify:
   - All public APIs are preserved
   - Behavior matches the specification
   - Edge cases are handled
4. Check Rust idioms:
   - Proper Result/Option usage
   - No unnecessary unwrap()
   - Correct ownership/borrowing

## Target Files to Review
{chr(10).join(f'  - {v}' for v in TARGET_FILES.values())}

## Output Format
```
## Review: <module name>

### API Completeness
- [x] or [ ] <each public item from spec>

### Behavioral Correctness
<any concerns about logic>

### Rust Idioms
<any style issues>

### Verdict
PASS / FAIL with summary
```

Be critical but constructive. Focus on correctness first.""",
    "tools": ["Read", "Glob", "Grep"],
    "model": "haiku"
}

# Migration configuration
MIGRATION_CONFIG = {
    "source_dir": SOURCE_DIR,
    "target_dir": TARGET_DIR,
    "project_dir": PROJECT_DIR,
    "modules": [
        {"python": "tokens.py", "rust": "tokens.rs", "phase": "core"},
        {"python": "ast_nodes.py", "rust": "ast.rs", "phase": "core"},
        {"python": "errors.py", "rust": "error.rs", "phase": "core"},
        {"python": "lexer.py", "rust": "lexer.rs", "phase": "pipeline"},
        {"python": "parser.py", "rust": "parser.rs", "phase": "pipeline"},
        {"python": "latex_gen.py", "rust": "latex.rs", "phase": "pipeline"},
        {"python": "cli.py", "rust": "main.rs", "phase": "cli"},
    ],
    "source_files": SOURCE_FILES,
    "target_files": TARGET_FILES,
    "quality_gates": [
        "cargo check && cargo clippy -- -D warnings",
        "cargo fmt",
        "cargo test",
    ]
}
