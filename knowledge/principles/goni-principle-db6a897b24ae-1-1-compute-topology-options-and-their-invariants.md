---
id: GONI-PRINCIPLE-DB6A897B24AE
title: 1.1 Compute topology options and their invariants
type: principle
status: draft
implementation_state: specified_only
proposition: This section enumerates topology options as platform contracts.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/20-architecture-options.md
  heading: 1.1 Compute topology options and their invariants
  revision: 13ad3abaaba4ed31afc8523aa6cd5a401d49a27f
---

# 1.1 Compute topology options and their invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 1.1 Compute topology options and their invariants

This section enumerates topology options as platform contracts. It defines
invariants, routing implications, telemetry needs, and failure modes without
hard performance numbers.

Cross-layer links:
- scheduling behavior: `blueprint/software/10-requirements.md`
- runtime routing: `blueprint/software/30-components/llm-runtime.md`
- duty cycle policy: `blueprint/30-specs/scheduler-and-interrupts.md`
