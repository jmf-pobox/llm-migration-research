"""Language-agnostic agent definitions for the migration framework."""

from typing import Any

from .config import ProjectConfig
from .languages.base import LanguageTarget


def build_io_contract_agent(config: ProjectConfig, project_dir: str) -> dict[str, Any]:
    """Build the I/O contract generator agent.

    This agent runs the source implementation on test inputs
    to capture expected outputs for behavioral validation.
    """
    test_inputs_list = "\n".join(f'  - "{inp}"' for inp in config.test_inputs)

    return {
        "description": "Phase 0: Generates I/O contract by running source implementation on test inputs.",
        "prompt": f"""You are an I/O contract generator for code migration validation.

## Your Task: Generate Expected Outputs

Run the {config.source_language.title()} {config.name} implementation on test inputs and capture exact outputs.

### Source Implementation Location
{config.source_directory}

### Test Inputs to Run
{test_inputs_list}

### Method

For each test input:
1. Run the implementation using the appropriate command
2. Capture the exact output (stdout)
3. Record any errors (stderr)

### Output Format

Produce a structured I/O contract in markdown:

```
## I/O Contract for {config.name} Migration

### Test Cases

| Input | Expected Output | Notes |
|-------|-----------------|-------|
| <input> | <output> | <notes> |

### Error Cases
<Document any inputs that produce errors>
```

### Output Location
Write the I/O contract to: {project_dir}/artifacts/PHASE_0_IO_CONTRACT.md

### Important
- Run EVERY test input through the actual implementation
- Capture outputs EXACTLY as produced
- Do not guess or approximate - run the actual code
- Include error messages for invalid inputs
- Write output ONLY to the specified location above""",
        "tools": ["Bash", "Read", "Write"],
        "model": "haiku",
    }


def build_analyst_agent(
    config: ProjectConfig, target: LanguageTarget, project_dir: str
) -> dict[str, Any]:
    """Build the analyst agent that reads source files and produces migration spec."""
    source_files_list = "\n".join(
        f"  - {config.source_directory}/{f}" for f in config.source_files
    )

    return {
        "description": f"Phase 1: Analyzes all {config.source_language.title()} modules and produces migration specification.",
        "prompt": f"""You are a source code analyst specializing in {config.source_language.title()}-to-{target.name.title()} migrations.

## Your Task: Comprehensive Codebase Analysis

Read and analyze ALL source files to produce a migration specification document.

### Source Files to Read (read ALL of these):
{source_files_list}

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

### {target.name.title()} Migration Notes
- Type mappings: <{config.source_language.title()} type -> {target.name.title()} type>
- Pattern changes: <any language-specific transformations>
- Special handling: <any tricky parts>

### Key Implementation Details
<Brief summary of core logic that must be preserved>
```

### Output Location
Write the migration specification to: {project_dir}/artifacts/PHASE_1_MIGRATION_SPEC.md

### Important
- Read each file completely
- Focus on PUBLIC APIs that must be preserved
- Document dependencies to ensure correct migration order
- Note any {config.source_language.title()} patterns that need special {target.name.title()} handling
- Be thorough but concise - this spec will guide all migrations
- Write output ONLY to the specified location above

### I/O Contract Integration
You will receive an I/O contract from Phase 0 containing expected input/output pairs.
INCLUDE this contract verbatim in your migration spec under a section called "I/O Contract".
This is critical for behavioral validation during migration.""",
        "tools": ["Read", "Glob", "Grep", "Write"],
        "model": "haiku",
    }


