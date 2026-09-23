---
id: MULTIAGENT-EVAL-01
title: Compute-normalized single-agent and multi-agent comparison
type: experiment
status: draft
implementation_state: not_applicable
proposition: Goni should justify multi-agent or Council decomposition against a strong single-agent baseline under matched reasoning tokens, model calls, latency or cost budgets, tools, context access, and verification conditions.
domains:
- research
- evaluation
- agents
aliases:
- multi-agent evaluation
relations:
- type: tests
  target: GONI-THESIS-D2E69CC012F4
sources:
- SRC-TRAN2026-COMPUTE-NORMALIZED-MULTIAGENT
artifacts: []
uncertainty: Different tasks may benefit from heterogeneous agents, parallelism, independent evidence, or organizational separation; the experiment is intended to identify when that benefit survives resource normalization.
legacy: []
---

# Compute-normalized single-agent and multi-agent comparison

Multi-agent architecture is a hypothesis to test, not a maturity badge.

For every Goni Council or multi-agent workflow, compare at least:

```text
strong single agent
vs
candidate multi-agent architecture
```

under matched or explicitly reported resources.

## Minimum controls

Hold constant or report:

- base model family and versions,
- available tools and permissions,
- source/context access,
- total reasoning-token budget,
- model-call count,
- wall-clock and latency budget,
- monetary or energy cost,
- verification and retry allowance.

## Outcome metrics

Measure:

- task success,
- policy violations,
- evidence quality,
- disagreement resolution,
- context loss during handoff,
- duplicated work,
- coordination overhead,
- total compute,
- latency,
- reliability across repeated runs.

Promote multi-agent decomposition when the improvement is attributable to genuine structure such as heterogeneous expertise, parallelism, independent evidence, adversarial checking, or authority separation rather than merely additional inference budget.
