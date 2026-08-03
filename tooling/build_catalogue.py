#!/usr/bin/env python3
"""Build deterministic public projections from canonical Markdown and metadata."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any

import yaml


FRONTMATTER = re.compile(r"\A---\s*\n(.*?)\n---\s*\n?", re.S)
FULL_SHA = re.compile(r"^[0-9a-f]{40}$")


def load_node(path: Path, repo: Path) -> dict[str, Any]:
    text = path.read_text(encoding="utf-8")
    match = FRONTMATTER.match(text)
    if not match:
        raise ValueError(f"Missing frontmatter: {path.relative_to(repo).as_posix()}")
    metadata = yaml.safe_load(match.group(1)) or {}
    if not isinstance(metadata, dict):
        raise ValueError(f"Frontmatter is not an object: {path.relative_to(repo).as_posix()}")
    result = dict(metadata)
    result["path"] = path.relative_to(repo).as_posix()
    result["body"] = text[match.end() :].strip()
    return result


def load_sources(repo: Path) -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []
    for path in sorted((repo / "sources").glob("*.yml"), key=lambda item: item.as_posix()):
        value = yaml.safe_load(path.read_text(encoding="utf-8")) or {}
        value["path"] = path.relative_to(repo).as_posix()
        records.append(value)
    return sorted(records, key=lambda item: item["id"])


def relation_policy(repo: Path) -> dict[str, dict[str, Any]]:
    ontology = yaml.safe_load((repo / "ontology" / "relations.yml").read_text(encoding="utf-8"))
    return {item["id"]: item for item in ontology["relations"]}


def catalogue(repo: Path) -> dict[str, Any]:
    nodes = [load_node(path, repo) for path in sorted((repo / "knowledge").rglob("*.md"), key=lambda item: item.as_posix()) if path.name != "README.md"]
    nodes.sort(key=lambda item: item["id"])
    sources = load_sources(repo)
    policy = relation_policy(repo)

    authored: list[dict[str, str]] = []
    generated: list[dict[str, str]] = []
    for node in nodes:
        for relation in node.get("relations", []):
            edge = {"source": node["id"], "type": relation["type"], "target": relation["target"], "projection": "authored"}
            if relation.get("note"):
                edge["note"] = relation["note"]
            authored.append(edge)
            definition = policy.get(relation["type"], {})
            inverse = definition.get("inverse")
            if inverse:
                generated.append({"source": relation["target"], "type": inverse, "target": node["id"], "projection": "generated"})
            elif definition.get("symmetric"):
                generated.append({"source": relation["target"], "type": relation["type"], "target": node["id"], "projection": "generated"})
    relations = sorted(authored + generated, key=lambda item: (item["source"], item["type"], item["target"], item["projection"]))

    aliases = sorted(
        ({"alias": alias, "node": node["id"]} for node in nodes for alias in node.get("aliases", [])),
        key=lambda item: (item["alias"], item["node"]),
    )

    def index(field: str) -> dict[str, list[str]]:
        values: dict[str, list[str]] = {}
        for node in nodes:
            raw = node.get(field, [])
            for value in raw if isinstance(raw, list) else [raw]:
                values.setdefault(value, []).append(node["id"])
        return {key: sorted(value) for key, value in sorted(values.items())}

    contested = sorted({edge["source"] for edge in relations if edge["type"] in {"objects_to", "objected_to_by", "conflicts_with"}} | {edge["target"] for edge in relations if edge["type"] in {"objects_to", "objected_to_by", "conflicts_with"}})

    ledger = json.loads((repo / "migration" / "ledger.json").read_text(encoding="utf-8"))
    legacy_routes = []
    for entry in ledger["entries"]:
        if entry["primary_node"]:
            legacy_routes.append({
                "legacy_path": entry["legacy_path"],
                "primary_node": entry["primary_node"],
                "extracted_nodes": sorted(entry["extracted_nodes"]),
            })

    return {
        "schema_version": 1,
        "nodes": nodes,
        "relations": relations,
        "sources": sources,
        "aliases": aliases,
        "indexes": {
            "by_type": index("type"),
            "by_domain": index("domains"),
            "by_status": index("status"),
            "contested": contested,
        },
        "legacy_routes": sorted(legacy_routes, key=lambda item: item["legacy_path"]),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--edition-sha", required=True)
    args = parser.parse_args()
    if not FULL_SHA.fullmatch(args.edition_sha):
        parser.error("--edition-sha must be a full lowercase 40-character Git SHA")
    repo = args.repo.resolve()
    result = catalogue(repo)
    payload = (json.dumps(result, indent=2, sort_keys=True, ensure_ascii=False) + "\n").encode("utf-8")
    maps = repo / "maps"
    maps.mkdir(exist_ok=True)
    (maps / "catalogue.json").write_bytes(payload)
    edition = {
        "schema_version": 1,
        "repository": "duracell04/goni",
        "sha": args.edition_sha,
        "catalogue_sha256": hashlib.sha256(payload).hexdigest(),
    }
    (maps / "edition.json").write_text(json.dumps(edition, indent=2, sort_keys=True) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"nodes": len(result["nodes"]), "relations": len(result["relations"]), "sources": len(result["sources"]), "catalogue_sha256": edition["catalogue_sha256"]}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