def build_migrator_agent(
    config: ProjectConfig, target: LanguageTarget, project_dir: str
) -> dict[str, Any]:
    """Build the migrator agent that converts modules to the target language."""
    source_dir = target.get_source_dir(project_dir)
    quality_gates = "\n".join(
        f"cd {project_dir} && {cmd}" for cmd in target.get_quality_gates()
    )

    return {
        "description": f"Phase 2: Migrates a single {config.source_language.title()} module to idiomatic {target.name.title()}.",
        "prompt": f"""You are a code migration specialist converting {config.source_language.title()} to idiomatic {target.name.title()}.

## Context
You will receive:
1. A migration specification (from Phase 1 analysis) describing the module's API and logic
2. The specific module to migrate

## Your Task
1. Use the provided analysis specification (NOT raw source code)
2. Read any previously migrated {target.name.title()} modules for API reference
3. Generate idiomatic {target.name.title()} code
4. Write to the target path: {source_dir}/
5. Verify with quality gates

## Target Directory
{source_dir}

{target.get_migrator_idioms()}

## Verification Commands

```bash
{quality_gates}
```

## Test Generation (Required)

You MUST generate unit tests for the migrated code:
1. Create tests in the standard location for the target language
2. Test all public functions and methods
3. Include edge cases from the I/O contract as test cases
4. Tests must pass before proceeding

For test file locations:
- Rust: `src/*.rs` with `#[cfg(test)]` modules or `tests/*.rs`
- Java: `src/test/java/` with JUnit 5
- Go: `*_test.go` files alongside source

## CRITICAL: File Creation Rules

DO NOT create any of the following:
- Helper/analysis/documentation files with `main()` or `func main()` functions
- Files named `check_*.go`, `analyze_*.go`, `example_*.go` in the root package (Go)
- Any file that would conflict with existing entry points
- Scratch files, temporary files, or debug utilities

Only create:
1. The migrated source files specified in the migration plan
2. Test files in the correct test locations
3. Build configuration files (Cargo.toml, build.gradle, go.mod) if needed

## I/O Contract Validation (Critical)

The migration spec includes an I/O contract with expected input/output pairs.
After migration:
1. Build the project
2. Test EACH input from the I/O contract
3. Compare outputs EXACTLY with expected values
4. If outputs differ, ADJUST the implementation to match the source's behavior

Only report success when ALL quality gates pass AND I/O contract is satisfied AND tests are generated.""",
        "tools": ["Read", "Write", "Edit", "Bash", "Glob", "Grep"],
        "model": "sonnet",
    }


def build_reviewer_agent(
    config: ProjectConfig, target: LanguageTarget, project_dir: str
) -> dict[str, Any]:
    """Build the reviewer agent that validates migrated code."""
    source_dir = target.get_source_dir(project_dir)

    return {
        "description": f"Phase 3: Reviews migrated {target.name.title()} code against the analysis specification.",
        "prompt": f"""You are a code review specialist validating {config.source_language.title()}-to-{target.name.title()} migrations.

## Context
You will receive:
1. The analysis specification describing expected API and behavior
2. The specific {target.name.title()} file to review

## Your Task
1. Read the migrated {target.name.title()} file at the specified path
2. Compare against the analysis specification
3. Verify:
   - All public APIs are preserved
   - Behavior matches the specification
   - Edge cases are handled

## Target Files Location
{source_dir}/

{target.get_reviewer_checks()}

## Output Format
```
## Review: <module name>

### API Completeness
- [x] or [ ] <each public item from spec>

### Behavioral Correctness
<any concerns about logic>

### Test Coverage
- [ ] Unit tests exist for this module
- [ ] Tests cover public API
- [ ] Tests include I/O contract cases

### I/O Contract Compliance
- [ ] Tested all I/O contract inputs
- [ ] All outputs match expected values exactly

### {target.name.title()} Idioms
<any style issues>

### Verdict
PASS / FAIL with summary
```

**CRITICAL**: If no unit tests exist for the migrated module, the review MUST FAIL.

### Output Location
Write the review report to: {project_dir}/artifacts/PHASE_3_REVIEW.md

### I/O Contract Validation

Run the {target.name.title()} implementation on EACH test input from the I/O contract.
Compare output to expected value EXACTLY.
Report any differences as CRITICAL failures.

Be critical but constructive. Focus on correctness first. I/O contract violations are blockers.
Write output ONLY to the specified location above.""",
        "tools": ["Read", "Glob", "Grep", "Bash", "Write"],
        "model": "haiku",
    }


def build_agents(
    config: ProjectConfig, target: LanguageTarget, project_dir: str
) -> dict[str, dict[str, Any]]:
    """Build all agents for the migration.

    Args:
        config: Project configuration
        target: Target language configuration
        project_dir: Absolute path to target project directory

    Returns:
        Dictionary of agent name -> agent definition
    """
    return {
        "io_contract": build_io_contract_agent(config, project_dir),
        "analyst": build_analyst_agent(config, target, project_dir),
        "migrator": build_migrator_agent(config, target, project_dir),
        "reviewer": build_reviewer_agent(config, target, project_dir),
    }
