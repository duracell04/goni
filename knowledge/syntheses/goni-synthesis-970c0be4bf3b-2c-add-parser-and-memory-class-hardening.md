---
id: GONI-SYNTHESIS-970C0BE4BF3B
title: 2c) Add parser and memory-class hardening
type: synthesis
status: draft
implementation_state: specified_only
proposition: Add parser/ingestion fixtures with source hash, parser ID/version, chunk boundaries, confidence flags, and expected parser_basis.
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/30-next-steps.md
  heading: 2c) Add parser and memory-class hardening
  revision: 050465b8d1a68fe8cc36e542344414705c3e08a7
---

# 2c) Add parser and memory-class hardening

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2c) Add parser and memory-class hardening
- Add parser/ingestion fixtures with source hash, parser ID/version, chunk
  boundaries, confidence flags, and expected `parser_basis`.
- Add memory-class policy examples for `personal_private`, `project_shared`,
  `relationship`, `model_system`, `ephemeral`, and `quarantine`.
- Verify quarantine and expired memory are absent from normal retrieval.
