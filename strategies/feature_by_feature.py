"""Feature-by-feature migration strategy.

Instead of migrating module-by-module (lexer.py → lexer.rs), this strategy
migrates feature-by-feature:

1. Pick a feature (e.g., "addition operator")
2. Migrate the slices of lexer, parser, and generator that handle this feature
3. Validate with I/O contract for just this feature
4. Repeat for next feature

This mirrors how codebases are originally built - incrementally adding capabilities.
"""

from strategies.base import MigrationStrategy, MigrationSlice
from framework.config import ProjectConfig
from languages.base import LanguageTarget


class FeatureByFeatureStrategy(MigrationStrategy):
    """Migrate one feature at a time across all modules."""

    name = "feature-by-feature"

    def get_slices(self, config: ProjectConfig) -> list[MigrationSlice]:
        """Each feature is a separate slice (may touch multiple files)."""
        if not hasattr(config, "features") or not config.features:
            raise ValueError(
                f"Project {config.name} does not define features. "
                "Add a 'features' section to config.yaml to use feature-by-feature strategy."
            )

        return [
            MigrationSlice(
                name=feature.name,
                description=feature.description,
                source_files=feature.touches,
                test_inputs=[
                    {"input": tc.input, "output": tc.output}
                    for tc in feature.test_cases
                ],
            )
            for feature in config.features
        ]

    def get_prompt(
        self,
        config: ProjectConfig,
        target: LanguageTarget,
        project_dir: str,
    ) -> str:
        """Generate feature-by-feature migration prompt."""
        features_list = "\n".join(
            f"    {i+1}. {f.name}: {f.description}"
            for i, f in enumerate(config.features)
        )

        feature_details = []
        for f in config.features:
            touches = ", ".join(f.touches)
            depends = ", ".join(f.depends_on) if f.depends_on else "none"
            tests = "\n".join(
                f'        - "{tc.input}" → "{tc.output}"' for tc in f.test_cases
            )
            feature_details.append(
                f"""### {f.name}
    Description: {f.description}
    Files touched: {touches}
    Dependencies: {depends}
    Test cases:
{tests}"""
            )

        feature_sections = "\n\n".join(feature_details)

        source_files_list = "\n".join(
            f"    - {f}: {config.source_directory}/{f}" for f in config.source_files
        )

        target_dir = target.get_source_dir(project_dir)
        target_files_list = "\n".join(
            f"    - {target.get_file_mapping(f)}: {target_dir}/{target.get_file_mapping(f)}"
            for f in config.source_files
        )

        quality_gates = "\n".join(
            f"- `{cmd}`" for cmd in target.get_quality_gates()
        )

        return f"""
Migrate the {config.name} {config.source_language.title()} codebase to {target.name.title()} using FEATURE-BY-FEATURE migration with I/O validation.

## IMPORTANT: Feature-by-Feature Approach

Instead of migrating one module at a time, migrate one FEATURE at a time. Each feature cuts across multiple modules (lexer, parser, generator).

This approach:
1. Validates correctness incrementally (each feature has its own I/O contract)
2. Isolates complexity (don't need entire module in context)
3. Mirrors how the codebase was built

## Features to Migrate (in dependency order):
{features_list}

## Feature Details:

{feature_sections}

## Multi-Phase Orchestration

### PHASE 0: I/O Contract Verification

Before migration, verify the test cases are correct by running them against the {config.source_language.title()} implementation.

### PHASE 1: Comprehensive Analysis

Spawn the **analyst** agent ONCE to:
- Read all source files
- Understand the feature boundaries
- Produce a migration specification organized by FEATURE (not by module)

### PHASE 2: Feature-by-Feature Migration

For EACH feature in dependency order:
1. Spawn the **migrator** agent with:
   - The feature specification from the analyst
   - The feature's test cases (I/O contract)
2. Migrator writes/updates the relevant {target.name.title()} files
3. Migrator validates that feature's test cases pass
4. Quality gates must pass before next feature

### PHASE 3: Feature-by-Feature Review

After EACH feature migration:
1. Spawn the **reviewer** agent with:
   - The feature specification
   - The {target.name.title()} files that were modified
2. Reviewer verifies correctness for this feature

## Source Files:
{source_files_list}

## Target Files:
{target_files_list}

## Project Directory:
    {project_dir}

## Quality Gates (MUST pass after each feature):
{quality_gates}
- Feature's test cases must produce EXACT output

## Key Points:
1. Migrate features in dependency order (later features depend on earlier ones)
2. Each feature has its own I/O contract - validate before proceeding
3. Target files will be built up incrementally across features
4. A feature is complete when its test cases pass AND quality gates pass

Begin with Phase 0: verify the I/O contract by running test cases against {config.source_language.title()}.
"""
