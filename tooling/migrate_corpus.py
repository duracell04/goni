#!/usr/bin/env python3
"""Deterministically reconstruct the legacy GONI blueprint as atomic nodes.

This is a one-wave migration tool. It reads the immutable baseline Git tree,
never the partially migrated working tree, then emits canonical nodes, owned
artifacts, archived governance inputs, source records, and a complete ledger.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path, PurePosixPath
from typing import Any

import yaml


BASELINE_DEFAULT = "pre-knowledge-graph-2026-08-03"
NODE_DIRS = {
    "thesis": "theses",
    "principle": "principles",
    "specification": "specifications",
    "decision": "decisions",
    "proposal": "proposals",
    "objection": "objections",
    "evidence": "evidence",
    "experiment": "experiments",
    "synthesis": "syntheses",
    "glossary": "glossaries",
    "implementation-map": "implementation-maps",
}
ID_PREFIX = {
    "thesis": "THESIS",
    "principle": "PRINCIPLE",
    "specification": "SPEC",
    "decision": "DECISION",
    "proposal": "PROPOSAL",
    "objection": "OBJECTION",
    "evidence": "EVIDENCE",
    "experiment": "EXPERIMENT",
    "synthesis": "SYNTHESIS",
    "glossary": "GLOSSARY",
    "implementation-map": "IMAP",
}
STABLE_ID = re.compile(r"^[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+$")
HEADING = re.compile(r"^(#{1,6})\s+(.+?)\s*$")
FRONTMATTER = re.compile(r"\A---\s*\n(.*?)\n---\s*\n?", re.S)
URL = re.compile(r"https?://[^\s)>]+")


@dataclass
class Section:
    heading: str
    body: str
    anchor: str


def git(repo: Path, *args: str, text: bool = True) -> str | bytes:
    result = subprocess.run(
        ["git", "-c", "safe.directory=*", "-C", str(repo), *args],
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=text,
        encoding="utf-8" if text else None,
        errors="replace" if text else None,
    )
    return result.stdout


def baseline_bytes(repo: Path, baseline: str, path: str) -> bytes:
    return git(repo, "show", f"{baseline}:{path}", text=False)  # type: ignore[return-value]


def original_revision(repo: Path, baseline: str, path: str) -> str:
    value = git(repo, "log", "-1", "--format=%H", baseline, "--", path)
    return str(value).strip() or str(git(repo, "rev-parse", baseline)).strip()


def slug(value: str) -> str:
    value = re.sub(r"`([^`]+)`", r"\1", value)
    value = re.sub(r"[^a-zA-Z0-9]+", "-", value).strip("-").lower()
    return value[:96] or "section"


def clean_title(value: str) -> str:
    return re.sub(r"\s+#+$", "", value).strip()


def strip_frontmatter(text: str) -> tuple[dict[str, Any], str]:
    match = FRONTMATTER.match(text)
    if not match:
        return {}, text
    try:
        data = yaml.safe_load(match.group(1)) or {}
        if not isinstance(data, dict):
            data = {}
    except yaml.YAMLError:
        data = {}
    return data, text[match.end() :]


def split_sections(text: str, fallback_title: str) -> list[Section]:
    lines = text.splitlines()
    title = fallback_title
    intro: list[str] = []
    sections: list[Section] = []
    current_heading: str | None = None
    current: list[str] = []
    in_fence = False

    def flush() -> None:
        nonlocal current
        body = "\n".join(current).strip()
        heading = current_heading or title
        substantive = re.sub(r"[`#>*_\-|\s]", "", body)
        if body and len(substantive) >= 12:
            sections.append(Section(heading=heading, body=body + "\n", anchor=slug(heading)))
        current = []

    for line in lines:
        if line.lstrip().startswith("```") or line.lstrip().startswith("~~~"):
            in_fence = not in_fence
        match = None if in_fence else HEADING.match(line)
        if match and len(match.group(1)) == 1 and current_heading is None:
            title = clean_title(match.group(2))
            intro.append(line)
            continue
        if match and len(match.group(1)) in (2, 3):
            if current_heading is None:
                current = intro + current
            flush()
            current_heading = clean_title(match.group(2))
            current = [line]
            continue
        current.append(line)
    if current_heading is None:
        current = intro + current
    flush()
    return sections or [Section(title, text.strip() + "\n", slug(title))]


def first_proposition(title: str, body: str) -> str:
    plain_lines: list[str] = []
    in_fence = False
    for line in body.splitlines():
        if line.lstrip().startswith(("```", "~~~")):
            in_fence = not in_fence
            continue
        if in_fence or HEADING.match(line):
            continue
        line = re.sub(r"^\s*(?:[-*+] |\d+[.)]\s+|>\s*)", "", line).strip()
        line = re.sub(r"`([^`]+)`", r"\1", line)
        line = re.sub(r"\[([^]]+)]\([^)]+\)", r"\1", line)
        if line and not re.fullmatch(r"[-|: ]+", line):
            plain_lines.append(line)
        if len(" ".join(plain_lines)) > 320:
            break
    plain = re.sub(r"\s+", " ", " ".join(plain_lines)).strip()
    if not plain:
        return title
    sentence = re.split(r"(?<=[.!?])\s+", plain, maxsplit=1)[0]
    if len(sentence) < 20 and len(plain) > len(sentence):
        sentence = plain[:320].rsplit(" ", 1)[0]
    return sentence[:500].strip() or title


def path_type(path: str, heading: str = "", frontmatter_type: str = "") -> str:
    hay = f"{path} {heading}".lower()
    heading_lower = heading.lower()
    if any(word in heading_lower for word in ("objection", "counterargument", "failure mode", "limitation", "risk", "alternative")):
        return "objection"
    if any(word in heading_lower for word in ("validation plan", "evaluation", "benchmark", "experiment", "test plan")):
        return "experiment"
    if "glossar" in hay or "terminology" in heading_lower:
        return "glossary"
    if "50-evidence" in path or "/evidence/" in path:
        return "experiment"
    if "decision" in hay or re.search(r"\badr\b", hay):
        return "decision"
    if "axiom" in hay or "principle" in hay or "invariant" in heading_lower or "doctrine" in hay:
        return "principle"
    if "spec" in path or frontmatter_type.upper() == "SPEC" or "requirements" in hay or "/api/" in path or "/schemas/" in path:
        return "specification"
    if "thesis" in hay:
        return "thesis"
    if any(word in hay for word in ("implementation-map", "mapping", "prototype", "software/", "hardware/")) and "docs/" not in path:
        return "implementation-map"
    if any(word in heading_lower for word in ("proposal", "recommendation", "roadmap", "next step")):
        return "proposal"
    if any(part in path for part in ("10-product/", "20-system/", "docs/", "README.md", "00-index.md")):
        return "synthesis"
    return "proposal"


def domain_tags(path: str) -> list[str]:
    candidates = [
        "product", "system", "specs", "hardware", "software", "security",
        "data", "kernel", "memory", "network", "policy", "agent", "billing",
        "crdt", "market", "community", "validation", "research",
    ]
    lower = path.lower()
    result = [item for item in candidates if item in lower]
    return sorted(set(result or ["repository"]))


def deterministic_id(node_type: str, seed: str, used: set[str]) -> str:
    digest = hashlib.sha256(seed.encode("utf-8")).hexdigest().upper()
    for size in range(12, 65, 2):
        candidate = f"GONI-{ID_PREFIX[node_type]}-{digest[:size]}"
        if candidate not in used:
            used.add(candidate)
            return candidate
    raise RuntimeError(f"Unable to allocate unique ID for {seed}")


def extract_explicit_ids(frontmatter: dict[str, Any], body: str, path: str) -> list[str]:
    values: list[str] = []
    for key in ("id", "doc_id", "doc-id", "spec_id", "evidence_id"):
        value = frontmatter.get(key)
        if isinstance(value, str):
            values.append(value.strip().upper())
    for match in re.finditer(r"(?im)^\s*(?:DOC[- ]?ID|SPEC[- ]?ID|EVIDENCE[- ]?ID)\s*:\s*`?([A-Z][A-Z0-9-]+)`?\s*$", body):
        values.append(match.group(1).upper())
    stem = PurePosixPath(path).stem.upper()
    stem = re.sub(r"^(?:SPEC|EVID)-", lambda m: m.group(0), stem)
    if STABLE_ID.fullmatch(stem):
        values.append(stem)
    return list(dict.fromkeys(value for value in values if STABLE_ID.fullmatch(value)))


def stable_path_map(repo: Path, baseline: str) -> dict[str, list[str]]:
    result: dict[str, list[str]] = {}
    registry = yaml.safe_load(baseline_bytes(repo, baseline, "blueprint/30-specs/registry.yml"))
    for entry in registry.get("entries", []):
        path = "blueprint/" + entry["path"].lstrip("/")
        result.setdefault(path, []).append(entry["id"])
    truth = json.loads(baseline_bytes(repo, baseline, "blueprint/docs/meta/truth-map.json"))
    for entry in truth.get("entries", []):
        path = "blueprint/" + entry["path"].lstrip("/")
        result.setdefault(path, []).append(entry["id"])
    return {path: list(dict.fromkeys(ids)) for path, ids in result.items()}


def yaml_frontmatter(metadata: dict[str, Any]) -> str:
    return "---\n" + yaml.safe_dump(metadata, sort_keys=False, allow_unicode=True, width=1000) + "---\n"


def write_node(repo: Path, metadata: dict[str, Any], body: str) -> str:
    folder = NODE_DIRS[metadata["type"]]
    filename = f"{metadata['id'].lower()}-{slug(metadata['title'])[:60]}.md"
    relative = PurePosixPath("knowledge", folder, filename).as_posix()
    target = repo / Path(relative)
    target.parent.mkdir(parents=True, exist_ok=True)
    boundary = (
        "> Status boundary: this is a migrated draft. For `specified_only` nodes, "
        "present-tense or enforcement language below states intended contract behavior, "
        "not observed implementation, verification, or non-bypassability.\n"
    )
    target.write_text(yaml_frontmatter(metadata) + f"\n# {metadata['title']}\n\n" + boundary + "\n" + body.strip() + "\n", encoding="utf-8", newline="\n")
    return relative


def bibliography_records(
    repo: Path,
    path: str,
    text: str,
    revision: str,
    used_ids: set[str],
    primary_by_legacy: dict[str, str],
) -> tuple[list[str], list[dict[str, Any]]]:
    blocks = re.split(r"(?m)(?=^Key:\s*\[\[)", text)
    node_ids: list[str] = []
    source_records: list[dict[str, Any]] = []
    for block in blocks:
        key_match = re.search(r"(?m)^Key:\s*\[\[([^]]+)]]", block)
        if not key_match:
            continue
        key = key_match.group(1)
        normalized = re.sub(r"[^A-Z0-9]+", "-", key.upper()).strip("-")
        source_id = f"SRC-{normalized}"
        claim_match = re.search(r"(?ms)^Claim:\s*(.*?)(?=^Relevance:|^Used in:|^Source:|\Z)", block)
        claim = re.sub(r"\s+", " ", claim_match.group(1)).strip() if claim_match else f"Annotated source {key}"
        urls = URL.findall(block)
        citation = urls[0] if urls else f"Legacy annotated bibliography key [[{key}]]"
        record: dict[str, Any] = {
            "id": source_id,
            "title": key,
            "kind": "academic" if any(host in citation for host in ("arxiv.org", "doi.org", "papers", "proceedings")) else "web",
            "citation": citation,
            "notes": claim,
        }
        if urls:
            record["url"] = urls[0]
        source_path = repo / "sources" / f"{source_id.lower()}.yml"
        source_path.write_text(yaml.safe_dump(record, sort_keys=False, allow_unicode=True, width=1000), encoding="utf-8", newline="\n")
        source_records.append(record)

        node_id = deterministic_id("evidence", f"{path}::{key}", used_ids)
        used_targets: list[str] = []
        used_match = re.search(r"(?ms)^Used in:\s*(.*?)(?=^Source:|\Z)", block)
        if used_match:
            for legacy in re.findall(r"`(blueprint/[^`]+\.md)`", used_match.group(1)):
                if legacy in primary_by_legacy:
                    used_targets.append(primary_by_legacy[legacy])
        metadata = {
            "id": node_id,
            "title": f"Source claim: {key}",
            "type": "evidence",
            "status": "draft",
            "implementation_state": "not_applicable",
            "proposition": claim,
            "domains": ["research"],
            "aliases": [],
            "relations": [{"type": "supports", "target": target} for target in sorted(set(used_targets))],
            "sources": [source_id],
            "artifacts": [],
            "uncertainty": "The legacy bibliography summarizes the source; the cited source must be consulted for scope and limitations.",
            "legacy": [{"path": path, "heading": f"Key: [[{key}]]", "revision": revision}],
        }
        write_node(repo, metadata, block.strip())
        node_ids.append(node_id)
    return node_ids, source_records


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--baseline", default=BASELINE_DEFAULT)
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    repo = args.repo.resolve()
    baseline = str(git(repo, "rev-parse", args.baseline)).strip()
    paths = [line for line in str(git(repo, "ls-tree", "-r", "--name-only", baseline)).splitlines() if line]
    path_ids = stable_path_map(repo, baseline)

    used_ids: set[str] = set()
    primary_by_legacy: dict[str, str] = {}
    prepared: dict[str, tuple[dict[str, Any], list[Section], str]] = {}

    markdown_paths = [p for p in paths if p.endswith(".md") and (p.startswith("blueprint/") or p in {"README.md", "SCOPE-CONTRACT.md"})]
    for path in markdown_paths:
        text = baseline_bytes(repo, baseline, path).decode("utf-8", errors="replace")
        fm, body = strip_frontmatter(text)
        sections = split_sections(body, PurePosixPath(path).stem.replace("-", " ").title())
        candidates = path_ids.get(path, []) + extract_explicit_ids(fm, body, path)
        candidates = list(dict.fromkeys(candidate.upper() for candidate in candidates if STABLE_ID.fullmatch(candidate.upper())))
        primary_type = path_type(path, sections[0].heading, str(fm.get("type", "")))
        if candidates:
            primary_id = candidates[0]
            used_ids.add(primary_id)
        else:
            primary_id = deterministic_id(primary_type, f"{path}::{sections[0].anchor}", used_ids)
        primary_by_legacy[path] = primary_id
        prepared[path] = (fm, sections, primary_id)

    ledger_entries: list[dict[str, Any]] = []
    all_node_paths: dict[str, str] = {}

    for path in paths:
        revision = original_revision(repo, baseline, path)
        base_entry: dict[str, Any] = {
            "legacy_path": path,
            "original_revision": revision,
            "primary_node": None,
            "extracted_nodes": [],
            "headings": [],
            "notes": "",
        }

        if path in markdown_paths:
            fm, sections, primary_id = prepared[path]
            explicit = path_ids.get(path, []) + extract_explicit_ids(fm, baseline_bytes(repo, baseline, path).decode("utf-8", errors="replace"), path)
            explicit = list(dict.fromkeys(value.upper() for value in explicit if STABLE_ID.fullmatch(value.upper())))
            aliases = [value for value in explicit if value != primary_id]
            section_nodes: list[tuple[Section, str, str]] = []
            for index, section in enumerate(sections):
                node_type = path_type(path, section.heading, str(fm.get("type", "")))
                node_id = primary_id if index == 0 else deterministic_id(node_type, f"{path}::{section.anchor}::{index}", used_ids)
                section_nodes.append((section, node_id, node_type))
            for index, (section, node_id, node_type) in enumerate(section_nodes):
                relations: list[dict[str, str]] = []
                if index == 0 and node_type == "synthesis":
                    relations = [{"type": "synthesizes", "target": item[1]} for item in section_nodes[1:]]
                metadata = {
                    "id": node_id,
                    "title": section.heading,
                    "type": node_type,
                    "status": "draft",
                    "implementation_state": "not_applicable" if node_type in {"evidence", "experiment", "glossary", "objection"} else "specified_only",
                    "proposition": first_proposition(section.heading, section.body),
                    "domains": domain_tags(path),
                    "aliases": aliases if index == 0 else [],
                    "relations": relations,
                    "sources": [],
                    "artifacts": [],
                    "uncertainty": "Preserved from the legacy draft without status promotion or newly inferred evidence strength.",
                    "legacy": [{"path": path, "heading": section.heading, "revision": revision}],
                }
                node_path = write_node(repo, metadata, section.body)
                all_node_paths[node_id] = node_path
                base_entry["extracted_nodes"].append(node_id)
                base_entry["headings"].append({"heading": section.heading, "node": node_id})
            base_entry["primary_node"] = primary_id
            base_entry["disposition"] = "extract"
            base_entry["notes"] = "Split deterministically at substantive level-two and level-three headings; normative status remains draft."
            ledger_entries.append(base_entry)
            continue

        if path.startswith("blueprint/"):
            raw = baseline_bytes(repo, baseline, path)
            generated_or_registry = (
                path.endswith("AGENTS.md")
                or path in {
                    "blueprint/30-specs/registry.yml",
                    "blueprint/docs/meta/truth-map.json",
                    "blueprint/scripts/blueprint_guard.py",
                }
            )
            if generated_or_registry or path.endswith("NOTICE"):
                target = repo / "archive" / "legacy" / Path(path)
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(raw)
                base_entry["disposition"] = "archive"
                base_entry["notes"] = "Archived as a non-canonical migration input; generated projections replace it."
            else:
                artifact_rel = PurePosixPath("artifacts", "legacy", path).as_posix()
                target = repo / Path(artifact_rel)
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(raw)
                node_id = deterministic_id("implementation-map", f"artifact::{path}", used_ids)
                metadata = {
                    "id": node_id,
                    "title": f"Legacy artifact map: {PurePosixPath(path).name}",
                    "type": "implementation-map",
                    "status": "draft",
                    "implementation_state": "specified_only",
                    "proposition": f"The artifact {PurePosixPath(path).name} records a legacy design input and does not by itself prove implementation.",
                    "domains": domain_tags(path),
                    "aliases": [],
                    "relations": [],
                    "sources": [],
                    "artifacts": [artifact_rel],
                    "uncertainty": "Artifact provenance is preserved; implementation and conformance remain unverified unless separately evidenced.",
                    "legacy": [{"path": path, "revision": revision}],
                }
                all_node_paths[node_id] = write_node(repo, metadata, f"Owned artifact: `{artifact_rel}`")
                base_entry["disposition"] = "artifact"
                base_entry["primary_node"] = node_id
                base_entry["extracted_nodes"] = [node_id]
                base_entry["notes"] = "Moved to node-owned artifacts with an implementation-map owner."
            ledger_entries.append(base_entry)
            continue

        if path in {"BROKEN-LINK-FIXES.md", "MOVE-DELETE-PLAN.md"}:
            target = repo / "archive" / "legacy" / "root" / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(baseline_bytes(repo, baseline, path))
            if (repo / path).exists():
                (repo / path).unlink()
            base_entry["disposition"] = "archive"
            base_entry["notes"] = "Completed migration planning record retained as non-canonical history."
        elif path == "SCOPE-CONTRACT.md":
            if (repo / path).exists():
                (repo / path).unlink()
            base_entry["disposition"] = "extract"
            base_entry["primary_node"] = primary_by_legacy.get(path)
            base_entry["notes"] = "Scope content extracted; repository operating policy now carries the live boundary."
        else:
            base_entry["disposition"] = "move"
            base_entry["notes"] = "Operational or community file retained outside the knowledge canon."
        ledger_entries.append(base_entry)

    bibliography_path = "blueprint/docs/references/bibliography.md"
    if bibliography_path in prepared:
        revision = original_revision(repo, baseline, bibliography_path)
        text = baseline_bytes(repo, baseline, bibliography_path).decode("utf-8", errors="replace")
        source_nodes, _ = bibliography_records(repo, bibliography_path, text, revision, used_ids, primary_by_legacy)
        entry = next(item for item in ledger_entries if item["legacy_path"] == bibliography_path)
        entry["extracted_nodes"].extend(source_nodes)
        entry["notes"] += " Each annotated source claim also became an evidence node and stable source record."

    blueprint = repo / "blueprint"
    if blueprint.exists():
        archived_tree = repo / "archive" / "corpus-2026-08-03" / "blueprint"
        archived_tree.parent.mkdir(parents=True, exist_ok=True)
        if archived_tree.exists():
            raise RuntimeError(f"Refusing to replace existing archive: {archived_tree}")
        shutil.move(str(blueprint), str(archived_tree))
    scope_contract = repo / "SCOPE-CONTRACT.md"
    if scope_contract.exists():
        scope_contract.unlink()

    readme = """# GONI — canonical blueprint knowledge graph

