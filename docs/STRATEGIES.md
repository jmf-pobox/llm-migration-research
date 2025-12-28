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

**File:** `strategies/module_by_module.py`

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

**Pros:**
- Faster execution (~25 min for rpn2tex)
- Simpler prompts
- Easier to debug (one module at a time)
- Lower cost

**Cons:**
- May produce redundant code across modules
- Less cohesive architecture
- Larger code output (+27% vs feature-by-feature)

**Usage:**
```bash
python run_migration.py --target rust --project projects/rpn2tex
# or explicitly:
python run_migration.py --target rust --project projects/rpn2tex --strategy module-by-module
```

### Feature-by-Feature

**File:** `strategies/feature_by_feature.py`

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

**Pros:**
- More cohesive, focused code
- Catches integration issues early
- Produces 21% less code
- Better architectural decisions

**Cons:**
- 72% longer execution time (~43 min)
- More complex orchestration
- Higher cost per run
- Requires careful feature decomposition

**Usage:**
```bash
python run_migration.py --target rust --project projects/rpn2tex --strategy feature-by-feature
```

## Comparison

| Metric | Module-by-Module | Feature-by-Feature |
|--------|------------------|-------------------|
| Duration | 25 min | 43 min |
| Cost | $3.74 | $6.07 |
| Production LOC | 1,184 | 931 |
| Test LOC | 2,191 | 3,025 |
| I/O Match | 100% | 100% |
| Code Style | More boilerplate | More cohesive |

## Choosing a Strategy

**Use Module-by-Module when:**
- Rapid iteration is needed
- Budget is constrained
- Source modules are already well-structured
- You want predictable, straightforward migration

**Use Feature-by-Feature when:**
- Code quality is paramount
- You want minimal, focused output
- The source has tightly coupled features
- You need continuous validation during migration

## Adding a New Strategy

1. Create `strategies/{name}.py`:

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

2. Register in `strategies/__init__.py`:

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
