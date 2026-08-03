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

## Validate

```bash
python tooling/build_catalogue.py --edition-sha <full-git-sha>
python tooling/validate.py --strict --baseline pre-knowledge-graph-2026-08-03
```
