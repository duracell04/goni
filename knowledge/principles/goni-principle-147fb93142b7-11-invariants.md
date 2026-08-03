---
id: GONI-PRINCIPLE-147FB93142B7
title: 11. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: Observation does not imply extraction.
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/vision-memory-actuation-boundaries.md
  heading: 11. Invariants
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# 11. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 11. Invariants

- Observation does not imply extraction.
- Extraction does not imply memory.
- Memory does not imply actuation.
- Actuation does not imply authority.
- Synthetic input requires a capability token and tool mediation.
- Remote extraction routes through the Network Gate.
- Continuous memory capture requires a memory grant.
- Every allowed or denied boundary transition emits a receipt or receipt-linked
  audit record.
- Raw private screenshots, full OCR text, accessibility dumps, audio
  transcripts, and unbounded prompts are not stored in receipts by default.
- If required boundary policy, sandbox, approval, or receipt support is
  unavailable, execution fails closed.
