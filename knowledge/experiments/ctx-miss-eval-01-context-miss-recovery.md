---
id: CTX-MISS-EVAL-01
title: Context-Miss Recovery Evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Construct tasks whose decisive evidence or procedural dependency is absent from the initial working set and measure whether CTX-MISS-01 detects, recovers, and recompiles the missing context within bounded recursion and resource budgets.
domains:
- research
- agent
- memory
- software
aliases: []
relations:
- type: tests
  target: CTX-MISS-01
sources:
- SRC-PACKER2023-MEMGPT
artifacts: []
uncertainty: Detection may trade off false misses against unnecessary page-ins. The experiment must report both and treat unresolved recovery as uncertainty rather than success.
legacy: []
---

# Context-Miss Recovery Evaluation

## Task construction

Create cases in which the initial ContextPack intentionally omits one of:

- decisive evidence;
- exact source wording;
- an entity-resolution fact;
- a freshness-critical fact;
- a conflicting source;
- a decisive procedural rule.

Include control cases where the initial context is already sufficient.

## Metrics

Report:

- miss-detection precision and recall;
- successful recovery rate;
- unnecessary page-in rate;
- false confidence after failed recovery;
- recursion depth;
- added and evicted tokens;
- retrieval/tool calls;
- latency;
- final task quality;
- citation fidelity;
- authority violations: target zero.

## Promotion criterion

A context-miss policy may be promoted only when recovery meaningfully improves
task quality or evidence fidelity without unacceptable recursion, latency, false
misses, unnecessary retrieval, or authority regressions.
