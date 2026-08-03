#!/usr/bin/env python3
"""Resolve ambiguous legacy ID reuse while retaining one stable canonical owner."""

from __future__ import annotations

import hashlib
import json
from collections import defaultdict
from pathlib import Path

import yaml

from build_catalogue import load_node


TYPE_PREFIX = {
    "thesis": "THESIS", "principle": "PRINCIPLE", "specification": "SPEC",
    "decision": "DECISION", "proposal": "PROPOSAL", "objection": "OBJECTION",
    "evidence": "EVIDENCE", "experiment": "EXPERIMENT", "synthesis": "SYNTHESIS",
    "glossary": "GLOSSARY", "implementation-map": "IMAP",
}


def priority(node: dict) -> tuple[int, str]:
    legacy = node["legacy"][0]["path"]
    if "/30-specs/" in legacy or "/20-system/" in legacy:
        score = 0
    elif "/hardware/" in legacy:
        score = 1
    elif "/software/" in legacy:
        score = 2
    elif "/docs/hubs/" in legacy or "taxonomy" in legacy:
        score = 8
    else:
        score = 4
    return score, legacy


def allocate(node: dict, used: set[str]) -> str:
    seed = node["legacy"][0]["path"] + "::" + node["legacy"][0].get("heading", node["title"])
    digest = hashlib.sha256(seed.encode("utf-8")).hexdigest().upper()
    prefix = TYPE_PREFIX[node["type"]]
    for size in range(12, 65, 2):
        candidate = f"GONI-{prefix}-{digest[:size]}"
        if candidate not in used:
            used.add(candidate)
            return candidate
    raise RuntimeError(seed)


def render(node: dict) -> str:
    body = node.pop("body")
    node.pop("path")
    return "---\n" + yaml.safe_dump(node, sort_keys=False, allow_unicode=True, width=1000) + "---\n\n" + body.strip() + "\n"


def main() -> int:
    repo = Path(__file__).resolve().parents[1]
    paths = sorted((path for path in (repo / "knowledge").rglob("*.md") if path.name != "README.md"), key=lambda item: item.as_posix())
    nodes = [load_node(path, repo) for path in paths]
    groups: dict[str, list[dict]] = defaultdict(list)
    for node in nodes:
        groups[node["id"]].append(node)
    used = {node["id"] for node in nodes}
    ledger_path = repo / "migration" / "ledger.json"
    ledger = json.loads(ledger_path.read_text(encoding="utf-8"))
    entry_by_path = {entry["legacy_path"]: entry for entry in ledger["entries"]}
    decisions: list[str] = []

    for old_id, duplicates in sorted(groups.items()):
        if len(duplicates) < 2:
            continue
        ordered = sorted(duplicates, key=priority)
        keeper = ordered[0]
        decisions.append(f"- `{old_id}` remains owned by `{keeper['legacy'][0]['path']}`.")
        for node in ordered[1:]:
            old_path = repo / node["path"]
            legacy_path = node["legacy"][0]["path"]
            new_id = allocate(node, used)
            decisions.append(f"  - Ambiguous reuse in `{legacy_path}` became `{new_id}`; the legacy pointer preserves the collision boundary.")
            node["id"] = new_id
            new_name = old_path.name.replace(old_id.lower(), new_id.lower(), 1)
            new_path = old_path.with_name(new_name)
            new_path.write_text(render(dict(node)), encoding="utf-8", newline="\n")
            old_path.unlink()
            entry = entry_by_path[legacy_path]
            if entry["primary_node"] == old_id:
                entry["primary_node"] = new_id
            entry["extracted_nodes"] = [new_id if value == old_id else value for value in entry["extracted_nodes"]]
            for heading in entry.get("headings", []):
                if heading.get("node") == old_id:
                    heading["node"] = new_id
            entry["notes"] += f" Legacy ID {old_id} was reused elsewhere; canonical ownership remains singular and this node received {new_id}."

    ledger_path.write_text(json.dumps(ledger, indent=2, sort_keys=True) + "\n", encoding="utf-8", newline="\n")
    report = "# Legacy ID collision decisions\n\nLegacy files reused several IDs for unrelated propositions. A stable ID can resolve to only one canonical node, so the coordinator retained it on the strongest domain document and assigned deterministic IDs to the other propositions. No legacy path or content was discarded.\n\n" + "\n".join(decisions) + "\n"
    (repo / "migration" / "id-collisions.md").write_text(report, encoding="utf-8", newline="\n")
    print(f"collision_groups={len([items for items in groups.values() if len(items) > 1])}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
