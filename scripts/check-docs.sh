#!/usr/bin/env bash
set -euo pipefail

# check-docs.sh
# Skeleton script for verifying documentation snippets.
# For now: extracts ```rust,should-compile blocks from src/**/*.md and reports counts.

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
SRC_DIR="${ROOT_DIR}/src"

total_files=0
total_blocks=0

if ! command -v find >/dev/null 2>&1; then
    echo "error: find is required" >&2
    exit 1
fi

find "${SRC_DIR}" -type f -name '*.md' -print0 | while IFS= read -r -d '' file; do
    # Extract fenced code blocks labeled rust,should-compile
    # awk state machine: inside a ```rust,should-compile block -> print until ```
    blocks=$(awk '
        /^```rust,should-compile/ { in_block = 1; next }
        /^```/ && in_block { in_block = 0; block_count++; next }
        in_block { print }
        END { print block_count + 0 }
    ' "$file")

    count=$(echo "$blocks" | tail -n 1)
    if [ "${count:-0}" -gt 0 ]; then
        echo "${file}: ${count} rust,should-compile block(s)"
        total_blocks=$((total_blocks + count))
        total_files=$((total_files + 1))
    fi
done

echo "---"
echo "Report: ${total_blocks} \`\`\`rust,should-compile block(s) found in ${total_files} file(s)"
echo "Note: full compilation checking is not yet implemented in this skeleton."
