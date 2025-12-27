#!/usr/bin/env python3
"""
Claude Agent SDK Migration Runner - Version 4: Multi-Phase with I/O Validation

Uses a 4-phase approach to ensure behavioral equivalence:
- Phase 0: I/O contract generator runs Python on test inputs, captures outputs
- Phase 1: Analyst reads ALL Python files ONCE, includes I/O contract in spec
- Phase 2: For each module, migrator uses spec and validates I/O contract
- Phase 3: For each module, reviewer validates against spec + I/O contract

Usage:
    python run_migration.py
    python run_migration.py --dry-run
"""

import argparse
import asyncio
import os
import sys
from datetime import datetime
from pathlib import Path

# Check for required environment variable
if not os.environ.get("ANTHROPIC_API_KEY"):
    print("Error: ANTHROPIC_API_KEY environment variable not set")
    print("Set it with: export ANTHROPIC_API_KEY=your-api-key")
    sys.exit(1)

try:
    from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition
except ImportError:
    print("Error: claude-agent-sdk not installed")
    print("Install with: pip install claude-agent-sdk")
    print("Also ensure Claude Code is installed: curl -fsSL https://claude.ai/install.sh | sh")
    sys.exit(1)

from agents import (
    IO_CONTRACT_AGENT, ANALYST_AGENT, MIGRATOR_AGENT, REVIEWER_AGENT,
    MIGRATION_CONFIG, SOURCE_FILES, TARGET_FILES, PROJECT_DIR, TEST_INPUTS
)


# Setup logging
LOG_DIR = Path(__file__).parent / "logs"
LOG_DIR.mkdir(exist_ok=True)
LOG_FILE = LOG_DIR / f"migration_{datetime.now().strftime('%Y%m%d_%H%M%S')}.log"


def log(msg: str, also_print: bool = True) -> None:
    """Log a message with timestamp to file and optionally stdout."""
    timestamp = datetime.now().strftime("%H:%M:%S")
    formatted = f"[{timestamp}] {msg}"

    # Always write to log file
    with open(LOG_FILE, "a") as f:
        f.write(formatted + "\n")
        f.flush()

    # Optionally print to stdout
    if also_print:
        print(formatted)
        sys.stdout.flush()


def build_migration_prompt() -> str:
    """Build the multi-phase migration prompt (Version 4 with I/O validation)."""
    modules_list = "\n".join(
        f"    {i+1}. {m['python']} -> {m['rust']} ({m['phase']} phase)"
        for i, m in enumerate(MIGRATION_CONFIG["modules"])
    )

    source_files_list = "\n".join(
        f"    - {k}: {v}" for k, v in SOURCE_FILES.items()
    )

    target_files_list = "\n".join(
        f"    - {k}: {v}" for k, v in TARGET_FILES.items()
    )

    test_inputs_list = "\n".join(
        f'    - "{inp}"' for inp in TEST_INPUTS
    )

    return f"""
Migrate the rpn2tex Python codebase to Rust using a multi-phase approach with I/O validation.

## IMPORTANT: Multi-Phase Orchestration with I/O Contract

This migration uses FOUR distinct phases to ensure behavioral equivalence:

### PHASE 0: I/O Contract Generation (DO THIS FIRST)

Spawn the **io_contract** agent ONCE to:
1. Run the Python implementation on curated test inputs
2. Capture EXACT outputs for each input
3. Produce an I/O contract document

Test inputs to run:
{test_inputs_list}

SAVE the I/O contract - you will include it in the migration spec.

### PHASE 1: Comprehensive Analysis

Spawn the **analyst** agent ONCE to analyze ALL Python modules:
- The analyst will read all 7 Python source files
- It will produce a comprehensive migration specification
- INCLUDE the I/O contract from Phase 0 in the spec
- This spec guides all subsequent phases

### PHASE 2: Sequential Migration

For EACH module in dependency order:
1. Spawn the **migrator** agent with:
   - The relevant section of the migration spec (including I/O contract)
   - The specific module to migrate (python file -> rust file)
2. For latex.rs: migrator MUST validate outputs match I/O contract
3. Migrator should NOT read Python files - use the spec instead

### PHASE 3: Sequential Review

For EACH module, AFTER its migration completes:
1. Spawn the **reviewer** agent with:
   - The relevant section of the migration spec (including I/O contract)
   - The Rust file path to review
2. For latex.rs/main.rs: reviewer MUST verify I/O contract compliance

## Source Files (analyst will read these in Phase 1):
{source_files_list}

## Target Files (migrators write to these exact paths):
{target_files_list}

## Project Directory:
    {PROJECT_DIR}

## Migration Order (respect dependencies):
{modules_list}

## Quality Gates (MUST pass before next module):
- `cargo check && cargo clippy -- -D warnings` - zero errors/warnings
- `cargo fmt` - properly formatted
- `cargo test` - tests pass
- **I/O contract validation** - outputs must match Python EXACTLY

## Key Points:
1. Phase 0 generates I/O contract from Python - critical for validation
2. Phase 1 reads Python files ONCE and includes I/O contract in spec
3. Migrators validate against I/O contract, especially for latex.rs
4. I/O contract violations are BLOCKERS - fix before proceeding
5. Run all cargo commands from: {PROJECT_DIR}

Begin with Phase 0: spawn the io_contract agent to generate the I/O contract.
"""


