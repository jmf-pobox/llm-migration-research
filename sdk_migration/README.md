# Code Migration with Claude Agent SDK

Cross-language code migration using the Claude Agent SDK.

## Status

**Phase 1 complete:** Tested with trivial codebase (352 LOC), two target languages (Rust, Java)
**Next:** Test with larger codebase and external dependencies

## Results

Migrated `rpn2tex` (Python RPN-to-LaTeX converter) to Rust and Java:

| Metric | Rust | Java |
|--------|------|------|
| Duration | ~25 min | ~25 min |
| Cost | $3.74 | $7.24 |
| I/O Match | 21/21 | 21/21 |
| Test Coverage | 97.66% | 95.87% |

Source complexity: 352 LOC, avg cyclomatic complexity 2.8, no external dependencies.

## How It Works

The CLI generates a prompt describing the migration task and passes it to the Claude Agent SDK. The prompt describes:
- Source and target file paths
- Module dependency order
- Build commands to run
- Test inputs and expected outputs
- Sub-agent definitions (different tool permissions)

The SDK and model handle execution.

```python
async for message in query(prompt=prompt, options=options):
    log(message)
```

## Structure

```
sdk_migration/
├── run_migration.py      # CLI
├── framework/
│   ├── agents.py         # Prompt templates
│   ├── runner.py         # SDK call + logging
│   └── config.py         # YAML parsing
├── languages/            # Target language configs
└── projects/rpn2tex/     # Source and outputs
```

## Usage

```bash
export ANTHROPIC_API_KEY=your-key
pip install claude-agent-sdk

python run_migration.py --target rust --project projects/rpn2tex
python run_migration.py --target java --project projects/rpn2tex
```

## Observations

- Pre-capturing expected outputs (I/O contract) helped ensure behavioral equivalence
- Structured prompts (describing phases) gave more predictable results
- Embedding source in prompts made things slower (4x) and more expensive

## Limitations

- Only tested on trivial codebase
- No external dependencies in test case
- 21 test cases, not exhaustive

## License

MIT
