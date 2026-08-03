#!/usr/bin/env python3
"""Strict graph, provenance, determinism, and commit-history validation."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tempfile
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

import jsonschema
import yaml

from build_catalogue import catalogue, load_node, load_sources, relation_policy


FULL_SHA = re.compile(r"^[0-9a-f]{40}$")
MESSAGE_SECTIONS = ("Intent:", "Rationale:", "Status:", "Contracts:", "Files:", "Evidence:", "Unresolved:")


def git(repo: Path, *args: str) -> str:
    return subprocess.run(
        ["git", "-c", "safe.directory=*", "-C", str(repo), *args],
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        encoding="utf-8",
        errors="replace",
    ).stdout


def schema(repo: Path, name: str) -> dict[str, Any]:
    return json.loads((repo / "schema" / name).read_text(encoding="utf-8"))


def frontmatter_paths(repo: Path) -> list[Path]:
    return sorted((path for path in (repo / "knowledge").rglob("*.md") if path.name != "README.md"), key=lambda item: item.as_posix())


def cycle(nodes: set[str], edges: list[tuple[str, str]]) -> bool:
    graph: dict[str, list[str]] = defaultdict(list)
    for source, target in edges:
        graph[source].append(target)
    visiting: set[str] = set()
    visited: set[str] = set()

    def visit(node: str) -> bool:
        if node in visiting:
            return True
        if node in visited:
            return False
        visiting.add(node)
        if any(visit(target) for target in graph[node]):
            return True
        visiting.remove(node)
        visited.add(node)
        return False

    return any(visit(node) for node in nodes)


def changed_paths(repo: Path, commit: str) -> list[str]:
    parents = git(repo, "rev-list", "--parents", "-n", "1", commit).split()
    if len(parents) == 1:
        return sorted(git(repo, "diff-tree", "--no-commit-id", "--name-only", "-r", "--root", commit).splitlines())
    return sorted(git(repo, "diff", "--name-only", parents[1], commit).splitlines())


def message_files(message: str) -> list[str]:
    match = re.search(r"(?ms)^Files:\s*\n(.*?)(?=^Evidence:)", message)
    if not match:
        return []
    return sorted(line[2:].strip() for line in match.group(1).splitlines() if line.startswith("- "))


def validate_commits(repo: Path, baseline: str, errors: list[str]) -> None:
    commits = git(repo, "rev-list", "--reverse", f"{baseline}..HEAD").splitlines()
    for commit in commits:
        message = git(repo, "show", "-s", "--format=%B", commit)
        if not all(section in message for section in MESSAGE_SECTIONS):
            errors.append(f"commit {commit[:12]} does not contain the complete COMMIT_STANDARD anatomy")
            continue
        declared = message_files(message)
        actual = changed_paths(repo, commit)
        if declared != actual:
            errors.append(f"commit {commit[:12]} Files section differs from first-parent diff")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--strict", action="store_true")
    parser.add_argument("--baseline", default="pre-knowledge-graph-2026-08-03")
    parser.add_argument("--skip-commits", action="store_true")
    args = parser.parse_args()
    repo = args.repo.resolve()
    errors: list[str] = []

    node_schema = schema(repo, "node.schema.json")
    source_schema = schema(repo, "source.schema.json")
    ledger_schema = schema(repo, "migration-ledger.schema.json")
    nodes: list[dict[str, Any]] = []
    for path in frontmatter_paths(repo):
        try:
            node = load_node(path, repo)
            public = {key: value for key, value in node.items() if key not in {"path", "body"}}
            jsonschema.validate(public, node_schema)
            nodes.append(node)
        except Exception as exc:  # noqa: BLE001 - aggregate every validation failure
            errors.append(f"{path.relative_to(repo).as_posix()}: {exc}")

    sources = load_sources(repo)
    for source in sources:
        try:
            jsonschema.validate({key: value for key, value in source.items() if key != "path"}, source_schema)
        except Exception as exc:  # noqa: BLE001
            errors.append(f"{source.get('path', source.get('id'))}: {exc}")

    ledger_path = repo / "migration" / "ledger.json"
    ledger = json.loads(ledger_path.read_text(encoding="utf-8"))
    try:
        jsonschema.validate(ledger, ledger_schema)
    except Exception as exc:  # noqa: BLE001
        errors.append(f"migration/ledger.json: {exc}")

    node_ids = [node["id"] for node in nodes]
    aliases = [alias for node in nodes for alias in node.get("aliases", [])]
    source_ids = [source["id"] for source in sources]
    for label, values in (("node ID", node_ids), ("alias", aliases), ("source ID", source_ids)):
        duplicates = sorted(value for value, count in Counter(values).items() if count > 1)
        if duplicates:
            errors.append(f"duplicate {label}s: {', '.join(duplicates)}")
    overlap = sorted(set(node_ids) & set(aliases))
    if overlap:
        errors.append(f"aliases collide with node IDs: {', '.join(overlap)}")

    known_nodes = set(node_ids)
    known_sources = set(source_ids)
    policy = relation_policy(repo)
    authored_edges: list[tuple[str, str, str]] = []
    artifact_owners: dict[str, list[str]] = defaultdict(list)
    for node in nodes:
        if node["status"] != "draft":
            errors.append(f"{node['id']}: migrated status must remain draft")
        if node["implementation_state"] in {"implemented_untested", "implemented_tested"} and not any(edge["type"] == "implements" for edge in node.get("relations", [])):
            errors.append(f"{node['id']}: stronger implementation state lacks pinned implements relation")
        if node["type"] == "evidence" and re.search(r"(?im)^\s*(?:goal|artifact links)\s*:\s*(?:verify|\n-\s*TBD)", node.get("body", "")):
            errors.append(f"{node['id']}: planned evaluation is typed as evidence")
        for source_id in node.get("sources", []):
            if source_id not in known_sources:
                errors.append(f"{node['id']}: unknown source {source_id}")
        for artifact in node.get("artifacts", []):
            artifact_owners[artifact].append(node["id"])
            if not (repo / artifact).is_file():
                errors.append(f"{node['id']}: dangling artifact {artifact}")
        seen: set[tuple[str, str]] = set()
        for edge in node.get("relations", []):
            relation_type, target = edge["type"], edge["target"]
            if relation_type not in policy:
                errors.append(f"{node['id']}: unknown relation type {relation_type}")
            if target not in known_nodes:
                errors.append(f"{node['id']}: dangling relation target {target}")
            if target == node["id"]:
                errors.append(f"{node['id']}: self relation {relation_type}")
            if (relation_type, target) in seen:
                errors.append(f"{node['id']}: duplicate relation {relation_type} -> {target}")
            seen.add((relation_type, target))
            authored_edges.append((node["id"], relation_type, target))
    supersession = [(source, target) for source, rel, target in authored_edges if rel == "supersedes"]
    if cycle(known_nodes, supersession):
        errors.append("supersession relation contains a cycle")

    for path in sorted((repo / "artifacts").rglob("*"), key=lambda item: item.as_posix()):
        rel = path.relative_to(repo).as_posix()
        if path.is_file() and path.name != "README.md" and rel not in artifact_owners:
            errors.append(f"unowned artifact: {rel}")

    baseline_sha = git(repo, "rev-parse", args.baseline).strip()
    baseline_paths = sorted(git(repo, "ls-tree", "-r", "--name-only", baseline_sha).splitlines())
    ledger_paths = [entry["legacy_path"] for entry in ledger["entries"]]
    if sorted(ledger_paths) != baseline_paths:
        missing = sorted(set(baseline_paths) - set(ledger_paths))
        extra = sorted(set(ledger_paths) - set(baseline_paths))
        errors.append(f"ledger coverage mismatch; missing={missing} extra={extra}")
    duplicate_legacy = sorted(value for value, count in Counter(ledger_paths).items() if count > 1)
    if duplicate_legacy:
        errors.append(f"duplicate ledger paths: {', '.join(duplicate_legacy)}")
    for entry in ledger["entries"]:
        for node_id in entry["extracted_nodes"]:
            if node_id not in known_nodes:
                errors.append(f"{entry['legacy_path']}: ledger references missing node {node_id}")
        if entry["primary_node"] and entry["primary_node"] not in known_nodes:
            errors.append(f"{entry['legacy_path']}: missing primary node {entry['primary_node']}")

    generated = catalogue(repo)
    expected = (json.dumps(generated, indent=2, sort_keys=True, ensure_ascii=False) + "\n").encode("utf-8")
    catalogue_path = repo / "maps" / "catalogue.json"
    if not catalogue_path.is_file() or catalogue_path.read_bytes() != expected:
        errors.append("maps/catalogue.json is absent or differs from deterministic projection")

    if args.strict and not args.skip_commits:
        validate_commits(repo, baseline_sha, errors)

    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        print(json.dumps({"ok": False, "errors": len(errors), "nodes": len(nodes), "sources": len(sources)}, sort_keys=True))
        return 1
    print(json.dumps({"ok": True, "nodes": len(nodes), "sources": len(sources), "ledger_entries": len(ledger["entries"])}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
