#!/bin/bash
set -e
cd "$(dirname "$0")/.."

echo "=== Batch Migration Runner ==="
echo "Started at: $(date)"
echo "Working directory: $(pwd)"
echo ""

RUNS=(
  "rust module-by-module"
  "java module-by-module"
  "go module-by-module"
  "go feature-by-feature"
)

for run in "${RUNS[@]}"; do
  read target strategy <<< "$run"
  echo "=== Starting $target $strategy at $(date) ==="
  python run_migration.py \
    --target "$target" \
    --strategy "$strategy" \
    --project projects/rpn2tex/config.yaml
  echo "=== Completed $target $strategy at $(date) ==="
  echo ""
done

echo "=== All migrations complete! ==="
echo "Finished at: $(date)"
