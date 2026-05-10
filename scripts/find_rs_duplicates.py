#!/usr/bin/env python3
"""Find repeated Rust code and duplicate type definitions.

Modes:
- repeated consecutive blocks of normalized lines
- duplicate `struct` / `enum` definitions by normalized shape
"""

from __future__ import annotations

import argparse
import hashlib
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


@dataclass(frozen=True)
class BlockHit:
    path: Path
    start_line: int
    end_line: int


@dataclass(frozen=True)
class TypeHit:
    kind: str
    name: str
    path: Path
    start_line: int
    end_line: int


TYPE_START_RE = re.compile(r"^(?:pub\s+)?(struct|enum)\s+([A-Za-z_][A-Za-z0-9_]*)\b")


def iter_rust_files(root: Path) -> Iterable[Path]:
    for path in root.rglob("*.rs"):
        if "target" in path.parts:
            continue
        yield path


def strip_line_comment(line: str) -> str:
    if "//" not in line:
        return line
    return line[: line.find("//")]


def strip_block_comments(lines: list[str]) -> list[str]:
    out: list[str] = []
    in_block = False

    for raw in lines:
        line = raw
        if in_block:
            end = line.find("*/")
            if end == -1:
                continue
            line = line[end + 2 :]
            in_block = False

        while "/*" in line:
            start = line.find("/*")
            end = line.find("*/", start + 2)
            if end == -1:
                line = line[:start]
                in_block = True
                break
            line = line[:start] + line[end + 2 :]

        out.append(line)

    return out


def normalize_lines(path: Path) -> list[str]:
    raw_lines = path.read_text(encoding="utf-8", errors="ignore").splitlines()
    lines = strip_block_comments(raw_lines)
    normalized: list[str] = []

    for raw in lines:
        line = strip_line_comment(raw).strip()
        if line:
            normalized.append(" ".join(line.split()))

    return normalized


def hash_text(text: str) -> str:
    return hashlib.sha1(text.encode("utf-8")).hexdigest()


def scan_blocks(path: Path, lines: list[str], min_lines: int) -> dict[str, BlockHit]:
    hits: dict[str, BlockHit] = {}
    for start in range(0, len(lines) - min_lines + 1):
        chunk = lines[start : start + min_lines]
        key = hash_text("\n".join(chunk))
        hits.setdefault(key, BlockHit(path=path, start_line=start + 1, end_line=start + min_lines))
    return hits


def scan_type_defs(path: Path) -> dict[str, TypeHit]:
    text_lines = path.read_text(encoding="utf-8", errors="ignore").splitlines()
    hits: dict[str, TypeHit] = {}
    i = 0

    while i < len(text_lines):
        raw = strip_line_comment(text_lines[i]).strip()
        if not raw or raw.startswith("#["):
            i += 1
            continue

        match = TYPE_START_RE.match(raw)
        if not match:
            i += 1
            continue

        kind, name = match.groups()
        start = i
        collected: list[str] = []
        brace_depth = 0
        saw_brace = False
        j = i

        while j < len(text_lines):
            cleaned = strip_line_comment(text_lines[j])
            collected.append(cleaned)
            brace_depth += cleaned.count("{") - cleaned.count("}")
            saw_brace = saw_brace or ("{" in cleaned)
            if saw_brace and brace_depth <= 0:
                break
            if not saw_brace and ";" in cleaned:
                break
            j += 1

        snippet = "\n".join(collected)
        if saw_brace:
            body = snippet[snippet.find("{") + 1 : snippet.rfind("}")]
        else:
            body = snippet.split(name, 1)[-1]
            if ";" in body:
                body = body.split(";", 1)[0]

        canonical = " ".join(body.split())
        if canonical:
            key = f"{kind}:{canonical}"
            hits.setdefault(
                key,
                TypeHit(kind=kind, name=name, path=path, start_line=start + 1, end_line=j + 1),
            )

        i = j + 1

    return hits


def print_groups(title: str, groups: dict[str, list], label: str) -> None:
    if not groups:
        return
    print(f"Found {len(groups)} {title}.")
    for _, hits in sorted(groups.items(), key=lambda item: (-len(item[1]), item[1][0].path.as_posix())):
        sample = hits[0]
        print()
        print(f"{label}: {sample.path}:{sample.start_line}-{sample.end_line}  (x{len(hits)})")
        for hit in hits:
            print(f"  - {hit.path}:{hit.start_line}-{hit.end_line}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default=".", help="project root to scan")
    parser.add_argument("--min-lines", type=int, default=6, help="minimum block size")
    args = parser.parse_args()

    root = Path(args.root).resolve()
    min_lines = max(2, args.min_lines)

    block_groups: dict[str, list[BlockHit]] = {}
    type_groups: dict[str, list[TypeHit]] = {}

    for path in iter_rust_files(root):
        lines = normalize_lines(path)
        for key, hit in scan_blocks(path, lines, min_lines).items():
            block_groups.setdefault(key, []).append(hit)
        for key, hit in scan_type_defs(path).items():
            type_groups.setdefault(key, []).append(hit)

    block_groups = {k: v for k, v in block_groups.items() if len(v) > 1}
    type_groups = {k: v for k, v in type_groups.items() if len(v) > 1}

    if not block_groups and not type_groups:
        print("No duplicate Rust code found.")
        return 0

    print_groups("duplicate type groups", type_groups, "Type")
    print_groups(f"duplicate block groups (min_lines={min_lines})", block_groups, "Block")
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
