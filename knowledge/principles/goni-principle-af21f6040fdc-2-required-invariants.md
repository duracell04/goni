---
id: GONI-PRINCIPLE-AF21F6040FDC
title: 2. Required invariants
type: principle
status: draft
implementation_state: specified_only
proposition: '**Policy mediation:** every tool call and state write is policy-checked and audited.'
domains:
- agent
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/agent-definition.md
  heading: 2. Required invariants
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 2. Required invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Required invariants

- **Policy mediation:** every tool call and state write is policy-checked and
  audited.
- **Latent-first loop:** continuous cognition does not require LLM decoding.
- **Budget enforcement:** solver calls, CPU/GPU time, and disk writes are
  quota-governed.
- **Crash consistency:** state can be reconstructed from snapshots + deltas.
