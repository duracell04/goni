---
id: GONI-SYNTHESIS-F7FB0BF591B6
title: 4) Make the kernel pipeline canonical
type: synthesis
status: draft
implementation_state: specified_only
proposition: Move receipt emission into the kernel pipeline (not just HTTP handlers).
domains:
- product
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/10-product/30-next-steps.md
  heading: 4) Make the kernel pipeline canonical
  revision: 050465b8d1a68fe8cc36e542344414705c3e08a7
---

# 4) Make the kernel pipeline canonical

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 4) Make the kernel pipeline canonical
- Move receipt emission into the kernel pipeline (not just HTTP handlers).
- Make policy checks unavoidable:
  - memory write gate
  - redaction/egress gate
- Ensure CLI/jobs/UI use the same pipeline.
