# Code Migration Experiment Report

## Project Status: Phase 1 Complete

**Completed:** Small-scale migration tested with two target languages (Rust, Java)
**Next Phase:** Scale to medium-complexity codebase with external dependencies

---

## Summary

We used the Claude Agent SDK to migrate a trivial Python codebase to Rust and Java. The migrations produced working code with 100% behavioral equivalence.

### Results

| Metric | Rust | Java |
|--------|------|------|
| Duration | ~25 min | ~25 min |
| Cost | $3.74 USD | $7.24 USD |
| I/O Match | 100% (21/21) | 100% (21/21) |
| Test Coverage | 97.66% | 95.87% |

### Source Codebase

The source was deliberately trivial:

| Metric | Value |
|--------|-------|
| Production LOC | 352 |
| Avg Cyclomatic Complexity | 2.8 |
| External Dependencies | None |

## What We Actually Built

~500 lines of Python that:
1. Generates prompts from YAML config
2. Calls the Claude Agent SDK once
3. Logs output

```python
# The entire migration logic:
async for message in query(prompt=prompt, options=options):
    log(message)
```

### Division of Work

| Component | Responsibility |
|-----------|----------------|
| Claude Agent SDK | Orchestration, tool execution, retries, state management |
| Claude Model | Code understanding, generation, reasoning, self-correction |
| This repo | Prompt templates, config parsing |

The "multi-phase workflow" and "agent coordination" exist in the **prompt text**, not in code. We describe what we want; the SDK and model execute it.

## Methodology

We wrote a prompt describing:
1. Run source implementation, capture outputs (I/O contract)
2. Read all source files, produce analysis
3. Migrate each module, verify outputs match
4. Review each module

The model interprets this and executes accordingly. There is no orchestration logic on our side.

### Prompt Structure

The prompt includes:
- Source file paths
- Target file paths
- Module dependency order
- Build commands to run
- I/O test cases

We also define 4 "agents" (really just prompt templates with different tool access):
- `io_contract` - Haiku, Bash/Read access
- `analyst` - Haiku, Read-only access
- `migrator` - Sonnet, full tool access
- `reviewer` - Haiku, Read/Bash access

The SDK spawns these as sub-agents when the main prompt requests it.

## Observations

### What Worked

1. **I/O contracts** - Pre-capturing expected outputs helped ensure behavioral equivalence
2. **Structured prompts** - Describing phases gave more predictable results than unstructured requests
3. **Quality gates in prompt** - Including build commands in the prompt caused the model to run them

### What We Learned

1. The SDK handles everything - our "framework" is just prompt generation
2. Trivial codebases (no dependencies, low complexity) migrate successfully
3. Cost scales with output verbosity (Java cost 2x Rust)

### Limitations

- Only tested on 352 LOC with no dependencies
- No actual orchestration - relies entirely on SDK/model behavior
- Cannot handle codebases larger than context window
- No error recovery beyond SDK defaults

## Complexity Metrics

| Metric | Python | Rust | Java |
|--------|--------|------|------|
| Production LOC | 352 | 408 | 529 |
| Function count | 25 | 32 | 42 |
| Avg cyclomatic complexity | 2.8 | 2.4 | 2.9 |
| Max cyclomatic complexity | 10 | 7 | 15 |

## File Structure

```
sdk_migration/
├── run_migration.py          # CLI wrapper
├── framework/
│   ├── agents.py             # Prompt templates
│   ├── runner.py             # SDK call + logging
│   └── config.py             # YAML parsing
├── languages/                # Build command configs
└── projects/rpn2tex/         # Source and outputs
```

## Future Work

Test whether this approach works for:
- Larger codebases (5,000-20,000 LOC)
- External dependencies
- Stateful systems

---

*Report updated: 2025-12-28*
*Author: James Freeman, Pembroke College, University of Oxford*
