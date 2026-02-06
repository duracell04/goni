#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from pathlib import Path


def main() -> None:
    blueprint_root = Path(__file__).resolve().parents[1]
    truth_map_path = blueprint_root / "docs" / "meta" / "truth-map.json"

    truth_map = json.loads(truth_map_path.read_text(encoding="utf-8-sig"))
    errors: list[str] = []
    ids: dict[str, str] = {}

    for entry in truth_map.get("entries", []):
        entry_id = entry.get("id", "").strip()
        if not entry_id:
            errors.append("Missing id in truth map entry.")
            continue

        if entry_id in ids:
            errors.append(f"Duplicate id in truth map: {entry_id} ({ids[entry_id]})")
        else:
            ids[entry_id] = entry.get("path", "")

        rel_path = entry.get("path", "")
        target = blueprint_root / rel_path
        if not target.exists():
            errors.append(f"Missing file for {entry_id}: {rel_path}")
            continue

        anchors = entry.get("anchors", [])
        if anchors:
            content = target.read_text(encoding="utf-8", errors="ignore")
            for anchor in anchors:
                if anchor not in content:
                    errors.append(f"Missing anchor '{anchor}' in {rel_path}")

    if errors:
        for error in errors:
            print(error)
        sys.exit(1)


if __name__ == "__main__":
    main()
