#!/usr/bin/env bash
set -euo pipefail

# check-docs.sh — extract ```rust,should-compile blocks from src/**/*.md,
# write them as integration tests under exercises/tests/doc_check_N.rs,
# and compile them with `cargo test --test doc_check_N --no-run`.

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
SRC_DIR="${ROOT_DIR}/src"
EXERCISES_DIR="${ROOT_DIR}/exercises"
TESTS_DIR="${EXERCISES_DIR}/tests"

mkdir -p "${TESTS_DIR}"

# Clean up previously generated files.
rm -f "${TESTS_DIR}"/doc_check_*.rs

# Collect all rust,should-compile blocks into an indexed list.
# Each block is preceded by a header line of the form:
#   >>>FILE:/path/to/file.md
#   >>>START:line
# This metadata is removed when writing the test file, but is used for
# diagnostics.
blocks_file="$(mktemp)"
trap 'rm -f "${blocks_file}"' EXIT

block_index=0

find "${SRC_DIR}" -type f -name '*.md' -print0 | while IFS= read -r -d '' file; do
    awk -v file="${file}" '
        /^```rust,should-compile/ { in_block = 1; start = NR + 1; next }
        /^```/ && in_block {
            in_block = 0
            printf ">>>FILE:%s\n>>>START:%d\n", file, start
            for (i = 1; i <= n; i++) printf "%s\n", buf[i]
            delete buf
            n = 0
            block_count++
            next
        }
        in_block { buf[++n] = $0 }
        END { exit (block_count + 0) }
    ' "$file" > "${blocks_file}.${file##*/}" 2>/dev/null || true
    if [ -s "${blocks_file}.${file##*/}" ]; then
        cat "${blocks_file}.${file##*/}" >> "${blocks_file}"
        rm -f "${blocks_file}.${file##*/}"
    fi
done

# Split the collected blocks into individual test files.
# Each file is wrapped in a module with a unique name to avoid symbol collisions.
{
    current_file=""
    current_start=""
    buf=()

    flush_block() {
        if [ -z "${current_file}" ]; then
            return
        fi
        test_file="${TESTS_DIR}/doc_check_${block_index}.rs"
        {
            echo "// Generated from ${current_file}:${current_start}"
            echo "fn main() {"
            printf '%s\n' "${buf[@]}"
            echo "}"
        } > "${test_file}"
        block_index=$((block_index + 1))
        buf=()
    }

    while IFS= read -r line; do
        case "${line}" in
            '>>>FILE:'*)
                flush_block
                current_file="${line#>>>FILE:}"
                ;;
            '>>>START:'*)
                current_start="${line#>>>START:}"
                ;;
            *)
                buf+=("${line}")
                ;;
        esac
    done < "${blocks_file}"
    flush_block
}

# Report and compile each generated test file.
compiled=0
failed=0
total_blocks=${block_index}

if [ "${total_blocks}" -eq 0 ]; then
    echo "Report: 0 \`\`\`rust,should-compile block(s) found"
    exit 0
fi

for test_file in "${TESTS_DIR}"/doc_check_*.rs; do
    [ -e "${test_file}" ] || continue
    name=$(basename "${test_file}" .rs)
    source=$(head -n 1 "${test_file}" | sed 's|^// Generated from ||')
    echo "[check] ${source}"
    if (
        cd "${EXERCISES_DIR}"
        cargo test --test "${name}" --no-run
    ); then
        compiled=$((compiled + 1))
        echo "  ✓ ${name} compiled"
    else
        failed=$((failed + 1))
        echo "  ✗ ${name} failed to compile"
    fi
done

echo "---"
echo "Report: ${total_blocks} total \`\`\`rust,should-compile block(s); ${compiled} compiled, ${failed} failed"

if [ "${failed}" -gt 0 ]; then
    exit 1
fi
