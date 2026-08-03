# AGENTS.md — repository operating instructions

## Scope and authority

This file applies repository-wide. The repository files, not an external task prompt, define operating rules. Read `EDITORIAL_POLICY.md`, `COMMIT_STANDARD.md`, `ontology/relations.yml`, and the applicable schemas before changing canonical knowledge.

GONI is a blueprint and architectural plan. Do not describe a specified mechanism as implemented, enforced, verified, validated, guaranteed, or non-bypassable without repository-and-full-commit-pinned implementation evidence and a stated boundary.

## Canonical content

- Only `knowledge/` contains canonical knowledge nodes.
- Every node is atomic, permanently identified, and `status: draft` unless a later explicit governance decision promotes it.
- `implementation_state` is factual and independent from status. Use `specified_only` for design-bearing claims unless pinned evidence supports more.
- Syntheses explain and link; they do not duplicate normative prose.
- Preserve objections, alternatives, uncertainty, derivation, sources, and legacy provenance.
- Author forward relations only. Reverse indexes are generated.
- Planned evaluations are experiments, not evidence.

## Required validation

Run:

```text
python tooling/build_catalogue.py --edition-sha <full-git-sha>
python tooling/validate.py --strict --baseline pre-knowledge-graph-2026-08-03
```

The deterministic catalogue must be byte-identical on repeated builds from the same tree.

## Git completion contract

A task that changes repository files reaches completion when:

1. The assigned logical change passes its required validation.
2. The change is committed with the repository-defined commit anatomy.
3. Every changed file is accounted for in the commit message.
4. The topic branch is pushed with an upstream.
5. A draft pull request is created or updated.
6. The final response reports the branch, commit SHA, validation results, and pull-request URL.

For coordinated work, each agent commits and publishes its own isolated workstream. The coordinator integrates completed branches through the repository's approved merge strategy and publishes the integration result.

GitHub branch publication and draft pull-request creation are standard completion actions for authorized repository work.
