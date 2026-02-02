import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TRUTH_MAP = ROOT / "docs" / "meta" / "truth-map.json"


def load_truth_map():
    with TRUTH_MAP.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def error(message):
    print(f"ERROR: {message}")


def validate():
    data = load_truth_map()
    entries = data.get("entries", [])

    seen_ids = set()
    ok = True

    for entry in entries:
        entry_id = entry.get("id")
        path = entry.get("path")
        anchors = entry.get("anchors", [])

        if not entry_id or not path:
            error(f"entry missing id/path: {entry}")
            ok = False
            continue

        if entry_id in seen_ids:
            error(f"duplicate id: {entry_id}")
            ok = False
        else:
            seen_ids.add(entry_id)

        file_path = ROOT / path
        if not file_path.exists():
            error(f"missing path for {entry_id}: {path}")
            ok = False
            continue

        content = file_path.read_bytes()
        for anchor in anchors:
            anchor_bytes = anchor.encode("ascii", errors="strict")
            if anchor_bytes not in content:
                error(f"missing anchor for {entry_id} in {path}: {anchor}")
                ok = False

    return ok


def main():
    if not validate():
        sys.exit(1)


if __name__ == "__main__":
    main()
