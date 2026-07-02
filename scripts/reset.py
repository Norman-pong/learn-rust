#!/usr/bin/env python3
"""reset.py — 将 exercises/src/*.rs 重置为待完成状态。

用法: scripts/reset.py [chapter]
无参数时重置全部章节（除 lib.rs），有参数时只重置对应章节。
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
EXERCISE_DIR = ROOT / "exercises" / "src"
SOLUTION_DIR = ROOT / "solutions"


TEST_ATTR_RE = re.compile(r"^\s*#\[(test|ignore|should_panic).*\]\s*$")
FN_START_RE = re.compile(r"^fn\s+([A-Za-z_][A-Za-z0-9_]*)\(\)\s*\{\s*$")
IAMNOTDONE_RE = re.compile(r"^\s*//\s*I\s*AM\s*NOT\s*DONE\s*$")


def load_hints(solution_path: Path) -> dict[str, str]:
    """从 solutions/<chapter>.rs 提取每个 Exercise 的中文提示。

    提示格式：
        // Exercise variables1
        // 变量默认不可变，需要 `mut` 才能重新赋值。
    """
    hints: dict[str, str] = {}
    if not solution_path.exists():
        return hints

    current_title = ""
    for line in solution_path.read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if stripped.startswith("// Exercise "):
            current_title = stripped.replace("// Exercise ", "").strip()
            continue
        if stripped.startswith("// ") and current_title:
            hints[current_title] = stripped.replace("// ", "").strip()
            current_title = ""
        elif current_title and not stripped.startswith("//"):
            # Exercise 标题后的下一行不是注释，说明该 exercise 没有独立提示
            current_title = ""
    return hints


def reset_source(path: Path, hints: dict[str, str]) -> str:
    """将单个练习文件内容重置为 todo! 占位状态。

    原始练习文件格式示例：
    #[test]
    #[ignore]
    fn variables1() {
        // I AM NOT DONE
        // 变量默认不可变，需要 `mut` 才能重新赋值。
        let mut x = 5;
        x = 7;
        assert_eq!(x, 7);
    }
    """
    original = path.read_text(encoding="utf-8")
    lines = original.splitlines(keepends=True)
    output: list[str] = []
    i = 0
    n = len(lines)

    while i < n:
        line = lines[i]
        stripped = line.rstrip()

        # 收集测试属性（#[test], #[ignore], #[should_panic]）
        if TEST_ATTR_RE.match(stripped):
            attrs = [stripped]
            i += 1
            while i < n and TEST_ATTR_RE.match(lines[i].rstrip()):
                attrs.append(lines[i].rstrip())
                i += 1

            if i >= n:
                break

            fn_line = lines[i].rstrip()
            match = FN_START_RE.match(fn_line)
            if not match:
                # 不是预期的函数头，原样输出收集的属性
                output.extend(f"{attr}\n" for attr in attrs)
                output.append(lines[i])
                i += 1
                continue

            fn_name = match.group(1)
            title = fn_name.replace("_", "")
            i += 1  # 跳过函数头 fn name() {

            # 跳过 // I AM NOT DONE（如果它在函数头后的第一行）
            if i < n and IAMNOTDONE_RE.match(lines[i].rstrip()):
                i += 1

            # 收集函数体，直到找到与函数开头 { 匹配的 }
            body_lines: list[str] = []
            depth = 1
            while i < n and depth > 0:
                current = lines[i]
                for ch in current:
                    if ch == "{":
                        depth += 1
                    elif ch == "}":
                        depth -= 1
                if depth == 0:
                    # 函数体的最后一行可能包含 }，例如 "    }\n"
                    content = current.rstrip()
                    if content != "}":
                        # 去掉行尾的 }，保留前面的内容（如果有）
                        ridx = content.rfind("}")
                        if ridx != -1:
                            body_lines.append(content[:ridx] + "\n")
                    i += 1
                    break
                body_lines.append(current)
                i += 1

            # 输出重置后的测试函数
            output.extend(f"{attr}\n" for attr in attrs)
            output.append(f"fn {fn_name}() {{\n")
            output.append("    // I AM NOT DONE\n")
            hint = hints.get(title, "")
            if hint:
                output.append(f"    // {hint}\n")
            else:
                output.append("    // 请完成此练习\n")
            if hint:
                escaped_hint = hint.replace("\\", "\\\\").replace('"', '\\"')
                output.append(f'    todo!("{escaped_hint}");\n')
            else:
                output.append(f'    todo!("完成练习 {fn_name}");\n')
            for bline in body_lines:
                if "assert_eq!" in bline:
                    # 注释掉 assert 行，保持缩进
                    output.append("    // " + bline.lstrip())
            output.append("}\n")
            output.append("\n")
            continue

        # 普通行原样输出
        output.append(line)
        i += 1

    return "".join(output)


def main() -> int:
    parser = argparse.ArgumentParser(description="重置 Rust 练习文件为待完成状态")
    parser.add_argument("chapter", nargs="?", default="", help="指定章节名，如 variables")
    args = parser.parse_args()
    chapter: str = args.chapter

    if not EXERCISE_DIR.is_dir():
        print(f"error: exercises/src not found at {EXERCISE_DIR}", file=sys.stderr)
        return 1

    if chapter:
        file = EXERCISE_DIR / f"{chapter}.rs"
        if not file.is_file():
            print(f"error: chapter not found: {chapter}", file=sys.stderr)
            return 1
        files = [file]
    else:
        files = sorted(p for p in EXERCISE_DIR.glob("*.rs") if p.name != "lib.rs")

    for src in files:
        basename = src.stem
        soln = SOLUTION_DIR / f"{basename}.rs"
        hints = load_hints(soln)
        new_content = reset_source(src, hints)
        src.write_text(new_content, encoding="utf-8")
        print(f"reset: {src}")

    print("Done. Run 'just test' to see ignored tests fail.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
