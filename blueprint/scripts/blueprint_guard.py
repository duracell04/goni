#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path


FRONTMATTER_REQUIRED_DIRS = [
    "30-specs",
    "50-evidence",
    "decisions",
]

LINK_CHECK_FILES = [
    "README.md",
]

LINK_CHECK_DIRS = [
    "30-specs",
    "00-map",
]

ALLOWED_BLUEPRINT_ROOT_FILES = {"README.md", "AGENTS.md"}

LINK_PATTERN = re.compile(r"!?\[[^]]*\]\(([^)]+)\)")


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8-sig", errors="ignore")


def strip_fenced_code_blocks(text: str) -> str:
    lines = text.splitlines()
    result = []
    in_code = False
    fence = ""
    for line in lines:
        if line.strip().startswith("```"):
            marker = line.strip()[:3]
            if not in_code:
                in_code = True
                fence = marker
            elif marker == fence:
                in_code = False
                fence = ""
            continue
        if not in_code:
            result.append(line)
    return "\n".join(result)


def parse_frontmatter(text: str) -> dict | None:
    stripped = text.lstrip()
    if not stripped.startswith("---"):
        return None
    lines = stripped.splitlines()
    if not lines or lines[0].strip() != "---":
        return None
    data_lines = []
    for line in lines[1:]:
        if line.strip() == "---":
            return parse_simple_yaml(data_lines)
        data_lines.append(line)
    return None


def parse_simple_yaml(lines: list[str]) -> dict:
    data: dict[str, str] = {}
    for line in lines:
        if ":" not in line:
            continue
        key, value = line.split(":", 1)
        data[key.strip()] = value.strip()
    return data


def check_blueprint_root(blueprint_root: Path, errors: list[str]) -> None:
    for item in blueprint_root.iterdir():
        if item.is_file() and item.name not in ALLOWED_BLUEPRINT_ROOT_FILES:
            errors.append(f"Unexpected file in blueprint root: {item.name}")


def check_frontmatter(blueprint_root: Path, errors: list[str]) -> dict[str, Path]:
    ids: dict[str, Path] = {}
    for relative in FRONTMATTER_REQUIRED_DIRS:
        folder = blueprint_root / relative
        if not folder.exists():
            continue
        for path in folder.rglob("*.md"):
            text = read_text(path)
            frontmatter = parse_frontmatter(text)
            if not frontmatter:
                errors.append(f"Missing frontmatter: {path}")
                continue
            for key in ("id", "type", "status"):
                if key not in frontmatter or not frontmatter[key]:
                    errors.append(f"Frontmatter missing '{key}': {path}")
            doc_id = frontmatter.get("id")
            if doc_id:
                if doc_id in ids:
                    errors.append(
                        f"Duplicate frontmatter id '{doc_id}' in {path} "
                        f"(already in {ids[doc_id]})"
                    )
                else:
                    ids[doc_id] = path
    return ids


def resolve_link(path: Path, link: str, repo_root: Path) -> Path | None:
    link = link.strip()
    if not link or link.startswith("#"):
        return None

    if link.startswith("http://") or link.startswith("https://"):
        return None
    if link.startswith("mailto:") or link.startswith("tel:"):
        return None

    link = link.split("#", 1)[0].split("?", 1)[0]
    link = link.replace("%20", " ").strip()
    if not link:
        return None

    if link.startswith("/"):
        return repo_root / link.lstrip("/")
    return (path.parent / link).resolve()


def check_links(repo_root: Path, blueprint_root: Path, errors: list[str]) -> None:
    files = [blueprint_root / file for file in LINK_CHECK_FILES]
    for rel in LINK_CHECK_DIRS:
        folder = blueprint_root / rel
        if folder.exists():
            files.extend(folder.rglob("*.md"))

    for path in files:
        if not path.exists():
            errors.append(f"Missing link-check target: {path}")
            continue
        content = strip_fenced_code_blocks(read_text(path))
        for match in LINK_PATTERN.finditer(content):
            raw_link = match.group(1).strip()
            if not raw_link:
                continue
            if raw_link.startswith("<") and raw_link.endswith(">"):
                raw_link = raw_link[1:-1].strip()
            if " " in raw_link:
                raw_link = raw_link.split(" ", 1)[0].strip()
            resolved = resolve_link(path, raw_link, repo_root)
            if resolved is None:
                continue
            if not resolved.exists():
                errors.append(f"Broken link in {path}: {raw_link}")


def main() -> None:
    repo_root = Path(__file__).resolve().parents[2]
    blueprint_root = repo_root / "blueprint"

    errors: list[str] = []
    check_blueprint_root(blueprint_root, errors)
    check_frontmatter(blueprint_root, errors)
    check_links(repo_root, blueprint_root, errors)

    if errors:
        for error in errors:
            print(error)
        sys.exit(1)


if __name__ == "__main__":
    main()
