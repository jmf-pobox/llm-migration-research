# Code Migration Experiment with Claude Agent SDK

An experiment in using the Claude Agent SDK for cross-language code migration.

## Project Status: Phase 1 Complete

**Completed:** Small-scale migration tested with two target languages (Rust, Java)
**Next Phase:** Scale to medium-complexity codebase with external dependencies

## What This Project Is

This is a **prompt engineering experiment**, not a framework. We wrote ~500 lines of Python that:

1. Generates structured prompts describing a migration task
2. Passes those prompts to the Claude Agent SDK
3. Logs the output

The Claude Agent SDK and the underlying Claude model do all the actual work: reading files, generating code, running build tools, fixing errors, and coordinating the workflow.

### What We Built vs. What Anthropic Built

| Component | Who Built It | What It Does |
|-----------|--------------|--------------|
| Claude Agent SDK | Anthropic | Orchestration, tool execution, error handling, retries |
| Claude Model | Anthropic | Code understanding, generation, reasoning |
| This repo | Us | Prompt templates, YAML config parsing, CLI wrapper |

## Results

We migrated `rpn2tex` (a trivial 352-LOC Python codebase) to Rust and Java:

| Metric | Rust | Java |
|--------|------|------|
| Duration | ~25 min | ~25 min |
| Cost | $3.74 USD | $7.24 USD |
| I/O Match | 100% (21/21) | 100% (21/21) |
| Test Coverage | 97.66% | 95.87% |

### Source Codebase Complexity

The source was deliberately trivial:

| Metric | Value |
|--------|-------|
| Production LOC | 352 |
| Avg Cyclomatic Complexity | 2.8 |
| External Dependencies | None |

## How It Works

We send one prompt to the Claude Agent SDK describing what we want:

```
"Migrate this Python codebase to Rust. First run the source to capture
expected outputs. Then analyze all files. Then migrate each module and
verify outputs match. Use these agents: io_contract, analyst, migrator,
reviewer..."
```

The SDK and model handle everything else. The "4-phase workflow" and "agent coordination" exist in the prompt text - we describe what we want, the model figures out how to do it.

### The Entire "Orchestration"

```python
async for message in query(prompt=prompt, options=options):
    log(message)
```

## Project Structure

```
sdk_migration/
├── run_migration.py          # CLI (~180 LOC)
├── framework/
│   ├── agents.py             # Prompt templates (~250 LOC)
│   ├── runner.py             # SDK call + logging (~260 LOC)
│   └── config.py             # YAML parsing
├── languages/                # Config strings (file extensions, build commands)
└── projects/rpn2tex/         # Test project with source and outputs
```

## Usage

```bash
export ANTHROPIC_API_KEY=your-key
pip install claude-agent-sdk

python run_migration.py --target rust --project projects/rpn2tex
python run_migration.py --target java --project projects/rpn2tex
```

## Findings

1. **I/O contracts help** - Pre-capturing expected outputs prevents stylistic drift
2. **Prompt structure matters** - Describing phases in the prompt improved results
3. **Trivial codebases work** - No external dependencies, low complexity
4. **The SDK does the work** - Our contribution is prompt engineering

## Limitations

- Only tested on trivial codebase (352 LOC, no dependencies)
- No actual orchestration logic - relies entirely on SDK/model
- No error recovery beyond what the SDK provides
- Cannot handle codebases the model can't fit in context

## Future Work

Test whether this approach scales to:
- Larger codebases (5,000-20,000 LOC)
- External dependencies requiring library mapping
- Stateful systems with side effects

## License

MIT
