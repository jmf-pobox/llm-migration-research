"""Module-by-module migration strategy.

This is the original approach: migrate each Python module to a corresponding
target language module in dependency order.
"""

from .base import MigrationStrategy, MigrationSlice
from ..config import ProjectConfig
from ..languages.base import LanguageTarget


class ModuleByModuleStrategy(MigrationStrategy):
    """Migrate one module at a time in dependency order."""

    name = "module-by-module"

    def get_slices(self, config: ProjectConfig) -> list[MigrationSlice]:
        """Each module is a separate slice."""
        return [
            MigrationSlice(
                name=module.name,
                description=module.description,
                source_files=[module.source],
                test_inputs=[
                    {"input": inp, "output": ""}  # Output captured at runtime
                    for inp in config.test_inputs
                ],
            )
            for module in config.modules
        ]

    def get_prompt(
        self,
        config: ProjectConfig,
        target: LanguageTarget,
        project_dir: str,
    ) -> str:
        """Generate module-by-module migration prompt."""
        modules_list = "\n".join(
            f"    {i+1}. {m.source} -> {target.get_file_mapping(m.source)} ({m.phase} phase)"
            for i, m in enumerate(config.modules)
        )

        source_files_list = "\n".join(
            f"    - {f}: {config.source_directory}/{f}" for f in config.source_files
        )

        target_dir = target.get_source_dir(project_dir)
        target_files_list = "\n".join(
            f"    - {target.get_file_mapping(f)}: {target_dir}/{target.get_file_mapping(f)}"
            for f in config.source_files
        )

        test_inputs_list = "\n".join(f'    - "{inp}"' for inp in config.test_inputs)

        quality_gates = "\n".join(
            f"- `{cmd}`" for cmd in target.get_quality_gates()
        )

        return f"""
Migrate the {config.name} {config.source_language.title()} codebase to {target.name.title()} using a multi-phase approach with I/O validation.

## IMPORTANT: Multi-Phase Orchestration with I/O Contract

This migration uses FOUR distinct phases to ensure behavioral equivalence:

### PHASE 0: I/O Contract Generation (DO THIS FIRST)

Spawn the **io_contract** agent ONCE to:
1. Run the {config.source_language.title()} implementation on curated test inputs
2. Capture EXACT outputs for each input
3. Produce an I/O contract document

Test inputs to run:
{test_inputs_list}

SAVE the I/O contract - you will include it in the migration spec.

### PHASE 1: Comprehensive Analysis

Spawn the **analyst** agent ONCE to analyze ALL {config.source_language.title()} modules:
- The analyst will read all source files
- It will produce a comprehensive migration specification
- INCLUDE the I/O contract from Phase 0 in the spec
- This spec guides all subsequent phases

### PHASE 2: Sequential Migration

For EACH module in dependency order:
1. Spawn the **migrator** agent with:
   - The relevant section of the migration spec (including I/O contract)
   - The specific module to migrate
2. Migrator MUST validate outputs match I/O contract
3. Migrator should NOT read source files directly - use the spec instead

### PHASE 3: Sequential Review

For EACH module, AFTER its migration completes:
1. Spawn the **reviewer** agent with:
   - The relevant section of the migration spec (including I/O contract)
   - The {target.name.title()} file path to review
2. Reviewer MUST verify I/O contract compliance

## Source Files (analyst will read these in Phase 1):
{source_files_list}

## Target Files (migrators write to these exact paths):
{target_files_list}

## Project Directory:
    {project_dir}

## Migration Order (respect dependencies):
{modules_list}

## Quality Gates (MUST pass before next module):
{quality_gates}
- **I/O contract validation** - outputs must match {config.source_language.title()} EXACTLY

## Key Points:
1. Phase 0 generates I/O contract from {config.source_language.title()} - critical for validation
2. Phase 1 reads source files ONCE and includes I/O contract in spec
3. Migrators validate against I/O contract
4. I/O contract violations are BLOCKERS - fix before proceeding
5. Run all build commands from: {project_dir}

Begin with Phase 0: spawn the io_contract agent to generate the I/O contract.
"""
