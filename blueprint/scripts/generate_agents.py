#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path


def render_truth_map(truth_map: dict) -> str:
    groups = {group["id"]: group["title"] for group in truth_map.get("groups", [])}
    entries_by_group: dict[str, list[dict]] = {group_id: [] for group_id in groups}

    for entry in truth_map.get("entries", []):
        entries_by_group.setdefault(entry["group"], []).append(entry)

    lines: list[str] = []
    for group_id, title in groups.items():
        lines.append(f"### {title}")
        for entry in entries_by_group.get(group_id, []):
            lines.append(f"- {entry['id']} - {entry['title']}: `{entry['path']}`")
        lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def write_file(path: Path, content: str) -> None:
    path.write_text(content, encoding="utf-8")


def main() -> None:
    blueprint_root = Path(__file__).resolve().parents[1]

    truth_map_path = blueprint_root / "docs" / "meta" / "truth-map.json"
    template_path = blueprint_root / "docs" / "meta" / "agents.root.template.md"
    output_path = blueprint_root / "AGENTS.md"

    truth_map = json.loads(truth_map_path.read_text(encoding="utf-8-sig"))
    truth_map_md = render_truth_map(truth_map)

    template = template_path.read_text(encoding="utf-8")
    content = template.replace("{{TRUTH_MAP}}", truth_map_md)
    write_file(output_path, content)

    template_targets = {
        blueprint_root / "docs" / "meta" / "agents.hardware.template.md": blueprint_root
        / "hardware"
        / "AGENTS.md",
        blueprint_root / "docs" / "meta" / "agents.software.template.md": blueprint_root
        / "software"
        / "AGENTS.md",
    }

    for template_file, target_file in template_targets.items():
        if template_file.exists():
            write_file(target_file, template_file.read_text(encoding="utf-8"))


if __name__ == "__main__":
    main()
