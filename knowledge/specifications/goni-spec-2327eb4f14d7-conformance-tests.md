---
id: GONI-SPEC-2327EB4F14D7
title: Conformance tests
type: specification
status: draft
implementation_state: specified_only
proposition: A single correction produces a scoped hypothesis, not a global preference.
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/correction-delta-compiler.md
  heading: Conformance tests
  revision: e3e487b4f8de4b5cdd83d5be45e0f966f2cb4a8a
---

# Conformance tests

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## Conformance tests

- A single correction produces a scoped hypothesis, not a global preference.
- Repeated consistent corrections increase confidence only within matching
  scope.
- Contradictory corrections reduce confidence, narrow scope, or require review.
- Accepted learning emits `MemoryEntry`, receipt, and regression test refs.
- Learning receipts omit raw content by default.
- High-risk or constitutional preferences require explicit approval.
- Prompt, retrieval, policy, or harness changes attach to declared seams.
