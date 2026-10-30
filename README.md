# GONI — canonical blueprint knowledge graph

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

## Architectural direction

Goni uses [Minimal Sufficient Software](knowledge/decisions/goni-decision-20261030-minimal-sufficient-software.md): adaptive intelligence provides flexible capability while a small trusted software foundation enforces authorization, integrity, and accountability. [Descriptive naming](knowledge/principles/goni-principle-20261030-explicit-naming.md), [human-governed capability growth](knowledge/principles/goni-principle-20261030-capability-growth-with-bounded-authority.md), [visual governance tracks](knowledge/specifications/goni-spec-20261030-visual-governance-tracks.md), and [typed natural-language mandate translation](knowledge/specifications/goni-spec-20261030-natural-language-policy-translation.md) are canonical design proposals, with implementation evidence tracked separately.

## Validate

```bash
python tooling/build_catalogue.py --edition-sha <full-git-sha>
python tooling/validate.py --strict --baseline pre-knowledge-graph-2026-08-03
```
