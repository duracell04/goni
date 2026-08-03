---
id: GONI-SYNTHESIS-45F2FF4D1E6D
title: 5) Council
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Purpose: policy-gated escalation path for high-risk, high-uncertainty, or long-context tasks.'
domains:
- system
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/20-system/10-primitives.md
  heading: 5) Council
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 5) Council

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5) Council
- Purpose: policy-gated escalation path for high-risk, high-uncertainty, or long-context tasks.
- Contract anchors: `docs/llm-council.md`, `docs/remote-llm-architecture.md`.
- Core invariant: council is optional and never bypasses policy, budgets, or network gate.
- Metrics: escalation rate, added latency, and resource overhead per escalated execution.
