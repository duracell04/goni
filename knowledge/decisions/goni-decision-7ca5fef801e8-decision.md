---
id: GONI-DECISION-7CA5FEF801E8
title: Decision
type: decision
status: draft
implementation_state: specified_only
proposition: Goni assumes decoding is memory-bound and routes by arithmetic intensity.
domains:
- hardware
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/hardware/90-decisions.md
  heading: Decision
  revision: 4bec51b6b6a9edb3370019a7c8d98c8cf4d65996
---

# Decision

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### Decision

- Goni assumes decoding is memory-bound and routes by arithmetic intensity.
- NPUs are treated as fixed-graph accelerators with explicit shape buckets.
- Persistence MUST control write amplification via LSM-style buffering and gated
  compaction.
- Solver bursts are DVFS-clamped and duty-cycle limited.
