---
id: GONI-SYNTHESIS-44FB7018181A
title: 7. Cost and performance model
type: synthesis
status: draft
implementation_state: specified_only
proposition: 'Assumptions: council is slower and more expensive than local or single-cloud.'
domains:
- repository
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/docs/llm-council.md
  heading: 7. Cost and performance model
  revision: 9d6703bc3b42e745ba582d335ab07ca714350976
---

# 7. Cost and performance model

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Cost and performance model
- Assumptions: council is slower and more expensive than local or single-cloud.
- Guards: soft budget per query (max tokens across members), daily/monthly soft limits.
- Tracking: log tokens per council call, cost per model, added latency.
- Routing objective: Council use is justified only when expected quality or
  evidence value exceeds added latency, cloud cost, privacy risk, audit burden,
  energy/thermal cost, and external-dependency cost.
