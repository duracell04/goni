---
id: GONI-SYNTHESIS-CC02D2607B90
title: 6a) Prompt-budget decomposition
type: synthesis
status: draft
implementation_state: specified_only
proposition: Prompt budgeting should distinguish logical input size from the fresh prefill work actually executed when compatible prefix state is reused.
domains:
- repository
aliases: []
relations: []
sources:
- SRC-GIM2024-PROMPT-CACHE
- SRC-ZHENG2024-SGLANG
artifacts: []
uncertainty: Runtime providers expose different cache accounting. Goni should report only measurements available from the active runtime and must not infer hidden cache or reasoning-token behavior.
legacy:
- path: blueprint/docs/metrics.md
  heading: 6a) Prompt-budget decomposition
  revision: 2322669539d78790badb2d923cafd9b6ece16e5a
---

# 6a) Prompt-budget decomposition

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 6a) Prompt-budget decomposition

Report the logical input and runtime work separately where the backend exposes
the measurements:

- history tokens carried from prior turns;
- context tokens selected by retrieval/context compilation;
- skill-fragment tokens materialized for the Work Order;
- system/policy module tokens;
- tool-schema tokens added by tool declarations or function signatures;
- **logical prompt tokens** after model-specific serialization;
- **fresh prefill tokens** actually computed for the request;
- **reused prefix tokens** served from a compatible cache where measurable;
- output tokens produced for the visible answer;
- branch count in the assembled prompt/plan;
- variant count requested by the user or control plane;
- reasoning-token usage only when the active runtime/provider exposes it
  directly; hidden internal tokens must not be inferred or fabricated.

A cache hit changes computational work without making the corresponding semantic
material disappear from the logical inference frame. Therefore:

[
	ext{logical prompt size}

eq
	ext{fresh prefill work}
]

when compatible prefix reuse is active.
