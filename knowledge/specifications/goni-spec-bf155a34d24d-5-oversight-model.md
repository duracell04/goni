---
id: GONI-SPEC-BF155A34D24D
title: 5. Oversight model
type: specification
status: draft
implementation_state: specified_only
proposition: 'Oversight is post-hoc by default: anomaly-first feed (only blocked, high-risk, or policy-drift events), periodic sampling of autonomous actions, fast policy override and corridor downgrade.'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/delegation-and-autonomy.md
  heading: 5. Oversight model
  revision: f9ecbb3f8f9eaf949387a074d704b342de01bcd3
---

# 5. Oversight model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Oversight model

Oversight is post-hoc by default:

- anomaly-first feed (only blocked, high-risk, or policy-drift events),
- periodic sampling of autonomous actions,
- fast policy override and corridor downgrade.

This avoids per-action approval loops while preserving meaningful human control.
