---
id: GONI-SPEC-A5D58DC91383
title: 2.2 Capability handle
type: specification
status: draft
implementation_state: specified_only
proposition: 'Capability handles are opaque references to policy-issued authority: unforgeable by userland tool code, scoped to resource/action sets, time-bounded and revocable.'
domains:
- agent
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-KERN-01-agent-kernel-abi.md
  heading: 2.2 Capability handle
  revision: 3dd57d3f2f82b64e66389712fc66d3308856bac4
---

# 2.2 Capability handle

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 2.2 Capability handle

Capability handles are opaque references to policy-issued authority:
- unforgeable by userland tool code,
- scoped to resource/action sets,
- time-bounded and revocable.
