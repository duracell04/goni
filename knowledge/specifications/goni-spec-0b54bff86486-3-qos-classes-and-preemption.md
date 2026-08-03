---
id: GONI-SPEC-0B54BFF86486
title: 3. QoS classes and preemption
type: specification
status: draft
implementation_state: specified_only
proposition: 'Work is scheduled into classes: interactive (user-facing), background (indexing, maintenance), maintenance (compaction, audits).'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/scheduler-and-interrupts.md
  heading: 3. QoS classes and preemption
  revision: eb8ffb0621bb5cdda9a0a3f7e0107d648253565a
---

# 3. QoS classes and preemption

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. QoS classes and preemption

Work is scheduled into classes:

- interactive (user-facing),
- background (indexing, maintenance),
- maintenance (compaction, audits).

Class priorities are enforced by the scheduler (MaxWeight policy).
