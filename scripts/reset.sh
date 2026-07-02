#!/usr/bin/env bash
set -euo pipefail

# reset.sh — 将 exercises/src/*.rs 重置为待完成状态
# 用法: scripts/reset.sh [chapter]
# 无参数时重置全部章节（除 lib.rs），有参数时只重置对应章节。

CHAPTER="${1:-}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXERCISE_DIR="$ROOT/exercises/src"
SOLUTION_DIR="$ROOT/solutions"

if [[ ! -d "$EXERCISE_DIR" ]]; then
    echo "error: exercises/src not found at $EXERCISE_DIR" >&2
    exit 1
fi

if [[ -n "$CHAPTER" ]]; then
    FILE="$EXERCISE_DIR/${CHAPTER}.rs"
    if [[ ! -f "$FILE" ]]; then
        echo "error: chapter not found: $CHAPTER" >&2
        exit 1
    fi
    FILES=("$FILE")
else
    FILES=()
    while IFS= read -r -d '' file; do
        FILES+=("$file")
    done < <(find "$EXERCISE_DIR" -maxdepth 1 -name '*.rs' ! -name 'lib.rs' -print0 | sort -z)
fi

for SRC in "${FILES[@]}"; do
    BASENAME="$(basename "$SRC" .rs)"
    SOLN="$SOLUTION_DIR/${BASENAME}.rs"
    TMP="$(mktemp)"

    awk -v soln="$SOLN" '
        function ltrim(s) {
            sub(/^[ \t]+/, "", s)
            return s
        }
        function gen_todo(fn, title,    out) {
            if (title in hint) {
                out = hint[title]
                gsub(/"/, "\\\"", out)
                return out
            }
            return "完成练习 " fn
        }
        BEGIN {
            in_test = 0
            brace_depth = 0
            fn_name = ""
            test_attrs = ""
            if (soln != "") {
                while ((getline line < soln) > 0) {
                    if (line ~ /^\/\/ Exercise /) {
                        gsub(/^\/\/ Exercise /, "", line)
                        current_title = line
                        next_hint_line = 1
                    } else if (line ~ /^\/\// && next_hint_line) {
                        gsub(/^\/\/ */, "", line)
                        hint[current_title] = line
                        next_hint_line = 0
                    }
                }
                close(soln)
            }
        }
        /^#\[test\]/ {
            test_attrs = (test_attrs == "" ? "" : test_attrs "\n") $0
            next
        }
        /^#\[ignore\]/ {
            test_attrs = (test_attrs == "" ? "" : test_attrs "\n") $0
            next
        }
        /^#\[should_panic[^\]]*\]/ {
            test_attrs = (test_attrs == "" ? "" : test_attrs "\n") $0
            next
        }
        /^\/\/ I AM NOT DONE/ { next }
        /^fn [A-Za-z_][A-Za-z0-9_]*\(\)/ {
            if (test_attrs != "") {
                in_test = 1
                fn_name = $0
                sub(/^fn /, "", fn_name)
                sub(/\(\).*$/, "", fn_name)
                brace_depth = 0
                fn_body = ""
                body_started = 0
                next
            }
        }
        in_test {
            for (i = 1; i <= length($0); i++) {
                c = substr($0, i, 1)
                if (c == "{") brace_depth++
                else if (c == "}") brace_depth--
            }
            if (!body_started) {
                body_started = 1
                next
            }
            if (fn_body != "") fn_body = fn_body "\n"
            fn_body = fn_body $0
            if (brace_depth == 0) {
                print test_attrs
                print "fn " fn_name "() {"
                print "    // I AM NOT DONE"
                title = fn_name
                gsub(/_/, "", title)
                if (title in hint) {
                    print "    // " hint[title]
                } else {
                    print "    // 请完成此练习"
                }
                print "    todo!(\"" gen_todo(fn_name, title) "\");"
                n = split(fn_body, lines, "\n")
                for (i = 1; i < n; i++) {
                    line = lines[i]
                    if (line ~ /assert_eq!/) {
                        print "    // " line
                    }
                }
                print "}"
                print ""
                in_test = 0
                test_attrs = ""
                fn_name = ""
                fn_body = ""
                body_started = 0
            }
            next
        }
        { print }
    ' "$SRC" > "$TMP"

    mv "$TMP" "$SRC"
    echo "reset: $SRC"
done

echo "Done. Run 'just test' to see ignored tests fail."