GONI is a blueprint and architectural plan for a local-first Delegation OS. It is not an implemented, enforced, verified, or non-bypassable runtime unless a specific node cites pinned implementation and test evidence.

## Start here

- `knowledge/` contains the canonical atomic nodes and explanatory syntheses.
- `maps/catalogue.json` is the deterministic machine projection.
- `EDITORIAL_POLICY.md` defines status, evidence, uncertainty, and provenance boundaries.
- `ontology/relations.yml` defines the controlled relation vocabulary.
- `migration/ledger.json` accounts for every file in the pre-migration repository.
- `artifacts/` contains node-owned non-narrative material.
- `archive/` is historical and non-canonical.

Every migrated node is `status: draft`. Design-bearing material defaults to `implementation_state: specified_only`.

Runnable experiments remain separate in [goni-prototype-lab](https://github.com/duracell04/goni-prototype-lab). Experimental cognitive architecture remains separate in [goni-cognitive-os](https://github.com/duracell04/goni-cognitive-os). Public documentation is a pinned-commit projection in [goni-docs-hub](https://github.com/duracell04/goni-docs-hub).

## Validate

```bash
python tooling/build_catalogue.py --edition-sha <full-git-sha>
python tooling/validate.py --strict --baseline pre-knowledge-graph-2026-08-03
```
"""
    (repo / "README.md").write_text(readme, encoding="utf-8", newline="\n")

    ledger = {"baseline": baseline, "entries": sorted(ledger_entries, key=lambda item: item["legacy_path"])}
    (repo / "migration" / "ledger.json").write_text(json.dumps(ledger, indent=2, sort_keys=True) + "\n", encoding="utf-8", newline="\n")
    subprocess.run([sys.executable, str(repo / "tooling" / "resolve_id_collisions.py")], check=True)
    subprocess.run([sys.executable, str(repo / "tooling" / "normalize_claims.py")], check=True)
    print(json.dumps({"baseline": baseline, "legacy_files": len(paths), "nodes": len(all_node_paths), "sources": len(list((repo / 'sources').glob('*.yml')))}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