async def run_migration(dry_run: bool = False) -> None:
    """Run the migration using Claude Agent SDK with multi-phase orchestration."""
    prompt = build_migration_prompt()

    if dry_run:
        print("=" * 60)
        print("DRY RUN - Would execute with the following prompt:")
        print("=" * 60)
        print(prompt)
        print("=" * 60)
        print("\nAgents configured:")
        print(f"  - io_contract: {IO_CONTRACT_AGENT['description'][:60]}...")
        print(f"  - analyst: {ANALYST_AGENT['description'][:60]}...")
        print(f"  - migrator: {MIGRATOR_AGENT['description'][:60]}...")
        print(f"  - reviewer: {REVIEWER_AGENT['description'][:60]}...")
        return

    log("=" * 60)
    log("Starting Claude Agent SDK Migration (Version 4: Multi-Phase with I/O Validation)")
    log("=" * 60)
    log(f"Source: {MIGRATION_CONFIG['source_dir']}")
    log(f"Target: {MIGRATION_CONFIG['target_dir']}")
    log(f"Modules: {len(MIGRATION_CONFIG['modules'])}")
    log(f"Log file: {LOG_FILE}")
    log("=" * 60)

    # Create AgentDefinition objects
    agents = {
        "io_contract": AgentDefinition(
            description=IO_CONTRACT_AGENT["description"],
            prompt=IO_CONTRACT_AGENT["prompt"],
            tools=IO_CONTRACT_AGENT["tools"],
            model=IO_CONTRACT_AGENT["model"],
        ),
        "analyst": AgentDefinition(
            description=ANALYST_AGENT["description"],
            prompt=ANALYST_AGENT["prompt"],
            tools=ANALYST_AGENT["tools"],
            model=ANALYST_AGENT["model"],
        ),
        "migrator": AgentDefinition(
            description=MIGRATOR_AGENT["description"],
            prompt=MIGRATOR_AGENT["prompt"],
            tools=MIGRATOR_AGENT["tools"],
            model=MIGRATOR_AGENT["model"],
        ),
        "reviewer": AgentDefinition(
            description=REVIEWER_AGENT["description"],
            prompt=REVIEWER_AGENT["prompt"],
            tools=REVIEWER_AGENT["tools"],
            model=REVIEWER_AGENT["model"],
        ),
    }

    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Edit", "Bash", "Glob", "Grep", "Task"],
        agents=agents,
        permission_mode="acceptEdits",  # Auto-accept file edits
    )

    message_count = 0
    try:
        async for message in query(prompt=prompt, options=options):
            message_count += 1

            # Log ALL messages for debugging
            log(f"MSG #{message_count}: type={type(message).__name__}", also_print=False)

            # Try to extract useful info from the message
            if hasattr(message, "type"):
                msg_type = message.type
                if msg_type == "text":
                    text = getattr(message, "text", "")
                    log(f"[TEXT] {text}", also_print=True)
                elif msg_type == "tool_use":
                    tool_name = getattr(message, "name", "unknown")
                    tool_input = getattr(message, "input", {})
                    if isinstance(tool_input, dict):
                        if "file_path" in tool_input:
                            log(f"[TOOL] {tool_name}: {tool_input.get('file_path', '')}")
                        elif "command" in tool_input:
                            cmd = tool_input.get("command", "")[:80]
                            log(f"[TOOL] {tool_name}: {cmd}...")
                        else:
                            log(f"[TOOL] {tool_name}: {str(tool_input)[:100]}")
                    else:
                        log(f"[TOOL] {tool_name}")
                elif msg_type == "tool_result":
                    result = getattr(message, "result", "")
                    if isinstance(result, str):
                        log(f"[RESULT] ({len(result)} chars)", also_print=False)
                    else:
                        log(f"[RESULT] {type(result).__name__}", also_print=False)
                else:
                    log(f"[{msg_type.upper()}] {str(message)[:200]}")
            elif hasattr(message, "content"):
                content = message.content
                log(f"[CONTENT] {str(content)[:200]}")
            else:
                msg_str = str(message)
                if len(msg_str) > 300:
                    log(f"[RAW] {msg_str[:300]}...")
                else:
                    log(f"[RAW] {msg_str}")

    except KeyboardInterrupt:
        log("\n\nMigration interrupted by user.")
        sys.exit(1)
    except Exception as e:
        log(f"\n\nError during migration: {e}")
        raise

    log("")
    log("=" * 60)
    log("Migration Complete")
    log("=" * 60)
    log(f"Total messages processed: {message_count}")
    log("")
    log("Next steps:")
    log("1. Run: cargo test")
    log("2. Run: cargo clippy")
    log("3. Compare with Python implementation")


def main():
    parser = argparse.ArgumentParser(
        description="Migrate Python code to Rust using Claude Agent SDK (Version 4: Multi-Phase with I/O Validation)"
    )
    parser.add_argument(
        "--dry-run",
        "-n",
        action="store_true",
        help="Show what would be done without executing",
    )

    args = parser.parse_args()

    asyncio.run(run_migration(dry_run=args.dry_run))


if __name__ == "__main__":
    main()
