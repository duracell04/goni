---
id: CTX-MISS-EVAL-01
title: Context-Miss Recovery Evaluation
type: experiment
status: draft
implementation_state: not_applicable
proposition: Evaluate whether bounded context-miss recovery detects deliberately omitted decisive dependencies and retrieves sufficient additional material without unbounded recursion, unnecessary page-ins, or authority expansion.
domains:
- research
- memory
- software
- system
aliases: []
relations:
- type: tests
  target: CTX-MISS-01
sources:
- SRC-PACKER2023-MEMGPT
artifacts: []
uncertainty: Miss detection may fail silently or over-trigger. Evaluation must include both omitted-decisive-evidence cases and cases where the initial ContextPack is already sufficient.
legacy: []
---

# Context-Miss Recovery Evaluation

## Corpus construction

Create paired Work Orders where:

1. decisive evidence or a procedural rule is deliberately withheld from the
   initial ContextPack but remains retrievable under policy;
2. the initial ContextPack is already sufficient;
3. the missing material exists but is policy-inaccessible;
4. the required source is unavailable or stale;
5. retrieved candidates remain contradictory after the permitted recovery
   budget.

## Metrics

Report:

- miss-detection precision and recall;
- successful recovery rate;
- decisive-evidence recovery rate;
- unnecessary page-ins;
- context-miss recursion depth;
- tool/retrieval calls per recovery;
- additional tokens and latency;
- final task quality;
- citation fidelity;
- unresolved-uncertainty surfacing rate;
- budget breaches;
- authority-policy violations.

Authority-policy violations have a target of zero. Retrieval of additional
information must not change tool or execution authority.

## Promotion criterion

The mechanism advances only when it improves recovery of decisive missing
material without unacceptable over-triggering, recursion, latency, or safety
cost.
