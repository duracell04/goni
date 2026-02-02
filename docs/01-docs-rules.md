# Docs Rules
DOC-ID: SYS-01
Status: Specified only / roadmap

These rules keep the blueprint documentation consistent, status-honest, and
traceable.

## Canonical sources
- Specs live in [docs/specs/](./specs/00-index.md) and are the normative
  contracts.
- Data plane contracts live under [software/50-data/](../software/50-data/00-index.md).
- Decisions live in [software/90-decisions.md](../software/90-decisions.md) and
  [hardware/90-decisions.md](../hardware/90-decisions.md).
- The canonical file list is maintained in the truth map
  [docs/meta/truth-map.json](./meta/truth-map.json).

## Status vocabulary
When describing status, use exactly one of:
- Implemented and tested (with evidence link)
- Implemented (untested)
- Specified only / roadmap

Avoid claiming "enforced", "verified", or "validated" without evidence.

## Spec vs implementation
- Specs define contracts and invariants.
- Implementation details belong in code or prototype docs.
- Do not duplicate definitions across multiple docs.

## Evidence links
If you assert "enforced", "supported", "measured", or "verified", link to
tests, ADRs, benchmarks, or dated measurements.

## Link discipline
- Prefer relative links over plain-text paths.
- Add upstream/downstream/adjacent links when a doc affects other parts.
- Folder indexes are required when a directory grows beyond a few docs.

## Generated files
Do not edit AGENTS.md directly. Update templates under docs/meta/ and regenerate.
