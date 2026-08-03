---
id: GONI-IMAP-5FFD8D83919B
title: 2. Lyapunov Scheduling (Control)
type: implementation-map
status: draft
implementation_state: specified_only
proposition: 'Inputs: 𝒦.Tasks.expected_cost_tokens, queue weights (from QueueSnapshot when materialised).'
domains:
- data
- software
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/software/50-data/52-zero-copy-mechanics.md
  heading: 2. Lyapunov Scheduling (Control)
  revision: 3bbc841959041cbd69fbc175f064a2c94340b28d
---

# 2. Lyapunov Scheduling (Control)

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Lyapunov Scheduling (Control)
- Inputs: 𝒦.Tasks.expected_cost_tokens, queue weights (from QueueSnapshot when materialised).
- Process: drift-plus-penalty; select task_ids for dispatch; update `Tasks.state`.
- Output: prioritized `task_id` vector; telemetry in ℰ. Copies: 0 (metadata only).
