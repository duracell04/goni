#!/usr/bin/env python3
"""Make the specified-only boundary explicit on every migrated node."""

from __future__ import annotations

import re
from pathlib import Path

import yaml


FRONTMATTER = re.compile(r"\A---\s*\n(.*?)\n---\s*\n?", re.S)
BOUNDARY = (
    "> Status boundary: this is a migrated draft. For `specified_only` nodes, "
    "present-tense or enforcement language below states intended contract behavior, "
    "not observed implementation, verification, or non-bypassability."
)
STRONG_CLAIM = re.compile(r"\b(?:implemented|enforced|verified|validated|non-bypassable)\b", re.I)


def main() -> int:
    repo = Path(__file__).resolve().parents[1]
    changed = 0
    for path in sorted((repo / "knowledge").rglob("*.md"), key=lambda item: item.as_posix()):
        if path.name == "README.md":
            continue
        text = path.read_text(encoding="utf-8")
        match = FRONTMATTER.match(text)
        if not match:
            continue
        metadata = yaml.safe_load(match.group(1)) or {}
        rest = text[match.end() :].lstrip()
        if metadata.get("implementation_state") == "specified_only" and STRONG_CLAIM.search(str(metadata.get("proposition", ""))):
            metadata["proposition"] = "Specified design intent: " + str(metadata["proposition"])
        if BOUNDARY not in rest:
            lines = rest.splitlines()
            insert_at = 1 if lines and lines[0].startswith("# ") else 0
            lines[insert_at:insert_at] = ["", BOUNDARY]
            rest = "\n".join(lines).strip() + "\n"
        normalized = "---\n" + yaml.safe_dump(metadata, sort_keys=False, allow_unicode=True, width=1000) + "---\n\n" + rest
        if normalized != text:
            path.write_text(normalized, encoding="utf-8", newline="\n")
            changed += 1
    print(f"normalized={changed}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
