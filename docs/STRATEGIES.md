# Migration Strategies

Migration strategies define how source code is decomposed and translated to the target language. Different strategies make different trade-offs between speed, code quality, and validation rigor.

## Strategy Interface

All strategies implement the `MigrationStrategy` base class:

```python
from strategies.base import MigrationStrategy

class MyStrategy(MigrationStrategy):
    name = "my-strategy"

    def get_prompt(self, config: ProjectConfig, target: LanguageTarget, project_dir: str) -> str:
        """Generate the migration prompt for this strategy."""
        return f"..."
```

## Available Strategies

### Module-by-Module

**File:** `src/migration/strategies/module_by_module.py`

Migrates each source module (file) in dependency order. Each module is fully translated before moving to the next.

**Process:**
1. Generate I/O contract from source
2. Analyze all source modules
3. For each module in dependency order:
   - Migrate the entire module
   - Run quality gates (compile, lint, format)
   - Validate against I/O contract
4. Review all migrated code

**Phases:**
- Phase 0: I/O Contract Generation
- Phase 1: Comprehensive Analysis
- Phase 2: Sequential Migration (per module)
- Phase 3: Sequential Review (per module)

**Characteristics:**
- Faster execution (32-37 min)
- Simpler prompts
- Easier to debug (one module at a time)
- Generates more test code per module

**Usage:**
```bash
python run_migration.py --target rust --project projects/rpn2tex
# or explicitly:
python run_migration.py --target rust --project projects/rpn2tex --strategy module-by-module
```

### Feature-by-Feature

**File:** `src/migration/strategies/feature_by_feature.py`

Migrates by feature slice, implementing end-to-end functionality incrementally. Each feature is validated against the I/O contract before proceeding.

**Process:**
1. Generate I/O contract from source
2. Analyze source to identify feature slices
3. For each feature (simplest to most complex):
   - Implement minimal code for that feature
   - Validate I/O contract cases for that feature
   - Refactor if needed
4. Final integration and review

**Feature Order (for rpn2tex):**
1. Numbers - Parse and format integers/decimals
2. Addition - Basic addition operator
3. Subtraction - Subtraction with proper ordering
4. Multiplication - Multiplication with implicit notation
5. Division - Division with fraction notation
6. Precedence - Complex expressions with mixed operators

**Characteristics:**
- More cohesive, focused code
- Catches integration issues early via incremental I/O validation
- Slower execution (55-60 min)
- More complex orchestration

**Usage:**
```bash
python run_migration.py --target rust --project projects/rpn2tex --strategy feature-by-feature
```

## Comparison (from 6 migrations)

| Target | Strategy | Duration | Cost | Coverage |
|--------|----------|----------|------|----------|
| Rust | MbM | 32 min | $8.83 | 94.7% |
| Rust | FbF | 60 min | $9.60 | 94.8% |
| Java | MbM | 37 min | $14.11 | 84.0% |
| Java | FbF | 55 min | $8.18 | 73.0% |
| Go | MbM | 37 min | $8.99 | 64.9% |
| Go | FbF | 56 min | $6.43 | 68.2% |

**Key findings:**
- Module-by-module is consistently faster (32-37 min vs 55-60 min)
- Cost varies without clear pattern by strategy
- Both strategies achieve 100% I/O contract match
- Coverage varies by language, not strategy

## Choosing a Strategy

**Use Module-by-Module when:**
- Speed is priority
- Source modules are well-structured with clear dependencies
- You want predictable, straightforward migration

**Use Feature-by-Feature when:**
- You want more cohesive code
- The source has tightly coupled features
- You need continuous I/O validation during migration

## Adding a New Strategy

1. Create `src/migration/strategies/{name}.py`:

```python
from strategies.base import MigrationStrategy
from framework.config import ProjectConfig
from languages.base import LanguageTarget

class MyStrategy(MigrationStrategy):
    name = "my-strategy"

    def get_prompt(self, config: ProjectConfig, target: LanguageTarget, project_dir: str) -> str:
        # Build custom prompt
        modules_list = "\n".join(f"- {m.source}" for m in config.modules)
        test_inputs = "\n".join(f'- "{inp}"' for inp in config.test_inputs)

        return f"""
Migrate {config.name} from {config.source_language} to {target.name}.

## Source Modules
{modules_list}

## Test Inputs
{test_inputs}

## Your Custom Instructions
...
"""
```

2. Register in `src/migration/strategies/__init__.py`:

```python
from .my_strategy import MyStrategy

STRATEGIES = {
    "module-by-module": ModuleByModuleStrategy,
    "feature-by-feature": FeatureByFeatureStrategy,
    "my-strategy": MyStrategy,
}
```

3. Use with CLI:

```bash
python run_migration.py --target rust --project projects/myproject --strategy my-strategy
```

## Strategy Design Principles

### I/O Contract First

All strategies should generate an I/O contract before migration. This provides:
- Ground truth for behavioral equivalence
- Validation checkpoints during migration
- Confidence in final output

### Quality Gates

Strategies should validate after each step:
- Compilation passes
- Linter has no errors
- Formatter has no issues
- Tests pass (if applicable)

### Incremental Validation

Validate early and often:
- Module-by-module: After each module
- Feature-by-feature: After each feature slice
- Both: Run full I/O contract at end

### Clear Phases

Structure the prompt with explicit phases:
- Analysis phase (read and understand)
- Migration phase (write code)
- Review phase (verify quality)

This gives the model clear checkpoints and prevents confusion.
