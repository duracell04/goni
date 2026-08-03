---
id: GONI-SPEC-5CD378282A73
title: 4.1 Memory write policy (normative)
type: specification
status: draft
implementation_state: specified_only
proposition: 'MemoryEntries are governed by explicit write gates: kind MUST be one of: fact, preference, decision, hypothesis, derived.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/latent-state-contract.md
  heading: 4.1 Memory write policy (normative)
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 4.1 Memory write policy (normative)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4.1 Memory write policy (normative)

MemoryEntries are governed by explicit write gates:
- `kind` MUST be one of: `fact`, `preference`, `decision`, `hypothesis`, `derived`.
- `fact` MUST include at least one `source_chunk_id` or a `confirmed_by_event_id`.
- `hypothesis` MUST set `ttl_ms` or `review_at` and MUST NOT be promoted to `fact`
  without a confirmation event or new source evidence.
- `derived` MUST include source references and provenance for the transform.

Writes that do not meet these requirements MUST be rejected by policy checks.
